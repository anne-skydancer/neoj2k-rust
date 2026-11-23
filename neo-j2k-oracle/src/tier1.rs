// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The C++ Tier-1 as the twin's oracle (`shim/shim_t1.cpp`):
//!
//! - [`T1Decoder`]: the decoder's code-block, `t1_decode_cblk` and the store into a tile buffer;
//! - [`T1Encoder`]: the encoder's code-block, `enc_t1_encode` and its commit;
//! - [`mq_decode`], [`mq_decode_cases`]: the MQ decoder's decision (`opj_mqc_decode_macro`) on a
//!   byte string or on registers and a few bytes the caller gives, singly or in a batch;
//! - [`mq_encode`], [`mq_encode_cases`]: the MQ encoder (`MqEnc`), likewise.
//!
//! The contexts of the MQ entries are state indices 0 to 93 (`state * 2 + MPS`, OpenJPEG's order).

use std::ptr::NonNull;

use super::{ffi, init};

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

    /// The block's coefficients as the decoder writes them into a tile buffer (`width * height`
    /// samples, stride `width`), or `None` when the C++ refuses the block (bit-planes plus ROI shift
    /// at or above 31).
    pub fn decode(&mut self, block: &DecodeBlock<'_>) -> Option<Vec<i32>> {
        let (lens, passes): (Vec<u32>, Vec<u32>) = block.segments.iter().copied().unzip();
        assert_eq!(
            block.chunks.iter().map(|&c| c as usize).sum::<usize>(),
            block.bytes.len(),
            "the chunks' lengths add up to the bytes"
        );
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
            bytes: block.bytes.as_ptr(),
            chunk_len: block.chunks.as_ptr(),
            nchunks: block.chunks.len(),
            reversible: u32::from(block.reversible),
            stepsize: block.step_size,
        };
        let mut out = vec![0i32; block.width as usize * block.height as usize];
        // SAFETY: the handle is live; the request's arrays (segments, chunks, bytes) outlive the call, which
        // only reads them; out holds the width * height samples the shim writes.
        let ok =
            unsafe { ffi::nj2k_shim_t1_decode(self.raw.as_ptr(), &raw const request, out.as_mut_ptr(), out.len()) };
        (ok == 1).then_some(out)
    }
}

impl Drop for T1Decoder {
    fn drop(&mut self) {
        // SAFETY: the handle came from nj2k_shim_t1_new and is freed once.
        unsafe { ffi::nj2k_shim_t1_free(self.raw.as_ptr()) };
    }
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
    ok: u32,
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
    /// False when the C++ refuses the block (its output clamp).
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
    pub fn encode(&mut self, block: &EncodeBlock<'_>) -> EncodedBlock {
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
        let ok = unsafe {
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
        assert_eq!(ok, 1, "the C++ harness failed on an encoder block");
        bytes.truncate(result.byte_len as usize);
        EncodedBlock {
            ok: result.ok == 1,
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

/// Decodes `contexts.len()` decisions, decision `i` on `contexts[i]`. Starts as a segment does
/// (INITDEC on `data`, the standard's initial states) when `init` is `None`, else from the registers.
/// Returns the bits and the registers at the end.
#[must_use]
pub fn mq_decode(init_regs: Option<&MqDecoderRegs>, data: &[u8], contexts: &[u8]) -> (Vec<u8>, MqDecoderRegs) {
    init();
    let mut bits = vec![0u8; contexts.len()];
    let mut out = MqDecoderRegs::default();
    let init_ptr = init_regs.map_or(std::ptr::null(), std::ptr::from_ref);
    // SAFETY: the arrays are live and read only (bits is writable, as long as contexts); init_ptr is null or
    // a live record; out is a writable record.
    let ok = unsafe {
        ffi::nj2k_shim_mq_dec_run(
            init_ptr,
            data.as_ptr(),
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
    pub a: u32,
    pub c: u32,
    pub ct: u32,
    /// The context's state index.
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
pub fn mq_decode_cases(cases: &[MqDecodeCase], results: &mut Vec<MqDecodeResult>) {
    init();
    results.clear();
    results.resize(cases.len(), MqDecodeResult::default());
    // SAFETY: cases is read; results holds one record per case.
    unsafe { ffi::nj2k_shim_mq_dec_cases(cases.as_ptr(), cases.len(), results.as_mut_ptr()) };
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

/// Codes decisions `ctx * 2 + bit`, then flushes when `flush`, into `buf` (`buf[0]` is the fake first
/// byte, 0 for a fresh start; nothing past `buf[limit]` is written). Starts a fresh coder when `init` is
/// `None`. Returns the registers at the end and after each decision.
pub fn mq_encode(
    init_regs: Option<&MqEncoderRegs>,
    ctxbit: &[u8],
    flush: bool,
    buf: &mut [u8],
    limit: usize,
) -> (MqEncoderRegs, Vec<MqEncoderRegs>) {
    init();
    let mut out = MqEncoderRegs::default();
    let mut trace = vec![MqEncoderRegs::default(); ctxbit.len()];
    let init_ptr = init_regs.map_or(std::ptr::null(), std::ptr::from_ref);
    // SAFETY: the arrays are live; buf is writable for its length, trace has one record per decision; init_ptr
    // is null or a live record; out is a writable record.
    let ok = unsafe {
        ffi::nj2k_shim_mq_enc_run(
            init_ptr,
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
    pub a: u32,
    pub c: u32,
    pub ct: u32,
    pub state: u32,
    pub bit: u32,
    pub prev: u32,
    pub limit_off: u32,
}

/// What a decision left.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MqEncodeResult {
    pub a: u32,
    pub c: u32,
    pub ct: u32,
    /// The context's state index after the decision.
    pub state: u32,
    pub bp: u32,
    pub overflow: u32,
    pub bytes: [u8; 8],
}

/// The decisions of `cases`, one each, in the C++.
pub fn mq_encode_cases(cases: &[MqEncodeCase], results: &mut Vec<MqEncodeResult>) {
    init();
    results.clear();
    results.resize(cases.len(), MqEncodeResult::default());
    // SAFETY: cases is read; results holds one record per case.
    unsafe { ffi::nj2k_shim_mq_enc_cases(cases.as_ptr(), cases.len(), results.as_mut_ptr()) };
}
