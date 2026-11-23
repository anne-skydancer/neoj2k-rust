// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The encoder's half of `crate::bench`: one code-block through the encoder's Tier-1.

use super::tier1::{Workspace, encode};
use super::{Block, T1Params};

/// What a code-block's coding depends on beside its samples.
#[derive(Clone, Copy, Debug)]
pub struct EncodeParams {
    /// The band's orientation, 0 LL, 1 HL, 2 LH, 3 HH.
    pub orientation: u8,
    /// The resolution level (the distortion weight).
    pub level: usize,
    /// The colour component (with `mct`, the colour transform's norm).
    pub component: usize,
    /// The 5/3 wavelet.
    pub reversible: bool,
    /// The band's quantiser step.
    pub step_size: f32,
    /// The component is one of three under the colour transform.
    pub mct: bool,
}

/// One pass of an encoded block.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EncodedPass {
    pub rate: u32,
    pub len: u32,
    pub terminated: bool,
    /// The bits of the cumulative weighted distortion decrease.
    pub distortion_bits: u64,
}

/// The last block a workspace encoded.
#[derive(Clone, Copy, Debug)]
pub struct EncodedBlock<'a> {
    /// The bit-plane count, wrapped as OpenJPEG's for a block under one step.
    pub numbps: u32,
    /// The codeword bytes.
    pub data: &'a [u8],
    passes: &'a [super::Pass],
}

impl EncodedBlock<'_> {
    /// The passes, in coding order.
    pub fn passes(&self) -> impl Iterator<Item = EncodedPass> + '_ {
        self.passes.iter().map(|p| EncodedPass {
            rate: p.rate,
            len: p.len,
            terminated: p.terminated,
            distortion_bits: p.distortion_dec.to_bits(),
        })
    }
}

/// The encoder's Tier-1 scratch and the block it last produced, kept by the caller.
#[derive(Debug, Default)]
pub struct EncodeWorkspace {
    workspace: Workspace,
    block: Block,
}

impl EncodeWorkspace {
    /// An empty scratch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Encodes the block (at most 64 x 64 samples, signed fixed point with six fractional bits, in the order the
    /// encoder gathers them). False when Tier-1 refuses it.
    pub fn encode(&mut self, samples: &[i32], width: usize, height: usize, params: &EncodeParams) -> bool {
        let params = T1Params {
            orientation: params.orientation,
            level: params.level,
            component: params.component,
            reversible: params.reversible,
            step_size: params.step_size,
            mct: params.mct,
        };
        encode(&mut self.workspace, samples, width, height, params, &mut self.block).is_ok()
    }

    /// The block the last successful [`encode`](Self::encode) made.
    #[must_use]
    pub fn block(&self) -> EncodedBlock<'_> {
        EncodedBlock {
            numbps: self.block.numbps,
            data: &self.block.data,
            passes: &self.block.passes,
        }
    }
}
