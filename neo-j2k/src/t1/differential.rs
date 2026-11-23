// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The decoder's Tier-1 against the C++ block by block (`t1_decode_cblk` and the store, through the
//! oracle's shim): the C++ repository's committed vectors and bench dumps (read from `NJ2K_CPP_SOURCE`), and
//! generated code-blocks of every size, orientation and style, as `tests/unit/t1_unit.cpp` draws them, with its
//! other block tests: segments longer than the bytes (`test_overlong`), a raw segment holding a cleanup pass
//! (`test_raw_cleanup`), and tiny blocks over every sign and significance pattern of a column (`test_split`), and
//! the store alone. The destination is a tile buffer with a stride and an offset, and the samples between its
//! rows must stay as they were. The generated runs in the normal suite are trimmed; the `nightly_*` tests are the
//! long ones.
//!
//! Not mirrored: the C++ tests of the MQ tables (`test_tables`: Rust's table is checked against the C++ decisions
//! instead) and of the AGG run (`test_agg_run`: the run is the C++'s reformulation, which Rust does not have).

use neo_j2k_oracle::tier1::{self, DecodeBlock, FILL, T1Decoder};
use neo_j2k_testkit::generate::{LONG_SHAPES, SplitMix64 as Rng, split_chunks};
use neo_j2k_testkit::mqng::{self, VectorBlock};

use super::{Block, Destination, Scratch, store_coefficients};
use crate::error::Diagnostics;
use crate::tile::Segment;

const TERMALL: u32 = 0x04;
const LAZY: u32 = 0x01;
const RESET: u32 = 0x02;
const VSC: u32 = 0x08;
const SEGSYM: u32 = 0x20;

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
    /// The tile buffer's row stride and the block's offset in it.
    stride: u32,
    offset: u32,
}

impl Spec {
    fn new(w: u32, h: u32) -> Self {
        Self {
            w,
            h,
            numbps: 0,
            orient: 0,
            roishift: 0,
            cblksty: 0,
            len: Vec::new(),
            passes: Vec::new(),
            bytes: Vec::new(),
            chunks: Vec::new(),
            reversible: true,
            step: 1.3,
            stride: w,
            offset: 0,
        }
    }

    fn from_vector(block: &VectorBlock, reversible: bool) -> Self {
        Self {
            numbps: block.numbps,
            orient: block.orient,
            roishift: block.roishift,
            cblksty: block.cblksty,
            len: block.segments.iter().map(|s| s.0).collect(),
            passes: block.segments.iter().map(|s| s.1).collect(),
            bytes: block.bytes.clone(),
            chunks: split_chunks(block.bytes.len(), 1),
            reversible,
            ..Self::new(block.width, block.height)
        }
    }

    fn buffer_len(&self) -> usize {
        self.offset as usize + (self.h as usize - 1) * self.stride as usize + self.w as usize + tier1::TAIL
    }

    /// Whether Tier-1 must refuse the block: its bit-planes plus ROI shift, as a signed number, reach 31.
    fn refused(&self) -> bool {
        i32::from_ne_bytes(self.numbps.wrapping_add(self.roishift).to_ne_bytes()) >= 31
    }
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

fn byte(rng: &mut Rng, n: u32) -> u8 {
    u8::try_from(rng.below(n)).expect("a byte")
}

fn fill_bytes(v: &mut [u8], rng: &mut Rng, kind: u32) {
    for (i, b) in v.iter_mut().enumerate() {
        let r = byte(rng, 256);
        *b = match kind {
            0 => r,
            1 => 0xff,
            2 => 0,
            3 => {
                if i % 2 == 1 {
                    0x90 + byte(rng, 0x70)
                } else {
                    0xff
                }
            }
            4 => {
                if rng.below(8) == 0 {
                    0xff
                } else {
                    byte(rng, 0x90)
                }
            }
            5 => u8::try_from((i * 7 + (i >> 5)) & 0xff).expect("a byte"),
            _ => {
                if rng.below(2) == 1 {
                    0xff
                } else {
                    0x8f - byte(rng, 3)
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
    let step = 0.25 + f32::from(u16::try_from(rng.below(200)).expect("small")) / 64.0;
    Spec {
        numbps,
        orient,
        roishift,
        cblksty: style,
        len,
        passes,
        chunks: split_chunks(bytes.len(), parts),
        bytes,
        reversible,
        step,
        ..Spec::new(w, h)
    }
}

/// The block stored into a tile buffer of another stride, at another offset (one in five blocks of the draws).
fn strided(rng: &mut Rng, mut spec: Spec) -> Spec {
    spec.stride = spec.w + rng.below(6);
    spec.offset = rng.below(5);
    spec
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
    let mut out = vec![FILL; spec.buffer_len()];
    let destination = Destination {
        samples: &mut out,
        offset: spec.offset as usize,
        stride: spec.stride as usize,
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
        stride: spec.stride,
        offset: spec.offset,
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

    /// Panics at the first difference, saying which block; the C++'s tile buffer otherwise (none for a refused block,
    /// which both must refuse, and only for the bit-planes plus ROI shift rule).
    fn check(&mut self, spec: &Spec, what: &str) -> Option<Vec<i32>> {
        let want = cpp_decode(&mut self.cpp, spec);
        let got = rust_decode(&mut self.rust, spec);
        self.blocks += 1;
        self.refused += u64::from(want.is_none());
        let describe = || {
            format!(
                "{what}: {} x {} orient {} numbps {} roi {} style {:02x} {}, {} segments, {} bytes, stride {} offset {}",
                spec.w,
                spec.h,
                spec.orient,
                spec.numbps,
                spec.roishift,
                spec.cblksty,
                if spec.reversible { "5/3" } else { "9/7" },
                spec.len.len(),
                spec.bytes.len(),
                spec.stride,
                spec.offset
            )
        };
        assert_eq!(
            want.is_none(),
            spec.refused(),
            "{}: the C++ refuses exactly the blocks of 31 bit-planes",
            describe()
        );
        assert_eq!(
            got.is_none(),
            spec.refused(),
            "{}: Rust refuses exactly the blocks of 31 bit-planes",
            describe()
        );
        if let (Some(want), Some(got)) = (&want, &got)
            && let Some(i) = want.iter().zip(got).position(|(a, b)| a != b)
        {
            let row = (i as u64).saturating_sub(u64::from(spec.offset)) / u64::from(spec.stride);
            panic!(
                "{}: first sample {i} (row {row}) is {} in the C++ and {} in Rust",
                describe(),
                want[i],
                got[i]
            );
        }
        want
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
        "fewer than seven code-block styles in the vectors (the C++ README names 00, 01, 04, 05, 12, 28 and 3f)"
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
        eprintln!(
            "skipped: NJ2K_T1_VECTORS does not name the C++ study's full dump set (37,424 blocks, not committed)"
        );
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

/// The generated blocks of the C++ unit test, the trimmed form: every size 1..64 x 1..64 (a style by rotation, the
/// heights that are no multiple of four among them), the edge sizes in every style and orientation, the long shapes,
/// and full-size blocks; one in five stored with another stride and offset. `rounds` repeats the draw with the generator
/// moved on.
fn run_generated(pair: &mut Pair, seed: u64, rounds: u32) {
    let mut rng = Rng(seed);
    for round in 0..rounds {
        for h in 1..=64u32 {
            for w in 1..=64u32 {
                let style = (w * 5 + h * 3 + rng.below(64)) % 64;
                let spec = make_spec(&mut rng, w, h, style, (w + h) % 5 == 0, (w * h + round) % 11 == 0);
                let spec = if (w + h) % 5 == 1 {
                    strided(&mut rng, spec)
                } else {
                    spec
                };
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
            let spec = if i % 5 == 0 { strided(&mut rng, spec) } else { spec };
            pair.check(&spec, "generated (long shapes)");
        }
        for i in 0..300 {
            let style = rng.below(64);
            let spec = make_spec(&mut rng, 64, 64, style, i % 3 == 0, i % 13 == 0);
            let spec = if i % 5 == 0 { strided(&mut rng, spec) } else { spec };
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

/// Segments longer than the bytes that are there (Tier 2 gives none): both decoders take the bytes present, each
/// segment cut to what is left after the ones before it (`test_overlong`; lengths near 0xFFFFFFFF must not wrap a sum).
#[test]
fn overlong_segments_are_cut_as_the_cpp_cuts_them() {
    let mut pair = Pair::new();
    let mut rng = Rng(4242);
    for i in 0..1_500u32 {
        let shapes = [(64, 64), (rng.below(64) + 1, rng.below(64) + 1), (1024, 4), (4, 1024)];
        let (w, h) = shapes[(i % 4) as usize];
        let style = rng.below(64);
        let mut spec = make_spec(&mut rng, w, h, style, i % 2 == 0, false);
        let bytes = u32::try_from(spec.bytes.len()).expect("small");
        for l in &mut spec.len {
            *l = match rng.below(10) {
                0 => *l + 1 + rng.below(40),
                1 => 0xffff_fff0 + rng.below(16),
                2 => bytes + rng.below(3),
                _ => *l,
            };
        }
        pair.check(&spec, "overlong segments");
    }
}

/// A segment BYPASS types raw holds significance and refinement passes only (Tier 2 gives at most two); hand-built
/// ones with a third, a cleanup pass, end at the cleanup pass in both decoders (`test_raw_cleanup`; the C++ ported
/// that in the stage the Rust crate follows).
#[test]
fn a_raw_segment_with_a_cleanup_pass_ends_there_as_in_the_cpp() {
    let mut pair = Pair::new();
    let mut rng = Rng(99);
    let mut raw_segments = 0;
    for _ in 0..600 {
        let (w, h) = (1 + rng.below(64), 1 + rng.below(64));
        let mut spec = Spec::new(w, h);
        spec.cblksty = LAZY
            | if rng.below(2) == 1 { RESET } else { 0 }
            | if rng.below(2) == 1 { VSC } else { 0 }
            | if rng.below(2) == 1 { SEGSYM } else { 0 };
        spec.orient = rng.below(4);
        spec.numbps = 5 + rng.below(10);
        let nsegs = 2 + rng.below(7);
        let mut total = 0;
        for k in 0..nsegs {
            let len = rng.below(w * h / 3 + 24);
            let raw = k % 2 == 1;
            spec.len.push(len);
            spec.passes.push(if k == 0 {
                10
            } else if raw {
                3
            } else {
                1
            });
            raw_segments += u32::from(raw);
            total += len;
        }
        spec.bytes = vec![0; total as usize];
        let kind = rng.below(7);
        fill_bytes(&mut spec.bytes, &mut rng, kind);
        spec.chunks = split_chunks(spec.bytes.len(), 1);
        pair.check(&spec, "raw segment with a cleanup pass");
    }
    assert!(raw_segments > 1_000);
}

/// The significance and the sign of a newly significant sample (decoder step D2 of the C++): tiny blocks over every
/// zero, positive and negative pattern of a column's four rows (`test_split`). Each block is one segment of random
/// bytes, every pass of its bit-planes; the patterns the C++ decode leaves in the columns of the blocks one and
/// three wide, four rows high, are counted. Returns them: `[orientation][column kind][pattern]`.
fn run_split(orient: u32, per_combo: u32, seed: u64) -> [[bool; 81]; 4] {
    const SHAPES: [(u32, u32); 12] = [
        (1, 4),
        (2, 4),
        (3, 4),
        (1, 8),
        (3, 8),
        (2, 5),
        (3, 6),
        (2, 7),
        (1, 3),
        (3, 1),
        (3, 2),
        (5, 12),
    ];
    let mut pair = Pair::new();
    let mut rng = Rng(seed);
    let mut seen = [[false; 81]; 4];
    for (w, h) in SHAPES {
        for style in [0, VSC] {
            // the blocks whose columns are counted get more draws: a rare pattern of a block one wide must be seen
            let draws = if h == 4 && (w == 1 || w == 3) {
                per_combo * 4
            } else {
                per_combo
            };
            for i in 0..draws {
                let mut spec = Spec::new(w, h);
                spec.cblksty = style;
                spec.orient = orient;
                // the 9/7 store keeps a sample's sign and whether it is zero in the float's bits (the 5/3's halving
                // can take a one-step coefficient to zero)
                spec.reversible = false;
                spec.step = 2.0;
                spec.numbps = 1 + rng.below(5);
                let len = 1 + rng.below(w * h * 3 / 2 + 6);
                spec.len.push(len);
                spec.passes.push(1 + 3 * (spec.numbps - 1));
                spec.bytes = vec![0; len as usize];
                fill_bytes(&mut spec.bytes, &mut rng, if i % 7 == 6 { 4 } else { 0 });
                spec.chunks = split_chunks(spec.bytes.len(), 1);
                let out = pair
                    .check(&spec, "split")
                    .expect("a block of at most five bit-planes is not refused");
                if h == 4 && (w == 1 || w == 3) {
                    for x in 0..w {
                        let kind = if w == 1 { 0 } else { 1 + x as usize };
                        let mut pattern = 0;
                        for row in (0..4usize).rev() {
                            let v = out[row * w as usize + x as usize];
                            pattern = pattern * 3 + usize::from(v != 0) + usize::from(v < 0);
                        }
                        seen[kind][pattern] = true;
                    }
                }
            }
        }
    }
    seen
}

#[test]
fn tiny_blocks_decode_as_the_cpp_does_in_every_orientation() {
    for orient in 0..4 {
        run_split(orient, 250, 20_261_124 + u64::from(orient));
    }
}

/// The full `test_split` (3.84 million blocks, 5.8 million with the extra draws of the counted shapes), one thread an orientation, and the count of the patterns it has to see.
#[test]
#[ignore = "nightly: the C++ test_split, extended: 5.8 million tiny blocks"]
fn nightly_tier1_decode_split() {
    let seen: Vec<[[bool; 81]; 4]> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..4u32)
            .map(|orient| scope.spawn(move || run_split(orient, 40_000, 20_261_124 + u64::from(orient))))
            .collect();
        handles.into_iter().map(|h| h.join().expect("a split thread")).collect()
    });
    let mut missing = 0;
    for (orient, per_orient) in seen.iter().enumerate() {
        for (kind, patterns) in per_orient.iter().enumerate() {
            for (pattern, &seen) in patterns.iter().enumerate() {
                if !seen {
                    eprintln!("never decoded: orientation {orient}, column kind {kind}, pattern {pattern}");
                    missing += 1;
                }
            }
        }
    }
    assert_eq!(
        missing, 0,
        "{missing} of the 4 x 4 x 81 (orientation, column kind, pattern) cases were never decoded"
    );
}

/// The store alone: coefficients around every ROI threshold, the extremes and noise, in stores of other strides, in
/// both wavelets, Rust's scalar and (where built) its vector path against the C++'s.
#[test]
fn stores_match_the_cpp_including_the_roi_threshold() {
    let mut rng = Rng(77);
    let mut cases = 0;
    for roishift in [0u32, 1, 2, 5, 8, 15, 29, 30, 31, 32, 40] {
        for &width in &[1u32, 7, 8, 9, 16, 17, 33] {
            for height in [1u32, 2, 4] {
                let n = (width * height) as usize;
                let mut specials: Vec<i32> = vec![0, 1, -1, 7, i32::MIN, i32::MIN + 1, i32::MAX, i32::MAX - 1];
                if (1..31).contains(&roishift) {
                    let t = 1i32 << roishift;
                    for v in [
                        t - 1,
                        t,
                        t + 1,
                        t.saturating_mul(2) - 1,
                        t.saturating_mul(2),
                        t.saturating_mul(3),
                    ] {
                        specials.extend([v, -v]);
                    }
                }
                let coefficients: Vec<i32> = (0..n)
                    .map(|i| {
                        if rng.below(3) == 0 {
                            i32::try_from(rng.below(1 << 20)).expect("small") - (1 << 19)
                        } else {
                            specials[(i + rng.below(8) as usize) % specials.len()]
                        }
                    })
                    .collect();
                for reversible in [true, false] {
                    for step in [0.0f32, 0.3, 1.0, 7.5] {
                        for stride in [width, width + 3] {
                            let want = tier1::store(&coefficients, (width, height), roishift, reversible, step, stride);
                            for vector in [false, true] {
                                let mut got = vec![FILL; want.len()];
                                let end = (height as usize - 1) * stride as usize + width as usize;
                                let mut input = coefficients.clone();
                                let (w, s) = (width as usize, stride as usize);
                                if vector {
                                    store_coefficients::<true>(
                                        &mut input,
                                        &mut got[..end],
                                        w,
                                        s,
                                        roishift,
                                        reversible,
                                        step,
                                    );
                                } else {
                                    store_coefficients::<false>(
                                        &mut input,
                                        &mut got[..end],
                                        w,
                                        s,
                                        roishift,
                                        reversible,
                                        step,
                                    );
                                }
                                assert_eq!(
                                    got,
                                    want,
                                    "store {width} x {height} stride {stride} roi {roishift} {} step {step} vector {vector}",
                                    if reversible { "5/3" } else { "9/7" }
                                );
                                cases += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(cases > 7_000);
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
    eprintln!("{total} generated code-blocks decoded as the cpp does ({threads} threads)");
}
