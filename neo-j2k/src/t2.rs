// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The packet bit reader and tag trees derive from OpenJPEG's bio.c, tgt.c and t2.c
// through neo-j2k's src/t2.cpp and src/tile.cpp.
//
// The copyright in this software is being made available under the 2-clauses
// BSD License, included below. This software may be subject to other third
// party and contributor rights, including patent rights, and no such rights
// are granted under this license.
//
// Copyright (c) 2002-2014, Universite catholique de Louvain (UCL), Belgium
// Copyright (c) 2002-2014, Professor Benoit Macq
// Copyright (c) 2001-2003, David Janssens
// Copyright (c) 2002-2003, Yannick Verschueren
// Copyright (c) 2003-2007, Francois-Olivier Devaux
// Copyright (c) 2003-2014, Antonin Descampe
// Copyright (c) 2005, Herve Drolon, FreeImage Team
// Copyright (c) 2006-2007, Parvatha Elangovan
// Copyright (c) 2008, 2011-2012, Centre National d'Etudes Spatiales (CNES), FR
// Copyright (c) 2012, CS Systemes d'Information, France
// Copyright (c) 2017, IntoPIX SA <support@intopix.com>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS `AS IS'
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

//! Packet header bits and precinct tag trees for Tier 2.

use crate::{Error, ErrorKind};

/// Packet header bit reader, with one stuffed high bit after each `0xff` byte.
#[derive(Debug)]
pub(crate) struct Bio<'a> {
    remaining: &'a [u8],
    bytes_read: usize,
    buffer: u16,
    bits_left: u8,
    overrun: bool,
}

impl<'a> Bio<'a> {
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self {
            remaining: bytes,
            bytes_read: 0,
            buffer: 0,
            bits_left: 0,
            overrun: false,
        }
    }

    fn refill(&mut self) -> bool {
        self.buffer <<= 8;
        self.bits_left = if self.buffer == 0xff00 { 7 } else { 8 };
        let Some((&byte, rest)) = self.remaining.split_first() else {
            self.overrun = true;
            return false;
        };
        self.buffer |= u16::from(byte);
        self.remaining = rest;
        self.bytes_read += 1;
        true
    }

    fn bit(&mut self) -> u32 {
        if self.bits_left == 0 {
            self.refill();
        }
        self.bits_left -= 1;
        u32::from((self.buffer >> self.bits_left) & 1)
    }

    /// Reads up to 32 bits, most significant first. Missing bits are zero and set `overrun`.
    pub(crate) fn read(&mut self, count: u32) -> u32 {
        (0..count).fold(0, |value, _| (value << 1) | self.bit())
    }

    /// Reads the run of one bits before the zero terminating the packet's comma code.
    pub(crate) fn comma_code(&mut self) -> u32 {
        let mut count = 0u32;
        while self.bit() != 0 {
            count = count.wrapping_add(1);
        }
        count
    }

    /// Reads the variable-length code for the number of new coding passes.
    pub(crate) fn num_passes(&mut self) -> u32 {
        if self.bit() == 0 {
            return 1;
        }
        if self.bit() == 0 {
            return 2;
        }
        let short = self.read(2);
        if short < 3 {
            return 3 + short;
        }
        let extended = self.read(5);
        if extended < 31 {
            return 6 + extended;
        }
        37 + self.read(7)
    }

    /// Discards padding and consumes a stuffed byte after a final `0xff`.
    /// Returns false when that extra byte is missing. `overrun` remains sticky.
    pub(crate) fn align(&mut self) -> bool {
        if self.buffer & 0xff == 0xff && !self.refill() {
            return false;
        }
        self.bits_left = 0;
        true
    }

    /// Bytes actually consumed, including padding and stuffed bytes.
    pub(crate) const fn num_bytes(&self) -> usize {
        self.bytes_read
    }

    /// Whether the reader needed another byte after the end of the input.
    pub(crate) const fn overrun(&self) -> bool {
        self.overrun
    }
}

#[derive(Debug, Clone, Copy)]
struct TagNode {
    value: i32,
    low: i32,
}

impl TagNode {
    // OpenJPEG's sentinel also bounds how far an unterminated tag code is read.
    const UNSET: Self = Self { value: 999, low: 0 };
}

/// Flat tag tree, with leaf rows first and successive parent levels ending at the root.
#[derive(Debug)]
pub(crate) struct TagTree {
    width: usize,
    height: usize,
    nodes: Vec<TagNode>,
}

fn oversized_tree() -> Error {
    Error::new(ErrorKind::Limit, "Packet tag tree dimensions are too large\n")
}

impl TagTree {
    pub(crate) fn new(width: u32, height: u32) -> Result<Self, Error> {
        let width = usize::try_from(width).map_err(|_| oversized_tree())?;
        let height = usize::try_from(height).map_err(|_| oversized_tree())?;
        let (mut columns, mut rows) = (width, height);
        let mut count = columns.checked_mul(rows).ok_or_else(oversized_tree)?;
        let mut total = count;
        while count > 1 {
            columns = columns.div_ceil(2);
            rows = rows.div_ceil(2);
            // A parent level never has more nodes than the already checked leaf level.
            count = columns * rows;
            total = total.checked_add(count).ok_or_else(oversized_tree)?;
        }
        let mut nodes = Vec::new();
        nodes
            .try_reserve_exact(total)
            .map_err(|_| Error::new(ErrorKind::NoMemory, "Not enough memory for a packet tag tree\n"))?;
        nodes.resize(total, TagNode::UNSET);
        Ok(Self { width, height, nodes })
    }

    pub(crate) fn reset(&mut self) {
        self.nodes.fill(TagNode::UNSET);
    }

    /// Decodes the path from the root to `leaf` and reports whether its tag is below `threshold`.
    /// Each call retains the bits already decoded for later calls in the same precinct.
    pub(crate) fn decode(&mut self, bio: &mut Bio<'_>, leaf: u32, threshold: i32) -> Result<bool, Error> {
        let leaf = usize::try_from(leaf).map_err(|_| invalid_leaf())?;
        if leaf >= self.width * self.height {
            return Err(invalid_leaf());
        }

        // At most 32 ceil-halvings bring a u32 width and height to the root.
        let mut path = [0; 33];
        let mut depth = 0;
        let mut offset = 0;
        let (mut width, mut height) = (self.width, self.height);
        let (mut x, mut y) = (leaf % width, leaf / width);
        loop {
            path[depth] = offset + y * width + x;
            depth += 1;
            let count = width * height;
            if count == 1 {
                break;
            }
            offset += count;
            width = width.div_ceil(2);
            height = height.div_ceil(2);
            x /= 2;
            y /= 2;
        }

        let mut low = 0;
        for &index in path[..depth].iter().rev() {
            let node = &mut self.nodes[index];
            low = low.max(node.low);
            while low < threshold && low < node.value {
                if bio.bit() != 0 {
                    node.value = low;
                    break;
                }
                low += 1;
            }
            node.low = low;
        }
        Ok(self.nodes[leaf].value < threshold)
    }
}

fn invalid_leaf() -> Error {
    Error::new(ErrorKind::Data, "Packet tag tree leaf is outside the precinct\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_header_bits_are_stuffed_and_missing_bits_read_as_zero() {
        let mut bio = Bio::new(&[0xff, 0xdb, 0xa5]);
        assert_eq!(bio.read(0), 0);
        assert_eq!(bio.num_bytes(), 0);
        assert_eq!(bio.read(8), 0xff);
        assert_eq!(bio.read(7), 0x5b);
        assert_eq!(bio.num_bytes(), 2);
        assert_eq!(bio.read(8), 0xa5);
        assert!(!bio.overrun());
        assert_eq!(bio.read(3), 0);
        assert!(bio.overrun());
        assert_eq!(bio.num_bytes(), 3);

        let mut bio = Bio::new(&[0x12, 0x34, 0x56, 0x78]);
        assert_eq!(bio.read(32), 0x1234_5678);
        assert!(!bio.overrun());
    }

    #[test]
    fn aligning_consumes_the_byte_after_ff_or_marks_an_overrun() {
        let mut bio = Bio::new(&[0xff, 0x00, 0x12]);
        assert_eq!(bio.read(1), 1);
        assert!(bio.align());
        assert_eq!(bio.num_bytes(), 2);
        assert_eq!(bio.read(8), 0x12);

        let mut cut = Bio::new(&[0xff]);
        assert_eq!(cut.read(8), 0xff);
        assert!(!cut.align());
        assert_eq!(cut.num_bytes(), 1);
        assert!(cut.overrun());
        assert!(cut.align());
        assert!(cut.overrun());
    }

    #[test]
    fn packet_header_codes_cover_each_pass_count_range() {
        for (input, passes) in [
            (&[0x00][..], 1),
            (&[0x80][..], 2),
            (&[0xc0][..], 3),
            (&[0xd0][..], 4),
            (&[0xe0][..], 5),
            (&[0xf0, 0x00][..], 6),
            (&[0xff, 0x00][..], 36),
            (&[0xff, 0x40, 0x00][..], 37),
            (&[0xff, 0x7f, 0x80][..], 164),
        ] {
            let mut bio = Bio::new(input);
            assert_eq!(bio.num_passes(), passes);
            assert!(!bio.overrun());
        }
        let mut bio = Bio::new(&[0xff, 0x60]);
        assert_eq!(bio.comma_code(), 10);
        assert!(!bio.overrun());
        assert_eq!(bio.num_bytes(), 2);

        let mut cut = Bio::new(&[0xff]);
        assert_eq!(cut.comma_code(), 8);
        assert!(cut.overrun());
        assert_eq!(cut.num_bytes(), 1);
    }

    #[test]
    fn repeated_thresholds_share_the_decoded_root_and_leaf_values() {
        let mut tree = TagTree::new(2, 1).unwrap();
        let mut bio = Bio::new(&[0b1100_1000]);
        assert!(tree.decode(&mut bio, 0, 1).unwrap());
        assert!(!tree.decode(&mut bio, 1, 1).unwrap());
        assert!(!tree.decode(&mut bio, 1, 2).unwrap());
        assert!(tree.decode(&mut bio, 1, 3).unwrap());
        assert!(!bio.overrun());

        let mut next = Bio::new(&[]);
        assert!(tree.decode(&mut next, 0, 1).unwrap());
        assert!(tree.decode(&mut next, 1, 3).unwrap());
        assert!(!next.overrun());

        tree.reset();
        assert!(!tree.decode(&mut next, 0, 1).unwrap());
        assert!(next.overrun());
    }

    #[test]
    fn odd_grids_share_intermediate_nodes_and_reject_invalid_leaves() {
        let mut tree = TagTree::new(3, 2).unwrap();
        assert_eq!(tree.nodes.len(), 9);
        let mut bio = Bio::new(&[0b1110_0000]);
        assert!(tree.decode(&mut bio, 5, 1).unwrap());
        assert!(!tree.decode(&mut bio, 2, 1).unwrap());
        assert_eq!(bio.bits_left, 4);
        assert_eq!(tree.decode(&mut bio, 6, 1).unwrap_err().kind(), ErrorKind::Data);

        let mut empty = TagTree::new(0, 7).unwrap();
        assert!(empty.nodes.is_empty());
        assert_eq!(empty.decode(&mut bio, 0, 1).unwrap_err().kind(), ErrorKind::Data);
        assert_eq!(TagTree::new(u32::MAX, u32::MAX).unwrap_err().kind(), ErrorKind::Limit);
    }
}
