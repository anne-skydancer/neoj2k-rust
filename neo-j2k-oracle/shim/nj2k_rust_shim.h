/* SPDX-License-Identifier: BSD-2-Clause
 * Copyright (c) 2026, Anne Skydancer (neo-j2k)
 *
 * The test-only C interface the Rust crate neo-j2k-oracle links (rust/README.md): what the Rust
 * twin's differential tests need from the C++ that the public API does not give. Built only with
 * -DNJ2K_BUILD_RUST_SHIM=ON, never installed or shipped, and never linked by the neo-j2k crate
 * (its dev-dependency only).
 *
 * Every function is noexcept: a C++ exception (an allocation failure in the harness) never crosses
 * into Rust; it becomes a null result or 0. Every pointer argument must be valid for the call;
 * outcomes and backends are released by their own free functions. The bindings in
 * rust/neo-j2k-oracle/src/ffi.rs mirror this file and are checked against NJ2K_SHIM_ABI_VERSION.
 */
#ifndef NJ2K_RUST_SHIM_H
#define NJ2K_RUST_SHIM_H

#include <stddef.h>
#include <stdint.h>

#include <neoj2k/neoj2k.h>

#ifdef __cplusplus
extern "C" {
#endif

#define NJ2K_SHIM_ABI_VERSION 5u

uint32_t nj2k_shim_abi_version(void);
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
/* the scan's summary (rust/neo-j2k-testkit/src/scan.rs writes the same); writes up to cap bytes,
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
const uint8_t *nj2k_shim_codestream_bytes(const nj2k_shim_codestream_t *codestream, size_t *len);
void nj2k_shim_codestream_free(nj2k_shim_codestream_t *codestream);

/* ---- a memory codec of neo-j2k, its ledger numbers and its thresholds ------------------------- */
/* A memory decoder (NJ2K_CODEC_J2K or NJ2K_CODEC_JP2) with the given options and limits. */
nj2k_codec_t *nj2k_shim_codec_new(uint32_t format, uint32_t reduce, uint32_t reduce_policy,
                                  const nj2k_decoder_limits_t *limits, uint32_t max_layers, uint32_t strict);
void nj2k_shim_codec_free(nj2k_codec_t *codec);
uint32_t nj2k_shim_codec_set_max_memory(nj2k_codec_t *codec, uint64_t max_memory);
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
/* the header from memory, then the decode into planar int32 planes (each component w x h, one
 * after the other) in out (cap samples; the samples needed through *count): the status */
int32_t nj2k_shim_codec_decode_i32(nj2k_codec_t *codec, const uint8_t *data, size_t size, int32_t *out,
                                   uint64_t cap, uint64_t *count);
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

#ifdef __cplusplus
}
#endif

#endif
