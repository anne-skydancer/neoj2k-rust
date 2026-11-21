// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use core::arch::x86_64::*;

const LANES: usize = 8;
type Block = [i32; LANES];

#[target_feature(enable = "avx2")]
fn read(block: &Block) -> __m256i {
    // SAFETY: a block occupies 32 initialized bytes, and the load permits unaligned addresses.
    unsafe { _mm256_loadu_si256(block.as_ptr().cast()) }
}

#[target_feature(enable = "avx2")]
fn write(block: &mut Block, values: __m256i) {
    // SAFETY: each block has 32 exclusive writable bytes, and the store permits unaligned addresses.
    unsafe { _mm256_storeu_si256(block.as_mut_ptr().cast(), values) };
}

pub(crate) fn inverse(planes: [&mut [i32]; 3], reversible: bool) -> usize {
    // SAFETY: the module is compiled only when the target enables AVX2.
    unsafe { inverse_avx2(planes, reversible) }
}

#[target_feature(enable = "avx2")]
fn inverse_avx2([y, u, v]: [&mut [i32]; 3], reversible: bool) -> usize {
    let (ys, _) = y.as_chunks_mut::<LANES>();
    let (us, _) = u.as_chunks_mut::<LANES>();
    let (vs, _) = v.as_chunks_mut::<LANES>();
    let count = ys.len().min(us.len()).min(vs.len()) * LANES;
    if reversible {
        for ((a, b), c) in ys.iter_mut().zip(us).zip(vs) {
            let [y, u, v] = [read(a), read(b), read(c)];
            let g = _mm256_sub_epi32(y, _mm256_srai_epi32::<2>(_mm256_add_epi32(u, v)));
            write(a, _mm256_add_epi32(v, g));
            write(b, g);
            write(c, _mm256_add_epi32(u, g));
        }
    } else {
        let r = _mm256_set1_ps(1.402);
        let g1 = _mm256_set1_ps(0.34413);
        let g2 = _mm256_set1_ps(0.71414);
        let b = _mm256_set1_ps(1.772);
        for ((a, b_out), c) in ys.iter_mut().zip(us).zip(vs) {
            let y = _mm256_castsi256_ps(read(a));
            let u = _mm256_castsi256_ps(read(b_out));
            let v = _mm256_castsi256_ps(read(c));
            let red = _mm256_add_ps(y, _mm256_mul_ps(v, r));
            let green = _mm256_sub_ps(_mm256_sub_ps(y, _mm256_mul_ps(u, g1)), _mm256_mul_ps(v, g2));
            let blue = _mm256_add_ps(y, _mm256_mul_ps(u, b));
            write(a, _mm256_castps_si256(red));
            write(b_out, _mm256_castps_si256(green));
            write(c, _mm256_castps_si256(blue));
        }
    }
    count
}

pub(crate) fn level_shift(line: &mut [i32], shift: i32, lo: i32, hi: i32, reversible: bool) -> usize {
    // SAFETY: the module is compiled only when the target enables AVX2.
    unsafe { level_shift_avx2(line, shift, lo, hi, reversible) }
}

#[target_feature(enable = "avx2")]
fn level_shift_avx2(line: &mut [i32], shift: i32, lo: i32, hi: i32, reversible: bool) -> usize {
    let (blocks, _) = line.as_chunks_mut::<LANES>();
    let count = blocks.len() * LANES;
    let lower = _mm256_set1_epi32(lo - shift);
    let upper = _mm256_set1_epi32(hi - shift);
    let bias = _mm256_set1_epi32(shift);
    let apply = |value| _mm256_add_epi32(_mm256_min_epi32(_mm256_max_epi32(value, lower), upper), bias);
    if reversible {
        for block in blocks {
            write(block, apply(read(block)));
        }
    } else {
        let overflow = _mm256_set1_ps(2_147_483_648.0);
        let max_sample = _mm256_set1_epi32(hi);
        for block in blocks {
            let value = _mm256_castsi256_ps(read(block));
            let rounded = _mm256_round_ps::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(value);
            // The integer conversion yields INT_MIN for NaN and both overflow signs. Preserve
            // the positive overflow case separately, since that clamps to the other endpoint.
            let integer = _mm256_cvttps_epi32(rounded);
            let high = _mm256_castps_si256(_mm256_cmp_ps::<_CMP_GE_OQ>(value, overflow));
            write(block, _mm256_blendv_epi8(apply(integer), max_sample, high));
        }
    }
    count
}
