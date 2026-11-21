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

//! Packet progression, headers and codeword contributions for Tier 2.

use crate::error::Diagnostics;
use crate::header::{PocEntry, TileCoding};
use crate::mem::{CType, WorkBudget, capacity_bytes};
use crate::tile::Tile;
use crate::{Error, ErrorKind};

mod packet;
mod progression;

use packet::{MISSING_EPH, Reader};
use progression::{Grid, Packet, Seen};

/// Tile packet data and the effective coding parameters selected by the tile-part reader.
#[derive(Debug)]
pub(crate) struct PacketInput<'data, 'cursor, 'ppm> {
    pub(crate) coding: TileCoding,
    /// Tile POC entries when present, else the main header's.
    pub(crate) pocs: &'data [PocEntry],
    /// A maximum layer count, or zero for all layers.
    pub(crate) layers: u32,
    pub(crate) strict: bool,
    /// The bytes ended before the tile-part's declared length.
    pub(crate) cut: bool,
    /// The combined SOD bodies, in tile-part order. Chunk ranges refer to this slice.
    pub(crate) data: &'data [u8],
    /// Main-header packed headers, consumed across tiles.
    pub(crate) ppm: Option<&'cursor mut &'ppm [u8]>,
    /// Packed headers from the tile's PPT markers, in marker-index order.
    pub(crate) ppt: Option<&'data [u8]>,
}

/// Packet traversal storage reused between tiles and decoder requests.
#[derive(Debug, Default)]
pub(crate) struct Workspace {
    grid: Grid,
    seen: Seen,
    decoded: Vec<u32>,
    first_pass_failed: Vec<bool>,
}

impl Workspace {
    pub(crate) fn retained_bytes(&self) -> u64 {
        self.grid.retained_bytes()
            + self.seen.retained_bytes()
            + capacity_bytes(&self.decoded)
            + capacity_bytes(&self.first_pass_failed)
    }
}

/// Reads one tile's packets, preserving every complete codeword contribution.
/// The result is each component's highest decoded resolution index.
pub(crate) fn read_packets<'work>(
    workspace: &'work mut Workspace,
    tile: &mut Tile,
    input: PacketInput<'_, '_, '_>,
    budget: &mut WorkBudget,
    diagnostics: &mut Diagnostics,
) -> Result<&'work [u32], Error> {
    let coding = input.coding;
    let pocs = input.pocs;
    let deduplicate = !pocs.is_empty() || coding.prg > 1;
    let layer_limit = if input.layers == 0 {
        u32::from(coding.numlayers)
    } else {
        input.layers
    };
    let components = tile.comps.len();
    let Workspace {
        grid,
        seen,
        decoded,
        first_pass_failed,
    } = workspace;
    packet_array(decoded, CType::U32, components, 0u32, budget)?;
    grid.configure(tile, deduplicate, budget)?;
    grid.prepare_seen(seen, coding, deduplicate, budget)?;
    packet_array(first_pass_failed, CType::Bool, components, true, budget)?;
    let mut reader = Reader::new(input);
    let mut exhausted = false;
    let mut final_components = 0;

    for index in 0..pocs.len().max(1) {
        let poc = pocs.get(index);
        if let Some(poc) = poc
            && usize::from(poc.compno0) >= components
            && let Some(name) = ["lrcp", "rlcp", "rpcl", "pcrl", "cprl"].get(usize::from(poc.prg))
        {
            diagnostics.error_fmt(format_args!("opj_pi_next_{name}(): invalid compno0/compno1\n"));
            continue;
        }
        first_pass_failed.fill(true);
        let done = grid.visit(coding, poc, |packet| {
            if deduplicate && !seen.insert(packet) {
                return Ok(false);
            }
            let comp = &tile.comps[packet.comp];
            let last = comp.minimum_num_resolutions - 1;
            let resolution = u32::try_from(packet.res).expect("resolution index fits u32");
            let skip = skip_packet(tile, layer_limit, packet, resolution);
            let decoded_res = &mut decoded[packet.comp];
            let first_failed = &mut first_pass_failed[packet.comp];

            if exhausted {
                if reader.strict_eph() {
                    return Err(Error::new(ErrorKind::Data, MISSING_EPH));
                }
                // Visiting an absent packet can only update the reported resolution.
                if *decoded_res < last {
                    record_resolution(decoded_res, first_failed, resolution, last, skip);
                    if *decoded_res >= last {
                        final_components += 1;
                        return Ok(final_components == components);
                    }
                }
                return Ok(false);
            }

            reader.visit(tile, packet, skip, budget, diagnostics)?;
            record_resolution(decoded_res, first_failed, resolution, last, skip);
            if reader.exhausted() {
                exhausted = true;
                final_components = decoded
                    .iter()
                    .zip(&tile.comps)
                    .filter(|(resolution, comp)| **resolution + 1 >= comp.minimum_num_resolutions)
                    .count();
                return Ok(final_components == components);
            }
            Ok(false)
        })?;
        if done {
            break;
        }
    }
    Ok(decoded)
}

fn packet_array<T: Copy>(
    array: &mut Vec<T>,
    ty: CType,
    count: usize,
    value: T,
    budget: &mut WorkBudget,
) -> Result<(), Error> {
    budget.array(ty, count)?;
    array
        .try_reserve_exact(count.saturating_sub(array.len()))
        .map_err(|_| Error::static_message(ErrorKind::NoMemory, "Not enough memory for a packet iterator array\n"))?;
    array.resize(count, value);
    array.fill(value);
    Ok(())
}

fn skip_packet(tile: &Tile, layer_limit: u32, packet: Packet, resolution: u32) -> bool {
    let comp = &tile.comps[packet.comp];
    if packet.layer >= layer_limit || resolution >= comp.minimum_num_resolutions {
        return true;
    }
    // The packet reader traverses unselected components, too. Selection happens only in Tier 1.
    !comp.res[packet.res].bands.iter().any(|band| {
        let precinct = &band.precincts[packet.precinct];
        band.area_of_interest(precinct.rect, comp.reversible)
    })
}

fn record_resolution(decoded: &mut u32, first_failed: &mut bool, res: u32, last: u32, skip: bool) {
    if !skip {
        *first_failed = false;
        *decoded = (*decoded).max(res);
    } else if *first_failed && *decoded == 0 {
        *decoded = last;
    }
}

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
        let mut remaining = u8::try_from(count).expect("packet fields occupy at most 32 bits");
        let mut value = 0;
        while remaining != 0 {
            if self.bits_left == 0 {
                self.refill();
            }
            let take = remaining.min(self.bits_left);
            self.bits_left -= take;
            remaining -= take;
            let bits = u32::from(self.buffer) >> self.bits_left;
            value = (value << take) | (bits & ((1 << take) - 1));
        }
        value
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
    const MAX_VALUE: i32 = 999;
    const UNSET: Self = Self {
        value: Self::MAX_VALUE,
        low: 0,
    };

    fn decode(&mut self, bio: &mut Bio<'_>, inherited_low: i32, threshold: i32) -> i32 {
        let mut low = inherited_low.max(self.low);
        while low < threshold && low < self.value {
            if bio.bit() != 0 {
                self.value = low;
                break;
            }
            low += 1;
        }
        self.low = low;
        low
    }
}

/// Flat tag tree, with leaf rows first and successive parent levels ending at the root.
#[derive(Debug, Default)]
pub(crate) struct TagTree {
    width: usize,
    height: usize,
    nodes: Vec<TagNode>,
}

fn oversized_tree() -> Error {
    Error::new(ErrorKind::Limit, "Packet tag tree dimensions are too large\n")
}

impl TagTree {
    pub(crate) fn retained_bytes(&self) -> u64 {
        capacity_bytes(&self.nodes)
    }

    #[cfg(test)]
    pub(crate) fn new(width: u32, height: u32) -> Result<Self, Error> {
        let mut tree = Self::default();
        tree.configure(width, height, Self::node_count(width, height)?)?;
        Ok(tree)
    }

    pub(crate) fn node_count(width: u32, height: u32) -> Result<usize, Error> {
        let mut columns = usize::try_from(width).map_err(|_| oversized_tree())?;
        let mut rows = usize::try_from(height).map_err(|_| oversized_tree())?;
        let mut count = columns.checked_mul(rows).ok_or_else(oversized_tree)?;
        let mut total = count;
        while count > 1 {
            columns = columns.div_ceil(2);
            rows = rows.div_ceil(2);
            // A parent level never has more nodes than the already checked leaf level.
            count = columns * rows;
            total = total.checked_add(count).ok_or_else(oversized_tree)?;
        }
        Ok(total)
    }

    pub(crate) fn configure(&mut self, width: u32, height: u32, total: usize) -> Result<(), Error> {
        self.nodes
            .try_reserve_exact(total.saturating_sub(self.nodes.len()))
            .map_err(|_| Error::static_message(ErrorKind::NoMemory, "Not enough memory for a packet tag tree\n"))?;
        self.nodes.clear();
        self.nodes.resize(total, TagNode::UNSET);
        self.width = usize::try_from(width).map_err(|_| oversized_tree())?;
        self.height = usize::try_from(height).map_err(|_| oversized_tree())?;
        Ok(())
    }

    /// Decodes the path from the root to `leaf` and reports whether its tag is below `threshold`.
    /// Each call retains the bits already decoded for later calls in the same precinct.
    pub(crate) fn decode(&mut self, bio: &mut Bio<'_>, leaf: u32, threshold: i32) -> Result<bool, Error> {
        self.decode_until(bio, leaf, threshold).map(|value| value < threshold)
    }

    /// Decodes a complete tag. Each descendant starts consuming bits only after its
    /// parent's value is known, so a single walk matches increasing the threshold one at a time.
    fn decode_value(&mut self, bio: &mut Bio<'_>, leaf: u32) -> Result<i32, Error> {
        self.decode_until(bio, leaf, TagNode::MAX_VALUE + 1)
    }

    fn decode_until(&mut self, bio: &mut Bio<'_>, leaf: u32, threshold: i32) -> Result<i32, Error> {
        let leaf = usize::try_from(leaf).map_err(|_| invalid_leaf())?;
        if let [node] = self.nodes.as_mut_slice()
            && leaf == 0
        {
            node.decode(bio, 0, threshold);
            return Ok(node.value);
        }
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
            low = self.nodes[index].decode(bio, low, threshold);
        }
        Ok(self.nodes[leaf].value)
    }
}

fn invalid_leaf() -> Error {
    Error::new(ErrorKind::Data, "Packet tag tree leaf is outside the precinct\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Rect;
    use crate::tile::{Band, CodeBlock, Precinct, Resolution, TileComp};

    fn single_block(cblksty: u8) -> Tile {
        let rect = Rect {
            x0: 0,
            y0: 0,
            x1: 2,
            y1: 2,
        };
        let precinct = Precinct {
            rect,
            cw: 1,
            ch: 1,
            cblks: vec![CodeBlock::default()],
            incl: TagTree::new(1, 1).unwrap(),
            imsb: TagTree::new(1, 1).unwrap(),
        };
        let band = Band {
            rect,
            numbps: 8,
            precincts: vec![precinct],
            ..Band::default()
        };
        let res = Resolution {
            rect,
            pw: 1,
            ph: 1,
            pdx: 1,
            pdy: 1,
            bands: vec![band],
        };
        let comp = TileComp {
            rect,
            dx: 1,
            dy: 1,
            cblksty,
            minimum_num_resolutions: 1,
            res: vec![res],
            ..TileComp::default()
        };
        Tile {
            rect,
            comps: vec![comp],
        }
    }

    fn stream<'data, 'cursor, 'ppm>(data: &'data [u8], numlayers: u16) -> PacketInput<'data, 'cursor, 'ppm> {
        PacketInput {
            coding: TileCoding {
                csty: 0,
                prg: 0,
                numlayers,
                mct: 0,
            },
            pocs: &[],
            layers: 0,
            strict: false,
            cut: false,
            data,
            ppm: None,
            ppt: None,
        }
    }

    fn first_block(tile: &Tile) -> &CodeBlock {
        &tile.comps[0].res[0].bands[0].precincts[0].cblks[0]
    }

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
    fn packed_fields_match_single_bit_reads_at_every_offset() {
        let bytes = [0xab, 0xff, 0x7f, 0xff, 0x01, 0xff];
        for length in 0..=bytes.len() {
            for offset in 0..=32 {
                for width in 0..=32 {
                    let mut field = Bio::new(&bytes[..length]);
                    let mut bitwise = Bio::new(&bytes[..length]);
                    assert_eq!(field.read(offset), (0..offset).fold(0, |v, _| (v << 1) | bitwise.bit()));
                    assert_eq!(field.read(width), (0..width).fold(0, |v, _| (v << 1) | bitwise.bit()));
                    assert_eq!(field.num_bytes(), bitwise.num_bytes());
                    assert_eq!(field.overrun(), bitwise.overrun());
                    assert_eq!(field.align(), bitwise.align());
                }
            }
        }
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

        tree.configure(2, 1, 3).unwrap();
        assert!(!tree.decode(&mut next, 0, 1).unwrap());
        assert!(next.overrun());
    }

    #[test]
    fn complete_tags_match_successive_thresholds_with_and_without_input() {
        let bytes = [0x20, 0x3f, 0xff, 0x56, 0xe3, 0x76, 0x01, 0x21, 0x11, 0xff];
        for (width, height) in [(1, 1), (2, 1), (3, 2), (7, 5)] {
            for length in 0..=bytes.len() {
                let mut direct = TagTree::new(width, height).unwrap();
                let mut stepped = TagTree::new(width, height).unwrap();
                let mut first = Bio::new(&bytes[..length]);
                let mut second = Bio::new(&bytes[..length]);
                for leaf in (0..width * height).rev() {
                    let value = direct.decode_value(&mut first, leaf).unwrap();
                    let mut threshold = 0;
                    while !stepped.decode(&mut second, leaf, threshold).unwrap() {
                        threshold += 1;
                    }
                    assert_eq!(value + 1, threshold);
                    assert_eq!(first.num_bytes(), second.num_bytes());
                    assert_eq!(first.bits_left, second.bits_left);
                    assert_eq!(first.overrun(), second.overrun());
                }
            }
        }
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

    #[test]
    fn layers_and_cut_segments_keep_only_complete_wanted_contributions() {
        let mut workspace = Workspace::default();
        let mut budget = WorkBudget::default();
        let mut diagnostics = Diagnostics::default();
        // First inclusion and zero-bit-plane code, one pass and length 2 in layer 0.
        // Already included, one pass and length 2 in layer 1.
        let bytes = [0xe2, 0xaa, 0xbb, 0xc4, 0xcc, 0xdd];

        let mut tile = single_block(0);
        let cut = PacketInput {
            cut: true,
            ..stream(&bytes[..5], 2)
        };
        assert_eq!(
            read_packets(&mut workspace, &mut tile, cut, &mut budget, &mut diagnostics).unwrap(),
            [0]
        );
        let block = first_block(&tile);
        assert_eq!(block.chunks.len(), 1);
        assert_eq!(block.chunks[0], 1..3);
        assert_eq!(block.real_num_segs, 1);
        assert_eq!(block.segs[0].numpasses, 2);
        assert_eq!(block.segs[0].real_num_passes, 1);
        assert!(block.corrupted);
        assert_eq!(diagnostics.entries().len(), 1);

        let limited = PacketInput {
            layers: 1,
            ..stream(&bytes, 2)
        };
        let mut tile = single_block(0);
        read_packets(&mut workspace, &mut tile, limited, &mut budget, &mut diagnostics).unwrap();
        let block = first_block(&tile);
        assert_eq!(block.chunks.len(), 1);
        assert_eq!(block.chunks[0], 1..3);
        assert_eq!(block.segs[0].numpasses, 2);
        assert_eq!(block.segs[0].real_num_passes, 1);
        assert!(!block.corrupted);

        let strict = PacketInput {
            strict: true,
            ..stream(&bytes[..5], 2)
        };
        let mut tile = single_block(0);
        let error = read_packets(&mut workspace, &mut tile, strict, &mut budget, &mut diagnostics).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Data);
        assert!(error.message().starts_with("read: segment too long (2) with max (1)"));
    }

    #[test]
    fn packed_header_cursor_crosses_tiles_and_precedes_ppt() {
        let mut workspace = Workspace::default();
        let mut budget = WorkBudget::default();
        let mut diagnostics = Diagnostics::default();
        let mut ppm = &[0xe2, 0xe2][..];
        for left in [1, 0] {
            let mut tile = single_block(0);
            let packed = PacketInput {
                ppm: Some(&mut ppm),
                ppt: Some(&[0]),
                ..stream(&[0xaa, 0xbb], 1)
            };
            read_packets(&mut workspace, &mut tile, packed, &mut budget, &mut diagnostics).unwrap();
            assert_eq!(ppm.len(), left);
            let block = first_block(&tile);
            assert_eq!(block.chunks.len(), 1);
            assert_eq!(block.chunks[0], 0..2);
        }

        let mut tile = single_block(0);
        let packed = PacketInput {
            ppt: Some(&[0xe2]),
            ..stream(&[0xaa, 0xbb], 1)
        };
        read_packets(&mut workspace, &mut tile, packed, &mut budget, &mut diagnostics).unwrap();
        assert_eq!(first_block(&tile).chunks[0], 0..2);
    }

    #[test]
    fn sop_and_eph_are_outside_the_retained_segment() {
        let mut workspace = Workspace::default();
        let mut budget = WorkBudget::default();
        let mut diagnostics = Diagnostics::default();
        let mut tile = single_block(0);
        let input = [0xff, 0x91, 0, 4, 0, 1, 0xe2, 0xff, 0x92, 0xaa, 0xbb];
        let mut framed = stream(&input, 1);
        framed.coding.csty = 6;
        framed.strict = true;
        read_packets(&mut workspace, &mut tile, framed, &mut budget, &mut diagnostics).unwrap();
        assert_eq!(first_block(&tile).chunks[0], 9..11);

        let mut framed = stream(&input[..8], 1);
        framed.coding.csty = 6;
        framed.strict = true;
        let mut tile = single_block(0);
        let error = read_packets(&mut workspace, &mut tile, framed, &mut budget, &mut diagnostics).unwrap_err();
        assert_eq!(error.message(), MISSING_EPH);
    }

    #[test]
    fn a_warmed_workspace_reuses_storage_and_charges_the_same_tile_budget() {
        let mut workspace = Workspace::default();
        let mut budget = WorkBudget::default();
        let mut diagnostics = Diagnostics::default();
        let bytes = [0xe2, 0xaa, 0xbb];
        let warm = read_packets(
            &mut workspace,
            &mut single_block(0),
            stream(&bytes, 1),
            &mut budget,
            &mut diagnostics,
        )
        .unwrap()
        .as_ptr();
        let first = workspace.first_pass_failed.as_ptr();
        let expected_watermark = budget.high_water();

        budget.reset(u64::MAX, 0);
        let again = read_packets(
            &mut workspace,
            &mut single_block(0),
            stream(&bytes, 1),
            &mut budget,
            &mut diagnostics,
        )
        .unwrap()
        .as_ptr();
        assert_eq!(warm, again);
        assert_eq!(first, workspace.first_pass_failed.as_ptr());
        assert_eq!(budget.high_water(), expected_watermark);
    }
}
