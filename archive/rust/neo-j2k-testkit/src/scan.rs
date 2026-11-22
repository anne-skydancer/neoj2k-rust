// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! A minimal codestream reader for the tests (`tests/common/j2k_scan.cpp`): the main header's SIZ
//! and COD facts, the tile-parts, the coding parameters of every COD and COC marker, and packet
//! boundaries from SOP markers or PLT. It never decodes; it picks cut points and names the
//! departures behind a difference. Same walk and checks as the C++ ([`summary`] writes what the
//! oracle crate's shim writes, line for line, for the differential test).

use std::fmt::Write as _;

const SOC: u16 = 0xFF4F;
const SIZ: u16 = 0xFF51;
const COD: u16 = 0xFF52;
const COC: u16 = 0xFF53;
const TLM: u16 = 0xFF55;
const PLM: u16 = 0xFF57;
const PLT: u16 = 0xFF58;
const QCD: u16 = 0xFF5C;
const QCC: u16 = 0xFF5D;
const RGN: u16 = 0xFF5E;
const POC: u16 = 0xFF5F;
const PPM: u16 = 0xFF60;
const PPT: u16 = 0xFF61;
const CRG: u16 = 0xFF63;
const COM: u16 = 0xFF64;
const CAP: u16 = 0xFF50;
const CPF: u16 = 0xFF59;
const MCT: u16 = 0xFF74;
const MCC: u16 = 0xFF75;
const MCO: u16 = 0xFF77;
const CBD: u16 = 0xFF78;
const SOT: u16 = 0xFF90;
const SOD: u16 = 0xFF93;
const EOC: u16 = 0xFFD9;

/// The markers OpenJPEG reads in a main header after SIZ (`j2k.c`, the handlers whose states
/// include `J2K_STATE_MH`).
fn main_header_marker(m: u16) -> bool {
    matches!(
        m,
        COD | COC | RGN | QCD | QCC | POC | TLM | PLM | PPM | CRG | COM | MCT | CBD | CAP | CPF | MCC | MCO
    )
}

struct Reader<'a> {
    b: &'a [u8],
}

impl Reader<'_> {
    fn has(&self, pos: usize, n: usize) -> bool {
        pos <= self.b.len() && n <= self.b.len() - pos
    }
    fn u16(&self, p: usize) -> u16 {
        u16::from_be_bytes([self.b[p], self.b[p + 1]])
    }
    fn u32(&self, p: usize) -> u32 {
        u32::from_be_bytes([self.b[p], self.b[p + 1], self.b[p + 2], self.b[p + 3]])
    }
}

/// The coding parameters of one COD or COC marker segment.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Coding {
    /// COD (it sets the layers) or COC.
    pub is_cod: bool,
    /// COD only.
    pub layers: u16,
    /// Decomposition levels: resolutions less one.
    pub levels: u8,
    /// Code-block size exponents less 2, as coded.
    pub xcb: u8,
    #[allow(missing_docs)]
    pub ycb: u8,
    /// `PPx | PPy << 4` per resolution, when the marker gives them.
    pub precincts: Vec<u8>,
}

/// The main header's facts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct MainHeader {
    pub xsiz: u32,
    pub ysiz: u32,
    pub xosiz: u32,
    pub yosiz: u32,
    pub xtsiz: u32,
    pub ytsiz: u32,
    pub xtosiz: u32,
    pub ytosiz: u32,
    pub rsiz: u16,
    pub csiz: u16,
    pub ssiz: Vec<u8>,
    pub xrsiz: Vec<u8>,
    pub yrsiz: Vec<u8>,
    /// The last COD's fields (a later COD of the main header replaces an earlier one).
    pub scod: u8,
    pub progression: u8,
    pub mct: u8,
    pub levels: u8,
    pub cblk_w_exp: u8,
    pub cblk_h_exp: u8,
    pub cblk_style: u8,
    pub transform: u8,
    pub layers: u16,
    /// `PPx | PPy << 4` per resolution, when `Scod & 1`.
    pub precincts: Vec<u8>,
    /// Every COD and COC of the main header, in order.
    pub codings: Vec<Coding>,
    /// The fewest resolutions of any component in the main header (COD, COC).
    pub min_resolutions: i32,
    pub has_coc: bool,
    pub has_qcc: bool,
    pub has_poc: bool,
    pub has_ppm: bool,
    pub has_tlm: bool,
    pub has_plm: bool,
    pub has_com: bool,
    /// A marker OpenJPEG does not read in a main header (or anything but SIZ first).
    pub unknown_marker: bool,
    /// The offset of the first SOT: the main header's end.
    pub end: usize,
}

fn ceil_div(a: u64, b: u64) -> u64 {
    if b == 0 { 0 } else { a.div_ceil(b) }
}

impl MainHeader {
    /// Tiles across.
    #[must_use]
    pub fn tiles_x(&self) -> u64 {
        if self.xtsiz != 0 && self.xsiz > self.xtosiz {
            ceil_div(u64::from(self.xsiz) - u64::from(self.xtosiz), u64::from(self.xtsiz))
        } else {
            0
        }
    }
    /// Tiles down.
    #[must_use]
    pub fn tiles_y(&self) -> u64 {
        if self.ytsiz != 0 && self.ysiz > self.ytosiz {
            ceil_div(u64::from(self.ysiz) - u64::from(self.ytosiz), u64::from(self.ytsiz))
        } else {
            0
        }
    }
    /// The image's width.
    #[must_use]
    pub fn width(&self) -> u64 {
        u64::from(self.xsiz).wrapping_sub(u64::from(self.xosiz))
    }
    /// The image's height.
    #[must_use]
    pub fn height(&self) -> u64 {
        u64::from(self.ysiz).wrapping_sub(u64::from(self.yosiz))
    }
}

/// COD: Scod, progression, layers (2 bytes), MCT, then SPcod; `None` when the segment is short.
fn read_cod(body: &[u8]) -> Option<Coding> {
    if body.len() < 10 {
        return None;
    }
    Some(Coding {
        is_cod: true,
        layers: u16::from_be_bytes([body[2], body[3]]),
        levels: body[5],
        xcb: body[6],
        ycb: body[7],
        precincts: if body[0] & 1 != 0 {
            body[10..].to_vec()
        } else {
            Vec::new()
        },
    })
}

/// COC: Ccoc (two bytes past 256 components), Scoc, then SPcoc as in COD.
fn read_coc(body: &[u8], csiz: u16) -> Option<Coding> {
    let at = if csiz >= 257 { 2 } else { 1 };
    if body.len() < at + 6 {
        return None;
    }
    Some(Coding {
        is_cod: false,
        layers: 0,
        levels: body[at + 1],
        xcb: body[at + 2],
        ycb: body[at + 3],
        precincts: if body[at] & 1 != 0 {
            body[at + 6..].to_vec()
        } else {
            Vec::new()
        },
    })
}

/// Parses SOC to the first SOT; `Err` with the C++'s reason when the bytes do not hold a complete
/// main header with SIZ and COD.
pub fn parse_main_header(bytes: &[u8]) -> Result<MainHeader, &'static str> {
    let r = Reader { b: bytes };
    let mut mh = MainHeader::default();
    if !r.has(0, 2) || r.u16(0) != SOC {
        return Err("no SOC marker");
    }
    let (mut siz, mut cod) = (false, false);
    let mut pos = 2usize;
    let mut coc_min = 0i32;
    loop {
        if !r.has(pos, 2) {
            return Err("main header cut");
        }
        let marker = r.u16(pos);
        if marker == SOT {
            mh.end = pos;
            break;
        }
        if !r.has(pos + 2, 2) {
            return Err("main header cut");
        }
        let len = usize::from(r.u16(pos + 2));
        if len < 2 || !r.has(pos + 2, len) {
            return Err("marker segment past the end");
        }
        let body = pos + 4;
        let seg = &bytes[body..body + len - 2];
        // SIZ first, then the markers OpenJPEG reads in a main header
        if if siz {
            !main_header_marker(marker)
        } else {
            marker != SIZ
        } {
            mh.unknown_marker = true;
        }
        match marker {
            SIZ if !siz => {
                if seg.len() < 38 {
                    return Err("short SIZ");
                }
                mh.rsiz = r.u16(body);
                mh.xsiz = r.u32(body + 2);
                mh.ysiz = r.u32(body + 6);
                mh.xosiz = r.u32(body + 10);
                mh.yosiz = r.u32(body + 14);
                mh.xtsiz = r.u32(body + 18);
                mh.ytsiz = r.u32(body + 22);
                mh.xtosiz = r.u32(body + 26);
                mh.ytosiz = r.u32(body + 30);
                mh.csiz = r.u16(body + 34);
                let csiz = usize::from(mh.csiz);
                if csiz == 0
                    || seg.len() != 36 + 3 * csiz
                    || mh.xsiz <= mh.xosiz
                    || mh.ysiz <= mh.yosiz
                    || mh.xtsiz == 0
                    || mh.ytsiz == 0
                {
                    return Err("bad SIZ");
                }
                for c in 0..csiz {
                    mh.ssiz.push(seg[36 + 3 * c]);
                    mh.xrsiz.push(seg[37 + 3 * c]);
                    mh.yrsiz.push(seg[38 + 3 * c]);
                }
                siz = true;
            }
            COD => {
                let c = read_cod(seg).ok_or("short COD")?;
                mh.scod = seg[0];
                mh.progression = seg[1];
                mh.layers = c.layers;
                mh.mct = seg[4];
                mh.levels = c.levels;
                mh.cblk_w_exp = c.xcb;
                mh.cblk_h_exp = c.ycb;
                mh.cblk_style = seg[8];
                mh.transform = seg[9];
                mh.precincts.clone_from(&c.precincts);
                mh.codings.push(c);
                cod = true;
            }
            COC => {
                mh.has_coc = true;
                if siz && let Some(c) = read_coc(seg, mh.csiz) {
                    let res = i32::from(c.levels) + 1;
                    coc_min = if coc_min != 0 { coc_min.min(res) } else { res };
                    mh.codings.push(c);
                }
            }
            QCC => mh.has_qcc = true,
            POC => mh.has_poc = true,
            PPM => mh.has_ppm = true,
            TLM => mh.has_tlm = true,
            PLM => mh.has_plm = true,
            COM => mh.has_com = true,
            // a second SIZ (OpenJPEG refuses it, marked above) and every other marker
            _ => {}
        }
        pos += 2 + len;
    }
    if !siz || !cod {
        return Err("no SIZ or COD before the first SOT");
    }
    mh.min_resolutions = i32::from(mh.levels) + 1;
    if coc_min != 0 {
        mh.min_resolutions = mh.min_resolutions.min(coc_min);
    }
    Ok(mh)
}

/// One tile-part.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TilePart {
    /// The offset of the SOT marker.
    pub sot: usize,
    /// The first byte after SOD.
    pub data: usize,
    /// One past the last byte of the tile-part (clamped to the bytes).
    pub end: usize,
    #[allow(missing_docs)]
    pub tile: u16,
    #[allow(missing_docs)]
    pub psot: u32,
    #[allow(missing_docs)]
    pub tpsot: u8,
    #[allow(missing_docs)]
    pub tnsot: u8,
    #[allow(missing_docs)]
    pub has_ppt: bool,
    #[allow(missing_docs)]
    pub has_plt: bool,
    /// Packet lengths, in order, when PLT is present.
    pub plt: Vec<u32>,
    /// The COD and COC markers of the tile-part header.
    pub codings: Vec<Coding>,
}

/// Every tile-part, following Psot (Psot = 0: to the end, less an EOC), as OpenJPEG does: after a
/// tile-part it reads a SOT or the end, nothing else.
#[must_use]
pub fn scan_tile_parts(bytes: &[u8], mh: &MainHeader) -> Vec<TilePart> {
    let r = Reader { b: bytes };
    let mut out = Vec::new();
    let mut pos = mh.end;
    while r.has(pos, 12) && r.u16(pos) == SOT && r.u16(pos + 2) == 10 {
        let mut tp = TilePart {
            sot: pos,
            tile: r.u16(pos + 4),
            psot: r.u32(pos + 6),
            tpsot: bytes[pos + 10],
            tnsot: bytes[pos + 11],
            ..TilePart::default()
        };
        let mut p = pos + 12;
        let mut sod = false;
        while r.has(p, 2) {
            let m = r.u16(p);
            if m == SOD {
                tp.data = p + 2;
                sod = true;
                break;
            }
            if !r.has(p + 2, 2) {
                break;
            }
            let len = usize::from(r.u16(p + 2));
            if len < 2 || !r.has(p + 2, len) {
                break;
            }
            let seg = &bytes[p + 4..p + 2 + len];
            if m == PPT {
                tp.has_ppt = true;
            } else if m == PLT && len >= 3 {
                tp.has_plt = true;
                // Zplt, then lengths: 7 bits a byte, the high bit set on every byte but the last
                let mut v = 0u32;
                for &b in &bytes[p + 5..p + 2 + len] {
                    v = (v << 7) | u32::from(b & 0x7F);
                    if b & 0x80 == 0 {
                        tp.plt.push(v);
                        v = 0;
                    }
                }
            } else if m == COD || m == COC {
                // a tile's COD and COC replace the main header's for that tile
                if let Some(c) = if m == COD {
                    read_cod(seg)
                } else {
                    read_coc(seg, mh.csiz)
                } {
                    tp.codings.push(c);
                }
            }
            p += 2 + len;
        }
        if !sod {
            break; // a cut header: OpenJPEG stops there too
        }
        let psot = tp.psot as usize;
        if psot != 0 {
            tp.end = (tp.sot + psot).min(bytes.len());
        } else {
            tp.end = bytes.len();
            if tp.end >= 2 && r.u16(tp.end - 2) == EOC {
                tp.end -= 2;
            }
        }
        let next = tp.sot + psot;
        out.push(tp);
        if psot == 0 || next >= bytes.len() {
            break;
        }
        pos = next;
    }
    out
}

/// Packet start offsets of a tile-part, from SOP markers or else from PLT; `None` when neither
/// gives them.
#[must_use]
pub fn packet_starts(bytes: &[u8], mh: &MainHeader, tp: &TilePart) -> Option<Vec<usize>> {
    let mut starts = Vec::new();
    if mh.scod & 2 != 0 {
        // SOP: FF91 0004 Nsop. Coded data never holds FF followed by a byte above 0x8F.
        let mut p = tp.data;
        while p + 6 <= tp.end {
            if bytes[p] == 0xFF && bytes[p + 1] == 0x91 && bytes[p + 2] == 0 && bytes[p + 3] == 4 {
                starts.push(p);
                p += 5;
            }
            p += 1;
        }
        return Some(starts);
    }
    if tp.has_plt && !mh.has_ppm && !tp.has_ppt {
        let mut p = tp.data;
        for &len in &tp.plt {
            starts.push(p);
            p = p.wrapping_add(len as usize);
        }
        // the lengths must cover the tile-part
        return (p == tp.end).then_some(starts);
    }
    None
}

/// Offsets where layer k (1..layers-1) starts, for a one-tile, one-tile-part LRCP stream whose
/// packets can be located: every prefix that ends there holds exactly the first k layers.
#[must_use]
pub fn layer_boundaries(bytes: &[u8]) -> Option<Vec<usize>> {
    let mh = parse_main_header(bytes).ok()?;
    if mh.progression != 0 || mh.layers < 2 || mh.has_poc || mh.tiles_x().wrapping_mul(mh.tiles_y()) != 1 {
        return None;
    }
    let tps = scan_tile_parts(bytes, &mh);
    if tps.len() != 1 || !tps[0].codings.is_empty() {
        return None;
    }
    let starts = packet_starts(bytes, &mh, &tps[0])?;
    let layers = usize::from(mh.layers);
    if starts.is_empty() || !starts.len().is_multiple_of(layers) {
        return None;
    }
    let per_layer = starts.len() / layers;
    Some((1..layers).map(|k| starts[k * per_layer]).collect())
}

fn hex_bytes(v: &[u8]) -> String {
    if v.is_empty() {
        return "-".to_owned();
    }
    v.iter().fold(String::with_capacity(2 * v.len()), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

fn list<T: ToString>(v: &[T]) -> String {
    if v.is_empty() {
        "-".to_owned()
    } else {
        v.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")
    }
}

fn coding_line(s: &mut String, tag: &str, c: &Coding) {
    let _ = writeln!(
        s,
        "{tag} {} {} {} {} {} prec {}",
        u32::from(c.is_cod),
        c.layers,
        c.levels,
        c.xcb,
        c.ycb,
        hex_bytes(&c.precincts)
    );
}

/// The scan of a codestream as text, line for line as the oracle crate's shim writes it from the
/// C++ scanner (`nj2k_shim_scan_summary`).
#[must_use]
pub fn summary(bytes: &[u8]) -> String {
    let mut s = String::new();
    match parse_main_header(bytes) {
        Err(why) => {
            let _ = writeln!(s, "main none {why}");
        }
        Ok(mh) => {
            let _ = writeln!(
                s,
                "siz {} {} {} {} {} {} {} {} rsiz {} csiz {}",
                mh.xsiz, mh.ysiz, mh.xosiz, mh.yosiz, mh.xtsiz, mh.ytsiz, mh.xtosiz, mh.ytosiz, mh.rsiz, mh.csiz
            );
            for c in 0..mh.ssiz.len() {
                let _ = writeln!(s, "comp {} {} {}", mh.ssiz[c], mh.xrsiz[c], mh.yrsiz[c]);
            }
            let _ = writeln!(
                s,
                "cod {} {} {} {} {} {} {} {} {} prec {}",
                mh.scod,
                mh.progression,
                mh.layers,
                mh.mct,
                mh.levels,
                mh.cblk_w_exp,
                mh.cblk_h_exp,
                mh.cblk_style,
                mh.transform,
                hex_bytes(&mh.precincts)
            );
            for c in &mh.codings {
                coding_line(&mut s, "coding", c);
            }
            let _ = writeln!(
                s,
                "flags {} {} {} {} {} {} {} {} {} end {} tiles {} {}",
                mh.min_resolutions,
                u32::from(mh.has_coc),
                u32::from(mh.has_qcc),
                u32::from(mh.has_poc),
                u32::from(mh.has_ppm),
                u32::from(mh.has_tlm),
                u32::from(mh.has_plm),
                u32::from(mh.has_com),
                u32::from(mh.unknown_marker),
                mh.end,
                mh.tiles_x(),
                mh.tiles_y()
            );
            for tp in scan_tile_parts(bytes, &mh) {
                let _ = writeln!(
                    s,
                    "tp {} {} {} {} {} {} {} ppt {} plt {} {}",
                    tp.sot,
                    tp.data,
                    tp.end,
                    tp.tile,
                    tp.psot,
                    tp.tpsot,
                    tp.tnsot,
                    u32::from(tp.has_ppt),
                    u32::from(tp.has_plt),
                    list(&tp.plt)
                );
                for c in &tp.codings {
                    coding_line(&mut s, "tpcoding", c);
                }
                let packets = packet_starts(bytes, &mh, &tp).map_or_else(|| "none".to_owned(), |v| list(&v));
                let _ = writeln!(s, "packets {packets}");
            }
        }
    }
    let layers = layer_boundaries(bytes).map_or_else(|| "none".to_owned(), |v| list(&v));
    let _ = writeln!(s, "layers {layers}");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    /// SOC, SIZ (one component, 8 x 8, one tile), COD (LRCP, 2 layers, 1 level), then a tile-part.
    fn tiny() -> Vec<u8> {
        let mut b = vec![0xFF, 0x4F, 0xFF, 0x51, 0, 41, 0, 0];
        for v in [8u32, 8, 0, 0, 8, 8, 0, 0] {
            b.extend_from_slice(&v.to_be_bytes());
        }
        b.extend_from_slice(&[0, 1, 7, 1, 1]);
        b.extend_from_slice(&[0xFF, 0x52, 0, 12, 0, 0, 0, 2, 0, 1, 4, 4, 0, 1]);
        b.extend_from_slice(&[0xFF, 0x90, 0, 10, 0, 0, 0, 0, 0, 20, 0, 1, 0xFF, 0x93, 1, 2, 3, 4, 5, 6]);
        b.extend_from_slice(&[0xFF, 0xD9]);
        b
    }

    #[test]
    fn reads_a_tiny_codestream() {
        let b = tiny();
        let mh = parse_main_header(&b).unwrap();
        assert_eq!(
            (mh.xsiz, mh.csiz, mh.layers, mh.levels, mh.min_resolutions),
            (8, 1, 2, 1, 2)
        );
        assert_eq!((mh.tiles_x(), mh.tiles_y()), (1, 1));
        let tps = scan_tile_parts(&b, &mh);
        assert_eq!(tps.len(), 1);
        assert_eq!(
            (tps[0].sot, tps[0].data, tps[0].end),
            (mh.end, mh.end + 14, mh.end + 20)
        );
        assert_eq!(packet_starts(&b, &mh, &tps[0]), None);
        assert!(summary(&b).ends_with("packets none\nlayers none\n"));
    }

    #[test]
    fn refusals_carry_the_cpp_reasons() {
        let b = tiny();
        assert_eq!(parse_main_header(&b[..1]), Err("no SOC marker"));
        assert_eq!(parse_main_header(&b[..3]), Err("main header cut"));
        assert_eq!(parse_main_header(&b[..20]), Err("marker segment past the end"));
        assert_eq!(summary(&b[..3]), "main none main header cut\nlayers none\n");
    }

    #[test]
    fn every_prefix_scans_without_panic() {
        let b = tiny();
        for n in 0..=b.len() {
            let _ = summary(&b[..n]);
        }
    }
}
