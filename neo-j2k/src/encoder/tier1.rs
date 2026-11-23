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
use mq::{CONTEXT_AGGREGATE, CONTEXT_UNIFORM, Encoder};

const FRACTIONAL_BITS: u32 = 6;
const MAX_SIDE: usize = 64;
const MAX_SAMPLES: usize = MAX_SIDE * MAX_SIDE;
const MAX_FLAGS: usize = (MAX_SIDE + 2) * (MAX_SIDE.div_ceil(4) + 2);
const MAX_PASSES: usize = 3 * (31 - FRACTIONAL_BITS as usize);
const CODE_CAPACITY: usize = 1 + 74 + 3 * (MAX_PASSES - 2) + MAX_SAMPLES * 4;

const SIGMA_THIS: u32 = 1 << 4;
const SIGMA_NEIGHBOURS: u32 = 0x1ef;
const MU_THIS: u32 = 1 << 20;
const PI_THIS: u32 = 1 << 21;
const ALL_PI: u32 = PI_THIS | (PI_THIS << 3) | (PI_THIS << 6) | (PI_THIS << 9);
const ALL_SIGMA: u32 = SIGMA_THIS | (SIGMA_THIS << 3) | (SIGMA_THIS << 6) | (SIGMA_THIS << 9);
const PI_TO_SIGMA: u32 = PI_THIS.trailing_zeros() - SIGMA_THIS.trailing_zeros();
const CONTEXT_MAGNITUDE: usize = 14;

#[derive(Debug, Default)]
pub(super) struct Workspace {
    samples: Vec<u32>,
    flags: Vec<u32>,
    bytes: Vec<u8>,
    passes: Vec<Pass>,
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
#[derive(Clone, Copy)]
struct Plane {
    one: u32,
    shift: u32,
    significance: &'static [i16; 128],
    refinement: &'static [i16; 128],
}

impl Plane {
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
        i32::from(table[((magnitude(sample) >> self.shift) & 0x7f) as usize])
    }
}

struct Passes<'a> {
    data: &'a [u32],
    flags: &'a mut [u32],
    width: usize,
    height: usize,
    zero_context: &'static [u8; 512],
}

impl Passes<'_> {
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

    fn significance_context(&self, flags: u32) -> usize {
        usize::from(self.zero_context[(flags & SIGMA_NEIGHBOURS) as usize])
    }

    fn sign(&mut self, coder: &mut Encoder<'_>, index: usize, lane: usize, sample: u32, column_flags: &mut u32) {
        let flags = *column_flags;
        let shift = lane * 3;
        let mut lookup = (flags >> shift) & 0x00aa;
        lookup |= (self.flags[index - 1] >> (19 + shift)) & 1;
        lookup |= (self.flags[index + 1] >> (17 + shift)) & (1 << 2);
        let north_shift = if lane == 0 { 14 } else { 15 + (lane - 1) * 3 };
        lookup |= (flags >> north_shift) & (1 << 4);
        lookup |= (flags >> (16 + shift)) & (1 << 6);
        let lookup = lookup as usize;
        let sign = sample >> 31;
        coder.symbol(
            usize::from(SIGN_CONTEXT[lookup]),
            sign ^ u32::from(SIGN_PREDICTION[lookup]),
        );

        self.flags[index - 1] |= (1 << 5) << shift;
        *column_flags |= ((sign << 19) | SIGMA_THIS) << shift;
        self.flags[index + 1] |= (1 << 3) << shift;
        if lane == 0 {
            let above = index - self.width - 2;
            self.flags[above] |= (sign << 31) | (1 << 16);
            self.flags[above - 1] |= 1 << 17;
            self.flags[above + 1] |= 1 << 15;
        }
        if lane == 3 {
            let below = index + self.width + 2;
            self.flags[below] |= (sign << 18) | (1 << 1);
            self.flags[below - 1] |= 1 << 2;
            self.flags[below + 1] |= 1;
        }
    }

    /// One lane of the significance-propagation pass. The caller has bounded `lane` by the
    /// stripe's rows.
    #[inline(always)]
    fn significance_lane(
        &mut self,
        coder: &mut Encoder<'_>,
        index: usize,
        lane: usize,
        sample: u32,
        plane: Plane,
        column_flags: &mut u32,
    ) -> i32 {
        let shift = lane * 3;
        let flags = *column_flags >> shift;
        let mut change = 0;
        if flags & (SIGMA_THIS | PI_THIS) == 0 && flags & SIGMA_NEIGHBOURS != 0 {
            let significant = plane.bit(sample);
            coder.symbol(self.significance_context(flags), significant);
            if significant != 0 {
                self.sign(coder, index, lane, sample, column_flags);
                change = plane.decrease(plane.significance, sample);
            }
            *column_flags |= PI_THIS << shift;
        }
        change
    }

    /// The flags index of a stripe's first column, and the samples of a stripe column.
    fn stripe_base(&self, row: usize) -> usize {
        (row / 4 + 1) * (self.width + 2) + 1
    }

    fn significance(&mut self, coder: &mut Encoder<'_>, plane: Plane) -> i32 {
        let data = self.data;
        let mut change = 0;
        for row in (0..self.height).step_by(4) {
            let rows = (self.height - row).min(4);
            let base = self.stripe_base(row);
            for column in 0..self.width {
                let index = base + column;
                let mut column_flags = self.flags[index];
                // Nothing to code in a column with no flags, or whose lanes are all significant
                // or padding (padding is marked visited).
                if column_flags == 0 || (column_flags | (column_flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                    continue;
                }
                let samples = &data[row * self.width + column * rows..][..rows];
                for (lane, &sample) in samples.iter().enumerate() {
                    change += self.significance_lane(coder, index, lane, sample, plane, &mut column_flags);
                }
                self.flags[index] = column_flags;
            }
        }
        change
    }

    fn refine(&mut self, coder: &mut Encoder<'_>, plane: Plane) -> i32 {
        let data = self.data;
        let mut change = 0;
        for row in (0..self.height).step_by(4) {
            let rows = (self.height - row).min(4);
            let base = self.stripe_base(row);
            for column in 0..self.width {
                let index = base + column;
                let column_flags = self.flags[index];
                // Padding lanes never get SIGMA_THIS, so only real samples are eligible.
                let mut eligible = (column_flags & !(column_flags >> PI_TO_SIGMA)) & ALL_SIGMA;
                if eligible == 0 {
                    continue;
                }
                let samples = &data[row * self.width + column * rows..][..rows];
                let mut refined = 0;
                while eligible != 0 {
                    let shift = eligible.trailing_zeros() - SIGMA_THIS.trailing_zeros();
                    let flags = column_flags >> shift;
                    let context = if flags & MU_THIS != 0 {
                        2
                    } else {
                        usize::from(flags & SIGMA_NEIGHBOURS != 0)
                    };
                    let sample = samples[(shift / 3) as usize];
                    coder.symbol(CONTEXT_MAGNITUDE + context, plane.bit(sample));
                    change += plane.decrease(plane.refinement, sample);
                    refined |= MU_THIS << shift;
                    eligible &= eligible - 1;
                }
                self.flags[index] = column_flags | refined;
            }
        }
        change
    }

    fn cleanup(&mut self, coder: &mut Encoder<'_>, plane: Plane) -> i32 {
        let data = self.data;
        let mut change = 0;
        for row in (0..self.height).step_by(4) {
            let rows = (self.height - row).min(4);
            let base = self.stripe_base(row);
            // The visited marks of this stripe's real lanes; padding lanes stay marked.
            let real_pi = if rows == 4 {
                ALL_PI
            } else {
                ALL_PI & !(u32::MAX << (21 + rows * 3))
            };
            for column in 0..self.width {
                let index = base + column;
                let mut column_flags = self.flags[index];
                if (column_flags | (column_flags >> PI_TO_SIGMA)) & ALL_SIGMA == ALL_SIGMA {
                    self.flags[index] = column_flags & !real_pi;
                    continue;
                }
                let samples = &data[row * self.width + column * rows..][..rows];
                let mut run = None;
                if rows == 4 && column_flags == 0 {
                    let first = samples.iter().position(|&sample| sample & plane.one != 0);
                    coder.symbol(CONTEXT_AGGREGATE, u32::from(first.is_some()));
                    let Some(first) = first else { continue };
                    let bits = u32::try_from(first).expect("run fits in two bits");
                    coder.symbol(CONTEXT_UNIFORM, bits >> 1);
                    coder.symbol(CONTEXT_UNIFORM, bits & 1);
                    run = Some(first);
                }
                for (lane, &sample) in samples.iter().enumerate().skip(run.unwrap_or(0)) {
                    let shift = lane * 3;
                    let flags = column_flags >> shift;
                    let significant = if run == Some(lane) {
                        true
                    } else if flags & (SIGMA_THIS | PI_THIS) == 0 {
                        let bit = plane.bit(sample);
                        coder.symbol(self.significance_context(flags), bit);
                        bit != 0
                    } else {
                        false
                    };
                    if significant {
                        self.sign(coder, index, lane, sample, &mut column_flags);
                        change += plane.decrease(plane.significance, sample);
                    }
                    column_flags &= !(PI_THIS << shift);
                }
                self.flags[index] = column_flags;
            }
        }
        change
    }
}

/// Encode a viewer-profile code-block. The signed fixed-point input is in stripe-column order.
pub(super) fn encode(
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
    workspace.passes.clear();
    workspace
        .passes
        .try_reserve_exact(MAX_PASSES)
        .map_err(|_| memory_error())?;
    let mut coder = Encoder::new(&mut workspace.bytes);
    let mut passes = Passes {
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
    let weight = wavelet::colour_norm(params.component, params.reversible, params.mct)
        * wavelet::norm(params.level, params.orientation, params.reversible)
        * step_size;
    let mut total_distortion = 0.0;
    for plane in (0..numbps).rev() {
        let context = Plane::new(plane);
        let first = if plane == numbps - 1 { 2 } else { 0 };
        for pass_index in first..3 {
            let decrease = match pass_index {
                0 => passes.significance(&mut coder, context),
                1 => passes.refine(&mut coder, context),
                _ => passes.cleanup(&mut coder, context),
            };
            let mut weighted = weight * f64::from(1 << plane);
            weighted *= weighted * f64::from(decrease) / 8192.0;
            total_distortion += weighted;
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
