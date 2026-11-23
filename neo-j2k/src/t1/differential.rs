// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The decoder's Tier-1 against the C++ block by block (`t1_decode_cblk` and the store, through the
//! oracle's shim): the C++ repository's committed vectors and bench dumps (read from `NJ2K_CPP_SOURCE`), and
//! generated code-blocks of every size, orientation and style, as `tests/unit/t1_unit.cpp` draws them.
//! The generated run in the normal suite is trimmed; `nightly_tier1_decode_sweep` is the long one.

use neo_j2k_oracle::tier1::{DecodeBlock, T1Decoder};
use neo_j2k_testkit::mqng::{self, VectorBlock};

use super::{Block, Destination, Scratch};
use crate::error::Diagnostics;
use crate::tile::Segment;

const TERMALL: u32 = 0x04;
const LAZY: u32 = 0x01;

/// SplitMix64, the generator of the C++ unit tests.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: u32) -> u32 {
        u32::try_from((self.next() >> 11) % u64::from(n)).unwrap()
    }
}

/// A code-block as Tier-1 receives it, with the store that follows.
#[derive(Clone, Debug)]
struct Spec {
    w: u32,
    h: u32,
    numbps: u32,
    orient: u32,
    roishift: u32,
    cblksty: u32,
    len: Vec<u32>,
    passes: Vec<u32>,
    bytes: Vec<u8>,
    /// The lengths the bytes arrive in.
    chunks: Vec<u32>,
    reversible: bool,
    step: f32,
}

impl Spec {
    fn from_vector(block: &VectorBlock, reversible: bool) -> Self {
        Self {
            w: block.width,
            h: block.height,
            numbps: block.numbps,
            orient: block.orient,
            roishift: block.roishift,
            cblksty: block.cblksty,
            len: block.segments.iter().map(|s| s.0).collect(),
            passes: block.segments.iter().map(|s| s.1).collect(),
            bytes: block.bytes.clone(),
            chunks: split(block.bytes.len(), 1),
            reversible,
            step: 1.3,
        }
    }
}

/// `parts` chunk lengths of `n` bytes, as the C++ test splits them; none for no bytes.
fn split(n: usize, parts: u32) -> Vec<u32> {
    if n == 0 {
        return Vec::new();
    }
    let parts = parts.max(1) as usize;
    let mut out = Vec::new();
    let mut at = 0;
    for i in 0..parts {
        let to = if i + 1 == parts { n } else { at + (n - at) / (parts - i) };
        out.push(u32::try_from(to - at).unwrap());
        at = to;
    }
    out
}

/// The most passes segment `i` of a block of this style can have (`opj_t2_init_seg`).
fn seg_maxpasses(style: u32, i: usize) -> u32 {
    if style & TERMALL != 0 {
        1
    } else if style & LAZY != 0 {
        if i == 0 {
            10
        } else if i % 2 == 1 {
            2
        } else {
            1
        }
    } else {
        109
    }
}

fn fill_bytes(v: &mut [u8], rng: &mut Rng, kind: u32) {
    for (i, b) in v.iter_mut().enumerate() {
        let r = u8::try_from(rng.below(256)).unwrap();
        *b = match kind {
            0 => r,
            1 => 0xff,
            2 => 0,
            3 => {
                if i % 2 == 1 {
                    0x90 + u8::try_from(rng.below(0x70)).unwrap()
                } else {
                    0xff
                }
            }
            4 => {
                if rng.below(8) == 0 {
                    0xff
                } else {
                    u8::try_from(rng.below(0x90)).unwrap()
                }
            }
            5 => u8::try_from((i * 7 + (i >> 5)) & 0xff).unwrap(),
            _ => {
                if rng.below(2) == 1 {
                    0xff
                } else {
                    0x8f - u8::try_from(rng.below(3)).unwrap()
                }
            }
        };
    }
}

/// A block of the given shape and style with segments, passes and bytes drawn at random, the segments'
/// passes as Tier 2 gives them (`make_spec` of `t1_unit.cpp`). `hostile` takes `numbps` up to 31, a ROI
/// shift that now and then refuses the block, and the adversarial byte streams; `overlong` cuts the bytes
/// short of the segments' lengths (Tier 2 gives none such).
fn make_spec(rng: &mut Rng, w: u32, h: u32, style: u32, hostile: bool, overlong: bool) -> Spec {
    let orient = rng.below(4);
    let numbps = if rng.below(10) == 0 {
        0
    } else {
        1 + rng.below(if hostile { 31 } else { 12 })
    };
    let roishift = if rng.below(8) == 0 {
        rng.below(if hostile { 32 } else { 12 })
    } else {
        0
    };
    let shape = rng.below(8);
    let nsegs = if shape == 0 {
        109
    } else if style & (TERMALL | LAZY) != 0 {
        1 + rng.below(12)
    } else {
        1 + rng.below(3)
    } as usize;
    let max_passes = 3 * numbps + 3;
    let mut len = Vec::new();
    let mut passes = Vec::new();
    for i in 0..nsegs {
        let mut l = if shape == 0 {
            1
        } else if rng.below(6) == 0 {
            0
        } else {
            rng.below(w * h / 3 + 24)
        };
        if hostile && rng.below(12) == 0 {
            l = 1 + rng.below(3);
        }
        len.push(l);
        let most = seg_maxpasses(style, i);
        passes.push(if i + 1 < nsegs {
            most
        } else {
            rng.below(most.min(max_passes) + 1)
        });
    }
    let total: u32 = len.iter().sum();
    let mut count = total + if rng.below(4) == 0 { rng.below(5) } else { 0 };
    if overlong {
        count = count.saturating_sub(1 + rng.below(total / 2 + 1));
    }
    let mut bytes = vec![0; count as usize];
    let kind = rng.below(if hostile { 7 } else { 1 });
    fill_bytes(&mut bytes, rng, kind);
    let parts = 1 + if rng.below(5) == 0 { rng.below(4) } else { 0 };
    let reversible = rng.below(2) == 0;
    let step = 0.25 + f32::from(u16::try_from(rng.below(200)).unwrap()) / 64.0;
    Spec {
        w,
        h,
        numbps,
        orient,
        roishift,
        cblksty: style,
        len,
        passes,
        chunks: split(bytes.len(), parts),
        bytes,
        reversible,
        step,
    }
}

fn rust_decode(scratch: &mut Scratch, spec: &Spec) -> Option<Vec<i32>> {
    let segments: Vec<Segment> = spec
        .len
        .iter()
        .zip(&spec.passes)
        .map(|(&len, &passes)| Segment {
            len,
            numpasses: passes,
            real_num_passes: passes,
            ..Segment::default()
        })
        .collect();
    let mut at = 0;
    let chunks: Vec<&[u8]> = spec
        .chunks
        .iter()
        .map(|&l| {
            let chunk = &spec.bytes[at..at + l as usize];
            at += l as usize;
            chunk
        })
        .collect();
    let block = Block {
        width: spec.w,
        height: spec.h,
        numbps: spec.numbps,
        orient: spec.orient,
        roishift: spec.roishift,
        cblksty: spec.cblksty,
        segments: &segments,
    };
    let mut out = vec![0x5a5a_5a5a; (spec.w * spec.h) as usize];
    let destination = Destination {
        samples: &mut out,
        offset: 0,
        stride: spec.w as usize,
        reversible: spec.reversible,
        stepsize: spec.step,
    };
    scratch
        .decode(block, chunks, destination, &mut Diagnostics::default())
        .ok()
        .map(|()| out)
}

fn cpp_decode(cpp: &mut T1Decoder, spec: &Spec) -> Option<Vec<i32>> {
    let segments: Vec<(u32, u32)> = spec.len.iter().copied().zip(spec.passes.iter().copied()).collect();
    cpp.decode(&DecodeBlock {
        width: spec.w,
        height: spec.h,
        numbps: spec.numbps,
        orient: spec.orient,
        roishift: spec.roishift,
        cblksty: spec.cblksty,
        segments: &segments,
        bytes: &spec.bytes,
        chunks: &spec.chunks,
        reversible: spec.reversible,
        step_size: spec.step,
    })
}

/// Both decoders, each with its scratch kept from block to block (no state may survive one).
struct Pair {
    cpp: T1Decoder,
    rust: Scratch,
    blocks: u64,
    refused: u64,
}

impl Pair {
    fn new() -> Self {
        Self {
            cpp: T1Decoder::new(),
            rust: Scratch::default(),
            blocks: 0,
            refused: 0,
        }
    }

    /// Panics at the first difference, saying which block.
    fn check(&mut self, spec: &Spec, what: &str) {
        let want = cpp_decode(&mut self.cpp, spec);
        let got = rust_decode(&mut self.rust, spec);
        self.blocks += 1;
        self.refused += u64::from(want.is_none());
        let describe = || {
            format!(
                "{what}: {} x {} orient {} numbps {} roi {} style {:02x} {}, {} segments, {} bytes",
                spec.w,
                spec.h,
                spec.orient,
                spec.numbps,
                spec.roishift,
                spec.cblksty,
                if spec.reversible { "5/3" } else { "9/7" },
                spec.len.len(),
                spec.bytes.len()
            )
        };
        match (want, got) {
            (None, None) => {}
            (Some(want), Some(got)) => {
                if let Some(i) = want.iter().zip(&got).position(|(a, b)| a != b) {
                    let at = u32::try_from(i).unwrap();
                    panic!(
                        "{}: first coefficient {i} (x {} y {}) is {} in the C++ and {} in Rust",
                        describe(),
                        at % spec.w,
                        at / spec.w,
                        want[i],
                        got[i]
                    );
                }
            }
            (want, got) => panic!(
                "{}: the C++ {} the block and Rust {} it",
                describe(),
                if want.is_some() { "decodes" } else { "refuses" },
                if got.is_some() { "decodes" } else { "refuses" }
            ),
        }
    }
}

fn run_vectors(pair: &mut Pair, name: &str, blocks: &[VectorBlock]) {
    let lossless = name.contains("lossless");
    for (i, block) in blocks.iter().enumerate() {
        // the dumps do not say the wavelet: both stores, the bench's name first
        for reversible in [lossless, !lossless] {
            pair.check(&Spec::from_vector(block, reversible), &format!("{name} block {i}"));
        }
    }
}

#[test]
fn committed_vectors_decode_as_the_cpp_does() {
    let Some(cpp_source) = mqng::require_cpp_source() else {
        return;
    };
    let files = mqng::committed_vectors(&cpp_source).expect("the committed Tier-1 vectors read");
    let total: usize = files.iter().map(|f| f.1.len()).sum();
    assert_eq!(
        total, 3_927,
        "the committed vectors: 3,777 corpus blocks and 150 study blocks"
    );
    let mut pair = Pair::new();
    let mut styles = [0u32; 64];
    for (name, blocks) in &files {
        for b in blocks {
            styles[(b.cblksty & 63) as usize] += 1;
        }
        run_vectors(&mut pair, name, blocks);
    }
    assert_eq!(pair.blocks, 2 * 3_927);
    assert!(
        styles.iter().filter(|&&n| n != 0).count() >= 7,
        "styles 00 01 04 05 12 28 3f at least"
    );
}

#[test]
fn bench_dumps_decode_as_the_cpp_does() {
    let Some(cpp_source) = mqng::require_cpp_source() else {
        return;
    };
    let files = mqng::bench_dumps(&cpp_source).expect("the bench dumps read");
    assert_eq!(files.len(), 6);
    let counts: Vec<usize> = files.iter().map(|f| f.1.len()).collect();
    // 1024_lossless, 1024_lossy, 256_lossless, 256_lossy, 512_lossless, 512_lossy
    assert_eq!(counts, [777, 718, 75, 75, 210, 179]);
    let mut pair = Pair::new();
    for (name, blocks) in &files {
        run_vectors(&mut pair, name, blocks);
    }
    assert_eq!(pair.refused, 0);
}

/// The full dump set of the C++ study (37,424 blocks, not committed) when `NJ2K_T1_VECTORS` names it, as
/// the C++ unit test takes it.
#[test]
fn the_full_dump_set_decodes_as_the_cpp_does_when_named() {
    let Some(dir) = std::env::var_os("NJ2K_T1_VECTORS") else {
        return;
    };
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .expect("NJ2K_T1_VECTORS is a folder")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "blk"))
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no .blk files in NJ2K_T1_VECTORS");
    let mut pair = Pair::new();
    for path in files {
        let blocks = mqng::read(&path).expect("a dump reads");
        run_vectors(&mut pair, &path.file_name().unwrap().to_string_lossy(), &blocks);
    }
}

const EDGE: [u32; 8] = [1, 4, 5, 8, 17, 33, 63, 64];
const LONG_SHAPES: [(u32, u32); 16] = [
    (1024, 4),
    (4, 1024),
    (512, 8),
    (8, 512),
    (256, 16),
    (16, 256),
    (128, 32),
    (32, 128),
    (1024, 3),
    (2, 1024),
    (1024, 1),
    (1, 1024),
    (1024, 2),
    (64, 64),
    (4, 64),
    (64, 4),
];

/// The generated blocks of the C++ unit test, the trimmed form: every size 1..64 x 1..64 (a style by rotation, the
/// heights that are no multiple of four among them), the edge sizes in every style and orientation, the long shapes,
/// and full-size blocks. `rounds` repeats the draw with the generator moved on.
fn run_generated(pair: &mut Pair, seed: u64, rounds: u32) {
    let mut rng = Rng(seed);
    for round in 0..rounds {
        for h in 1..=64u32 {
            for w in 1..=64u32 {
                let style = (w * 5 + h * 3 + rng.below(64)) % 64;
                let spec = make_spec(&mut rng, w, h, style, (w + h) % 5 == 0, (w * h + round) % 11 == 0);
                pair.check(&spec, "generated");
            }
        }
        for style in 0..64u32 {
            for orient in 0..4u32 {
                for &w in &EDGE {
                    for &h in &EDGE {
                        let mut spec = make_spec(&mut rng, w, h, style, (w ^ h) & 1 != 0, false);
                        spec.orient = orient;
                        pair.check(&spec, "generated (edge sizes)");
                    }
                }
            }
        }
        for i in 0..16 * 24usize {
            let (w, h) = LONG_SHAPES[i % 16];
            let style = rng.below(64);
            let spec = make_spec(&mut rng, w, h, style, (i / 16) % 3 == 0, i % 7 == 0);
            pair.check(&spec, "generated (long shapes)");
        }
        for i in 0..300 {
            let style = rng.below(64);
            let spec = make_spec(&mut rng, 64, 64, style, i % 3 == 0, i % 13 == 0);
            pair.check(&spec, "generated (64 x 64)");
        }
    }
}

#[test]
fn generated_blocks_decode_as_the_cpp_does() {
    let mut pair = Pair::new();
    run_generated(&mut pair, 20_261_123, 1);
    // 4,096 sizes + 64 styles x 4 orientations x 64 edge pairs + 384 long shapes + 300 full-size blocks
    assert_eq!(pair.blocks, 4_096 + 16_384 + 384 + 300);
    assert!(pair.refused > 0, "some blocks reach 31 bit-planes with the ROI shift");
}

/// The nightly sweep: the same draw on many seeds, in every thread there is.
#[test]
#[ignore = "nightly: the long generated Tier-1 decode sweep"]
fn nightly_tier1_decode_sweep() {
    let threads = std::thread::available_parallelism().map_or(4, std::num::NonZeroUsize::get);
    let rounds = 6;
    let total: u64 = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                scope.spawn(move || {
                    let mut pair = Pair::new();
                    run_generated(&mut pair, 0x7101_0000 + t as u64, rounds);
                    pair.blocks
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().expect("a sweep thread")).sum()
    });
    eprintln!("{total} generated code-blocks decoded as the C++ does ({threads} threads)");
}
