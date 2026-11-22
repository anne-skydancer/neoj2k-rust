// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Store Tier-1 code-block coefficients in the tile's strided subband.

use core::arch::x86_64::{
    __m256i, _mm256_add_epi32, _mm256_castps_si256, _mm256_cvtepi32_ps, _mm256_loadu_si256, _mm256_mul_ps,
    _mm256_set1_ps, _mm256_srai_epi32, _mm256_srli_epi32, _mm256_storeu_si256,
};

const LANES: usize = 8;

/// Store the block when each row contains at least one group of eight samples.
pub(crate) fn store(
    coefficients: &[i32],
    output: &mut [i32],
    width: usize,
    stride: usize,
    reversible: bool,
    half_step: f32,
) -> bool {
    if width < LANES {
        return false;
    }
    // SAFETY: this module is compiled only when the target enables AVX2.
    unsafe { store_avx2(coefficients, output, width, stride, reversible, half_step) };
    true
}

#[target_feature(enable = "avx2")]
fn store_avx2(coefficients: &[i32], output: &mut [i32], width: usize, stride: usize, reversible: bool, half_step: f32) {
    if reversible {
        store_rows(
            coefficients,
            output,
            width,
            stride,
            |values| {
                // Add one to negative lanes before shifting so division rounds towards zero.
                _mm256_srai_epi32::<1>(_mm256_add_epi32(values, _mm256_srli_epi32::<31>(values)))
            },
            |value| value / 2,
        );
    } else {
        let scale = _mm256_set1_ps(half_step);
        store_rows(
            coefficients,
            output,
            width,
            stride,
            |values| _mm256_castps_si256(_mm256_mul_ps(_mm256_cvtepi32_ps(values), scale)),
            |value| ((value as f32) * half_step).to_bits().cast_signed(),
        );
    }
}

#[target_feature(enable = "avx2")]
fn store_rows(
    coefficients: &[i32],
    output: &mut [i32],
    width: usize,
    stride: usize,
    convert: impl Fn(__m256i) -> __m256i,
    convert_tail: impl Fn(i32) -> i32,
) {
    for (row, input) in coefficients.chunks_exact(width).enumerate() {
        let start = row * stride;
        let (input, input_tail) = input.as_chunks::<LANES>();
        let (output, output_tail) = output[start..start + width].as_chunks_mut::<LANES>();
        for (source, target) in input.iter().zip(output) {
            // SAFETY: each source array has 32 initialized bytes, and the load allows any alignment.
            let values = unsafe { _mm256_loadu_si256(source.as_ptr().cast()) };
            let values = convert(values);
            // SAFETY: each target array has 32 writable bytes, and the store allows any alignment.
            unsafe { _mm256_storeu_si256(target.as_mut_ptr().cast(), values) };
        }
        for (&source, target) in input_tail.iter().zip(output_tail) {
            *target = convert_tail(source);
        }
    }
}
