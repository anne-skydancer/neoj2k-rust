// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! `sizeof` and `alignof` of the C++ types the arenas hold. The C++ refuses at the memory limit by
//! the bytes its arenas reserve, and those depend on these layouts; the twin's own types are laid
//! out differently (indices, not pointers), so its ledger charges the C++'s sizes, from this table.
//! A test compares the table with the C++'s on every CI platform, through the oracle.
//!
//! Types the C++ defines inside a `.cpp` file (`decode.cpp`'s `TileWalk`, `t2.cpp`'s
//! `PiResolution` and `PiComp`, `encode.cpp`'s `T1Job`) are added by the stage that transcribes
//! their unit, with the C++ accessor that stage needs.

/// One C++ type of the table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code, reason = "the ledgers of stages R1 and R2 charge them")]
pub(crate) enum CType {
    U8,
    I16,
    U32,
    U64,
    Bool,
    ImageComp,
    SizComponent,
    CodingStyle,
    Quantization,
    PocEntry,
    TlmEntry,
    PackedChunk,
    TileOverrides,
    TileDirectoryPage,
    MainHeader,
    TileComponentParams,
    Jp2Header,
    Jp2CmapEntry,
    Jp2CdefEntry,
    OutComp,
    ResRect,
    TileComp,
    Resolution,
    Precinct,
    CodeBlock,
    TagNode,
    Segment,
    Chunk,
    T1,
    EncComp,
    EncRes,
    EncCblk,
    EncPass,
    EncLayer,
    EncTagNode,
}

/// The C++ name, `sizeof` and `alignof` of every [`CType`], in its order.
const TABLE: [(&str, usize, usize); 35] = [
    ("u8", 1, 1),
    ("i16", 2, 2),
    ("u32", 4, 4),
    ("u64", 8, 8),
    ("bool", 1, 1),
    ("ImageComp", 64, 8),
    ("SizComponent", 4, 1),
    ("CodingStyle", 39, 1),
    ("Quantization", 198, 2),
    ("PocEntry", 10, 2),
    ("TlmEntry", 8, 4),
    ("PackedChunk", 16, 8),
    ("TileOverrides", 144, 8),
    ("TileDirectoryPage", 1024, 8),
    ("MainHeader", 472, 8),
    ("TileComponentParams", 32, 8),
    ("Jp2Header", 152, 8),
    ("Jp2CmapEntry", 4, 2),
    ("Jp2CdefEntry", 6, 2),
    ("OutComp", 56, 8),
    ("ResRect", 16, 4),
    ("TileComp", 88, 8),
    ("Resolution", 168, 8),
    ("Precinct", 72, 8),
    ("CodeBlock", 56, 8),
    ("TagNode", 8, 4),
    ("Segment", 24, 4),
    ("Chunk", 16, 8),
    ("T1", 192, 64),
    ("EncComp", 40, 8),
    ("EncRes", 312, 8),
    ("EncCblk", 64, 8),
    ("EncPass", 24, 8),
    ("EncLayer", 24, 8),
    ("EncTagNode", 24, 8),
];

// one row per variant, in order
const _: () = assert!(CType::EncTagNode as usize + 1 == TABLE.len());

#[allow(dead_code, reason = "the ledgers of stages R1 and R2 charge them")]
impl CType {
    /// The C++ type's name.
    pub(crate) const fn name(self) -> &'static str {
        TABLE[self as usize].0
    }
    /// `sizeof` of the C++ type: the bytes an element takes in an arena.
    pub(crate) const fn csize(self) -> usize {
        TABLE[self as usize].1
    }
    /// `alignof` of the C++ type; the arenas align every array to at least 16 bytes.
    pub(crate) const fn calign(self) -> usize {
        TABLE[self as usize].2
    }
}

#[cfg(test)]
mod table_tests {
    use super::CType;

    #[test]
    fn variants_name_their_rows() {
        assert_eq!(CType::U8.name(), "u8");
        assert_eq!(
            (
                CType::CodeBlock.name(),
                CType::CodeBlock.csize(),
                CType::CodeBlock.calign()
            ),
            ("CodeBlock", 56, 8)
        );
        assert_eq!(CType::EncTagNode.name(), "EncTagNode");
    }
}

#[cfg(all(test, not(miri)))]
mod tests {
    use super::TABLE;

    /// The table equals the C++'s on this platform, row for row (the R0 gate).
    #[test]
    fn equals_the_cpp_size_table() {
        let cpp = neo_j2k_oracle::layout_table();
        let rows: Vec<(String, u64, u64)> = cpp.iter().map(|r| (r.name.clone(), r.size, r.align)).collect();
        let ours: Vec<(String, u64, u64)> = TABLE
            .iter()
            .map(|&(n, s, a)| (n.to_owned(), s as u64, a as u64))
            .collect();
        if rows != ours {
            for r in &rows {
                eprintln!("    (\"{}\", {}, {}),", r.0, r.1, r.2);
            }
        }
        assert_eq!(
            rows, ours,
            "the C++ size table changed: update mem/layout.rs (the rows above)"
        );
    }
}
