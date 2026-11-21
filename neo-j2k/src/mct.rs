// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The sample arithmetic is from OpenJPEG's mct.c and tcd.c through src/mct.h.
// See LICENSE.OpenJPEG for OpenJPEG's copyright notices and licence.

//! Inverse component transforms and sample finishing.

#[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
use crate::simd::mct as vector;

/// Reversible inverse colour transform. Intermediate values wrap as int32.
#[inline]
pub(crate) fn inverse_reversible([y, u, v]: [i32; 3]) -> [i32; 3] {
    let g = y.wrapping_sub(u.wrapping_add(v) >> 2);
    [v.wrapping_add(g), g, u.wrapping_add(g)]
}

/// Irreversible inverse colour transform with float samples stored as their bit patterns.
#[inline]
pub(crate) fn inverse_irreversible(samples: [i32; 3]) -> [i32; 3] {
    let [y, u, v] = samples.map(|sample| f32::from_bits(sample.cast_unsigned()));
    [y + v * 1.402, y - u * 0.34413 - v * 0.71414, y + u * 1.772].map(|sample| sample.to_bits().cast_signed())
}

pub(crate) fn inverse([y, u, v]: [&mut [i32]; 3], reversible: bool) {
    #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
    let start = vector::inverse([y, u, v], reversible);
    #[cfg(not(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2")))]
    let start = 0;
    let lanes = y[start..].iter_mut().zip(&mut u[start..]).zip(&mut v[start..]);
    if reversible {
        for ((y, u), v) in lanes {
            [*y, *u, *v] = inverse_reversible([*y, *u, *v]);
        }
    } else {
        for ((y, u), v) in lanes {
            [*y, *u, *v] = inverse_irreversible([*y, *u, *v]);
        }
    }
}

/// DC level shift and clamp for one decoded component.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Finish {
    shift: i32,
    lo: i32,
    hi: i32,
    reversible: bool,
}

impl Finish {
    pub(crate) fn new(precision: u8, signed: bool, reversible: bool) -> Self {
        debug_assert!((1..=31).contains(&precision));
        let half = 1_i32 << (precision - 1);
        let (shift, lo, hi) = if signed {
            (0, -half, half - 1)
        } else {
            (half, 0, (half - 1) + half)
        };
        Self {
            shift,
            lo,
            hi,
            reversible,
        }
    }

    #[inline]
    pub(crate) fn sample(self, sample: i32) -> i32 {
        let integer = if self.reversible {
            sample
        } else {
            let value = f32::from_bits(sample.cast_unsigned());
            if value >= 2_147_483_648.0 {
                return self.hi;
            }
            if value < -2_147_483_648.0 || value.is_nan() {
                return self.lo;
            }
            #[allow(clippy::cast_possible_truncation, reason = "the float was range checked and rounded")]
            let rounded = value.round_ties_even() as i32;
            rounded
        };
        integer.clamp(self.lo - self.shift, self.hi - self.shift) + self.shift
    }

    pub(crate) fn apply(self, line: &mut [i32]) {
        #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
        let start = vector::level_shift(line, self.shift, self.lo, self.hi, self.reversible);
        #[cfg(not(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2")))]
        let start = 0;
        if self.reversible {
            let (lo, hi) = (self.lo - self.shift, self.hi - self.shift);
            for sample in &mut line[start..] {
                *sample = (*sample).clamp(lo, hi) + self.shift;
            }
        } else {
            for sample in &mut line[start..] {
                *sample = self.sample(*sample);
            }
        }
    }
}

#[cfg(all(test, not(miri)))]
mod tests {
    use super::*;
    use neo_j2k_oracle::kernels;

    #[test]
    fn colour_transforms_match_cpp_precision_and_wraparound() {
        let y = [127_i32, 0, i32::MAX, i32::MIN, -40000];
        let u = [-128, i32::MIN, i32::MAX, 13, 56];
        let v = [127, i32::MIN, 1, i32::MAX, -2400];
        let expected = kernels::rct_scalar([&y, &u, &v]);
        for (index, ((&y, &u), &v)) in y.iter().zip(&u).zip(&v).enumerate() {
            assert_eq!(inverse_reversible([y, u, v]), expected.each_ref().map(|row| row[index]));
        }

        let y = [0.0_f32, -0.0, -128.125, 127.125, 30.5, 0.0001];
        let u = [-0.0_f32, 0.0, -15.0, 31.25, -80.2, 0.3];
        let v = [0.0_f32, -0.0, 62.0, -25.3, 3.25, -0.2];
        let [y, u, v] = [y, u, v].map(|row| row.map(|f| f.to_bits().cast_signed()));
        let expected = kernels::ict_scalar([&y, &u, &v]);
        for (index, ((&y, &u), &v)) in y.iter().zip(&u).zip(&v).enumerate() {
            assert_eq!(
                inverse_irreversible([y, u, v]),
                expected.each_ref().map(|row| row[index])
            );
        }
    }

    #[test]
    fn dc_shift_matches_cpp_on_limits_and_ties() {
        let input = [i32::MIN, i32::MAX, -129, -128, 0, 127, 128];
        let mut expected = input;
        kernels::dc_shift_53_scalar(&mut expected, 128, 0, 255);
        assert_eq!(input.map(|sample| Finish::new(8, false, true).sample(sample)), expected);

        let input = [
            f32::NAN,
            f32::NEG_INFINITY,
            f32::INFINITY,
            -2_147_483_648.0,
            2_147_483_648.0,
            -129.5,
            -128.5,
            -1.5,
            -0.5,
            0.5,
            1.5,
            126.5,
            127.5,
        ]
        .map(|sample| sample.to_bits().cast_signed());
        for (signed, shift, lo, hi) in [(false, 128, 0, 255), (true, 0, -128, 127)] {
            let mut expected = input;
            kernels::dc_shift_97_scalar(&mut expected, shift, lo, hi);
            let finish = Finish::new(8, signed, false);
            assert_eq!(input.map(|sample| finish.sample(sample)), expected);
            let mut actual = input;
            finish.apply(&mut actual);
            assert_eq!(actual, expected);
        }
    }
}
