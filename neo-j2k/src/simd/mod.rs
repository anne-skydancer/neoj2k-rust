// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#[cfg(target_feature = "avx2")]
pub(super) mod dwt;
mod lanes;
pub(super) mod mct;
#[cfg(target_feature = "avx2")]
pub(super) mod output;
#[cfg(target_feature = "avx2")]
pub(super) mod t1;

/// Round to nearest even in the default floating-point environment.
/// NaN and an unrepresentable result produce i32::MIN.
#[inline]
#[allow(clippy::cast_possible_truncation, reason = "rounded and range checked")]
pub(super) fn round_i32(value: f32) -> i32 {
    if (-4_194_304.0..=4_194_304.0).contains(&value) {
        return (value + 12_582_912.0).to_bits().wrapping_sub(0x4b40_0000).cast_signed();
    }
    round_general(value)
}

// Exceptional quantizer values and wider-precision decoded samples use the same
// conversion rules without replicating the general path in every gathered lane.
#[cold]
#[inline(never)]
#[allow(clippy::cast_possible_truncation, reason = "rounded and range checked")]
fn round_general(value: f32) -> i32 {
    // Values with magnitude at least 2^23 have no fractional bits. In the
    // remaining outer interval a signed 2^23 bias removes the fraction.
    let rounded = if value.abs() < 8_388_608.0 {
        let bias = f32::from_bits(0x4b00_0000 | (value.to_bits() & 0x8000_0000));
        (value + bias) - bias
    } else {
        value
    };
    if (-2_147_483_648.0..2_147_483_648.0).contains(&rounded) {
        rounded as i32
    } else {
        i32::MIN
    }
}

#[cfg(test)]
mod round_tests;
