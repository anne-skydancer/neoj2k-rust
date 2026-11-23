// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The C++ Tier-1 as the twin's oracle (`shim/shim_t1.cpp`):
//!
//! - [`T1Decoder`]: the decoder's code-block, `t1_decode_cblk` and the store into a tile buffer; [`store`], the store
//!   alone;
//! - [`T1Encoder`]: the encoder's code-block, `enc_t1_encode` and its commit;
//! - [`mq_decode`], [`mq_decode_cases`]: the MQ decoder's decision (`opj_mqc_decode_macro`) on a byte string from a
//!   segment's start, or on registers and a few bytes the caller gives, in a batch;
//! - [`mq_encode`], [`mq_encode_cases`], [`mq_flush_cases`]: the MQ encoder (`MqEnc`), likewise, and its flush.
//!
//! The contexts of the MQ entries are state indices 0 to 93 (`state * 2 + MPS`, OpenJPEG's order). The shim refuses
//! what the C++ is not meant to take; these wrappers check the same first and panic (a request of the test's own
//! making is a test bug), and only the C++'s own refusal of a code-block is a result.

use std::mem::size_of;
use std::ptr::NonNull;

use super::{ffi, init};

/// What the shim returns for a code-block entry (`NJ2K_SHIM_T1_*`).
const INVALID: u32 = 0;
const DONE: u32 = 1;
const REFUSED: u32 = 2;

/// The sample the decoder's output buffers are filled with before a block is stored into them: what lies between
/// the rows of a tile buffer must stay as it is.
pub const FILL: i32 = 0x5a5a_5a5a;
/// The samples after the last row of a block's output buffer.
pub const TAIL: usize = 3;

/// One code-block as the decoder's Tier-1 receives it, with the store that follows it.
#[derive(Clone, Copy, Debug)]
pub struct DecodeBlock<'a> {
    pub width: u32,
    pub height: u32,
    pub numbps: u32,
    pub orient: u32,
    pub roishift: u32,
    pub cblksty: u32,
    /// `(len, passes)` of each codeword segment.
    pub segments: &'a [(u32, u32)],
    /// The chunks' bytes, gathered.
    pub bytes: &'a [u8],
    /// The lengths the bytes arrive in; their sum is `bytes.len()`. None: no data.
    pub chunks: &'a [u32],
    /// The store: 5/3 halves the coefficients, 9/7 multiplies them by `0.5 * step_size`.
    pub reversible: bool,
    pub step_size: f32,
    /// The tile buffer's row stride in samples, at least the width.
    pub stride: u32,
    /// Where the block's first sample lies in the tile buffer.
    pub offset: u32,
}

impl DecodeBlock<'_> {
    /// The samples of the tile buffer the block is stored into (`FILL` between the rows and `TAIL` after the last).
    #[must_use]
    pub fn buffer_len(&self) -> usize {
        self.offset as usize + (self.height as usize - 1) * self.stride as usize + self.width as usize + TAIL
    }
}

#[repr(C)]
pub(crate) struct RawDecodeRequest {
    w: u32,
    h: u32,
    numbps: u32,
    orient: u32,
    roishift: u32,
    cblksty: u32,
    seg_len: *const u32,
    seg_passes: *const u32,
    nsegs: usize,
    bytes: *const u8,
    chunk_len: *const u32,
    nchunks: usize,
    reversible: u32,
    stepsize: f32,
    out_stride: u32,
    out_offset: u32,
}

/// The C++ decoder's Tier-1 scratch, kept from block to block as a worker keeps it.
#[derive(Debug)]
pub struct T1Decoder {
    raw: NonNull<ffi::T1>,
}

// SAFETY: the handle is a C++ object without thread affinity; every call takes `&mut self`.
unsafe impl Send for T1Decoder {}

impl Default for T1Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl T1Decoder {
    /// A decoder scratch.
    #[must_use]
    pub fn new() -> Self {
        init();
        // SAFETY: no arguments; the result is a new handle or null.
        let raw = unsafe { ffi::nj2k_shim_t1_new() };
        Self {
            raw: NonNull::new(raw).expect("the shim could not make a Tier-1 scratch"),
        }
    }

    /// The tile buffer the block is stored into (`buffer_len` samples, `FILL` where the block does not write), or
    /// `None` when the C++ refuses the block (bit-planes plus ROI shift at or above 31) or the block is one the header
    /// cannot hold (a side of 0 or above 1,024, above 4,096 samples; nothing is allocated for it).
    ///
    /// # Panics
    /// On a request the C++ is not meant to take (an orientation above 3, a style above 63, a stride under the width,
    /// chunks that do not add up to the bytes) and when the harness fails.
    pub fn decode(&mut self, block: &DecodeBlock<'_>) -> Option<Vec<i32>> {
        if !(1..=1024).contains(&block.width)
            || !(1..=1024).contains(&block.height)
            || u64::from(block.width) * u64::from(block.height) > 4096
        {
            return None;
        }
        assert!(
            block.orient <= 3 && block.cblksty <= 63,
            "orientation and style of a code-block"
        );
        assert!(block.stride >= block.width, "a stride under the width");
        assert!(
            block.segments.len() <= 4096 && block.chunks.len() <= 4096,
            "too many segments or chunks"
        );
        assert_eq!(
            block.chunks.iter().map(|&c| c as usize).sum::<usize>(),
            block.bytes.len(),
            "the chunks' lengths add up to the bytes"
        );
        let (lens, passes): (Vec<u32>, Vec<u32>) = block.segments.iter().copied().unzip();
        let request = RawDecodeRequest {
            w: block.width,
            h: block.height,
            numbps: block.numbps,
            orient: block.orient,
            roishift: block.roishift,
            cblksty: block.cblksty,
            seg_len: lens.as_ptr(),
            seg_passes: passes.as_ptr(),
            nsegs: lens.len(),
            bytes: if block.bytes.is_empty() {
                std::ptr::null()
            } else {
                block.bytes.as_ptr()
            },
            chunk_len: block.chunks.as_ptr(),
            nchunks: block.chunks.len(),
            reversible: u32::from(block.reversible),
            stepsize: block.step_size,
            out_stride: block.stride,
            out_offset: block.offset,
        };
        let mut out = vec![FILL; block.buffer_len()];
        // SAFETY: the handle is live; the request's arrays (segments, chunks, bytes) outlive the call, which
        // only reads them; out holds the tile buffer the shim writes into, at least offset + (h - 1) * stride + w.
        let code =
            unsafe { ffi::nj2k_shim_t1_decode(self.raw.as_ptr(), &raw const request, out.as_mut_ptr(), out.len()) };
        assert_ne!(code, INVALID, "the shim refused a code-block request");
        (code == DONE).then_some(out)
    }
}

impl Drop for T1Decoder {
    fn drop(&mut self) {
        // SAFETY: the handle came from nj2k_shim_t1_new and is freed once.
        unsafe { ffi::nj2k_shim_t1_free(self.raw.as_ptr()) };
    }
}

/// The C++'s `t1_store` alone: the `width * height` coefficients (stride `width`) into a buffer of
/// `(height - 1) * stride + width + TAIL` samples (`FILL` where it does not write), the ROI shift undone, 5/3 halved
/// or 9/7 multiplied by `0.5 * step_size`.
///
/// # Panics
/// On a request the C++ is not meant to take.
#[must_use]
pub fn store(
    coefficients: &[i32],
    (width, height): (u32, u32),
    roishift: u32,
    reversible: bool,
    step_size: f32,
    stride: u32,
) -> Vec<i32> {
    init();
    assert!(
        (1..=1024).contains(&width) && (1..=1024).contains(&height) && u64::from(width) * u64::from(height) <= 4096,
        "block size"
    );
    assert!(stride >= width && coefficients.len() == width as usize * height as usize);
    let mut out = vec![FILL; (height as usize - 1) * stride as usize + width as usize + TAIL];
    // SAFETY: src holds width * height samples, dst the (height - 1) * stride + width the shim writes at most.
    let code = unsafe {
        ffi::nj2k_shim_t1_store(
            coefficients.as_ptr(),
            width,
            height,
            roishift,
            u32::from(reversible),
            step_size,
            out.as_mut_ptr(),
            out.len(),
            stride,
        )
    };
    assert_eq!(code, DONE, "the shim refused a store request");
    out
}

/// One code-block as the encoder's Tier-1 receives it.
#[derive(Clone, Copy, Debug)]
pub struct EncodeBlock<'a> {
    pub width: u32,
    pub height: u32,
    /// `width * height` signed fixed-point samples (six fractional bits), in the order `enc_t1_data` takes.
    pub samples: &'a [i32],
    pub orient: u32,
    pub component: u32,
    pub level: u32,
    pub reversible: bool,
    /// The colour transform's norms for the component (three components), or none.
    pub mct: bool,
    pub step_size: f64,
}

#[repr(C)]
pub(crate) struct RawEncodeRequest {
    w: u32,
    h: u32,
    samples: *const i32,
    orient: u32,
    compno: u32,
    level: u32,
    reversible: u32,
    mct: u32,
    stepsize: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct RawPass {
    rate: u32,
    len: u32,
    term: u32,
    distortion: f64,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct RawEncodeResult {
    numbps: u32,
    totalpasses: u32,
    byte_len: u32,
}

/// One pass of an encoded code-block.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EncodedPass {
    pub rate: u32,
    pub len: u32,
    pub terminated: bool,
    pub distortion: f64,
}

/// What the C++ encoder's Tier-1 makes of a block.
#[derive(Clone, Debug, PartialEq)]
pub struct EncodedBlock {
    /// False when the C++ refuses the block (its output clamp). The clamp is not reachable through a block (a block's
    /// buffer holds four bytes a sample and the coder writes about one and a half at most, as the C++ unit test
    /// notes), so the block tests expect it never.
    pub ok: bool,
    /// The block's bit-plane count, wrapped as the C++ has it for a block under one step.
    pub numbps: u32,
    pub passes: Vec<EncodedPass>,
    /// The committed bytes up to the last pass's rate.
    pub bytes: Vec<u8>,
}

/// The C++ encoder's Tier-1 scratch.
#[derive(Debug)]
pub struct T1Encoder {
    raw: NonNull<ffi::EncT1>,
}

// SAFETY: as T1Decoder.
unsafe impl Send for T1Encoder {}

impl Default for T1Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl T1Encoder {
    /// An encoder scratch.
    #[must_use]
    pub fn new() -> Self {
        init();
        // SAFETY: no arguments; the result is a new handle or null.
        let raw = unsafe { ffi::nj2k_shim_enc_t1_new() };
        Self {
            raw: NonNull::new(raw).expect("the shim could not make an encoder Tier-1 scratch"),
        }
    }

    /// Encodes one block.
    ///
    /// # Panics
    /// On a request the C++ is not meant to take (a side of 0 or above 64, an orientation above 3, samples that are not
    /// `width * height`) and when the harness fails.
    pub fn encode(&mut self, block: &EncodeBlock<'_>) -> EncodedBlock {
        assert!(
            (1..=64).contains(&block.width) && (1..=64).contains(&block.height) && block.orient <= 3,
            "encoder block size and orientation"
        );
        assert_eq!(block.samples.len(), block.width as usize * block.height as usize);
        let request = RawEncodeRequest {
            w: block.width,
            h: block.height,
            samples: block.samples.as_ptr(),
            orient: block.orient,
            compno: block.component,
            level: block.level,
            reversible: u32::from(block.reversible),
            mct: u32::from(block.mct),
            stepsize: block.step_size,
        };
        let mut result = RawEncodeResult::default();
        let mut passes = [RawPass::default(); 128];
        let mut bytes = vec![0u8; 64 * 64 * 4 + 256];
        // SAFETY: the handle is live; the request's samples outlive the call, which only reads them; the
        // output arrays have the capacities passed.
        let code = unsafe {
            ffi::nj2k_shim_enc_t1_encode(
                self.raw.as_ptr(),
                &raw const request,
                &raw mut result,
                passes.as_mut_ptr(),
                passes.len(),
                bytes.as_mut_ptr(),
                bytes.len(),
            )
        };
        assert_ne!(code, INVALID, "the shim refused an encoder block request");
        bytes.truncate(result.byte_len as usize);
        EncodedBlock {
            ok: code != REFUSED,
            numbps: result.numbps,
            passes: passes[..result.totalpasses as usize]
                .iter()
                .map(|p| EncodedPass {
                    rate: p.rate,
                    len: p.len,
                    terminated: p.term != 0,
                    distortion: p.distortion,
                })
                .collect(),
            bytes,
        }
    }
}

impl Drop for T1Encoder {
    fn drop(&mut self) {
        // SAFETY: the handle came from nj2k_shim_enc_t1_new and is freed once.
        unsafe { ffi::nj2k_shim_enc_t1_free(self.raw.as_ptr()) };
    }
}

/// The MQ decoder's registers and contexts.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MqDecoderRegs {
    pub a: u32,
    pub c: u32,
    pub ct: u32,
    /// The read position, a byte offset in the data.
    pub bp: u32,
    /// The 19 contexts' state indices.
    pub state: [u8; 19],
}

/// Decodes `contexts.len()` decisions, decision `i` on `contexts[i]` (below 19), from a segment's start: INITDEC on
/// `data`, the standard's initial states (the shim's restatement of the C++'s, see `shim_t1.cpp`). Returns the bits
/// and the registers at the end.
///
/// # Panics
/// On a context of 19 or more.
#[must_use]
pub fn mq_decode(data: &[u8], contexts: &[u8]) -> (Vec<u8>, MqDecoderRegs) {
    init();
    assert!(contexts.iter().all(|&c| c < 19), "a context below 19");
    let mut bits = vec![0u8; contexts.len()];
    let mut out = MqDecoderRegs::default();
    // SAFETY: the arrays are live and read only (bits is writable, as long as contexts); out is a writable record.
    let ok = unsafe {
        ffi::nj2k_shim_mq_dec_run(
            if data.is_empty() {
                std::ptr::null()
            } else {
                data.as_ptr()
            },
            data.len(),
            contexts.as_ptr(),
            contexts.len(),
            bits.as_mut_ptr(),
            &raw mut out,
        )
    };
    assert_eq!(ok, 1, "the C++ harness refused an MQ decoder run");
    (bits, out)
}

/// One decision on context 0 from registers and four bytes (then the sentinel), the read position at 0.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MqDecodeCase {
    /// The interval, 0x8000 to 0xFFFF.
    pub a: u32,
    pub c: u32,
    pub ct: u32,
    /// The context's state index, below 94.
    pub state: u32,
    pub bytes: [u8; 4],
}

/// What a decision left.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MqDecodeResult {
    pub bit: u32,
    pub a: u32,
    pub c: u32,
    pub ct: u32,
    pub bp: u32,
    /// The context's state index after the decision.
    pub state: u32,
}

/// The decisions of `cases`, one each, in the C++.
///
/// # Panics
/// On a case outside what the shim takes (`a` outside 0x8000 to 0xFFFF, a state of 94 or more).
pub fn mq_decode_cases(cases: &[MqDecodeCase], results: &mut Vec<MqDecodeResult>) {
    init();
    assert!(
        cases.iter().all(|k| (0x8000..=0xffff).contains(&k.a) && k.state < 94),
        "a decoder case: the interval 0x8000 to 0xFFFF, a state below 94"
    );
    results.clear();
    results.resize(cases.len(), MqDecodeResult::default());
    // SAFETY: cases is read; results holds one record per case.
    let ok = unsafe { ffi::nj2k_shim_mq_dec_cases(cases.as_ptr(), cases.len(), results.as_mut_ptr()) };
    assert_eq!(ok, 1, "the C++ harness refused the decoder cases");
}

/// The MQ encoder's registers and contexts.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MqEncoderRegs {
    pub a: u32,
    pub c: u32,
    pub ct: u32,
    /// The write position, a byte offset in the buffer (the fake first byte is at 0).
    pub bp: u32,
    pub overflow: u32,
    pub state: [u8; 19],
}

/// A fresh coder (the standard's initial states, as `enc_t1.cpp` sets them), the decisions `ctx * 2 + bit` (context
/// below 19), then the flush when `flush`, into `buf` (`buf[0]` is the fake first byte, 0; nothing past `buf[limit]`
/// is written; `1 <= limit < buf.len()`). Returns the registers at the end and after each decision.
///
/// # Panics
/// On a request outside that.
pub fn mq_encode(ctxbit: &[u8], flush: bool, buf: &mut [u8], limit: usize) -> (MqEncoderRegs, Vec<MqEncoderRegs>) {
    init();
    assert!(ctxbit.iter().all(|&d| d < 38), "a context below 19");
    assert!(limit >= 1 && limit < buf.len(), "1 <= limit < the buffer");
    assert_eq!(buf[0], 0, "the fake first byte");
    let mut out = MqEncoderRegs::default();
    let mut trace = vec![MqEncoderRegs::default(); ctxbit.len()];
    // SAFETY: the arrays are live; buf is writable for its length, trace has one record per decision; out is a
    // writable record.
    let ok = unsafe {
        ffi::nj2k_shim_mq_enc_run(
            ctxbit.as_ptr(),
            ctxbit.len(),
            u32::from(flush),
            buf.as_mut_ptr(),
            buf.len(),
            limit,
            &raw mut out,
            trace.as_mut_ptr(),
        )
    };
    assert_eq!(ok, 1, "the C++ harness refused an MQ encoder run");
    (out, trace)
}

/// One decision on context 0 into eight bytes of `prev`, the write position at 2, the limit at
/// `2 + limit_off`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MqEncodeCase {
    /// The interval, 0x8000 to 0xFFFF.
    pub a: u32,
    pub c: u32,
    /// The bit counter, 1 to 12.
    pub ct: u32,
    /// The context's state index, below 94.
    pub state: u32,
    pub bit: u32,
    /// The byte the buffer is filled with, below 256.
    pub prev: u32,
    /// The limit's offset from the write position, 0 to 5.
    pub limit_off: u32,
}

/// What a decision left.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MqEncodeResult {
    pub a: u32,
    pub c: u32,
    pub ct: u32,
    /// The context's state index after the decision (0 after a flush).
    pub state: u32,
    pub bp: u32,
    pub overflow: u32,
    pub bytes: [u8; 8],
}

/// The decisions of `cases`, one each, in the C++.
///
/// # Panics
/// On a case outside what the shim takes (the field docs have the ranges).
pub fn mq_encode_cases(cases: &[MqEncodeCase], results: &mut Vec<MqEncodeResult>) {
    init();
    assert!(
        cases.iter().all(|k| (0x8000..=0xffff).contains(&k.a)
            && (1..=12).contains(&k.ct)
            && k.state < 94
            && k.bit <= 1
            && k.prev <= 0xff
            && k.limit_off <= 5),
        "an encoder case outside the shim's ranges"
    );
    results.clear();
    results.resize(cases.len(), MqEncodeResult::default());
    // SAFETY: cases is read; results holds one record per case.
    let ok = unsafe { ffi::nj2k_shim_mq_enc_cases(cases.as_ptr(), cases.len(), results.as_mut_ptr()) };
    assert_eq!(ok, 1, "the C++ harness refused the encoder cases");
}

/// The flush from registers, in eight bytes of `prev`, the write position at 2, the limit at `2 + limit_off`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MqFlushCase {
    pub a: u32,
    pub c: u32,
    pub ct: u32,
    pub prev: u32,
    pub limit_off: u32,
}

/// The flushes of `cases`, one each, in the C++.
///
/// # Panics
/// On a case outside what the shim takes (as [`MqEncodeCase`]).
pub fn mq_flush_cases(cases: &[MqFlushCase], results: &mut Vec<MqEncodeResult>) {
    init();
    assert!(
        cases.iter().all(|k| (0x8000..=0xffff).contains(&k.a)
            && (1..=12).contains(&k.ct)
            && k.prev <= 0xff
            && k.limit_off <= 5),
        "a flush case outside the shim's ranges"
    );
    results.clear();
    results.resize(cases.len(), MqEncodeResult::default());
    // SAFETY: cases is read; results holds one record per case.
    let ok = unsafe { ffi::nj2k_shim_mq_flush_cases(cases.as_ptr(), cases.len(), results.as_mut_ptr()) };
    assert_eq!(ok, 1, "the C++ harness refused the flush cases");
}

// the records mirror the header's: the sizes the shim asserts on its side
const _: () = {
    assert!(size_of::<RawDecodeRequest>() == 88);
    assert!(size_of::<RawEncodeRequest>() == 48);
    assert!(size_of::<RawPass>() == 24);
    assert!(size_of::<RawEncodeResult>() == 12);
    assert!(size_of::<MqDecoderRegs>() == 36);
    assert!(size_of::<MqDecodeCase>() == 20);
    assert!(size_of::<MqDecodeResult>() == 24);
    assert!(size_of::<MqEncoderRegs>() == 40);
    assert!(size_of::<MqEncodeCase>() == 28);
    assert!(size_of::<MqEncodeResult>() == 32);
    assert!(size_of::<MqFlushCase>() == 20);
};
