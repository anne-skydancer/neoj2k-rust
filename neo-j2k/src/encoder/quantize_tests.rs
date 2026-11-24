// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use super::{quantize, quantize_four};

fn check(values: [f32; 4], step: f32) {
    let samples = values.map(|v| v.to_bits().cast_signed());
    assert_eq!(
        quantize_four(samples, step),
        samples.map(|sample| quantize(sample, step)),
        "sample bits {:?}, step bits {:08x}",
        samples.map(i32::cast_unsigned),
        step.to_bits()
    );
}

#[test]
fn grouped_quantization_matches_scalar_at_rounding_and_range_boundaries() {
    for step in [
        1.0f32,
        64.0,
        0.1,
        -1.0,
        0.0,
        -0.0,
        f32::MIN_POSITIVE,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ] {
        for value in [
            0.0f32,
            0.5,
            1.5,
            2.5,
            65_536.0,
            4_194_304.0,
            8_388_608.0,
            2_147_483_648.0,
            f32::MIN_POSITIVE,
            f32::MAX,
            f32::INFINITY,
            f32::NAN,
        ] {
            check([value.next_down(), value, value.next_up(), -value], step);
            // The exceptional fourth lane must force the shared fallback without changing the others.
            check([0.0, 0.5, -1.5, value], step);
        }
    }
}

#[test]
fn grouped_quantization_matches_scalar_across_float_bit_patterns() {
    let mut state = 0xba39_4d17u32;
    for _ in 0..if cfg!(miri) { 16 } else { 4_096 } {
        let values = core::array::from_fn(|_| {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            f32::from_bits(state)
        });
        for step in [0.25, 1.0, 64.0] {
            check(values, step);
        }
    }
}
