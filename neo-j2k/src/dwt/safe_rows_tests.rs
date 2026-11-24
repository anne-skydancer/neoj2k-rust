// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use super::*;
fn compare(width: usize, padding: usize, parity: usize, nonfinite: bool) {
    let stride = width + padding;
    let prefix = 3;
    let tail = 11;
    let sentinel = 0x1357_2468;
    let mut input = vec![sentinel; prefix + 7 * stride + width + tail];
    let mut state = u32::try_from(width).unwrap() + 17;
    let patterns = [
        f32::NEG_INFINITY.to_bits(),
        f32::INFINITY.to_bits(),
        0x7fc0_1234,
        (-0.0f32).to_bits(),
        0,
        1,
        0x7f7f_ffff,
        0xff7f_ffff,
        0x4eff_ffff,
        0xceff_ffff,
    ];
    for row in 0..8 {
        for col in 0..width {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            input[prefix + row * stride + col] = if nonfinite {
                patterns[(row + col) % patterns.len()].cast_signed()
            } else {
                to_sample(((state >> 8) & 65535) as f32 / 256. - 128.)
            };
        }
    }
    let mut expected = input.clone();
    for row in 0..8 {
        inverse_line(
            &mut expected[prefix + row * stride..prefix + row * stride + width],
            1,
            parity,
            false,
            false,
            &mut vec![0; width],
        );
    }
    let mut actual = input.clone();
    let mut scratch = vec![[0x5a5a_5a5a; 8]; width];
    crate::simd::dwt::inverse_97_rows(&mut actual[prefix..], stride, parity, &mut scratch);
    if !nonfinite {
        if let Some(i) = expected.iter().zip(&actual).position(|(a, b)| a != b) {
            panic!(
                "width{width} pad{padding} parity{parity} index{i}: expected{:08x} actual{:08x}",
                expected[i], actual[i]
            );
        }
    } else {
        let finish = crate::mct::Finish::new(31, true, false);
        for row in 0..8 {
            for col in 0..width {
                let i = prefix + row * stride + col;
                assert_eq!(
                    finish.sample(expected[i]),
                    finish.sample(actual[i]),
                    "width{width} parity{parity} row{row} col{col}"
                );
            }
        }
        for i in 0..actual.len() {
            let inside = i >= prefix && i < prefix + 7 * stride + width && (i - prefix) % stride < width;
            if !inside {
                assert_eq!(actual[i], input[i]);
            }
        }
    }
}
#[test]
fn avx2_dense_horizontal_matches_scalar_with_parities_odd_widths_and_padding() {
    let widths: &[usize] = if cfg!(miri) {
        &[16, 17, 32, 33, 64]
    } else {
        &[16, 17, 23, 24, 31, 32, 33, 47, 63, 64, 65, 127, 128, 129, 255, 256, 257]
    };
    for &width in widths {
        for padding in [0, 3] {
            for parity in [0, 1] {
                compare(width, padding, parity, false);
            }
        }
    }
}
#[test]
fn avx2_dense_horizontal_nonfinite_samples_finish_as_scalar_does() {
    for width in [16, 17, 32, 33, 64] {
        for parity in [0, 1] {
            compare(width, 3, parity, true);
        }
    }
}
