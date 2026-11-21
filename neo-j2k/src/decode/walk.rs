// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// Tile-part order and exhaustion rules follow OpenJPEG's j2k.c through src/decode.cpp.
// See LICENSE.OpenJPEG for OpenJPEG's notices.

use crate::error::Diagnostics;
use crate::header::{
    HeaderSettings, MainHeader,
    tile::{self, TileDirectory, TileMarker},
};
use crate::mem::{CType, HeaderLedger, WorkBudget, capacity_bytes};
use crate::stream::StreamBounds;
use crate::{Error, ErrorKind, Limits};
use core::ops::Range;

const SOT: u16 = 0xff90;
const SOD: u16 = 0xff93;
const EOC: u16 = 0xffd9;
const PAGE_SIZE: u32 = 128;
// Layout of TileWalk and Range in src/decode.cpp on the supported 64-bit platforms.
const WALK_BYTES: usize = 32;
const RANGE_BYTES: usize = 16;
const SHORT_STREAM: &str = "Stream too short\n";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Position {
    Sot,
    Header,
    Cut,
    Eoc,
}

#[derive(Debug, Default)]
pub(super) struct TileParts {
    next_part: u32,
    total_parts: u32,
    seen: bool,
    cut: bool,
    ppt_merged: bool,
    ranges: Vec<Range<usize>>,
}

impl TileParts {
    fn reset(&mut self) {
        self.next_part = 0;
        self.total_parts = 0;
        self.seen = false;
        self.cut = false;
        self.ppt_merged = false;
        self.ranges.clear();
    }
}

#[derive(Debug, Default)]
struct Page {
    active: bool,
    tiles: Vec<TileParts>,
}

#[derive(Debug, Default)]
pub(super) struct Pages {
    parts: PartPages,
    overrides: TileDirectory,
}

impl Pages {
    pub(super) fn retained_bytes(&self) -> u64 {
        self.overrides.retained_bytes()
            + capacity_bytes(&self.parts.pages)
            + self
                .parts
                .pages
                .iter()
                .map(|page| {
                    capacity_bytes(&page.tiles)
                        + page.tiles.iter().map(|tile| capacity_bytes(&tile.ranges)).sum::<u64>()
                })
                .sum::<u64>()
    }
}

#[derive(Debug, Default)]
struct PartPages {
    active: bool,
    pages: Vec<Page>,
}

impl PartPages {
    fn reset(&mut self) {
        self.active = false;
        for page in &mut self.pages {
            if page.active {
                for tile in &mut page.tiles {
                    tile.reset();
                }
                page.active = false;
            }
        }
    }

    fn make(&mut self, tile: u32, tiles: u32, budget: &mut WorkBudget) -> Result<&mut TileParts, Error> {
        if !self.active {
            let count = tiles.div_ceil(PAGE_SIZE) as usize;
            budget.array(CType::U64, count)?;
            super::super::tile::resize(&mut self.pages, count)?;
            self.active = true;
        }
        let index = tile / PAGE_SIZE;
        let page = &mut self.pages[index as usize];
        if !page.active {
            let count = (tiles - index * PAGE_SIZE).min(PAGE_SIZE) as usize;
            budget.charge(count * WALK_BYTES, 16)?;
            super::super::tile::resize(&mut page.tiles, count)?;
            page.active = true;
        }
        Ok(&mut page.tiles[(tile % PAGE_SIZE) as usize])
    }

    fn get(&self, tile: u32) -> Option<&TileParts> {
        let page = self.pages.get((tile / PAGE_SIZE) as usize).filter(|page| page.active)?;
        page.tiles.get((tile % PAGE_SIZE) as usize)
    }

    fn get_mut(&mut self, tile: u32) -> &mut TileParts {
        &mut self.pages[(tile / PAGE_SIZE) as usize].tiles[(tile % PAGE_SIZE) as usize]
    }
}

pub(super) struct Walk<'a, 'data> {
    data: &'data [u8],
    stream: Option<StreamBounds>,
    main: &'a MainHeader,
    pages: &'a mut PartPages,
    pub(super) overrides: &'a mut TileDirectory,
    settings: HeaderSettings,
    pos: usize,
    position: Position,
    current_tile: u32,
    can_decode: bool,
    open_ended: bool,
    checked_correction: bool,
    correction: u32,
    room: u32,
    reduce: u32,
    strict: bool,
    cut: bool,
}

impl<'data> Walk<'_, 'data> {
    pub(super) fn new<'a>(
        data: &'data [u8],
        main: &'a MainHeader,
        pages: &'a mut Pages,
        limits: Limits,
        reduce: u32,
        strict: bool,
        stream: Option<StreamBounds>,
    ) -> Walk<'a, 'data> {
        let Pages { parts, overrides } = pages;
        parts.reset();
        overrides.reset();
        Walk {
            data,
            stream,
            main,
            pages: parts,
            overrides,
            settings: HeaderSettings {
                limits,
                ihdr_width: 0,
                ihdr_height: 0,
                allow_varying_depth: true,
            },
            pos: usize::try_from(main.end)
                .unwrap_or(usize::MAX)
                .saturating_add(2)
                .min(data.len()),
            position: Position::Sot,
            current_tile: 0,
            can_decode: false,
            open_ended: false,
            checked_correction: false,
            correction: 0,
            room: 0,
            reduce,
            strict,
            cut: false,
        }
    }

    fn bytes_left(&self) -> usize {
        let Some(stream) = self.stream else {
            return self.data.len() - self.pos;
        };
        stream.claimed_end.map_or(0, |end| {
            usize::try_from(end.saturating_sub(self.pos as u64)).unwrap_or(usize::MAX)
        })
    }

    fn final_eoc(&self, remaining: usize) -> bool {
        if remaining < 2 {
            return false;
        }
        if self.stream.is_some_and(|stream| !stream.has_seek) {
            return true;
        }
        self.pos
            .checked_add(remaining)
            .and_then(|end| self.data.get(end - 2..end))
            .is_none_or(|bytes| bytes == EOC.to_be_bytes())
    }

    pub(super) fn cut_or_incomplete(&self) -> bool {
        self.cut || self.position != Position::Eoc
    }

    pub(super) fn past_cut(&self) -> bool {
        self.position == Position::Cut && self.pos == self.data.len()
    }

    pub(super) fn saw_cut(&self) -> bool {
        self.cut || self.position == Position::Cut
    }

    pub(super) fn tile_cut(&self, tile: u32) -> bool {
        self.pages.get(tile).is_some_and(|parts| parts.cut)
    }

    fn packed_headers(&self, index: u32) -> bool {
        !self.main.ppm_data.is_empty()
            || self
                .overrides
                .find(index)
                .and_then(|tile| tile.packed_headers().next())
                .is_some()
    }

    pub(super) fn decodable(&self, index: u32) -> bool {
        self.pages
            .get(index)
            .is_some_and(|parts| parts.seen && (!parts.ranges.is_empty() || self.packed_headers(index)))
    }

    pub(super) fn tile_bytes<'s>(
        &self,
        tile: u32,
        combined: &'s mut Vec<u8>,
        budget: &mut WorkBudget,
    ) -> Result<&'s [u8], Error>
    where
        'data: 's,
    {
        let ranges = &self.pages.get(tile).expect("a tile was read").ranges;
        if let [only] = ranges.as_slice() {
            return Ok(&self.data[only.clone()]);
        }
        let count: usize = ranges
            .iter()
            .try_fold(0_usize, |size, range| size.checked_add(range.len()))
            .filter(|&size| u32::try_from(size).is_ok())
            .ok_or_else(no_memory)?;
        budget.array(CType::U8, count)?;
        combined.clear();
        combined.try_reserve_exact(count).map_err(|_| no_memory())?;
        for range in ranges {
            combined.extend_from_slice(&self.data[range.clone()]);
        }
        Ok(combined)
    }

    pub(super) fn merge_ppt<'s>(
        &mut self,
        tile: u32,
        output: &'s mut Vec<u8>,
        budget: &mut WorkBudget,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<&'s [u8]>, Error> {
        let Some(overrides) = self.overrides.find(tile) else {
            return Ok(None);
        };
        if overrides.packed_headers().next().is_none() {
            return Ok(None);
        }
        let parts = self.pages.get_mut(tile);
        if parts.ppt_merged {
            diagnostics.error("opj_j2k_merge_ppt() has already been called\n");
            return Err(header_error("Failed to merge PPT data\n"));
        }
        parts.ppt_merged = true;
        merge_ppt(overrides.packed_headers(), self.data, output, budget).map(Some)
    }

    pub(super) fn next_tile(
        &mut self,
        budget: &mut WorkBudget,
        ledger: &mut HeaderLedger,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<u32>, Error> {
        let mut marker = match self.position {
            Position::Eoc => EOC,
            Position::Sot => SOT,
            _ => return Err(data_error("The codestream ended before its next tile-part\n")),
        };
        while !self.can_decode && marker != EOC {
            while marker != SOD {
                if self.bytes_left() == 0 {
                    self.position = Position::Cut;
                    break;
                }
                self.read_marker(marker, budget, ledger, diagnostics)?;
                if self.position == Position::Cut {
                    break;
                }
                marker = self.marker()?;
            }
            if self.past_cut() {
                break;
            }
            self.check_tile_reduce()?;
            self.read_sod(budget, diagnostics)?;
            self.correct_part_count(diagnostics)?;
            if !self.can_decode && !self.strict && self.open_ended && self.bytes_left() == 0 {
                self.position = Position::Cut;
                break;
            }
            if !self.can_decode {
                match self.marker() {
                    Ok(next) => marker = next,
                    Err(error) => {
                        let Some(tile) = self.spot6_missing_end() else {
                            return Err(error);
                        };
                        diagnostics.info_fmt(format_args!(
                            "Tile {tile} has TPsot == 0 and TNsot == 0, but no other tile-parts were found. EOC is also missing.\n"
                        ));
                        self.current_tile = tile;
                        self.position = Position::Eoc;
                        marker = EOC;
                    }
                }
            }
        }

        if marker == EOC && self.position != Position::Eoc {
            self.current_tile = 0;
            self.position = Position::Eoc;
        }
        if !self.can_decode {
            while self.current_tile < self.main.tile_count() && !self.decodable(self.current_tile) {
                self.current_tile += 1;
            }
            if self.current_tile == self.main.tile_count() {
                return Ok(None);
            }
        }
        Ok(Some(self.current_tile))
    }

    fn read_marker(
        &mut self,
        code: u16,
        budget: &mut WorkBudget,
        ledger: &mut HeaderLedger,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Error> {
        let inside = self.position == Position::Header;
        if inside && matches!(code, 0xff4f | 0xff92 | EOC) {
            return Err(wrong_marker());
        }
        if inside && (0xff30..=0xff3f).contains(&code) {
            self.subtract_room(2)?;
            diagnostics.warn_fmt(format_args!(
                "Marker 0x{code:04X} without a segment skipped in a tile-part header\n"
            ));
            return Ok(());
        }

        let marker_len = self.marker()?;
        if marker_len < 2 {
            return Err(header_error("Inconsistent marker size\n"));
        }
        if code == 0x8080 && self.bytes_left() == 0 {
            self.position = Position::Cut;
            return Ok(());
        }
        if inside {
            self.subtract_room(u32::from(marker_len) + 2)?;
        }
        if code < 0xff00 {
            return Err(header_error(format!(
                "A marker ID was expected (0xff--) instead of {code:08x}\n"
            )));
        }
        let sot = code == SOT;
        if inside == sot {
            return Err(wrong_marker());
        }
        if (0xff70..=0xff7f).contains(&code) {
            return Err(Error::new(
                ErrorKind::Unsupported,
                format!("Marker 0x{code:04X} is a JPEG 2000 Part 2 extension, not supported\n"),
            ));
        }
        let size = usize::from(marker_len - 2);
        if size > self.bytes_left() {
            return Err(need_data("Marker size inconsistent with stream length\n"));
        }
        if size > self.data.len() - self.pos {
            return Err(short());
        }
        let body_start = self.pos;
        let body = &self.data[body_start..body_start + size];
        self.pos += size;
        if sot && size > 8 {
            return Err(header_error("Error reading SOT marker\n"));
        }
        let result = if sot {
            self.read_sot(body, budget, diagnostics)
        } else {
            ledger.charge(size, 1)?;
            let tile = u16::try_from(self.current_tile).expect("tile numbers are 16 bit");
            tile::read_tile_part_marker(
                self.main,
                self.overrides,
                TileMarker {
                    tile,
                    code,
                    body,
                    body_start,
                },
                self.settings,
                ledger,
                diagnostics,
            )
        };
        result.map_err(|error| {
            diagnostics.error(error.message());
            Error::formatted(
                error.kind(),
                format_args!("Fail to read the current marker segment ({code:#x})\n"),
                "Fail to read the current marker segment\n",
            )
        })
    }

    fn marker(&mut self) -> Result<u16, Error> {
        self.take_marker().ok_or_else(short)
    }

    fn take_marker(&mut self) -> Option<u16> {
        let Some(bytes) = self.data.get(self.pos..self.pos.saturating_add(2)) else {
            self.pos = self.data.len();
            return None;
        };
        self.pos += 2;
        Some(be16(bytes))
    }

    fn subtract_room(&mut self, used: u32) -> Result<(), Error> {
        if self.room != 0 {
            self.room = self
                .room
                .checked_sub(used)
                .ok_or_else(|| header_error("Sot length is less than marker size + marker ID\n"))?;
        }
        Ok(())
    }

    fn read_sot(&mut self, data: &[u8], budget: &mut WorkBudget, diagnostics: &mut Diagnostics) -> Result<(), Error> {
        if data.len() != 8 {
            diagnostics.error("Error reading SOT marker\n");
            return Err(header_error("Error reading SOT marker\n"));
        }
        let index = u32::from(be16(data));
        let length = be32(&data[2..]);
        let part = u32::from(data[6]);
        let total = u32::from(data[7]);
        if index >= self.main.tile_count() {
            return Err(header_error(format!("Invalid tile number {index}\n")));
        }
        self.current_tile = index;
        let walk = self.pages.make(index, self.main.tile_count(), budget)?;
        if part != walk.next_part {
            let next = walk.next_part;
            return Err(header_error(format!(
                "Invalid tile part index for tile number {index}. Got {part}, expected {next}\n"
            )));
        }
        walk.next_part += 1;
        if length != 0 && length < 14 {
            if length != 12 {
                return Err(header_error(format!(
                    "Psot value is not correct regards to the JPEG2000 norm: {length}.\n"
                )));
            }
            diagnostics.warn("Empty SOT marker detected: Psot=12.\n");
        }
        self.open_ended |= length == 0;
        if walk.total_parts != 0 && part >= walk.total_parts {
            let previous = walk.total_parts;
            return Err(header_error(format!(
                "In SOT marker, TPSot ({part}) is not valid regards to the previous number of tile-part ({previous}), giving up\n"
            )));
        }
        if total != 0 {
            let total = total + self.correction;
            if part >= total {
                return Err(header_error(format!(
                    "In SOT marker, TPSot ({part}) is not valid regards to the current number of tile-part (header) ({total}), giving up\n"
                )));
            }
            walk.total_parts = total;
        }
        self.can_decode = walk.total_parts == part + 1;
        self.room = if self.open_ended { 0 } else { length - 12 };
        self.position = Position::Header;
        Ok(())
    }

    fn check_tile_reduce(&self) -> Result<(), Error> {
        if self.reduce == 0 {
            return Ok(());
        }
        let Some(overrides) = self.overrides.find(self.current_tile).filter(|t| t.has_own_coding()) else {
            return Ok(());
        };
        for index in 0..self.main.numcomps {
            let params = tile::resolve_tile_component(self.main, Some(overrides), index)?;
            let resolutions = u32::from(params.style.numresolutions);
            if self.reduce >= resolutions {
                return Err(super::super::tile::too_few_resolutions(
                    index as usize,
                    self.reduce,
                    resolutions,
                ));
            }
        }
        Ok(())
    }

    fn read_sod(&mut self, budget: &mut WorkBudget, diagnostics: &mut Diagnostics) -> Result<(), Error> {
        let remaining = self.bytes_left();
        let mut length_from_cut = false;
        let length = if self.open_ended {
            if self.strict || self.final_eoc(remaining) {
                remaining.checked_sub(2).ok_or_else(length_error)?
            } else {
                self.cut = true;
                length_from_cut = true;
                remaining
            }
        } else {
            self.room.checked_sub(2).unwrap_or(self.room) as usize
        };
        if length > remaining {
            if self.strict {
                return Err(length_error());
            }
            diagnostics.warn(length_error().message());
        }
        if length > u32::MAX as usize - 2 {
            return Err(data_error(
                "p_j2k->m_specific_param.m_decoder.m_sot_length > UINT_MAX - OPJ_COMMON_CBLK_DATA_EXTRA",
            ));
        }
        let read = length.min(self.data.len() - self.pos);
        let parts = self.pages.get_mut(self.current_tile);
        parts.seen |= length != 0 || !length_from_cut;
        if length != 0 && read == 0 {
            return Err(short());
        }
        if read != length {
            self.cut = true;
            parts.cut = true;
            self.position = Position::Cut;
        } else {
            self.position = Position::Sot;
        }
        parts.cut |= length_from_cut;
        if read != 0 {
            let next = parts.ranges.len();
            if next == 0 || (next >= 4 && next.is_power_of_two()) {
                let capacity = (next * 2).max(4);
                budget.charge(capacity * RANGE_BYTES, 16)?;
                parts
                    .ranges
                    .try_reserve_exact(capacity - next)
                    .map_err(|_| no_memory())?;
            }
            parts.ranges.push(self.pos..self.pos + read);
        }
        self.pos += read;
        Ok(())
    }

    fn correct_part_count(&mut self, diagnostics: &mut Diagnostics) -> Result<(), Error> {
        if !self.can_decode || self.checked_correction {
            return Ok(());
        }
        self.checked_correction = true;
        let parts = self.pages.get(self.current_tile).expect("the tile was read");
        if parts.total_parts == 1 || self.stream.is_some_and(|stream| !stream.has_seek) {
            return Ok(());
        }
        let needs = self.look_for_wrong_part_count().map_err(|error| {
            diagnostics.error(error.message());
            Error::static_message(error.kind(), "opj_j2k_apply_nb_tile_parts_correction error\n")
        })?;
        if needs {
            self.can_decode = false;
            self.correction = 1;
            for page in self.pages.pages.iter_mut().filter(|page| page.active) {
                for tile in &mut page.tiles {
                    tile.total_parts += u32::from(tile.total_parts != 0);
                }
            }
            diagnostics.warn("Non conformant codestream TPsot==TNsot.\n");
        }
        Ok(())
    }

    fn look_for_wrong_part_count(&self) -> Result<bool, Error> {
        let mut next = &self.data[self.pos..];
        while next.get(..2).is_some_and(|bytes| be16(bytes) == SOT) {
            let size = next.get(2..4).map(be16).ok_or_else(short)?;
            if size != 10 {
                return Err(header_error("Inconsistent marker size\n"));
            }
            let body = next.get(4..12).ok_or_else(short)?;
            if u32::from(be16(body)) == self.current_tile {
                return Ok(body[6] == body[7]);
            }
            let length = be32(&body[2..]) as usize;
            if length < 14 {
                break;
            }
            let Some(rest) = next.get(length..) else {
                break;
            };
            next = rest;
        }
        Ok(false)
    }

    fn spot6_missing_end(&self) -> Option<u32> {
        if self.current_tile + 1 != self.main.tile_count() {
            return None;
        }
        (0..self.main.tile_count()).find(|&tile| {
            self.pages
                .get(tile)
                .is_some_and(|parts| parts.next_part == 1 && parts.total_parts == 0)
        })
    }

    pub(super) fn after_tile(&mut self, tile: u32, diagnostics: &mut Diagnostics) -> Result<(), Error> {
        self.can_decode = false;
        if self.position != Position::Eoc && !self.past_cut() {
            match self.take_marker() {
                Some(EOC) => {
                    self.current_tile = 0;
                    self.position = Position::Eoc;
                }
                Some(SOT) => {}
                Some(_) if self.bytes_left() == 0 => {
                    self.position = Position::Cut;
                    self.cut = true;
                    diagnostics.warn("Stream does not end with EOC\n");
                }
                Some(_) => return Err(data_error("Stream too short, expected SOT\n")),
                None => {
                    self.cut = true;
                    if self.strict {
                        return Err(short());
                    }
                    diagnostics.warn(SHORT_STREAM);
                }
            }
        }
        let parts = self.pages.get_mut(tile);
        parts.seen = false;
        parts.cut = false;
        parts.ranges.clear();
        Ok(())
    }
}

fn merge_ppt<'a>(
    headers: impl IntoIterator<Item = (u8, Range<usize>)>,
    input: &[u8],
    output: &'a mut Vec<u8>,
    budget: &mut WorkBudget,
) -> Result<&'a [u8], Error> {
    let mut ordered = [None; 256];
    let mut total = 0_usize;
    for (index, range) in headers {
        let data = input
            .get(range)
            .ok_or_else(|| header_error("Invalid packed packet header range\n"))?;
        ordered[usize::from(index)] = Some(data);
        total = total.checked_add(data.len()).ok_or_else(no_memory)?;
    }
    budget.array(CType::U8, total)?;
    output.clear();
    output.try_reserve_exact(total).map_err(|_| no_memory())?;
    for data in ordered.into_iter().flatten() {
        output.extend_from_slice(data);
    }
    Ok(output)
}

fn be16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes([bytes[0], bytes[1]])
}

fn be32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn length_error() -> Error {
    data_error("Tile part length size inconsistent with stream length\n")
}

fn no_memory() -> Error {
    Error::static_message(ErrorKind::NoMemory, "Not enough memory to decode\n")
}

fn header_error(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Header, message)
}

fn wrong_marker() -> Error {
    header_error("Marker is not compliant with its position\n")
}

pub(super) fn short() -> Error {
    need_data(SHORT_STREAM)
}

pub(super) fn need_data(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::NeedData, message)
}

pub(super) fn data_error(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Data, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_header_ranges_are_merged_by_marker_index() {
        let mut output = Vec::new();
        let mut budget = WorkBudget::default();
        let headers = [(2, 0..1), (0, 5..7), (1, 2..4)];
        let input = [0, 1, 2, 3, 4, 5, 6];
        let packed = merge_ppt(headers.clone(), &input, &mut output, &mut budget).unwrap();
        assert_eq!(packed, [5, 6, 2, 3, 0]);
        let storage = packed.as_ptr();

        let input = [10, 11, 12, 13, 14, 15, 16];
        let packed = merge_ppt(headers, &input, &mut output, &mut budget).unwrap();
        assert_eq!(packed, [15, 16, 12, 13, 10]);
        assert_eq!(packed.as_ptr(), storage);
    }
}
