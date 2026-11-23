// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! What the Tier-1 tests and the fuzz target draw their code-blocks with, once: the generator of the C++ unit tests
//! (`tests/unit/t1_unit.cpp`), the long code-block shapes the header allows, and the split of a block's bytes into the
//! chunks packets deliver them in.

/// SplitMix64, the generator of the C++ unit tests.
#[derive(Clone, Debug)]
pub struct SplitMix64(pub u64);

impl SplitMix64 {
    /// The next 64 bits.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A number below `n` (which must not be 0), as the C++ test's `u(n)` draws it.
    ///
    /// # Panics
    /// When `n` is 0.
    pub fn below(&mut self, n: u32) -> u32 {
        u32::try_from((self.next_u64() >> 11) % u64::from(n)).expect("below a u32")
    }
}

/// The long shapes of a code-block the header allows (at most 4,096 samples, 1,024 a side), `(width, height)`.
pub const LONG_SHAPES: [(u32, u32); 16] = [
    (1024, 4),
    (4, 1024),
    (512, 8),
    (8, 512),
    (256, 16),
    (16, 256),
    (128, 32),
    (32, 128),
    (1024, 3),
    (2, 1024),
    (1024, 1),
    (1, 1024),
    (1024, 2),
    (64, 64),
    (4, 64),
    (64, 4),
];

/// The lengths of `parts` chunks the `n` bytes arrive in, as the C++ unit tests split them (the last takes what is
/// left); none for no bytes.
#[must_use]
pub fn split_chunks(n: usize, parts: u32) -> Vec<u32> {
    if n == 0 {
        return Vec::new();
    }
    let parts = parts.max(1) as usize;
    let mut at = 0;
    (0..parts)
        .map(|i| {
            let to = if i + 1 == parts { n } else { at + (n - at) / (parts - i) };
            let len = u32::try_from(to - at).expect("a chunk fits u32");
            at = to;
            len
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunks_add_up_and_the_generator_is_splitmix() {
        for n in [0usize, 1, 5, 100, 4097] {
            for parts in 1..=5 {
                let chunks = split_chunks(n, parts);
                assert_eq!(chunks.iter().map(|&c| c as usize).sum::<usize>(), n);
                assert_eq!(chunks.len(), if n == 0 { 0 } else { parts as usize });
            }
        }
        // the first outputs of SplitMix64 from seed 0 (the reference implementation's)
        let mut rng = SplitMix64(0);
        assert_eq!(rng.next_u64(), 0xe220_a839_7b1d_cdaf);
        assert_eq!(rng.next_u64(), 0x6e78_9e6a_a1b9_65f4);
        assert!(
            LONG_SHAPES
                .iter()
                .all(|&(w, h)| w * h <= 4096 && w <= 1024 && h <= 1024)
        );
    }
}
