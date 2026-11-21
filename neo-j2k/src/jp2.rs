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

//! JP2 and JPH container inspection, up to the beginning of the codestream.

use core::ops::Range;

use crate::error::Diagnostics;
use crate::mem::{CType, HeaderLedger};
use crate::{Error, ErrorKind};

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
    #[allow(dead_code, reason = "the decoded image consumes the palette")]
    entries: usize,
    #[allow(dead_code, reason = "the decoded image consumes the palette")]
    bits: Range<usize>,
    #[allow(dead_code, reason = "the decoded image consumes the palette")]
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

#[cfg(test)]
mod tests {
    use super::*;
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
}
