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

#[target_feature(enable = "avx2")]
fn inverse_reversible([a, b, c]: [&Block; 3]) -> [__m256i; 3] {
    let [y, u, v] = [read(a), read(b), read(c)];
    let g = _mm256_sub_epi32(y, _mm256_srai_epi32::<2>(_mm256_add_epi32(u, v)));
    [_mm256_add_epi32(v, g), g, _mm256_add_epi32(u, g)]
}

struct Ict {
    red: __m256,
    green_u: __m256,
    green_v: __m256,
    blue: __m256,
}

impl Ict {
    #[target_feature(enable = "avx2")]
    fn new() -> Self {
        Self {
            red: _mm256_set1_ps(1.402),
            green_u: _mm256_set1_ps(0.34413),
            green_v: _mm256_set1_ps(0.71414),
            blue: _mm256_set1_ps(1.772),
        }
    }

    #[target_feature(enable = "avx2")]
    fn apply(&self, [a, b, c]: [&Block; 3]) -> [__m256; 3] {
        let [y, u, v] = [a, b, c].map(|block| _mm256_castsi256_ps(read(block)));
        [
            _mm256_add_ps(y, _mm256_mul_ps(v, self.red)),
            _mm256_sub_ps(
                _mm256_sub_ps(y, _mm256_mul_ps(u, self.green_u)),
                _mm256_mul_ps(v, self.green_v),
            ),
            _mm256_add_ps(y, _mm256_mul_ps(u, self.blue)),
        ]
    }
}

struct LevelShift {
    lower: __m256i,
    upper: __m256i,
    bias: __m256i,
    max_sample: __m256i,
}

impl LevelShift {
    #[target_feature(enable = "avx2")]
    fn new((shift, lo, hi): (i32, i32, i32)) -> Self {
        Self {
            lower: _mm256_set1_epi32(lo - shift),
            upper: _mm256_set1_epi32(hi - shift),
            bias: _mm256_set1_epi32(shift),
            max_sample: _mm256_set1_epi32(hi),
        }
    }

    #[target_feature(enable = "avx2")]
    fn integer(&self, value: __m256i) -> __m256i {
        _mm256_add_epi32(
            _mm256_min_epi32(_mm256_max_epi32(value, self.lower), self.upper),
            self.bias,
        )
    }

    #[target_feature(enable = "avx2")]
    fn float(&self, value: __m256) -> __m256i {
        let rounded = _mm256_round_ps::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(value);
        // The integer conversion yields INT_MIN for NaN and both overflow signs. Preserve
        // the positive overflow case separately, since that clamps to the other endpoint.
        let integer = _mm256_cvttps_epi32(rounded);
        let overflow = _mm256_set1_ps(2_147_483_648.0);
        let high = _mm256_castps_si256(_mm256_cmp_ps::<_CMP_GE_OQ>(value, overflow));
        _mm256_blendv_epi8(self.integer(integer), self.max_sample, high)
    }
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
            let [r, g, b_out] = inverse_reversible([a, b, c]);
            write(a, r);
            write(b, g);
            write(c, b_out);
        }
    } else {
        let ict = Ict::new();
        for ((a, b), c) in ys.iter_mut().zip(us).zip(vs) {
            let [red, green, blue] = ict.apply([a, b, c]);
            write(a, _mm256_castps_si256(red));
            write(b, _mm256_castps_si256(green));
            write(c, _mm256_castps_si256(blue));
        }
    }
    count
}

pub(crate) fn inverse_and_finish(planes: [&mut [i32]; 3], reversible: bool, finishes: [(i32, i32, i32); 3]) -> usize {
    // SAFETY: the module is compiled only when the target enables AVX2.
    unsafe { inverse_and_finish_avx2(planes, reversible, finishes) }
}

#[target_feature(enable = "avx2")]
fn inverse_and_finish_avx2([y, u, v]: [&mut [i32]; 3], reversible: bool, finishes: [(i32, i32, i32); 3]) -> usize {
    let (ys, _) = y.as_chunks_mut::<LANES>();
    let (us, _) = u.as_chunks_mut::<LANES>();
    let (vs, _) = v.as_chunks_mut::<LANES>();
    let count = ys.len().min(us.len()).min(vs.len()) * LANES;
    let [red, green, blue] = finishes.map(|finish| LevelShift::new(finish));
    if reversible {
        for ((a, b), c) in ys.iter_mut().zip(us).zip(vs) {
            let [r, g, b_out] = inverse_reversible([a, b, c]);
            write(a, red.integer(r));
            write(b, green.integer(g));
            write(c, blue.integer(b_out));
        }
    } else {
        let ict = Ict::new();
        for ((a, b), c) in ys.iter_mut().zip(us).zip(vs) {
            let [r, g, b_out] = ict.apply([a, b, c]);
            write(a, red.float(r));
            write(b, green.float(g));
            write(c, blue.float(b_out));
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
    let finish = LevelShift::new((shift, lo, hi));
    if reversible {
        for block in blocks {
            write(block, finish.integer(read(block)));
        }
    } else {
        for block in blocks {
            let value = _mm256_castsi256_ps(read(block));
            write(block, finish.float(value));
        }
    }
    count
}
