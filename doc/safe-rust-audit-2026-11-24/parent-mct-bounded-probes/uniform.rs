// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
use super::lanes::*;
const LANES: usize = 8;
type Block = [i32; LANES];
#[inline(always)]
fn inverse_reversible([y, u, v]: [Block; 3]) -> [Block; 3] {
    let g = sub_i32(y, arithmetic_right::<2>(add_i32(u, v)));
    [add_i32(v, g), g, add_i32(u, g)]
}
#[inline(always)]
fn inverse_irreversible([y, u, v]: [Block; 3]) -> [FloatLanes; 3] {
    let [y, u, v] = [y, u, v].map(from_float_bits);
    [
        core::array::from_fn(|i| y[i] + v[i] * 1.402),
        core::array::from_fn(|i| (y[i] - u[i] * 0.34413) - v[i] * 0.71414),
        core::array::from_fn(|i| y[i] + u[i] * 1.772),
    ]
}
#[inline(always)]
fn integer(value: Block, (shift, lo, hi): (i32, i32, i32)) -> Block {
    value.map(|v| v.clamp(lo - shift, hi - shift) + shift)
}
// The clamp-before-bias approach is also used by Aurora-Viewer's safe aurora-j2k
// mct::shift_97_fast (1ea0749c798daf81ccb059de74557b862c9e2340), derived from neo-j2k.
#[derive(Clone, Copy)]
struct FastShift {
    low: f32,
    high: f32,
    shift: i32,
}
impl FastShift {
    fn new((shift, lo, hi): (i32, i32, i32)) -> Option<Self> {
        let (low, high) = (lo - shift, hi - shift);
        (low >= -4_194_304 && high <= 4_194_304).then_some(Self {
            low: low as f32,
            high: high as f32,
            shift,
        })
    }
    #[inline(always)]
    fn apply(self, value: FloatLanes) -> Block {
        // Ordered comparisons make NaN choose the lower endpoint. The integer bounds
        // are exact floats, so clamping before nearest-even rounding preserves results.
        let rounded = value.map(|v| {
            let v = if v > self.low { v } else { self.low };
            let v = if v < self.high { v } else { self.high };
            v + 12_582_912.0
        });
        rounded.map(|v| v.to_bits().wrapping_sub(0x4b40_0000).cast_signed() + self.shift)
    }
}
#[inline(always)]
#[allow(
    clippy::cast_possible_truncation,
    reason = "the low mantissa word encodes the signed integer"
)]
fn float_general(value: FloatLanes, (shift, lo, hi): (i32, i32, i32)) -> Block {
    // Widening is exact, and every i32 endpoint is exact in f64. The bias has unit
    // spacing throughout this range, so its low word holds the nearest-even signed
    // integer. This avoids scalar saturating f32-to-i32 conversions in wider finishes.
    let lower = f64::from(lo - shift);
    let upper = f64::from(hi - shift);
    value.map(|v| {
        let v = f64::from(v);
        let v = if v > lower { v } else { lower };
        let v = if v < upper { v } else { upper };
        ((v + 6_755_399_441_055_744.0).to_bits() as u32).cast_signed() + shift
    })
}
pub(crate) fn inverse([y, u, v]: [&mut [i32]; 3], reversible: bool) -> usize {
    let (ys, _) = y.as_chunks_mut::<LANES>();
    let (us, _) = u.as_chunks_mut::<LANES>();
    let (vs, _) = v.as_chunks_mut::<LANES>();
    let count = ys.len().min(us.len()).min(vs.len()) * LANES;
    if reversible {
        for ((a, b), c) in ys.iter_mut().zip(us).zip(vs) {
            let [r, g, blue] = inverse_reversible([*a, *b, *c]);
            *a = r;
            *b = g;
            *c = blue;
        }
    } else {
        for ((a, b), c) in ys.iter_mut().zip(us).zip(vs) {
            let [r, g, blue] = inverse_irreversible([*a, *b, *c]);
            *a = float_bits(r);
            *b = float_bits(g);
            *c = float_bits(blue);
        }
    }
    count
}
pub(crate) fn inverse_and_finish(
    [y, u, v]: [&mut [i32]; 3],
    reversible: bool,
    finishes: [(i32, i32, i32); 3],
) -> usize {
    let (ys, _) = y.as_chunks_mut::<LANES>();
    let (us, _) = u.as_chunks_mut::<LANES>();
    let (vs, _) = v.as_chunks_mut::<LANES>();
    let count = ys.len().min(us.len()).min(vs.len()) * LANES;
    if reversible {
        for ((a, b), c) in ys.iter_mut().zip(us).zip(vs) {
            let [r, g, blue] = inverse_reversible([*a, *b, *c]);
            *a = integer(r, finishes[0]);
            *b = integer(g, finishes[1]);
            *c = integer(blue, finishes[2]);
        }
    } else if let [Some(red), Some(green), Some(blue)] = finishes.map(FastShift::new) {
        for ((a, b), c) in ys.iter_mut().zip(us).zip(vs) {
            let [r, g, b_out] = inverse_irreversible([*a, *b, *c]);
            *a = red.apply(r);
            *b = green.apply(g);
            *c = blue.apply(b_out);
        }
    } else if finishes[0] == finishes[1] && finishes[1] == finishes[2] {
        let finish = finishes[0];
        for ((a, b), c) in ys.iter_mut().zip(us).zip(vs) {
            let [r, g, b_out] = inverse_irreversible([*a, *b, *c]);
            *a = float_general(r, finish);
            *b = float_general(g, finish);
            *c = float_general(b_out, finish);
        }
    } else {
        for ((a, b), c) in ys.iter_mut().zip(us).zip(vs) {
            let [r, g, b_out] = inverse_irreversible([*a, *b, *c]);
            *a = float_general(r, finishes[0]);
            *b = float_general(g, finishes[1]);
            *c = float_general(b_out, finishes[2]);
        }
    }
    count
}
pub(crate) fn level_shift(line: &mut [i32], shift: i32, lo: i32, hi: i32, reversible: bool) -> usize {
    let (blocks, _) = line.as_chunks_mut::<LANES>();
    let count = blocks.len() * LANES;
    if reversible {
        for block in blocks {
            *block = integer(*block, (shift, lo, hi));
        }
    } else if let Some(fast) = FastShift::new((shift, lo, hi)) {
        for block in blocks {
            *block = fast.apply(from_float_bits(*block));
        }
    } else {
        for block in blocks {
            *block = float_general(from_float_bits(*block), (shift, lo, hi));
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(
        clippy::cast_possible_truncation,
        reason = "reference saturation follows nearest-even rounding"
    )]
    fn wide_finishing_matches_independent_rounding_and_integer_clamping() {
        let specials = [
            0,
            0x8000_0000,
            1,
            0x8000_0001,
            0x7fc0_0000,
            0x7fa0_0001,
            0xffc0_0001,
            0x7f80_0000,
            0xff80_0000,
            0x4eff_ffff,
            0x4f00_0000,
            0xcf00_0000,
            0x4aff_ffff,
            0xcaff_ffff,
            0x3f00_0000,
            0xbf00_0000,
        ];
        let mut state = 0x3128_5678u32;
        for precision in 1..=31 {
            for signed in [false, true] {
                let magnitude = 1_i32 << (precision - 1);
                let shift = if signed { 0 } else { magnitude };
                let (lo, hi) = if signed {
                    (-magnitude, magnitude - 1)
                } else {
                    (0, 2 * (magnitude - 1) + 1)
                };
                for block in 0..if cfg!(miri) { 4 } else { 512 } {
                    let values: FloatLanes = core::array::from_fn(|lane| {
                        state ^= state << 13;
                        state ^= state >> 17;
                        state ^= state << 5;
                        let index = block * LANES + lane;
                        f32::from_bits(if index < specials.len() { specials[index] } else { state })
                    });
                    let expected = values.map(|v| {
                        let n = if v.is_nan() {
                            i32::MIN
                        } else {
                            v.round_ties_even() as i32
                        };
                        n.clamp(lo - shift, hi - shift) + shift
                    });
                    assert_eq!(float_general(values, (shift, lo, hi)), expected);
                }
            }
        }
    }
}
