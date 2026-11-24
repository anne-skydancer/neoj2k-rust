// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use super::{inverse_and_finish, level_shift};

fn bounds(precision: u32, signed: bool) -> (i32, i32, i32) {
    let half = 1_i32 << (precision - 1);
    if signed {
        (0, -half, half - 1)
    } else {
        (half, 0, (half - 1) * 2 + 1)
    }
}

#[allow(
    clippy::cast_possible_truncation,
    reason = "independent reference saturates after rounding"
)]
fn finish(value: f32, (shift, low, high): (i32, i32, i32)) -> i32 {
    let rounded = if value.is_nan() {
        i32::MIN
    } else {
        value.round_ties_even() as i32
    };
    rounded.clamp(low - shift, high - shift) + shift
}

#[test]
fn avx2_wide_colour_matches_reference_with_mixed_finishes_and_untouched_tails() {
    let patterns = [
        0,
        0x8000_0000,
        1,
        0x8000_0001,
        0x3f00_0000,
        0xbf00_0000,
        0x3fc0_0000,
        0xbfc0_0000,
        0x4a7f_fffe,
        0xca7f_fffe,
        0x4aff_ffff,
        0xcaff_ffff,
        0x4b00_0000,
        0xcb00_0000,
        0x4eff_ffff,
        0xceff_ffff,
        0x4f00_0000,
        0xcf00_0000,
        0x7f80_0000,
        0xff80_0000,
        0x7fc0_1234,
        0xffa0_1234,
        0x7f7f_ffff,
        0xff7f_ffff,
    ];
    let precisions: Vec<u32> = if cfg!(miri) {
        vec![8, 24, 31]
    } else {
        (1..=31).collect()
    };
    let widths: &[usize] = if cfg!(miri) {
        &[0, 7, 8, 9, 25]
    } else {
        &[0, 1, 7, 8, 9, 15, 16, 17, 25, 33]
    };
    let mut state = 0x7153_2849_u32;
    for precision in precisions {
        for finishes in [
            [bounds(precision, true); 3],
            [bounds(precision, false); 3],
            [
                bounds(precision, false),
                bounds(precision, true),
                bounds(precision.min(24), false),
            ],
        ] {
            for &width in widths {
                for coloured in [false, true] {
                    let prefix = 3;
                    let mut actual = [[0x1357_2468_i32; 48]; 3];
                    for (channel, plane) in actual.iter_mut().enumerate() {
                        for (index, sample) in plane[prefix..prefix + width + 3].iter_mut().enumerate() {
                            state ^= state << 13;
                            state ^= state >> 17;
                            state ^= state << 5;
                            let pattern = index + channel * 7;
                            let bits = if !coloured && channel != 0 {
                                0
                            } else if pattern < patterns.len() {
                                patterns[pattern]
                            } else {
                                state
                            };
                            *sample = bits.cast_signed();
                        }
                    }
                    let count = width / 8 * 8;
                    let mut expected = actual;
                    for index in prefix..prefix + count {
                        let [y, u, v] = actual.map(|plane| f32::from_bits(plane[index].cast_unsigned()));
                        let transformed = [y + v * 1.402, (y - u * 0.34413) - v * 0.71414, y + u * 1.772];
                        for channel in 0..3 {
                            expected[channel][index] = finish(transformed[channel], finishes[channel]);
                        }
                    }
                    let [y, u, v] = &mut actual;
                    assert_eq!(
                        inverse_and_finish(
                            [
                                &mut y[prefix..prefix + width],
                                &mut u[prefix..prefix + width + 1],
                                &mut v[prefix..prefix + width + 3],
                            ],
                            false,
                            finishes
                        ),
                        count,
                    );
                    assert_eq!(
                        actual, expected,
                        "precision{precision} width{width} coloured{coloured} finishes{finishes:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn avx2_wide_level_shift_matches_reference_with_untouched_tails() {
    let patterns = [
        0,
        0x8000_0000,
        1,
        0x8000_0001,
        0x3f00_0000,
        0xbf00_0000,
        0x3fc0_0000,
        0xbfc0_0000,
        0x4aff_ffff,
        0xcaff_ffff,
        0x4b00_0000,
        0xcb00_0000,
        0x4eff_ffff,
        0xceff_ffff,
        0x4f00_0000,
        0xcf00_0000,
        0x7f80_0000,
        0xff80_0000,
        0x7fc0_1234,
        0xffa0_1234,
        0x7f7f_ffff,
        0xff7f_ffff,
    ];
    let precisions: Vec<u32> = if cfg!(miri) {
        vec![8, 24, 31]
    } else {
        (1..=31).collect()
    };
    let widths: &[usize] = if cfg!(miri) {
        &[0, 7, 8, 9, 33]
    } else {
        &[0, 1, 7, 8, 9, 15, 16, 17, 31, 32, 33]
    };
    let mut state = 0x9137_4265_u32;
    for precision in precisions {
        for signed in [false, true] {
            let parameters = bounds(precision, signed);
            for &width in widths {
                let prefix = 3;
                let mut actual = [0x1357_2468_i32; 48];
                for (index, sample) in actual[prefix..prefix + width].iter_mut().enumerate() {
                    state ^= state << 13;
                    state ^= state >> 17;
                    state ^= state << 5;
                    let bits = patterns.get(index).copied().unwrap_or(state);
                    *sample = bits.cast_signed();
                }
                let count = width / 8 * 8;
                let mut expected = actual;
                for sample in &mut expected[prefix..prefix + count] {
                    *sample = finish(f32::from_bits(sample.cast_unsigned()), parameters);
                }
                let (shift, low, high) = parameters;
                assert_eq!(
                    level_shift(&mut actual[prefix..prefix + width], shift, low, high, false),
                    count,
                );
                assert_eq!(actual, expected, "precision{precision} signed{signed} width{width}");
            }
        }
    }
}
