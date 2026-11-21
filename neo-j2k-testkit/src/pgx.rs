// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! PGX, the one-component format of the ISO/IEC 15444-4 reference images (`tests/common/pgx.cpp`):
//! a text header `PG <ML|LM> [+|-]<depth> <width> <height>` and one whitespace byte, then the
//! samples in 1, 2 or 4 bytes, big-endian for `ML`. Same checks and messages as the C++ (its
//! `isdigit`/`isspace` in the C locale: ASCII only).

use std::path::Path;

/// A PGX image.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Pgx {
    /// Samples a row.
    pub width: i32,
    /// Rows.
    pub height: i32,
    /// Bits a sample, 1 to 32.
    pub depth: i32,
    /// Two's-complement samples.
    pub is_signed: bool,
    /// `width x height` samples in rows, sign-extended when signed.
    pub samples: Vec<i32>,
}

/// Reads a PGX file; `Err` with the C++'s reason when it does not read as one.
pub fn read_pgx(path: &Path) -> Result<Pgx, &'static str> {
    let bytes = std::fs::read(path).map_err(|_| "cannot read the file")?;
    parse_pgx(&bytes)
}

/// Parses PGX bytes; `Err` with the C++'s reason.
pub fn parse_pgx(b: &[u8]) -> Result<Pgx, &'static str> {
    let skip_spaces = |p: &mut usize| {
        while *p < b.len() && (b[*p] == b' ' || b[*p] == b'\t') {
            *p += 1;
        }
    };
    // up to nine digits (strtol of them cannot overflow)
    let number = |p: &mut usize| -> Option<i32> {
        skip_spaces(p);
        let start = *p;
        while *p < b.len() && b[*p].is_ascii_digit() {
            *p += 1;
        }
        if *p == start || *p - start > 9 {
            return None;
        }
        Some(b[start..*p].iter().fold(0i32, |v, &d| v * 10 + i32::from(d - b'0')))
    };
    if b.len() < 8 || b[0] != b'P' || b[1] != b'G' {
        return Err("not a PGX file");
    }
    let mut p = 2usize;
    skip_spaces(&mut p);
    if p + 2 > b.len() {
        return Err("header cut");
    }
    let big_endian = b[p] == b'M' && b[p + 1] == b'L';
    if !big_endian && !(b[p] == b'L' && b[p + 1] == b'M') {
        return Err("bad byte order");
    }
    p += 2;
    skip_spaces(&mut p);
    let mut out = Pgx::default();
    if p < b.len() && (b[p] == b'+' || b[p] == b'-') {
        out.is_signed = b[p] == b'-';
        p += 1;
    }
    let (Some(depth), Some(w), Some(h)) = (number(&mut p), number(&mut p), number(&mut p)) else {
        return Err("bad header numbers");
    };
    if !(1..=32).contains(&depth) || w < 1 || h < 1 {
        return Err("bad header numbers");
    }
    // isspace in the C locale
    if p >= b.len() || !matches!(b[p], b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') {
        return Err("no whitespace after the header");
    }
    p += 1; // one whitespace byte ends the header
    out.depth = depth;
    out.width = w;
    out.height = h;
    let bytes_per: usize = if depth <= 8 {
        1
    } else if depth <= 16 {
        2
    } else {
        4
    };
    // w and h are below 10^9, so n * bytes_per fits 64 bits
    let n = w.unsigned_abs() as usize * h.unsigned_abs() as usize;
    if b.len() - p < n * bytes_per {
        return Err("samples cut");
    }
    out.samples = b[p..p + n * bytes_per]
        .chunks_exact(bytes_per)
        .map(|s| {
            let v = s.iter().enumerate().fold(0u32, |v, (k, &byte)| {
                if big_endian {
                    (v << 8) | u32::from(byte)
                } else {
                    v | (u32::from(byte) << (8 * k))
                }
            });
            if out.is_signed && bytes_per < 4 {
                let sign = 1u32 << (8 * bytes_per - 1);
                if v & sign != 0 {
                    // the sign bit set: v - 2 * sign, which fits i32 for 1 and 2 bytes
                    #[allow(clippy::cast_possible_truncation)]
                    let s = (i64::from(v) - (i64::from(sign) << 1)) as i32;
                    s
                } else {
                    v.cast_signed()
                }
            } else {
                v.cast_signed()
            }
        })
        .collect();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_both_byte_orders_and_signs() {
        let p = parse_pgx(b"PG ML -12 2 1\n\xff\xfe\x00\x05").unwrap();
        assert_eq!((p.width, p.height, p.depth, p.is_signed), (2, 1, 12, true));
        assert_eq!(p.samples, vec![-2, 5]);
        let p = parse_pgx(b"PG LM +8 3 1 \x01\x80\xff").unwrap();
        assert_eq!(p.samples, vec![1, 128, 255]);
        let p = parse_pgx(b"PG LM -8 2 1 \x80\x7f").unwrap();
        assert_eq!(p.samples, vec![-128, 127]);
        let p = parse_pgx(b"PG ML 32 1 1 \xff\xff\xff\xff").unwrap();
        assert_eq!(p.samples, vec![-1]);
    }

    #[test]
    fn refusals_carry_the_cpp_reasons() {
        assert_eq!(parse_pgx(b"P5 1 1 255\n"), Err("not a PGX file"));
        assert_eq!(parse_pgx(b"PG       "), Err("header cut"));
        assert_eq!(parse_pgx(b"PG XX 8 1 1 \x00"), Err("bad byte order"));
        assert_eq!(parse_pgx(b"PG ML 33 1 1 \x00"), Err("bad header numbers"));
        assert_eq!(parse_pgx(b"PG ML 8 1234567890 1 \x00"), Err("bad header numbers"));
        assert_eq!(parse_pgx(b"PG ML 8 1 1x\x00"), Err("no whitespace after the header"));
        assert_eq!(parse_pgx(b"PG ML 16 2 1 \x00\x01"), Err("samples cut"));
    }
}
