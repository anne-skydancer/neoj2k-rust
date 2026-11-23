// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The fuzz target `t1`: the decoder's Tier-1 on a code-block of any bytes, against the C++'s. The input becomes a
//! block, which Rust's Tier-1 (the `bench-internals` entry point) and the C++'s (`t1_decode_cblk` and the store,
//! through the oracle) decode; the refusal of a block whose bit-planes plus ROI shift reach 31 and every coefficient
//! must be the same, 5/3 and 9/7 store alike, with no panic and no hang. The input format is the C++
//! `fuzz_t1`'s, so its seeds (`seeds/t1`, taken from the committed vectors) are this target's:
//!
//! - bytes 0-1: the shape; bit 7 of byte 0 set picks one of 16 long shapes (1024 x 4, 4 x 1024, ...) by its low 4
//!   bits, else the width is `1 + byte 0 % 64` and the height `1 + byte 1 % 64`;
//! - byte 2: the code-block style (low 6 bits) and the number of chunks the bytes arrive in less one (bits 6-7);
//! - byte 3: the orientation (bits 0-1) and the ROI shift (bits 2-7, 0-63 taken modulo 32);
//! - byte 4: the bit-planes (modulo 36: from 31 on they are refused);
//! - byte 5: the number of segments (modulo 112; the most a code-block can have is 109);
//! - then two bytes a segment (its length, its passes), then the code-block's bytes.
//!
//! A segment's length is as the byte says: past the bytes that are there, both decoders take the bytes present. A
//! segment's passes are as Tier 2 gives them: at most what its style allows (one a segment with TERMALL; 10, 2, 1, 2,
//! 1, ... with BYPASS; else 109), and all of them but in the last segment.

use std::cell::RefCell;

use neo_j2k::bench::{DecodeBlock, DecodeWorkspace};
use neo_j2k_oracle::tier1::{DecodeBlock as CppBlock, T1Decoder};
use neo_j2k_testkit::generate::{LONG_SHAPES, split_chunks};

thread_local! {
    // both kept from input to input, as a worker keeps its scratch
    static SCRATCH: RefCell<Option<(T1Decoder, DecodeWorkspace)>> = const { RefCell::new(None) };
}

pub fn t1(data: &[u8]) {
    if data.len() < 6 {
        return;
    }
    let (width, height) = if data[0] & 0x80 != 0 {
        LONG_SHAPES[usize::from(data[0] & 15)]
    } else {
        (1 + u32::from(data[0]) % 64, 1 + u32::from(data[1]) % 64)
    };
    let style = u32::from(data[2] & 63);
    let parts = 1 + u32::from(data[2] >> 6);
    let orient = u32::from(data[3] & 3);
    let roishift = u32::from(data[3] >> 2) % 32;
    let numbps = u32::from(data[4]) % 36;
    let nsegs = usize::from(data[5]) % 112;
    let mut at = 6;
    if data.len() < at + 2 * nsegs {
        return;
    }
    let seg_bytes = &data[at..at + 2 * nsegs];
    at += 2 * nsegs;
    let bytes = &data[at..data.len().min(at + (1 << 20))];

    let segments: Vec<(u32, u32)> = (0..nsegs)
        .map(|i| {
            let most = if style & 0x04 != 0 {
                1
            } else if style & 0x01 != 0 {
                match i {
                    0 => 10,
                    _ if i % 2 == 1 => 2,
                    _ => 1,
                }
            } else {
                109
            };
            let passes = if i + 1 < nsegs {
                most
            } else {
                u32::from(seg_bytes[2 * i + 1]).min(most)
            };
            (u32::from(seg_bytes[2 * i]), passes)
        })
        .collect();
    let chunks = split_chunks(bytes.len(), parts);
    let rust_block = DecodeBlock::new(
        (width, height),
        (numbps, orient, roishift, style),
        &segments,
        bytes.to_vec(),
    )
    .split(parts);

    SCRATCH.with(|cell| {
        let mut slot = cell.borrow_mut();
        let (cpp, rust) = slot.get_or_insert_with(|| (T1Decoder::new(), DecodeWorkspace::new()));
        for (reversible, step_size) in [(true, 1.0f32), (false, 1.7)] {
            let want = cpp.decode(&CppBlock {
                width,
                height,
                numbps,
                orient,
                roishift,
                cblksty: style,
                segments: &segments,
                bytes,
                chunks: &chunks,
                reversible,
                step_size,
                stride: width,
                offset: 0,
            });
            let mut got = vec![neo_j2k_oracle::tier1::FILL; (width * height) as usize + neo_j2k_oracle::tier1::TAIL];
            let ok = rust.decode(&rust_block, reversible, step_size, &mut got);
            // a refusal is the bit-planes plus ROI shift rule and nothing else
            let refused = i32::from_ne_bytes(numbps.wrapping_add(roishift).to_ne_bytes()) >= 31;
            assert_eq!(
                want.is_none(),
                refused,
                "t1: the C++'s refusal is not the bit-plane rule ({width} x {height})"
            );
            match want {
                None => assert!(
                    !ok,
                    "t1: the C++ refuses the block and Rust decodes it ({width} x {height}, style {style:02x})"
                ),
                Some(want) => {
                    assert!(
                        ok,
                        "t1: Rust refuses a block the C++ decodes ({width} x {height}, style {style:02x})"
                    );
                    assert!(
                        got == want,
                        "t1: the coefficients differ ({width} x {height}, style {style:02x}, {})",
                        if reversible { "5/3" } else { "9/7" }
                    );
                }
            }
        }
    });
}
