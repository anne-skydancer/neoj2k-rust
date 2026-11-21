// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Packet headers and codeword segments (OpenJPEG's t2.c via src/t2.cpp; notice in parent module).

use crate::error::Diagnostics;
use crate::mem::{CType, WorkBudget};
use crate::tile::{CodeBlock, Resolution, Segment, Tile};
use crate::{Error, ErrorKind};

use super::progression::Packet;
use super::{Bio, PacketInput};

const SOP: [u8; 4] = [0xff, 0x91, 0, 4];
const EPH: [u8; 2] = [0xff, 0x92];
const CARRY_SOP: u8 = 2;
const CARRY_EPH: u8 = 4;
const BYPASS: u8 = 1;
const TERMINATE_ALL: u8 = 4;

pub(super) const MISSING_EPH: &str = "Not enough space for required EPH marker\n";

pub(super) struct Reader<'data, 'cursor, 'ppm> {
    data: &'data [u8],
    pos: usize,
    ppm: Option<&'cursor mut &'ppm [u8]>,
    ppt: Option<&'data [u8]>,
    csty: u8,
    strict: bool,
    cut: bool,
    sop_warned: bool,
    cut_warned: bool,
    cut_sop: bool,
}

impl<'data, 'cursor, 'ppm> Reader<'data, 'cursor, 'ppm> {
    pub(super) fn new(input: PacketInput<'data, 'cursor, 'ppm>) -> Self {
        Self {
            data: input.data,
            pos: 0,
            ppm: input.ppm,
            ppt: input.ppt,
            csty: input.coding.csty,
            strict: input.strict,
            cut: input.cut,
            sop_warned: false,
            cut_warned: false,
            cut_sop: false,
        }
    }

    pub(super) fn exhausted(&self) -> bool {
        self.pos == self.data.len()
            && self.ppm.as_deref().is_none_or(|cursor| cursor.is_empty())
            && self.ppt.is_none_or(<[u8]>::is_empty)
    }

    pub(super) fn strict_eph(&self) -> bool {
        self.strict && self.csty & CARRY_EPH != 0
    }

    pub(super) fn visit(
        &mut self,
        tile: &mut Tile,
        packet: Packet,
        skip: bool,
        budget: &mut WorkBudget,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Error> {
        let comp = &mut tile.comps[packet.comp];
        let res = &mut comp.res[packet.res];
        let result = self
            .read_header(res, packet, comp.cblksty, budget, diagnostics)
            .and_then(|present| {
                if present {
                    self.read_data(res, packet, skip, budget, diagnostics)?;
                }
                Ok(())
            });
        result.map_err(|error| {
            if self.cut_sop && error.kind() == ErrorKind::Data {
                Error::new(ErrorKind::NeedData, error.message())
            } else {
                error
            }
        })
    }

    fn read_sop(&mut self, diagnostics: &mut Diagnostics) {
        self.cut_sop = false;
        if self.csty & CARRY_SOP == 0 {
            return;
        }
        let remaining = &self.data[self.pos..];
        if remaining.len() < 6 {
            if !self.sop_warned {
                self.sop_warned = true;
                diagnostics.warn("Not enough space for expected SOP marker\n");
            }
            let prefix = remaining.len().min(SOP.len());
            self.cut_sop = self.cut && prefix != 0 && remaining[..prefix] == SOP[..prefix];
        } else if remaining.starts_with(&SOP[..2]) {
            self.pos += 6;
        } else if !self.sop_warned {
            self.sop_warned = true;
            diagnostics.warn("Expected SOP marker\n");
        }
    }

    fn read_header(
        &mut self,
        res: &mut Resolution,
        packet: Packet,
        cblksty: u8,
        budget: &mut WorkBudget,
        diagnostics: &mut Diagnostics,
    ) -> Result<bool, Error> {
        // Tile setup initializes the precincts. Each progression starts at layer zero,
        // and the packet iterator rejects revisiting layers across progression changes.
        self.read_sop(diagnostics);
        let source = if let Some(ppm) = self.ppm.as_deref() {
            *ppm
        } else {
            self.ppt.unwrap_or(&self.data[self.pos..])
        };
        let mut bits = Bio::new(source);
        let present = bits.bit() != 0;
        if present {
            read_pass_headers(res, packet, cblksty, &mut bits, budget)?;
        }
        let aligned = bits.align();
        if (!aligned || bits.overrun()) && !self.strict {
            self.drop_header(res, packet, source.len(), present);
            return Ok(false);
        }
        if present && !aligned {
            return Err(bad_packet(""));
        }
        let mut length = bits.num_bytes();
        if self.csty & CARRY_EPH != 0 {
            let rest = &source[length..];
            if rest.len() < EPH.len() {
                if self.strict {
                    return Err(bad_packet(MISSING_EPH));
                }
                self.drop_header(res, packet, source.len(), present);
                return Ok(false);
            }
            if !rest.starts_with(&EPH) {
                return Err(bad_packet("Expected EPH marker\n"));
            }
            length += EPH.len();
        }
        if present && length == 0 {
            return Err(bad_packet(""));
        }
        self.advance_header(length);
        Ok(present)
    }

    fn advance_header(&mut self, length: usize) {
        if let Some(ppm) = &mut self.ppm {
            let remaining = **ppm;
            **ppm = &remaining[length..];
        } else if let Some(ppt) = &mut self.ppt {
            *ppt = &ppt[length..];
        } else {
            self.pos += length;
        }
    }

    fn drop_header(&mut self, res: &mut Resolution, packet: Packet, length: usize, present: bool) {
        if present {
            for band in res.bands.iter_mut().filter(|band| !band.empty()) {
                for block in &mut band.precincts[packet.precinct].cblks {
                    if block.numnewpasses != 0 {
                        block.corrupted = true;
                        block.numnewpasses = 0;
                    }
                }
            }
        }
        self.advance_header(length);
        self.pos = self.data.len();
    }

    fn read_data(
        &mut self,
        res: &mut Resolution,
        packet: Packet,
        skip: bool,
        budget: &mut WorkBudget,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Error> {
        let max_length = self.data.len() - self.pos;
        let mut partial = false;
        for (band_index, band) in res.bands.iter_mut().enumerate() {
            if band.empty() {
                continue;
            }
            for (block_index, block) in band.precincts[packet.precinct].cblks.iter_mut().enumerate() {
                if block.numnewpasses == 0 {
                    continue;
                }
                let segment = first_new_segment(block);
                if partial || (!skip && block.corrupted) {
                    skip_passes(block, segment);
                    if !skip {
                        block.corrupted = true;
                    }
                    continue;
                }
                let mut at = segment;
                loop {
                    let newlen = block.segs[at].newlen;
                    let len = usize::try_from(newlen).expect("byte count fits usize");
                    if len > self.data.len() - self.pos {
                        if self.strict || !self.cut_warned {
                            let mode = if skip { "skip" } else { "read" };
                            let message = format!(
                                "{mode}: segment too long ({newlen}) with max ({max_length}) for codeblock \
                                 {block_index} (p={}, b={band_index}, r={}, c={})\n",
                                packet.precinct, packet.res, packet.comp,
                            );
                            if self.strict {
                                return Err(bad_packet(message));
                            }
                            diagnostics.warn(message);
                            self.cut_warned = true;
                        }
                        partial = true;
                        if !skip {
                            block.real_num_segs = block.numsegs - u32::from(block.segs[at].numpasses == 0);
                            block.corrupted = true;
                        }
                        skip_passes(block, at);
                        break;
                    }
                    let end = self.pos + len;
                    if !skip {
                        grow_chunks(block, budget)?;
                        block.chunks.push(self.pos..end);
                    }
                    self.pos = end;
                    let seg = &mut block.segs[at];
                    seg.numpasses += seg.numnewpasses;
                    block.numnewpasses -= seg.numnewpasses;
                    if !skip {
                        seg.len += newlen;
                        seg.real_num_passes = seg.numpasses;
                    }
                    if block.numnewpasses == 0 {
                        if !skip {
                            block.real_num_segs = block.numsegs;
                        }
                        break;
                    }
                    at += 1;
                    block.numsegs += 1;
                }
            }
        }
        if partial {
            self.pos = self.data.len();
        }
        Ok(())
    }
}

fn bad_packet(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Data, message)
}

fn read_pass_headers(
    res: &mut Resolution,
    packet: Packet,
    cblksty: u8,
    bio: &mut Bio<'_>,
    budget: &mut WorkBudget,
) -> Result<(), Error> {
    for band in res.bands.iter_mut().filter(|band| !band.empty()) {
        let precinct = &mut band.precincts[packet.precinct];
        for (index, block) in (0..).zip(&mut precinct.cblks) {
            let first = block.numsegs == 0;
            let included = if first {
                let threshold = i32::try_from(packet.layer + 1).expect("layer count fits i32");
                precinct.incl.decode(bio, index, threshold)?
            } else {
                bio.bit() != 0
            };
            if !included {
                block.numnewpasses = 0;
                continue;
            }
            if first {
                let missing = precinct.imsb.decode_value(bio, index)?;
                block.numbps = (band.numbps - missing).cast_unsigned();
                block.numlenbits = 3;
            }
            block.numnewpasses = bio.num_passes();
            block.numlenbits = block.numlenbits.wrapping_add(bio.comma_code());
            let mut at = if first { 0 } else { used_segments(block) - 1 };
            if first || block.segs[at].numpasses == block.segs[at].maxpasses {
                at += usize::from(!first);
                init_segment(block, at, cblksty, budget)?;
            }

            let mut passes = block.numnewpasses;
            loop {
                let seg = &mut block.segs[at];
                seg.numnewpasses = passes.min(seg.maxpasses - seg.numpasses);
                let width = block
                    .numlenbits
                    .wrapping_add(seg.numnewpasses.checked_ilog2().unwrap_or(0));
                if width > 32 {
                    return Err(bad_packet(format!(
                        "Invalid bit number {width} in opj_t2_read_packet_header()\n"
                    )));
                }
                seg.newlen = bio.read(width);
                passes -= seg.numnewpasses;
                if passes == 0 {
                    break;
                }
                at += 1;
                init_segment(block, at, cblksty, budget)?;
            }
        }
    }
    Ok(())
}

fn used_segments(block: &CodeBlock) -> usize {
    usize::try_from(block.numsegs).expect("segment count fits usize")
}

fn first_new_segment(block: &mut CodeBlock) -> usize {
    let count = used_segments(block);
    if count == 0 {
        block.numsegs = 1;
        return 0;
    }
    let last = count - 1;
    if block.segs[last].numpasses == block.segs[last].maxpasses {
        block.numsegs += 1;
        count
    } else {
        last
    }
}

fn skip_passes(block: &mut CodeBlock, mut at: usize) {
    loop {
        let seg = &mut block.segs[at];
        seg.numpasses += seg.numnewpasses;
        block.numnewpasses -= seg.numnewpasses;
        if block.numnewpasses == 0 {
            break;
        }
        at += 1;
        block.numsegs += 1;
    }
}

fn next_capacity(len: usize, description: &str) -> Result<usize, Error> {
    if len == 0 {
        Ok(4)
    } else {
        len.checked_mul(2)
            .ok_or_else(|| Error::new(ErrorKind::NoMemory, description))
    }
}

fn init_segment(block: &mut CodeBlock, at: usize, cblksty: u8, budget: &mut WorkBudget) -> Result<(), Error> {
    if at >= block.segs.len() {
        const MESSAGE: &str = "";
        let old = block.segs.len();
        let new = next_capacity(old, MESSAGE)?;
        budget.array(CType::Segment, new)?;
        block
            .segs
            .try_reserve_exact(new - old)
            .map_err(|_| Error::new(ErrorKind::NoMemory, MESSAGE))?;
        block.segs.resize(new, Segment::default());
    }
    let maxpasses = if cblksty & TERMINATE_ALL != 0 {
        1
    } else if cblksty & BYPASS != 0 {
        if at == 0 {
            10
        } else if matches!(block.segs[at - 1].maxpasses, 1 | 10) {
            2
        } else {
            1
        }
    } else {
        109
    };
    block.segs[at] = Segment {
        maxpasses,
        ..Segment::default()
    };
    Ok(())
}

fn grow_chunks(block: &mut CodeBlock, budget: &mut WorkBudget) -> Result<(), Error> {
    const MESSAGE: &str = "cannot allocate opj_tcd_seg_data_chunk_t* array";
    let len = block.chunks.len();
    if len == 0 || (len >= 4 && len.is_power_of_two()) {
        let capacity = next_capacity(len, MESSAGE)?;
        budget.array(CType::Chunk, capacity)?;
        block
            .chunks
            .try_reserve_exact(capacity - len)
            .map_err(|_| Error::new(ErrorKind::NoMemory, MESSAGE))?;
    }
    Ok(())
}
