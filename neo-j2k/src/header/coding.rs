// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Shared readers for the coding and quantization marker segments.

use super::{be16, component_room, invalid, limit, read_component};
use crate::error::Diagnostics;
use crate::{Error, ErrorKind, Limits};

pub(crate) const MAX_RESOLUTIONS: usize = 33;
const MAX_BANDS: usize = 3 * MAX_RESOLUTIONS - 2;

const CSTY_PRECINCTS: u8 = 0x01;
const CSTY_SOP: u8 = 0x02;
const CSTY_EPH: u8 = 0x04;
const CBLK_HT_MIXED: u8 = 0x80;
const QNT_NONE: u8 = 0;
pub(crate) const QNT_DERIVED: u8 = 1;
const QNT_EXPOUNDED: u8 = 2;

#[derive(Clone, Copy, Debug)]
#[allow(
    dead_code,
    reason = "all coding flags and exponents are used by the tile decoder in stage R2"
)]
pub(crate) struct CodingStyle {
    pub(crate) csty: u8,
    pub(crate) numresolutions: u8,
    pub(crate) cblkw: u8,
    pub(crate) cblkh: u8,
    pub(crate) cblksty: u8,
    pub(crate) qmfbid: u8,
    pub(crate) precincts: [u8; MAX_RESOLUTIONS],
}

impl Default for CodingStyle {
    fn default() -> Self {
        Self {
            csty: 0,
            numresolutions: 0,
            cblkw: 0,
            cblkh: 0,
            cblksty: 0,
            qmfbid: 0,
            precincts: [0; MAX_RESOLUTIONS],
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[allow(dead_code, reason = "the packet iterator uses all tile coding flags in stage R2")]
pub(crate) struct TileCoding {
    pub(crate) csty: u8,
    pub(crate) prg: u8,
    pub(crate) numlayers: u16,
    pub(crate) mct: u8,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code, reason = "the tile decoder uses guard bits and step sizes in stage R2")]
pub(crate) struct Quantization {
    pub(crate) qntsty: u8,
    pub(crate) numgbits: u8,
    pub(crate) nbands: u8,
    pub(crate) steps: [u16; MAX_BANDS],
}

impl Default for Quantization {
    fn default() -> Self {
        Self {
            qntsty: 0,
            numgbits: 0,
            nbands: 0,
            steps: [0; MAX_BANDS],
        }
    }
}

impl Quantization {
    #[allow(dead_code, reason = "the tile decoder reads individual band steps in stage R2")]
    pub(crate) fn step(&self, band: u32) -> (u32, u32) {
        if self.qntsty == QNT_DERIVED {
            let entry = u32::from(self.steps[0]);
            let exponent = (entry >> 11).saturating_sub(band.saturating_sub(1) / 3);
            (exponent, entry & 0x7ff)
        } else if band < u32::from(self.nbands) {
            let entry = u32::from(self.steps[band as usize]);
            (entry >> 11, entry & 0x7ff)
        } else {
            (0, 0)
        }
    }
}

fn read_spcod(body: &[u8], csty: u8) -> Result<(CodingStyle, usize), Error> {
    const SPC_ERROR: &str = "Error reading SPCod SPCoc element\n";
    if body.len() < 5 {
        return Err(invalid(SPC_ERROR));
    }
    let resolutions = usize::from(body[0]) + 1;
    if resolutions > MAX_RESOLUTIONS {
        return Err(invalid(format!(
            "Invalid value for numresolutions : {resolutions}, max value is set in openjpeg.h at {MAX_RESOLUTIONS}\n",
        )));
    }
    if body[1] > 8 || body[2] > 8 || u16::from(body[1]) + u16::from(body[2]) > 8 {
        return Err(invalid(
            "Error reading SPCod SPCoc element, Invalid cblkw/cblkh combination\n",
        ));
    }
    if body[3] & CBLK_HT_MIXED != 0 {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "Error reading SPCod SPCoc element. Unsupported Mixed HT code-block style found\n",
        ));
    }
    if body[4] > 1 {
        return Err(invalid(
            "Error reading SPCod SPCoc element, Invalid transformation found\n",
        ));
    }
    let mut style = CodingStyle {
        csty,
        numresolutions: body[0] + 1,
        cblkw: body[1] + 2,
        cblkh: body[2] + 2,
        cblksty: body[3],
        qmfbid: body[4],
        precincts: [0; MAX_RESOLUTIONS],
    };
    let used = if csty & CSTY_PRECINCTS == 0 {
        style.precincts.fill(0xff);
        5
    } else {
        let precincts = body.get(5..5 + resolutions).ok_or_else(|| invalid(SPC_ERROR))?;
        if precincts[1..].iter().any(|p| p & 0x0f == 0 || p >> 4 == 0) {
            return Err(invalid("Invalid precinct size\n"));
        }
        style.precincts[..resolutions].copy_from_slice(precincts);
        5 + resolutions
    };
    Ok((style, used))
}

pub(super) fn read_cod(
    body: &[u8],
    limits: Limits,
    diagnostics: &mut Diagnostics,
) -> Result<(TileCoding, CodingStyle), Error> {
    const COD_ERROR: &str = "Error reading COD marker\n";
    if body.len() < 5 {
        return Err(invalid(COD_ERROR));
    }
    let csty = body[0];
    if csty & !(CSTY_PRECINCTS | CSTY_SOP | CSTY_EPH) != 0 {
        return Err(invalid("Unknown Scod value in COD marker\n"));
    }
    let prg = body[1];
    if prg > 4 {
        return Err(invalid("Unknown progression order in COD marker\n"));
    }
    let numlayers = be16(&body[2..]);
    if numlayers == 0 {
        return Err(invalid(
            "Invalid number of layers in COD marker : 0 not in range [1-65535]\n",
        ));
    }
    if limits.max_layers != 0 && u32::from(numlayers) > limits.max_layers {
        return Err(limit(format!(
            "Error with COD marker: {numlayers} layers exceed the decoder's limit of {}\n",
            limits.max_layers,
        )));
    }
    let mct = body[4];
    if mct > 1 {
        return Err(invalid("Invalid multiple component transformation\n"));
    }
    let (style, used) = read_spcod(&body[5..], csty & CSTY_PRECINCTS)
        .map_err(|error| super::wrap_error(error, COD_ERROR, diagnostics))?;
    if 5 + used != body.len() {
        return Err(invalid(COD_ERROR));
    }
    Ok((
        TileCoding {
            csty,
            prg,
            numlayers,
            mct,
        },
        style,
    ))
}

pub(super) fn read_coc(body: &[u8], numcomps: u32, diagnostics: &mut Diagnostics) -> Result<(u16, CodingStyle), Error> {
    const COC_ERROR: &str = "Error reading COC marker\n";
    let room = component_room(numcomps);
    if body.len() < room + 1 {
        return Err(invalid(COC_ERROR));
    }
    let comp = read_component(body, room);
    if u32::from(comp) >= numcomps {
        return Err(invalid("Error reading COC marker (bad number of components)\n"));
    }
    let (style, used) =
        read_spcod(&body[room + 1..], body[room]).map_err(|error| super::wrap_error(error, COC_ERROR, diagnostics))?;
    if room + 1 + used != body.len() {
        return Err(invalid(COC_ERROR));
    }
    Ok((comp, style))
}

pub(super) fn read_sqcd(body: &[u8], marker: &str, diagnostics: &mut Diagnostics) -> Result<Quantization, Error> {
    let (&flags, rest) = body
        .split_first()
        .ok_or_else(|| invalid("Error reading SQcd or SQcc element\n"))?;
    let mut quant = Quantization {
        qntsty: flags & 0x1f,
        numgbits: flags >> 5,
        ..Quantization::default()
    };
    let marker_error = || invalid(format!("Error reading {marker} marker\n"));
    match quant.qntsty {
        QNT_DERIVED => {
            if rest.len() != 2 {
                return Err(marker_error());
            }
            quant.nbands = 1;
            quant.steps[0] = be16(rest);
        }
        QNT_NONE | QNT_EXPOUNDED => {
            let width = if quant.qntsty == QNT_NONE { 1 } else { 2 };
            if !rest.len().is_multiple_of(width) {
                return Err(marker_error());
            }
            let bands = rest.len() / width;
            if bands > MAX_BANDS {
                diagnostics.warn_fmt(format_args!(
                    "While reading CCP_QNTSTY element inside QCD or QCC marker segment, number of subbands ({bands}) is greater to OPJ_J2K_MAXBANDS ({MAX_BANDS}). So we limit the number of elements stored to OPJ_J2K_MAXBANDS ({MAX_BANDS}) and skip the rest. \n",
                ));
            }
            let bands = bands.min(MAX_BANDS);
            quant.nbands = u8::try_from(bands).expect("band count capped at MAX_BANDS");
            for (step, bytes) in quant.steps.iter_mut().zip(rest.chunks_exact(width)) {
                *step = if width == 1 {
                    u16::from(bytes[0] >> 3) << 11
                } else {
                    be16(bytes)
                };
            }
        }
        _ => {
            return Err(invalid(format!(
                "Unknown quantization style {} in {marker} marker\n",
                quant.qntsty
            )));
        }
    }
    Ok(quant)
}

pub(super) fn read_qcc(
    body: &[u8],
    numcomps: u32,
    diagnostics: &mut Diagnostics,
) -> Result<(u16, Quantization), Error> {
    let room = component_room(numcomps);
    if body.len() < room {
        return Err(invalid("Error reading QCC marker\n"));
    }
    let comp = read_component(body, room);
    if u32::from(comp) >= numcomps {
        return Err(invalid(format!(
            "Invalid component number: {comp}, regarding the number of components {numcomps}\n"
        )));
    }
    Ok((comp, read_sqcd(&body[room..], "QCC", diagnostics)?))
}
