// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// OpenJPEG's coding passes, ported through src/t1.cpp. See the notice in t1.rs.

use super::mq::{CONTEXT_AGGREGATE, CONTEXT_UNIFORM, Decoder, Mq};
use super::tables::{SIGN_CONTEXT, SIGN_PREDICTION, ZERO_CONTEXT};

const SIGMA_THIS: u32 = 1 << 4;
const SIGMA_NEIGHBOURS: u32 = 0x1ef;
const MU_THIS: u32 = 1 << 20;
const PI_THIS: u32 = 1 << 21;
const ALL_PI: u32 = PI_THIS | (PI_THIS << 3) | (PI_THIS << 6) | (PI_THIS << 9);
const ALL_SIGMA: u32 = SIGMA_THIS | (SIGMA_THIS << 3) | (SIGMA_THIS << 6) | (SIGMA_THIS << 9);
const PI_TO_SIGMA: u32 = PI_THIS.trailing_zeros() - SIGMA_THIS.trailing_zeros();
const CONTEXT_MAGNITUDE: usize = 14;
const CHI_THIS_BIT: u32 = 19;
// The widest code-block (t1.rs rejects more). With it known, `width + 2` cannot wrap, so the three flag words of a
// column are provably inside a row of `width + 2`.
const MAX_WIDTH: usize = 1024;

/// What the passes of one stripe work on, taken from the block's two arrays once per stripe: the flag rows above, of
/// and below the stripe, each exactly `width + 2` words, and the sample rows of its (up to) four lanes. A column of the
/// stripe is then three flag words at `col .. col + 3` of a row and one sample at `col` of a lane's row, in range for
/// every `col < width`. Rows that the stripe does not have (a stripe of fewer than four lanes, the rows above and
/// below a single stripe) are empty and never read.
struct Stripe<'f> {
    above: &'f mut [u32],
    here: &'f mut [u32],
    below: &'f mut [u32],
    d0: &'f mut [i32],
    d1: &'f mut [i32],
    d2: &'f mut [i32],
    d3: &'f mut [i32],
    width: usize,
    rows: usize,
    vertical_causal: bool,
}

impl Stripe<'_> {
    #[inline(always)]
    fn sample_row(&mut self, lane: usize) -> &mut [i32] {
        match lane {
            0 => &mut *self.d0,
            1 => &mut *self.d1,
            2 => &mut *self.d2,
            _ => &mut *self.d3,
        }
    }
}

/// A column while the lanes of its stripe are decoded: its flag word (the significance and visited bits that the next
/// lane's context reads) and its sign (chi) bits, merged into the word when the column is done, so that the next
/// row's zero-coding context does not wait on the sign decision.
struct Column {
    flags: u32,
    chi: u32,
}

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

    /// The rows of stripe `index`, of `rows` lanes (4 but for the last stripe of a block, and the height of a single
    /// stripe).
    #[inline(always)]
    fn stripe<const SINGLE_STRIPE: bool>(&mut self, index: usize, rows: usize) -> Stripe<'_> {
        let width = self.width;
        assert!(width <= MAX_WIDTH);
        let stride = width + 2;
        let samples = &mut self.data[index * 4 * width..];
        let (d0, rest) = samples.split_at_mut(width.min(samples.len()));
        let (d1, rest) = rest.split_at_mut(width.min(rest.len()));
        let (d2, rest) = rest.split_at_mut(width.min(rest.len()));
        let (d3, _) = rest.split_at_mut(width.min(rest.len()));
        let vertical_causal = self.vertical_causal;
        if SINGLE_STRIPE {
            // One stripe needs only its own row: no neighbour can write across a stripe.
            Stripe {
                above: &mut [],
                here: &mut self.flags[..stride],
                below: &mut [],
                d0,
                d1,
                d2,
                d3,
                width,
                rows,
                vertical_causal,
            }
        } else {
            let (above, rest) = self.flags[index * stride..(index + 3) * stride].split_at_mut(stride);
            let (here, below) = rest.split_at_mut(stride);
            Stripe {
                above,
                here,
                below: &mut below[..stride],
                d0,
                d1,
                d2,
                d3,
                width,
                rows,
                vertical_causal,
            }
        }
    }

    #[inline(always)]
    fn sign(mq: &mut Mq<'_, '_>, stripe: &Stripe<'_>, col: usize, lane: usize, flags: u32) -> u32 {
        let shift = lane * 3;
        let mut lu = (flags >> shift) & 0x00aa;
        lu |= (stripe.here[col] >> (19 + shift)) & 1;
        lu |= (stripe.here[col + 2] >> (17 + shift)) & (1 << 2);
        let north_shift = if lane == 0 { 14 } else { 15 + (lane - 1) * 3 };
        lu |= (flags >> north_shift) & (1 << 4);
        lu |= (flags >> (16 + shift)) & (1 << 6);
        let lu = lu as usize;
        mq.symbol(usize::from(SIGN_CONTEXT[lu])) ^ u32::from(SIGN_PREDICTION[lu])
    }

    /// Record the sample at `lane` of the column as significant with `sign`: its value, the flag words of the
    /// neighbouring columns at once, and its own significance bit in `column.flags` and its sign in `column.chi`. The
    /// caller stores the column's own word when the column is done. Nothing reads a column's own sign bits before that
    /// store except `sign`, which is given the merged word.
    #[inline(always)]
    fn record<const SINGLE_STRIPE: bool>(
        stripe: &mut Stripe<'_>,
        col: usize,
        column: &mut Column,
        magnitude: i32,
        lane: usize,
        sign: u32,
    ) {
        stripe.sample_row(lane)[col] = if sign != 0 { -magnitude } else { magnitude };
        let shift = lane * 3;
        stripe.here[col] |= (1 << 5) << shift;
        stripe.here[col + 2] |= (1 << 3) << shift;
        if !SINGLE_STRIPE && lane == 0 && !stripe.vertical_causal {
            stripe.above[col + 1] |= (sign << 31) | (1 << 16);
            stripe.above[col] |= 1 << 17;
            stripe.above[col + 2] |= 1 << 15;
        }
        if !SINGLE_STRIPE && lane == 3 {
            stripe.below[col + 1] |= (sign << 18) | (1 << 1);
            stripe.below[col] |= 1 << 2;
            stripe.below[col + 2] |= 1;
        }
        column.flags |= SIGMA_THIS << shift;
        column.chi |= (sign << CHI_THIS_BIT) << shift;
    }

    /// Decode the sign of the sample at `lane` of the column and record it.
    #[inline(always)]
    fn mark_sample<const SINGLE_STRIPE: bool>(
        mq: &mut Mq<'_, '_>,
        stripe: &mut Stripe<'_>,
        col: usize,
        column: &mut Column,
        magnitude: i32,
        lane: usize,
    ) {
        let sign = Self::sign(mq, stripe, col, lane, column.flags | column.chi);
        Self::record::<SINGLE_STRIPE>(stripe, col, column, magnitude, lane, sign);
    }

    #[inline(always)]
    fn significance_step<const RAW: bool, const SINGLE_STRIPE: bool>(
        mq: &mut Mq<'_, '_>,
        stripe: &mut Stripe<'_>,
        zero_context: &[u8; 512],
        magnitude: i32,
        col: usize,
        column: &mut Column,
        lane: usize,
    ) {
        let shift = lane * 3;
        let flags = column.flags >> shift;
        if flags & (SIGMA_THIS | PI_THIS) != 0 || flags & SIGMA_NEIGHBOURS == 0 {
            return;
        }
        let significant = if RAW {
            mq.raw()
        } else {
            mq.symbol(Self::significance_context(zero_context, flags))
        };
        if significant != 0 {
            let sign = if RAW {
                mq.raw()
            } else {
                Self::sign(mq, stripe, col, lane, column.flags | column.chi)
            };
            Self::record::<SINGLE_STRIPE>(stripe, col, column, magnitude, lane, sign);
        }
        column.flags |= PI_THIS << shift;
    }

    #[inline(always)]
    fn cleanup_step<const SINGLE_STRIPE: bool>(
        mq: &mut Mq<'_, '_>,
        stripe: &mut Stripe<'_>,
        zero_context: &[u8; 512],
        magnitude: i32,
        col: usize,
        column: &mut Column,
        lane: usize,
    ) {
        let flags = column.flags >> (lane * 3);
        if flags & (SIGMA_THIS | PI_THIS) == 0 && mq.symbol(Self::significance_context(zero_context, flags)) != 0 {
            Self::mark_sample::<SINGLE_STRIPE>(mq, stripe, col, column, magnitude, lane);
        }
    }

    pub(super) fn significance<const RAW: bool, const SINGLE_STRIPE: bool>(
        &mut self,
        state: &mut Decoder<'_>,
        bitplane: u32,
    ) {
        // The MQ registers live in locals for the length of the pass.
        let mut mq = state.enter();
        let one = 1i32 << bitplane;
        let magnitude = one | (one >> 1);
        let zero_context = &ZERO_CONTEXT.as_chunks::<512>().0[self.orient];
        let height = self.height;
        for index in 0..if SINGLE_STRIPE { 1 } else { height.div_ceil(4) } {
            let rows = if SINGLE_STRIPE {
                height
            } else {
                (height - index * 4).min(4)
            };
            let mut stripe = self.stripe::<SINGLE_STRIPE>(index, rows);
            for col in 0..stripe.width {
                let mut column = Column {
                    flags: stripe.here[col + 1],
                    chi: 0,
                };
                if column.flags == 0 || (column.flags | (column.flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                    continue;
                }
                for lane in 0..stripe.rows {
                    Self::significance_step::<RAW, SINGLE_STRIPE>(
                        &mut mq,
                        &mut stripe,
                        zero_context,
                        magnitude,
                        col,
                        &mut column,
                        lane,
                    );
                }
                stripe.here[col + 1] = column.flags | column.chi;
            }
        }
        mq.leave();
    }

    pub(super) fn refinement<const RAW: bool, const SINGLE_STRIPE: bool>(
        &mut self,
        state: &mut Decoder<'_>,
        bitplane: u32,
    ) {
        let mut mq = state.enter();
        let half = (1i32 << bitplane) >> 1;
        let height = self.height;
        for index in 0..if SINGLE_STRIPE { 1 } else { height.div_ceil(4) } {
            let rows = if SINGLE_STRIPE {
                height
            } else {
                (height - index * 4).min(4)
            };
            let mut stripe = self.stripe::<SINGLE_STRIPE>(index, rows);
            for col in 0..stripe.width {
                let column = stripe.here[col + 1];
                // Only real samples can receive SIGMA_THIS. Any padding in the last stripe is skipped.
                let mut eligible = (column & !(column >> PI_TO_SIGMA)) & ALL_SIGMA;
                let mut refined = 0;
                while eligible != 0 {
                    let shift = eligible.trailing_zeros() - SIGMA_THIS.trailing_zeros();
                    let lane = (shift / 3) as usize;
                    let flags = column >> shift;
                    let decision = if RAW {
                        mq.raw()
                    } else {
                        let context = if flags & MU_THIS != 0 {
                            2
                        } else {
                            usize::from(flags & SIGMA_NEIGHBOURS != 0)
                        };
                        mq.symbol(CONTEXT_MAGNITUDE + context)
                    };
                    let value = &mut stripe.sample_row(lane)[col];
                    *value += if decision != u32::from(*value < 0) { half } else { -half };
                    refined |= MU_THIS << shift;
                    eligible &= eligible - 1;
                }
                stripe.here[col + 1] = column | refined;
            }
        }
        mq.leave();
    }

    pub(super) fn cleanup<const SINGLE_STRIPE: bool>(
        &mut self,
        state: &mut Decoder<'_>,
        bitplane: u32,
        segment_symbol: bool,
    ) {
        let mut mq = state.enter();
        let one = 1i32 << bitplane;
        let magnitude = one | (one >> 1);
        let zero_context = &ZERO_CONTEXT.as_chunks::<512>().0[self.orient];
        let height = self.height;
        for index in 0..if SINGLE_STRIPE { 1 } else { height.div_ceil(4) } {
            let rows = if SINGLE_STRIPE {
                height
            } else {
                (height - index * 4).min(4)
            };
            let mut stripe = self.stripe::<SINGLE_STRIPE>(index, rows);
            let width = stripe.width;
            let mut next = 0;
            while next < width {
                let col = next;
                next += 1;
                let mut column = Column {
                    flags: stripe.here[col + 1],
                    chi: 0,
                };
                if (column.flags | (column.flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                    stripe.here[col + 1] = column.flags & !ALL_PI;
                    continue;
                }
                if rows == 4 && column.flags == 0 {
                    // A run of empty columns whose AGG decisions are 0 without renormalisation is taken at once.
                    let run = mq.aggregate_run(&stripe.here[col + 1..width + 1]);
                    if run != 0 {
                        next = col + run;
                        continue;
                    }
                }
                let start = if rows == 4 && column.flags == 0 {
                    if mq.symbol(CONTEXT_AGGREGATE) == 0 {
                        continue;
                    }
                    let first = mq.symbol(CONTEXT_UNIFORM) as usize;
                    let second = mq.symbol(CONTEXT_UNIFORM) as usize;
                    let lane = (first << 1) | second;
                    Self::mark_sample::<SINGLE_STRIPE>(&mut mq, &mut stripe, col, &mut column, magnitude, lane);
                    lane + 1
                } else {
                    0
                };
                for lane in start..rows {
                    Self::cleanup_step::<SINGLE_STRIPE>(
                        &mut mq,
                        &mut stripe,
                        zero_context,
                        magnitude,
                        col,
                        &mut column,
                        lane,
                    );
                }
                stripe.here[col + 1] = (column.flags | column.chi) & !ALL_PI;
            }
        }
        if segment_symbol {
            // The C++ decoder consumes but does not check the four-bit segmentation symbol.
            for _ in 0..4 {
                mq.symbol(CONTEXT_UNIFORM);
            }
        }
        mq.leave();
    }
}
