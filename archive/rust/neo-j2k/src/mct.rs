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

/// Transform and finish three complete, equally sized component planes in place.
pub(crate) fn inverse_and_finish([y, u, v]: [&mut [i32]; 3], reversible: bool, finishes: [Finish; 3]) {
    debug_assert!(y.len() == u.len() && u.len() == v.len());
    if finishes.iter().any(|finish| finish.reversible != reversible) {
        inverse([y, u, v], reversible);
        for (finish, plane) in finishes.into_iter().zip([y, u, v]) {
            finish.apply(plane);
        }
        return;
    }

    #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
    let start = vector::inverse_and_finish(
        [y, u, v],
        reversible,
        finishes.map(|finish| (finish.shift, finish.lo, finish.hi)),
    );
    #[cfg(not(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2")))]
    let start = 0;
    let tail = [&mut y[start..], &mut u[start..], &mut v[start..]];
    if reversible {
        inverse_and_finish_tail::<true>(tail, finishes);
    } else {
        inverse_and_finish_tail::<false>(tail, finishes);
    }
}

fn inverse_and_finish_tail<const REVERSIBLE: bool>([y, u, v]: [&mut [i32]; 3], [red, green, blue]: [Finish; 3]) {
    for ((y, u), v) in y.iter_mut().zip(u).zip(v) {
        let samples = [*y, *u, *v];
        let [r, g, b] = if REVERSIBLE {
            inverse_reversible(samples)
        } else {
            inverse_irreversible(samples)
        };
        *y = red.sample_with::<REVERSIBLE>(r);
        *u = green.sample_with::<REVERSIBLE>(g);
        *v = blue.sample_with::<REVERSIBLE>(b);
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
        if self.reversible {
            self.sample_with::<true>(sample)
        } else {
            self.sample_with::<false>(sample)
        }
    }

    #[inline]
    fn sample_with<const REVERSIBLE: bool>(self, sample: i32) -> i32 {
        let integer = if REVERSIBLE {
            sample
        } else {
            round_sample(f32::from_bits(sample.cast_unsigned()))
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
            let (lo, hi) = (self.lo - self.shift, self.hi - self.shift);
            for sample in &mut line[start..] {
                let value = f32::from_bits(sample.cast_unsigned());
                *sample = round_sample(value).clamp(lo, hi) + self.shift;
            }
        }
    }
}

#[inline]
fn round_sample(value: f32) -> i32 {
    if value >= 2_147_483_648.0 {
        return i32::MAX;
    }
    #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "sse"))]
    let integer = crate::simd::round_i32(value);
    #[cfg(not(all(feature = "simd", target_arch = "x86_64", target_feature = "sse")))]
    #[allow(
        clippy::cast_possible_truncation,
        reason = "the saturating cast is on a rounded value"
    )]
    let integer = if value.is_nan() {
        i32::MIN
    } else {
        value.round_ties_even() as i32
    };
    integer
}

#[cfg(test)]
#[test]
fn irreversible_finishing_matches_round_to_even_across_float_exponents() {
    let finishes = [Finish::new(8, false, false), Finish::new(31, true, false)];
    for sign in [0, 1 << 31] {
        for exponent in 0..=255 {
            for fraction in [0, 1, 0x3f_ffff, 0x40_0000, 0x40_0001, 0x7f_fffe, 0x7f_ffff] {
                let bits = sign | (exponent << 23) | fraction;
                let value = f32::from_bits(bits);
                for finish in finishes {
                    let expected = if value >= 2_147_483_648.0 {
                        finish.hi
                    } else if value < -2_147_483_648.0 || value.is_nan() {
                        finish.lo
                    } else {
                        #[allow(clippy::cast_possible_truncation, reason = "it is a range-checked integer")]
                        let rounded = value.round_ties_even() as i32;
                        rounded.clamp(finish.lo - finish.shift, finish.hi - finish.shift) + finish.shift
                    };
                    let mut sample = [bits.cast_signed()];
                    finish.apply(&mut sample);
                    assert_eq!(sample[0], expected, "float bits {bits:08x}");
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn avx2_and_scalar_fused_colour_finishing_match_staged_output() {
    let float_bits = [
        f32::NAN.to_bits(),
        0x7fa0_0001,
        f32::NEG_INFINITY.to_bits(),
        f32::INFINITY.to_bits(),
        (-2_147_483_648.0f32).to_bits(),
        2_147_483_648.0f32.to_bits(),
        2_147_483_520.0f32.to_bits(),
        (-128.5f32).to_bits(),
        (-1.5f32).to_bits(),
        (-0.5f32).to_bits(),
        0.5f32.to_bits(),
        1.5f32.to_bits(),
        127.5f32.to_bits(),
        (-0.0f32).to_bits(),
        0.0f32.to_bits(),
        f32::MAX.to_bits(),
        f32::MIN_POSITIVE.to_bits(),
        1,
    ];
    let integers = [i32::MIN, i32::MAX, -1_073_741_824, 1_073_741_824, -32769, 255, 1, 0, -1];
    for reversible in [true, false] {
        for mixed in [false, true] {
            // Invalid mixed transforms can expose an ICT NaN payload as an integer.
            // Use bounded finite inputs for that fallback comparison.
            let bits = if mixed { &float_bits[7..15] } else { &float_bits[..] };
            let original: [[i32; 37]; 3] = core::array::from_fn(|plane| {
                core::array::from_fn(|sample| {
                    let index = plane * 7 + sample;
                    if reversible {
                        integers[index % integers.len()]
                    } else {
                        bits[index % bits.len()].cast_signed()
                    }
                })
            });
            for len in [0, 1, 7, 8, 9, 19, 37] {
                let finishes = [
                    Finish::new(31, false, reversible),
                    Finish::new(8, true, reversible ^ mixed),
                    Finish::new(1, false, reversible),
                ];
                let [mut r, mut g, mut b] = original;
                inverse([&mut r[..len], &mut g[..len], &mut b[..len]], reversible);
                for (finish, plane) in finishes.into_iter().zip([&mut r, &mut g, &mut b]) {
                    finish.apply(&mut plane[..len]);
                }

                let [mut y, mut u, mut v] = original;
                inverse_and_finish([&mut y[..len], &mut u[..len], &mut v[..len]], reversible, finishes);
                assert_eq!([y, u, v], [r, g, b]);
                for i in 0..len {
                    let samples = original.each_ref().map(|plane| plane[i]);
                    let rgb = if reversible {
                        inverse_reversible(samples)
                    } else {
                        inverse_irreversible(samples)
                    };
                    assert_eq!([y[i], u[i], v[i]], core::array::from_fn(|c| finishes[c].sample(rgb[c])));
                }
            }
        }
    }
}

#[cfg(all(test, feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
mod avx2_tests {
    use super::*;

    #[test]
    fn avx2_colour_and_finishing_match_scalar_with_tails() {
        for reversible in [true, false] {
            let original: [[i32; 19]; 3] = std::array::from_fn(|plane| {
                std::array::from_fn(|sample| {
                    let value = i16::try_from(plane * 16 + sample).unwrap() - 20;
                    if reversible {
                        i32::from(value).wrapping_mul(1 << 28)
                    } else {
                        (f32::from(value) * 19.5).to_bits().cast_signed()
                    }
                })
            });
            let [mut y, mut u, mut v] = original;
            inverse([&mut y, &mut u, &mut v], reversible);
            let transform = if reversible {
                inverse_reversible
            } else {
                inverse_irreversible
            };
            for (index, actual) in y.iter().zip(&u).zip(&v).enumerate() {
                let ((&y, &u), &v) = actual;
                assert_eq!([y, u, v], transform(original.each_ref().map(|plane| plane[index])));
            }
            for signed in [false, true] {
                let mut values = y;
                let finish = Finish::new(8, signed, reversible);
                finish.apply(&mut values);
                assert_eq!(values, y.map(|sample| finish.sample(sample)));
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

    #[test]
    #[ignore = "exhaustive sweep of all 32-bit floating-point encodings"]
    fn irreversible_shift_matches_cpp_for_all_float_bits() {
        let mut input = vec![0; 1 << 16];
        let mut actual = input.clone();
        let mut expected = input.clone();
        let finishes = [Finish::new(8, false, false), Finish::new(31, true, false)];
        for high in 0..=u16::MAX {
            let upper = i32::from(high) << 16;
            for (low, bits) in (0..=u16::MAX).zip(&mut input) {
                *bits = upper | i32::from(low);
            }
            for finish in finishes {
                actual.copy_from_slice(&input);
                expected.copy_from_slice(&input);
                finish.apply(&mut actual);
                kernels::dc_shift_97_scalar(&mut expected, finish.shift, finish.lo, finish.hi);
                if actual != expected {
                    let index = actual.iter().zip(&expected).position(|(a, b)| a != b).unwrap();
                    panic!(
                        "float bits {:08x}, clamp {}..={}: Rust {}, C++ {}",
                        input[index], finish.lo, finish.hi, actual[index], expected[index]
                    );
                }
            }
        }
    }
}
