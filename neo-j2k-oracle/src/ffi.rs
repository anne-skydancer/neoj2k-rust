// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The raw bindings of `shim/nj2k_rust_shim.h` and the few public `nj2k_*`
//! functions the wrapper calls. Written by hand from the header; `lib.rs` checks the ABI version
//! before any other call. Nothing outside this crate sees these.

use std::ffi::{c_char, c_void};

use super::tier1::{
    MqDecodeCase, MqDecodeResult, MqDecoderRegs, MqEncodeCase, MqEncodeResult, MqEncoderRegs, RawDecodeRequest,
    RawEncodeRequest, RawEncodeResult, RawPass,
};
use super::{Limits, kernels::WaveletRect};

/// `nj2k_codec_t`, opaque.
#[repr(C)]
pub(crate) struct Codec {
    _private: [u8; 0],
}
/// `nj2k_shim_backend_t`, opaque.
#[repr(C)]
pub(crate) struct Backend {
    _private: [u8; 0],
}
/// `nj2k_shim_outcome_t`, opaque.
#[repr(C)]
pub(crate) struct Outcome {
    _private: [u8; 0],
}
/// `nj2k_shim_pgx_t`, opaque.
#[repr(C)]
pub(crate) struct Pgx {
    _private: [u8; 0],
}

/// `nj2k_shim_header_t`, opaque.
#[repr(C)]
pub(crate) struct Header {
    _private: [u8; 0],
}

/// `nj2k_shim_t1_t`, opaque.
#[repr(C)]
pub(crate) struct T1 {
    _private: [u8; 0],
}

/// `nj2k_shim_enc_t1_t`, opaque.
#[repr(C)]
pub(crate) struct EncT1 {
    _private: [u8; 0],
}

/// `nj2k_shim_pool_t`, opaque.
#[repr(C)]
pub(crate) struct Pool {
    _private: [u8; 0],
}

pub(crate) const ABI_VERSION: u32 = 8;

/// `nj2k_shim_codestream_t`, opaque.
#[repr(C)]
pub(crate) struct Codestream {
    _private: [u8; 0],
}

#[repr(C)]
pub(crate) struct EncodeRequest {
    pub(crate) pixels: *const u8,
    pub(crate) size: usize,
    pub(crate) stride: usize,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) components: u32,
    pub(crate) flip_vertical: u32,
    pub(crate) reversible: u32,
    pub(crate) resolutions: u32,
    pub(crate) mct: u32,
    pub(crate) max_codestream_size: u32,
    pub(crate) rates: *const f32,
    pub(crate) rate_count: usize,
    pub(crate) comment: *const u8,
    pub(crate) comment_len: usize,
}

#[repr(C)]
pub(crate) struct EncodeUpdates {
    pub(crate) max_codestream_size: u32,
    pub(crate) layer_count: u32,
    pub(crate) ratios: [f32; 100],
}

#[repr(C)]
pub(crate) struct EncodeTraceRequest {
    pub(crate) buffer_size: usize,
    pub(crate) max_write: usize,
    pub(crate) stop_at: usize,
    pub(crate) fail_with_error: u32,
    pub(crate) extra_options: *const *const c_char,
}

#[repr(C)]
pub(crate) struct Layout {
    pub(crate) name: *const c_char,
    pub(crate) size: u64,
    pub(crate) align: u64,
}

#[repr(C)]
pub(crate) struct Request {
    pub(crate) data: *const u8,
    pub(crate) size: u64,
    pub(crate) reduce: i32,
    pub(crate) layers: i32,
    pub(crate) mode: u32,
    pub(crate) jp2: u32,
    pub(crate) reduce_after_header: u32,
    pub(crate) limits: u32,
    pub(crate) max_memory: u64,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct OutcomeInfo {
    pub(crate) ok: u32,
    pub(crate) x0: u32,
    pub(crate) y0: u32,
    pub(crate) x1: u32,
    pub(crate) y1: u32,
    pub(crate) numcomps: u32,
    pub(crate) color_space: i32,
    pub(crate) warnings: u32,
    pub(crate) errors: u32,
    pub(crate) hash: u64,
    pub(crate) u8_hash: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct Component {
    pub(crate) w: u32,
    pub(crate) h: u32,
    pub(crate) dx: u32,
    pub(crate) dy: u32,
    pub(crate) x0: u32,
    pub(crate) y0: u32,
    pub(crate) prec: u32,
    pub(crate) sgnd: u32,
    pub(crate) factor: u32,
    pub(crate) resno_decoded: u32,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct HeaderInfo {
    pub(crate) x0: u32,
    pub(crate) y0: u32,
    pub(crate) x1: u32,
    pub(crate) y1: u32,
    pub(crate) numcomps: u32,
    pub(crate) color_space: i32,
    pub(crate) tx0: u32,
    pub(crate) ty0: u32,
    pub(crate) tdx: u32,
    pub(crate) tdy: u32,
    pub(crate) tw: u32,
    pub(crate) th: u32,
    pub(crate) numlayers: u32,
    pub(crate) min_resolutions: u32,
    pub(crate) progression: u32,
    pub(crate) reduce_status: i32,
    pub(crate) dimensions_status: i32,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct Ledger {
    pub(crate) header_reserved: u64,
    pub(crate) tile_reserved: u64,
    pub(crate) work_reserved: u64,
    pub(crate) work_high_water: u64,
    pub(crate) kept_bytes: u64,
}

pub(crate) const BACKEND_ORACLE: u32 = 0;
pub(crate) const BACKEND_NEOJ2K: u32 = 1;
pub(crate) const BACKEND_NEOJ2K_MEMORY: u32 = 2;
pub(crate) const TEXT_STAGE: u32 = 0;
pub(crate) const TEXT_LAST_ERROR: u32 = 1;
pub(crate) const TEXT_WARNING_KINDS: u32 = 2;
/// `NJ2K_CODEC_J2K`, `NJ2K_CODEC_JP2` (`neoj2k.h`)
pub(crate) const CODEC_J2K: u32 = 0;
pub(crate) const CODEC_JP2: u32 = 2;

unsafe extern "C" {
    pub(crate) fn nj2k_shim_abi_version() -> u32;
    pub(crate) fn nj2k_shim_cpp_version() -> u32;
    pub(crate) fn nj2k_shim_ready() -> u32;
    pub(crate) fn nj2k_shim_console_reports();
    pub(crate) fn nj2k_shim_layout_table(out: *mut *const Layout) -> usize;

    pub(crate) fn nj2k_shim_backend_new(kind: u32) -> *mut Backend;
    pub(crate) fn nj2k_shim_backend_free(backend: *mut Backend);
    pub(crate) fn nj2k_shim_decode(backend: *mut Backend, request: *const Request) -> *mut Outcome;
    pub(crate) fn nj2k_shim_outcome_free(outcome: *mut Outcome);
    pub(crate) fn nj2k_shim_outcome_info(outcome: *const Outcome, info: *mut OutcomeInfo);
    pub(crate) fn nj2k_shim_outcome_component(
        outcome: *const Outcome,
        c: u32,
        comp: *mut Component,
        samples: *mut *const i32,
        count: *mut u64,
    ) -> u32;
    pub(crate) fn nj2k_shim_outcome_text(outcome: *const Outcome, which: u32, len: *mut u64) -> *const u8;

    pub(crate) fn nj2k_shim_viewer_data_size_j2c(w: i32, h: i32, discard: i32) -> i64;
    pub(crate) fn nj2k_shim_viewer_grid_side(tiles: u64, tile_size: u32) -> i32;
    pub(crate) fn nj2k_shim_viewer_components_supported(components: u64) -> u32;
    pub(crate) fn nj2k_shim_viewer_reduce_factor(
        requested_discard: i32,
        resolutions: i32,
        width: i32,
        height: i32,
        max_image_size: i32,
    ) -> i32;
    pub(crate) fn nj2k_shim_viewer_decode_allowed(width: u64, height: u64, components: u64, reduce: u32) -> u32;

    pub(crate) fn nj2k_shim_scan_summary(data: *const u8, size: usize, out: *mut c_char, cap: usize) -> usize;
    pub(crate) fn nj2k_shim_read_pgx(path_utf8: *const c_char, why: *mut c_char, why_cap: usize) -> *mut Pgx;
    pub(crate) fn nj2k_shim_pgx_info(
        pgx: *const Pgx,
        width: *mut i32,
        height: *mut i32,
        depth: *mut i32,
        is_signed: *mut u32,
        samples: *mut *const i32,
        count: *mut u64,
    );
    pub(crate) fn nj2k_shim_pgx_free(pgx: *mut Pgx);

    pub(crate) fn nj2k_shim_dc_shift_53_scalar(p: *mut i32, n: usize, shift: i32, lo: i32, hi: i32);
    pub(crate) fn nj2k_shim_dc_shift_97_scalar(p: *mut i32, n: usize, shift: i32, lo: i32, hi: i32);
    pub(crate) fn nj2k_shim_rct_scalar(
        c0: *const i32,
        c1: *const i32,
        c2: *const i32,
        o0: *mut i32,
        o1: *mut i32,
        o2: *mut i32,
        n: usize,
    );
    pub(crate) fn nj2k_shim_ict_scalar(
        c0: *const i32,
        c1: *const i32,
        c2: *const i32,
        o0: *mut i32,
        o1: *mut i32,
        o2: *mut i32,
        n: usize,
    );
    pub(crate) fn nj2k_shim_dwt_decode(
        plane: *mut i32,
        size: usize,
        stride: u32,
        resolutions: *const WaveletRect,
        count: usize,
        reversible: u32,
    ) -> u32;
    pub(crate) fn nj2k_shim_encode_u8(request: *const EncodeRequest, out: *mut *mut Codestream) -> i32;
    pub(crate) fn nj2k_shim_generate_codestream(seed: u32, index: u32, out: *mut *mut Codestream) -> i32;
    pub(crate) fn nj2k_shim_encode_u8_with_updates(
        request: *const EncodeRequest,
        updates: *mut EncodeUpdates,
        out: *mut *mut Codestream,
    ) -> i32;
    pub(crate) fn nj2k_shim_encode_u8_trace(
        request: *const EncodeRequest,
        trace: *const EncodeTraceRequest,
        out: *mut *mut Codestream,
    ) -> i32;
    pub(crate) fn nj2k_shim_codestream_offers(codestream: *const Codestream, count: *mut usize) -> *const usize;
    pub(crate) fn nj2k_shim_codestream_diagnostic(
        codestream: *const Codestream,
        index: usize,
        severity: *mut u32,
        text: *mut *const u8,
        len: *mut usize,
    ) -> u32;
    pub(crate) fn nj2k_shim_codestream_bytes(codestream: *const Codestream, len: *mut usize) -> *const u8;
    pub(crate) fn nj2k_shim_codestream_free(codestream: *mut Codestream);

    pub(crate) fn nj2k_shim_codec_new(
        format: u32,
        reduce: u32,
        reduce_policy: u32,
        limits: *const Limits,
        max_layers: u32,
        strict: u32,
    ) -> *mut Codec;
    pub(crate) fn nj2k_shim_codec_free(codec: *mut Codec);
    pub(crate) fn nj2k_shim_codec_set_max_memory(codec: *mut Codec, max_memory: u64) -> u32;
    pub(crate) fn nj2k_shim_codec_set_reduce(codec: *mut Codec, reduce: u32) -> i32;
    pub(crate) fn nj2k_shim_codec_header(
        codec: *mut Codec,
        data: *const u8,
        size: usize,
        reduce_after_header: *const u32,
        capture_diagnostics: u32,
        info: *mut HeaderInfo,
        out: *mut *mut Header,
    ) -> i32;
    pub(crate) fn nj2k_shim_header_components(header: *const Header, sized: u32) -> *const Component;
    pub(crate) fn nj2k_shim_header_diagnostic(
        header: *const Header,
        index: usize,
        severity: *mut u32,
        text: *mut *const u8,
        len: *mut usize,
    ) -> u32;
    pub(crate) fn nj2k_shim_header_free(header: *mut Header);
    pub(crate) fn nj2k_shim_codec_set_retained_bytes(codec: *mut Codec, retained_bytes: usize) -> u32;
    pub(crate) fn nj2k_shim_pool_new(workers: u32) -> *mut Pool;
    pub(crate) fn nj2k_shim_pool_free(pool: *mut Pool);
    pub(crate) fn nj2k_shim_pool_batches(pool: *const Pool) -> u64;
    pub(crate) fn nj2k_shim_pool_run(
        pool: *mut Pool,
        count: u32,
        job: extern "C" fn(*mut c_void, u32),
        context: *mut c_void,
    ) -> u32;
    pub(crate) fn nj2k_shim_codec_decode_i32(
        codec: *mut Codec,
        data: *const u8,
        size: usize,
        out: *mut i32,
        cap: u64,
        count: *mut u64,
        pool: *mut Pool,
    ) -> i32;
    pub(crate) fn nj2k_shim_ledger(codec: *mut Codec, ledger: *mut Ledger) -> u32;
    pub(crate) fn nj2k_shim_set_parallel_thresholds(
        codec: *mut Codec,
        tile_samples: u64,
        level_area: u64,
        output_samples: u64,
        clear_bytes: u64,
    ) -> u32;
    pub(crate) fn nj2k_shim_codec_dimensions_for(
        codec: *mut Codec,
        data: *const u8,
        size: usize,
        indices: *const u32,
        count: u32,
        numcomps: *mut u32,
        out: *mut *mut Header,
    ) -> i32;

    pub(crate) fn nj2k_shim_t1_new() -> *mut T1;
    pub(crate) fn nj2k_shim_t1_free(t1: *mut T1);
    pub(crate) fn nj2k_shim_t1_decode(
        t1: *mut T1,
        request: *const RawDecodeRequest,
        out: *mut i32,
        out_len: usize,
    ) -> u32;
    pub(crate) fn nj2k_shim_enc_t1_new() -> *mut EncT1;
    pub(crate) fn nj2k_shim_enc_t1_free(t1: *mut EncT1);
    pub(crate) fn nj2k_shim_enc_t1_encode(
        t1: *mut EncT1,
        request: *const RawEncodeRequest,
        result: *mut RawEncodeResult,
        passes: *mut RawPass,
        pass_cap: usize,
        bytes: *mut u8,
        byte_cap: usize,
    ) -> u32;
    pub(crate) fn nj2k_shim_mq_dec_run(
        init: *const MqDecoderRegs,
        data: *const u8,
        len: usize,
        ctx: *const u8,
        n: usize,
        bits: *mut u8,
        out: *mut MqDecoderRegs,
    ) -> u32;
    pub(crate) fn nj2k_shim_mq_dec_cases(cases: *const MqDecodeCase, n: usize, results: *mut MqDecodeResult);
    pub(crate) fn nj2k_shim_mq_enc_run(
        init: *const MqEncoderRegs,
        ctxbit: *const u8,
        n: usize,
        flush: u32,
        buf: *mut u8,
        cap: usize,
        limit: usize,
        out: *mut MqEncoderRegs,
        trace: *mut MqEncoderRegs,
    ) -> u32;
    pub(crate) fn nj2k_shim_mq_enc_cases(cases: *const MqEncodeCase, n: usize, results: *mut MqEncodeResult);
}
