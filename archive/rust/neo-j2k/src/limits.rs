// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

/// The decoder's bounds on header and decoded image resources. Zero leaves a field unlimited.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Limits {
    /// Maximum number of tiles declared in the SIZ marker.
    pub max_tiles: u32,
    /// Maximum number of components declared in the SIZ marker.
    pub max_components: u32,
    /// Maximum number of quality layers declared in a COD marker.
    pub max_layers: u32,
    /// Maximum width or height of a component after applying the reduce factor.
    pub max_decoded_side: u32,
    /// Maximum header arena reservation and estimated decoder working memory, in bytes.
    pub max_memory: u64,
}

impl Limits {
    /// The Second Life viewer's texture limits.
    pub const SECOND_LIFE: Self = Self {
        max_tiles: 1024,
        max_components: 8,
        max_layers: 64,
        max_decoded_side: 2048,
        max_memory: 256 << 20,
    };

    /// The OpenSim viewer's texture limits.
    #[cfg(feature = "opensim")]
    pub const OPENSIM: Self = Self {
        max_decoded_side: 16_384,
        ..Self::SECOND_LIFE
    };
}
