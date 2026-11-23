// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The two Tier-1 entry points the unpublished benchmark crate `neo-j2k-bench` times: decode a code-block and
//! encode a code-block. Behind the off-by-default feature `bench-internals`, hidden from the documentation and
//! not part of the API; nothing here is a stable interface. Each entry is a thin call of the code the decoder and the
//! encoder themselves run, with the buffers kept by the caller so that a timed call allocates nothing.

use crate::error::Diagnostics;
use crate::t1::{Block, Destination, Scratch};
use crate::tile::Segment;

pub use crate::encoder::bench::{EncodeParams, EncodeWorkspace, EncodedBlock, EncodedPass};
pub use crate::stage::Stage;

/// Clears the calling thread's stage totals.
pub fn stage_reset() {
    crate::stage::reset();
}

/// The calling thread's stage totals since the last reset, one entry per [`Stage::ALL`], in that order.
#[must_use]
pub fn stage_totals() -> [core::time::Duration; Stage::ALL.len()] {
    crate::stage::totals()
}

/// One code-block as the decoder's Tier-1 receives it, prepared once (its segments are built here, not in the
/// timed call).
#[derive(Clone, Debug)]
pub struct DecodeBlock {
    width: u32,
    height: u32,
    numbps: u32,
    orient: u32,
    roishift: u32,
    cblksty: u32,
    segments: Vec<Segment>,
    bytes: Vec<u8>,
    parts: u32,
}

impl DecodeBlock {
    /// A block from its header fields, the `(len, passes)` of each codeword segment and the gathered bytes.
    #[must_use]
    pub fn new(
        (width, height): (u32, u32),
        (numbps, orient, roishift, cblksty): (u32, u32, u32, u32),
        segments: &[(u32, u32)],
        bytes: Vec<u8>,
    ) -> Self {
        Self {
            width,
            height,
            numbps,
            orient,
            roishift,
            cblksty,
            segments: segments
                .iter()
                .map(|&(len, passes)| Segment {
                    len,
                    numpasses: passes,
                    real_num_passes: passes,
                    ..Segment::default()
                })
                .collect(),
            bytes,
            parts: 1,
        }
    }

    /// The bytes arrive in this many chunks (as packets deliver them), split as the C++ unit tests split them.
    #[must_use]
    pub fn split(mut self, parts: u32) -> Self {
        self.parts = parts.max(1);
        self
    }

    /// The samples the block decodes to.
    #[must_use]
    pub fn samples(&self) -> usize {
        self.width as usize * self.height as usize
    }
}

/// The decoder's Tier-1 scratch, kept from block to block as a worker keeps it.
#[derive(Debug)]
pub struct DecodeWorkspace {
    scratch: Scratch,
    diagnostics: Diagnostics,
}

impl Default for DecodeWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

impl DecodeWorkspace {
    /// An empty scratch (its buffers are made by the first block).
    #[must_use]
    pub fn new() -> Self {
        Self {
            scratch: Scratch::default(),
            diagnostics: Diagnostics::new(false),
        }
    }

    /// Decodes the block into `out` (the block's samples, stride its width) as the decoder writes it into a tile
    /// buffer: a 5/3 block's coefficients halved, a 9/7 block's multiplied by `0.5 * step_size`. False when Tier-1
    /// refuses the block.
    ///
    /// # Panics
    /// When `out` is shorter than the block's samples.
    pub fn decode(&mut self, block: &DecodeBlock, reversible: bool, step_size: f32, out: &mut [i32]) -> bool {
        let tier1_block = Block {
            width: block.width,
            height: block.height,
            numbps: block.numbps,
            orient: block.orient,
            roishift: block.roishift,
            cblksty: block.cblksty,
            segments: &block.segments,
        };
        let destination = Destination {
            samples: &mut out[..block.samples()],
            offset: 0,
            stride: block.width as usize,
            reversible,
            stepsize: step_size,
        };
        let bytes = block.bytes.as_slice();
        let parts = if bytes.is_empty() { 0 } else { block.parts as usize };
        let chunks = (0..parts).scan(0usize, |at, i| {
            let to = if i + 1 == parts {
                bytes.len()
            } else {
                *at + (bytes.len() - *at) / (parts - i)
            };
            let chunk = &bytes[*at..to];
            *at = to;
            Some(chunk)
        });
        self.scratch
            .decode(tier1_block, chunks, destination, &mut self.diagnostics)
            .is_ok()
    }
}
