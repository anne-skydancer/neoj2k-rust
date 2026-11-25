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

//! Inverse wavelet transforms of a whole tile-component.

mod lifting97;

use crate::executor::{Jobs, MAX_JOBS};
use crate::geometry::Rect;
#[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
use crate::simd::dwt as vector;
use crate::{Error, ErrorKind};

const GROUP_WIDTH_53: usize = 16;
const COLUMN_BATCH_97: usize = 8;
const ROW_BATCH: usize = 8;
pub(crate) const PARALLEL_LEVEL_AREA: usize = if cfg!(miri) { 1 } else { 1 << 18 };
const ROW_WISE_AREA_97: usize = 128 * 128;
const ROW_WISE_AREA_53: usize = 512 * 512;

pub(crate) const ALPHA: f32 = f32::from_bits(0xbfcb_0673);
pub(crate) const BETA: f32 = f32::from_bits(0xbd59_01ae);
pub(crate) const GAMMA: f32 = f32::from_bits(0x3f62_0676);
pub(crate) const DELTA: f32 = f32::from_bits(0x3ee3_1355);
pub(crate) const K: f32 = f32::from_bits(0x3f9d_7658);
// OpenJPEG keeps this high-band scale and compensates for it in dequantisation.
pub(crate) const TWO_INV_K: f32 = f32::from_bits(0x3fd0_1800);

/// Scratch for a line or a batch of adjacent lines, reused across tile-components.
#[derive(Debug, Default)]
pub(crate) struct Workspace {
    line: Vec<i32>,
    parallel_lines: Vec<Vec<i32>>,
}

impl Workspace {
    pub(crate) fn retained_bytes(&self) -> u64 {
        crate::mem::capacity_bytes(&self.line)
            + crate::mem::capacity_bytes(&self.parallel_lines)
            + self.parallel_lines.iter().map(crate::mem::capacity_bytes).sum::<u64>()
    }

    pub(crate) fn release_parallel_scratch(&mut self) {
        self.parallel_lines = Vec::new();
    }
}

fn invalid_geometry() -> Error {
    Error::static_message(ErrorKind::Data, "Invalid tile-component wavelet geometry\n")
}

fn no_memory() -> Error {
    Error::static_message(ErrorKind::NoMemory, "Not enough memory for the inverse wavelet\n")
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
    inverse_with::<true>(data, stride, resolutions, reversible, workspace, None)
}

pub(crate) fn inverse_parallel(
    data: &mut [i32],
    stride: usize,
    resolutions: &[Rect],
    reversible: bool,
    workspace: &mut Workspace,
    jobs: Jobs<'_>,
) -> Result<(), Error> {
    inverse_with::<true>(data, stride, resolutions, reversible, workspace, Some(jobs))
}

fn inverse_with<const VECTOR: bool>(
    data: &mut [i32],
    stride: usize,
    resolutions: &[Rect],
    reversible: bool,
    workspace: &mut Workspace,
    jobs: Option<Jobs<'_>>,
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
    #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
    let scratch_size = if VECTOR {
        let row = if !reversible && height >= vector::LANES {
            width
        } else {
            0
        };
        let group = if reversible { GROUP_WIDTH_53 } else { vector::LANES };
        let column = if width >= group { height } else { 0 };
        scratch_size.max(row.max(column).checked_mul(vector::LANES).ok_or_else(no_memory)?)
    } else {
        scratch_size
    };
    let use_column_batches =
        !reversible && (!VECTOR || !cfg!(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2")));
    let scratch_size = if use_column_batches && width >= COLUMN_BATCH_97 {
        scratch_size.max(height.checked_mul(COLUMN_BATCH_97).ok_or_else(no_memory)?)
    } else {
        scratch_size
    };
    let scratch_size = if row_wise(width, height, reversible) {
        scratch_size.max(width.checked_add(height).ok_or_else(no_memory)?)
    } else {
        scratch_size
    };
    if let Err(error) = reserve_line(&mut workspace.line, scratch_size) {
        if workspace.parallel_lines.is_empty() {
            return Err(error);
        }
        workspace.release_parallel_scratch();
        reserve_line(&mut workspace.line, scratch_size)?;
    }
    for &rect in finer {
        let (width, height) = dimensions(rect).ok_or_else(invalid_geometry)?;
        if width == 0 || height == 0 {
            continue;
        }
        let level = Level {
            width,
            height,
            stride,
            x_parity: (rect.x0 & 1) as usize,
            y_parity: (rect.y0 & 1) as usize,
            reversible,
        };
        let end = (height - 1) * stride + width;
        if let Some(jobs) = jobs.filter(|_| width * height >= PARALLEL_LEVEL_AREA)
            && level.parallel::<VECTOR>(
                &mut data[..end],
                jobs,
                &mut workspace.parallel_lines,
                &mut workspace.line,
            )
        {
            continue;
        }
        if !reversible && row_wise(width, height, false) {
            level.columns_97_by_rows::<true>(&mut data[..end], &mut workspace.line, None);
        } else {
            level.rows::<VECTOR>(&mut data[..end], height, &mut workspace.line);
            level.columns::<VECTOR>(&mut data[..end], &mut workspace.line, use_column_batches);
        }
    }
    Ok(())
}

fn reserve_line(line: &mut Vec<i32>, size: usize) -> Result<(), Error> {
    if line.len() < size {
        line.try_reserve_exact(size - line.len()).map_err(|_| no_memory())?;
        line.resize(size, 0);
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct Level {
    width: usize,
    height: usize,
    stride: usize,
    x_parity: usize,
    y_parity: usize,
    reversible: bool,
}

impl Level {
    fn rows<const VECTOR: bool>(self, data: &mut [i32], height: usize, line: &mut [i32]) {
        let Self {
            width,
            stride,
            x_parity,
            reversible,
            ..
        } = self;
        let rows_done = 0;
        #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
        let rows_done = if VECTOR && !reversible && height >= vector::LANES {
            let rows = height / vector::LANES * vector::LANES;
            let lanes = line[..width * vector::LANES].as_chunks_mut::<{ vector::LANES }>().0;
            let groups = (0..rows).step_by(vector::LANES);
            // Two blocks in each band amortize the transpose setup.
            if width >= 4 * vector::LANES {
                for row in groups {
                    vector::inverse_97_rows(&mut data[row * stride..], stride, x_parity, lanes);
                }
            } else {
                for row in groups {
                    vector::inverse(&mut data[row * stride..], 1, stride, x_parity, false, lanes);
                }
            }
            rows
        } else {
            rows_done
        };
        for row in rows_done..height {
            let offset = row * stride;
            inverse_line(
                &mut data[offset..offset + width],
                1,
                x_parity,
                reversible,
                false,
                &mut line[..width],
            );
        }
    }

    fn columns<const VECTOR: bool>(self, data: &mut [i32], line: &mut [i32], use_column_batches: bool) {
        let Self {
            width,
            height,
            stride,
            y_parity,
            reversible,
            ..
        } = self;
        if reversible && row_wise(width, height, true) {
            self.columns_53_by_rows(data, line);
            return;
        }
        let group_end = width / GROUP_WIDTH_53 * GROUP_WIDTH_53;
        let columns_done = 0;
        #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
        let columns_done = if VECTOR {
            let columns = if reversible {
                group_end
            } else {
                width / vector::LANES * vector::LANES
            };
            if columns != 0 {
                let lanes = line[..height * vector::LANES].as_chunks_mut::<{ vector::LANES }>().0;
                for col in (0..columns).step_by(vector::LANES) {
                    vector::inverse(&mut data[col..], stride, 1, y_parity, reversible, lanes);
                }
            }
            columns
        } else {
            columns_done
        };
        let columns_done = if use_column_batches && width >= COLUMN_BATCH_97 {
            let columns = width / COLUMN_BATCH_97 * COLUMN_BATCH_97;
            let scratch = line[..height * COLUMN_BATCH_97].as_chunks_mut::<COLUMN_BATCH_97>().0;
            for col in (0..columns).step_by(COLUMN_BATCH_97) {
                inverse_97_columns(&mut data[col..], stride, y_parity, scratch);
            }
            columns
        } else {
            columns_done
        };
        for col in columns_done..width {
            inverse_line(
                &mut data[col..],
                stride,
                y_parity,
                reversible,
                col < group_end,
                &mut line[..height],
            );
        }
    }

    fn columns_97_by_rows<const FUSE_HORIZONTAL: bool>(
        self,
        data: &mut [i32],
        line: &mut [i32],
        parallel: Option<(Jobs<'_>, &mut Vec<Vec<i32>>)>,
    ) {
        let Self {
            width,
            height,
            stride,
            y_parity,
            ..
        } = self;
        let low_len = height / 2 + (height % 2) * (1 - y_parity);
        let Some((jobs, lines)) = parallel else {
            lifting97::lift_columns::<FUSE_HORIZONTAL>(data, width, height, stride, y_parity, self.x_parity);
            self.interleave_rows(data, low_len, line, true);
            return;
        };
        if !self.lift_97_stripes(data, jobs) {
            lifting97::lift_columns::<false>(data, width, height, stride, y_parity, self.x_parity);
        }
        if !self.parallel_interleave(data, low_len, jobs, lines, &mut line[..height]) {
            self.interleave_rows(data, low_len, line, true);
        }
    }

    fn lift_97_stripes(self, data: &mut [i32], jobs: Jobs<'_>) -> bool {
        let Self {
            width,
            height,
            stride,
            y_parity,
            ..
        } = self;
        let group = width.div_ceil(jobs.width() * GROUP_WIDTH_53) * GROUP_WIDTH_53;
        let count = width.div_ceil(group);
        if count < 2 {
            return false;
        }
        let Some(entries) = count.checked_mul(height) else {
            return false;
        };
        let mut stripes: Vec<&mut [i32]> = Vec::new();
        if stripes.try_reserve_exact(entries).is_err() {
            return false;
        }
        stripes.resize_with(entries, || &mut []);
        for (row_index, row) in data.chunks_mut(stride).enumerate() {
            for (column, segment) in row[..width].chunks_mut(group).enumerate() {
                stripes[column * height + row_index] = segment;
            }
        }
        jobs.run(stripes.chunks_exact_mut(height), |stripe| {
            lifting97::lift_stripe(stripe, y_parity)
        });
        true
    }

    fn columns_53_by_rows(self, data: &mut [i32], line: &mut [i32]) {
        let Self {
            width,
            height,
            stride,
            y_parity,
            ..
        } = self;
        let low_len = height / 2 + (height % 2) * (1 - y_parity);
        let (low, high) = data.split_at_mut(low_len * stride);
        let group_end = if height == 2 && y_parity == 1 {
            0
        } else {
            width / GROUP_WIDTH_53 * GROUP_WIDTH_53
        };
        for (index, row) in low.chunks_mut(stride).enumerate() {
            let pos = 2 * index + y_parity;
            let previous = pos.abs_diff(1) / 2 * stride;
            let left = &high[previous..previous + width];
            let row = &mut row[..width];
            if pos == 0 || pos + 1 == height {
                for (sample, &adjacent) in row[..group_end].iter_mut().zip(left) {
                    *sample = sample.wrapping_sub(edge_53(adjacent, true));
                }
                for (sample, &adjacent) in row[group_end..].iter_mut().zip(&left[group_end..]) {
                    *sample = sample.wrapping_sub(edge_53(adjacent, false));
                }
            } else {
                let next = pos.div_ceil(2) * stride;
                let right = &high[next..next + width];
                for ((sample, &a), &b) in row.iter_mut().zip(left).zip(right) {
                    *sample = sample.wrapping_sub(update_53(a, b));
                }
            }
        }
        for (index, row) in high.chunks_mut(stride).enumerate() {
            let pos = 2 * index + (1 - y_parity);
            let previous = pos.abs_diff(1) / 2 * stride;
            let left = &low[previous..previous + width];
            let row = &mut row[..width];
            if pos == 0 || pos + 1 == height {
                for (sample, &adjacent) in row.iter_mut().zip(left) {
                    *sample = sample.wrapping_add(adjacent);
                }
            } else {
                let next = pos.div_ceil(2) * stride;
                let right = &low[next..next + width];
                for ((sample, &a), &b) in row.iter_mut().zip(left).zip(right) {
                    *sample = predict_53(*sample, a, b);
                }
            }
        }
        self.interleave_rows(data, low_len, line, false);
    }

    fn interleave_rows(self, data: &mut [i32], low_len: usize, line: &mut [i32], split_columns: bool) {
        let Self {
            width,
            height,
            stride,
            y_parity,
            ..
        } = self;
        let (visited, row) = line[..height + width].split_at_mut(height);
        visited.fill(0);
        let input_row = |row: usize| {
            if row % 2 == y_parity {
                row / 2
            } else {
                low_len + row / 2
            }
        };
        for start in 0..height {
            if visited[start] != 0 || (!split_columns && input_row(start) == start) {
                continue;
            }
            row.copy_from_slice(&data[start * stride..start * stride + width]);
            let mut destination = start;
            loop {
                visited[destination] = 1;
                let source = input_row(destination);
                let to = destination * stride;
                if source == start {
                    if split_columns {
                        interleave_line(row, &mut data[to..to + width], self.x_parity);
                    } else {
                        data[to..to + width].copy_from_slice(row);
                    }
                    break;
                }
                let from = source * stride;
                if split_columns {
                    let [dst, src] = data
                        .get_disjoint_mut([to..to + width, from..from + width])
                        .expect("distinct rows of validated geometry");
                    interleave_line(src, dst, self.x_parity);
                } else {
                    data.copy_within(from..from + width, to);
                }
                destination = source;
            }
        }
    }

    fn parallel_interleave(
        self,
        data: &mut [i32],
        low_len: usize,
        jobs: Jobs<'_>,
        lines: &mut Vec<Vec<i32>>,
        positions: &mut [i32],
    ) -> bool {
        let Self {
            width,
            height,
            stride,
            x_parity,
            y_parity,
            ..
        } = self;
        #[derive(Default)]
        struct Row<'a> {
            samples: Option<&'a mut [i32]>,
            last: bool,
        }

        if i32::try_from(height).is_err() {
            return false;
        }
        let mut rows = Vec::<Row<'_>>::new();
        if rows.try_reserve_exact(height).is_err() {
            *lines = Vec::new();
            return false;
        }
        positions.fill(-1);
        rows.resize_with(height, Row::default);
        let mut edges = [0; MAX_JOBS + 1];
        let mut count = 1;
        let mut cursor = 0;
        let step = height.div_ceil(jobs.width());
        for start in 0..height {
            if positions[start] != -1 {
                continue;
            }
            let mut index = start;
            loop {
                positions[index] = i32::try_from(cursor).expect("each row fits the approved map");
                cursor += 1;
                index = if index % 2 == y_parity {
                    index / 2
                } else {
                    low_len + index / 2
                };
                if index == start {
                    break;
                }
            }
            rows[cursor - 1].last = true;
            if cursor < height && count < jobs.width() && cursor - edges[count - 1] >= step {
                edges[count] = cursor;
                count += 1;
            }
        }
        if count < 2 {
            return false;
        }
        if lines.len() < count {
            if lines.try_reserve_exact(count - lines.len()).is_err() {
                *lines = Vec::new();
                return false;
            }
            lines.resize_with(count, Vec::new);
        }
        if lines[..count].iter_mut().any(|line| reserve_line(line, width).is_err()) {
            *lines = Vec::new();
            return false;
        }
        edges[count] = height;
        for (&position, row) in positions.iter().zip(data.chunks_mut(stride)) {
            rows[usize::try_from(position).expect("each row is mapped")].samples = Some(&mut row[..width]);
        }

        // A cycle only reads rows that it will overwrite. Each job borrows whole cycles.
        let mut remaining = rows.as_mut_slice();
        let tasks = edges[..=count].windows(2).zip(lines).map(|(span, line)| {
            let (assigned, rest) = core::mem::take(&mut remaining).split_at_mut(span[1] - span[0]);
            remaining = rest;
            (assigned, &mut line[..width])
        });
        jobs.run(tasks, |(assigned, line)| {
            for cycle in assigned.split_inclusive_mut(|row| row.last) {
                line.copy_from_slice(cycle[0].samples.as_deref().expect("a mapped row"));
                let (last, body) = cycle.split_last_mut().expect("a cycle has at least one row");
                let last = last.samples.as_deref_mut().expect("a mapped row");
                for index in 0..body.len() {
                    let (to, rest) = body[index..].split_first_mut().expect("a row in the cycle");
                    let from = rest.first().and_then(|row| row.samples.as_deref()).unwrap_or(last);
                    interleave_line(from, to.samples.as_deref_mut().expect("a mapped row"), x_parity);
                }
                interleave_line(line, last, x_parity);
            }
        });
        true
    }

    fn parallel<const VECTOR: bool>(
        self,
        data: &mut [i32],
        jobs: Jobs<'_>,
        lines: &mut Vec<Vec<i32>>,
        serial_line: &mut [i32],
    ) -> bool {
        let Self {
            width, height, stride, ..
        } = self;
        let rows_per_job = height.div_ceil(jobs.width() * ROW_BATCH) * ROW_BATCH;
        let columns_per_job = width.div_ceil(jobs.width() * GROUP_WIDTH_53) * GROUP_WIDTH_53;
        let count = height.div_ceil(rows_per_job).max(width.div_ceil(columns_per_job));
        if count < 2 {
            return false;
        }
        if !self.reversible && row_wise(width, height, false) {
            let span = height.div_ceil(jobs.width()) * stride;
            jobs.run(data.chunks_mut(span), |batch| {
                for row in batch.chunks_mut(stride) {
                    lift_horizontal_97(&mut row[..width], self.x_parity);
                }
            });
            self.columns_97_by_rows::<false>(data, serial_line, Some((jobs, lines)));
            return true;
        }
        if lines.len() < count {
            if lines.try_reserve_exact(count - lines.len()).is_err() {
                *lines = Vec::new();
                return false;
            }
            lines.resize_with(count, Vec::new);
        }
        let vector = VECTOR && cfg!(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"));
        let row_lanes = if vector && !self.reversible && height >= ROW_BATCH {
            ROW_BATCH
        } else {
            1
        };
        let columns = if self.reversible {
            vector && width >= GROUP_WIDTH_53
        } else {
            width >= COLUMN_BATCH_97
        };
        let column_lanes = if columns { COLUMN_BATCH_97 } else { 1 };
        let Some(scratch_size) = width
            .checked_mul(row_lanes)
            .zip(height.checked_mul(column_lanes))
            .map(|(row, col)| row.max(col))
        else {
            return false;
        };
        if lines[..count]
            .iter_mut()
            .any(|line| reserve_line(line, scratch_size).is_err())
        {
            *lines = Vec::new();
            return false;
        }

        let transform_rows = |data: &mut [i32], lines: &mut Vec<Vec<i32>>| {
            jobs.run(data.chunks_mut(rows_per_job * stride).zip(lines), |(rows, line)| {
                self.rows::<VECTOR>(rows, rows.len().div_ceil(stride), line)
            });
        };

        // Allocate the row-slice tables before modifying the samples, so either pass can still run serially.
        let columns = width.div_ceil(columns_per_job);
        let mut stripes: [Vec<&mut [i32]>; MAX_JOBS] = [const { Vec::new() }; MAX_JOBS];
        let stripes = &mut stripes[..columns];
        if stripes
            .iter_mut()
            .any(|stripe| stripe.try_reserve_exact(height).is_err())
        {
            *lines = Vec::new();
            return false;
        }
        transform_rows(data, lines);

        // Each job borrows only its section of every row. Stripe boundaries preserve complete SIMD groups.
        for row in data.chunks_mut(stride) {
            for (stripe, segment) in stripes.iter_mut().zip(row[..width].chunks_mut(columns_per_job)) {
                stripe.push(segment);
            }
        }
        jobs.run(stripes.iter_mut().zip(lines), |(rows, line)| {
            self.stripe::<VECTOR>(rows, line)
        });
        true
    }

    fn stripe<const VECTOR: bool>(self, rows: &mut [&mut [i32]], line: &mut [i32]) {
        let Self {
            height,
            reversible,
            y_parity,
            ..
        } = self;
        if height < 2 {
            if reversible && y_parity == 1 {
                for sample in rows[0].iter_mut() {
                    *sample /= 2;
                }
            }
            return;
        }
        let width = rows[0].len();
        let group_end = width / GROUP_WIDTH_53 * GROUP_WIDTH_53;
        let done = 0;
        #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
        let done = if VECTOR {
            let end = if reversible {
                group_end
            } else {
                width / vector::LANES * vector::LANES
            };
            if end != 0 {
                let lanes = line[..height * vector::LANES].as_chunks_mut::<{ vector::LANES }>().0;
                for column in (0..end).step_by(vector::LANES) {
                    vector::inverse_stripe(rows, column, y_parity, reversible, lanes);
                }
            }
            end
        } else {
            done
        };

        if reversible {
            let low_len = height / 2 + (height % 2) * (1 - y_parity);
            let line = &mut line[..height];
            for column in done..width {
                let low = rows[..low_len].iter().map(|row| row[column]);
                let high = rows[low_len..].iter().map(|row| row[column]);
                inverse_53(low, high, y_parity, column < group_end, line);
                for (row, &sample) in rows.iter_mut().zip(line.iter()) {
                    row[column] = sample;
                }
            }
        } else {
            let batch_end = width / COLUMN_BATCH_97 * COLUMN_BATCH_97;
            for column in (done..batch_end).step_by(COLUMN_BATCH_97) {
                inverse_97_stripe(
                    rows,
                    column,
                    y_parity,
                    line[..height * COLUMN_BATCH_97].as_chunks_mut::<COLUMN_BATCH_97>().0,
                );
            }
            for column in batch_end.max(done)..width {
                inverse_97_stripe(rows, column, y_parity, line[..height].as_chunks_mut::<1>().0);
            }
        }
    }
}

fn row_wise(width: usize, height: usize, reversible: bool) -> bool {
    let area = if reversible { ROW_WISE_AREA_53 } else { ROW_WISE_AREA_97 };
    // Wide, short 5/3 planes also favour contiguous row lifts: column batches repeatedly
    // cross the large row stride even when the plane falls below the square-image area gate.
    let wide_53 = reversible && width >= 256 && height <= width / 4;
    width >= 32 && height > 1 && (width * height >= area || wide_53)
}

fn inverse_line(data: &mut [i32], stride: usize, parity: usize, reversible: bool, full_group: bool, line: &mut [i32]) {
    let len = line.len();
    if len < 2 {
        if len == 1 && reversible && parity == 1 {
            data[0] /= 2;
        }
        return;
    }
    let low_len = len / 2 + (len % 2) * (1 - parity);
    if !reversible {
        if stride == 1 {
            inverse_97_row(&mut data[..len], line, parity, low_len);
        } else {
            inverse_97_columns(data, stride, parity, line.as_chunks_mut::<1>().0);
        }
        return;
    }

    let (low, high) = data.split_at(low_len * stride);
    let low = low.iter().step_by(stride).copied();
    let high = high.iter().step_by(stride).take(len - low_len).copied();
    inverse_53(low, high, parity, full_group, line);
    if stride == 1 {
        data[..len].copy_from_slice(line);
    } else {
        for (out, &sample) in data.iter_mut().step_by(stride).zip(line.iter()) {
            *out = sample;
        }
    }
}

fn update_53(a: i32, b: i32) -> i32 {
    a.wrapping_add(b).wrapping_add(2) >> 2
}

fn edge_53(sample: i32, full_group: bool) -> i32 {
    if full_group {
        update_53(sample, sample)
    } else {
        sample.wrapping_add(1) >> 1
    }
}

fn predict_53(high: i32, left: i32, right: i32) -> i32 {
    high.wrapping_add(left.wrapping_add(right) >> 1)
}

// Each newly adjusted low sample finishes the previous high sample. Emit pairs directly into
// scratch instead of interleaving the bands and walking the scratch twice to lift them.
fn inverse_53(
    mut low: impl Iterator<Item = i32>,
    mut high: impl Iterator<Item = i32>,
    parity: usize,
    full_group: bool,
    line: &mut [i32],
) {
    let odd_length = !line.len().is_multiple_of(2);
    let first_low = low.next().expect("low band of a nontrivial line");
    let first_high = high.next().expect("high band of a nontrivial line");
    if parity == 0 {
        let (mut a, mut b) = (first_low.wrapping_sub(edge_53(first_high, full_group)), first_high);
        let (pairs, trailing) = line.as_chunks_mut::<2>();
        let (last, body) = pairs.split_last_mut().expect("at least one pair");
        for ((next_high, next_low), out) in high.zip(low.by_ref()).zip(body) {
            let adjusted = next_low.wrapping_sub(update_53(b, next_high));
            *out = [a, predict_53(b, a, adjusted)];
            (a, b) = (adjusted, next_high);
        }
        if odd_length {
            let adjusted = low
                .next()
                .expect("the extra low sample")
                .wrapping_sub(edge_53(b, full_group));
            *last = [a, predict_53(b, a, adjusted)];
            trailing[0] = adjusted;
        } else {
            *last = [a, b.wrapping_add(a)];
        }
        return;
    }

    let Some(mut b) = high.next() else {
        // The two-sample odd-origin shortcut uses the scalar mirror even in a full column group.
        let a = first_low.wrapping_sub(edge_53(first_high, false));
        line.copy_from_slice(&[first_high.wrapping_add(a), a]);
        return;
    };
    let mut a = first_low.wrapping_sub(update_53(first_high, b));
    line[0] = first_high.wrapping_add(a);
    let (pairs, trailing) = line[1..].as_chunks_mut::<2>();
    let (last, body) = pairs
        .split_last_mut()
        .expect("at least one pair after the odd-origin sample");
    for ((next_high, next_low), out) in high.zip(low.by_ref()).zip(body) {
        let adjusted = next_low.wrapping_sub(update_53(b, next_high));
        *out = [a, predict_53(b, a, adjusted)];
        (a, b) = (adjusted, next_high);
    }
    if odd_length {
        *last = [a, b.wrapping_add(a)];
    } else {
        let adjusted = low
            .next()
            .expect("the final low sample")
            .wrapping_sub(edge_53(b, full_group));
        *last = [a, predict_53(b, a, adjusted)];
        trailing[0] = adjusted;
    }
}

fn from_sample(sample: i32) -> f32 {
    f32::from_bits(sample.cast_unsigned())
}

fn to_sample(sample: f32) -> i32 {
    sample.to_bits().cast_signed()
}

fn lift_horizontal_97(row: &mut [i32], parity: usize) {
    if row.len() < 2 {
        return;
    }
    let low_len = row.len() / 2 + (row.len() % 2) * (1 - parity);
    let (low, high) = row.split_at_mut(low_len);
    for sample in low.iter_mut() {
        *sample = to_sample(from_sample(*sample) * K);
    }
    for sample in high.iter_mut() {
        *sample = to_sample(from_sample(*sample) * TWO_INV_K);
    }
    lift_97_bands(low, high, parity);
}

// Keep each band contiguous during the four lifts so interior samples can be processed together.
fn inverse_97_row(data: &mut [i32], line: &mut [i32], parity: usize, low_len: usize) {
    let (low, high) = line.split_at_mut(low_len);
    let (input_low, input_high) = data.split_at(low_len);
    for (out, &sample) in low.iter_mut().zip(input_low) {
        *out = to_sample(from_sample(sample) * K);
    }
    for (out, &sample) in high.iter_mut().zip(input_high) {
        *out = to_sample(from_sample(sample) * TWO_INV_K);
    }
    lift_97_bands(low, high, parity);
    interleave_line(line, data, parity);
}

fn lift_97_bands(low: &mut [i32], high: &mut [i32], parity: usize) {
    lift_97_split(low, high, parity == 0, -DELTA);
    lift_97_split(high, low, parity == 1, -GAMMA);
    lift_97_split(low, high, parity == 0, -BETA);
    lift_97_split(high, low, parity == 1, -ALPHA);
}

fn interleave_line(line: &[i32], data: &mut [i32], parity: usize) {
    let low_len = line.len() / 2 + (line.len() % 2) * (1 - parity);
    let (low, high) = line.split_at(low_len);
    let (first, second) = if parity == 0 { (low, high) } else { (high, low) };
    let (pairs, tail) = data.as_chunks_mut::<2>();
    for ((&a, &b), out) in first.iter().zip(second.iter()).zip(pairs) {
        *out = [a, b];
    }
    if let [last] = tail {
        *last = first[first.len() - 1];
    }
}

fn lift_97_split(mut band: &mut [i32], neighbour: &[i32], first: bool, coefficient: f32) {
    if first {
        let (sample, rest) = band.split_first_mut().expect("leading wavelet sample");
        // Add both neighbours even when they mirror the same sample.
        let mirrored = from_sample(neighbour[0]);
        *sample = to_sample(from_sample(*sample) + (mirrored + mirrored) * coefficient);
        band = rest;
    }
    let span = band.len().min(neighbour.len() - 1);
    let (body, tail) = band.split_at_mut(span);
    for ((sample, &left), &right) in body.iter_mut().zip(neighbour).zip(&neighbour[1..]) {
        let sum = from_sample(left) + from_sample(right);
        *sample = to_sample(from_sample(*sample) + sum * coefficient);
    }
    if let [last] = tail {
        // Match the far edge's doubled coefficient before multiplying the mirrored sample.
        let product = (coefficient + coefficient) * from_sample(neighbour[neighbour.len() - 1]);
        *last = to_sample(from_sample(*last) + product);
    }
}

// Adjacent columns are contiguous in the tile and in scratch, including on baseline targets.
fn inverse_97_columns<const LANES: usize>(data: &mut [i32], stride: usize, parity: usize, line: &mut [[i32; LANES]]) {
    if line.len() < 2 {
        return;
    }
    load_and_lift_97(data.chunks(stride), parity, line);
    for (row, samples) in data.chunks_mut(stride).zip(line) {
        row[..LANES].copy_from_slice(samples);
    }
}

fn inverse_97_stripe<const LANES: usize>(
    rows: &mut [&mut [i32]],
    column: usize,
    parity: usize,
    line: &mut [[i32; LANES]],
) {
    let columns = column..column + LANES;
    load_and_lift_97(rows.iter().map(|row| &row[columns.clone()]), parity, line);
    for (row, samples) in rows.iter_mut().zip(line) {
        row[columns.clone()].copy_from_slice(samples);
    }
}

fn load_and_lift_97<'a, const LANES: usize>(
    rows: impl Iterator<Item = &'a [i32]> + Clone,
    parity: usize,
    line: &mut [[i32; LANES]],
) {
    let low_len = line.len() / 2 + line.len() % 2 * (1 - parity);
    for (samples, row) in line.iter_mut().skip(parity).step_by(2).zip(rows.clone()) {
        scale_97_batch(samples, &row[..LANES], K);
    }
    for (samples, row) in line.iter_mut().skip(1 - parity).step_by(2).zip(rows.skip(low_len)) {
        scale_97_batch(samples, &row[..LANES], TWO_INV_K);
    }
    lift_97_batch(line, parity, -DELTA);
    lift_97_batch(line, 1 - parity, -GAMMA);
    lift_97_batch(line, parity, -BETA);
    lift_97_batch(line, 1 - parity, -ALPHA);
}

fn scale_97_batch<const LANES: usize>(samples: &mut [i32; LANES], input: &[i32], scale: f32) {
    for (out, &sample) in samples.iter_mut().zip(input) {
        *out = to_sample(from_sample(sample) * scale);
    }
}

fn lift_97_batch<const LANES: usize>(line: &mut [[i32; LANES]], parity: usize, coefficient: f32) {
    let last = line.len() - 1;
    for at in (parity..last).step_by(2) {
        let left = line[at.abs_diff(1)];
        let right = line[at + 1];
        let samples = &mut line[at];
        for lane in 0..LANES {
            let sum = from_sample(left[lane]) + from_sample(right[lane]);
            samples[lane] = to_sample(from_sample(samples[lane]) + sum * coefficient);
        }
    }
    if last % 2 == parity {
        let previous = line[last - 1];
        let samples = &mut line[last];
        let doubled = coefficient + coefficient;
        for lane in 0..LANES {
            let product = doubled * from_sample(previous[lane]);
            samples[lane] = to_sample(from_sample(samples[lane]) + product);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Executor;

    fn rect(x0: u32, y0: u32, x1: u32, y1: u32) -> Rect {
        Rect { x0, y0, x1, y1 }
    }

    struct ReverseJobs;

    impl Executor for ReverseJobs {
        fn concurrency(&self) -> usize {
            4
        }

        fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync)) {
            for index in (0..count).rev() {
                job(index);
            }
        }
    }

    #[test]
    fn avx2_irreversible_stripes_and_cycles_run_on_separate_threads() {
        struct TwoWorkers;
        impl Executor for TwoWorkers {
            fn concurrency(&self) -> usize {
                2
            }

            fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync)) {
                assert_eq!(count, 2);
                std::thread::scope(|scope| {
                    scope.spawn(|| job(1));
                    job(0);
                });
            }
        }

        let level = Level {
            width: 31,
            height: 9,
            stride: 34,
            x_parity: 1,
            y_parity: 0,
            reversible: false,
        };
        let mut scheduled: Vec<_> = (0..level.stride * level.height)
            .map(|index| to_sample(f32::from(u8::try_from(index % 23).unwrap()) / 5.0))
            .collect();
        let mut serial = scheduled.clone();
        let mut line = vec![0; level.width + level.height];
        level.columns_97_by_rows::<false>(&mut serial, &mut line, None);
        let jobs = Jobs::new(&TwoWorkers).unwrap();
        level.columns_97_by_rows::<false>(&mut scheduled, &mut line, Some((jobs, &mut Vec::new())));
        assert_eq!(scheduled, serial);
    }

    #[test]
    fn avx2_and_scalar_scheduled_wavelets_match_serial_with_edge_stripes_and_padding() {
        let host = ReverseJobs;
        let jobs = Jobs::new(&host).unwrap();
        let stride = 42;
        for (width, height) in [(37, 23), (38, 24), (17, 2), (37, 1), (1, 23)] {
            for reversible in [false, true] {
                let input: Vec<i32> = (0..height * stride)
                    .map(|index| {
                        if index % stride >= width {
                            -77
                        } else if reversible {
                            i32::try_from(index).unwrap().wrapping_mul(1_645_103_939)
                        } else {
                            to_sample((f32::from(u16::try_from(index % 79).unwrap()) - 39.0) / 4.0)
                        }
                    })
                    .collect();
                for parity in 0..=1 {
                    let level = Level {
                        width,
                        height,
                        stride,
                        x_parity: parity,
                        y_parity: 1 - parity,
                        reversible,
                    };
                    let mut serial = input.clone();
                    let mut scheduled = input.clone();
                    let mut line = vec![0; width.max(height) * COLUMN_BATCH_97];
                    level.rows::<true>(&mut serial, height, &mut line);
                    let horizontal = serial.clone();
                    let batches =
                        !reversible && !cfg!(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"));
                    level.columns::<true>(&mut serial, &mut line, batches);
                    let row_wise = if height > 1 {
                        let mut output = if reversible { horizontal } else { input.clone() };
                        if reversible {
                            level.columns_53_by_rows(&mut output, &mut line);
                        } else {
                            for row in output.chunks_mut(stride) {
                                lift_horizontal_97(&mut row[..width], level.x_parity);
                            }
                            let mut direct = input.clone();
                            level.columns_97_by_rows::<false>(&mut output, &mut line, Some((jobs, &mut Vec::new())));
                            level.columns_97_by_rows::<true>(&mut direct, &mut line, None);
                            for (index, ((&fused, &scheduled), &reference)) in
                                direct.iter().zip(&output).zip(&serial).enumerate()
                            {
                                assert_eq!(
                                    scheduled, reference,
                                    "scheduled row-wise {width}x{height}, parity {parity}, sample {index}"
                                );
                                assert_eq!(
                                    fused, reference,
                                    "fused columns {width}x{height}, parity {parity}, sample {index}"
                                );
                            }
                        }
                        Some(output)
                    } else {
                        None
                    };
                    assert!(level.parallel::<true>(&mut scheduled, jobs, &mut Vec::new(), &mut line));
                    assert_eq!(
                        scheduled, serial,
                        "{width}x{height}, parity {parity}, reversible {reversible}"
                    );
                    if let Some(output) = row_wise {
                        assert_eq!(output, serial, "row-wise {width}x{height}, parity {parity}");
                    }
                }
            }
        }
    }

    #[test]
    fn wide_reversible_levels_match_column_reference_with_parities_overflow_and_padding() {
        // Include both sides of the width/aspect gates, odd tails and the benchmark geometry.
        let shapes: &[(usize, usize)] = if cfg!(miri) {
            &[(256, 2), (257, 3)]
        } else {
            &[
                (255, 63),
                (256, 2),
                (256, 64),
                (256, 65),
                (257, 65),
                (2048, 64),
                (2049, 65),
            ]
        };
        for &(width, height) in shapes {
            let stride = width + 3;
            let input: Vec<i32> = (0..stride * height)
                .map(|i| i32::try_from(i).unwrap().wrapping_mul(1_645_103_939))
                .collect();
            for x_parity in 0..=1 {
                for y_parity in 0..=1 {
                    let level = Level {
                        width,
                        height,
                        stride,
                        x_parity,
                        y_parity,
                        reversible: true,
                    };
                    let mut expected = input.clone();
                    let mut actual = input.clone();
                    let mut line = vec![0; (width + height) * COLUMN_BATCH_97];
                    level.rows::<false>(&mut expected, height, &mut line);
                    for col in 0..width {
                        inverse_line(
                            &mut expected[col..],
                            stride,
                            y_parity,
                            true,
                            col < width / GROUP_WIDTH_53 * GROUP_WIDTH_53,
                            &mut line[..height],
                        );
                    }
                    level.rows::<true>(&mut actual, height, &mut line);
                    level.columns::<true>(&mut actual, &mut line, false);
                    assert_eq!(actual, expected, "{width}x{height}, parity {x_parity},{y_parity}");
                    for (before, after) in input.chunks(stride).zip(actual.chunks(stride)) {
                        assert_eq!(&before[width..], &after[width..]);
                    }
                }
            }
        }
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

    #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
    #[test]
    fn avx2_lanes_match_scalar_and_reuse_scratch() {
        let mut scalar = Workspace::default();
        let mut avx2 = Workspace::default();
        for (w, h, x, y) in [
            (16_u32, 2_u32, 0_u32, 1_u32),
            (17, 4, 1, 1),
            (9, 3, 1, 0),
            (3, 9, 0, 1),
            (17, 17, 1, 1),
            (33, 35, 1, 1),
        ] {
            let levels = [
                rect(x.div_ceil(2), y.div_ceil(2), (x + w).div_ceil(2), (y + h).div_ceil(2)),
                rect(x, y, x + w, y + h),
            ];
            let stride = usize::try_from(w + 5).unwrap();
            for reversible in [true, false] {
                let original: Vec<_> = (0..stride * usize::try_from(h).unwrap())
                    .map(|index| {
                        let word = u32::try_from(index).unwrap().wrapping_mul(0x9e37_79b9);
                        if reversible {
                            word.cast_signed()
                        } else {
                            to_sample((f32::from(word.to_le_bytes()[0]) - 127.0) / 16.0)
                        }
                    })
                    .collect();
                let mut expected = original.clone();
                inverse_with::<false>(&mut expected, stride, &levels, reversible, &mut scalar, None).unwrap();
                let mut actual = original.clone();
                inverse(&mut actual, stride, &levels, reversible, &mut avx2).unwrap();
                assert_eq!(actual, expected);

                let buffer = avx2.line.as_ptr();
                actual.copy_from_slice(&original);
                inverse(&mut actual, stride, &levels, reversible, &mut avx2).unwrap();
                assert_eq!(avx2.line.as_ptr(), buffer);
                assert_eq!(actual, expected);
            }
        }
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
                // A contiguous vertical line at each level must touch only its active samples.
                6 => (4, 4, 0, 3, 1, 13),
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
            let stride = if case == 6 { 1 } else { width + 1 + next() % 19 };
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

#[cfg(all(test, feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
mod safe_rows_tests;
