// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// OpenJPEG's encoder passes, transcribed from src/enc_t1.cpp. See LICENSE.OpenJPEG.

#[cfg(all(test, not(miri)))]
mod differential;
mod distortion;
mod mq;

use super::{Block, Pass, T1Params, memory_error, wavelet};
use crate::t1::tables::{SIGN_CONTEXT, SIGN_PREDICTION, ZERO_CONTEXT};
use crate::{Error, ErrorKind};
use mq::{CONTEXT_AGGREGATE, CONTEXT_UNIFORM, Encoder, Run};

const FRACTIONAL_BITS: u32 = 6;
const MAX_SIDE: usize = 64;
const MAX_SAMPLES: usize = MAX_SIDE * MAX_SIDE;
const MAX_FLAGS: usize = (MAX_SIDE + 2) * (MAX_SIDE.div_ceil(4) + 2);
const MAX_PASSES: usize = 3 * (31 - FRACTIONAL_BITS as usize);
const CODE_CAPACITY: usize = 1 + 74 + 3 * (MAX_PASSES - 2) + MAX_SAMPLES * 4;

const SIGMA_THIS: u32 = 1 << 4;
const SIGMA_NEIGHBOURS: u32 = 0x1ef;
const PI_THIS: u32 = 1 << 21;
const ALL_PI: u32 = PI_THIS | (PI_THIS << 3) | (PI_THIS << 6) | (PI_THIS << 9);
const ALL_SIGMA: u32 = SIGMA_THIS | (SIGMA_THIS << 3) | (SIGMA_THIS << 6) | (SIGMA_THIS << 9);
const PI_TO_SIGMA: u32 = PI_THIS.trailing_zeros() - SIGMA_THIS.trailing_zeros();
const CONTEXT_MAGNITUDE: u8 = 14;

#[derive(Debug, Default)]
pub(super) struct Workspace {
    samples: Vec<u32>,
    flags: Vec<u32>,
    bytes: Vec<u8>,
    passes: Vec<Pass>,
    pairs: Vec<u8>,
}

fn reserve_buffer<T: Copy + Default>(buffer: &mut Vec<T>, len: usize) -> Result<(), Error> {
    if buffer.len() < len {
        buffer
            .try_reserve_exact(len - buffer.len())
            .map_err(|_| memory_error())?;
        buffer.resize(len, T::default());
    }
    Ok(())
}

fn magnitude(value: u32) -> u32 {
    value & 0x7fff_ffff
}

fn distortion_table(refinement: bool, plane: u32) -> &'static [i16; 128] {
    match (refinement, plane == 0) {
        (false, false) => &distortion::SIGNIFICANCE,
        (false, true) => &distortion::SIGNIFICANCE_PLANE_ZERO,
        (true, false) => &distortion::REFINEMENT,
        (true, true) => &distortion::REFINEMENT_PLANE_ZERO,
    }
}

/// What the three passes need of a bit-plane: the magnitude bit, the distortion table and the shift.
///
/// `DIST` is whether the passes measure the distortion they remove; without it `decrease` is zero and the table loads
/// and sums vanish from the passes.
#[derive(Clone, Copy)]
struct Plane<const DIST: bool> {
    one: u32,
    shift: u32,
    significance: &'static [i16; 128],
    refinement: &'static [i16; 128],
}

impl<const DIST: bool> Plane<DIST> {
    fn new(shift: u32) -> Self {
        Self {
            one: 1 << (shift + FRACTIONAL_BITS),
            shift,
            significance: distortion_table(false, shift),
            refinement: distortion_table(true, shift),
        }
    }

    fn bit(self, sample: u32) -> u32 {
        u32::from(sample & self.one != 0)
    }

    fn decrease(self, table: &[i16; 128], sample: u32) -> i32 {
        if DIST {
            i32::from(table[((magnitude(sample) >> self.shift) & 0x7f) as usize])
        } else {
            0
        }
    }
}

struct Passes<'a, const DIST: bool> {
    data: &'a [u32],
    flags: &'a mut [u32],
    width: usize,
    height: usize,
    zero_context: &'static [u8; 512],
}

/// The refinement pass's magnitude contexts, doubled (a pair's bit goes in bit 0), by whether a neighbour is
/// significant (bit 0 of the index) and whether the sample was refined before (bit 1).
const MAGNITUDE_PAIR: [u8; 4] = [
    CONTEXT_MAGNITUDE * 2,
    CONTEXT_MAGNITUDE * 2 + 2,
    CONTEXT_MAGNITUDE * 2 + 4,
    CONTEXT_MAGNITUDE * 2 + 4,
];
const _: () = assert!(CONTEXT_MAGNITUDE * 2 + 4 < 128);

/// The refinement pass's pairs (context, bit), one byte a sample at most.
type PairList = [u8; MAX_SAMPLES];

/// The rows of a column that a cleanup pass may code: bit 3r is row r.
const ALL_ROWS: u32 = 0x249;

impl<const DIST: bool> Passes<'_, DIST> {
    fn initialize(&mut self) {
        self.flags.fill(0);
        let stride = self.width + 2;
        let stripes = self.height.div_ceil(4);
        self.flags[..stride].fill(ALL_PI);
        self.flags[(stripes + 1) * stride..].fill(ALL_PI);
        let rows = self.height % 4;
        if rows != 0 {
            let out_of_bounds = ALL_PI & (u32::MAX << (21 + rows * 3));
            self.flags[stripes * stride..(stripes + 1) * stride].fill(out_of_bounds);
        }
    }

    #[inline(always)]
    fn significance_context(&self, flags: u32) -> usize {
        usize::from(self.zero_context[(flags & SIGMA_NEIGHBOURS) as usize])
    }

    /// Codes the sign of the sample of row `LANE` of the column at `index`, which has just become significant, and
    /// marks it and its neighbours in the flags. The row is a constant, so that every shift is.
    #[inline(always)]
    fn sign<const LANE: usize>(&mut self, coder: &mut Run<'_>, index: usize, sample: u32, column_flags: &mut u32) {
        let flags = *column_flags;
        let shift = LANE * 3;
        let west = self.flags[index - 1];
        let east = self.flags[index + 1];
        let north_shift = if LANE == 0 { 14 } else { 12 + LANE * 3 };
        let mut lookup = (flags >> shift) & 0x00aa;
        lookup |= (west >> (19 + shift)) & 1;
        lookup |= (east >> (17 + shift)) & (1 << 2);
        lookup |= (flags >> north_shift) & (1 << 4);
        lookup |= (flags >> (16 + shift)) & (1 << 6);
        let lookup = (lookup & 0xff) as usize;
        let sign = sample >> 31;
        coder.symbol(
            usize::from(SIGN_CONTEXT[lookup]),
            sign ^ u32::from(SIGN_PREDICTION[lookup]),
        );

        self.flags[index - 1] = west | ((1 << 5) << shift);
        *column_flags = flags | (((sign << 19) | SIGMA_THIS) << shift);
        self.flags[index + 1] = east | ((1 << 3) << shift);
        if LANE == 0 {
            let above = index - self.width - 2;
            self.flags[above] |= (sign << 31) | (1 << 16);
            self.flags[above - 1] |= 1 << 17;
            self.flags[above + 1] |= 1 << 15;
        }
        if LANE == 3 {
            let below = index + self.width + 2;
            self.flags[below] |= (sign << 18) | (1 << 1);
            self.flags[below - 1] |= 1 << 2;
            self.flags[below + 1] |= 1;
        }
    }

    /// One row of a column in the significance-propagation pass.
    #[inline(always)]
    fn significance_lane<const LANE: usize>(
        &mut self,
        coder: &mut Run<'_>,
        index: usize,
        sample: u32,
        plane: Plane<DIST>,
        column_flags: &mut u32,
    ) -> i32 {
        let flags = *column_flags >> (LANE * 3);
        let mut change = 0;
        if flags & (SIGMA_THIS | PI_THIS) == 0 && flags & SIGMA_NEIGHBOURS != 0 {
            let significant = plane.bit(sample);
            coder.symbol(self.significance_context(flags), significant);
            if significant != 0 {
                self.sign::<LANE>(coder, index, sample, column_flags);
                change = plane.decrease(plane.significance, sample);
            }
            *column_flags |= PI_THIS << (LANE * 3);
        }
        change
    }

    /// The flags index of a stripe's first column.
    fn stripe_base(&self, stripe: usize) -> usize {
        (stripe + 1) * (self.width + 2) + 1
    }

    #[inline(never)]
    fn significance(&mut self, registers: &mut Encoder<'_>, plane: Plane<DIST>) -> i32 {
        let mut coder = registers.lend();
        let data = self.data;
        let (whole, partial) = data.split_at(self.height / 4 * 4 * self.width);
        let mut change = 0;
        for (stripe, samples) in whole.chunks_exact(4 * self.width).enumerate() {
            let base = self.stripe_base(stripe);
            for (column, lanes) in samples.as_chunks::<4>().0.iter().enumerate() {
                let index = base + column;
                let mut column_flags = self.flags[index];
                // Nothing to code in a column with no flags, or whose lanes are all significant
                // or padding (padding is marked visited).
                if column_flags == 0 || (column_flags | (column_flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                    continue;
                }
                change += self.significance_lane::<0>(&mut coder, index, lanes[0], plane, &mut column_flags);
                change += self.significance_lane::<1>(&mut coder, index, lanes[1], plane, &mut column_flags);
                change += self.significance_lane::<2>(&mut coder, index, lanes[2], plane, &mut column_flags);
                change += self.significance_lane::<3>(&mut coder, index, lanes[3], plane, &mut column_flags);
                self.flags[index] = column_flags;
            }
        }
        let rows = partial.len() / self.width;
        if rows != 0 {
            let base = self.stripe_base(self.height / 4);
            for (column, lanes) in partial.chunks_exact(rows).enumerate() {
                let index = base + column;
                let mut column_flags = self.flags[index];
                if column_flags == 0 || (column_flags | (column_flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                    continue;
                }
                change += self.significance_lane::<0>(&mut coder, index, lanes[0], plane, &mut column_flags);
                if let Some(&sample) = lanes.get(1) {
                    change += self.significance_lane::<1>(&mut coder, index, sample, plane, &mut column_flags);
                }
                if let Some(&sample) = lanes.get(2) {
                    change += self.significance_lane::<2>(&mut coder, index, sample, plane, &mut column_flags);
                }
                self.flags[index] = column_flags;
            }
        }
        let kept = coder.finish();
        registers.restore(kept);
        change
    }

    /// One row of a column in the refinement pass: its pair is stored whatever the row (the next pair overwrites
    /// it when the row is not refined), and the list grows by one when the column's `eligible` mask has the row.
    #[inline(always)]
    fn refine_row<const LANE: usize>(
        list: &mut PairList,
        length: &mut usize,
        flags: u32,
        eligible: u32,
        sample: u32,
        plane: Plane<DIST>,
        change: &mut i32,
    ) {
        let refined = (eligible >> (4 + 3 * LANE)) & 1 != 0;
        let row = flags >> (3 * LANE);
        let context = u32::from(row & SIGMA_NEIGHBOURS != 0) | ((row >> 19) & 2);
        list[*length & (MAX_SAMPLES - 1)] = MAGNITUDE_PAIR[(context & 3) as usize] | u8::from(plane.bit(sample) != 0);
        *length += usize::from(refined);
        *change += plane.decrease(plane.refinement, sample) & -i32::from(refined);
    }

    /// The refinement pass changes no significance or visited mark, so the samples it codes, their order and their
    /// contexts are fixed by the flags at its start: one scan lists the pairs of every stripe, and one loop codes
    /// the list.
    #[inline(never)]
    fn refine(&mut self, registers: &mut Encoder<'_>, plane: Plane<DIST>, list: &mut PairList) -> i32 {
        let mut coder = registers.lend();
        let data = self.data;
        let (whole, partial) = data.split_at(self.height / 4 * 4 * self.width);
        let mut length = 0;
        let mut change = 0;
        for (stripe, samples) in whole.chunks_exact(4 * self.width).enumerate() {
            let base = self.stripe_base(stripe);
            for (column, lanes) in samples.as_chunks::<4>().0.iter().enumerate() {
                let index = base + column;
                let flags = self.flags[index];
                // Padding lanes never get SIGMA_THIS, so only real samples are eligible.
                let eligible = flags & !(flags >> PI_TO_SIGMA) & ALL_SIGMA;
                if eligible == 0 {
                    continue;
                }
                Self::refine_row::<0>(list, &mut length, flags, eligible, lanes[0], plane, &mut change);
                Self::refine_row::<1>(list, &mut length, flags, eligible, lanes[1], plane, &mut change);
                Self::refine_row::<2>(list, &mut length, flags, eligible, lanes[2], plane, &mut change);
                Self::refine_row::<3>(list, &mut length, flags, eligible, lanes[3], plane, &mut change);
                self.flags[index] = flags | (eligible << 16);
            }
        }
        let rows = partial.len() / self.width;
        if rows != 0 {
            let base = self.stripe_base(self.height / 4);
            for (column, lanes) in partial.chunks_exact(rows).enumerate() {
                let index = base + column;
                let flags = self.flags[index];
                let eligible = flags & !(flags >> PI_TO_SIGMA) & ALL_SIGMA;
                if eligible == 0 {
                    continue;
                }
                Self::refine_row::<0>(list, &mut length, flags, eligible, lanes[0], plane, &mut change);
                if let Some(&sample) = lanes.get(1) {
                    Self::refine_row::<1>(list, &mut length, flags, eligible, sample, plane, &mut change);
                }
                if let Some(&sample) = lanes.get(2) {
                    Self::refine_row::<2>(list, &mut length, flags, eligible, sample, plane, &mut change);
                }
                self.flags[index] = flags | (eligible << 16);
            }
        }
        coder.pairs(&list[..length]);
        let kept = coder.finish();
        registers.restore(kept);
        change
    }

    /// One row of a column of the cleanup pass, coded when the column's candidate mask `rows` has it: its
    /// significance, and its sign when it is.
    #[inline(always)]
    fn cleanup_lane<const LANE: usize>(
        &mut self,
        coder: &mut Run<'_>,
        index: usize,
        sample: u32,
        plane: Plane<DIST>,
        rows: u32,
        column_flags: &mut u32,
    ) -> i32 {
        let mut change = 0;
        if (rows >> (3 * LANE)) & 1 != 0 {
            let flags = *column_flags >> (3 * LANE);
            let significant = plane.bit(sample);
            coder.symbol(self.significance_context(flags), significant);
            if significant != 0 {
                self.sign::<LANE>(coder, index, sample, column_flags);
                change = plane.decrease(plane.significance, sample);
            }
        }
        change
    }

    /// The first significant row of a run-mode column: its sign alone is coded.
    #[inline(always)]
    fn cleanup_first<const LANE: usize>(
        &mut self,
        coder: &mut Run<'_>,
        index: usize,
        lanes: &[u32; 4],
        plane: Plane<DIST>,
        column_flags: &mut u32,
    ) -> i32 {
        self.sign::<LANE>(coder, index, lanes[LANE], column_flags);
        plane.decrease(plane.significance, lanes[LANE])
    }

    #[inline(never)]
    fn cleanup(&mut self, registers: &mut Encoder<'_>, plane: Plane<DIST>) -> i32 {
        let mut coder = registers.lend();
        let data = self.data;
        let (whole, partial) = data.split_at(self.height / 4 * 4 * self.width);
        let mut change = 0;
        for (stripe, samples) in whole.chunks_exact(4 * self.width).enumerate() {
            let base = self.stripe_base(stripe);
            for (column, lanes) in samples.as_chunks::<4>().0.iter().enumerate() {
                let index = base + column;
                let mut column_flags = self.flags[index];
                if (column_flags | (column_flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                    self.flags[index] = column_flags & !ALL_PI;
                    continue;
                }
                if column_flags == 0 {
                    // Run mode: one decision for the column unless a row is significant at this plane.
                    let bits = u32::from(lanes[0] & plane.one != 0)
                        | (u32::from(lanes[1] & plane.one != 0) << 1)
                        | (u32::from(lanes[2] & plane.one != 0) << 2)
                        | (u32::from(lanes[3] & plane.one != 0) << 3);
                    coder.symbol(CONTEXT_AGGREGATE, u32::from(bits != 0));
                    if bits == 0 {
                        continue;
                    }
                    let first = bits.trailing_zeros();
                    coder.symbol(CONTEXT_UNIFORM, first >> 1);
                    coder.symbol(CONTEXT_UNIFORM, first & 1);
                    // No row is significant or visited yet, so every row after the first is coded.
                    match first {
                        0 => {
                            change += self.cleanup_first::<0>(&mut coder, index, lanes, plane, &mut column_flags);
                            change +=
                                self.cleanup_lane::<1>(&mut coder, index, lanes[1], plane, ALL_ROWS, &mut column_flags);
                            change +=
                                self.cleanup_lane::<2>(&mut coder, index, lanes[2], plane, ALL_ROWS, &mut column_flags);
                            change +=
                                self.cleanup_lane::<3>(&mut coder, index, lanes[3], plane, ALL_ROWS, &mut column_flags);
                        }
                        1 => {
                            change += self.cleanup_first::<1>(&mut coder, index, lanes, plane, &mut column_flags);
                            change +=
                                self.cleanup_lane::<2>(&mut coder, index, lanes[2], plane, ALL_ROWS, &mut column_flags);
                            change +=
                                self.cleanup_lane::<3>(&mut coder, index, lanes[3], plane, ALL_ROWS, &mut column_flags);
                        }
                        2 => {
                            change += self.cleanup_first::<2>(&mut coder, index, lanes, plane, &mut column_flags);
                            change +=
                                self.cleanup_lane::<3>(&mut coder, index, lanes[3], plane, ALL_ROWS, &mut column_flags);
                        }
                        _ => change += self.cleanup_first::<3>(&mut coder, index, lanes, plane, &mut column_flags),
                    }
                } else {
                    // A row is coded when it is neither significant nor visited, a set fixed at the column's
                    // start: coding a row changes that row's own significance mark and the neighbour marks of the
                    // others, never another row's own marks.
                    let rows = !((column_flags >> 4) | (column_flags >> 21)) & ALL_ROWS;
                    change += self.cleanup_lane::<0>(&mut coder, index, lanes[0], plane, rows, &mut column_flags);
                    change += self.cleanup_lane::<1>(&mut coder, index, lanes[1], plane, rows, &mut column_flags);
                    change += self.cleanup_lane::<2>(&mut coder, index, lanes[2], plane, rows, &mut column_flags);
                    change += self.cleanup_lane::<3>(&mut coder, index, lanes[3], plane, rows, &mut column_flags);
                }
                self.flags[index] = column_flags & !ALL_PI;
            }
        }
        let rows = partial.len() / self.width;
        if rows != 0 {
            let base = self.stripe_base(self.height / 4);
            // The visited marks of this stripe's real rows; the padding rows stay marked.
            let real_pi = ALL_PI & !(u32::MAX << (21 + rows * 3));
            for (column, lanes) in partial.chunks_exact(rows).enumerate() {
                let index = base + column;
                let mut column_flags = self.flags[index];
                if (column_flags | (column_flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                    self.flags[index] = column_flags & !real_pi;
                    continue;
                }
                let candidates = !((column_flags >> 4) | (column_flags >> 21)) & ALL_ROWS;
                change += self.cleanup_lane::<0>(&mut coder, index, lanes[0], plane, candidates, &mut column_flags);
                if let Some(&sample) = lanes.get(1) {
                    change += self.cleanup_lane::<1>(&mut coder, index, sample, plane, candidates, &mut column_flags);
                }
                if let Some(&sample) = lanes.get(2) {
                    change += self.cleanup_lane::<2>(&mut coder, index, sample, plane, candidates, &mut column_flags);
                }
                self.flags[index] = column_flags & !real_pi;
            }
        }
        let kept = coder.finish();
        registers.restore(kept);
        change
    }
}

/// Encode a viewer-profile code-block. The signed fixed-point input is in stripe-column order. Every pass carries
/// its weighted distortion decrease (the bench and oracle contract).
pub(super) fn encode(
    workspace: &mut Workspace,
    samples_i32: &[i32],
    width: usize,
    height: usize,
    params: T1Params,
    output: &mut Block,
) -> Result<(), Error> {
    encode_with::<true>(workspace, samples_i32, width, height, params, output)
}

/// Encode a code-block of a request that keeps every pass: the passes' symbols, rates, lengths and terminations are
/// those of [`encode`], but no distortion is measured and every pass's `distortion_dec` is zero. Only the packet
/// writer reads it, and it reads none when each layer takes all the passes.
pub(super) fn encode_all_passes(
    workspace: &mut Workspace,
    samples_i32: &[i32],
    width: usize,
    height: usize,
    params: T1Params,
    output: &mut Block,
) -> Result<(), Error> {
    encode_with::<false>(workspace, samples_i32, width, height, params, output)
}

fn encode_with<const DIST: bool>(
    workspace: &mut Workspace,
    samples_i32: &[i32],
    width: usize,
    height: usize,
    params: T1Params,
    output: &mut Block,
) -> Result<(), Error> {
    if width > MAX_SIDE || height > MAX_SIDE || samples_i32.len() != width * height || params.orientation > 3 {
        return Err(Error::new(
            ErrorKind::Argument,
            "Invalid encoder code-block dimensions or orientation\n",
        ));
    }
    output.numbps = 0;
    output.data.clear();
    output.passes.clear();
    reserve_buffer(&mut workspace.samples, samples_i32.len())?;
    let data = &mut workspace.samples[..samples_i32.len()];
    let mut maximum = 0;
    for (&value, dest) in samples_i32.iter().zip(data.iter_mut()) {
        let magnitude = value.unsigned_abs().min(0x7fff_ffff);
        maximum = maximum.max(magnitude);
        *dest = magnitude | (u32::from(value < 0) << 31);
    }
    if maximum == 0 {
        return Ok(());
    }
    // Fractions below one step retain OpenJPEG's wrapped numbps field but emit no passes.
    let numbps = (u32::BITS - maximum.leading_zeros()).wrapping_sub(FRACTIONAL_BITS);
    output.numbps = numbps;
    if !(1..=31 - FRACTIONAL_BITS).contains(&numbps) {
        return Ok(());
    }

    let flag_count = (width + 2) * (height.div_ceil(4) + 2);
    debug_assert!(flag_count <= MAX_FLAGS);
    reserve_buffer(&mut workspace.flags, flag_count)?;
    reserve_buffer(&mut workspace.bytes, CODE_CAPACITY)?;
    reserve_buffer(&mut workspace.pairs, MAX_SAMPLES)?;
    let pairs: &mut PairList = (&mut workspace.pairs[..MAX_SAMPLES])
        .try_into()
        .expect("a pair list is as long as the largest block");
    workspace.passes.clear();
    workspace
        .passes
        .try_reserve_exact(MAX_PASSES)
        .map_err(|_| memory_error())?;
    let mut coder = Encoder::new(&mut workspace.bytes);
    let mut passes = Passes::<DIST> {
        data,
        flags: &mut workspace.flags[..flag_count],
        width,
        height,
        zero_context: &ZERO_CONTEXT.as_chunks::<512>().0[usize::from(params.orientation)],
    };
    passes.initialize();
    let mut step_size = f64::from(params.step_size);
    if !params.reversible {
        step_size /= match params.orientation {
            0 => 1.0,
            3 => 4.0,
            _ => 2.0,
        };
    }
    let weight = if DIST {
        wavelet::colour_norm(params.component, params.reversible, params.mct)
            * wavelet::norm(params.level, params.orientation, params.reversible)
            * step_size
    } else {
        0.0
    };
    let mut total_distortion = 0.0;
    for plane in (0..numbps).rev() {
        let context = Plane::<DIST>::new(plane);
        let first = if plane == numbps - 1 { 2 } else { 0 };
        for pass_index in first..3 {
            let decrease = match pass_index {
                0 => passes.significance(&mut coder, context),
                1 => passes.refine(&mut coder, context, pairs),
                _ => passes.cleanup(&mut coder, context),
            };
            if DIST {
                let mut weighted = weight * f64::from(1 << plane);
                weighted *= weighted * f64::from(decrease) / 8192.0;
                total_distortion += weighted;
            }
            let terminated = pass_index == 2 && plane == 0;
            let rate = if terminated {
                coder.flush();
                coder.rate()
            } else {
                coder.rate().wrapping_add(3)
            };
            workspace.passes.push(Pass {
                rate,
                len: 0,
                distortion_dec: total_distortion,
                terminated,
            });
        }
    }
    if coder.overflowed() {
        return Err(Error::new(
            ErrorKind::Limit,
            "The encoded code-block exceeds the Tier-1 buffer\n",
        ));
    }
    let total_rate = coder.rate();
    let mut bounded_rate = total_rate;
    for pass in workspace.passes.iter_mut().rev() {
        pass.rate = pass.rate.min(bounded_rate);
        bounded_rate = pass.rate;
    }
    let mut previous_rate = 0u32;
    for pass in &mut workspace.passes {
        if workspace.bytes[pass.rate as usize] == 0xff {
            pass.rate -= 1;
        }
        pass.len = pass.rate.wrapping_sub(previous_rate);
        previous_rate = pass.rate;
    }
    let len = total_rate as usize;
    output.data.try_reserve_exact(len).map_err(|_| memory_error())?;
    output
        .passes
        .try_reserve_exact(workspace.passes.len())
        .map_err(|_| memory_error())?;
    output.data.extend_from_slice(&workspace.bytes[1..1 + len]);
    output.passes.extend_from_slice(&workspace.passes);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codeword_and_pass_distortion_match_the_cpp() {
        let mut workspace = Workspace::default();
        let mut block = Block::default();
        for (case, side, expected) in [
            (23u32, 16, 6_790_654_518_683_636_029u64),
            (63, 64, 14_005_651_912_185_562_873),
        ] {
            let mut random = 0x2fa9_3c95 ^ case;
            let samples: Vec<i32> = (0..side * side)
                .map(|_| {
                    random ^= random << 13;
                    random ^= random >> 17;
                    random ^= random << 5;
                    (i32::try_from(random % 1913).unwrap() - 956) * 64
                })
                .collect();
            let params = T1Params {
                orientation: u8::try_from(case % 4).unwrap(),
                level: ((case / 7) % 10) as usize,
                component: (case % 5) as usize,
                reversible: !case.is_multiple_of(3),
                step_size: 0.125 * ((case % 23) + 1) as f32,
                mct: !case.is_multiple_of(7),
            };
            encode(&mut workspace, &samples, side, side, params, &mut block).unwrap();
            let mut hash = 1_469_598_103_934_665_603u64;
            let mut add = |value: u64| hash = (hash ^ value).wrapping_mul(1_099_511_628_211);
            add(u64::from(block.numbps));
            add(block.passes.len() as u64);
            add(block.data.len() as u64);
            for &byte in &block.data {
                add(u64::from(byte));
            }
            for pass in &block.passes {
                add(u64::from(pass.rate));
                add(u64::from(pass.len));
                add(pass.distortion_dec.to_bits());
                add(u64::from(pass.terminated));
            }
            assert_eq!(hash, expected, "encoder code-block case {case}");
        }
    }

    #[test]
    fn all_passes_encoding_differs_from_the_full_one_only_in_the_distortion() {
        let mut workspace = Workspace::default();
        let (mut full, mut lean) = (Block::default(), Block::default());
        for case in 0..12u32 {
            let (width, height) = [(64, 64), (64, 7), (13, 64), (1, 1), (5, 6), (64, 63)][case as usize % 6];
            let mut random = 0x1f2e_3d4c ^ case.wrapping_mul(0x9e37_79b9);
            let samples: Vec<i32> = (0..width * height)
                .map(|_| {
                    random ^= random << 13;
                    random ^= random >> 17;
                    random ^= random << 5;
                    (i32::try_from(random % (1 << (6 + case % 9))).unwrap() - (1 << (5 + case % 9))) * 64
                })
                .collect();
            let params = T1Params {
                orientation: u8::try_from(case % 4).unwrap(),
                level: (case % 6) as usize,
                component: (case % 3) as usize,
                reversible: case % 2 == 0,
                step_size: 0.25 * ((case % 7) + 1) as f32,
                mct: case % 5 == 0,
            };
            encode(&mut workspace, &samples, width, height, params, &mut full).unwrap();
            encode_all_passes(&mut workspace, &samples, width, height, params, &mut lean).unwrap();
            assert_eq!(full.numbps, lean.numbps, "case {case}");
            assert_eq!(full.data, lean.data, "case {case}");
            assert_eq!(full.passes.len(), lean.passes.len(), "case {case}");
            for (full, lean) in full.passes.iter().zip(&lean.passes) {
                assert_eq!(
                    (full.rate, full.len, full.terminated),
                    (lean.rate, lean.len, lean.terminated)
                );
                assert_eq!(lean.distortion_dec.to_bits(), 0.0f64.to_bits());
            }
        }
    }

    #[test]
    fn fractional_only_block_has_no_passes() {
        let mut workspace = Workspace::default();
        let mut block = Block::default();
        let params = T1Params {
            orientation: 0,
            level: 0,
            component: 0,
            reversible: false,
            step_size: 1.0,
            mct: false,
        };
        encode(&mut workspace, &[-16], 1, 1, params, &mut block).unwrap();
        assert_eq!(block.numbps, u32::MAX);
        assert!(block.passes.is_empty());
        assert!(block.data.is_empty());
    }
}
