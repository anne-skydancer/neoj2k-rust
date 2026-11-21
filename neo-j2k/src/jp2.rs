// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The box reader and diagnostics derive from OpenJPEG's jp2.c through src/jp2.cpp.
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
// Copyright (c) 2010-2011, Kaori Hagihara
// Copyright (c) 2008, 2011-2012, Centre National d'Etudes Spatiales (CNES), FR
// Copyright (c) 2012, CS Systemes d'Information, France
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
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

//! JP2 and JPH container inspection, palette expansion and channel definitions.

use core::ops::Range;

use crate::error::Diagnostics;
use crate::mem::{CType, HeaderLedger};
use crate::{DecodedComponent, Error, ErrorKind, Limits};

pub(crate) const MAGIC: [u8; 12] = *b"\x00\x00\x00\x0cjP  \x0d\x0a\x87\x0a";

const SIGNATURE: [u8; 4] = *b"jP  ";
const FILE_TYPE: [u8; 4] = *b"ftyp";
const JP2_HEADER: [u8; 4] = *b"jp2h";
const CODESTREAM: [u8; 4] = *b"jp2c";
const IMAGE_HEADER: [u8; 4] = *b"ihdr";
const BITS_PER_COMPONENT: [u8; 4] = *b"bpcc";
const COLOUR: [u8; 4] = *b"colr";
const PALETTE: [u8; 4] = *b"pclr";
const MAPPING: [u8; 4] = *b"cmap";
const CHANNELS: [u8; 4] = *b"cdef";

/// Ranges are relative to the beginning of the JP2 file and remain valid for the same header prefix.
#[derive(Debug, Default)]
pub(crate) struct Jp2Header {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) bpc: u8,
    pub(crate) codestream_start: usize,
    pub(crate) codestream_len: Option<u64>,
    pub(crate) colour: Option<ColourSpec>,
    components: u16,
    bits_per_component: Option<Range<usize>>,
    palette: Option<Palette>,
    mapping: Option<Range<usize>>,
    channels: Option<Range<usize>>,
}

#[derive(Debug)]
pub(crate) enum ColourSpec {
    Enumerated { code: u32, cielab: Option<[u32; 9]> },
    Icc(Range<usize>),
}

#[derive(Debug)]
struct Palette {
    columns: usize,
    entries: usize,
    bits: Range<usize>,
    values: Range<usize>,
}

impl Jp2Header {
    /// The values of the C++ codec's colour-space enum.
    pub(crate) fn color_space(&self) -> i32 {
        match self.colour {
            Some(ColourSpec::Enumerated { code: 16, .. }) => 1,
            Some(ColourSpec::Enumerated { code: 17, .. }) => 2,
            Some(ColourSpec::Enumerated { code: 18, .. }) => 3,
            Some(ColourSpec::Enumerated { code: 24, .. }) => 4,
            Some(ColourSpec::Enumerated { code: 12, .. }) => 5,
            _ => -1,
        }
    }

    pub(crate) const fn changes_channels(&self) -> bool {
        (self.palette.is_some() && self.mapping.is_some()) || self.channels.is_some()
    }
}

fn be16(data: &[u8], at: usize) -> u16 {
    u16::from_be_bytes([data[at], data[at + 1]])
}

fn be32(data: &[u8], at: usize) -> u32 {
    u32::from_be_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]])
}

fn box_type(data: &[u8]) -> [u8; 4] {
    [data[4], data[5], data[6], data[7]]
}

fn is_image_box(kind: [u8; 4]) -> bool {
    matches!(
        kind,
        IMAGE_HEADER | BITS_PER_COMPONENT | COLOUR | PALETTE | MAPPING | CHANNELS
    )
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Header, message)
}

fn need_data() -> Error {
    Error::new(
        ErrorKind::NeedData,
        "Stream too short: the bytes end before the JP2 codestream box\n",
    )
}

fn arena_failure(error: Error) -> Error {
    let message = if error.kind() == ErrorKind::Limit {
        "The JP2 boxes need more memory than the decoder's limit\n"
    } else {
        "Not enough memory to read the JP2 boxes\n"
    };
    Error::new(error.kind(), message)
}

/// Read the boxes before the first codestream without copying their payloads. The ledger still
/// charges each of the C++ reader's allocations in order so memory limits give the same result.
pub(crate) fn read_boxes(
    data: &[u8],
    ledger: &mut HeaderLedger,
    diagnostics: &mut Diagnostics,
) -> Result<Jp2Header, Error> {
    ledger.array(CType::Jp2Header, 1).map_err(arena_failure)?;
    let mut reader = Reader {
        header: Jp2Header::default(),
        ledger,
        diagnostics,
    };
    let mut signature = false;
    let mut file_type = false;
    let mut header = false;
    let mut start = 0;
    loop {
        let tail = &data[start..];
        if !signature && !MAGIC.starts_with(&tail[..tail.len().min(8)]) {
            return Err(Error::new(
                ErrorKind::NotJpeg2000,
                "Malformed JP2 file format: first box must be JPEG 2000 signature box\n",
            ));
        }
        let prefix = tail.get(..8).ok_or_else(need_data)?;
        let mut length = be32(prefix, 0);
        let kind = box_type(prefix);
        let mut prefix_len = 8;
        if length == 1 {
            let extended = tail.get(..16).ok_or_else(need_data)?;
            prefix_len = 16;
            if be32(extended, 8) != 0 {
                if !header {
                    return Err(invalid("Cannot handle box sizes higher than 2^32\n"));
                }
                reader.diagnostics.warn("Cannot handle box sizes higher than 2^32\n");
                reader.header.codestream_start = start + prefix_len;
                return Ok(reader.header);
            }
            length = be32(extended, 12);
        }
        if kind == CODESTREAM {
            if !header {
                return Err(invalid("bad placed jpeg codestream\n"));
            }
            reader.header.codestream_start = start + prefix_len;
            reader.header.codestream_len = u64::from(length)
                .checked_sub(u64::try_from(prefix_len).expect("box header length fits in u64"))
                .filter(|&size| size != 0);
            return Ok(reader.header);
        }
        if length == 0 {
            return Err(invalid("Cannot handle box of undefined sizes\n"));
        }
        let size = usize::try_from(length).map_err(|_| need_data())?;
        if size < prefix_len {
            return Err(invalid(format!(
                "invalid box size {length} ({:x})\n",
                u32::from_be_bytes(kind)
            )));
        }
        let image_box = is_image_box(kind);
        if image_box {
            let name = core::str::from_utf8(&kind).expect("known box types are ASCII");
            reader
                .diagnostics
                .warn(format!("Found a misplaced '{name}' box outside jp2h box\n"));
            if !header {
                reader.diagnostics.warn(format!(
                    "JPEG2000 Header box not read yet, '{name}' box will be ignored\n"
                ));
            }
        }
        let handled = matches!(kind, SIGNATURE | FILE_TYPE | JP2_HEADER) || (image_box && header);
        if !handled && !file_type {
            return Err(invalid("Malformed JP2 file format: second box must be file type box\n"));
        }
        let body = tail.get(prefix_len..size).ok_or_else(need_data)?;
        let body_start = start + prefix_len;
        start += size;
        if !handled {
            continue;
        }
        if !body.is_empty() {
            reader.ledger.charge(body.len(), 4).map_err(arena_failure)?;
        }
        match kind {
            SIGNATURE => {
                if signature {
                    return Err(invalid("The signature box must be the first box in the file.\n"));
                }
                if body.len() != 4 {
                    return Err(invalid("Error with JP signature Box size\n"));
                }
                if body != &MAGIC[8..] {
                    return Err(Error::new(
                        ErrorKind::NotJpeg2000,
                        "Error with JP Signature : bad magic number\n",
                    ));
                }
                signature = true;
            }
            FILE_TYPE => {
                if file_type {
                    return Err(invalid("The ftyp box must be the second box in the file.\n"));
                }
                if body.len() < 8 || !(body.len() - 8).is_multiple_of(4) {
                    return Err(invalid("Error with FTYP signature Box size\n"));
                }
                // JP2 and JPH use the same reader. The brand does not constrain the codestream.
                file_type = true;
            }
            JP2_HEADER => {
                if !file_type {
                    return Err(invalid("The  box must be the first box in the file.\n"));
                }
                reader.header_box(body, body_start)?;
                header = true;
            }
            _ => reader.image_box(kind, body, body_start)?,
        }
    }
}

struct Reader<'a> {
    header: Jp2Header,
    ledger: &'a mut HeaderLedger,
    diagnostics: &'a mut Diagnostics,
}

impl Reader<'_> {
    fn header_box(&mut self, mut data: &[u8], mut start: usize) -> Result<(), Error> {
        let mut saw_ihdr = false;
        while !data.is_empty() {
            if data.len() < 8 {
                return Err(invalid("Cannot handle box of less than 8 bytes\n"));
            }
            let kind = box_type(data);
            let mut length = be32(data, 0);
            let mut prefix_len = 8;
            if length == 1 {
                if data.len() < 16 {
                    return Err(invalid("Cannot handle XL box of less than 16 bytes\n"));
                }
                if be32(data, 8) != 0 {
                    return Err(invalid("Cannot handle box sizes higher than 2^32\n"));
                }
                length = be32(data, 12);
                prefix_len = 16;
            }
            if length == 0 {
                return Err(invalid("Cannot handle box of undefined sizes\n"));
            }
            let inconsistent = || invalid("Stream error while reading JP2 Header box: box length is inconsistent.\n");
            let length = usize::try_from(length).map_err(|_| inconsistent())?;
            if length < prefix_len {
                return Err(invalid("Box length is inconsistent.\n"));
            }
            let body = data.get(prefix_len..length).ok_or_else(inconsistent)?;
            self.image_box(kind, body, start + prefix_len)?;
            saw_ihdr |= kind == IMAGE_HEADER;
            data = &data[length..];
            start += length;
        }
        if !saw_ihdr {
            return Err(invalid("Stream error while reading JP2 Header box: no 'ihdr' box.\n"));
        }
        Ok(())
    }

    fn image_box(&mut self, kind: [u8; 4], data: &[u8], start: usize) -> Result<(), Error> {
        match kind {
            IMAGE_HEADER => self.image_header(data),
            BITS_PER_COMPONENT => self.bits_per_component(data, start),
            COLOUR => self.colour(data, start),
            PALETTE => self.palette(data, start),
            MAPPING => self.mapping(data, start),
            CHANNELS => self.channels(data, start),
            _ => Ok(()),
        }
    }

    fn image_header(&mut self, data: &[u8]) -> Result<(), Error> {
        if self.header.components != 0 {
            self.diagnostics
                .warn("Ignoring ihdr box. First ihdr box already read\n");
            return Ok(());
        }
        if data.len() != 14 {
            return Err(invalid("Bad image header box (bad size)\n"));
        }
        let height = be32(data, 0);
        let width = be32(data, 4);
        let components = be16(data, 8);
        if height == 0 || width == 0 || components == 0 {
            return Err(invalid(format!(
                "Wrong values for: w({width}) h({height}) numcomps({components}) (ihdr)\n"
            )));
        }
        if components > 16384 {
            return Err(invalid("Invalid number of components (ihdr)\n"));
        }
        self.header.height = height;
        self.header.width = width;
        self.header.components = components;
        self.header.bpc = data[10];
        let compression = data[11];
        if compression != 7 {
            self.diagnostics.info(format!(
                "JP2 IHDR box: compression type indicate that the file is not a conforming JP2 file ({compression}) \n"
            ));
        }
        Ok(())
    }

    fn bits_per_component(&mut self, data: &[u8], start: usize) -> Result<(), Error> {
        if self.header.bpc != 255 {
            self.diagnostics.warn(format!(
                "A BPCC header box is available although BPC given by the IHDR box ({}) indicate components bit depth is constant\n",
                self.header.bpc
            ));
        }
        if data.len() != usize::from(self.header.components) {
            return Err(invalid("Bad BPCC header box (bad size)\n"));
        }
        self.header.bits_per_component = (!data.is_empty()).then_some(start..start + data.len());
        Ok(())
    }

    fn colour(&mut self, data: &[u8], start: usize) -> Result<(), Error> {
        let len = data.len();
        if len < 3 {
            return Err(invalid("Bad COLR header box (bad size)\n"));
        }
        if self.header.colour.is_some() {
            self.diagnostics.info(
                "A conforming JP2 reader shall ignore all Colour Specification boxes after the first, so we ignore this one.\n",
            );
            return Ok(());
        }
        let spec = match data[0] {
            1 => {
                if len < 7 {
                    return Err(invalid(format!("Bad COLR header box (bad size: {len})\n")));
                }
                let code = be32(data, 3);
                let cielab = if code == 14 {
                    let mut values = [14, 0x4445_4600, 0, 0, 0, 0, 0, 0, 0x0044_3530];
                    if len == 35 {
                        values[1] = 0;
                        for (i, value) in values[2..].iter_mut().enumerate() {
                            *value = be32(data, 7 + i * 4);
                        }
                    } else if len != 7 {
                        self.diagnostics
                            .warn(format!("Bad COLR header box (CIELab, bad size: {len})\n"));
                    }
                    Some(values)
                } else {
                    if len > 7 {
                        self.diagnostics
                            .warn(format!("Bad COLR header box (bad size: {len})\n"));
                    }
                    None
                };
                ColourSpec::Enumerated { code, cielab }
            }
            2 => {
                if len == 3 {
                    return Err(invalid("Bad COLR header box (empty ICC profile)\n"));
                }
                ColourSpec::Icc(start + 3..start + len)
            }
            method => {
                self.diagnostics.info(format!(
                    "COLR BOX meth value is not a regular value ({method}), so we will ignore the entire Colour Specification box. \n"
                ));
                return Ok(());
            }
        };
        self.header.colour = Some(spec);
        Ok(())
    }

    fn palette(&mut self, data: &[u8], start: usize) -> Result<(), Error> {
        if self.header.palette.is_some() {
            return Err(invalid("Only one PCLR box is allowed.\n"));
        }
        let bad_size = || invalid("Bad PCLR box (bad size)\n");
        if data.len() < 3 {
            return Err(bad_size());
        }
        let entries = usize::from(be16(data, 0));
        let columns = usize::from(data[2]);
        if entries == 0 || entries > 1024 {
            return Err(invalid(format!("Invalid PCLR box. Reports {entries} entries\n")));
        }
        if columns == 0 {
            return Err(invalid("Invalid PCLR box. Reports 0 palette columns\n"));
        }
        let bits_end = 3 + columns;
        let bits = data.get(3..bits_end).ok_or_else(bad_size)?;
        let entry_bytes: usize = bits
            .iter()
            .map(|byte| (usize::from(byte & 0x7f) + 1).div_ceil(8).min(4))
            .sum();
        let values_end = bits_end + entries * entry_bytes;
        if data.len() < values_end {
            return Err(bad_size());
        }
        self.ledger
            .array(CType::U32, entries * columns)
            .map_err(arena_failure)?;
        self.header.palette = Some(Palette {
            columns,
            entries,
            bits: start + 3..start + bits_end,
            values: start + bits_end..start + values_end,
        });
        Ok(())
    }

    fn mapping(&mut self, data: &[u8], start: usize) -> Result<(), Error> {
        let Some(palette) = &self.header.palette else {
            return Err(invalid("Need to read a PCLR box before the CMAP box.\n"));
        };
        if self.header.mapping.is_some() {
            return Err(invalid("Only one CMAP box is allowed.\n"));
        }
        let len = palette.columns * 4;
        if data.len() < len {
            return Err(invalid("Insufficient data for CMAP box.\n"));
        }
        self.ledger
            .array(CType::Jp2CmapEntry, palette.columns)
            .map_err(arena_failure)?;
        self.header.mapping = Some(start..start + len);
        Ok(())
    }

    fn channels(&mut self, data: &[u8], start: usize) -> Result<(), Error> {
        if self.header.channels.is_some() {
            return Err(invalid("Only one CDEF box is allowed.\n"));
        }
        let short_box = || invalid("Insufficient data for CDEF box.\n");
        if data.len() < 2 {
            return Err(short_box());
        }
        let count = usize::from(be16(data, 0));
        if count == 0 {
            return Err(invalid("Number of channel description is equal to zero in CDEF box.\n"));
        }
        let len = 2 + count * 6;
        if data.len() < len {
            return Err(short_box());
        }
        self.ledger.array(CType::Jp2CdefEntry, count).map_err(arena_failure)?;
        self.header.channels = Some(start + 2..start + len);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Mapping {
    component: usize,
    method: u8,
    column: usize,
}

#[derive(Clone, Copy, Debug)]
struct Channel {
    number: usize,
    kind: u16,
    association: u16,
}

fn channel_defs(bytes: &[u8]) -> impl Iterator<Item = Channel> {
    bytes.as_chunks::<6>().0.iter().map(|def| Channel {
        number: usize::from(be16(def, 0)),
        kind: be16(def, 2),
        association: be16(def, 4),
    })
}

fn colour_error(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Data, message)
}

fn colour_bytes<'a>(data: &'a [u8], range: &Range<usize>) -> Result<&'a [u8], Error> {
    data.get(range.clone())
        .ok_or_else(|| colour_error("Incomplete JP2 colour data\n"))
}

fn validate_channels(defs: &[u8], channels: usize) -> Result<(), Error> {
    // Channel numbers are u16. Count distinct channels without rescanning the definitions for each one.
    let mut seen = [0_u64; 1024];
    let mut count = 0;
    for def in channel_defs(defs) {
        if def.number >= channels {
            return Err(colour_error(format!(
                "Invalid component index {} (>= {channels}).\n",
                def.number
            )));
        }
        if def.association != u16::MAX && def.association != 0 {
            let index = usize::from(def.association - 1);
            if index >= channels {
                return Err(colour_error(format!(
                    "Invalid component index {index} (>= {channels}).\n"
                )));
            }
        }
        let bit = 1 << (def.number % 64);
        let slot = &mut seen[def.number / 64];
        if *slot & bit == 0 {
            *slot |= bit;
            count += 1;
        }
    }
    if count != channels {
        return Err(colour_error("Incomplete channel definitions.\n"));
    }
    Ok(())
}

fn validate_mapping(mapping: &mut [Mapping], components: usize, diagnostics: &mut Diagnostics) -> Result<(), Error> {
    let mut used = [false; 256];
    let mut last_error = None;
    // The caller reports the returned error. Emit the earlier errors here to keep C++ order.
    let mut record = |message| {
        if let Some(previous) = last_error.replace(message) {
            diagnostics.error(previous);
        }
    };
    for entry in mapping.iter() {
        if entry.component >= components {
            record(format!(
                "Invalid component index {} (>= {components}).\n",
                entry.component
            ));
        }
    }
    for (index, &Mapping { method, column, .. }) in mapping.iter().enumerate() {
        if method > 1 {
            record(format!("Invalid value for cmap[{index}].mtyp = {method}.\n"));
        } else if column >= mapping.len() {
            record(format!(
                "Invalid component/palette index for direct mapping {column}.\n"
            ));
        } else if used[column] && method == 1 {
            record(format!("Component {column} is mapped twice.\n"));
        } else if method == 0 && column != 0 {
            record(format!("Direct use at #{index} however pcol={column}.\n"));
        } else if method == 1 && column != index {
            record(format!(
                "Implementation limitation: for palette mapping, pcol[{index}] should be equal to {index}, but is equal to {column}.\n"
            ));
        } else {
            used[column] = true;
        }
    }
    for (index, entry) in mapping.iter().enumerate() {
        if !used[index] && entry.method != 0 {
            record(format!("Component {index} doesn't have a mapping.\n"));
        }
    }
    if let Some(error) = last_error {
        return Err(colour_error(error));
    }
    if components == 1 && used[..mapping.len()].contains(&false) {
        diagnostics.warn("Component mapping seems wrong. Trying to correct.\n");
        for (column, entry) in mapping.iter_mut().enumerate() {
            entry.method = 1;
            entry.column = column;
        }
    }
    Ok(())
}

fn palette_memory_error() -> Error {
    colour_error("Memory allocation failure in opj_jp2_apply_pclr().\n")
}

fn expand_palette(
    palette: &Palette,
    mapping: &[Mapping],
    data: &[u8],
    image: &[DecodedComponent],
) -> Result<Vec<DecodedComponent>, Error> {
    for (index, entry) in mapping.iter().enumerate() {
        if image[entry.component].samples.is_empty() {
            return Err(colour_error(format!(
                "image->comps[{index}].data == NULL in opj_jp2_apply_pclr().\n"
            )));
        }
    }
    let bits = colour_bytes(data, &palette.bits)?;
    let values = colour_bytes(data, &palette.values)?;
    let mut offsets = [0; 256];
    for (column, &format) in bits.iter().enumerate() {
        offsets[column + 1] = offsets[column] + (usize::from(format & 0x7f) + 1).div_ceil(8).min(4);
    }
    let row_stride = offsets[bits.len()];
    let top = i32::try_from(palette.entries - 1).expect("palette has at most 1024 entries");

    let mut result = Vec::new();
    result
        .try_reserve_exact(mapping.len())
        .map_err(|_| palette_memory_error())?;
    for (index, entry) in mapping.iter().enumerate() {
        let source = &image[entry.component];
        let mut info = source.info;
        info.precision = u32::from(bits[index] & 0x7f) + 1;
        info.signed = bits[index] & 0x80 != 0;
        let mut samples = Vec::new();
        samples
            .try_reserve_exact(source.samples.len())
            .map_err(|_| palette_memory_error())?;
        if entry.method == 0 {
            samples.extend_from_slice(&source.samples);
        } else {
            let left = offsets[entry.column];
            let right = offsets[entry.column + 1];
            for &sample in &source.samples {
                let row = usize::try_from(sample.clamp(0, top)).expect("clamped palette index is nonnegative");
                let base = row * row_stride;
                // OpenJPEG reads at most four bytes per entry as an unsigned word and interprets
                // those bits as i32. Short signed columns retain their raw value.
                let value = values[base + left..base + right]
                    .iter()
                    .fold(0_u32, |acc, &byte| (acc << 8) | u32::from(byte));
                samples.push(value.cast_signed());
            }
        }
        result.push(DecodedComponent {
            info,
            samples,
            resolution: source.resolution,
            alpha: source.alpha,
        });
    }
    Ok(result)
}

fn assign_channels(defs: &[u8], image: &mut [DecodedComponent]) -> Result<(), Error> {
    let mut positions = Vec::new();
    positions
        .try_reserve_exact(image.len())
        .map_err(|_| Error::new(ErrorKind::NoMemory, "Not enough memory for the channel definitions\n"))?;
    // Keep each original channel's current position and each position's original channel.
    // This follows the swaps without rewriting every later definition.
    positions.extend((0..image.len()).map(|index| (index, index)));
    for def in channel_defs(defs) {
        let number = positions[def.number].0;
        if def.association != 0 && def.association != u16::MAX && def.kind == 0 {
            let other = usize::from(def.association - 1);
            if number != other {
                image.swap(number, other);
                let here_origin = positions[number].1;
                let other_origin = positions[other].1;
                positions[here_origin].0 = other;
                positions[other_origin].0 = number;
                positions[number].1 = other_origin;
                positions[other].1 = here_origin;
            }
        }
        image[number].alpha = def.kind;
    }
    Ok(())
}

/// Apply the transformations used by the C++ stream API's JP2 postprocessing. The image is
/// in original component order and `data` contains the same container header prefix as `header`.
pub(crate) fn apply_colour(
    header: &Jp2Header,
    data: &[u8],
    image: &mut Vec<DecodedComponent>,
    limits: Limits,
    diagnostics: &mut Diagnostics,
) -> Result<(), Error> {
    let palette = header.palette.as_ref().zip(header.mapping.as_ref());
    let defs = header
        .channels
        .as_ref()
        .map(|range| colour_bytes(data, range))
        .transpose()?;
    if let Some(defs) = defs {
        let channels = palette.map_or(image.len(), |(palette, _)| palette.columns);
        validate_channels(defs, channels)?;
    }
    if let Some((palette, cmap)) = palette {
        let bytes = colour_bytes(data, cmap)?;
        let mut entries = [Mapping::default(); 256];
        let mapping = &mut entries[..palette.columns];
        for (entry, bytes) in mapping.iter_mut().zip(bytes.as_chunks::<4>().0) {
            *entry = Mapping {
                component: usize::from(be16(bytes, 0)),
                method: bytes[2],
                column: usize::from(bytes[3]),
            };
        }
        validate_mapping(mapping, image.len(), diagnostics)?;
        if limits.max_components != 0 && u32::try_from(palette.columns).unwrap() > limits.max_components {
            return Err(Error::new(
                ErrorKind::Limit,
                format!(
                    "The palette (pclr box) makes {} components, more than the decoder's limit of {}\n",
                    palette.columns, limits.max_components
                ),
            ));
        }
        *image = expand_palette(palette, mapping, data, image)?;
    }
    if let Some(defs) = defs {
        assign_channels(defs, image)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ComponentInfo;
    use crate::error::Diagnostic;

    const IHDR: [u8; 14] = [0, 0, 0, 5, 0, 0, 0, 7, 0, 3, 7, 7, 0, 0];

    fn append_box(data: &mut Vec<u8>, kind: [u8; 4], contents: &[u8]) {
        let size = u32::try_from(8 + contents.len()).unwrap();
        data.extend(size.to_be_bytes());
        data.extend(kind);
        data.extend(contents);
    }

    fn prefix(extra_image_boxes: &[u8]) -> Vec<u8> {
        let mut data = MAGIC.to_vec();
        append_box(&mut data, FILE_TYPE, b"jph \0\0\0\0jph ");
        let mut jp2h = Vec::new();
        append_box(&mut jp2h, IMAGE_HEADER, &IHDR);
        jp2h.extend(extra_image_boxes);
        append_box(&mut data, JP2_HEADER, &jp2h);
        data
    }

    fn inspect(data: &[u8]) -> Result<Jp2Header, Error> {
        read_boxes(data, &mut HeaderLedger::default(), &mut Diagnostics::default())
    }

    #[test]
    fn accepts_a_complete_box_header_even_without_its_codestream() {
        let mut colour = Vec::new();
        append_box(&mut colour, COLOUR, &[2, 0, 0, 5, 6, 7]);
        let mut data = prefix(&colour);
        data.extend(40_u32.to_be_bytes());
        data.extend(CODESTREAM);
        for cut in 0..data.len() {
            assert_eq!(inspect(&data[..cut]).unwrap_err().kind(), ErrorKind::NeedData);
        }
        let header = inspect(&data).unwrap();
        assert_eq!((header.width, header.height, header.bpc), (7, 5, 7));
        assert_eq!(header.codestream_start, data.len());
        assert_eq!(header.codestream_len, Some(32));
        let Some(ColourSpec::Icc(profile)) = header.colour else {
            panic!("expected the ICC profile");
        };
        assert_eq!(&data[profile], &[5, 6, 7]);
    }

    #[test]
    fn zero_short_and_large_codestream_boxes_extend_to_the_end() {
        let mut data = prefix(&[]);
        let size_at = data.len();
        append_box(&mut data, CODESTREAM, &[]);
        for length in [0_u32, 2, 7, 8] {
            data[size_at..size_at + 4].copy_from_slice(&length.to_be_bytes());
            let header = inspect(&data).unwrap();
            assert_eq!(header.codestream_start, data.len());
            assert_eq!(header.codestream_len, None);
        }
        data[size_at..size_at + 4].copy_from_slice(&1_u32.to_be_bytes());
        data.extend((1_u64 << 32).to_be_bytes());
        let mut diagnostics = Diagnostics::default();
        let header = read_boxes(&data, &mut HeaderLedger::default(), &mut diagnostics).unwrap();
        assert_eq!(header.codestream_start, data.len());
        assert_eq!(header.codestream_len, None);
        assert_eq!(
            diagnostics.entries(),
            &[Diagnostic::Warning("Cannot handle box sizes higher than 2^32\n".into())]
        );
    }

    #[test]
    fn incomplete_children_inside_a_complete_box_are_invalid() {
        let data = prefix(&[0, 0, 0, 12, b'x', b'x', b'x', b'x']);
        let error = inspect(&data).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Header);
        assert_eq!(
            error.message(),
            "Stream error while reading JP2 Header box: box length is inconsistent.\n"
        );
        assert_eq!(
            inspect(&data[..data.len() - 1]).unwrap_err().kind(),
            ErrorKind::NeedData
        );
    }

    #[test]
    fn validates_palette_payload_before_charging_entries() {
        let mut extra = Vec::new();
        append_box(&mut extra, PALETTE, &[2, 0, 1, 7]);
        let mut ledger = HeaderLedger::default();
        ledger.reset(0, 2048);
        let error = read_boxes(&prefix(&extra), &mut ledger, &mut Diagnostics::default()).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Header);
        let mut palette = vec![2, 0, 1, 7];
        palette.resize(4 + 512, 0);
        extra.clear();
        append_box(&mut extra, PALETTE, &palette);
        ledger.reset(0, 2048);
        let error = read_boxes(&prefix(&extra), &mut ledger, &mut Diagnostics::default()).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Limit);
        assert_eq!(
            error.message(),
            "The JP2 boxes need more memory than the decoder's limit\n"
        );
    }

    fn plane(samples: &[i32]) -> Vec<DecodedComponent> {
        vec![DecodedComponent {
            info: ComponentInfo {
                width: u32::try_from(samples.len()).unwrap(),
                height: 1,
                precision: 8,
                ..ComponentInfo::default()
            },
            samples: samples.to_vec(),
            ..DecodedComponent::default()
        }]
    }

    #[test]
    fn palette_values_and_following_channel_definitions_match_cpp_order() {
        let mut boxes = Vec::new();
        append_box(
            &mut boxes,
            PALETTE,
            &[
                0, 2, 3, 7, 0x87, 31, 10, 255, 0x80, 0, 0, 0, 20, 129, 0x7f, 0xff, 0xff, 0xff,
            ],
        );
        append_box(&mut boxes, MAPPING, &[0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 2]);
        append_box(
            &mut boxes,
            CHANNELS,
            &[0, 3, 0, 2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 2],
        );
        let mut data = prefix(&boxes);
        append_box(&mut data, CODESTREAM, &[]);
        let header = inspect(&data).unwrap();
        assert!(header.changes_channels());
        let mut original = plane(&[-3, 0, 1, 17]);
        let limits = Limits {
            max_components: 2,
            ..Limits::default()
        };
        let refused = apply_colour(&header, &data, &mut original, limits, &mut Diagnostics::default()).unwrap_err();
        assert_eq!(refused.kind(), ErrorKind::Limit);
        assert_eq!(original.len(), 1);

        apply_colour(
            &header,
            &data,
            &mut original,
            Limits::default(),
            &mut Diagnostics::default(),
        )
        .unwrap();
        assert_eq!(original[0].samples, &[i32::MIN, i32::MIN, i32::MAX, i32::MAX]);
        assert_eq!(original[0].info.precision, 32);
        assert_eq!(original[1].samples, &[255, 255, 129, 129]);
        assert!(original[1].info.signed);
        assert_eq!(original[2].samples, &[10, 10, 20, 20]);
        assert_eq!(original.iter().map(|comp| comp.alpha).collect::<Vec<_>>(), [0, 0, 1]);
    }

    #[test]
    fn one_component_direct_mapping_is_repaired_like_cpp() {
        let mut boxes = Vec::new();
        append_box(&mut boxes, PALETTE, &[0, 2, 2, 7, 7, 10, 20, 30, 40]);
        append_box(&mut boxes, MAPPING, &[0, 0, 0, 0, 0, 0, 0, 0]);
        let mut data = prefix(&boxes);
        append_box(&mut data, CODESTREAM, &[]);
        let header = inspect(&data).unwrap();
        let mut original = plane(&[0, 1]);
        let mut diagnostics = Diagnostics::default();
        apply_colour(&header, &data, &mut original, Limits::default(), &mut diagnostics).unwrap();
        assert_eq!(original[0].samples, &[10, 30]);
        assert_eq!(original[1].samples, &[20, 40]);
        assert_eq!(
            diagnostics.entries(),
            &[Diagnostic::Warning(
                "Component mapping seems wrong. Trying to correct.\n".into()
            )]
        );
    }

    #[cfg(not(miri))]
    #[test]
    fn decoded_palette_and_channel_order_match_the_cpp_stream_api() {
        use crate::{DecodeParams, Decoder, DecoderConfig, Format, Jp2Channels};
        use neo_j2k_oracle::{BackendKind, OracleBackend};
        use neo_j2k_testkit::files::corpus_dir;
        use neo_j2k_testkit::request::{Backend, Mode, Request};

        let codestream = std::fs::read(corpus_dir().join("size_3x5_g.j2k")).unwrap();
        let mut boxes = Vec::new();
        append_box(&mut boxes, IMAGE_HEADER, &[0, 0, 0, 5, 0, 0, 0, 3, 0, 1, 7, 7, 0, 0]);
        append_box(&mut boxes, COLOUR, &[1, 0, 0, 0, 0, 0, 16]);
        append_box(&mut boxes, PALETTE, &[0, 2, 3, 7, 7, 7, 10, 20, 30, 40, 50, 60]);
        append_box(&mut boxes, MAPPING, &[0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 2]);
        append_box(
            &mut boxes,
            CHANNELS,
            &[0, 3, 0, 2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 2],
        );
        let mut file = MAGIC.to_vec();
        append_box(&mut file, FILE_TYPE, b"jp2 \0\0\0\0jp2 ");
        append_box(&mut file, JP2_HEADER, &boxes);
        append_box(&mut file, CODESTREAM, &codestream);

        let mut request = Request::new(&file, 0, Mode::Plain);
        request.jp2 = true;
        let cpp = OracleBackend::new(BackendKind::NeoJ2k).decode(&request);
        assert!(cpp.ok, "{}", String::from_utf8_lossy(&cpp.last_error));

        let mut decoder = Decoder::new(DecoderConfig {
            format: Format::Jp2,
            ..DecoderConfig::default()
        });
        assert_eq!(
            decoder.decode(&file, DecodeParams::default()).unwrap_err().kind(),
            ErrorKind::Unsupported
        );
        let params = DecodeParams {
            jp2_channels: Jp2Channels::Ignore,
            ..DecodeParams::default()
        };
        assert_eq!(decoder.decode(&file, params).unwrap().components.len(), 1);
        let params = DecodeParams {
            jp2_channels: Jp2Channels::Apply,
            ..DecodeParams::default()
        };
        let rust = decoder.decode(&file, params).unwrap();
        assert_eq!(rust.components.len(), cpp.planes.len());
        for (rust, cpp) in rust.components.iter().zip(cpp.planes) {
            assert_eq!(rust.samples(), cpp);
        }
        assert_eq!(
            rust.components.iter().map(DecodedComponent::alpha).collect::<Vec<_>>(),
            [0, 0, 1]
        );
    }
}
