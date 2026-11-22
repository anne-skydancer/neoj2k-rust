// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The lifting, quantization, and colour transforms derive from OpenJPEG through src/enc_dwt.cpp.
//
// The copyright in this software is being made available under the 2-clauses
// BSD License, included below. This software may be subject to other third
// party and contributor rights, including patent rights, and no such rights
// are granted under this license.
//
// Copyright (c) 2002-2014, Universite catholique de Louvain (UCL), Belgium
// Copyright (c) 2002-2014, Professor Benoit Macq
// Copyright (c) 2001-2003, David Janssens
// Copyright (c) 2002-2003, Yannick Verschueren
// Copyright (c) 2003-2007, Francois-Olivier Devaux
// Copyright (c) 2003-2014, Antonin Descampe
// Copyright (c) 2005, Herve Drolon, FreeImage Team
// Copyright (c) 2007, Jonathan Ballard <dzonatas@dzonux.net>
// Copyright (c) 2007, Callum Lerwick <seg@haxxed.com>
// Copyright (c) 2008, Jerome Fimes, Communications & Systemes <jerome.fimes@c-s.fr>
// Copyright (c) 2012, Carl Hetherington
// Copyright (c) 2017, IntoPIX SA <support@intopix.com>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS `AS IS'
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

//! Encoder transforms for the core's one tile at the image origin.

const ALPHA: f32 = f32::from_bits(0xbfcb_0673);
const BETA: f32 = f32::from_bits(0xbd59_01ae);
const GAMMA: f32 = f32::from_bits(0x3f62_0676);
const DELTA: f32 = f32::from_bits(0x3ee3_1355);
const K: f32 = f32::from_bits(0x3f9d_7658);
const INV_K: f32 = f32::from_bits(0x3f50_19c3);
const COLUMN_BATCH_53: usize = 8;

const NORMS: [[f64; 10]; 4] = [
    [1.000, 1.500, 2.750, 5.375, 10.68, 21.34, 42.67, 85.33, 170.7, 341.3],
    [1.038, 1.592, 2.919, 5.703, 11.33, 22.64, 45.25, 90.48, 180.9, 0.0],
    [1.038, 1.592, 2.919, 5.703, 11.33, 22.64, 45.25, 90.48, 180.9, 0.0],
    [0.7186, 0.9218, 1.586, 3.043, 6.019, 12.01, 24.00, 47.97, 95.93, 0.0],
];

const NORMS_REAL: [[f64; 10]; 4] = [
    [1.000, 1.965, 4.177, 8.403, 16.90, 33.84, 67.69, 135.3, 270.6, 540.9],
    [2.022, 3.989, 8.355, 17.04, 34.27, 68.63, 137.3, 274.6, 549.0, 0.0],
    [2.022, 3.989, 8.355, 17.04, 34.27, 68.63, 137.3, 274.6, 549.0, 0.0],
    [2.080, 3.865, 8.307, 17.18, 34.71, 69.59, 139.3, 278.6, 557.2, 0.0],
];

const MCT_NORMS: [f64; 3] = [1.732, 0.8292, 0.8292];
const MCT_NORMS_REAL: [f64; 3] = [1.732, 1.805, 1.573];

pub(super) fn norm(level: usize, orientation: u8, reversible: bool) -> f64 {
    let index = level.min(if orientation == 0 { 9 } else { 8 });
    let table = if reversible { &NORMS } else { &NORMS_REAL };
    table[usize::from(orientation)][index]
}

pub(super) fn colour_norm(component: usize, reversible: bool, mct: bool) -> f64 {
    if !mct || component >= 3 {
        return 1.0;
    }
    let table = if reversible { MCT_NORMS } else { MCT_NORMS_REAL };
    table[component]
}

fn gain(orientation: u8) -> u32 {
    orientation.count_ones()
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Step {
    pub(super) exponent: u8,
    pub(super) mantissa: u16,
}

impl Step {
    /// The quantization step before the six fractional Tier-1 bits are added.
    pub(super) fn size(self, orientation: u8) -> f32 {
        // This is an exact binary fraction followed by multiplication by a power of two.
        let power =
            8 + i32::try_from(gain(orientation)).expect("subband gain is at most two") - i32::from(self.exponent);
        (1.0 + f32::from(self.mantissa) / 2048.0) * 2.0_f32.powi(power)
    }
}

/// OpenJPEG's explicit QCD step for one band at eight-bit precision.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the positive QCD step is floored as in OpenJPEG"
)]
pub(super) fn step(level: usize, orientation: u8, reversible: bool) -> Step {
    let band_gain = if reversible { gain(orientation) } else { 0 };
    let size = if reversible {
        1.0
    } else {
        1.0 / norm(level, orientation, false)
    };
    let scaled = (size * 8192.0).floor() as u32;
    let log = scaled.ilog2();
    let mantissa = if log > 11 {
        scaled >> (log - 11)
    } else {
        scaled << (11 - log)
    };
    Step {
        exponent: u8::try_from(8 + band_gain + 13 - log).expect("eight-bit QCD exponent fits in u8"),
        mantissa: u16::try_from(mantissa & 0x7ff).expect("masked mantissa fits in eleven bits"),
    }
}

pub(super) fn from_sample(value: i32) -> f32 {
    f32::from_bits(value.cast_unsigned())
}

pub(super) fn to_sample(value: f32) -> i32 {
    value.to_bits().cast_signed()
}

pub(super) fn shift_sample(value: u8, reversible: bool) -> i32 {
    let shifted = i16::from(value) - 128;
    if reversible {
        i32::from(shifted)
    } else {
        to_sample(f32::from(shifted))
    }
}

pub(super) fn colour_transform(planes: &mut [i32], samples: usize, reversible: bool) {
    let (red, tail) = planes.split_at_mut(samples);
    let (green, tail) = tail.split_at_mut(samples);
    let (blue, _) = tail.split_at_mut(samples);
    for ((red, green), blue) in red.iter_mut().zip(green).zip(blue) {
        if reversible {
            let (r, g, b) = (*red, *green, *blue);
            *red = (r + 2 * g + b) >> 2;
            *green = b - g;
            *blue = r - g;
        } else {
            let (r, g, b) = (from_sample(*red), from_sample(*green), from_sample(*blue));
            *red = to_sample(0.299 * r + 0.587 * g + 0.114 * b);
            *green = to_sample(-0.16875 * r - 0.331260 * g + 0.5 * b);
            *blue = to_sample(0.5 * r - 0.41869 * g - 0.08131 * b);
        }
    }
}

fn lift_53(line: &mut [i32]) {
    let last = line.len() - 1;
    for at in (1..last).step_by(2) {
        let prediction = line[at - 1].wrapping_add(line[at + 1]) >> 1;
        line[at] = line[at].wrapping_sub(prediction);
    }
    if last % 2 == 1 {
        line[last] = line[last].wrapping_sub(line[last - 1]);
    }
    for at in (0..=last).step_by(2) {
        let left = line[at.abs_diff(1)];
        let right = line[if at == last { at - 1 } else { at + 1 }];
        line[at] = line[at].wrapping_add(left.wrapping_add(right).wrapping_add(2) >> 2);
    }
}

fn lift_53_columns(line: &mut [[i32; COLUMN_BATCH_53]]) {
    let last = line.len() - 1;
    for row in (1..last).step_by(2) {
        let [above, centre, below] = [line[row - 1], line[row], line[row + 1]];
        line[row] = core::array::from_fn(|i| centre[i].wrapping_sub(above[i].wrapping_add(below[i]) >> 1));
    }
    if last % 2 == 1 {
        let [centre, above] = [line[last], line[last - 1]];
        line[last] = core::array::from_fn(|i| centre[i].wrapping_sub(above[i]));
    }
    for row in (0..=last).step_by(2) {
        let above = line[row.abs_diff(1)];
        let below = line[if row == last { last - 1 } else { row + 1 }];
        let centre = line[row];
        line[row] =
            core::array::from_fn(|i| centre[i].wrapping_add(above[i].wrapping_add(below[i]).wrapping_add(2) >> 2));
    }
}

fn forward_columns_53(plane: &mut [i32], stride: usize, cols: usize, rows: usize, line: &mut [i32]) -> usize {
    if cols < COLUMN_BATCH_53 || rows < 2 {
        return 0;
    }
    let lanes = line[..rows * COLUMN_BATCH_53].as_chunks_mut::<COLUMN_BATCH_53>().0;
    let end = cols / COLUMN_BATCH_53 * COLUMN_BATCH_53;
    for col in (0..end).step_by(COLUMN_BATCH_53) {
        let range = col..col + COLUMN_BATCH_53;
        for (tmp, row) in lanes.iter_mut().zip(plane.chunks_exact(stride)) {
            tmp.copy_from_slice(&row[range.clone()]);
        }
        lift_53_columns(lanes);
        let bands = lanes.iter().step_by(2).chain(lanes.iter().skip(1).step_by(2));
        for (row, tmp) in plane.chunks_exact_mut(stride).zip(bands) {
            row[range.clone()].copy_from_slice(tmp);
        }
    }
    end
}

fn lift_97(line: &mut [i32], parity: usize, coefficient: f32) {
    let last = line.len() - 1;
    for at in (parity..last).step_by(2) {
        let sum = from_sample(line[at.abs_diff(1)]) + from_sample(line[at + 1]);
        line[at] = to_sample(from_sample(line[at]) + sum * coefficient);
    }
    if last % 2 == parity {
        // Forward OpenJPEG doubles the neighbour, unlike its inverse's doubled coefficient.
        let neighbour = 2.0 * from_sample(line[last - 1]);
        line[last] = to_sample(from_sample(line[last]) + neighbour * coefficient);
    }
}

fn lift_and_scale_97(line: &mut [i32]) {
    lift_97(line, 1, ALPHA);
    lift_97(line, 0, BETA);
    lift_97(line, 1, GAMMA);
    lift_97(line, 0, DELTA);
    for sample in line.iter_mut().step_by(2) {
        *sample = to_sample(from_sample(*sample) * INV_K);
    }
    for sample in line.iter_mut().skip(1).step_by(2) {
        *sample = to_sample(from_sample(*sample) * K);
    }
}

fn forward_line(data: &mut [i32], stride: usize, reversible: bool, line: &mut [i32]) {
    if line.len() < 2 {
        return;
    }
    for (i, sample) in line.iter_mut().enumerate() {
        *sample = data[i * stride];
    }
    if reversible {
        lift_53(line);
    } else {
        lift_and_scale_97(line);
    }
    let bands = line.iter().step_by(2).chain(line.iter().skip(1).step_by(2));
    for (dst, &sample) in data.iter_mut().step_by(stride).zip(bands) {
        *dst = sample;
    }
}

pub(super) fn scratch_len(width: usize, height: usize, reversible: bool) -> usize {
    if reversible && width >= COLUMN_BATCH_53 {
        width.max(height * COLUMN_BATCH_53)
    } else {
        width.max(height)
    }
}

/// Work down from the largest resolution. Each step writes the low-pass rows and columns first.
/// `line` has the size returned by `scratch_len`.
pub(super) fn forward(
    plane: &mut [i32],
    width: usize,
    height: usize,
    levels: usize,
    reversible: bool,
    line: &mut [i32],
) {
    let (mut cols, mut rows) = (width, height);
    for _ in 1..levels {
        // The C++ encoder transforms columns before rows, the reverse of its decoder.
        let batched = if reversible {
            forward_columns_53(plane, width, cols, rows, line)
        } else {
            0
        };
        for col in batched..cols {
            forward_line(&mut plane[col..], width, reversible, &mut line[..rows]);
        }
        for row in 0..rows {
            let at = row * width;
            forward_line(&mut plane[at..at + cols], 1, reversible, &mut line[..cols]);
        }
        cols = cols.div_ceil(2);
        rows = rows.div_ceil(2);
    }
}
