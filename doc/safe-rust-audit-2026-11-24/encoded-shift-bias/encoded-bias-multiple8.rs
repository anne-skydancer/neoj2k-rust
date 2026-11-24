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
    } else if cfg!(target_feature = "avx2") && finishes[0] == finishes[1] && finishes[1] == finishes[2] {
        let finish = finishes[0];
        inverse_and_finish_wide_rows(
            ys.as_flattened_mut(),
            us.as_flattened_mut(),
            vs.as_flattened_mut(),
            finish,
        );
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
    } else if cfg!(target_feature = "avx2") {
        if shift == 0 {
            level_shift_typed::<true>(blocks.as_flattened_mut(), shift, lo, hi);
        } else {
            level_shift_typed::<false>(blocks.as_flattened_mut(), shift, lo, hi);
        }
    } else {
        for block in blocks {
            *block = float_general(from_float_bits(*block), (shift, lo, hi));
        }
    }
    count
}
#[inline(never)]
fn level_shift_typed<const SIGNED: bool>(line: &mut [i32], given_shift: i32, lo: i32, hi: i32) {
    let shift = if SIGNED { 0 } else { given_shift };
    let lower = (lo - shift) as f32;
    let upper = (hi - shift) as f32;
    let high_bias = f32::from_bits(0x5000_0000 | (shift.cast_unsigned().wrapping_sub(0x4b40_0000) >> 10));
    let combined_bias = high_bias + 12_582_912.0;
    for v in line {
        *v = round_wide_lane::<SIGNED>(f32::from_bits(v.cast_unsigned()), lower, upper, high_bias, combined_bias, hi);
    }
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

// Finish bounds normalize to [-2^(p-1),2^(p-1)-1], p<=31. The lower
// endpoint is exact in f32; the upper may round up by one, corrected by the integer cap.
// The high bias rounds to a multiple of256 and its exact residual is at most128.
// Translating by an even multiple preserves nearest-even ties in the low bias.
// The combined bias below is exactly the sum of the high and low biases. Its
// difference from `high` is exact, so adding v rounds the same low residual
// with one fewer floating-point operation.
#[inline(always)]
fn round_wide_lane<const SIGNED: bool>(v: f32, lower: f32, upper: f32, high_bias: f32, combined_bias: f32, hi: i32) -> i32 {
    let v = if v > lower { v } else { lower };
    let v = if v < upper { v } else { upper };
    let high = v + high_bias;
    // BIG+SMALL is exact; this difference is exact by Sterbenz's lemma.
    let low = v + (combined_bias - high);
    let encoded = (high.to_bits() << 10).wrapping_add(low.to_bits());
    // Signed normalized values lie within +/-2^30, so their encoded result fits
    // i32. Unsigned values are capped before conversion, including the rounded-up
    // f32 representation of a 31-bit upper endpoint.
    if SIGNED {
        encoded.cast_signed().min(hi)
    } else {
        encoded.min(hi.cast_unsigned()).cast_signed()
    }
}

#[inline(always)]
fn inverse_and_finish_wide_rows(y: &mut [i32], u: &mut [i32], v: &mut [i32], finish: (i32, i32, i32)) {
    if finish.0 == 0 {
        inverse_and_finish_typed_rows::<true>(y, u, v, finish);
    } else {
        inverse_and_finish_typed_rows::<false>(y, u, v, finish);
    }
}

// Separate mutable rows preserve alias information. Iterating individual pixels lets
// LLVM vectorize consecutive samples rather than widening eight strided lane streams.
#[inline(never)]
fn inverse_and_finish_typed_rows<const SIGNED: bool>(
    ys: &mut [i32],
    us: &mut [i32],
    vs: &mut [i32],
    (given_shift, lo, hi): (i32, i32, i32),
) {
    let ys=ys.as_chunks_mut::<LANES>().0.as_flattened_mut();
    let us=us.as_chunks_mut::<LANES>().0.as_flattened_mut();
    let vs=vs.as_chunks_mut::<LANES>().0.as_flattened_mut();
    let shift = if SIGNED { 0 } else { given_shift };
    let lower = (lo - shift) as f32;
    let upper = (hi - shift) as f32;
    let high_bias = f32::from_bits(0x5000_0000 | (shift.cast_unsigned().wrapping_sub(0x4b40_0000) >> 10));
    let combined_bias = high_bias + 12_582_912.0;
    for ((a, b), c) in ys.iter_mut().zip(us).zip(vs) {
        let y = f32::from_bits(a.cast_unsigned());
        let u = f32::from_bits(b.cast_unsigned());
        let v = f32::from_bits(c.cast_unsigned());
        *a = round_wide_lane::<SIGNED>(y + v * 1.402, lower, upper, high_bias, combined_bias, hi);
        *b = round_wide_lane::<SIGNED>((y - u * 0.34413) - v * 0.71414, lower, upper, high_bias, combined_bias, hi);
        *c = round_wide_lane::<SIGNED>(y + u * 1.772, lower, upper, high_bias, combined_bias, hi);
    }
}

#[cfg(test)]
mod wide_tests;

pub(crate) fn verify_wide(value: FloatLanes, (shift,lo,hi):(i32,i32,i32))->Block {
    assert_eq!(shift & 1023,0);
    let lower=(lo-shift) as f32; let upper=(hi-shift) as f32;
    let high_bias=f32::from_bits(0x5000_0000 | (shift.cast_unsigned().wrapping_sub(0x4b40_0000) >> 10));
    let combined_bias=high_bias+12_582_912.0;
    if shift==0 {value.map(|v|round_wide_lane::<true>(v,lower,upper,high_bias,combined_bias,hi))}
    else {value.map(|v|round_wide_lane::<false>(v,lower,upper,high_bias,combined_bias,hi))}
}
