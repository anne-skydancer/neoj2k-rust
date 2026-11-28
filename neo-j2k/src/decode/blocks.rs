// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Tier-1 writes of independent code-blocks into a tile component.

use super::{Reconstruction, WorkerScratch};
use crate::Error;
use crate::error::Diagnostics;
use crate::executor::{Jobs, MAX_JOBS};
use crate::geometry::Rect;
use crate::t1::{self, Destination};
use crate::tile::{Band, CodeBlock, Resolution, resize};
use core::cmp::Reverse;
use core::ops::Range;

struct Block<'a> {
    code: &'a CodeBlock,
    band: &'a Band,
    x: usize,
    y: usize,
}

fn blocks<const RESTRICT: bool>(
    resolutions: &[Resolution],
    output_rows: Range<usize>,
) -> impl Iterator<Item = Block<'_>> {
    resolutions.iter().enumerate().flat_map(move |(index, resolution)| {
        let coarser = index.checked_sub(1).map(|r| resolutions[r].rect);
        let output_rows = output_rows.clone();
        resolution.bands.iter().flat_map(move |band| {
            let horizontal = coarser.map_or(0, |r| if band.bandno & 1 == 0 { 0 } else { r.width() });
            let vertical = coarser.map_or(0, |r| if band.bandno & 2 == 0 { 0 } else { r.height() });
            let precincts = band.precincts.as_slice();
            let source_rows = |y: usize| (band.rect.y0 as usize).saturating_add(y.saturating_sub(vertical as usize));
            let rows = source_rows(output_rows.start)..source_rows(output_rows.end);
            let precincts = if RESTRICT {
                overlapping_rows(precincts, &rows, |precinct| precinct.rect)
            } else {
                precincts
            };
            precincts
                .iter()
                .flat_map(move |precinct| {
                    let codes = precinct.cblks.as_slice();
                    if RESTRICT {
                        overlapping_rows(codes, &rows, |code| code.rect)
                    } else {
                        codes
                    }
                })
                .filter(|code| code.rect.width() != 0 && code.rect.height() != 0)
                .map(move |code| Block {
                    code,
                    band,
                    x: (code.rect.x0 - band.rect.x0 + horizontal) as usize,
                    y: (code.rect.y0 - band.rect.y0 + vertical) as usize,
                })
        })
    })
}

// Both grids are stored in row order; find the intersecting rows without copying block metadata.
fn overlapping_rows<'a, T>(entries: &'a [T], rows: &Range<usize>, rect: impl Fn(&T) -> Rect) -> &'a [T] {
    if rows.is_empty() {
        return &[];
    }
    let start = entries.partition_point(|entry| rect(entry).y1 as usize <= rows.start);
    let end = entries.partition_point(|entry| (rect(entry).y0 as usize) < rows.end);
    entries.get(start..end).unwrap_or_default()
}

#[derive(Clone, Copy)]
struct Coding {
    stride: usize,
    roishift: u32,
    style: u32,
    reversible: bool,
}

impl Coding {
    fn new(component: &Reconstruction<'_>) -> Self {
        Self {
            stride: component.stride,
            roishift: u32::from(component.roishift),
            style: u32::from(component.cblksty),
            reversible: component.reversible,
        }
    }

    fn decode<const KNOWN_ZERO: bool>(
        self,
        block: &Block<'_>,
        samples: &mut [i32],
        origin: usize,
        data: &[u8],
        scratch: &mut t1::Scratch,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Error> {
        let code = block.code;
        scratch.decode_policy::<KNOWN_ZERO>(
            t1::Block {
                width: code.rect.width(),
                height: code.rect.height(),
                numbps: code.numbps,
                orient: block.band.bandno,
                roishift: self.roishift,
                cblksty: self.style,
                segments: &code.segs[..code.real_num_segs as usize],
            },
            code.chunks.iter().map(|chunk| &data[chunk.clone()]),
            Destination {
                samples,
                offset: (block.y - origin) * self.stride + block.x,
                stride: self.stride,
                reversible: self.reversible,
                stepsize: block.band.stepsize,
            },
            diagnostics,
        )
    }
}

pub(super) fn decode(
    component: &mut Reconstruction<'_>,
    data: &[u8],
    scratch: &mut t1::Scratch,
    diagnostics: &mut Diagnostics,
) -> Result<(), Error> {
    let coding = Coding::new(component);
    if component.known_zero && !component.reversible && component.samples.len() >= (1 << 18) {
        for block in blocks::<false>(component.res, 0..0) {
            coding.decode::<true>(&block, component.samples, 0, data, scratch, diagnostics)?;
        }
        return Ok(());
    }
    for block in blocks::<false>(component.res, 0..0) {
        coding.decode::<false>(&block, component.samples, 0, data, scratch, diagnostics)?;
    }
    Ok(())
}

pub(super) fn parallel_decode(
    component: &mut Reconstruction<'_>,
    jobs: Jobs<'_>,
    workers: &mut Vec<WorkerScratch>,
    data: &[u8],
) -> bool {
    let plane = Reconstruction {
        samples: &mut *component.samples,
        ..*component
    };
    parallel_decode_tile(&mut [(plane, 0)], jobs, workers, data)
}

#[derive(Default)]
struct Row {
    end: usize,
    cost: u64,
}

fn estimate_rows(component: &Reconstruction<'_>, rows: &mut [Row]) -> Option<u64> {
    let mut total = 0_u64;
    for block in blocks::<false>(component.res, 0..0) {
        let end = block.y + block.code.rect.height() as usize;
        if end > rows.len() {
            return None;
        }
        let bytes = block.code.chunks.iter().map(|chunk| chunk.len() as u64).sum::<u64>();
        let samples = u64::from(block.code.rect.width()) * u64::from(block.code.rect.height());
        // Empty blocks only clear their output rows. Give them less weight than coded blocks.
        let sample_cost = if block.code.chunks.is_empty() {
            samples.div_ceil(2)
        } else {
            samples
        };
        let cost = bytes.saturating_mul(100).saturating_add(sample_cost);
        let row = rows.get_mut(block.y)?;
        row.end = row.end.max(end);
        row.cost = row.cost.saturating_add(cost);
        total = total.saturating_add(cost);
    }
    Some(total)
}

struct Work<'a> {
    coding: Coding,
    resolutions: &'a [Resolution],
    rows: Range<usize>,
    samples: &'a mut [i32],
    cost: u64,
}

/// Decode independent row bands across all selected components in one pooled batch.
/// A failed batch discards its output and scratch so the caller can recover in serial order.
pub(super) fn parallel_decode_tile(
    planes: &mut [(Reconstruction<'_>, u32)],
    jobs: Jobs<'_>,
    workers: &mut Vec<WorkerScratch>,
    data: &[u8],
) -> bool {
    let width = jobs.width();
    // Wide pools need finer bands to balance differently coded components.
    let bands_per_worker = if planes.len() > 1 && width > 4 { 4 } else { 2 };
    let limit = (bands_per_worker * width).min(MAX_JOBS);
    if planes.is_empty() || planes.len() > limit {
        return false;
    }

    let mut heights = [0; MAX_JOBS];
    let mut height_sum = 0_usize;
    for ((plane, _), height) in planes.iter().zip(&mut heights) {
        if plane.stride == 0 || plane.samples.is_empty() || !plane.samples.len().is_multiple_of(plane.stride) {
            return false;
        }
        *height = plane.samples.len() / plane.stride;
        let Some(next) = height_sum.checked_add(*height) else {
            return false;
        };
        height_sum = next;
    }
    let mut estimates = Vec::<Row>::new();
    if resize(&mut estimates, height_sum).is_err() {
        *workers = Vec::new();
        return false;
    }
    let mut rows = estimates.as_mut_slice();
    let mut total = 0_u64;
    for ((plane, _), height) in planes.iter().zip(heights) {
        let (plane_rows, remaining) = core::mem::take(&mut rows).split_at_mut(height);
        rows = remaining;
        let Some(cost) = estimate_rows(plane, plane_rows) else {
            return false;
        };
        total = total.saturating_add(cost);
    }

    // Share one byte-and-sample work target. Reserve at least one job per later component and
    // cut only between rows no code-block crosses, so the jobs borrow disjoint slices.
    let target = total.div_ceil(limit as u64).max(1);
    let mut tasks: [Option<Work<'_>>; MAX_JOBS] = core::array::from_fn(|_| None);
    let mut rows = estimates.as_slice();
    let mut count = 0;
    let plane_count = planes.len();
    for (index, ((plane, _), height)) in planes.iter_mut().zip(heights).enumerate() {
        let limit = limit - (plane_count - index - 1);
        let (plane_rows, remaining) = rows.split_at(height);
        rows = remaining;
        let coding = Coding::new(plane);
        let resolutions = plane.res;
        let mut output = &mut *plane.samples;
        let mut origin = 0;
        let mut task = |end, cost| {
            let rows = origin..end;
            let (samples, remaining) = core::mem::take(&mut output).split_at_mut(rows.len() * coding.stride);
            output = remaining;
            origin = end;
            Some(Work {
                coding,
                resolutions,
                rows,
                samples,
                cost,
            })
        };
        let mut spent = 0_u64;
        let mut crossed_until = 0;
        for (row_index, row) in plane_rows.iter().enumerate() {
            if spent >= target && row_index >= crossed_until && count + 1 < limit {
                tasks[count] = task(row_index, spent);
                count += 1;
                spent = 0;
            }
            spent = spent.saturating_add(row.cost);
            crossed_until = crossed_until.max(row.end);
        }
        tasks[count] = task(height, spent);
        count += 1;
    }
    drop(estimates);

    if count < 2 {
        return false;
    }

    if workers.len() < count && resize(workers, count).is_err() {
        *workers = Vec::new();
        return false;
    }

    let tasks = &mut tasks[..count];
    tasks.sort_unstable_by_key(|task| Reverse(task.as_ref().expect("planned task").cost));
    let batch = tasks
        .iter_mut()
        .zip(&mut workers[..count])
        .map(|(task, worker)| (task.take().expect("planned task"), worker));
    jobs.run(batch, |(task, worker)| {
        let Work {
            coding,
            resolutions,
            rows,
            samples,
            ..
        } = task;
        let mut diagnostics = Diagnostics::new(false);
        let origin = rows.start;
        worker.result = Some(blocks::<true>(resolutions, rows).try_for_each(|block| {
            coding.decode::<false>(
                &block,
                samples,
                origin,
                data,
                &mut worker.component.tier1,
                &mut diagnostics,
            )
        }));
    });
    if workers[..count]
        .iter_mut()
        .all(|worker| worker.result.take().is_some_and(|result| result.is_ok()))
    {
        return true;
    }
    // Free worker scratch before the serial replay, which supplies the canonical diagnostics.
    *workers = Vec::new();
    for (plane, _) in planes {
        plane.samples.fill(0);
    }
    false
}
