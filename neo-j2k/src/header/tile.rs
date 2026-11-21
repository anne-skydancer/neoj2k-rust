// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Tile-part marker parsing and effective tile-component coding parameters.

use std::num::NonZeroUsize;

use super::{
    CAP, COC, COD, COM, CPF, CRG, CodingStyle, HeaderSettings, MAX_POCS, MainHeader, PLM, PLT, POC, PPM, PPT,
    PocEntry, QCC, QCD, Quantization, RGN, SIZ, SOP, SOT, TLM, TileCoding, be16, coding,
    component_room, invalid, no_memory, push_list, read_component, read_rgn,
};
#[cfg(test)]
use super::{EOC, EPH, SOC, SOD, be32, need_data};
use crate::error::Diagnostics;
use crate::mem::{CType, HeaderLedger};
use crate::{Error, ErrorKind};

const PAGE_TILES: usize = 128;

/// A directory of overrides for tiles whose tile parts supplied them.
#[derive(Debug, Default)]
pub(crate) struct TileDirectory<'data> {
    pages: Vec<Option<NonZeroUsize>>,
    slots: Vec<[Option<NonZeroUsize>; PAGE_TILES]>,
    overrides: Vec<TileOverrides<'data>>,
}

impl<'data> TileDirectory<'data> {
    pub(crate) fn find(&self, tile: u32) -> Option<&TileOverrides<'data>> {
        let tile = tile as usize;
        let page = self.pages.get(tile / PAGE_TILES).copied().flatten()?.get() - 1;
        let index = self.slots[page][tile % PAGE_TILES]?.get() - 1;
        Some(&self.overrides[index])
    }

    fn ensure(
        &mut self,
        tile_count: u32,
        tile: u16,
        ledger: &mut HeaderLedger,
    ) -> Result<&mut TileOverrides<'data>, Error> {
        if u32::from(tile) >= tile_count {
            return Err(invalid(format!("Invalid tile number {tile}\n")));
        }
        let page = usize::from(tile) / PAGE_TILES;
        let slot = usize::from(tile) % PAGE_TILES;
        if self.pages.is_empty() {
            let page_count = (tile_count as usize).div_ceil(PAGE_TILES);
            // The C++ directory holds one eight-byte pointer per potential page.
            ledger.array(CType::U64, page_count)?;
            self.pages.try_reserve_exact(page_count).map_err(|_| no_memory())?;
            self.pages.resize(page_count, None);
        }
        if self.pages[page].is_none() {
            ledger.array(CType::TileDirectoryPage, 1)?;
            self.slots.try_reserve(1).map_err(|_| no_memory())?;
            self.slots.push([None; PAGE_TILES]);
            self.pages[page] = NonZeroUsize::new(self.slots.len());
        }
        let page = self.pages[page].expect("page was just allocated").get() - 1;
        if self.slots[page][slot].is_none() {
            ledger.array(CType::TileOverrides, 1)?;
            self.overrides.try_reserve(1).map_err(|_| no_memory())?;
            self.overrides.push(TileOverrides {
                tile,
                ..TileOverrides::default()
            });
            self.slots[page][slot] = NonZeroUsize::new(self.overrides.len());
        }
        let index = self.slots[page][slot].expect("tile was just allocated").get() - 1;
        Ok(&mut self.overrides[index])
    }
}

#[derive(Debug, Default)]
pub(crate) struct TileOverrides<'data> {
    tile: u16,
    cod: Option<(TileCoding, CodingStyle)>,
    qcd: Option<Quantization>,
    coc: Vec<(u16, CodingStyle)>,
    qcc: Vec<(u16, Quantization)>,
    rgn: Vec<(u16, u8)>,
    pub(crate) poc: Vec<PocEntry>,
    ppt: Vec<(u8, &'data [u8])>,
}

impl<'data> TileOverrides<'data> {
    pub(crate) fn has_own_coding(&self) -> bool {
        self.cod.is_some() || !self.coc.is_empty()
    }

    pub(crate) fn packed_headers(&self) -> impl Iterator<Item = (u8, &'data [u8])> + '_ {
        self.ppt.iter().copied()
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct TileMarker<'data> {
    pub(crate) tile: u16,
    pub(crate) code: u16,
    pub(crate) body: &'data [u8],
}

fn read_poc_entries(
    body: &[u8],
    numcomps: u32,
    main_pocs: usize,
    tile: &mut TileOverrides<'_>,
    ledger: &mut HeaderLedger,
) -> Result<(), Error> {
    let room = component_room(numcomps);
    let entry_size = 5 + 2 * room;
    if body.is_empty() || !body.len().is_multiple_of(entry_size) {
        return Err(invalid("Error reading POC marker\n"));
    }
    let entries = main_pocs + tile.poc.len() + body.len() / entry_size;
    if entries >= MAX_POCS {
        return Err(invalid(format!("Too many POCs {entries}\n")));
    }
    for bytes in body.chunks_exact(entry_size) {
        let entry = PocEntry {
            resno0: bytes[0],
            compno0: read_component(&bytes[1..], room),
            layno1: be16(&bytes[1 + room..]),
            resno1: bytes[3 + room],
            compno1: read_component(&bytes[4 + room..], room),
            prg: bytes[4 + 2 * room],
        };
        push_list(&mut tile.poc, CType::PocEntry, entry, ledger)?;
    }
    Ok(())
}

fn add_ppt<'data>(body: &'data [u8], tile: &mut TileOverrides<'data>, ledger: &mut HeaderLedger) -> Result<(), Error> {
    if body.len() < 2 {
        return Err(invalid("Error reading PPT marker\n"));
    }
    let index = body[0];
    if tile.ppt.iter().any(|chunk| chunk.0 == index) {
        return Err(invalid(format!("Zppt {index} already read\n")));
    }
    let chunk = (index, &body[1..]);
    push_list(&mut tile.ppt, CType::PackedChunk, chunk, ledger)
}

fn check_plt(body: &[u8]) -> Result<(), Error> {
    const PLT_ERROR: &str = "Error reading PLT marker\n";
    let (_, lengths) = body.split_first().ok_or_else(|| invalid(PLT_ERROR))?;
    let mut length = 0u32;
    for &byte in lengths {
        length |= u32::from(byte & 0x7f);
        if byte & 0x80 != 0 {
            length <<= 7;
        } else {
            length = 0;
        }
    }
    if length != 0 {
        return Err(invalid(PLT_ERROR));
    }
    Ok(())
}

/// Validate and record a single segment whose marker code and complete body were already read.
/// The decode walk charges the body to its tile arena before this call.
pub(crate) fn read_tile_part_marker<'data>(
    main: &MainHeader,
    directory: &mut TileDirectory<'data>,
    marker: TileMarker<'data>,
    settings: HeaderSettings,
    ledger: &mut HeaderLedger,
    diagnostics: &mut Diagnostics,
) -> Result<(), Error> {
    let TileMarker { tile, code, body } = marker;
    let tile = match code {
        COD | COC | QCD | QCC | RGN | POC | PPT => directory.ensure(main.tile_count(), tile, ledger)?,
        PLT => return check_plt(body),
        COM => return Ok(()),
        SOT | SIZ | TLM | PLM | PPM | CRG | CAP | CPF | SOP => {
            return Err(invalid("Marker is not compliant with its position\n"));
        }
        _ => {
            diagnostics.warn_fmt(format_args!(
                "Unknown marker 0x{code:04X} ({} bytes) skipped in a tile-part header\n",
                body.len(),
            ));
            return Ok(());
        }
    };
    // A C++ component override containing a pointer has the same layout as PackedChunk
    // (16 bytes, 8-byte alignment), and a u16/u8 override occupies four bytes.
    match code {
        COD => tile.cod = Some(coding::read_cod(body, settings.limits, diagnostics)?),
        COC => {
            let value = coding::read_coc(body, main.numcomps, diagnostics)?;
            ledger.array(CType::CodingStyle, 1)?;
            push_list(&mut tile.coc, CType::PackedChunk, value, ledger)?;
        }
        QCD => {
            ledger.array(CType::Quantization, 1)?;
            tile.qcd = Some(coding::read_sqcd(body, "QCD", diagnostics)?);
        }
        QCC => {
            ledger.array(CType::Quantization, 1)?;
            let value = coding::read_qcc(body, main.numcomps, diagnostics)?;
            push_list(&mut tile.qcc, CType::PackedChunk, value, ledger)?;
        }
        RGN => {
            let value = read_rgn(body, main.numcomps, diagnostics)?;
            push_list(&mut tile.rgn, CType::U32, value, ledger)?;
        }
        POC => read_poc_entries(body, main.numcomps, main.poc.len(), tile, ledger)?,
        PPT => {
            if !main.ppm_data.is_empty() {
                return Err(invalid(
                    "Error reading PPT marker: packet header have been previously found in the main header (PPM marker).\n",
                ));
            }
            add_ppt(body, tile, ledger)?;
        }
        _ => unreachable!("non-override markers were already handled"),
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
#[cfg(test)]
pub(crate) struct TilePartHeader {
    pub(crate) sot: usize,
    pub(crate) data: usize,
    pub(crate) psot: u32,
    pub(crate) data_length: u32,
    pub(crate) tile: u16,
    pub(crate) tpsot: u8,
    pub(crate) tnsot: u8,
}

#[cfg(test)]
fn subtract_marker_room(room: u32, size: u32) -> Result<u32, Error> {
    room.checked_sub(size)
        .ok_or_else(|| invalid("Sot length is less than marker size + marker ID\n"))
}

/// Read a tile-part header from its SOT marker through SOD. The main decode walk can feed the
/// individual segments instead when it needs its OpenJPEG stream-state and truncation handling.
#[cfg(test)]
pub(crate) fn read_tile_part_header<'data>(
    data: &'data [u8],
    sot: usize,
    main: &MainHeader,
    directory: &mut TileDirectory<'data>,
    settings: HeaderSettings,
    ledger: &mut HeaderLedger,
    diagnostics: &mut Diagnostics,
) -> Result<TilePartHeader, Error> {
    let missing_sot = || need_data("a SOT marker segment");
    let bytes = data.get(sot..).ok_or_else(missing_sot)?;
    let (marker_and_length, bytes) = bytes.split_at_checked(4).ok_or_else(missing_sot)?;
    if be16(marker_and_length) != SOT || be16(&marker_and_length[2..]) != 10 {
        return Err(invalid("Error reading SOT marker\n"));
    }
    let (sot_body, mut bytes) = bytes.split_at_checked(8).ok_or_else(missing_sot)?;
    let tile = be16(sot_body);
    if u32::from(tile) >= main.tile_count() {
        return Err(invalid(format!("Invalid tile number {tile}\n")));
    }
    let psot = be32(&sot_body[2..]);
    if psot != 0 && psot < 14 {
        if psot != 12 {
            return Err(invalid(format!(
                "Psot value is not correct regards to the JPEG2000 norm: {psot}.\n",
            )));
        }
        diagnostics.warn_fmt(format_args!("Empty SOT marker detected: Psot={psot}.\n"));
    }
    let missing_header = || need_data("a tile-part header");
    let mut room = psot.saturating_sub(12);
    loop {
        let (marker, rest) = bytes.split_at_checked(2).ok_or_else(missing_header)?;
        let code = be16(marker);
        bytes = rest;
        match code {
            SOD => break,
            0..=0xfeff => {
                return Err(invalid(format!(
                    "A marker ID was expected (0xff--) instead of {code:08x}\n"
                )));
            }
            SOC | EOC | EPH => return Err(invalid("Marker is not compliant with its position\n")),
            0xff30..=0xff3f => {
                if psot != 0 {
                    room = subtract_marker_room(room, 2)?;
                }
                diagnostics.warn_fmt(format_args!(
                    "Marker 0x{code:04X} without a segment skipped in a tile-part header\n"
                ));
                continue;
            }
            _ => {}
        }
        let (length, rest) = bytes.split_at_checked(2).ok_or_else(missing_header)?;
        let length = be16(length);
        if length < 2 {
            return Err(invalid("Inconsistent marker size\n"));
        }
        if psot != 0 {
            room = subtract_marker_room(room, u32::from(length) + 2)?;
        }
        if (0xff70..=0xff7f).contains(&code) {
            return Err(Error::new(
                ErrorKind::Unsupported,
                format!("Marker 0x{code:04X} is a JPEG 2000 Part 2 extension, not supported\n"),
            ));
        }
        let (body, rest) = rest
            .split_at_checked(usize::from(length - 2))
            .ok_or_else(missing_header)?;
        bytes = rest;
        ledger.charge(body.len(), 1)?;
        read_tile_part_marker(
            main,
            directory,
            TileMarker { tile, code, body },
            settings,
            ledger,
            diagnostics,
        )?;
    }
    Ok(TilePartHeader {
        sot,
        data: data.len() - bytes.len(),
        psot,
        data_length: room.checked_sub(2).unwrap_or(room),
        tile,
        tpsot: sot_body[6],
        tnsot: sot_body[7],
    })
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct TileComponentParams<'a> {
    pub(crate) coding: &'a TileCoding,
    pub(crate) style: &'a CodingStyle,
    pub(crate) quant: &'a Quantization,
    pub(crate) roishift: u8,
}

fn component_defaults<'a>(
    main: &'a MainHeader,
    tile: Option<&'a TileOverrides<'_>>,
    comp: u32,
) -> TileComponentParams<'a> {
    let (coding, style) = tile
        .and_then(|tile| tile.cod.as_ref())
        .map_or_else(|| (&main.cod, main.main_style(comp)), |(coding, style)| (coding, style));
    TileComponentParams {
        coding,
        style,
        quant: tile
            .and_then(|tile| tile.qcd.as_ref())
            .unwrap_or_else(|| main.main_quant(comp)),
        roishift: main.main_roishift(comp),
    }
}

fn last_value<T>(overrides: &[(u16, T)], comp: u32) -> Option<&T> {
    overrides
        .iter()
        .rev()
        .find_map(|(index, value)| (u32::from(*index) == comp).then_some(value))
}

fn check_steps(params: TileComponentParams<'_>, tile: Option<u16>, comp: u32) -> Result<(), Error> {
    let bands = 3 * u32::from(params.style.numresolutions) - 2;
    let count = params.quant.nbands;
    if params.quant.qntsty == coding::QNT_DERIVED || u32::from(count) >= bands {
        return Ok(());
    }
    match tile {
        Some(tile) => Err(invalid(format!(
            "Tile {tile}, component {comp}: the quantization gives {count} step sizes for {bands} sub-bands\n",
        ))),
        None => Err(invalid(format!(
            "Component {comp}: the quantization gives {count} step sizes for {bands} sub-bands\n",
        ))),
    }
}

pub(crate) fn resolve_tile_component<'a>(
    main: &'a MainHeader,
    tile: Option<&'a TileOverrides<'_>>,
    comp: u32,
) -> Result<TileComponentParams<'a>, Error> {
    if comp >= main.numcomps {
        return Err(Error::new(ErrorKind::Argument, "Component index is out of range\n"));
    }
    let mut params = component_defaults(main, tile, comp);
    if let Some(tile) = tile {
        params.style = last_value(&tile.coc, comp).unwrap_or(params.style);
        params.quant = last_value(&tile.qcc, comp).unwrap_or(params.quant);
        params.roishift = last_value(&tile.rgn, comp).copied().unwrap_or(params.roishift);
    }
    check_steps(params, tile.map(|tile| tile.tile), comp)?;
    Ok(params)
}

/// Resolve all component parameters in one pass over the defaults and the sparse overrides.
pub(crate) fn resolve_tile<'a>(
    main: &'a MainHeader,
    tile: Option<&'a TileOverrides<'_>>,
) -> Result<Vec<TileComponentParams<'a>>, Error> {
    let mut params = Vec::new();
    params
        .try_reserve_exact(main.numcomps as usize)
        .map_err(|_| no_memory())?;
    params.extend((0..main.numcomps).map(|comp| component_defaults(main, tile, comp)));
    if let Some(tile) = tile {
        for (comp, value) in &tile.coc {
            params[usize::from(*comp)].style = value;
        }
        for (comp, value) in &tile.qcc {
            params[usize::from(*comp)].quant = value;
        }
        for (comp, value) in &tile.rgn {
            params[usize::from(*comp)].roishift = *value;
        }
    }
    for (comp, value) in (0..main.numcomps).zip(&params) {
        check_steps(*value, tile.map(|tile| tile.tile), comp)?;
    }
    Ok(params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Limits;

    fn marker(data: &mut Vec<u8>, code: u16, body: &[u8]) {
        data.extend_from_slice(&code.to_be_bytes());
        data.extend_from_slice(&u16::try_from(body.len() + 2).unwrap().to_be_bytes());
        data.extend_from_slice(body);
    }

    fn settings() -> HeaderSettings {
        HeaderSettings {
            limits: Limits::default(),
            ihdr_width: 0,
            ihdr_height: 0,
            allow_varying_depth: true,
        }
    }

    fn codestream() -> (MainHeader, Vec<u8>) {
        let mut data = SOC.to_be_bytes().to_vec();
        let mut siz = vec![0, 0];
        for value in [9u32, 11, 0, 0, 9, 11, 0, 0] {
            siz.extend_from_slice(&value.to_be_bytes());
        }
        siz.extend_from_slice(&[0, 1, 7, 1, 1]);
        marker(&mut data, SIZ, &siz);
        marker(&mut data, COD, &[0, 0, 0, 1, 0, 0, 4, 4, 0, 1]);
        marker(&mut data, QCD, &[0, 0]);
        marker(&mut data, SOT, &[0, 0, 0, 0, 0, 0, 0, 1]);
        let main = super::super::parse_main_header(
            &data,
            0,
            settings(),
            &mut HeaderLedger::default(),
            &mut Diagnostics::default(),
        )
        .unwrap();
        (main, data)
    }

    fn parse<'data>(
        main: &MainHeader,
        data: &'data [u8],
        dir: &mut TileDirectory<'data>,
    ) -> Result<TilePartHeader, Error> {
        read_tile_part_header(
            data,
            usize::try_from(main.end).unwrap(),
            main,
            dir,
            settings(),
            &mut HeaderLedger::default(),
            &mut Diagnostics::default(),
        )
    }

    #[test]
    fn malformed_and_cut_tile_headers_have_distinct_statuses() {
        let (main, mut data) = codestream();
        let sot = usize::try_from(main.end).unwrap();
        data.extend_from_slice(&[0xff, 0x64, 0, 6, 0]);
        assert_eq!(
            parse(&main, &data, &mut TileDirectory::default()).unwrap_err().kind(),
            ErrorKind::NeedData
        );
        data[sot + 9] = 14;
        let error = parse(&main, &data, &mut TileDirectory::default()).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Header);
        assert_eq!(error.message(), "Sot length is less than marker size + marker ID\n");
    }

    #[test]
    fn component_overrides_win_and_quantization_is_checked_after_resolution() {
        let (main, mut data) = codestream();
        marker(&mut data, COC, &[0, 0, 1, 4, 4, 0, 1]);
        marker(&mut data, COD, &[0, 1, 0, 2, 0, 0, 4, 4, 0, 1]);
        marker(&mut data, QCC, &[0, 1, 0x50, 0]);
        marker(&mut data, QCD, &[0, 0]);
        marker(&mut data, RGN, &[0, 0, 3]);
        marker(&mut data, PPT, &[0, 0x41]);
        data.extend_from_slice(&SOD.to_be_bytes());
        let mut directory = TileDirectory::default();
        let part = parse(&main, &data, &mut directory).unwrap();
        assert_eq!(part.tile, 0);
        assert_eq!(part.sot as u64, main.end);
        assert_eq!(part.data, data.len());
        assert_eq!((part.psot, part.data_length, part.tpsot, part.tnsot), (0, 0, 0, 1));
        let tile = directory.find(0).unwrap();
        let component = resolve_tile_component(&main, Some(tile), 0).unwrap();
        assert!(tile.has_own_coding());
        assert_eq!(component.coding.prg, 1);
        assert_eq!(component.style.numresolutions, 2);
        assert_eq!(component.quant.step(0), (10, 0));
        assert_eq!(component.roishift, 3);
        assert_eq!(tile.packed_headers().collect::<Vec<_>>(), [(0, &[0x41][..])]);

        let (main, mut short_quantization) = codestream();
        marker(&mut short_quantization, COC, &[0, 0, 1, 4, 4, 0, 1]);
        short_quantization.extend_from_slice(&SOD.to_be_bytes());
        let mut directory = TileDirectory::default();
        parse(&main, &short_quantization, &mut directory).unwrap();
        let error = resolve_tile(&main, directory.find(0)).unwrap_err();
        assert_eq!(
            error.message(),
            "Tile 0, component 0: the quantization gives 1 step sizes for 4 sub-bands\n"
        );
    }
}
