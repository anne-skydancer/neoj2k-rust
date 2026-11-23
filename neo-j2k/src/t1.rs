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
#[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
use crate::simd::t1 as vector;
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
    Error::static_message(ErrorKind::NoMemory, "Not enough memory to decode a code-block\n")
}

fn reserve_fixed<T: Copy + Default>(buffer: &mut Vec<T>, count: usize) -> Result<(), Error> {
    if buffer.is_empty() {
        buffer.try_reserve_exact(count).map_err(|_| no_memory())?;
        buffer.resize(count, T::default());
    }
    Ok(())
}

fn invalid_dimensions() -> Error {
    Error::static_message(ErrorKind::Data, "Invalid Tier-1 code-block dimensions\n")
}

fn invalid_output() -> Error {
    Error::static_message(ErrorKind::Output, "A code-block does not fit its tile component\n")
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
            ..
        } = block;
        let bitplane = bits_as_i32(numbps.wrapping_add(roishift));
        if bitplane >= 31 {
            diagnostics.warn_fmt(format_args!(
                "opj_t1_decode_cblk(): unsupported bpno_plus_one = {bitplane} >= 31\n"
            ));
            return Err(Error::static_message(ErrorKind::Data, "Tier-1 rejected a code-block\n"));
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
        let bytes = if let Some(second) = chunks.next() {
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
        let single_stripe = height <= 4;
        let used_flags = if single_stripe { width + 2 } else { flag_count };
        let mut passes = Passes {
            data: &mut self.data[..samples],
            flags: &mut self.flags[..used_flags],
            width,
            height,
            orient: orient as usize,
            vertical_causal: cblksty & VERTICAL_CAUSAL != 0,
        };
        if single_stripe {
            decode_single_stripe(&mut passes, block, bitplane, bytes);
        } else {
            decode_passes::<false>(&mut passes, block, bitplane, bytes);
        }
        store_coefficients::<true>(
            &mut self.data[..samples],
            &mut output[offset..last_end],
            width,
            stride,
            roishift,
            reversible,
            stepsize,
        );
        Ok(())
    }
}

#[inline(never)]
fn decode_single_stripe(passes: &mut Passes<'_>, block: Block<'_>, bitplane: i32, bytes: &[u8]) {
    decode_passes::<true>(passes, block, bitplane, bytes)
}

#[inline(always)]
fn decode_passes<const SINGLE_STRIPE: bool>(
    passes: &mut Passes<'_>,
    block: Block<'_>,
    mut bitplane: i32,
    mut bytes: &[u8],
) {
    let Block {
        numbps,
        cblksty,
        segments,
        ..
    } = block;
    passes.initialize::<SINGLE_STRIPE>();

    let mut decoder = Decoder::new();
    let mut pass = 2;
    let lazy = cblksty & LAZY != 0;
    let reset = cblksty & RESET != 0;
    let segment_symbol = cblksty & SEGMENT_SYMBOL != 0;
    let bypass_bitplane = bits_as_i32(numbps).wrapping_sub(4);
    // Tier 2 supplies segment boundaries for termination modes. OpenJPEG's decode path
    // performs no additional predictable-termination check.
    for segment in segments {
        // The one length check of a block, as in the C++: a segment longer than the bytes that are
        // left is cut to them (Tier 2 gives segments that fit).
        let (segment_data, remainder) = bytes.split_at(bytes.len().min(segment.len as usize));
        bytes = remainder;
        let raw = lazy && pass < 2 && bitplane <= bypass_bitplane;
        decoder.start_segment(segment_data, raw);
        for _ in 0..segment.real_num_passes {
            if bitplane < 1 {
                break;
            }
            // A raw segment holds significance and refinement passes only (Tier 2 gives it at most
            // two): a cleanup pass would run the arithmetic decoder on registers nobody set up. The
            // passes of such a segment end here, as in the C++ (v0.1.3).
            if raw && pass == 2 {
                break;
            }
            let plane = u32::try_from(bitplane).expect("positive bit-plane index");
            match pass {
                0 if raw => passes.significance::<true, SINGLE_STRIPE>(&mut decoder, plane),
                0 => passes.significance::<false, SINGLE_STRIPE>(&mut decoder, plane),
                1 if raw => passes.refinement::<true, SINGLE_STRIPE>(&mut decoder, plane),
                1 => passes.refinement::<false, SINGLE_STRIPE>(&mut decoder, plane),
                2 => passes.cleanup::<SINGLE_STRIPE>(&mut decoder, plane, segment_symbol),
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
}

/// Store a nonempty block in its validated destination, preserving row padding.
fn store_coefficients<const VECTOR: bool>(
    coefficients: &mut [i32],
    output: &mut [i32],
    width: usize,
    stride: usize,
    roishift: u32,
    reversible: bool,
    stepsize: f32,
) {
    // Undo ROI on the decoder's scratch as in C++. INT_MIN wraps on abs and is below the threshold.
    match roishift {
        0 => {}
        31.. => coefficients.fill(0),
        shift => {
            let threshold = 1 << shift;
            for value in &mut *coefficients {
                let magnitude = value.wrapping_abs();
                if magnitude >= threshold {
                    let magnitude = magnitude >> shift;
                    *value = if *value < 0 { -magnitude } else { magnitude };
                }
            }
        }
    }

    let half_step = 0.5 * stepsize;
    #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
    if VECTOR && vector::store(coefficients, output, width, stride, reversible, half_step) {
        return;
    }

    if reversible {
        store_scalar(coefficients, output, width, stride, |value| value / 2);
    } else {
        store_scalar(coefficients, output, width, stride, |value| {
            ((value as f32) * half_step).to_bits().cast_signed()
        });
    }
}

fn store_scalar(input: &[i32], output: &mut [i32], width: usize, stride: usize, convert: impl Fn(i32) -> i32) {
    for (row, input) in input.chunks_exact(width).enumerate() {
        let start = row * stride;
        for (target, &value) in output[start..start + width].iter_mut().zip(input) {
            *target = convert(value);
        }
    }
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

    #[test]
    fn roi_store_matches_threshold_and_wrapping_magnitude_rules() {
        let mut input = [i32::MIN, -17, -16, -15, 0, 15, 16, 17];
        let mut output = [0; 8];
        store_coefficients::<true>(&mut input, &mut output, 8, 8, 4, true, 0.0);
        assert_eq!(output, [i32::MIN / 2, 0, 0, -7, 0, 7, 0, 0]);
        store_coefficients::<true>(&mut input, &mut output, 8, 8, 31, true, 0.0);
        assert_eq!(output, [0; 8]);
    }

    #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
    #[test]
    fn avx2_stores_match_scalar_with_tails_and_padding() {
        const PADDING: i32 = 0x1234_5678;
        let extremes = [
            i32::MIN,
            i32::MIN + 1,
            -(1 << 24) - 1,
            -257,
            -7,
            -1,
            0,
            1,
            7,
            257,
            (1 << 24) + 1,
            i32::MAX,
        ];
        for width in [1, 7, 8, 9, 15, 16, 17, 32, 64, 1_024] {
            let stride = width + 5;
            let input: Vec<i32> = extremes.into_iter().cycle().take(3 * width).collect();
            for reversible in [true, false] {
                for roishift in [0, 1, 8, 30, 31, u32::MAX] {
                    for stepsize in [-0.0, f32::MIN_POSITIVE, 1.3, f32::MAX] {
                        let end = 1 + 2 * stride + width;
                        let mut scalar = vec![PADDING; end + 1];
                        let mut vector = scalar.clone();
                        store_coefficients::<false>(
                            &mut input.clone(),
                            &mut scalar[1..end],
                            width,
                            stride,
                            roishift,
                            reversible,
                            stepsize,
                        );
                        store_coefficients::<true>(
                            &mut input.clone(),
                            &mut vector[1..end],
                            width,
                            stride,
                            roishift,
                            reversible,
                            stepsize,
                        );
                        assert_eq!(
                            vector, scalar,
                            "width {width}, reversible {reversible}, ROI {roishift}, step {stepsize}"
                        );
                        assert_eq!(vector[0], PADDING);
                        assert_eq!(vector[end], PADDING);
                        for row in 0..2 {
                            let padding = 1 + row * stride + width;
                            assert_eq!(vector[padding..padding + 5], [PADDING; 5]);
                        }
                    }
                }
            }
        }
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

    /// Decodes a lazy (BYPASS) block of `numbps` 12 whose segments are the given byte lengths and
    /// pass counts over `bytes`.
    fn decode_lazy(segments: &[(u32, u32)], bytes: &[u8]) -> Result<Vec<i32>, Error> {
        let segments: Vec<Segment> = segments
            .iter()
            .map(|&(len, real_num_passes)| Segment {
                len,
                real_num_passes,
                ..Segment::default()
            })
            .collect();
        let block = Block {
            width: 16,
            height: 16,
            numbps: 12,
            orient: 0,
            roishift: 0,
            cblksty: LAZY,
            segments: &segments,
        };
        let mut output = vec![0; 16 * 16];
        let destination = Destination {
            samples: &mut output,
            offset: 0,
            stride: 16,
            reversible: true,
            stepsize: 1.0,
        };
        Scratch::default().decode(block, [bytes], destination, &mut Diagnostics::default())?;
        Ok(output)
    }

    fn lazy_bytes() -> Vec<u8> {
        let mut seed = 0x2545_f491;
        (0..90).map(|_| random_byte(&mut seed)).collect()
    }

    // C++ v0.1.3: the passes of a raw segment end at a cleanup pass, so a third pass asked of one
    // changes nothing (a stream Tier 2 cannot produce).
    #[test]
    fn a_raw_segments_passes_end_at_a_cleanup_pass() {
        let bytes = lazy_bytes();
        // ten MQ passes (the first cleanup and three bit-planes), then a raw segment
        let two = decode_lazy(&[(40, 10), (50, 2)], &bytes).unwrap();
        let three = decode_lazy(&[(40, 10), (50, 3)], &bytes).unwrap();
        assert_eq!(two, three);
        assert!(two.iter().any(|&v| v != 0));
    }

    // C++ v0.1.3: a segment longer than the bytes that are left is cut to them, not refused.
    #[test]
    fn an_over_long_segment_is_cut_to_the_bytes_there() {
        let bytes = lazy_bytes();
        let exact = decode_lazy(&[(90, 13)], &bytes).unwrap();
        assert_eq!(decode_lazy(&[(1_000, 13)], &bytes).unwrap(), exact);
        // a later segment finds no bytes left
        assert!(decode_lazy(&[(1_000, 10), (5, 2)], &bytes).is_ok());
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
