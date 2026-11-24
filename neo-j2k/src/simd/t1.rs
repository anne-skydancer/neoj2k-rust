// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Store Tier-1 code-block coefficients in the tile's strided subband.

use super::lanes::*;

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
    store_avx2(coefficients, output, width, stride, reversible, half_step);
    true
}

fn store_avx2(coefficients: &[i32], output: &mut [i32], width: usize, stride: usize, reversible: bool, half_step: f32) {
    if reversible {
        store_rows(
            coefficients,
            output,
            width,
            stride,
            |values| {
                // Add one to negative lanes before shifting so division rounds towards zero.
                arithmetic_right::<1>(add_i32(values, logical_right::<31>(values)))
            },
            |value| value / 2,
        );
    } else {
        let scale = splat_f32(half_step);
        store_rows(
            coefficients,
            output,
            width,
            stride,
            |values| float_bits(mul_f32(to_f32(values), scale)),
            |value| ((value as f32) * half_step).to_bits().cast_signed(),
        );
    }
}

fn store_rows(
    coefficients: &[i32],
    output: &mut [i32],
    width: usize,
    stride: usize,
    convert: impl Fn(IntLanes) -> IntLanes,
    convert_tail: impl Fn(i32) -> i32,
) {
    for (row, input) in coefficients.chunks_exact(width).enumerate() {
        let start = row * stride;
        let (input, input_tail) = input.as_chunks::<LANES>();
        let (output, output_tail) = output[start..start + width].as_chunks_mut::<LANES>();
        for (source, target) in input.iter().zip(output) {
            let values = *source;
            let values = convert(values);
            *target = values;
        }
        for (&source, target) in input_tail.iter().zip(output_tail) {
            *target = convert_tail(source);
        }
    }
}
