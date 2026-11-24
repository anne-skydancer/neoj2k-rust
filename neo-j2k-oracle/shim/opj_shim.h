/* SPDX-License-Identifier: BSD-2-Clause
 * Copyright (c) 2026, Anne Skydancer (neo-j2k)
 *
 * The test-only C interface to OpenJPEG's own API (the oracle at its pin, OpenJPEG 2.5.4), for the Rust twin's
 * timing against OpenJPEG (neo-j2k/tests/vs_openjpeg.rs). Plain types only: the implementation (shim_opj.cpp)
 * includes OpenJPEG's header and nothing of neo-j2k's. Built only with -DNJ2K_BUILD_RUST_SHIM=ON, never shipped.
 * A failure is a nonzero status or a null result; no C++ exception crosses this interface.
 */
#ifndef NJ2K_OPJ_SHIM_H
#define NJ2K_OPJ_SHIM_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* The settings of OpenJPEG decodes: OpenJPEG keeps no codec between decodes, so this holds the reduce factor and the
 * thread count only, and every nj2k_opj_decode makes (and destroys) its own codec and memory stream. */
typedef struct nj2k_opj_decoder nj2k_opj_decoder_t;
/* threads 0 or 1: no opj_codec_set_threads (OpenJPEG's serial path); more: opj_codec_set_threads(threads), which makes
 * that many worker threads in every decode (OpenJPEG's own pool, per codec). */
nj2k_opj_decoder_t *nj2k_opj_decoder_new(uint32_t reduce, uint32_t threads);
void nj2k_opj_decoder_free(nj2k_opj_decoder_t *decoder);
/* One decode of a J2K codestream in non-strict mode (a codestream missing its end marker decodes). Returns 0 on
 * success, else 1 (codec setup), 2 (header), 3 (decode), 4 (out too small). With out non-null the image's planes are
 * copied, one after the other, into out (cap samples); *count is the number of samples in the image. With out null
 * nothing is copied: the timed calls. */
int32_t nj2k_opj_decode(nj2k_opj_decoder_t *decoder, const uint8_t *data, uint64_t size, int32_t *out, uint64_t cap,
                        uint64_t *count);

typedef struct nj2k_opj_bytes nj2k_opj_bytes_t;
/* The viewer-style encode of an 8-bit interleaved image through OpenJPEG's API: opj_image_create, the samples copied
 * into its int32 planes, opj_setup_encoder (RLCP, one layer, no rate limit, 64 x 64 code-blocks, OpenJPEG's default
 * comment), a memory output stream, opj_encode. Returns 0 on success and the codestream in *out (free it with
 * nj2k_opj_bytes_free), else a nonzero status. */
int32_t nj2k_opj_encode_u8(const uint8_t *pixels, size_t stride, uint32_t width, uint32_t height, uint32_t components,
                           uint32_t reversible, uint32_t resolutions, uint32_t mct, nj2k_opj_bytes_t **out);
const uint8_t *nj2k_opj_bytes_data(const nj2k_opj_bytes_t *bytes, size_t *len);
void nj2k_opj_bytes_free(nj2k_opj_bytes_t *bytes);

/* OpenJPEG's version string (opj_version) */
const char *nj2k_opj_version(void);

#ifdef __cplusplus
}
#endif

#endif
