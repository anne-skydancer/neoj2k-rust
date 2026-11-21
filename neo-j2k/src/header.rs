// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The codestream main header, from SOC through the first SOT marker.

use std::num::NonZeroUsize;

use crate::error::Diagnostics;
use crate::mem::{CType, HeaderLedger};
use crate::{Error, ErrorKind, Limits};

mod coding;
pub(crate) mod tile;

pub(crate) use coding::{CodingStyle, Quantization, TileCoding};

const SOC: u16 = 0xFF4F;
const CAP: u16 = 0xFF50;
const SIZ: u16 = 0xFF51;
const COD: u16 = 0xFF52;
const COC: u16 = 0xFF53;
const TLM: u16 = 0xFF55;
const PLM: u16 = 0xFF57;
const PLT: u16 = 0xFF58;
const CPF: u16 = 0xFF59;
const QCD: u16 = 0xFF5C;
const QCC: u16 = 0xFF5D;
const RGN: u16 = 0xFF5E;
const POC: u16 = 0xFF5F;
const PPM: u16 = 0xFF60;
const PPT: u16 = 0xFF61;
const CRG: u16 = 0xFF63;
const COM: u16 = 0xFF64;
const SOT: u16 = 0xFF90;
const SOP: u16 = 0xFF91;
const EPH: u16 = 0xFF92;
const SOD: u16 = 0xFF93;
const EOC: u16 = 0xFFD9;

const MAX_COMPONENTS: u16 = 16_384;
const MAX_TILES: u64 = 65_535;
const MAX_POCS: usize = 32;

#[derive(Clone, Copy, Debug)]
pub(crate) struct HeaderSettings {
    pub(crate) limits: Limits,
    pub(crate) ihdr_width: u32,
    pub(crate) ihdr_height: u32,
    pub(crate) allow_varying_depth: bool,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SizComponent {
    pub(crate) prec: u8,
    pub(crate) sgnd: u8,
    pub(crate) dx: u8,
    pub(crate) dy: u8,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code, reason = "the packet iterator uses these fields in stage R2")]
pub(crate) struct PocEntry {
    pub(crate) compno0: u16,
    pub(crate) compno1: u16,
    pub(crate) layno1: u16,
    pub(crate) resno0: u8,
    pub(crate) resno1: u8,
    pub(crate) prg: u8,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code, reason = "the tile-part reader uses these fields in stage R2")]
pub(crate) struct TlmEntry {
    pub(crate) tile: u16,
    pub(crate) length: u32,
}

/// The sparse markers use one index per component and one value per override.
#[derive(Debug)]
struct Overrides<T> {
    indices: Vec<Option<NonZeroUsize>>,
    values: Vec<T>,
}

impl<T> Default for Overrides<T> {
    fn default() -> Self {
        Self {
            indices: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl<T> Overrides<T> {
    fn set(&mut self, comp: usize, value: T) -> Result<(), Error> {
        if let Some(index) = self.indices[comp] {
            self.values[index.get() - 1] = value;
        } else {
            self.values.try_reserve(1).map_err(|_| no_memory())?;
            self.values.push(value);
            self.indices[comp] = NonZeroUsize::new(self.values.len());
        }
        Ok(())
    }

    fn get(&self, comp: u32) -> Option<&T> {
        self.indices[comp as usize].map(|index| &self.values[index.get() - 1])
    }
}

#[derive(Debug, Default)]
pub(crate) struct MainHeader {
    pub(crate) rsiz: u16,
    pub(crate) x0: u32,
    pub(crate) y0: u32,
    pub(crate) x1: u32,
    pub(crate) y1: u32,
    pub(crate) tdx: u32,
    pub(crate) tdy: u32,
    pub(crate) tx0: u32,
    pub(crate) ty0: u32,
    pub(crate) tw: u32,
    pub(crate) th: u32,
    pub(crate) numcomps: u32,
    pub(crate) comps: Vec<SizComponent>,
    pub(crate) cod: TileCoding,
    pub(crate) cod_style: CodingStyle,
    pub(crate) qcd: Quantization,
    coc: Overrides<CodingStyle>,
    qcc: Overrides<Quantization>,
    rgn: Vec<Option<u8>>,
    pub(crate) poc: Vec<PocEntry>,
    pub(crate) ppm_data: Vec<u8>,
    pub(crate) ppm_lengths: Vec<u32>,
    pub(crate) tlm_invalid: bool,
    pub(crate) tlm: Vec<TlmEntry>,
    pub(crate) has_cap: bool,
    pub(crate) start: u64,
    pub(crate) end: u64,
    pub(crate) min_resolutions: u32,
}

impl MainHeader {
    pub(crate) fn tile_count(&self) -> u32 {
        self.tw * self.th
    }

    pub(crate) fn main_style(&self, comp: u32) -> &CodingStyle {
        self.coc.get(comp).unwrap_or(&self.cod_style)
    }

    pub(crate) fn main_quant(&self, comp: u32) -> &Quantization {
        self.qcc.get(comp).unwrap_or(&self.qcd)
    }

    #[allow(dead_code, reason = "the tile decoder uses the ROI shift in stage R2")]
    pub(crate) fn main_roishift(&self, comp: u32) -> u8 {
        self.rgn[comp as usize].unwrap_or(0)
    }
}

fn be16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes([bytes[0], bytes[1]])
}

fn be32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Header, message)
}

fn limit(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Limit, message)
}

fn need_data(where_: &str) -> Error {
    Error::new(
        ErrorKind::NeedData,
        format!("Stream too short: the bytes end inside {where_}\n"),
    )
}

fn no_memory() -> Error {
    Error::new(ErrorKind::NoMemory, "Not enough memory to read the headers\n")
}

fn wrap_error(error: Error, message: &str, diagnostics: &mut Diagnostics) -> Error {
    diagnostics.error(error.message());
    Error::new(error.kind(), message)
}

/// Charge the old and new capacities just as a C++ ArenaList growing in place in its arena does.
fn push_list<T>(list: &mut Vec<T>, kind: CType, value: T, ledger: &mut HeaderLedger) -> Result<(), Error> {
    let len = list.len();
    if len == u32::MAX as usize {
        return Err(no_memory());
    }
    if len == 0 || (len >= 4 && len.is_power_of_two()) {
        let capacity = len.saturating_mul(2).max(4).min(u32::MAX as usize);
        ledger.array(kind, capacity)?;
        list.try_reserve_exact(capacity - len).map_err(|_| no_memory())?;
    }
    list.push(value);
    Ok(())
}

fn component_room(numcomps: u32) -> usize {
    if numcomps <= 256 { 1 } else { 2 }
}

fn read_component(bytes: &[u8], room: usize) -> u16 {
    if room == 1 { u16::from(bytes[0]) } else { be16(bytes) }
}

fn read_siz(
    body: &[u8],
    settings: HeaderSettings,
    header: &mut MainHeader,
    ledger: &mut HeaderLedger,
    diagnostics: &mut Diagnostics,
) -> Result<(), Error> {
    if body.len() < 36 || !(body.len() - 36).is_multiple_of(3) {
        return Err(invalid("Error with SIZ marker size\n"));
    }
    header.rsiz = be16(body);
    header.x1 = be32(&body[2..]);
    header.y1 = be32(&body[6..]);
    header.x0 = be32(&body[10..]);
    header.y0 = be32(&body[14..]);
    header.tdx = be32(&body[18..]);
    header.tdy = be32(&body[22..]);
    header.tx0 = be32(&body[26..]);
    header.ty0 = be32(&body[30..]);
    let csiz = be16(&body[34..]);
    let limits = settings.limits;
    let (components, _) = body[36..].as_chunks::<3>();
    let count = usize::from(csiz);
    if csiz == 0 || csiz > MAX_COMPONENTS {
        return Err(invalid(format!(
            "Error with SIZ marker: number of component is illegal -> {csiz}\n"
        )));
    }
    if count != components.len() {
        return Err(invalid(format!(
            "Error with SIZ marker: number of component is not compatible with the remaining number of parameters ( {csiz} vs {})\n",
            components.len(),
        )));
    }
    if limits.max_components != 0 && u32::from(csiz) > limits.max_components {
        return Err(limit(format!(
            "Error with SIZ marker: {csiz} components exceed the decoder's limit of {}\n",
            limits.max_components,
        )));
    }
    let width = i64::from(header.x1) - i64::from(header.x0);
    let height = i64::from(header.y1) - i64::from(header.y0);
    if width <= 0 || height <= 0 {
        return Err(invalid(format!(
            "Error with SIZ marker: negative or zero image size ({width} x {height})\n"
        )));
    }
    if header.tdx == 0 || header.tdy == 0 {
        return Err(invalid(format!(
            "Error with SIZ marker: invalid tile size (tdx: {}, tdy: {})\n",
            header.tdx, header.tdy,
        )));
    }
    if header.tx0 > header.x0
        || header.ty0 > header.y0
        || u64::from(header.tx0) + u64::from(header.tdx) <= u64::from(header.x0)
        || u64::from(header.ty0) + u64::from(header.tdy) <= u64::from(header.y0)
    {
        return Err(invalid("Error with SIZ marker: illegal tile offset\n"));
    }
    let HeaderSettings {
        ihdr_width,
        ihdr_height,
        allow_varying_depth,
        ..
    } = settings;
    if ihdr_width > 0 && ihdr_height > 0 && (i64::from(ihdr_width) != width || i64::from(ihdr_height) != height) {
        return Err(invalid(format!(
            "Error with SIZ marker: IHDR w({ihdr_width}) h({ihdr_height}) vs. SIZ w({width}) h({height})\n",
        )));
    }
    let decode_component = |c: &[u8; 3]| SizComponent {
        prec: (c[0] & 0x7f) + 1,
        sgnd: c[0] >> 7,
        dx: c[1],
        dy: c[2],
    };
    let first = decode_component(&components[0]);
    for (index, bytes) in components.iter().enumerate() {
        let SizComponent { prec, sgnd, dx, dy } = decode_component(bytes);
        if !allow_varying_depth && bytes[0] != components[0][0] {
            diagnostics.warn_fmt(format_args!(
                "Despite JP2 BPC!=255, precision and/or sgnd values for comp[{index}] is different than comp[0]:\n        [0] prec({}) sgnd({}) [{index}] prec({prec}) sgnd({sgnd})\n",
                first.prec, first.sgnd,
            ));
        }
        if dx == 0 || dy == 0 {
            return Err(invalid(format!(
                "Invalid values for comp = {index} : dx={dx} dy={dy} (should be between 1 and 255 according to the JPEG2000 norm)\n",
            )));
        }
        if prec > 31 {
            return Err(invalid(format!(
                "Invalid values for comp = {index} : prec={prec} (should be between 1 and 38 according to the JPEG2000 norm. OpenJpeg only supports up to 31)\n",
            )));
        }
    }
    let tw = u64::from(header.x1 - header.tx0).div_ceil(u64::from(header.tdx));
    let th = u64::from(header.y1 - header.ty0).div_ceil(u64::from(header.tdy));
    if tw == 0 || th == 0 || tw > MAX_TILES / th {
        return Err(invalid(format!(
            "Invalid number of tiles : {tw} x {th} (maximum fixed by jpeg2000 norm is 65535 tiles)\n",
        )));
    }
    header.tw = u32::try_from(tw).expect("tile count was checked");
    header.th = u32::try_from(th).expect("tile count was checked");
    let tiles = header.tile_count();
    if limits.max_tiles != 0 && tiles > limits.max_tiles {
        return Err(limit(format!(
            "Error with SIZ marker: {tiles} tiles exceed the decoder's limit of {}\n",
            limits.max_tiles,
        )));
    }
    header.numcomps = u32::from(csiz);
    ledger.array(CType::SizComponent, count)?;
    // The C++ oracle's two override tables are arrays of eight-byte pointers.
    ledger.array(CType::U64, count)?;
    ledger.array(CType::U64, count)?;
    ledger.array(CType::I16, count)?;
    header.comps.try_reserve_exact(count).map_err(|_| no_memory())?;
    header.coc.indices.try_reserve_exact(count).map_err(|_| no_memory())?;
    header.qcc.indices.try_reserve_exact(count).map_err(|_| no_memory())?;
    header.rgn.try_reserve_exact(count).map_err(|_| no_memory())?;
    header.comps.extend(components.iter().map(decode_component));
    header.coc.indices.resize(count, None);
    header.qcc.indices.resize(count, None);
    header.rgn.resize(count, None);
    Ok(())
}

fn read_rgn(body: &[u8], numcomps: u32, diagnostics: &mut Diagnostics) -> Result<(u16, u8), Error> {
    let room = component_room(numcomps);
    if body.len() != room + 2 {
        return Err(invalid("Error reading RGN marker\n"));
    }
    let comp = read_component(body, room);
    if u32::from(comp) >= numcomps {
        return Err(invalid(format!(
            "bad component number in RGN ({comp} when there are only {numcomps})\n"
        )));
    }
    if body[room] != 0 {
        diagnostics.warn_fmt(format_args!(
            "RGN marker: region of interest style {} read as implicit\n",
            body[room]
        ));
    }
    Ok((comp, body[room + 1]))
}

fn read_poc(body: &[u8], header: &mut MainHeader, ledger: &mut HeaderLedger) -> Result<(), Error> {
    let room = component_room(header.numcomps);
    let entry_size = 5 + 2 * room;
    if body.is_empty() || !body.len().is_multiple_of(entry_size) {
        return Err(invalid("Error reading POC marker\n"));
    }
    let entries = header.poc.len() + body.len() / entry_size;
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
        push_list(&mut header.poc, CType::PocEntry, entry, ledger)?;
    }
    Ok(())
}

fn read_tlm(
    body: &[u8],
    header: &mut MainHeader,
    ledger: &mut HeaderLedger,
    diagnostics: &mut Diagnostics,
) -> Result<(), Error> {
    if body.len() < 2 {
        return Err(invalid("Error reading TLM marker.\n"));
    }
    if header.tlm_invalid {
        return Ok(());
    }
    let tile_size = usize::from((body[1] >> 4) & 3);
    let length_size = (usize::from((body[1] >> 6) & 1) + 1) * 2;
    if tile_size == 3 {
        header.tlm_invalid = true;
        diagnostics.warn("opj_j2k_read_tlm(): ST = 3 is invalid.\n");
        return Ok(());
    }
    let entry_size = tile_size + length_size;
    if !(body.len() - 2).is_multiple_of(entry_size) {
        header.tlm_invalid = true;
        diagnostics.warn("opj_j2k_read_tlm(): TLM marker not of expected size.\n");
        return Ok(());
    }
    for entry in body[2..].chunks_exact(entry_size) {
        let tile = if tile_size == 0 {
            header.tlm.len()
        } else {
            usize::from(read_component(entry, tile_size))
        };
        if tile >= header.tile_count() as usize {
            header.tlm_invalid = true;
            diagnostics.warn_fmt(format_args!("opj_j2k_read_tlm(): invalid tile number {tile}\n"));
            return Ok(());
        }
        let tile = u16::try_from(tile).expect("tile index was checked");
        let length = if length_size == 2 {
            u32::from(be16(&entry[tile_size..]))
        } else {
            be32(&entry[tile_size..])
        };
        push_list(&mut header.tlm, CType::TlmEntry, TlmEntry { tile, length }, ledger)?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct PackedChunk<'a> {
    index: u8,
    data: &'a [u8],
}

fn add_ppm<'a>(chunks: &mut Vec<PackedChunk<'a>>, body: &'a [u8], ledger: &mut HeaderLedger) -> Result<(), Error> {
    if body.len() < 2 {
        return Err(invalid("Error reading PPM marker\n"));
    }
    let index = body[0];
    if chunks.iter().any(|chunk| chunk.index == index) {
        return Err(invalid(format!("Zppm {index} already read\n")));
    }
    push_list(
        chunks,
        CType::PackedChunk,
        PackedChunk {
            index,
            data: &body[1..],
        },
        ledger,
    )
}

/// Visit lengths and data in separate events. Nppm fields must fit in a single marker, but the
/// packet-header bytes they count can continue into the next marker.
fn walk_ppm(
    chunks: &[PackedChunk<'_>],
    mut length: impl FnMut(u32) -> Result<(), Error>,
    mut data: impl FnMut(&[u8]),
) -> Result<(), Error> {
    let mut remaining = 0;
    for chunk in chunks {
        let carry = remaining.min(chunk.data.len());
        data(&chunk.data[..carry]);
        remaining -= carry;
        let mut bytes = &chunk.data[carry..];
        while !bytes.is_empty() {
            if bytes.len() < 4 {
                return Err(invalid("Not enough bytes to read Nppm\n"));
            }
            let count = be32(bytes);
            length(count)?;
            let available = &bytes[4..];
            let take = (count as usize).min(available.len());
            data(&available[..take]);
            bytes = &available[take..];
            remaining = count as usize - take;
        }
    }
    if remaining != 0 {
        return Err(invalid("Corrupted PPM markers\n"));
    }
    Ok(())
}

fn merge_ppm(chunks: &mut [PackedChunk<'_>], header: &mut MainHeader, ledger: &mut HeaderLedger) -> Result<(), Error> {
    chunks.sort_unstable_by_key(|chunk| chunk.index);
    let mut total = 0u32;
    walk_ppm(
        chunks,
        |length| {
            total = total
                .checked_add(length)
                .ok_or_else(|| invalid("Too large value for Nppm\n"))?;
            push_list(&mut header.ppm_lengths, CType::U32, length, ledger)
        },
        |_| {},
    )?;
    if total == 0 {
        return Err(invalid("Corrupted PPM markers: the packet headers hold no bytes\n"));
    }
    let total = total as usize;
    ledger.charge(total, 1)?;
    header.ppm_data.try_reserve_exact(total).map_err(|_| no_memory())?;
    walk_ppm(chunks, |_| Ok(()), |bytes| header.ppm_data.extend_from_slice(bytes))
}

pub(crate) fn parse_main_header(
    data: &[u8],
    start: usize,
    settings: HeaderSettings,
    ledger: &mut HeaderLedger,
    diagnostics: &mut Diagnostics,
) -> Result<MainHeader, Error> {
    let data = data.get(start..).unwrap_or_default();
    let mut bytes = match data {
        [] => return Err(need_data("the main header (no bytes)")),
        [0xff] => return Err(need_data("the SOC marker")),
        [0xff, 0x4f, rest @ ..] => rest,
        _ => return Err(Error::new(ErrorKind::NotJpeg2000, "Expected a SOC marker \n")),
    };
    ledger.array(CType::MainHeader, 1)?;
    let mut header = MainHeader {
        start: start as u64,
        min_resolutions: u32::try_from(coding::MAX_RESOLUTIONS).expect("MAX_RESOLUTIONS fits in u32"),
        ..MainHeader::default()
    };
    let mut packed = Vec::new();
    let mut has_cod = false;
    let mut has_qcd = false;
    loop {
        let at = start + data.len() - bytes.len();
        let (marker, rest) = bytes.split_at_checked(2).ok_or_else(|| need_data("the main header"))?;
        let marker = be16(marker);
        bytes = rest;
        if marker < 0xff00 {
            return Err(invalid(format!(
                "A marker ID was expected (0xff--) instead of {marker:08x}\n"
            )));
        }
        if header.numcomps == 0 && marker != SIZ {
            return Err(invalid(format!(
                "Expected a SIZ marker after SOC, found 0x{marker:04X}\n"
            )));
        }
        match marker {
            SOT => {
                header.end = at as u64;
                break;
            }
            0xff30..=0xff3f => {
                diagnostics.warn_fmt(format_args!(
                    "Marker 0x{marker:04X} without a segment skipped in the main header\n"
                ));
                continue;
            }
            SOC | SOD | EOC | EPH => {
                return Err(invalid(format!(
                    "Marker 0x{marker:04X} is not compliant with its position (main header)\n"
                )));
            }
            _ => {}
        }
        let (length, rest) = bytes.split_at_checked(2).ok_or_else(|| need_data("the main header"))?;
        let length = be16(length);
        if length < 2 {
            return Err(invalid("Invalid marker size\n"));
        }
        if (0xff70..=0xff7f).contains(&marker) {
            return Err(Error::new(
                ErrorKind::Unsupported,
                format!("Marker 0x{marker:04X} is a JPEG 2000 Part 2 extension, not supported\n"),
            ));
        }
        let (body, rest) = rest
            .split_at_checked(usize::from(length - 2))
            .ok_or_else(|| need_data("the main header"))?;
        bytes = rest;
        if !body.is_empty() {
            ledger.charge(body.len(), 1)?;
        }
        match marker {
            SIZ => {
                if header.numcomps != 0 {
                    return Err(invalid("A second SIZ marker in the main header\n"));
                }
                read_siz(body, settings, &mut header, ledger, diagnostics)?;
            }
            COD => {
                (header.cod, header.cod_style) = coding::read_cod(body, settings.limits, diagnostics)?;
                has_cod = true;
            }
            COC => {
                let (comp, style) = coding::read_coc(body, header.numcomps, diagnostics)?;
                ledger.array(CType::CodingStyle, 1)?;
                header.coc.set(usize::from(comp), style)?;
            }
            QCD => {
                header.qcd = coding::read_sqcd(body, "QCD", diagnostics)?;
                has_qcd = true;
            }
            QCC => {
                ledger.array(CType::Quantization, 1)?;
                let (comp, quant) = coding::read_qcc(body, header.numcomps, diagnostics)?;
                header.qcc.set(usize::from(comp), quant)?;
            }
            RGN => {
                let (comp, shift) = read_rgn(body, header.numcomps, diagnostics)?;
                header.rgn[usize::from(comp)] = Some(shift);
            }
            POC => read_poc(body, &mut header, ledger)?,
            PPM => add_ppm(&mut packed, body, ledger)?,
            TLM => read_tlm(body, &mut header, ledger, diagnostics)?,
            PLM if body.is_empty() => return Err(invalid("Error reading PLM marker\n")),
            CRG if body.len() != header.numcomps as usize * 4 => {
                return Err(invalid("Error reading CRG marker\n"));
            }
            CAP => header.has_cap = true,
            COM | CPF | PLM | CRG => {}
            PLT | PPT | SOP => return Err(invalid("Marker is not compliant with its position\n")),
            _ => diagnostics.warn_fmt(format_args!(
                "Unknown marker 0x{marker:04X} ({} bytes) skipped in the main header\n",
                body.len()
            )),
        }
    }
    if !has_cod {
        return Err(invalid("required COD marker not found in main header\n"));
    }
    if !has_qcd {
        return Err(invalid("required QCD marker not found in main header\n"));
    }
    if !packed.is_empty() {
        merge_ppm(&mut packed, &mut header, ledger)
            .map_err(|error| wrap_error(error, "Failed to merge PPM data\n", diagnostics))?;
    }
    for comp in 0..header.numcomps {
        let resolutions = u32::from(header.main_style(comp).numresolutions);
        let quant = header.main_quant(comp);
        let bands = 3 * resolutions - 2;
        if quant.qntsty != coding::QNT_DERIVED && u32::from(quant.nbands) < bands {
            return Err(invalid(format!(
                "Component {comp}: the quantization gives {} step sizes for {bands} sub-bands\n",
                quant.nbands
            )));
        }
        header.min_resolutions = header.min_resolutions.min(resolutions);
    }
    Ok(header)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_marker(data: &mut Vec<u8>, marker: u16, body: &[u8]) {
        data.extend_from_slice(&marker.to_be_bytes());
        data.extend_from_slice(&u16::try_from(body.len() + 2).unwrap().to_be_bytes());
        data.extend_from_slice(body);
    }

    fn first_markers() -> Vec<u8> {
        let mut data = SOC.to_be_bytes().to_vec();
        let mut siz = vec![0, 0];
        for value in [7u32, 9, 0, 0, 7, 9, 0, 0] {
            siz.extend_from_slice(&value.to_be_bytes());
        }
        siz.extend_from_slice(&[0, 1, 7, 1, 1]);
        write_marker(&mut data, SIZ, &siz);
        data
    }

    fn parse(data: &[u8]) -> Result<MainHeader, Error> {
        parse_main_header(
            data,
            0,
            HeaderSettings {
                limits: Limits::default(),
                ihdr_width: 0,
                ihdr_height: 0,
                allow_varying_depth: true,
            },
            &mut HeaderLedger::default(),
            &mut Diagnostics::default(),
        )
    }

    #[test]
    fn component_override_precedes_a_later_default() {
        let mut data = first_markers();
        write_marker(&mut data, COC, &[0, 0, 0, 4, 4, 0, 1]);
        write_marker(&mut data, QCC, &[0, 0x40, 0x50]);
        write_marker(&mut data, COD, &[0, 0, 0, 1, 0, 1, 4, 4, 0, 1]);
        write_marker(&mut data, QCD, &[0x40]);
        data.extend_from_slice(&SOT.to_be_bytes());
        let header = parse(&data).unwrap();
        assert_eq!(header.min_resolutions, 1);
        assert_eq!(header.main_quant(0).step(0), (10, 0));
        assert_eq!(header.end, (data.len() - 2) as u64);
    }

    #[test]
    fn ppm_lengths_and_bytes_have_distinct_marker_boundary_rules() {
        let mut data = first_markers();
        write_marker(&mut data, COD, &[0, 0, 0, 1, 0, 0, 4, 4, 0, 1]);
        write_marker(&mut data, QCD, &[0, 0]);
        write_marker(&mut data, PPM, &[1, 2, 3]);
        write_marker(&mut data, PPM, &[0, 0, 0, 0, 3, 1]);
        let mut split_length = data.clone();
        write_marker(&mut split_length, PPM, &[2, 0, 0]);
        data.extend_from_slice(&SOT.to_be_bytes());
        split_length.extend_from_slice(&SOT.to_be_bytes());
        let header = parse(&data).unwrap();
        assert_eq!(header.ppm_lengths, [3]);
        assert_eq!(header.ppm_data, [1, 2, 3]);
        let error = parse(&split_length).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Header);
        assert_eq!(error.message(), "Failed to merge PPM data\n");
    }

    #[test]
    fn partial_and_forbidden_marker_segments_have_distinct_statuses() {
        let mut prefix = first_markers();
        prefix.extend_from_slice(&[0xff, 0x64, 0, 10, 0]);
        assert_eq!(parse(&prefix).unwrap_err().kind(), ErrorKind::NeedData);
        let mut part_two = first_markers();
        part_two.extend_from_slice(&[0xff, 0x70, 0, 10, 0]);
        assert_eq!(parse(&part_two).unwrap_err().kind(), ErrorKind::Unsupported);
    }
}
