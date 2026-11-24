// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// OpenJPEG's own API at the oracle's pin, as the Rust twin's timing against OpenJPEG calls it (opj_shim.h). Only
// OpenJPEG's header is included here: neo-j2k's compatibility header renames the same opj_* names.
#include "opj_shim.h"

#include <openjpeg.h>

#include <algorithm>
#include <cstdint>
#include <cstring>
#include <memory>
#include <new>
#include <vector>

namespace {

struct Memory {
    const OPJ_BYTE* buffer = nullptr;
    OPJ_SIZE_T size = 0;
    OPJ_OFF_T offset = 0;
};

OPJ_SIZE_T mem_read(void* out, OPJ_SIZE_T bytes, void* user)
{
    auto* m = static_cast<Memory*>(user);
    if (m->offset < 0 || static_cast<OPJ_SIZE_T>(m->offset) >= m->size) {
        m->offset = static_cast<OPJ_OFF_T>(m->size);
        return static_cast<OPJ_SIZE_T>(-1);
    }
    const OPJ_SIZE_T left = m->size - static_cast<OPJ_SIZE_T>(m->offset);
    const OPJ_SIZE_T n = bytes < left ? bytes : left;
    std::memcpy(out, m->buffer + m->offset, n);
    m->offset += static_cast<OPJ_OFF_T>(n);
    return n;
}

OPJ_SIZE_T mem_write(void*, OPJ_SIZE_T, void*)
{
    return 0;  // decode streams are read-only
}

OPJ_OFF_T mem_skip(OPJ_OFF_T bytes, void* user)
{
    auto* m = static_cast<Memory*>(user);
    const OPJ_OFF_T to = m->offset + bytes;
    if (to < 0 || to > static_cast<OPJ_OFF_T>(m->size)) {
        m->offset = std::clamp<OPJ_OFF_T>(to, 0, static_cast<OPJ_OFF_T>(m->size));
        return static_cast<OPJ_OFF_T>(-1);
    }
    m->offset = to;
    return bytes;
}

OPJ_BOOL mem_seek(OPJ_OFF_T to, void* user)
{
    auto* m = static_cast<Memory*>(user);
    if (to < 0 || to > static_cast<OPJ_OFF_T>(m->size)) {
        return OPJ_FALSE;
    }
    m->offset = to;
    return OPJ_TRUE;
}

// The viewer's handlers ignore every message; so do these (OpenJPEG's default handlers print to stderr).
void ignore_message(const char*, void*) {}

struct Grow {
    std::vector<std::uint8_t>* out;
    std::size_t pos = 0;
};

OPJ_SIZE_T grow_write(void* buf, OPJ_SIZE_T n, void* user)
{
    auto* g = static_cast<Grow*>(user);
    if (g->pos + n > g->out->size()) {
        g->out->resize(g->pos + n);
    }
    std::memcpy(g->out->data() + g->pos, buf, n);
    g->pos += n;
    return n;
}

OPJ_OFF_T grow_skip(OPJ_OFF_T n, void* user)
{
    auto* g = static_cast<Grow*>(user);
    const OPJ_OFF_T to = static_cast<OPJ_OFF_T>(g->pos) + n;
    if (to < 0) {
        return static_cast<OPJ_OFF_T>(-1);
    }
    g->pos = static_cast<std::size_t>(to);
    if (g->pos > g->out->size()) {
        g->out->resize(g->pos);
    }
    return n;
}

OPJ_BOOL grow_seek(OPJ_OFF_T to, void* user)
{
    auto* g = static_cast<Grow*>(user);
    if (to < 0) {
        return OPJ_FALSE;
    }
    g->pos = static_cast<std::size_t>(to);
    if (g->pos > g->out->size()) {
        g->out->resize(g->pos);
    }
    return OPJ_TRUE;
}

}  // namespace

struct nj2k_opj_decoder {
    uint32_t reduce = 0;
    uint32_t threads = 0;
};

struct nj2k_opj_bytes {
    std::vector<std::uint8_t> bytes;
};

extern "C" {

nj2k_opj_decoder_t* nj2k_opj_decoder_new(uint32_t reduce, uint32_t threads)
{
    try {
        auto* d = new nj2k_opj_decoder();
        d->reduce = reduce;
        d->threads = threads;
        return d;
    } catch (...) {
        return nullptr;
    }
}

void nj2k_opj_decoder_free(nj2k_opj_decoder_t* decoder)
{
    delete decoder;
}

int32_t nj2k_opj_decode(nj2k_opj_decoder_t* decoder, const uint8_t* data, uint64_t size, int32_t* out, uint64_t cap,
                        uint64_t* count)
{
    if (!decoder || !data || !size || !count) {
        return 1;
    }
    *count = 0;
    opj_dparameters_t params;
    opj_set_default_decoder_parameters(&params);
    params.cp_reduce = decoder->reduce;
    opj_codec_t* codec = opj_create_decompress(OPJ_CODEC_J2K);
    if (!codec) {
        return 1;
    }
    opj_stream_t* stream = nullptr;
    opj_image_t* image = nullptr;
    int32_t status = 0;
    Memory memory;
    memory.buffer = data;
    memory.size = static_cast<OPJ_SIZE_T>(size);
    do {
        opj_set_info_handler(codec, ignore_message, nullptr);
        opj_set_warning_handler(codec, ignore_message, nullptr);
        opj_set_error_handler(codec, ignore_message, nullptr);
        if (!opj_setup_decoder(codec, &params) ||
            (decoder->threads > 1 && !opj_codec_set_threads(codec, static_cast<int>(decoder->threads)))) {
            status = 1;
            break;
        }
        stream = opj_stream_create(static_cast<OPJ_SIZE_T>(size), OPJ_TRUE);
        if (!stream) {
            status = 1;
            break;
        }
        opj_stream_set_user_data(stream, &memory, nullptr);
        opj_stream_set_user_data_length(stream, static_cast<OPJ_UINT64>(size));
        opj_stream_set_read_function(stream, mem_read);
        opj_stream_set_write_function(stream, mem_write);
        opj_stream_set_skip_function(stream, mem_skip);
        opj_stream_set_seek_function(stream, mem_seek);
        opj_decoder_set_strict_mode(codec, OPJ_FALSE);
        if (!opj_read_header(stream, codec, &image)) {
            status = 2;
            break;
        }
        if (!opj_decode(codec, stream, image) || !opj_end_decompress(codec, stream) || !image || !image->comps) {
            status = 3;
            break;
        }
        uint64_t total = 0;
        for (OPJ_UINT32 c = 0; c < image->numcomps; ++c) {
            total += static_cast<uint64_t>(image->comps[c].w) * image->comps[c].h;
        }
        *count = total;
        if (out) {
            if (total > cap) {
                status = 4;
                break;
            }
            uint64_t at = 0;
            for (OPJ_UINT32 c = 0; c < image->numcomps; ++c) {
                const uint64_t n = static_cast<uint64_t>(image->comps[c].w) * image->comps[c].h;
                if (n) {
                    if (!image->comps[c].data) {
                        status = 3;
                        break;
                    }
                    std::memcpy(out + at, image->comps[c].data, n * sizeof(int32_t));
                    at += n;
                }
            }
        }
    } while (false);
    if (image) {
        opj_image_destroy(image);
    }
    if (stream) {
        opj_stream_destroy(stream);
    }
    opj_destroy_codec(codec);
    return status;
}

int32_t nj2k_opj_encode_u8(const uint8_t* pixels, size_t stride, uint32_t width, uint32_t height, uint32_t components,
                           uint32_t reversible, uint32_t resolutions, uint32_t mct, nj2k_opj_bytes_t** out)
{
    if (!pixels || !out || !width || !height || !components || stride < size_t(width) * components) {
        return 1;
    }
    *out = nullptr;
    try {
        opj_cparameters_t p;
        std::memset(&p, 0, sizeof(p));
        opj_set_default_encoder_parameters(&p);
        p.cod_format = OPJ_CODEC_J2K;
        p.prog_order = OPJ_RLCP;
        p.cp_disto_alloc = 1;
        p.tcp_numlayers = 1;
        p.tcp_rates[0] = 0.0f;
        p.numresolution = static_cast<int>(resolutions);
        p.irreversible = reversible ? 0 : 1;
        p.tcp_mct = static_cast<char>(mct ? 1 : 0);

        std::vector<opj_image_cmptparm_t> cp(components);
        for (auto& c : cp) {
            std::memset(&c, 0, sizeof(c));
            c.prec = 8;
            c.dx = 1;
            c.dy = 1;
            c.w = width;
            c.h = height;
        }
        opj_image_t* image = opj_image_create(components, cp.data(), OPJ_CLRSPC_SRGB);
        if (!image) {
            return 2;
        }
        image->x0 = 0;
        image->y0 = 0;
        image->x1 = width;
        image->y1 = height;
        for (uint32_t c = 0; c < components; ++c) {
            for (uint32_t y = 0; y < height; ++y) {
                const uint8_t* row = pixels + size_t(y) * stride + c;
                OPJ_INT32* dst = image->comps[c].data + size_t(y) * width;
                for (uint32_t x = 0; x < width; ++x) {
                    dst[x] = row[size_t(x) * components];
                }
            }
        }
        auto result = std::make_unique<nj2k_opj_bytes>();
        int32_t status = 0;
        opj_codec_t* enc = opj_create_compress(OPJ_CODEC_J2K);
        opj_stream_t* st = nullptr;
        Grow sink{&result->bytes};
        if (!enc) {
            status = 2;
        } else {
            opj_set_info_handler(enc, ignore_message, nullptr);
            opj_set_warning_handler(enc, ignore_message, nullptr);
            opj_set_error_handler(enc, ignore_message, nullptr);
            if (!opj_setup_encoder(enc, &p, image)) {
                status = 3;
            }
        }
        if (status == 0) {
            st = opj_stream_create(1 << 16, OPJ_FALSE);
            if (!st) {
                status = 2;
            }
        }
        if (status == 0) {
            opj_stream_set_user_data(st, &sink, nullptr);
            opj_stream_set_write_function(st, grow_write);
            opj_stream_set_skip_function(st, grow_skip);
            opj_stream_set_seek_function(st, grow_seek);
            if (!(opj_start_compress(enc, image, st) && opj_encode(enc, st) && opj_end_compress(enc, st))) {
                status = 4;
            }
        }
        if (st) {
            opj_stream_destroy(st);
        }
        if (enc) {
            opj_destroy_codec(enc);
        }
        opj_image_destroy(image);
        if (status != 0) {
            return status;
        }
        result->bytes.resize(sink.pos);
        *out = result.release();
        return 0;
    } catch (...) {
        return 5;
    }
}

const uint8_t* nj2k_opj_bytes_data(const nj2k_opj_bytes_t* bytes, size_t* len)
{
    if (!bytes || !len) {
        return nullptr;
    }
    *len = bytes->bytes.size();
    return bytes->bytes.data();
}

void nj2k_opj_bytes_free(nj2k_opj_bytes_t* bytes)
{
    delete bytes;
}

const char* nj2k_opj_version(void)
{
    return opj_version();
}

}  // extern "C"
