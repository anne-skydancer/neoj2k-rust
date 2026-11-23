// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The encoder's Tier-1 against the C++ block by block (`enc_t1_encode` and its commit, through the oracle's
//! shim), on generated code-blocks as `tests/unit/enc_t1_unit.cpp` draws them: every size 1..64 x 1..64 (the edge
//! sizes in all orientation and wavelet combinations), full-size blocks over many contents, and blocks whose
//! largest magnitude gives each bit-plane count from 0 to 25, INT32_MIN among them. The passes (rate, length,
//! termination, distortion bit for bit), the bit-plane count and the bytes must be the C++'s. The normal suite
//! runs the trimmed draw; `nightly_tier1_encode_sweep` repeats it on many seeds and adds blocks whose samples have
//! leading bit-planes of their own.

use neo_j2k_oracle::tier1::{EncodeBlock, T1Encoder};
use neo_j2k_testkit::generate::SplitMix64 as Rng;

use super::{Block, T1Params, Workspace, encode};

/// The parameters of a block beside its samples.
#[derive(Clone, Copy, Debug)]
struct Setup {
    width: usize,
    height: usize,
    orient: u8,
    reversible: bool,
    mct: bool,
    level: usize,
    component: usize,
    step: f32,
}

impl Setup {
    fn draw(rng: &mut Rng, width: usize, height: usize, orient: u32, reversible: bool) -> Self {
        Self {
            width,
            height,
            orient: u8::try_from(orient).unwrap(),
            reversible,
            mct: rng.below(2) == 1,
            level: rng.below(6) as usize,
            component: rng.below(4) as usize,
            // exactly representable in f32, as a band's step size is
            step: 0.0123 + f32::from(u16::try_from(rng.below(2_000)).unwrap()) / 1_000.0,
        }
    }
}

struct Pair {
    cpp: T1Encoder,
    workspace: Workspace,
    block: Block,
    blocks: u64,
}

impl Pair {
    fn new() -> Self {
        Self {
            cpp: T1Encoder::new(),
            workspace: Workspace::default(),
            block: Block::default(),
            blocks: 0,
        }
    }

    /// Panics at the first difference, saying which block.
    fn check(&mut self, samples: &[i32], s: Setup, what: &str) {
        self.blocks += 1;
        let describe = || {
            format!(
                "{what}: {} x {} orient {} {} level {} component {} mct {}",
                s.width,
                s.height,
                s.orient,
                if s.reversible { "5/3" } else { "9/7" },
                s.level,
                s.component,
                s.mct
            )
        };
        let want = self.cpp.encode(&EncodeBlock {
            width: u32::try_from(s.width).unwrap(),
            height: u32::try_from(s.height).unwrap(),
            samples,
            orient: u32::from(s.orient),
            component: u32::try_from(s.component).unwrap(),
            level: u32::try_from(s.level).unwrap(),
            reversible: s.reversible,
            mct: s.mct,
            step_size: f64::from(s.step),
        });
        let params = T1Params {
            orientation: s.orient,
            level: s.level,
            component: s.component,
            reversible: s.reversible,
            step_size: s.step,
            mct: s.mct,
        };
        let got = encode(&mut self.workspace, samples, s.width, s.height, params, &mut self.block);
        // the encoder's output clamp is not reachable through a block (its buffer holds four bytes a sample and the
        // coder writes about one and a half at most, as the C++ unit test notes; the clamp is tested in the MQ tests)
        assert!(
            want.ok,
            "{}: the C++ refuses a block through its output clamp",
            describe()
        );
        got.unwrap_or_else(|e| panic!("{}: Rust refuses a block the C++ codes: {e}", describe()));
        let block = &self.block;
        assert_eq!(block.numbps, want.numbps, "{}: bit-plane count", describe());
        assert_eq!(block.passes.len(), want.passes.len(), "{}: pass count", describe());
        for (i, (got, want)) in block.passes.iter().zip(&want.passes).enumerate() {
            assert_eq!(
                (got.rate, got.len, got.terminated, got.distortion_dec.to_bits()),
                (want.rate, want.len, want.terminated, want.distortion.to_bits()),
                "{}: pass {i} (rate, length, termination, distortion)",
                describe()
            );
        }
        let last = want.passes.last().map_or(0, |p| p.rate as usize);
        // the C++ commits the coder's whole count; the last pass's rate is one less when it gave up a trailing 0xFF
        assert!(
            block.data.len() == last || block.data.len() == last + 1,
            "{}: {} bytes against a last rate of {last}",
            describe(),
            block.data.len()
        );
        assert_eq!(block.data[..last], want.bytes[..], "{}: bytes", describe());
        if block.data.len() == last + 1 {
            // the byte the last pass's rate gave up is the 0xFF a pass may not end with
            assert_eq!(block.data[last], 0xff, "{}: the byte past the last rate", describe());
        }
    }
}

/// Samples of the kinds of the C++ test's `fill_block`.
fn fill_block(rng: &mut Rng, w: usize, h: usize) -> Vec<i32> {
    let kind = rng.below(7);
    let bits = rng.below(31);
    let mag = if bits == 0 { 0 } else { (1i64 << bits) - 1 };
    let mut out = Vec::with_capacity(w * h);
    for y in 0..h {
        for x in 0..w {
            let random = |rng: &mut Rng| i64::try_from(rng.next_u64() % (u64::try_from(mag).unwrap() + 1)).unwrap();
            let signed = |rng: &mut Rng, v: i64| if rng.below(2) == 1 { -v } else { v };
            let v: i64 = match kind {
                0 => 0,
                1 => {
                    let v = if mag == 0 { 0 } else { random(rng) };
                    signed(rng, v)
                }
                2 => (i64::try_from(x + y).unwrap() * mag) / i64::try_from(w + h).unwrap(),
                3 => {
                    let v = if rng.below(16) == 0 && mag != 0 { random(rng) } else { 0 };
                    signed(rng, v)
                }
                4 => {
                    if rng.below(8) == 0 {
                        if rng.below(2) == 1 {
                            i64::from(i32::MIN)
                        } else {
                            i64::from(i32::MAX)
                        }
                    } else {
                        i64::from(rng.below(1_000)) - 500
                    }
                }
                5 => mag / 2 + 1,
                _ => {
                    let v = i64::try_from((x * 7 + y * 3) % 61).unwrap() * (mag / 61 + 1) + i64::from(rng.below(64));
                    if (x ^ y) & 1 == 1 { -v } else { v }
                }
            };
            out.push(i32::try_from(v.clamp(i64::from(i32::MIN), i64::from(i32::MAX))).unwrap());
        }
    }
    out
}

/// Samples whose largest magnitude gives `planes` bit-planes (0 to 25), the largest negative or positive by
/// chance and every other sample smaller (`test_encode_planes` of the C++ test).
fn plane_block(rng: &mut Rng, w: usize, h: usize, planes: u32, force_extremes: bool) -> Vec<i32> {
    let n = w * h;
    let lo = if planes == 0 { 0 } else { 1u64 << (planes + 5) };
    let hi = if planes == 0 { 1 << 6 } else { 1u64 << (planes + 6) };
    let below = if planes == 0 { hi } else { lo };
    let dense = rng.below(2) == 1;
    let top_at = rng.below(u32::try_from(n).unwrap()) as usize;
    let mut v: Vec<i32> = (0..n)
        .map(|i| {
            let mut m = rng.next_u64() % below;
            if dense {
                m &= rng.next_u64();
            }
            if i == top_at && planes != 0 {
                m = lo + rng.next_u64() % (hi - lo);
            }
            let m = i64::try_from(m).unwrap();
            i32::try_from(if rng.below(2) == 1 { -m } else { m }.clamp(i64::from(i32::MIN), i64::from(i32::MAX)))
                .unwrap()
        })
        .collect();
    if planes == 25 && force_extremes {
        v[top_at] = i32::MIN;
        if n > 1 {
            v[(top_at + 1) % n] = i32::MAX;
        }
    }
    v
}

/// Samples with a leading bit-plane of their own each (zero, or one of the planes up to `planes + 5`).
fn own_plane_block(rng: &mut Rng, w: usize, h: usize, planes: u32) -> Vec<i32> {
    (0..w * h)
        .map(|_| {
            if rng.below(5) == 0 {
                return 0;
            }
            let p = rng.below(planes + 6);
            let low = rng.next_u64() & ((1u64 << p) - 1);
            let m = i64::try_from((1u64 << p) | low).unwrap();
            i32::try_from(if rng.below(2) == 1 { -m } else { m }).unwrap()
        })
        .collect()
}

/// The C++ unit test's draws: every size (the edge sizes in every orientation and wavelet), full-size blocks,
/// the plane counts 0 to 25 over eight sizes, orientations and wavelets, and INT32_MIN alone and all zero.
fn run_draw(pair: &mut Pair, seed: u64) {
    const EDGE: [usize; 16] = [1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64];
    let mut rng = Rng(seed);
    for h in 1..=64usize {
        for w in 1..=64usize {
            let is_edge = EDGE.contains(&h) && EDGE.contains(&w);
            let reps = if is_edge { 8 } else { 1 };
            for r in 0..reps {
                let combo = if is_edge { r } else { (w * 3 + h) % 8 };
                let samples = fill_block(&mut rng, w, h);
                let setup = Setup::draw(&mut rng, w, h, u32::try_from(combo & 3).unwrap(), (combo >> 2) & 1 == 1);
                pair.check(&samples, setup, "generated");
            }
        }
    }
    for i in 0..400usize {
        let samples = fill_block(&mut rng, 64, 64);
        let setup = Setup::draw(&mut rng, 64, 64, u32::try_from(i & 3).unwrap(), (i >> 2) & 1 == 1);
        pair.check(&samples, setup, "generated (64 x 64)");
    }
    const SIZES: [(usize, usize); 8] = [(64, 64), (64, 1), (1, 64), (33, 17), (5, 7), (64, 63), (4, 4), (1, 1)];
    for planes in 0..=25u32 {
        for k in 0..64usize {
            let (w, h) = SIZES[k % 8];
            let samples = plane_block(&mut rng, w, h, planes, k & 4 != 0);
            let setup = Setup::draw(&mut rng, w, h, u32::try_from((k / 8) % 4).unwrap(), k / 32 == 1);
            pair.check(&samples, setup, &format!("{planes} bit-planes"));
        }
    }
    for (k, &(w, h)) in SIZES.iter().enumerate() {
        for fill in [i32::MIN, 0] {
            let samples = vec![fill; w * h];
            let setup = Setup::draw(&mut rng, w, h, u32::try_from(k & 3).unwrap(), (k >> 2) & 1 == 1);
            pair.check(&samples, setup, if fill == 0 { "all zero" } else { "INT32_MIN only" });
        }
    }
}

#[test]
fn generated_blocks_encode_as_the_cpp_does() {
    let mut pair = Pair::new();
    run_draw(&mut pair, 20_261_122);
    // 4,096 sizes with the 256 edge pairs eight times each, 400 full-size, 26 x 64 plane blocks, 16 extremes
    assert_eq!(pair.blocks, 4_096 + 256 * 7 + 400 + 26 * 64 + 16);
}

/// The nightly sweep: the draw on many seeds, and blocks whose samples have bit-planes of their own over every
/// size, in every thread there is.
#[test]
#[ignore = "nightly: the long generated Tier-1 encode sweep"]
fn nightly_tier1_encode_sweep() {
    let threads = std::thread::available_parallelism().map_or(4, std::num::NonZeroUsize::get);
    let total: u64 = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                scope.spawn(move || {
                    let mut pair = Pair::new();
                    let seed = 0x7102_0000 + t as u64;
                    run_draw(&mut pair, seed);
                    let mut rng = Rng(seed ^ 0xffff);
                    for h in 1..=64usize {
                        for w in 1..=64usize {
                            let planes = rng.below(26);
                            let samples = own_plane_block(&mut rng, w, h, planes);
                            let (orient, reversible) = (rng.below(4), rng.below(2) == 1);
                            let setup = Setup::draw(&mut rng, w, h, orient, reversible);
                            pair.check(&samples, setup, "own bit-planes");
                        }
                    }
                    pair.blocks
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().expect("a sweep thread")).sum()
    });
    eprintln!("{total} generated code-blocks encoded as the C++ does ({threads} threads)");
}

// ---- the block families built for the cleanup pass's candidate mask and the refinement pass's pair list ----
// (the C++ unit test's `test_encode_cleanup` and `test_encode_refine`)

/// A sample with six fractional bits under the integer magnitude `level`, with a random fraction.
fn level_sample(rng: &mut Rng, level: u32, negative: bool) -> i32 {
    let magnitude = i32::try_from((u64::from(level) << 6) | u64::from(rng.below(64))).unwrap();
    if negative { -magnitude } else { magnitude }
}

/// The sign of the sample at (`x`, `y`) in the four sign patterns of the row-mask families.
fn pattern_sign(rng: &mut Rng, kind: u32, x: usize, y: usize) -> bool {
    match kind {
        0 => false,
        1 => true,
        2 => (x ^ y) & 1 != 0,
        _ => rng.below(2) != 0,
    }
}

/// Dense 64 x 64 blocks of one magnitude (the all-significant, all-visited columns), 25 plane counts by four sign
/// patterns.
fn dense_blocks(pair: &mut Pair, rng: &mut Rng, what: &str) {
    for k in 0..100u32 {
        let planes = 1 + k % 25;
        let pattern = k / 25; // all positive, all negative, rows alternate, columns alternate
        let samples: Vec<i32> = (0..64 * 64)
            .map(|i| {
                let (x, y) = (i % 64, i / 64);
                let negative = pattern == 1 || (pattern == 2 && y & 1 == 1) || (pattern == 3 && x & 1 == 1);
                level_sample(rng, (1 << planes) - 1, negative)
            })
            .collect();
        let setup = Setup::draw(rng, 64, 64, k & 3, (k >> 2) & 1 == 1);
        pair.check(&samples, setup, &format!("{what}: {planes} planes, pattern {pattern}"));
    }
}

/// The sizes of the row-mask families: whole stripes and partial ones.
const MASK_SIZES: [(usize, usize); 9] = [
    (6, 4),
    (5, 8),
    (64, 4),
    (7, 5),
    (9, 3),
    (4, 1),
    (3, 7),
    (12, 9),
    (64, 64),
];

/// Every pair of 4-bit row masks (`high` rows lead three planes above `plane`, `mid` rows lead at it, the others
/// stay small), four sign patterns, every orientation and both wavelets over the pairs, on `MASK_SIZES`.
/// The other rows stay under the mid plane (the refinement family: under half of it).
fn row_mask_blocks(pair: &mut Pair, rng: &mut Rng, planes: [u32; 3], refinement: bool, what: &str) {
    for (si, &(w, h)) in MASK_SIZES.iter().enumerate() {
        for high in 0..16u32 {
            for mid in 0..16u32 {
                let sign_kind = (high * 16 + mid) % 4;
                let plane = planes[((high + mid) as usize + si) % 3];
                let samples: Vec<i32> = (0..w * h)
                    .map(|i| {
                        let (x, y) = (i % w, i / w);
                        let row = (y + x) & 3;
                        let level = if (high >> row) & 1 == 1 {
                            (1 << (plane + 3)) | rng.below(1 << (plane + 3))
                        } else if (mid >> row) & 1 == 1 {
                            (1 << plane) | rng.below(1 << plane)
                        } else if refinement {
                            rng.below(1 << plane.saturating_sub(1))
                        } else {
                            rng.below(1 << plane)
                        };
                        let negative = pattern_sign(rng, sign_kind, x, y);
                        level_sample(rng, level, negative)
                    })
                    .collect();
                let setup = Setup::draw(rng, w, h, (high + mid) & 3, ((high ^ mid) >> 2) & 1 == 1);
                pair.check(
                    &samples,
                    setup,
                    &format!("{what}: {w} x {h}, high {high}, mid {mid}, signs {sign_kind}"),
                );
            }
        }
    }
}

const EDGE_SIZES: [usize; 16] = [1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64];

/// Every size 1..64 x 1..64 (the edge sizes in all eight orientation and wavelet combinations); `block` builds
/// the samples from the size, the plane count and a repetition and says what the block is.
fn every_size_blocks(
    pair: &mut Pair,
    rng: &mut Rng,
    what: &str,
    mut block: impl FnMut(&mut Rng, usize, usize, u32, u32) -> (Vec<i32>, String),
) -> [u32; 26] {
    let mut per_plane = [0u32; 26];
    for h in 1..=64usize {
        for w in 1..=64usize {
            let reps = if EDGE_SIZES.contains(&h) && EDGE_SIZES.contains(&w) {
                8
            } else {
                1
            };
            for r in 0..reps {
                let combo = if reps == 8 { r } else { (w * 3 + h) % 8 };
                // 0 (every sample under 2^6: no pass runs) and 1 to 25 planes, in rotation
                let planes = u32::try_from((w * 64 + h + r * 7) % 26).unwrap();
                let (samples, note) = block(rng, w, h, planes, u32::try_from(r).unwrap());
                let setup = Setup::draw(rng, w, h, u32::try_from(combo & 3).unwrap(), (combo >> 2) & 1 == 1);
                pair.check(&samples, setup, &format!("{what}: {planes} planes, {note}"));
                per_plane[planes as usize] += 1;
            }
        }
    }
    per_plane
}

/// The cleanup pass: a column outside run mode codes the rows that are neither significant nor visited, the set
/// fixed at the column's start (E2).
#[test]
fn cleanup_blocks_encode_as_the_cpp_does() {
    let mut pair = Pair::new();
    let mut rng = Rng(20_261_124);
    // a. every pair of row masks; the other rows stay under the mid plane
    row_mask_blocks(&mut pair, &mut rng, [0, 1, 4], false, "cleanup a");
    // b. every size, a leading bit-plane of its own for every sample
    let per_plane = every_size_blocks(&mut pair, &mut rng, "cleanup b", |rng, w, h, planes, _| {
        let style = rng.below(4);
        let top_at = rng.below(u32::try_from(w * h).unwrap()) as usize;
        let mut samples: Vec<i32> = (0..w * h)
            .map(|i| {
                let (x, y) = (i % w, i / w);
                let mut level = 0;
                if planes != 0 {
                    let lead = rng.below(planes);
                    level = (1 << lead) | if lead != 0 { rng.below(1 << lead) } else { 0 };
                    if style == 1 && (x + y) & 3 != 0 {
                        level >>= 1;
                    } else if style == 2 && planes > 3 {
                        level = (1 << (planes - 1)) | rng.below(1 << (planes - 1));
                    } else if style == 3 && (y & 3) != (x & 3) {
                        level = u32::from(level != 0);
                    }
                }
                let negative = rng.below(2) != 0;
                level_sample(rng, level, negative)
            })
            .collect();
        if planes != 0 {
            let level = (1 << (planes - 1)) | if planes > 1 { rng.below(1 << (planes - 1)) } else { 0 };
            let negative = rng.below(2) != 0;
            samples[top_at] = level_sample(rng, level, negative);
        }
        (samples, format!("style {style}"))
    });
    assert!(per_plane.iter().all(|&n| n >= 100), "{per_plane:?}");
    // c. dense blocks
    dense_blocks(&mut pair, &mut rng, "cleanup c");
    assert_eq!(pair.blocks, 9 * 256 + 5_888 + 100);
}

/// The refinement pass: the samples it codes, their order and contexts are fixed by the flags at its start, so a
/// list of (context, bit) pairs is coded in one loop (E3).
#[test]
fn refinement_blocks_encode_as_the_cpp_does() {
    let mut pair = Pair::new();
    let mut rng = Rng(20_261_125);
    // a. every size, four kinds of significance
    let per_plane = every_size_blocks(&mut pair, &mut rng, "refinement a", |rng, w, h, planes, r| {
        let kind = (u32::try_from(w).unwrap() + 2 * u32::try_from(h).unwrap() + r) % 4;
        let mut samples: Vec<i32> = (0..w * h)
            .map(|i| {
                let (x, y) = (i % w, i / w);
                let mut level = 0;
                if planes != 0 {
                    let top = planes - 1;
                    level = match kind {
                        // dense: nearly every sample leads in the top plane
                        0 => {
                            if rng.below(8) == 0 {
                                rng.below(1 << top)
                            } else {
                                (1 << top) | rng.below(1 << top)
                            }
                        }
                        // sparse: an isolated leading sample every 4th row and 5th column
                        1 => {
                            if y % 4 == 1 && x % 5 == 2 {
                                (1 << top) | rng.below(1 << top)
                            } else {
                                rng.below(1 << (top / 2))
                            }
                        }
                        // staggered: a leading plane of its own
                        2 => {
                            let lead = rng.below(planes);
                            (1 << lead) | if lead != 0 { rng.below(1 << lead) } else { 0 }
                        }
                        // stripes: whole rows or columns lead, the others stay small
                        _ => {
                            if y & 4 != 0 || x % 3 == 0 {
                                (1 << top) | rng.below(1 << top)
                            } else {
                                rng.below(1 << (top / 2))
                            }
                        }
                    };
                }
                let negative = rng.below(2) != 0;
                level_sample(rng, level, negative)
            })
            .collect();
        if planes != 0 {
            let at = rng.below(u32::try_from(w * h).unwrap()) as usize;
            let level = (1 << (planes - 1)) | if planes > 1 { rng.below(1 << (planes - 1)) } else { 0 };
            let negative = rng.below(2) != 0;
            samples[at] = level_sample(rng, level, negative);
        }
        (samples, format!("kind {kind}"))
    });
    assert!(per_plane.iter().all(|&n| n >= 100), "{per_plane:?}");
    // b. every pair of row masks: the early rows refine in two or more passes, the late ones at their first
    row_mask_blocks(&mut pair, &mut rng, [1, 3, 8], true, "refinement b");
    // c. dense blocks
    dense_blocks(&mut pair, &mut rng, "refinement c");
    assert_eq!(pair.blocks, 5_888 + 9 * 256 + 100);
}
