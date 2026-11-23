// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// OpenJPEG's coding passes, ported through src/t1.cpp. See the notice in t1.rs.

use core::cell::Cell;

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
// The widest code-block (t1.rs rejects more): `width + 2` cannot wrap.
const MAX_WIDTH: usize = 1024;

/// A row of flag words as three views of `width` words each: the words left of the columns, of the columns, and right
/// of them (the row is `width + 2` words wide: a pad on each side). A column `col < width` is `mid[col]`, its left
/// neighbour `left[col]` and its right one `right[col]`: three accesses with one bound, which the compiler knows.
struct Row<'f> {
    left: &'f [Cell<u32>],
    mid: &'f [Cell<u32>],
    right: &'f [Cell<u32>],
}

impl<'f> Row<'f> {
    #[inline(always)]
    fn new(row: &'f [Cell<u32>], width: usize) -> Self {
        Self {
            left: &row[..width],
            mid: &row[1..width + 1],
            right: &row[2..width + 2],
        }
    }
}

/// The flag rows a stripe touches (the one above, its own and the one below, which a significant sample updates), taken
/// from the block's flag array once per stripe. The views overlap, so the words are cells.
struct Stripe<'f> {
    above: Row<'f>,
    here: Row<'f>,
    below: Row<'f>,
    width: usize,
    rows: usize,
    vertical_causal: bool,
}

/// A column while the lanes of its stripe are decoded: its flag word (the significance and visited bits that the next
/// lane's context reads) and its sign (chi) bits, merged into the word when the column is done, so that the next
/// row's zero-coding context does not wait on the sign decision.
struct Column {
    flags: u32,
    chi: u32,
}

/// What the lanes of a pass take from the pass itself.
struct Context<'z> {
    zero_context: &'z [u8; 512],
    magnitude: i32,
}

#[inline(always)]
fn set_bits(cell: &Cell<u32>, bits: u32) {
    cell.set(cell.get() | bits);
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

    fn significance_context(zero_context: &[u8; 512], flags: u32) -> usize {
        usize::from(zero_context[(flags & SIGMA_NEIGHBOURS) as usize])
    }

    /// The flag rows of stripe `index`, of `rows` lanes (4 but for the last stripe of a block).
    #[inline(always)]
    fn stripe(flags: &mut [u32], width: usize, index: usize, rows: usize, vertical_causal: bool) -> Stripe<'_> {
        assert!(width <= MAX_WIDTH);
        let stride = width + 2;
        let cells = Cell::from_mut(&mut flags[index * stride..(index + 3) * stride]).as_slice_of_cells();
        Stripe {
            above: Row::new(&cells[..stride], width),
            here: Row::new(&cells[stride..2 * stride], width),
            below: Row::new(&cells[2 * stride..], width),
            width,
            rows,
            vertical_causal,
        }
    }

    #[inline(always)]
    fn sign(mq: &mut Mq<'_, '_>, stripe: &Stripe<'_>, col: usize, lane: usize, flags: u32) -> u32 {
        let shift = lane * 3;
        let mut lu = (flags >> shift) & 0x00aa;
        lu |= (stripe.here.left[col].get() >> (19 + shift)) & 1;
        lu |= (stripe.here.right[col].get() >> (17 + shift)) & (1 << 2);
        let north_shift = if lane == 0 { 14 } else { 15 + (lane - 1) * 3 };
        lu |= (flags >> north_shift) & (1 << 4);
        lu |= (flags >> (16 + shift)) & (1 << 6);
        let lu = lu as usize;
        mq.symbol(usize::from(SIGN_CONTEXT[lu])) ^ u32::from(SIGN_PREDICTION[lu])
    }

    /// Record the sample at `lane` of the column as significant with `sign`: its value in `sample`, the flag words of
    /// the neighbouring columns at once, and its own significance bit in `column.flags` and its sign in `column.chi`.
    /// The caller stores the column's own word when the column is done. Nothing reads a column's own sign bits before
    /// that store except `sign`, which is given the merged word.
    #[inline(always)]
    fn record(
        stripe: &Stripe<'_>,
        col: usize,
        column: &mut Column,
        magnitude: i32,
        lane: usize,
        sign: u32,
        sample: &mut i32,
    ) {
        *sample = if sign != 0 { -magnitude } else { magnitude };
        let shift = lane * 3;
        set_bits(&stripe.here.left[col], (1 << 5) << shift);
        set_bits(&stripe.here.right[col], (1 << 3) << shift);
        if lane == 0 && !stripe.vertical_causal {
            set_bits(&stripe.above.mid[col], (sign << 31) | (1 << 16));
            set_bits(&stripe.above.left[col], 1 << 17);
            set_bits(&stripe.above.right[col], 1 << 15);
        }
        if lane == 3 {
            set_bits(&stripe.below.mid[col], (sign << 18) | (1 << 1));
            set_bits(&stripe.below.left[col], 1 << 2);
            set_bits(&stripe.below.right[col], 1);
        }
        column.flags |= SIGMA_THIS << shift;
        column.chi |= (sign << CHI_THIS_BIT) << shift;
    }

    /// Decode the sign of the sample at `lane` of the column and record it.
    #[inline(always)]
    fn mark_sample(
        mq: &mut Mq<'_, '_>,
        stripe: &Stripe<'_>,
        col: usize,
        column: &mut Column,
        magnitude: i32,
        lane: usize,
        sample: &mut i32,
    ) {
        let sign = Self::sign(mq, stripe, col, lane, column.flags | column.chi);
        Self::record(stripe, col, column, magnitude, lane, sign, sample);
    }

    #[inline(always)]
    fn significance_step<const RAW: bool>(
        mq: &mut Mq<'_, '_>,
        stripe: &Stripe<'_>,
        context: &Context<'_>,
        col: usize,
        column: &mut Column,
        lane: usize,
        sample: &mut i32,
    ) {
        let shift = lane * 3;
        let flags = column.flags >> shift;
        if flags & (SIGMA_THIS | PI_THIS) != 0 || flags & SIGMA_NEIGHBOURS == 0 {
            return;
        }
        let significant = if RAW {
            mq.raw()
        } else {
            mq.symbol(Self::significance_context(context.zero_context, flags))
        };
        if significant != 0 {
            let sign = if RAW {
                mq.raw()
            } else {
                Self::sign(mq, stripe, col, lane, column.flags | column.chi)
            };
            Self::record(stripe, col, column, context.magnitude, lane, sign, sample);
        }
        column.flags |= PI_THIS << shift;
    }

    #[inline(always)]
    fn cleanup_step(
        mq: &mut Mq<'_, '_>,
        stripe: &Stripe<'_>,
        context: &Context<'_>,
        col: usize,
        column: &mut Column,
        lane: usize,
        sample: &mut i32,
    ) {
        let flags = column.flags >> (lane * 3);
        if flags & (SIGMA_THIS | PI_THIS) == 0
            && mq.symbol(Self::significance_context(context.zero_context, flags)) != 0
        {
            Self::mark_sample(mq, stripe, col, column, context.magnitude, lane, sample);
        }
    }

    /// The columns of a stripe of four lanes in the significance pass, the lanes unrolled: each is a constant, so the
    /// shifts are immediates, the tests of the first and the last lane (the neighbours above and below) are resolved
    /// at compile time, and the four sample rows are exactly `width` long.
    #[inline(always)]
    fn significance_full<const RAW: bool>(
        mq: &mut Mq<'_, '_>,
        stripe: &Stripe<'_>,
        context: &Context<'_>,
        samples: &mut [i32],
    ) {
        let width = stripe.width;
        let (d0, rest) = samples[..4 * width].split_at_mut(width);
        let (d1, rest) = rest.split_at_mut(width);
        let (d2, d3) = rest.split_at_mut(width);
        let d3 = &mut d3[..width];
        for col in 0..width {
            let mut column = Column {
                flags: stripe.here.mid[col].get(),
                chi: 0,
            };
            if column.flags == 0 || (column.flags | (column.flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                continue;
            }
            Self::significance_step::<RAW>(mq, stripe, context, col, &mut column, 0, &mut d0[col]);
            Self::significance_step::<RAW>(mq, stripe, context, col, &mut column, 1, &mut d1[col]);
            Self::significance_step::<RAW>(mq, stripe, context, col, &mut column, 2, &mut d2[col]);
            Self::significance_step::<RAW>(mq, stripe, context, col, &mut column, 3, &mut d3[col]);
            stripe.here.mid[col].set(column.flags | column.chi);
        }
    }

    /// The columns of the last stripe of a block, of fewer than four lanes.
    #[inline(always)]
    fn significance_partial<const RAW: bool>(
        mq: &mut Mq<'_, '_>,
        stripe: &Stripe<'_>,
        context: &Context<'_>,
        samples: &mut [i32],
    ) {
        let width = stripe.width;
        for col in 0..width {
            let mut column = Column {
                flags: stripe.here.mid[col].get(),
                chi: 0,
            };
            if column.flags == 0 || (column.flags | (column.flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                continue;
            }
            for lane in 0..stripe.rows {
                let sample = &mut samples[lane * width + col];
                Self::significance_step::<RAW>(mq, stripe, context, col, &mut column, lane, sample);
            }
            stripe.here.mid[col].set(column.flags | column.chi);
        }
    }

    pub(super) fn significance<const RAW: bool>(&mut self, state: &mut Decoder<'_>, bitplane: u32) {
        // The MQ registers live in locals for the length of the pass.
        let mut mq = state.enter();
        let one = 1i32 << bitplane;
        let context = Context {
            zero_context: &ZERO_CONTEXT.as_chunks::<512>().0[self.orient],
            magnitude: one | (one >> 1),
        };
        let (width, height) = (self.width, self.height);
        for index in 0..height.div_ceil(4) {
            let rows = (height - index * 4).min(4);
            let stripe = Self::stripe(self.flags, width, index, rows, self.vertical_causal);
            let samples = &mut self.data[index * 4 * width..];
            if rows == 4 {
                Self::significance_full::<RAW>(&mut mq, &stripe, &context, samples);
            } else {
                Self::significance_partial::<RAW>(&mut mq, &stripe, &context, samples);
            }
        }
        mq.leave();
    }

    pub(super) fn refinement<const RAW: bool>(&mut self, state: &mut Decoder<'_>, bitplane: u32) {
        let mut mq = state.enter();
        let half = (1i32 << bitplane) >> 1;
        let (width, height) = (self.width, self.height);
        for index in 0..height.div_ceil(4) {
            let rows = (height - index * 4).min(4);
            let stripe = Self::stripe(self.flags, width, index, rows, self.vertical_causal);
            let samples = &mut self.data[index * 4 * width..];
            for col in 0..width {
                let column = stripe.here.mid[col].get();
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
                    let value = &mut samples[lane * width + col];
                    *value += if decision != u32::from(*value < 0) { half } else { -half };
                    refined |= MU_THIS << shift;
                    eligible &= eligible - 1;
                }
                stripe.here.mid[col].set(column | refined);
            }
        }
        mq.leave();
    }

    /// The columns of a stripe of four lanes in the cleanup pass, the lanes unrolled as in `significance_full`. Only
    /// such a stripe takes the run of empty columns and the AGG decision. The AGG decision may have settled the
    /// lanes up to `start - 1` of its column.
    #[inline(always)]
    fn cleanup_full(mq: &mut Mq<'_, '_>, stripe: &Stripe<'_>, context: &Context<'_>, samples: &mut [i32]) {
        let width = stripe.width;
        let (d0, rest) = samples[..4 * width].split_at_mut(width);
        let (d1, rest) = rest.split_at_mut(width);
        let (d2, d3) = rest.split_at_mut(width);
        let d3 = &mut d3[..width];
        let mut next = 0;
        while next < width {
            let col = next;
            next += 1;
            let mut column = Column {
                flags: stripe.here.mid[col].get(),
                chi: 0,
            };
            if (column.flags | (column.flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                stripe.here.mid[col].set(column.flags & !ALL_PI);
                continue;
            }
            let mut start = 0;
            if column.flags == 0 {
                // A run of empty columns whose AGG decisions are 0 without renormalisation is taken at once.
                let run = mq.aggregate_run(&stripe.here.mid[col..]);
                if run != 0 {
                    next = col + run;
                    continue;
                }
                if mq.symbol(CONTEXT_AGGREGATE) == 0 {
                    continue;
                }
                let first = mq.symbol(CONTEXT_UNIFORM) as usize;
                let second = mq.symbol(CONTEXT_UNIFORM) as usize;
                let lane = (first << 1) | second;
                let sample = match lane {
                    0 => &mut d0[col],
                    1 => &mut d1[col],
                    2 => &mut d2[col],
                    _ => &mut d3[col],
                };
                Self::mark_sample(mq, stripe, col, &mut column, context.magnitude, lane, sample);
                start = lane + 1;
            }
            if start == 0 {
                Self::cleanup_step(mq, stripe, context, col, &mut column, 0, &mut d0[col]);
            }
            if start <= 1 {
                Self::cleanup_step(mq, stripe, context, col, &mut column, 1, &mut d1[col]);
            }
            if start <= 2 {
                Self::cleanup_step(mq, stripe, context, col, &mut column, 2, &mut d2[col]);
            }
            if start <= 3 {
                Self::cleanup_step(mq, stripe, context, col, &mut column, 3, &mut d3[col]);
            }
            stripe.here.mid[col].set((column.flags | column.chi) & !ALL_PI);
        }
    }

    /// The columns of the last stripe of a block, of fewer than four lanes (no run and no AGG decision).
    #[inline(always)]
    fn cleanup_partial(mq: &mut Mq<'_, '_>, stripe: &Stripe<'_>, context: &Context<'_>, samples: &mut [i32]) {
        let width = stripe.width;
        for col in 0..width {
            let mut column = Column {
                flags: stripe.here.mid[col].get(),
                chi: 0,
            };
            if (column.flags | (column.flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                stripe.here.mid[col].set(column.flags & !ALL_PI);
                continue;
            }
            for lane in 0..stripe.rows {
                let sample = &mut samples[lane * width + col];
                Self::cleanup_step(mq, stripe, context, col, &mut column, lane, sample);
            }
            stripe.here.mid[col].set((column.flags | column.chi) & !ALL_PI);
        }
    }

    pub(super) fn cleanup(&mut self, state: &mut Decoder<'_>, bitplane: u32, segment_symbol: bool) {
        let mut mq = state.enter();
        let one = 1i32 << bitplane;
        let context = Context {
            zero_context: &ZERO_CONTEXT.as_chunks::<512>().0[self.orient],
            magnitude: one | (one >> 1),
        };
        let (width, height) = (self.width, self.height);
        for index in 0..height.div_ceil(4) {
            let rows = (height - index * 4).min(4);
            let stripe = Self::stripe(self.flags, width, index, rows, self.vertical_causal);
            let samples = &mut self.data[index * 4 * width..];
            if rows == 4 {
                Self::cleanup_full(&mut mq, &stripe, &context, samples);
            } else {
                Self::cleanup_partial(&mut mq, &stripe, &context, samples);
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
