// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The inverse wavelet arithmetic is from OpenJPEG's dwt.c through src/dwt.cpp.
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

//! Scalar inverse wavelet transforms of a whole tile-component.

use crate::geometry::Rect;
use crate::{Error, ErrorKind};

const GROUP_WIDTH_53: usize = 16;

const ALPHA: f32 = f32::from_bits(0xbfcb_0673);
const BETA: f32 = f32::from_bits(0xbd59_01ae);
const GAMMA: f32 = f32::from_bits(0x3f62_0676);
const DELTA: f32 = f32::from_bits(0x3ee3_1355);
const K: f32 = f32::from_bits(0x3f9d_7658);
// OpenJPEG keeps this high-band scale and compensates for it in dequantisation.
const TWO_INV_K: f32 = f32::from_bits(0x3fd0_1800);

/// One line, reused for both wavelets and across tile-components.
#[derive(Debug, Default)]
pub(crate) struct Workspace {
    line: Vec<i32>,
}

fn invalid_geometry() -> Error {
    Error::new(ErrorKind::Data, "Invalid tile-component wavelet geometry\n")
}

fn dimensions(rect: Rect) -> Option<(usize, usize)> {
    Some((
        usize::try_from(rect.x1.checked_sub(rect.x0)?).ok()?,
        usize::try_from(rect.y1.checked_sub(rect.y0)?).ok()?,
    ))
}

fn coarser_matches(coarse: Rect, fine: Rect) -> bool {
    coarse.x0 == fine.x0.div_ceil(2)
        && coarse.y0 == fine.y0.div_ceil(2)
        && coarse.x1 == fine.x1.div_ceil(2)
        && coarse.y1 == fine.y1.div_ceil(2)
}

/// Transform the requested resolutions in place, from coarsest to finest. Each row has `stride`
/// samples and each level's subbands occupy its upper-left quadrants. Irreversible samples use
/// the bit representation of `f32` in the same buffer. Padding beyond a level is preserved.
pub(crate) fn inverse(
    data: &mut [i32],
    stride: usize,
    resolutions: &[Rect],
    reversible: bool,
    workspace: &mut Workspace,
) -> Result<(), Error> {
    let [_, finer @ ..] = resolutions else {
        return Ok(());
    };
    let Some(&last) = finer.last() else {
        return Ok(());
    };
    let (width, height) = dimensions(last).ok_or_else(invalid_geometry)?;
    if width > stride || !resolutions.windows(2).all(|pair| coarser_matches(pair[0], pair[1])) {
        return Err(invalid_geometry());
    }
    if width == 0 || height == 0 {
        return Ok(());
    }
    if (height - 1)
        .checked_mul(stride)
        .and_then(|offset| offset.checked_add(width))
        .is_none_or(|end| end > data.len())
    {
        return Err(invalid_geometry());
    }

    let scratch_size = width.max(height);
    if workspace.line.len() < scratch_size {
        workspace
            .line
            .try_reserve_exact(scratch_size - workspace.line.len())
            .map_err(|_| Error::new(ErrorKind::NoMemory, "Not enough memory for the inverse wavelet\n"))?;
        workspace.line.resize(scratch_size, 0);
    }
    for &rect in finer {
        let (width, height) = dimensions(rect).ok_or_else(invalid_geometry)?;
        if width == 0 || height == 0 {
            continue;
        }
        let x_parity = usize::from(rect.x0 & 1 != 0);
        let y_parity = usize::from(rect.y0 & 1 != 0);
        for row in 0..height {
            let offset = row * stride;
            inverse_line(
                &mut data[offset..offset + width],
                1,
                x_parity,
                reversible,
                false,
                &mut workspace.line[..width],
            );
        }
        let group_end = width / GROUP_WIDTH_53 * GROUP_WIDTH_53;
        for col in 0..width {
            inverse_line(
                &mut data[col..],
                stride,
                y_parity,
                reversible,
                col < group_end,
                &mut workspace.line[..height],
            );
        }
    }
    Ok(())
}

fn inverse_line(data: &mut [i32], stride: usize, parity: usize, reversible: bool, full_group: bool, line: &mut [i32]) {
    let len = line.len();
    let low_len = len / 2 + (len % 2) * (1 - parity);
    for (i, sample) in line.iter_mut().enumerate() {
        let index = if i % 2 == parity { i / 2 } else { low_len + i / 2 };
        *sample = data[index * stride];
    }
    if reversible {
        inverse_53(line, parity, full_group);
    } else {
        inverse_97(line, parity);
    }
    for (i, &sample) in line.iter().enumerate() {
        data[i * stride] = sample;
    }
}

fn inverse_53(line: &mut [i32], parity: usize, full_group: bool) {
    if let [sample] = line {
        if parity == 1 {
            *sample /= 2;
        }
        return;
    }
    let last = line.len() - 1;
    // A two-sample odd-origin column uses the scalar shortcut even in a full group.
    let group_edges = full_group && (parity == 0 || last > 1);
    for at in (parity..=last).step_by(2) {
        let previous = line[at.abs_diff(1)];
        let next = line[if at == last { last - 1 } else { at + 1 }];
        let adjustment = if (at == 0 || at == last) && !group_edges {
            previous.wrapping_add(1) >> 1
        } else {
            previous.wrapping_add(next).wrapping_add(2) >> 2
        };
        line[at] = line[at].wrapping_sub(adjustment);
    }
    for at in ((1 - parity)..=last).step_by(2) {
        let previous = line[at.abs_diff(1)];
        let adjustment = if at == 0 || at == last {
            previous
        } else {
            previous.wrapping_add(line[at + 1]) >> 1
        };
        line[at] = line[at].wrapping_add(adjustment);
    }
}

fn from_sample(sample: i32) -> f32 {
    f32::from_bits(sample.cast_unsigned())
}

fn to_sample(sample: f32) -> i32 {
    sample.to_bits().cast_signed()
}

fn lift_97(line: &mut [i32], parity: usize, coefficient: f32) {
    let last = line.len() - 1;
    for at in (parity..last).step_by(2) {
        // OpenJPEG adds the two neighbours in order even when both mirror the first one.
        let sum = from_sample(line[at.abs_diff(1)]) + from_sample(line[at + 1]);
        line[at] = to_sample(from_sample(line[at]) + sum * coefficient);
    }
    if last % 2 == parity {
        // The far edge doubles the coefficient before it multiplies the mirrored sample.
        let product = (coefficient + coefficient) * from_sample(line[last - 1]);
        line[last] = to_sample(from_sample(line[last]) + product);
    }
}

fn inverse_97(line: &mut [i32], parity: usize) {
    if line.len() < 2 {
        return;
    }
    for sample in line.iter_mut().skip(parity).step_by(2) {
        *sample = to_sample(from_sample(*sample) * K);
    }
    for sample in line.iter_mut().skip(1 - parity).step_by(2) {
        *sample = to_sample(from_sample(*sample) * TWO_INV_K);
    }
    lift_97(line, parity, -DELTA);
    lift_97(line, 1 - parity, -GAMMA);
    lift_97(line, parity, -BETA);
    lift_97(line, 1 - parity, -ALPHA);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x0: u32, y0: u32, x1: u32, y1: u32) -> Rect {
        Rect { x0, y0, x1, y1 }
    }

    #[test]
    fn reversible_odd_origin_keeps_padding_and_reuses_scratch() {
        let mut data = [5, 0, 0, -99, 0, 0, 0, -99, 0, 0, 0, -99];
        let mut workspace = Workspace::default();
        inverse(
            &mut data,
            4,
            &[rect(1, 1, 2, 2), rect(1, 1, 4, 4)],
            true,
            &mut workspace,
        )
        .unwrap();
        assert_eq!(data, [5, 5, 5, -99, 5, 5, 5, -99, 5, 5, 5, -99]);

        let capacity = workspace.line.capacity();
        let mut singleton = [-15];
        let empty_low = [rect(1, 1, 1, 1), rect(1, 1, 2, 2)];
        inverse(&mut singleton, 1, &empty_low, true, &mut workspace).unwrap();
        assert_eq!(singleton, [-3]);
        assert_eq!(workspace.line.capacity(), capacity);
        inverse(&mut singleton, 1, &empty_low, false, &mut workspace).unwrap();
        assert_eq!(singleton, [-3]);
    }

    #[test]
    fn vertical_integer_mirror_wraps_only_for_complete_column_groups() {
        let mut data = [0; 38];
        data[19..28].fill(1 << 30);
        data[17..19].fill(-77);
        data[36..38].fill(-77);
        inverse(
            &mut data,
            19,
            &[rect(0, 0, 9, 1), rect(0, 0, 17, 2)],
            true,
            &mut Workspace::default(),
        )
        .unwrap();
        assert_eq!((data[0], data[19]), (1 << 29, 3 << 29));
        assert_eq!((data[16], data[35]), (-(1 << 29), 1 << 29));
        assert_eq!(data[17..19], [-77; 2]);
        assert_eq!(data[36..38], [-77; 2]);
    }

    #[test]
    fn irreversible_pair_matches_the_openjpeg_lifting_bits() {
        let input = [to_sample(-1.25), to_sample(2.5), 123];
        let expected = [0xc06f_fea6_u32.cast_signed(), 0x3f9f_fd50, 123];
        let mut data = input;
        inverse(
            &mut data,
            3,
            &[rect(0, 0, 1, 1), rect(0, 0, 2, 1)],
            false,
            &mut Workspace::default(),
        )
        .unwrap();
        assert_eq!(data, expected);

        let mut data = input;
        inverse(
            &mut data,
            3,
            &[rect(1, 0, 2, 1), rect(1, 0, 3, 1)],
            false,
            &mut Workspace::default(),
        )
        .unwrap();
        assert_eq!(data, [expected[1], expected[0], 123]);
    }

    #[test]
    fn invalid_plane_geometry_leaves_the_samples_unchanged() {
        let mut data = [10, 20, 30];
        let error = inverse(
            &mut data,
            2,
            &[rect(0, 0, 1, 1), rect(0, 0, 2, 2)],
            true,
            &mut Workspace::default(),
        );
        assert_eq!(error.unwrap_err().kind(), ErrorKind::Data);
        assert_eq!(data, [10, 20, 30]);
    }
}

#[cfg(all(test, not(miri)))]
mod differential {
    use super::*;
    use neo_j2k_oracle::kernels::{WaveletRect, dwt_decode};

    fn wavelet_rect(rect: &Rect) -> WaveletRect {
        WaveletRect {
            x0: rect.x0,
            y0: rect.y0,
            x1: rect.x1,
            y1: rect.y1,
        }
    }

    fn random_samples(n: u32, reversible: bool) -> i32 {
        if reversible {
            return n.cast_signed();
        }
        if n.is_multiple_of(32) {
            return match n & 96 {
                0 => to_sample(-0.0),
                32 => to_sample(f32::NAN),
                64 => to_sample(f32::from_bits(1)),
                _ => to_sample(f32::MAX),
            };
        }
        let sample = i16::try_from(n % 2048).unwrap();
        to_sample((f32::from(sample) - 1024.0) * 0.03125)
    }

    #[test]
    fn whole_tile_components_match_the_cpp_wavelet() {
        let mut state = 0x956a_2283_u32;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            state
        };
        let mut workspace = Workspace::default();
        for case in 0..320 {
            let reversible = case % 2 == 1;
            let (levels, decoded, x0, y0, width, height) = match case {
                // Exercise both C++ row-wise paths, one padded odd-origin case per wavelet.
                0 | 1 => (4, 4, 3, 5, 533, 525),
                // Full 16-column groups followed by a scalar remainder in the reversible path.
                3 => (2, 2, 1, 1, 17, 4),
                _ => {
                    let levels = 1 + next() % 7;
                    let decoded = if next().is_multiple_of(4) {
                        1 + next() % levels
                    } else {
                        levels
                    };
                    (
                        levels,
                        decoded,
                        next() % 17,
                        next() % 17,
                        1 + next() % 85,
                        1 + next() % 85,
                    )
                }
            };
            let stride = width + 1 + next() % 19;
            let len = usize::try_from(stride * height).unwrap();
            let stride_usize = usize::try_from(stride).unwrap();
            let mut rust: Vec<_> = (0..len).map(|_| random_samples(next(), reversible)).collect();
            let mut cpp = rust.clone();
            let resolution_rects: Vec<_> = (0..levels)
                .rev()
                .take(usize::try_from(decoded).unwrap())
                .map(|shift| {
                    let divisor = 1 << shift;
                    Rect {
                        x0: x0.div_ceil(divisor),
                        y0: y0.div_ceil(divisor),
                        x1: (x0 + width).div_ceil(divisor),
                        y1: (y0 + height).div_ceil(divisor),
                    }
                })
                .collect();
            let cpp_rects: Vec<_> = resolution_rects.iter().map(wavelet_rect).collect();

            assert!(
                dwt_decode(&mut cpp, stride, &cpp_rects, reversible),
                "C++ refused case {case}"
            );
            inverse(&mut rust, stride_usize, &resolution_rects, reversible, &mut workspace).unwrap();
            for (i, (rust, cpp)) in rust.into_iter().zip(cpp).enumerate() {
                // The C++ kernel tests consider NaN payloads equivalent, which also matches Rust's
                // floating-point contract. Every other sample and padding value must match by bits.
                assert!(
                    rust == cpp || (!reversible && from_sample(rust).is_nan() && from_sample(cpp).is_nan()),
                    "case {case}, sample {i}, reversible {reversible}: Rust {rust:08x}, C++ {cpp:08x}",
                );
            }
        }
    }

    #[test]
    fn cpp_wavelet_shim_checks_slice_and_resolution_bounds() {
        let mut plane = [10, 20, 30];
        let coarse = WaveletRect {
            x0: 0,
            y0: 0,
            x1: 1,
            y1: 1,
        };
        let fine = WaveletRect {
            x0: 0,
            y0: 0,
            x1: 2,
            y1: 2,
        };
        assert!(!dwt_decode(&mut plane, 2, &[coarse, fine], true));
        assert!(!dwt_decode(&mut plane, 2, &[fine, coarse], true));
        assert_eq!(plane, [10, 20, 30]);
    }
}
