// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Eight independent inverse wavelet lines in AVX2 lanes. The lifting order is the scalar DWT's.

use core::arch::x86_64::{
    __m256, __m256i, _mm256_add_epi32, _mm256_add_ps, _mm256_castps_si256, _mm256_castsi256_ps, _mm256_loadu_si256,
    _mm256_mul_ps, _mm256_permute2x128_si256, _mm256_set1_epi32, _mm256_set1_ps, _mm256_srai_epi32,
    _mm256_storeu_si256, _mm256_sub_epi32, _mm256_unpackhi_epi32, _mm256_unpackhi_epi64, _mm256_unpacklo_epi32,
    _mm256_unpacklo_epi64,
};

use crate::dwt::{ALPHA, BETA, DELTA, GAMMA, K, TWO_INV_K};

pub(crate) const LANES: usize = 8;
type Lane = [i32; LANES];

/// Transform eight rows or columns with one sample per lane. The source lines hold low samples
/// followed by high samples. `sample_stride` steps through a line, `lane_stride` steps across
/// lanes, and `scratch` has a lane group for every sample of one line.
pub(crate) fn inverse(
    data: &mut [i32],
    sample_stride: usize,
    lane_stride: usize,
    parity: usize,
    reversible: bool,
    scratch: &mut [Lane],
) {
    let len = scratch.len();
    if len < 2 {
        if len == 1 && reversible && parity == 1 {
            for lane in 0..LANES {
                data[lane * lane_stride] /= 2;
            }
        }
        return;
    }

    let low_len = len / 2 + (len % 2) * (1 - parity);
    for (index, lanes) in scratch.iter_mut().enumerate() {
        let input = index / 2 + usize::from(index % 2 != parity) * low_len;
        let start = input * sample_stride;
        if lane_stride == 1 {
            lanes.copy_from_slice(&data[start..start + LANES]);
        } else {
            for (lane, sample) in lanes.iter_mut().enumerate() {
                *sample = data[start + lane * lane_stride];
            }
        }
    }

    if reversible {
        // SAFETY: this module is compiled only when AVX2 is a target feature.
        unsafe { inverse_53(scratch, parity) };
    } else {
        // SAFETY: this module is compiled only when AVX2 is a target feature.
        unsafe { inverse_97(scratch, parity) };
    }

    for (index, lanes) in scratch.iter().enumerate() {
        let start = index * sample_stride;
        if lane_stride == 1 {
            data[start..start + LANES].copy_from_slice(lanes);
        } else {
            for (lane, &sample) in lanes.iter().enumerate() {
                data[start + lane * lane_stride] = sample;
            }
        }
    }
}

/// Transform eight contiguous 9/7 rows, transposing full blocks of coefficients into lanes.
pub(crate) fn inverse_97_rows(data: &mut [i32], stride: usize, parity: usize, scratch: &mut [Lane]) {
    // SAFETY: this module is compiled only when AVX2 is a target feature.
    unsafe { inverse_97_rows_avx2(data, stride, parity, scratch) };
}

/// Transform eight columns whose rows are borrowed separately by an executor job.
pub(crate) fn inverse_stripe(
    rows: &mut [&mut [i32]],
    column: usize,
    parity: usize,
    reversible: bool,
    scratch: &mut [Lane],
) {
    let len = scratch.len();
    let low_len = len / 2 + (len % 2) * (1 - parity);
    let columns = column..column + LANES;
    for (index, lanes) in scratch.iter_mut().enumerate() {
        let row = index / 2 + usize::from(index % 2 != parity) * low_len;
        lanes.copy_from_slice(&rows[row][columns.clone()]);
    }
    if reversible {
        // SAFETY: this module is compiled only when AVX2 is a target feature.
        unsafe { inverse_53(scratch, parity) };
    } else {
        // SAFETY: this module is compiled only when AVX2 is a target feature.
        unsafe { inverse_97(scratch, parity) };
    }
    for (row, lanes) in rows.iter_mut().zip(scratch) {
        row[columns.clone()].copy_from_slice(lanes);
    }
}

#[target_feature(enable = "avx2")]
fn read(lanes: &Lane) -> __m256i {
    // SAFETY: a Lane has exactly 32 initialized bytes, and loadu permits its alignment.
    unsafe { _mm256_loadu_si256(lanes.as_ptr().cast()) }
}

#[target_feature(enable = "avx2")]
fn write(lanes: &mut Lane, value: __m256i) {
    // SAFETY: the exclusive Lane gives 32 writable bytes, and storeu permits its alignment.
    unsafe { _mm256_storeu_si256(lanes.as_mut_ptr().cast(), value) };
}

#[target_feature(enable = "avx2")]
fn transpose_four(rows: [__m256i; 4]) -> [__m256i; 4] {
    let a = _mm256_unpacklo_epi32(rows[0], rows[1]);
    let b = _mm256_unpackhi_epi32(rows[0], rows[1]);
    let c = _mm256_unpacklo_epi32(rows[2], rows[3]);
    let d = _mm256_unpackhi_epi32(rows[2], rows[3]);
    [
        _mm256_unpacklo_epi64(a, c),
        _mm256_unpackhi_epi64(a, c),
        _mm256_unpacklo_epi64(b, d),
        _mm256_unpackhi_epi64(b, d),
    ]
}

#[target_feature(enable = "avx2")]
fn transpose([a, b, c, d, e, f, g, h]: [__m256i; LANES]) -> [__m256i; LANES] {
    // Transpose four rows within each 128-bit half, then join the halves into columns.
    let [a, b, c, d] = transpose_four([a, b, c, d]);
    let [e, f, g, h] = transpose_four([e, f, g, h]);
    [
        _mm256_permute2x128_si256::<0x20>(a, e),
        _mm256_permute2x128_si256::<0x20>(b, f),
        _mm256_permute2x128_si256::<0x20>(c, g),
        _mm256_permute2x128_si256::<0x20>(d, h),
        _mm256_permute2x128_si256::<0x31>(a, e),
        _mm256_permute2x128_si256::<0x31>(b, f),
        _mm256_permute2x128_si256::<0x31>(c, g),
        _mm256_permute2x128_si256::<0x31>(d, h),
    ]
}

#[target_feature(enable = "avx2")]
fn gather_band(rows: &[&[i32]; LANES], start: usize, count: usize, parity: usize, scratch: &mut [Lane]) {
    let complete = count / LANES * LANES;
    for at in (0..complete).step_by(LANES) {
        let x = start + at;
        let block = rows.map(|row| read(row[x..x + LANES].try_into().expect("eight samples")));
        for (sample, packed) in transpose(block).into_iter().enumerate() {
            write(&mut scratch[parity + 2 * (at + sample)], packed);
        }
    }
    for at in complete..count {
        scratch[parity + 2 * at] = rows.map(|row| row[start + at]);
    }
}

#[target_feature(enable = "avx2")]
fn inverse_97_rows_avx2(data: &mut [i32], stride: usize, parity: usize, scratch: &mut [Lane]) {
    // Transpose each input band in blocks. This loads eight adjacent samples of a row at once
    // instead of loading one sample from each of the eight rows.
    let len = scratch.len();
    let low_len = len / 2 + (len % 2) * (1 - parity);
    let rows = core::array::from_fn(|lane| &data[lane * stride..lane * stride + len]);
    gather_band(&rows, 0, low_len, parity, scratch);
    gather_band(&rows, low_len, len - low_len, 1 - parity, scratch);
    inverse_97(scratch, parity);

    let (blocks, tail) = scratch.as_chunks::<LANES>();
    for (at, [a, b, c, d, e, f, g, h]) in blocks.iter().enumerate() {
        let rows = transpose([read(a), read(b), read(c), read(d), read(e), read(f), read(g), read(h)]);
        let x = at * LANES;
        for (row, samples) in data.chunks_mut(stride).zip(rows) {
            let dest = (&mut row[x..x + LANES]).try_into().expect("eight samples");
            write(dest, samples);
        }
    }
    let x = blocks.len() * LANES;
    for (lane, row) in data.chunks_mut(stride).take(LANES).enumerate() {
        for (sample, packed) in row[x..len].iter_mut().zip(tail) {
            *sample = packed[lane];
        }
    }
}

#[target_feature(enable = "avx2")]
fn inverse_53(line: &mut [Lane], parity: usize) {
    let last = line.len() - 1;
    // Complete 16-column groups double the edge neighbour with wrapping arithmetic. The two-
    // sample odd-origin case is the scalar shortcut in the reference for both groups and tails.
    let full_group_edge = parity == 0 || last > 1;
    let one = _mm256_set1_epi32(1);
    let two = _mm256_set1_epi32(2);
    for index in (parity..=last).step_by(2) {
        let previous = read(&line[index.abs_diff(1)]);
        let next = read(&line[if index == last { last - 1 } else { index + 1 }]);
        let adjustment = if (index == 0 || index == last) && !full_group_edge {
            _mm256_srai_epi32::<1>(_mm256_add_epi32(previous, one))
        } else {
            _mm256_srai_epi32::<2>(_mm256_add_epi32(_mm256_add_epi32(previous, next), two))
        };
        let sample = _mm256_sub_epi32(read(&line[index]), adjustment);
        write(&mut line[index], sample);
    }
    for index in ((1 - parity)..=last).step_by(2) {
        let previous = read(&line[index.abs_diff(1)]);
        let adjustment = if index == 0 || index == last {
            previous
        } else {
            _mm256_srai_epi32::<1>(_mm256_add_epi32(previous, read(&line[index + 1])))
        };
        let sample = _mm256_add_epi32(read(&line[index]), adjustment);
        write(&mut line[index], sample);
    }
}

#[target_feature(enable = "avx2")]
fn read_float(lanes: &Lane) -> __m256 {
    _mm256_castsi256_ps(read(lanes))
}

#[target_feature(enable = "avx2")]
fn write_float(lanes: &mut Lane, value: __m256) {
    write(lanes, _mm256_castps_si256(value));
}

pub(crate) fn sweep_97_rows<const ODD: bool>(low: &mut [&mut [i32]; 4], high: &mut [&mut [i32]; 3]) -> usize {
    // SAFETY: this module is compiled only when AVX2 is a target feature.
    unsafe { sweep_97_avx2::<ODD>(low, high) }
}

#[target_feature(enable = "avx2")]
fn read_row(row: &[i32], column: usize) -> __m256 {
    read_float(row[column..column + LANES].try_into().expect("an AVX2 group"))
}

#[target_feature(enable = "avx2")]
fn write_row(row: &mut [i32], column: usize, samples: __m256) {
    write_float(
        (&mut row[column..column + LANES]).try_into().expect("an AVX2 group"),
        samples,
    );
}

#[target_feature(enable = "avx2")]
fn lift_rows(sample: __m256, a: __m256, b: __m256, coefficient: __m256) -> __m256 {
    _mm256_add_ps(sample, _mm256_mul_ps(_mm256_add_ps(a, b), coefficient))
}

#[target_feature(enable = "avx2")]
fn sweep_97_avx2<const ODD: bool>(low: &mut [&mut [i32]; 4], high: &mut [&mut [i32]; 3]) -> usize {
    let [l0, l1, l2, l3] = low;
    let [h0, h1, h2] = high;
    let width = l0.len();
    let end = width / LANES * LANES;
    let (k, two_inv_k) = (_mm256_set1_ps(K), _mm256_set1_ps(TWO_INV_K));
    let (delta, gamma) = (_mm256_set1_ps(-DELTA), _mm256_set1_ps(-GAMMA));
    let (beta, alpha) = (_mm256_set1_ps(-BETA), _mm256_set1_ps(-ALPHA));
    for x in (0..end).step_by(LANES) {
        let lk = _mm256_mul_ps(read_row(l0, x), k);
        let hk = _mm256_mul_ps(read_row(h0, x), two_inv_k);
        let hk1 = read_row(h1, x);
        let hk2 = read_row(h2, x);
        let lk1 = read_row(l1, x);
        let lk2 = read_row(l2, x);
        if ODD {
            let d = lift_rows(lk1, hk1, hk, delta);
            let g = lift_rows(hk1, lk2, d, gamma);
            let b = lift_rows(lk2, hk2, g, beta);
            let a = lift_rows(hk2, read_row(l3, x), b, alpha);
            write_row(l0, x, lk);
            write_row(l1, x, d);
            write_row(l2, x, b);
            write_row(h1, x, g);
            write_row(h2, x, a);
        } else {
            let d = lift_rows(lk, hk1, hk, delta);
            let g = lift_rows(hk1, lk1, d, gamma);
            let b = lift_rows(lk1, hk2, g, beta);
            let a = lift_rows(hk2, lk2, b, alpha);
            write_row(l0, x, d);
            write_row(l1, x, b);
            write_row(h1, x, g);
            write_row(h2, x, a);
        }
        write_row(h0, x, hk);
    }
    end
}

#[target_feature(enable = "avx2")]
fn lift_97(line: &mut [Lane], parity: usize, coefficient: f32) {
    let last = line.len() - 1;
    let scale = _mm256_set1_ps(coefficient);
    for index in (parity..last).step_by(2) {
        let sum = _mm256_add_ps(read_float(&line[index.abs_diff(1)]), read_float(&line[index + 1]));
        let delta = _mm256_mul_ps(sum, scale);
        let sample = _mm256_add_ps(read_float(&line[index]), delta);
        write_float(&mut line[index], sample);
    }
    if last % 2 == parity {
        let delta = _mm256_mul_ps(_mm256_add_ps(scale, scale), read_float(&line[last - 1]));
        let sample = _mm256_add_ps(read_float(&line[last]), delta);
        write_float(&mut line[last], sample);
    }
}

#[target_feature(enable = "avx2")]
fn inverse_97(line: &mut [Lane], parity: usize) {
    let low = _mm256_set1_ps(K);
    let high = _mm256_set1_ps(TWO_INV_K);
    for sample in line.iter_mut().skip(parity).step_by(2) {
        let scaled = _mm256_mul_ps(read_float(sample), low);
        write_float(sample, scaled);
    }
    for sample in line.iter_mut().skip(1 - parity).step_by(2) {
        let scaled = _mm256_mul_ps(read_float(sample), high);
        write_float(sample, scaled);
    }
    lift_97(line, parity, -DELTA);
    lift_97(line, 1 - parity, -GAMMA);
    lift_97(line, parity, -BETA);
    lift_97(line, 1 - parity, -ALPHA);
}
