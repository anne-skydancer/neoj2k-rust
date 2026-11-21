// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Packing finished component rows into the viewer's one- to four-channel byte layouts.

use core::arch::x86_64::{
    __m128i, __m256i, _mm_or_si128, _mm_slli_si128, _mm_srli_si128, _mm_storel_epi64, _mm_storeu_si128,
    _mm256_and_si256, _mm256_castsi256_si128, _mm256_extracti128_si256, _mm256_loadu_si256, _mm256_or_si256,
    _mm256_permute4x64_epi64, _mm256_permutevar8x32_epi32, _mm256_set1_epi32, _mm256_setr_epi8, _mm256_setr_epi32,
    _mm256_setzero_si256, _mm256_shuffle_epi8, _mm256_slli_epi32, _mm256_storeu_si256,
};

const PIXELS: usize = 8;

/// Write complete eight-pixel groups. Return the number of pixels written so that the caller can
/// copy the tail. Only the low byte of each 32-bit sample contributes, including negative samples.
pub(crate) fn write<const CHANNELS: usize>(rows: &[&[i32]; CHANNELS], dest: &mut [u8]) -> usize {
    assert!((1..=4).contains(&CHANNELS));
    let limit = rows.iter().fold(dest.len() / CHANNELS, |n, row| n.min(row.len()));
    let pixels = limit / PIXELS * PIXELS;
    if pixels == 0 {
        return 0;
    }
    let groups = rows.map(|row| row[..pixels].as_chunks::<PIXELS>().0);
    // SAFETY: this module is compiled only when AVX2 is a target feature. Each input slice has
    // pixels samples and the destination has pixels * CHANNELS bytes.
    unsafe { write_groups(&groups, &mut dest[..pixels * CHANNELS]) };
    pixels
}

#[target_feature(enable = "avx2")]
fn read(samples: &[i32; PIXELS]) -> __m256i {
    // SAFETY: this array holds 32 initialized bytes, and loadu permits any alignment.
    unsafe { _mm256_loadu_si256(samples.as_ptr().cast()) }
}

#[target_feature(enable = "avx2")]
fn store8(dest: &mut [u8], lanes: __m128i) {
    let dest = &mut dest[..8];
    // SAFETY: dest holds eight writable bytes. The store writes only the low 64 bits at any alignment.
    unsafe { _mm_storel_epi64(dest.as_mut_ptr().cast(), lanes) };
}

#[target_feature(enable = "avx2")]
fn store16(dest: &mut [u8], lanes: __m128i) {
    let dest = &mut dest[..16];
    // SAFETY: dest holds 16 writable bytes, and storeu permits any alignment.
    unsafe { _mm_storeu_si128(dest.as_mut_ptr().cast(), lanes) };
}

#[target_feature(enable = "avx2")]
fn store32(dest: &mut [u8], lanes: __m256i) {
    let dest = &mut dest[..32];
    // SAFETY: dest holds 32 writable bytes, and storeu permits any alignment.
    unsafe { _mm256_storeu_si256(dest.as_mut_ptr().cast(), lanes) };
}

#[target_feature(enable = "avx2")]
fn shuffle_mask<const CHANNELS: usize>() -> __m256i {
    match CHANNELS {
        1 => _mm256_setr_epi8(
            0, 4, 8, 12, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 4, 8, 12, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1,
        ),
        2 => _mm256_setr_epi8(
            0, 1, 4, 5, 8, 9, 12, 13, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 4, 5, 8, 9, 12, 13, -1, -1, -1, -1, -1, -1,
            -1, -1,
        ),
        3 => _mm256_setr_epi8(
            0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14, -1, -1, -1, -1, 0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14, -1, -1, -1,
            -1,
        ),
        _ => _mm256_setzero_si256(),
    }
}

#[target_feature(enable = "avx2")]
fn write_groups<const CHANNELS: usize>(rows: &[&[[i32; PIXELS]]; CHANNELS], dest: &mut [u8]) {
    let byte_mask = _mm256_set1_epi32(0xff);
    let shuffle = shuffle_mask::<CHANNELS>();
    let join_one = _mm256_setr_epi32(0, 4, 0, 0, 0, 0, 0, 0);
    for (at, target) in dest.chunks_exact_mut(PIXELS * CHANNELS).enumerate() {
        let a = read(&rows[0][at]);
        let mut lanes = a;
        if CHANNELS >= 2 {
            let b = _mm256_and_si256(read(&rows[1][at]), byte_mask);
            lanes = _mm256_or_si256(_mm256_and_si256(a, byte_mask), _mm256_slli_epi32::<8>(b));
        }
        if CHANNELS >= 3 {
            let c = _mm256_and_si256(read(&rows[2][at]), byte_mask);
            lanes = _mm256_or_si256(lanes, _mm256_slli_epi32::<16>(c));
        }
        if CHANNELS == 4 {
            let d = read(&rows[3][at]);
            lanes = _mm256_or_si256(lanes, _mm256_slli_epi32::<24>(d));
            store32(target, lanes);
            continue;
        }

        let packed = _mm256_shuffle_epi8(lanes, shuffle);
        match CHANNELS {
            1 => {
                let joined = _mm256_permutevar8x32_epi32(packed, join_one);
                store8(target, _mm256_castsi256_si128(joined));
            }
            2 => {
                let joined = _mm256_permute4x64_epi64::<0x08>(packed);
                store16(target, _mm256_castsi256_si128(joined));
            }
            3 => {
                // Join the two 12-byte halves without writing past the eight RGB pixels.
                let low = _mm256_castsi256_si128(packed);
                let high = _mm256_extracti128_si256::<1>(packed);
                store16(target, _mm_or_si128(low, _mm_slli_si128::<12>(high)));
                store8(&mut target[16..], _mm_srli_si128::<4>(high));
            }
            _ => unreachable!("the channel count was validated"),
        }
    }
}
