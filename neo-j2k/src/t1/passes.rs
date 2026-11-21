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
    pub(super) fn initialize(&mut self) {
        self.data.fill(0);
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

    fn significance_context(&self, flags: u32) -> usize {
        usize::from(ZERO_CONTEXT[(self.orient << 9) | (flags & SIGMA_NEIGHBOURS) as usize])
    }

    fn sign(&self, decoder: &mut Decoder<'_>, index: usize, row: usize) -> u32 {
        let flags = self.flags[index];
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

    fn mark_significant(&mut self, index: usize, row: usize, sign: u32) {
        let shift = row * 3;
        self.flags[index - 1] |= (1 << 5) << shift;
        self.flags[index] |= ((sign << 19) | SIGMA_THIS) << shift;
        self.flags[index + 1] |= (1 << 3) << shift;
        if row == 0 && !self.vertical_causal {
            let above = index - self.width - 2;
            self.flags[above] |= (sign << 31) | (1 << 16);
            self.flags[above - 1] |= 1 << 17;
            self.flags[above + 1] |= 1 << 15;
        }
        if row == 3 {
            let below = index + self.width + 2;
            self.flags[below] |= (sign << 18) | (1 << 1);
            self.flags[below - 1] |= 1 << 2;
            self.flags[below + 1] |= 1;
        }
    }

    pub(super) fn significance(&mut self, decoder: &mut Decoder<'_>, bitplane: u32, raw: bool) {
        let one = 1i32 << bitplane;
        let magnitude = one | (one >> 1);
        let stride = self.width + 2;
        for row in (0..self.height).step_by(4) {
            let rows = (self.height - row).min(4);
            for col in 0..self.width {
                let index = (row / 4 + 1) * stride + col + 1;
                if self.flags[index] == 0 {
                    continue;
                }
                for lane in 0..rows {
                    let shift = lane * 3;
                    let flags = self.flags[index] >> shift;
                    if flags & (SIGMA_THIS | PI_THIS) != 0 || flags & SIGMA_NEIGHBOURS == 0 {
                        continue;
                    }
                    let significant = if raw {
                        decoder.raw()
                    } else {
                        decoder.symbol(self.significance_context(flags))
                    };
                    if significant != 0 {
                        let sign = if raw {
                            decoder.raw()
                        } else {
                            self.sign(decoder, index, lane)
                        };
                        self.data[(row + lane) * self.width + col] = if sign != 0 { -magnitude } else { magnitude };
                        self.mark_significant(index, lane, sign);
                    }
                    self.flags[index] |= PI_THIS << shift;
                }
            }
        }
    }

    pub(super) fn refinement(&mut self, decoder: &mut Decoder<'_>, bitplane: u32, raw: bool) {
        let half = (1i32 << bitplane) >> 1;
        let stride = self.width + 2;
        for row in (0..self.height).step_by(4) {
            let rows = (self.height - row).min(4);
            for col in 0..self.width {
                let index = (row / 4 + 1) * stride + col + 1;
                if self.flags[index] == 0 {
                    continue;
                }
                for lane in 0..rows {
                    let shift = lane * 3;
                    let flags = self.flags[index] >> shift;
                    if flags & (SIGMA_THIS | PI_THIS) != SIGMA_THIS {
                        continue;
                    }
                    let decision = if raw {
                        decoder.raw()
                    } else {
                        let context = if flags & MU_THIS != 0 {
                            2
                        } else {
                            usize::from(flags & SIGMA_NEIGHBOURS != 0)
                        };
                        decoder.symbol(CONTEXT_MAGNITUDE + context)
                    };
                    let value = &mut self.data[(row + lane) * self.width + col];
                    *value += if decision != u32::from(*value < 0) { half } else { -half };
                    self.flags[index] |= MU_THIS << shift;
                }
            }
        }
    }

    pub(super) fn cleanup(&mut self, decoder: &mut Decoder<'_>, bitplane: u32, segment_symbol: bool) {
        let one = 1i32 << bitplane;
        let magnitude = one | (one >> 1);
        let stride = self.width + 2;
        for row in (0..self.height).step_by(4) {
            let rows = (self.height - row).min(4);
            for col in 0..self.width {
                let index = (row / 4 + 1) * stride + col + 1;
                let run_start = if rows == 4 && self.flags[index] == 0 {
                    if decoder.symbol(CONTEXT_AGGREGATE) == 0 {
                        continue;
                    }
                    let first = decoder.symbol(CONTEXT_UNIFORM) as usize;
                    let second = decoder.symbol(CONTEXT_UNIFORM) as usize;
                    Some((first << 1) | second)
                } else {
                    None
                };
                for lane in run_start.unwrap_or(0)..rows {
                    let flags = self.flags[index] >> (lane * 3);
                    if run_start.is_none() && flags & (SIGMA_THIS | PI_THIS) != 0 {
                        continue;
                    }
                    if run_start != Some(lane) && decoder.symbol(self.significance_context(flags)) == 0 {
                        continue;
                    }
                    let sign = self.sign(decoder, index, lane);
                    self.data[(row + lane) * self.width + col] = if sign != 0 { -magnitude } else { magnitude };
                    self.mark_significant(index, lane, sign);
                }
                self.flags[index] &= !ALL_PI;
            }
        }
        if segment_symbol {
            // The C++ decoder consumes but does not check the four-bit segmentation symbol.
            for _ in 0..4 {
                decoder.symbol(CONTEXT_UNIFORM);
            }
        }
    }
}
