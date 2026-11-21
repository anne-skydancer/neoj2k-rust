// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// Tier-1 decoding, transcribed from neo-j2k's C++ port of OpenJPEG 2.5.4.
// The arithmetic coder, context tables, and coding passes carry OpenJPEG's notice.
/*
 * The copyright in this software is being made available under the 2-clauses
 * BSD License, included below. This software may be subject to other third
 * party and contributor rights, including patent rights, and no such rights
 * are granted under this license.
 *
 * Copyright (c) 2002-2014, Universite catholique de Louvain (UCL), Belgium
 * Copyright (c) 2002-2014, Professor Benoit Macq
 * Copyright (c) 2001-2003, David Janssens
 * Copyright (c) 2002-2003, Yannick Verschueren
 * Copyright (c) 2003-2007, Francois-Olivier Devaux
 * Copyright (c) 2003-2014, Antonin Descampe
 * Copyright (c) 2005, Herve Drolon, FreeImage Team
 * Copyright (c) 2007, Callum Lerwick <seg@haxxed.com>
 * Copyright (c) 2008, Jerome Fimes, Communications & Systemes <jerome.fimes@c-s.fr>
 * Copyright (c) 2012, Carl Hetherington
 * Copyright (c) 2017, IntoPIX SA <support@intopix.com>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS `AS IS'
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

mod mq;
mod passes;
pub(crate) mod tables;

use crate::error::Diagnostics;
use crate::tile::Segment;
use crate::{Error, ErrorKind};

use mq::Decoder;
use passes::Passes;

const LAZY: u32 = 0x01;
const RESET: u32 = 0x02;
const VERTICAL_CAUSAL: u32 = 0x08;
const SEGMENT_SYMBOL: u32 = 0x20;

const MAX_SAMPLES: usize = 4096;
const MAX_FLAGS: usize = 3078;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Block<'a> {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) numbps: u32,
    pub(crate) orient: u32,
    pub(crate) roishift: u32,
    pub(crate) cblksty: u32,
    pub(crate) segments: &'a [Segment],
}

#[derive(Debug)]
pub(crate) struct Destination<'a> {
    pub(crate) samples: &'a mut [i32],
    pub(crate) offset: usize,
    pub(crate) stride: usize,
    pub(crate) reversible: bool,
    pub(crate) stepsize: f32,
}

/// Per-decoder Tier-1 coefficients, stripe flags, and gathered codeword bytes.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    data: Vec<i32>,
    flags: Vec<u32>,
    coded: Vec<u8>,
}

fn no_memory() -> Error {
    Error::new(ErrorKind::NoMemory, "Not enough memory to decode a code-block\n")
}

fn reserve_fixed<T: Copy + Default>(buffer: &mut Vec<T>, count: usize) -> Result<(), Error> {
    if buffer.is_empty() {
        buffer.try_reserve_exact(count).map_err(|_| no_memory())?;
        buffer.resize(count, T::default());
    }
    Ok(())
}

fn invalid_dimensions() -> Error {
    Error::new(ErrorKind::Data, "Invalid Tier-1 code-block dimensions\n")
}

fn invalid_output() -> Error {
    Error::new(ErrorKind::Output, "A code-block does not fit its tile component\n")
}

fn invalid_segment() -> Error {
    Error::new(ErrorKind::Data, "Code-block segments exceed their available bytes\n")
}

fn bits_as_i32(value: u32) -> i32 {
    i32::from_ne_bytes(value.to_ne_bytes())
}

impl Scratch {
    pub(crate) fn retained_bytes(&self) -> u64 {
        (self.data.capacity() * size_of::<i32>() + self.flags.capacity() * size_of::<u32>() + self.coded.capacity())
            as u64
    }

    /// Decode the code-block's complete packet contributions and write its tile coefficients.
    pub(crate) fn decode<'a>(
        &mut self,
        block: Block<'_>,
        chunks: impl IntoIterator<Item = &'a [u8]>,
        destination: Destination<'_>,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Error> {
        let Block {
            width,
            height,
            numbps,
            orient,
            roishift,
            cblksty,
            segments,
        } = block;
        let mut bitplane = bits_as_i32(numbps.wrapping_add(roishift));
        if bitplane >= 31 {
            diagnostics.warn_fmt(format_args!(
                "opj_t1_decode_cblk(): unsupported bpno_plus_one = {bitplane} >= 31\n"
            ));
            return Err(Error::new(ErrorKind::Data, "Tier-1 rejected a code-block\n"));
        }
        if width > 1024 || height > 1024 || orient > 3 {
            return Err(invalid_dimensions());
        }
        let (width, height) = (width as usize, height as usize);
        let samples = width * height;
        let flag_count = (width + 2) * (height.div_ceil(4) + 2);
        if samples > MAX_SAMPLES || flag_count > MAX_FLAGS {
            return Err(invalid_dimensions());
        }
        if width == 0 || height == 0 {
            return Ok(());
        }
        let Destination {
            samples: output,
            offset,
            stride,
            reversible,
            stepsize,
        } = destination;
        let last_end = (height - 1)
            .checked_mul(stride)
            .and_then(|distance| distance.checked_add(offset))
            .and_then(|last_start| last_start.checked_add(width))
            .ok_or_else(invalid_output)?;
        if stride < width || last_end > output.len() {
            return Err(invalid_output());
        }

        let mut chunks = chunks.into_iter();
        let Some(first) = chunks.next() else {
            for row in 0..height {
                let start = offset + row * stride;
                output[start..start + width].fill(0);
            }
            return Ok(());
        };
        let mut bytes = if let Some(second) = chunks.next() {
            self.coded.clear();
            for chunk in [first, second].into_iter().chain(chunks) {
                self.coded.try_reserve(chunk.len()).map_err(|_| no_memory())?;
                self.coded.extend_from_slice(chunk);
            }
            self.coded.as_slice()
        } else {
            first
        };
        reserve_fixed(&mut self.data, MAX_SAMPLES)?;
        reserve_fixed(&mut self.flags, MAX_FLAGS)?;
        let mut passes = Passes {
            data: &mut self.data[..samples],
            flags: &mut self.flags[..flag_count],
            width,
            height,
            orient: orient as usize,
            vertical_causal: cblksty & VERTICAL_CAUSAL != 0,
        };
        passes.initialize();

        let mut decoder = Decoder::new();
        let mut pass = 2;
        let lazy = cblksty & LAZY != 0;
        let reset = cblksty & RESET != 0;
        let segment_symbol = cblksty & SEGMENT_SYMBOL != 0;
        let bypass_bitplane = bits_as_i32(numbps).wrapping_sub(4);
        // Tier 2 supplies segment boundaries for termination modes. OpenJPEG's decode path
        // performs no additional predictable-termination check.
        for segment in segments {
            let (segment_data, remainder) = bytes
                .split_at_checked(segment.len as usize)
                .ok_or_else(invalid_segment)?;
            bytes = remainder;
            let raw = lazy && pass < 2 && bitplane <= bypass_bitplane;
            decoder.start_segment(segment_data, raw);
            for _ in 0..segment.real_num_passes {
                if bitplane < 1 {
                    break;
                }
                let plane = u32::try_from(bitplane).expect("positive bit-plane index");
                match pass {
                    0 if raw => passes.significance::<true>(&mut decoder, plane),
                    0 => passes.significance::<false>(&mut decoder, plane),
                    1 if raw => passes.refinement::<true>(&mut decoder, plane),
                    1 => passes.refinement::<false>(&mut decoder, plane),
                    2 => passes.cleanup(&mut decoder, plane, segment_symbol),
                    _ => unreachable!("a code-block cycles through three coding passes"),
                }
                if reset && !raw {
                    decoder.reset_contexts();
                }
                pass += 1;
                if pass == 3 {
                    pass = 0;
                    bitplane -= 1;
                }
            }
        }
        let half_step = 0.5 * stepsize;
        for (row, coefficients) in self.data[..samples].chunks_exact(width).enumerate() {
            let start = offset + row * stride;
            for (sample, &coefficient) in output[start..start + width].iter_mut().zip(coefficients) {
                let value = undo_roi(coefficient, roishift);
                *sample = if reversible {
                    value / 2
                } else {
                    i32::from_ne_bytes(((value as f32) * half_step).to_ne_bytes())
                };
            }
        }
        Ok(())
    }
}

fn undo_roi(value: i32, shift: u32) -> i32 {
    if shift == 0 {
        return value;
    }
    if shift >= 31 {
        return 0;
    }
    let magnitude = value.wrapping_abs();
    if magnitude < 1 << shift {
        return value;
    }
    let magnitude = magnitude >> shift;
    if value < 0 { -magnitude } else { magnitude }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn random_byte(state: &mut u32) -> u8 {
        *state ^= *state << 13;
        *state ^= *state >> 17;
        *state ^= *state << 5;
        state.to_le_bytes()[0]
    }

    // Hashes recorded from the C++ Tier-1 entry point using the same three segments and output.
    // The selected cases combine MQ and bypass, RESET, VSC, SEGSYM, ROI and both wavelet stores.
    #[test]
    fn fragmented_codewords_match_the_cpp_coefficient_hashes() {
        let mut scratch = Scratch::default();
        let mut diagnostics = Diagnostics::default();
        for (case, width, height, expected) in [
            (27u32, 64, 64, 8_259_553_377_929_840_235u64),
            (59, 64, 64, 9_526_155_953_375_417_358),
            (125, 1_024, 4, 17_778_397_483_968_790_667),
        ] {
            let bytes_count = 44 + case % 37;
            let mut seed = 0x51dc_a971 ^ case;
            let bytes: Vec<u8> = (0..bytes_count).map(|_| random_byte(&mut seed)).collect();
            let (first, second) = bytes.split_at((bytes_count / 4) as usize);
            let roishift = case % 5;
            let segment = |len, real_num_passes| Segment {
                len,
                real_num_passes,
                ..Segment::default()
            };
            let segments = [
                segment(bytes_count / 3, 10 + 3 * roishift),
                segment(bytes_count / 3, 2),
                segment(bytes_count - 2 * (bytes_count / 3), 8),
            ];
            let block = Block {
                width,
                height,
                numbps: 8 + case % 7,
                orient: case % 4,
                roishift,
                cblksty: case % 64,
                segments: &segments,
            };
            let stride = width as usize + 3;
            let mut output = vec![-1234; stride * height as usize + 2];
            let destination = Destination {
                samples: &mut output,
                offset: 2,
                stride,
                reversible: !case.is_multiple_of(3),
                stepsize: 16.3 / ((case + 1) as f32),
            };
            scratch
                .decode(block, [first, second], destination, &mut diagnostics)
                .unwrap();
            let hash = output.into_iter().fold(1_469_598_103_934_665_603u64, |hash, sample| {
                (hash ^ u64::from(u32::from_ne_bytes(sample.to_ne_bytes()))).wrapping_mul(1_099_511_628_211)
            });
            assert_eq!(hash, expected, "code-block case {case}");
        }
    }

    #[test]
    fn missing_blocks_zero_their_rows_and_still_check_bit_planes() {
        fn destination(samples: &mut [i32]) -> Destination<'_> {
            Destination {
                samples,
                offset: 1,
                stride: 3,
                reversible: false,
                stepsize: 2.0,
            }
        }
        let mut scratch = Scratch::default();
        let mut diagnostics = Diagnostics::default();
        let mut samples = [25; 8];
        let block = Block {
            width: 2,
            height: 2,
            numbps: 0,
            orient: 0,
            roishift: 0,
            cblksty: 0,
            segments: &[],
        };
        scratch
            .decode(block, [], destination(&mut samples), &mut diagnostics)
            .unwrap();
        let high_plane = Block { numbps: 31, ..block };
        assert_eq!(
            scratch
                .decode(high_plane, [], destination(&mut samples), &mut diagnostics)
                .unwrap_err()
                .kind(),
            ErrorKind::Data
        );
        assert_eq!(samples, [25, 0, 0, 25, 0, 0, 25, 25]);
        assert_eq!(diagnostics.entries().len(), 1);
    }
}
