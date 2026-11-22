// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The reference tables `nj2k_diff record` writes (`corpus/generated/oracle-reference*.tsv`):
//!
//! ```text
//! whole <file> <mode> <reduce> <bytes> <ok> <hash>
//! cuts <file> <mode> <reduce> <from> <step> <count> <decoded> <digest>
//! ```
//!
//! Fields are separated by tabs; lines starting with `#` are comments.

use crate::fnv::parse_hex64;
use crate::request::Mode;

/// One line of a reference table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line {
    /// A whole file at a reduce level.
    Whole {
        /// The file, relative to the table's root.
        file: String,
        #[allow(missing_docs)]
        mode: Mode,
        #[allow(missing_docs)]
        reduce: i32,
        /// The file's size.
        bytes: usize,
        /// Whether it decoded.
        ok: bool,
        /// [`crate::digest::whole_value`].
        hash: u64,
    },
    /// Every cut of a file from `from` bytes in steps of `step`.
    Cuts {
        #[allow(missing_docs)]
        file: String,
        #[allow(missing_docs)]
        mode: Mode,
        #[allow(missing_docs)]
        reduce: i32,
        #[allow(missing_docs)]
        from: usize,
        #[allow(missing_docs)]
        step: usize,
        /// [`crate::digest::CutDigest`]'s numbers.
        count: usize,
        #[allow(missing_docs)]
        decoded: usize,
        #[allow(missing_docs)]
        digest: u64,
    },
}

/// One table line; `Ok(None)` for a comment or an empty line, `Err` with the reason for a line
/// that does not parse.
pub fn parse_line(line: &str) -> Result<Option<Line>, String> {
    let line = line.trim_end_matches(['\r', '\n']);
    if line.is_empty() || line.starts_with('#') {
        return Ok(None);
    }
    let f: Vec<&str> = line.split('\t').collect();
    let bad = |what: &str| format!("{what} in reference line '{line}'");
    let mode = |s: &str| Mode::parse(s).ok_or_else(|| bad("bad mode"));
    let num = |s: &str| s.parse::<usize>().map_err(|_| bad("bad number"));
    let reduce = |s: &str| s.parse::<i32>().map_err(|_| bad("bad reduce level"));
    let hex = |s: &str| parse_hex64(s).ok_or_else(|| bad("bad hash"));
    match (f[0], f.len()) {
        ("whole", 7) => Ok(Some(Line::Whole {
            file: f[1].to_owned(),
            mode: mode(f[2])?,
            reduce: reduce(f[3])?,
            bytes: num(f[4])?,
            ok: match f[5] {
                "1" => true,
                "0" => false,
                _ => return Err(bad("bad ok")),
            },
            hash: hex(f[6])?,
        })),
        ("cuts", 9) => Ok(Some(Line::Cuts {
            file: f[1].to_owned(),
            mode: mode(f[2])?,
            reduce: reduce(f[3])?,
            from: num(f[4])?,
            step: num(f[5])?,
            count: num(f[6])?,
            decoded: num(f[7])?,
            digest: hex(f[8])?,
        })),
        _ => Err(bad("unknown kind or field count")),
    }
}

/// Every line of a table.
pub fn parse_table(text: &str) -> Result<Vec<Line>, String> {
    text.lines().filter_map(|l| parse_line(l).transpose()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_both_kinds() {
        let t = "# header\nwhole\ta.j2k\tviewer\t3\t1529\t1\taef9eee61540d8b9\n\
                 cuts\ta.j2k\tplain\t0\t1\t13\t118\t107\t07945346760c3730\r\n\n";
        let lines = parse_table(t).unwrap();
        assert_eq!(
            lines,
            vec![
                Line::Whole {
                    file: "a.j2k".into(),
                    mode: Mode::Viewer,
                    reduce: 3,
                    bytes: 1529,
                    ok: true,
                    hash: 0xaef9_eee6_1540_d8b9
                },
                Line::Cuts {
                    file: "a.j2k".into(),
                    mode: Mode::Plain,
                    reduce: 0,
                    from: 1,
                    step: 13,
                    count: 118,
                    decoded: 107,
                    digest: 0x0794_5346_760c_3730
                },
            ]
        );
    }

    #[test]
    fn refuses_bad_lines() {
        assert!(parse_line("whole\ta\tplain\t0\t1\t2\tzz").is_err());
        assert!(parse_line("whole\ta\tnone\t0\t1\t1\t00").is_err());
        assert!(parse_line("cuts\ta\tplain\t0").is_err());
        assert!(parse_line("other\tx").is_err());
    }
}
