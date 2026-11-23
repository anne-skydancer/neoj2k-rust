// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// OpenJPEG's coding passes, ported through src/t1.cpp. See the notice in t1.rs.

use super::mq::{CONTEXT_AGGREGATE, CONTEXT_UNIFORM, Decoder};
use super::tables::{SIGN_CONTEXT, SIGN_PREDICTION, ZERO_CONTEXT};

const SIGMA_THIS: u32 = 1 << 4;
const SIGMA_NEIGHBOURS: u32 = 0x1ef;
const MU_THIS: u32 = 1 << 20;
const PI_THIS: u32 = 1 << 21;
const ALL_PI: u32 = PI_THIS | (PI_THIS << 3) | (PI_THIS << 6) | (PI_THIS << 9);
const ALL_SIGMA: u32 = SIGMA_THIS | (SIGMA_THIS << 3) | (SIGMA_THIS << 6) | (SIGMA_THIS << 9);
const PI_TO_SIGMA: u32 = PI_THIS.trailing_zeros() - SIGMA_THIS.trailing_zeros();
const CONTEXT_MAGNITUDE: usize = 14;

#[derive(Debug)]
pub(super) struct Passes<'a> {
    pub(super) data: &'a mut [i32],
    pub(super) flags: &'a mut [u32],
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) orient: usize,
    pub(super) vertical_causal: bool,
}

impl Passes<'_> {
    pub(super) fn initialize<const SINGLE_STRIPE: bool>(&mut self) {
        self.data.fill(0);
        if SINGLE_STRIPE {
            // One stripe needs only its side padding. No neighbour can write across a stripe.
            let padding = if self.height == 4 {
                0
            } else {
                ALL_PI & (u32::MAX << (21 + self.height * 3))
            };
            self.flags.fill(padding);
            return;
        }
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

    fn significance_context(zero_context: &[u8; 512], flags: u32) -> usize {
        usize::from(zero_context[(flags & SIGMA_NEIGHBOURS) as usize])
    }

    #[inline(always)]
    fn sign(&self, decoder: &mut Decoder<'_>, index: usize, row: usize, flags: u32) -> u32 {
        let shift = row * 3;
        let mut lu = (flags >> shift) & 0x00aa;
        lu |= (self.flags[index - 1] >> (19 + shift)) & 1;
        lu |= (self.flags[index + 1] >> (17 + shift)) & (1 << 2);
        let north_shift = if row == 0 { 14 } else { 15 + (row - 1) * 3 };
        lu |= (flags >> north_shift) & (1 << 4);
        lu |= (flags >> (16 + shift)) & (1 << 6);
        let lu = lu as usize;
        decoder.symbol(usize::from(SIGN_CONTEXT[lu])) ^ u32::from(SIGN_PREDICTION[lu])
    }

    // Update the neighbouring columns and return the bits for the current column.
    fn mark_significant<const SINGLE_STRIPE: bool>(&mut self, index: usize, row: usize, sign: u32) -> u32 {
        let shift = row * 3;
        self.flags[index - 1] |= (1 << 5) << shift;
        self.flags[index + 1] |= (1 << 3) << shift;
        if !SINGLE_STRIPE && row == 0 && !self.vertical_causal {
            let above = index - self.width - 2;
            self.flags[above] |= (sign << 31) | (1 << 16);
            self.flags[above - 1] |= 1 << 17;
            self.flags[above + 1] |= 1 << 15;
        }
        if !SINGLE_STRIPE && row == 3 {
            let below = index + self.width + 2;
            self.flags[below] |= (sign << 18) | (1 << 1);
            self.flags[below - 1] |= 1 << 2;
            self.flags[below + 1] |= 1;
        }
        ((sign << 19) | SIGMA_THIS) << shift
    }

    pub(super) fn significance<const RAW: bool, const SINGLE_STRIPE: bool>(
        &mut self,
        state: &mut Decoder<'_>,
        bitplane: u32,
    ) {
        // The MQ registers live in locals for the length of the pass.
        let mut local = *state;
        let decoder = &mut local;
        let one = 1i32 << bitplane;
        let magnitude = one | (one >> 1);
        let stride = self.width + 2;
        let zero_context = &ZERO_CONTEXT.as_chunks::<512>().0[self.orient];
        let height = if SINGLE_STRIPE { 4 } else { self.height };
        for row in (0..height).step_by(4) {
            let rows = if SINGLE_STRIPE {
                self.height
            } else {
                (self.height - row).min(4)
            };
            let data_start = row * self.width;
            let flag_start = if SINGLE_STRIPE { 1 } else { (row / 4 + 1) * stride + 1 };
            for col in 0..self.width {
                let index = flag_start + col;
                let mut column = self.flags[index];
                if column == 0 || (column | (column >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                    continue;
                }
                for lane in 0..rows {
                    let shift = lane * 3;
                    let flags = column >> shift;
                    if flags & (SIGMA_THIS | PI_THIS) != 0 || flags & SIGMA_NEIGHBOURS == 0 {
                        continue;
                    }
                    let significant = if RAW {
                        decoder.raw()
                    } else {
                        decoder.symbol(Self::significance_context(zero_context, flags))
                    };
                    if significant != 0 {
                        let sign = if RAW {
                            decoder.raw()
                        } else {
                            self.sign(decoder, index, lane, column)
                        };
                        self.data[data_start + col + lane * self.width] =
                            if sign != 0 { -magnitude } else { magnitude };
                        column |= self.mark_significant::<SINGLE_STRIPE>(index, lane, sign);
                    }
                    column |= PI_THIS << shift;
                }
                self.flags[index] = column;
            }
        }
        *state = local;
    }

    pub(super) fn refinement<const RAW: bool, const SINGLE_STRIPE: bool>(
        &mut self,
        state: &mut Decoder<'_>,
        bitplane: u32,
    ) {
        let mut local = *state;
        let decoder = &mut local;
        let half = (1i32 << bitplane) >> 1;
        let stride = self.width + 2;
        let height = if SINGLE_STRIPE { 4 } else { self.height };
        for row in (0..height).step_by(4) {
            let data_start = row * self.width;
            let flag_start = if SINGLE_STRIPE { 1 } else { (row / 4 + 1) * stride + 1 };
            for col in 0..self.width {
                let index = flag_start + col;
                let column = self.flags[index];
                // Only real samples can receive SIGMA_THIS. Any padding in the last stripe is skipped.
                let mut eligible = (column & !(column >> PI_TO_SIGMA)) & ALL_SIGMA;
                let mut refined = 0;
                while eligible != 0 {
                    let shift = eligible.trailing_zeros() - SIGMA_THIS.trailing_zeros();
                    let lane = (shift / 3) as usize;
                    let flags = column >> shift;
                    let decision = if RAW {
                        decoder.raw()
                    } else {
                        let context = if flags & MU_THIS != 0 {
                            2
                        } else {
                            usize::from(flags & SIGMA_NEIGHBOURS != 0)
                        };
                        decoder.symbol(CONTEXT_MAGNITUDE + context)
                    };
                    let value = &mut self.data[data_start + col + lane * self.width];
                    *value += if decision != u32::from(*value < 0) { half } else { -half };
                    refined |= MU_THIS << shift;
                    eligible &= eligible - 1;
                }
                self.flags[index] = column | refined;
            }
        }
        *state = local;
    }

    pub(super) fn cleanup<const SINGLE_STRIPE: bool>(
        &mut self,
        state: &mut Decoder<'_>,
        bitplane: u32,
        segment_symbol: bool,
    ) {
        let mut local = *state;
        let decoder = &mut local;
        let one = 1i32 << bitplane;
        let magnitude = one | (one >> 1);
        let stride = self.width + 2;
        let zero_context = &ZERO_CONTEXT.as_chunks::<512>().0[self.orient];
        let height = if SINGLE_STRIPE { 4 } else { self.height };
        for row in (0..height).step_by(4) {
            let rows = if SINGLE_STRIPE {
                self.height
            } else {
                (self.height - row).min(4)
            };
            let data_start = row * self.width;
            let flag_start = if SINGLE_STRIPE { 1 } else { (row / 4 + 1) * stride + 1 };
            let mut next = 0;
            while next < self.width {
                let col = next;
                next += 1;
                let index = flag_start + col;
                let mut column = self.flags[index];
                if (column | (column >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                    self.flags[index] = column & !ALL_PI;
                    continue;
                }
                if rows == 4 && column == 0 {
                    // A run of empty columns whose AGG decisions are 0 without renormalisation is taken at once.
                    let run = decoder.aggregate_run(&self.flags[index..flag_start + self.width]);
                    if run != 0 {
                        next = col + run;
                        continue;
                    }
                }
                let mut mark = |decoder: &mut Decoder<'_>, column: &mut u32, lane: usize| {
                    let sign = self.sign(decoder, index, lane, *column);
                    self.data[data_start + col + lane * self.width] = if sign != 0 { -magnitude } else { magnitude };
                    *column |= self.mark_significant::<SINGLE_STRIPE>(index, lane, sign);
                };
                let start = if rows == 4 && column == 0 {
                    if decoder.symbol(CONTEXT_AGGREGATE) == 0 {
                        continue;
                    }
                    let first = decoder.symbol(CONTEXT_UNIFORM) as usize;
                    let second = decoder.symbol(CONTEXT_UNIFORM) as usize;
                    let lane = (first << 1) | second;
                    mark(decoder, &mut column, lane);
                    lane + 1
                } else {
                    0
                };
                for lane in start..rows {
                    let flags = column >> (lane * 3);
                    if flags & (SIGMA_THIS | PI_THIS) == 0
                        && decoder.symbol(Self::significance_context(zero_context, flags)) != 0
                    {
                        mark(decoder, &mut column, lane);
                    }
                }
                self.flags[index] = column & !ALL_PI;
            }
        }
        if segment_symbol {
            // The C++ decoder consumes but does not check the four-bit segmentation symbol.
            for _ in 0..4 {
                decoder.symbol(CONTEXT_UNIFORM);
            }
        }
        *state = local;
    }
}
