/* SPDX-License-Identifier: BSD-2-Clause
 * Copyright (c) 2026, Anne Skydancer (neo-j2k)
 *
 * The test-only C interface the Rust crate neo-j2k-oracle links (README.md): what the Rust
 * twin's differential tests need from the C++ that the public API does not give. Built only with
 * -DNJ2K_BUILD_RUST_SHIM=ON, never installed or shipped, and never linked by the neo-j2k crate
 * (its dev-dependency only).
 *
 * Every function is noexcept: a C++ exception (an allocation failure in the harness) never crosses
 * into Rust; it becomes a null result or 0. Every pointer argument must be valid for the call;
 * outcomes and backends are released by their own free functions. The bindings in
 * neo-j2k-oracle/src/ffi.rs mirror this file and are checked against NJ2K_SHIM_ABI_VERSION.
 */
#ifndef NJ2K_RUST_SHIM_H
#define NJ2K_RUST_SHIM_H

#include <stddef.h>
#include <stdint.h>

#include <neoj2k/neoj2k.h>

#ifdef __cplusplus
extern "C" {
#endif

#define NJ2K_SHIM_ABI_VERSION 8u

uint32_t nj2k_shim_abi_version(void);
/* the C++ checkout's NJ2K_VERSION_MAJOR << 16 | MINOR << 8 | PATCH (neoj2k.h) */
uint32_t nj2k_shim_cpp_version(void);
/* 1 when the harness's backends were made before main() (their making clears OPJ_NUM_THREADS, an
 * environment write that must not race the test threads) */
uint32_t nj2k_shim_ready(void);
/* failures on the console, never in a dialog (tests/common: console_reports) */
void nj2k_shim_console_reports(void);

/* ---- the C++ size table: sizeof and alignof of the types the arenas hold ------------------- */
typedef struct nj2k_shim_layout {
    const char *name;
    uint64_t size;
    uint64_t align;
} nj2k_shim_layout_t;
/* the decoder's types, then the encoder's; *out stays valid for the whole run */
size_t nj2k_shim_layout_table(const nj2k_shim_layout_t **out);

/* ---- the test harness's requests (tests/common/decoder.h) ------------------------------------ */
#define NJ2K_SHIM_BACKEND_ORACLE 0u        /* OpenJPEG at the oracle's pin */
#define NJ2K_SHIM_BACKEND_NEOJ2K 1u        /* neo-j2k through its OpenJPEG compatibility API */
#define NJ2K_SHIM_BACKEND_NEOJ2K_MEMORY 2u /* neo-j2k's memory API, one codec kept across requests */
#define NJ2K_SHIM_MODE_PLAIN 0u
#define NJ2K_SHIM_MODE_STRICT 1u
#define NJ2K_SHIM_MODE_VIEWER 2u

typedef struct nj2k_shim_backend nj2k_shim_backend_t;
nj2k_shim_backend_t *nj2k_shim_backend_new(uint32_t kind);
void nj2k_shim_backend_free(nj2k_shim_backend_t *backend);

typedef struct nj2k_shim_request {
    const uint8_t *data;
    uint64_t size;
    int32_t reduce;
    int32_t layers;
    uint32_t mode;
    uint32_t jp2;
    uint32_t reduce_after_header;
    uint32_t limits;
    uint64_t max_memory;
} nj2k_shim_request_t;

typedef struct nj2k_shim_outcome nj2k_shim_outcome_t;
/* one request, its sample planes kept and its hashes computed; null when the harness failed */
nj2k_shim_outcome_t *nj2k_shim_decode(nj2k_shim_backend_t *backend, const nj2k_shim_request_t *request);
void nj2k_shim_outcome_free(nj2k_shim_outcome_t *outcome);

typedef struct nj2k_shim_outcome_info_s {
    uint32_t ok;
    uint32_t x0, y0, x1, y1, numcomps;
    int32_t color_space;
    uint32_t warnings, errors;
    uint64_t hash, u8_hash;
} nj2k_shim_outcome_info_t;
void nj2k_shim_outcome_info(const nj2k_shim_outcome_t *outcome, nj2k_shim_outcome_info_t *info);

typedef struct nj2k_shim_component {
    uint32_t w, h, dx, dy, x0, y0, prec, sgnd, factor, resno_decoded;
} nj2k_shim_component_t;
/* component c's geometry and samples (w x h, none when the request failed); 0 past the last */
uint32_t nj2k_shim_outcome_component(const nj2k_shim_outcome_t *outcome, uint32_t c, nj2k_shim_component_t *comp,
                                     const int32_t **samples, uint64_t *count);

#define NJ2K_SHIM_TEXT_STAGE 0u
#define NJ2K_SHIM_TEXT_LAST_ERROR 1u
#define NJ2K_SHIM_TEXT_WARNING_KINDS 2u
/* the bytes of one of the outcome's texts (not NUL-terminated for the caller's purposes) */
const uint8_t *nj2k_shim_outcome_text(const nj2k_shim_outcome_t *outcome, uint32_t which, uint64_t *len);

/* ---- the viewer's rules (tests/common/viewer.cpp) --------------------------------------------- */
int64_t nj2k_shim_viewer_data_size_j2c(int32_t w, int32_t h, int32_t discard);
int32_t nj2k_shim_viewer_grid_side(uint64_t tiles, uint32_t tile_size);
uint32_t nj2k_shim_viewer_components_supported(uint64_t components);
int32_t nj2k_shim_viewer_reduce_factor(int32_t requested_discard, int32_t resolutions, int32_t width, int32_t height,
                                       int32_t max_image_size);
uint32_t nj2k_shim_viewer_decode_allowed(uint64_t width, uint64_t height, uint64_t components, uint32_t reduce);

/* ---- the codestream scanner (tests/common/j2k_scan.h) as text, the PGX reader ------------------ */
/* the scan's summary (neo-j2k-testkit/src/scan.rs writes the same); writes up to cap bytes,
 * returns the summary's length (0 when the harness failed) */
size_t nj2k_shim_scan_summary(const uint8_t *data, size_t size, char *out, size_t cap);

typedef struct nj2k_shim_pgx nj2k_shim_pgx_t;
/* null when the file does not read as PGX (the reason through why/why_len when given) */
nj2k_shim_pgx_t *nj2k_shim_read_pgx(const char *path_utf8, char *why, size_t why_cap);
void nj2k_shim_pgx_info(const nj2k_shim_pgx_t *pgx, int32_t *width, int32_t *height, int32_t *depth,
                        uint32_t *is_signed, const int32_t **samples, uint64_t *count);
void nj2k_shim_pgx_free(nj2k_shim_pgx_t *pgx);

/* ---- the library's row kernels: the scalar references the library itself compiled ---------- */
void nj2k_shim_dc_shift_53_scalar(int32_t *p, size_t n, int32_t shift, int32_t lo, int32_t hi);
void nj2k_shim_dc_shift_97_scalar(int32_t *p, size_t n, int32_t shift, int32_t lo, int32_t hi);
void nj2k_shim_rct_scalar(const int32_t *c0, const int32_t *c1, const int32_t *c2, int32_t *o0, int32_t *o1,
                          int32_t *o2, size_t n);
void nj2k_shim_ict_scalar(const int32_t *c0, const int32_t *c1, const int32_t *c2, int32_t *o0, int32_t *o1,
                          int32_t *o2, size_t n);

typedef struct nj2k_shim_wavelet_rect {
    uint32_t x0, y0, x1, y1;
} nj2k_shim_wavelet_rect_t;
/* Inverse whole-tile-component wavelet. The plane holds size samples, stride samples per row.
 * Resolutions go from coarsest to finest. Returns 0 for invalid geometry or unavailable scratch. */
uint32_t nj2k_shim_dwt_decode(int32_t *plane, size_t size, uint32_t stride, const nj2k_shim_wavelet_rect_t *resolutions,
                              size_t count, uint32_t reversible);

/* ---- the C++ encoder, using a memory output stream -------------------------------------------- */
typedef struct nj2k_shim_encode_request {
    const uint8_t *pixels;
    size_t size, stride;
    uint32_t width, height, components, flip_vertical;
    uint32_t reversible, resolutions, mct, max_codestream_size;
    const float *rates;
    size_t rate_count;
    const uint8_t *comment;
    size_t comment_len;
} nj2k_shim_encode_request_t;
typedef struct nj2k_shim_codestream nj2k_shim_codestream_t;
/* Returns the C++ encoder's status; on success out owns the codestream until it is freed. */
int32_t nj2k_shim_encode_u8(const nj2k_shim_encode_request_t *request, nj2k_shim_codestream_t **out);
typedef struct nj2k_shim_encode_updates {
    uint32_t max_codestream_size, layer_count;
    float ratios[100];
} nj2k_shim_encode_updates_t;
/* A nonzero layer_count records the settings written back by a successful setup, even if encoding fails. */
int32_t nj2k_shim_encode_u8_with_updates(const nj2k_shim_encode_request_t *request,
                                       nj2k_shim_encode_updates_t *updates, nj2k_shim_codestream_t **out);
typedef struct nj2k_shim_encode_trace_request_s {
    size_t buffer_size;
    size_t max_write; /* At most this many bytes are accepted in each callback. */
    size_t stop_at;   /* Refuse further writes after this many bytes have been accepted. */
    uint32_t fail_with_error; /* Return size_t(-1) instead of zero on refusal. */
    const char *const *extra_options; /* NULL-terminated option list. */
} nj2k_shim_encode_trace_request_t;
/* The snapshot includes bytes, offers, and diagnostics up to any codec error. */
int32_t nj2k_shim_encode_u8_trace(const nj2k_shim_encode_request_t *request,
                                 const nj2k_shim_encode_trace_request_t *trace, nj2k_shim_codestream_t **out);
const size_t *nj2k_shim_codestream_offers(const nj2k_shim_codestream_t *codestream, size_t *count);
uint32_t nj2k_shim_codestream_diagnostic(const nj2k_shim_codestream_t *codestream, size_t index, uint32_t *severity,
                                         const uint8_t **text, size_t *len);
const uint8_t *nj2k_shim_codestream_bytes(const nj2k_shim_codestream_t *codestream, size_t *len);
void nj2k_shim_codestream_free(nj2k_shim_codestream_t *codestream);

/* Generate one bounded case with the pinned OpenJPEG encoder. Owns out on success. */
int32_t nj2k_shim_generate_codestream(uint32_t seed, uint32_t index, nj2k_shim_codestream_t **out);

/* ---- a memory codec of neo-j2k, its ledger numbers and its thresholds ------------------------- */
/* A memory decoder (NJ2K_CODEC_J2K or NJ2K_CODEC_JP2) with the given options and limits. */
nj2k_codec_t *nj2k_shim_codec_new(uint32_t format, uint32_t reduce, uint32_t reduce_policy,
                                  const nj2k_decoder_limits_t *limits, uint32_t max_layers, uint32_t strict);
void nj2k_shim_codec_free(nj2k_codec_t *codec);
uint32_t nj2k_shim_codec_set_max_memory(nj2k_codec_t *codec, uint64_t max_memory);
int32_t nj2k_shim_codec_set_reduce(nj2k_codec_t *codec, uint32_t reduce);
uint32_t nj2k_shim_codec_set_retained_bytes(nj2k_codec_t *codec, size_t retained_bytes);
typedef struct nj2k_shim_pool nj2k_shim_pool_t;
nj2k_shim_pool_t *nj2k_shim_pool_new(uint32_t workers);
void nj2k_shim_pool_free(nj2k_shim_pool_t *pool);
uint64_t nj2k_shim_pool_batches(const nj2k_shim_pool_t *pool);
/* Run the jobs synchronously, holding no references to job or context after returning. */
uint32_t nj2k_shim_pool_run(nj2k_shim_pool_t *pool, uint32_t count, nj2k_job_fn job, void *context);
typedef struct nj2k_shim_header nj2k_shim_header_t;
typedef struct nj2k_shim_header_info {
    uint32_t x0, y0, x1, y1, numcomps;
    int32_t color_space;
    uint32_t tx0, ty0, tdx, tdy, tw, th, numlayers, min_resolutions, progression;
    int32_t reduce_status, dimensions_status;
} nj2k_shim_header_info_t;
/* Returns the header status. The optional reduce change is applied between the header read and the dimensions
 * check. Even on a rejected header, out can own a diagnostic trace of the header read. */
int32_t nj2k_shim_codec_header(nj2k_codec_t *codec, const uint8_t *data, size_t size, const uint32_t *reduce_after_header,
                               uint32_t capture_diagnostics, nj2k_shim_header_info_t *info, nj2k_shim_header_t **out);
/* each result points to info->numcomps entries until the snapshot is freed; NULL if the dimensions check failed */
const nj2k_shim_component_t *nj2k_shim_header_components(const nj2k_shim_header_t *header, uint32_t sized);
/* The next message's severity (0 info, 1 warning, 2 error) and bytes; returns 0 at the end of the trace. */
uint32_t nj2k_shim_header_diagnostic(const nj2k_shim_header_t *header, size_t index, uint32_t *severity,
                                     const uint8_t **text, size_t *len);
void nj2k_shim_header_free(nj2k_shim_header_t *header);
/* Read the header and report the total number of samples through count. A null out queries this
 * count only. Otherwise decode into cap samples, packed by component, skipping empty planes.
 * An optional pool is borrowed for the decode only. */
int32_t nj2k_shim_codec_decode_i32(nj2k_codec_t *codec, const uint8_t *data, size_t size, int32_t *out,
                                   uint64_t cap, uint64_t *count, nj2k_shim_pool_t *pool);
typedef struct nj2k_shim_ledger_s {
    uint64_t header_reserved;  /* the header arena's reserved bytes */
    uint64_t tile_reserved;    /* the tile-part arena's */
    uint64_t work_reserved;    /* the work arena's */
    uint64_t work_high_water;  /* the work arena's high-water mark (0 once a decode's trim reset it) */
    uint64_t kept_bytes;       /* Decoder::kept_bytes(), which the trim after a decode keeps within retain */
} nj2k_shim_ledger_t;
uint32_t nj2k_shim_ledger(nj2k_codec_t *codec, nj2k_shim_ledger_t *ledger);
uint32_t nj2k_shim_set_parallel_thresholds(nj2k_codec_t *codec, uint64_t tile_samples, uint64_t level_area,
                                           uint64_t output_samples, uint64_t clear_bytes);
/* Read a memory header, select indices, and report the resulting sizing status and geometry. */
int32_t nj2k_shim_codec_dimensions_for(nj2k_codec_t *codec, const uint8_t *data, size_t size,
                                      const uint32_t *indices, uint32_t count, uint32_t *numcomps,
                                      nj2k_shim_header_t **out);

/* ---- Tier-1 (shim_t1.cpp): code-blocks and single MQ decisions ------------------------------- */
/* Return codes of the entries that run a code-block: 0 the request is invalid (a null pointer, a size or a value the
 * C++ is not meant to be given) or the harness failed; the Rust wrappers panic on it. 1 done. 2 the C++ refused the
 * code-block itself (a refusal is a result: Rust has to refuse it too). */
#define NJ2K_SHIM_T1_INVALID 0u
#define NJ2K_SHIM_T1_DONE 1u
#define NJ2K_SHIM_T1_REFUSED 2u

/* The decoder's code-block (src/t1.cpp: t1_decode_cblk, then t1_store as the tile buffer takes it). One handle keeps the
 * decoder's scratch from block to block, as a worker does; use it from one thread. */
typedef struct nj2k_shim_t1 nj2k_shim_t1_t;
nj2k_shim_t1_t *nj2k_shim_t1_new(void);
void nj2k_shim_t1_free(nj2k_shim_t1_t *t1);
typedef struct nj2k_shim_t1_decode_request {
    uint32_t w, h;              /* 1 to 1024 each, at most 4096 samples */
    uint32_t numbps;
    uint32_t orient;            /* 0 to 3 */
    uint32_t roishift;
    uint32_t cblksty;           /* 0 to 63 */
    const uint32_t *seg_len;    /* nsegs codeword segments: their bytes... */
    const uint32_t *seg_passes; /* ...and their passes with data */
    size_t nsegs;               /* at most 4096 */
    const uint8_t *bytes;       /* the chunks' bytes, gathered; null only when there are none */
    const uint32_t *chunk_len;  /* the lengths the bytes arrive in (their sum is the byte count); none: no data */
    size_t nchunks;             /* at most 4096 */
    uint32_t reversible;        /* the store: 5/3 halves the coefficients, 9/7 multiplies them by 0.5 * stepsize */
    float stepsize;
    uint32_t out_stride;        /* the tile buffer's row stride in samples, at least w */
    uint32_t out_offset;        /* and where the block's first sample lies in it */
} nj2k_shim_t1_decode_request_t;
/* DONE: out holds the block as t1_store writes it into a tile buffer at out_offset with stride out_stride (the
 * samples between rows are not touched); out_len must be at least out_offset + (h - 1) * out_stride + w. REFUSED:
 * bit-planes plus ROI shift at or above 31; out is untouched then. */
uint32_t nj2k_shim_t1_decode(nj2k_shim_t1_t *t1, const nj2k_shim_t1_decode_request_t *request, int32_t *out,
                             size_t out_len);
/* t1_store alone: the w * h coefficients src (stride w) stored into dst at stride dst_stride (dst_len at least
 * (h - 1) * dst_stride + w) with the ROI shift undone, as above. DONE or INVALID. */
uint32_t nj2k_shim_t1_store(const int32_t *src, uint32_t w, uint32_t h, uint32_t roishift, uint32_t reversible,
                            float stepsize, int32_t *dst, size_t dst_len, uint32_t dst_stride);

/* The encoder's code-block (src/enc_t1.cpp: enc_t1_encode, then enc_t1_commit). */
typedef struct nj2k_shim_enc_t1 nj2k_shim_enc_t1_t;
nj2k_shim_enc_t1_t *nj2k_shim_enc_t1_new(void);
void nj2k_shim_enc_t1_free(nj2k_shim_enc_t1_t *t1);
typedef struct nj2k_shim_enc_t1_request {
    uint32_t w, h;            /* 1 to 64 each */
    const int32_t *samples;   /* w * h signed fixed-point samples (6 fractional bits), in the order enc_t1_data takes */
    uint32_t orient;          /* 0 to 3 */
    uint32_t compno, level, reversible;
    uint32_t mct;             /* 1: the colour transform's norms for compno (3 components), 0: none */
    double stepsize;
} nj2k_shim_enc_t1_request_t;
typedef struct nj2k_shim_enc_pass {
    uint32_t rate, len, term;
    double distortion;
} nj2k_shim_enc_pass_t;
typedef struct nj2k_shim_enc_t1_result {
    uint32_t numbps;      /* the code-block's, wrapped as the C++ has it */
    uint32_t totalpasses; /* the passes written to passes (at most pass_cap) */
    uint32_t byte_len;    /* the bytes written to bytes: up to the last pass's rate */
} nj2k_shim_enc_t1_result_t;
/* DONE: coded. REFUSED: enc_t1_encode refused the block (its output clamp). INVALID also when the passes or the bytes
 * do not fit pass_cap and byte_cap. */
uint32_t nj2k_shim_enc_t1_encode(nj2k_shim_enc_t1_t *t1, const nj2k_shim_enc_t1_request_t *request,
                                 nj2k_shim_enc_t1_result_t *result, nj2k_shim_enc_pass_t *passes, size_t pass_cap,
                                 uint8_t *bytes, size_t byte_cap);

/* The MQ decoder's decision (src/t1_mq.h opj_mqc_decode_macro). The contexts are given and returned as state indices
 * (0 to 93, state * 2 + MPS: the order of OpenJPEG's table). */
typedef struct nj2k_shim_mq_dec_regs {
    uint32_t a, c, ct;
    uint32_t bp;          /* the read position, a byte offset in the data */
    uint8_t state[19];
} nj2k_shim_mq_dec_regs_t;
/* n decisions, decision i on context ctx[i] (below 19), the bits to bits[i], from a segment's start (ISO 15444-1 C.3.5
 * INITDEC on data, at the standard's initial states). The shim puts the 0xFF 0xFF of the decoder's sentinel after the
 * data's len bytes; data may be null only when len is 0. The initial registers are the shim's restatement of the C++'s
 * (INITDEC is local to src/t1.cpp): see shim_t1.cpp. Returns 0 for an invalid request. */
uint32_t nj2k_shim_mq_dec_run(const uint8_t *data, size_t len, const uint8_t *ctx, size_t n, uint8_t *bits,
                              nj2k_shim_mq_dec_regs_t *out);
/* one decision each, on context 0, in a batch: the bytes are b[0..3] and the sentinel, the read position starts at 0.
 * Valid: a from 0x8000 to 0xFFFF (a renormalised interval), state below 94. Returns 0 at the first invalid case. */
typedef struct nj2k_shim_mq_dec_case {
    uint32_t a, c, ct, state;
    uint8_t b[4];
} nj2k_shim_mq_dec_case_t;
typedef struct nj2k_shim_mq_dec_result {
    uint32_t bit, a, c, ct, bp, state; /* the context's state index after the decision */
} nj2k_shim_mq_dec_result_t;
uint32_t nj2k_shim_mq_dec_cases(const nj2k_shim_mq_dec_case_t *cases, size_t n, nj2k_shim_mq_dec_result_t *results);

/* The MQ encoder (src/enc_mq.h MqEnc). */
typedef struct nj2k_shim_mq_enc_regs {
    uint32_t a, c, ct;
    uint32_t bp;        /* the write position, a byte offset in buf (the fake first byte is at 0) */
    uint32_t overflow;
    uint8_t state[19];
} nj2k_shim_mq_enc_regs_t;
/* A fresh coder (init_enc on buf + 1 with the standard's initial states, as enc_t1.cpp sets them), then n decisions,
 * decision i = ctx[i] * 2 + bit[i] in ctxbit[i] (below 38), then the flush when flush is set. The coder writes buf
 * (cap bytes, buf[0] is the fake byte and must be 0); no byte past buf[limit] is written; 1 <= limit < cap. out gets the
 * registers at the end, trace (n entries, or NULL) the registers after each decision. Returns 0 for an invalid request. */
uint32_t nj2k_shim_mq_enc_run(const uint8_t *ctxbit, size_t n, uint32_t flush, uint8_t *buf, size_t cap, size_t limit,
                              nj2k_shim_mq_enc_regs_t *out, nj2k_shim_mq_enc_regs_t *trace);
/* one decision each, on context 0, in a batch (tests/unit/enc_t1_unit.cpp's single decisions): the buffer is eight
 * bytes of prev, the write position 2, the limit at 2 + limit_off. Valid: a from 0x8000 to 0xFFFF, state below 94,
 * bit 0 or 1, ct 1 to 12, prev below 256, limit_off 0 to 5. Returns 0 at the first invalid case. */
typedef struct nj2k_shim_mq_enc_case {
    uint32_t a, c, ct, state, bit, prev, limit_off;
} nj2k_shim_mq_enc_case_t;
typedef struct nj2k_shim_mq_enc_result {
    uint32_t a, c, ct, state, bp, overflow; /* state: the context's index after the decision */
    uint8_t bytes[8];
} nj2k_shim_mq_enc_result_t;
uint32_t nj2k_shim_mq_enc_cases(const nj2k_shim_mq_enc_case_t *cases, size_t n, nj2k_shim_mq_enc_result_t *results);
/* the flush alone, from registers (a from 0x8000 to 0xFFFF, ct 1 to 12, prev below 256, limit_off 0 to 5), in the same
 * eight bytes; state of the result is 0 */
typedef struct nj2k_shim_mq_flush_case {
    uint32_t a, c, ct, prev, limit_off;
} nj2k_shim_mq_flush_case_t;
uint32_t nj2k_shim_mq_flush_cases(const nj2k_shim_mq_flush_case_t *cases, size_t n, nj2k_shim_mq_enc_result_t *results);

#ifdef __cplusplus
}
#endif

#endif
