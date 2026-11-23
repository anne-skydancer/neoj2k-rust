// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The C++ repository's Tier-1 code-block dumps ("MQNG" files: `tests/vectors/t1`, the decoder
//! bench's `tests/vectors/t1_bench`), read from a checkout of the C++ and never copied here.
//!
//! The format, all little-endian `u32` but the code-block bytes: the magic `MQNG`, the block
//! count, then for each block `w h numbps orient roishift cblksty`, the segment count, `len
//! passes` for each segment, the byte count and the bytes (`tests/vectors/t1/README.md`).

use std::path::{Path, PathBuf};

/// One code-block as the decoder's Tier-1 received it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VectorBlock {
    pub width: u32,
    pub height: u32,
    pub numbps: u32,
    pub orient: u32,
    pub roishift: u32,
    pub cblksty: u32,
    /// `(len, passes)` of each codeword segment.
    pub segments: Vec<(u32, u32)>,
    /// The chunks' bytes, gathered.
    pub bytes: Vec<u8>,
}

/// Why a dump does not read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MqngError {
    /// The file does not begin with `MQNG`.
    Magic,
    /// The dump ends inside a field or a block.
    Truncated,
    /// A block the decoder's header could not hold (a side of 0 or over 1,024, over 4,096 samples) or a
    /// byte count over 16 MiB.
    Block(usize),
}

impl std::fmt::Display for MqngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Magic => f.write_str("not an MQNG file"),
            Self::Truncated => f.write_str("the MQNG file is cut short"),
            Self::Block(i) => write!(f, "MQNG block {i} is out of range"),
        }
    }
}

impl std::error::Error for MqngError {}

struct Reader<'a>(&'a [u8]);

impl Reader<'_> {
    fn bytes(&mut self, n: usize) -> Result<&[u8], MqngError> {
        if self.0.len() < n {
            return Err(MqngError::Truncated);
        }
        let (head, rest) = self.0.split_at(n);
        self.0 = rest;
        Ok(head)
    }

    fn u32(&mut self) -> Result<u32, MqngError> {
        let b = self.bytes(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
}

/// The blocks of a dump held in memory.
///
/// # Errors
/// [`MqngError`] when the bytes are not a well-formed dump.
pub fn parse(data: &[u8]) -> Result<Vec<VectorBlock>, MqngError> {
    let mut r = Reader(data);
    if r.bytes(4)? != b"MQNG" {
        return Err(MqngError::Magic);
    }
    let count = r.u32()? as usize;
    // a block is at least 32 bytes: never reserve more than the data can hold
    let mut blocks = Vec::with_capacity(count.min(data.len() / 32));
    for index in 0..count {
        let (width, height, numbps, orient, roishift, cblksty) =
            (r.u32()?, r.u32()?, r.u32()?, r.u32()?, r.u32()?, r.u32()?);
        let nsegs = r.u32()? as usize;
        let mut segments = Vec::with_capacity(nsegs.min(r.0.len() / 8));
        for _ in 0..nsegs {
            segments.push((r.u32()?, r.u32()?));
        }
        let nbytes = r.u32()? as usize;
        let valid = (1..=1024).contains(&width)
            && (1..=1024).contains(&height)
            && u64::from(width) * u64::from(height) <= 4096
            && nbytes < 1 << 24;
        if !valid {
            return Err(MqngError::Block(index));
        }
        let bytes = r.bytes(nbytes)?.to_vec();
        blocks.push(VectorBlock {
            width,
            height,
            numbps,
            orient,
            roishift,
            cblksty,
            segments,
            bytes,
        });
    }
    Ok(blocks)
}

/// The blocks of a dump file.
///
/// # Errors
/// The I/O error of reading it, or [`MqngError`] (as `InvalidData`).
pub fn read(path: &Path) -> std::io::Result<Vec<VectorBlock>> {
    let data = std::fs::read(path)?;
    parse(&data).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{}: {e}", path.display())))
}

/// The C++ checkout the vectors are read from: `NJ2K_CPP_SOURCE`. `None` when it is not set or not a
/// folder.
#[must_use]
pub fn cpp_source() -> Option<PathBuf> {
    let dir = PathBuf::from(std::env::var_os("NJ2K_CPP_SOURCE")?);
    dir.is_dir().then_some(dir)
}

/// The C++ checkout for a test that reads the vectors: [`cpp_source`]. When `NJ2K_CPP_SOURCE` is not set, `None` after
/// a note on stderr so the test can return, unless `CI` is set (a CI job that forgot it fails).
///
/// # Panics
/// When `NJ2K_CPP_SOURCE` is set but names no folder (a wrong path must not skip the tests), or when `CI` is set and
/// it is not set.
#[must_use]
pub fn require_cpp_source() -> Option<PathBuf> {
    match std::env::var_os("NJ2K_CPP_SOURCE") {
        Some(dir) => {
            let dir = PathBuf::from(dir);
            assert!(
                dir.is_dir(),
                "NJ2K_CPP_SOURCE is set but {} is not a folder",
                dir.display()
            );
            Some(dir)
        }
        None => {
            assert!(
                std::env::var_os("CI").is_none(),
                "NJ2K_CPP_SOURCE must name the C++ checkout (the Tier-1 vectors are read from it)"
            );
            eprintln!("skipped: set NJ2K_CPP_SOURCE to the C++ neo-j2k v0.1.3 checkout to read its Tier-1 vectors");
            None
        }
    }
}

/// The committed decoder vectors of the C++ checkout (`tests/vectors/t1`, 3,927 blocks), by file name.
///
/// # Errors
/// The I/O or format error of the first file that does not read.
pub fn committed_vectors(cpp: &Path) -> std::io::Result<Vec<(String, Vec<VectorBlock>)>> {
    dumps_in(&cpp.join("tests/vectors/t1"))
}

/// The six bench dumps of the C++ checkout (`tests/vectors/t1_bench`), by file name.
///
/// # Errors
/// The I/O or format error of the first file that does not read.
pub fn bench_dumps(cpp: &Path) -> std::io::Result<Vec<(String, Vec<VectorBlock>)>> {
    dumps_in(&cpp.join("tests/vectors/t1_bench"))
}

/// Every `.blk` dump of a folder, by file name, in name order.
///
/// # Errors
/// The I/O or format error of the first file that does not read.
pub fn dumps_in(dir: &Path) -> std::io::Result<Vec<(String, Vec<VectorBlock>)>> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "blk"))
        .collect();
    files.sort();
    files
        .into_iter()
        .map(|p| {
            let name = p
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            read(&p).map(|blocks| (name, blocks))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A block to dump: its size, its segments and its bytes.
    type Spec<'a> = (u32, u32, &'a [(u32, u32)], &'a [u8]);

    fn count(n: usize) -> u32 {
        u32::try_from(n).unwrap()
    }

    fn dump(blocks: &[Spec<'_>]) -> Vec<u8> {
        let mut out = b"MQNG".to_vec();
        out.extend(count(blocks.len()).to_le_bytes());
        for &(w, h, segs, bytes) in blocks {
            for v in [w, h, 9, 1, 0, 0x3f, count(segs.len())] {
                out.extend(v.to_le_bytes());
            }
            for &(len, passes) in segs {
                out.extend(len.to_le_bytes());
                out.extend(passes.to_le_bytes());
            }
            out.extend(count(bytes.len()).to_le_bytes());
            out.extend(bytes);
        }
        out
    }

    #[test]
    fn a_dump_reads_back_field_for_field() {
        let data = dump(&[(64, 64, &[(3, 7), (2, 1)], &[1, 2, 3, 4, 5]), (1, 1024, &[], &[])]);
        let blocks = parse(&data).unwrap();
        assert_eq!(blocks.len(), 2);
        assert_eq!(
            blocks[0],
            VectorBlock {
                width: 64,
                height: 64,
                numbps: 9,
                orient: 1,
                roishift: 0,
                cblksty: 0x3f,
                segments: vec![(3, 7), (2, 1)],
                bytes: vec![1, 2, 3, 4, 5],
            }
        );
        assert_eq!((blocks[1].width, blocks[1].height), (1, 1024));
        assert!(blocks[1].segments.is_empty() && blocks[1].bytes.is_empty());
    }

    #[test]
    fn a_bad_dump_is_refused() {
        assert_eq!(parse(b"MQNX\0\0\0\0"), Err(MqngError::Magic));
        assert_eq!(parse(b"MQN"), Err(MqngError::Truncated));
        let good = dump(&[(8, 8, &[(2, 1)], &[7, 8])]);
        for cut in 4..good.len() {
            assert_eq!(parse(&good[..cut]), Err(MqngError::Truncated), "cut at {cut}");
        }
        assert_eq!(parse(&dump(&[(0, 8, &[], &[])])), Err(MqngError::Block(0)));
        assert_eq!(
            parse(&dump(&[(8, 8, &[], &[]), (65, 64, &[], &[])])),
            Err(MqngError::Block(1))
        );
        // a count the data cannot hold does not reserve it
        let mut huge = b"MQNG".to_vec();
        huge.extend(u32::MAX.to_le_bytes());
        assert_eq!(parse(&huge), Err(MqngError::Truncated));
    }
}
