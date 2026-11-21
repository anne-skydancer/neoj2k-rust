// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The test-only C interface of nj2k_rust_shim.h: the harness's backends and requests, the viewer's
// rules, the scanner and the PGX reader of tests/common, the library's scalar row kernels, and a
// memory codec with its ledger numbers and thresholds.
#include "nj2k_rust_shim.h"

#include <cstdarg>
#include <cstdio>
#include <cstring>
#include <exception>
#include <memory>
#include <new>
#include <span>
#include <string>
#include <vector>

#include "common.h"
#include "decoder.h"
#include "j2k_scan.h"
#include "pgx.h"

// the library's internals (by their path: src/ is no include directory)
#include "../../../src/codec.h"
#include "../../../src/mct.h"
#include "../../../src/parallel.h"

#include "layout.h"

namespace {

using nj2k::test::Backend;
using nj2k::test::Mode;
using nj2k::test::Outcome;
using nj2k::test::Request;
using nj2k::detail::Messages;

// The stateless backends, made before main(): making the oracle's clears OPJ_NUM_THREADS
// (single_thread_openjpeg), an environment write that must not race the Rust test threads, which
// do not exist yet. Their decode() keeps no state between requests (a constant API table), so any
// number of threads may share them.
struct Shared {
    std::unique_ptr<Backend> oracle, neoj2k;
    Shared() noexcept
    {
        try {
            oracle = nj2k::test::make_oracle_backend();
            neoj2k = nj2k::test::make_neoj2k_backend();
        } catch (...) {
            oracle.reset();
            neoj2k.reset();
        }
    }
};
const Shared g_shared;

struct HeaderMessage {
    Messages::Kind severity;
    std::string text;
};

class HeaderDiagnostics {
public:
    HeaderDiagnostics(Messages& messages, std::vector<HeaderMessage>& entries, bool enabled) noexcept
        : messages_(messages), previous_(messages), entries_(entries)
    {
        if (enabled) {
            messages_.set(Messages::Info, record<Messages::Info>, this);
            messages_.set(Messages::Warning, record<Messages::Warning>, this);
            messages_.set(Messages::Error, record<Messages::Error>, this);
        }
    }

    ~HeaderDiagnostics() { messages_ = previous_; }

    bool failed() const noexcept { return failed_; }

private:
    template <Messages::Kind Kind>
    static void record(const char* text, void* cookie) noexcept
    {
        auto& capture = *static_cast<HeaderDiagnostics*>(cookie);
        try {
            capture.entries_.push_back(HeaderMessage{Kind, text});
        } catch (...) {
            capture.failed_ = true;
        }
    }

    Messages& messages_;
    Messages previous_;
    std::vector<HeaderMessage>& entries_;
    bool failed_ = false;
};

std::vector<nj2k_shim_component_t> copy_components(const nj2k_image_t& image)
{
    std::vector<nj2k_shim_component_t> out;
    out.reserve(image.numcomps);
    for (uint32_t c = 0; c < image.numcomps; ++c) {
        const nj2k_image_comp_t& k = image.comps[c];
        out.push_back(nj2k_shim_component_t{k.w, k.h, k.dx, k.dy, k.x0, k.y0, k.prec, k.sgnd, k.factor,
                                            k.resno_decoded});
    }
    return out;
}

}  // namespace

// A backend handle: a shared stateless one, or a memory backend of its own (a codec it keeps).
struct nj2k_shim_backend {
    Backend* use = nullptr;
    std::unique_ptr<Backend> own;
};

struct nj2k_shim_outcome {
    Outcome o;
};

struct nj2k_shim_pgx {
    nj2k::test::Pgx p;
};

struct nj2k_shim_header {
    std::vector<nj2k_shim_component_t> original, sized;
    std::vector<HeaderMessage> diagnostics;
};

extern "C" {

uint32_t nj2k_shim_abi_version(void)
{
    return NJ2K_SHIM_ABI_VERSION;
}

uint32_t nj2k_shim_ready(void)
{
    return g_shared.oracle && g_shared.neoj2k ? 1u : 0u;
}

void nj2k_shim_console_reports(void)
{
    nj2k::test::console_reports();
}

size_t nj2k_shim_layout_table(const nj2k_shim_layout_t** out)
{
    // made on first use (a thread-safe local static); both halves in one array
    static const std::vector<nj2k_shim_layout_t> table = [] {
        std::vector<nj2k_shim_layout_t> t(nj2k::shim::kDecoderLayout,
                                          nj2k::shim::kDecoderLayout + nj2k::shim::kDecoderLayoutCount);
        t.insert(t.end(), nj2k::shim::kEncoderLayout, nj2k::shim::kEncoderLayout + nj2k::shim::kEncoderLayoutCount);
        return t;
    }();
    if (out) {
        *out = table.data();
    }
    return table.size();
}

nj2k_shim_backend_t* nj2k_shim_backend_new(uint32_t kind)
{
    try {
        auto b = std::make_unique<nj2k_shim_backend>();
        switch (kind) {
        case NJ2K_SHIM_BACKEND_ORACLE:
            b->use = g_shared.oracle.get();
            break;
        case NJ2K_SHIM_BACKEND_NEOJ2K:
            b->use = g_shared.neoj2k.get();
            break;
        case NJ2K_SHIM_BACKEND_NEOJ2K_MEMORY:
            b->own = nj2k::test::make_neoj2k_memory_backend();
            b->use = b->own.get();
            break;
        default:
            return nullptr;
        }
        return b->use ? b.release() : nullptr;
    } catch (...) {
        return nullptr;
    }
}

void nj2k_shim_backend_free(nj2k_shim_backend_t* backend)
{
    delete backend;
}

nj2k_shim_outcome_t* nj2k_shim_decode(nj2k_shim_backend_t* backend, const nj2k_shim_request_t* request)
{
    if (!backend || !backend->use || !request || request->mode > NJ2K_SHIM_MODE_VIEWER ||
        request->size > SIZE_MAX || (!request->data && request->size)) {
        return nullptr;
    }
    try {
        Request rq;
        rq.data = request->data;
        rq.size = static_cast<std::size_t>(request->size);
        rq.reduce = request->reduce;
        rq.layers = request->layers;
        rq.mode = request->mode == NJ2K_SHIM_MODE_PLAIN    ? Mode::Plain
                  : request->mode == NJ2K_SHIM_MODE_STRICT ? Mode::Strict
                                                           : Mode::Viewer;
        rq.capture = true;
        rq.inspect = true;
        rq.jp2 = request->jp2 != 0;
        rq.reduce_after_header = request->reduce_after_header != 0;
        rq.limits = request->limits != 0;
        rq.max_memory = request->max_memory;
        auto out = std::make_unique<nj2k_shim_outcome>();
        out->o = backend->use->decode(rq);
        return out.release();
    } catch (...) {
        return nullptr;
    }
}

void nj2k_shim_outcome_free(nj2k_shim_outcome_t* outcome)
{
    delete outcome;
}

void nj2k_shim_outcome_info(const nj2k_shim_outcome_t* outcome, nj2k_shim_outcome_info_t* info)
{
    if (!outcome || !info) {
        return;
    }
    const Outcome& o = outcome->o;
    *info = nj2k_shim_outcome_info_t{o.ok ? 1u : 0u, o.x0,       o.y0,     o.x1,   o.y1,    o.numcomps,
                                     o.color_space,   o.warnings, o.errors, o.hash, o.u8_hash};
}

uint32_t nj2k_shim_outcome_component(const nj2k_shim_outcome_t* outcome, uint32_t c, nj2k_shim_component_t* comp,
                                     const int32_t** samples, uint64_t* count)
{
    if (!outcome || !comp || !samples || !count || c >= outcome->o.comps.size()) {
        return 0;
    }
    const nj2k::test::Component& k = outcome->o.comps[c];
    *comp = nj2k_shim_component_t{k.w, k.h, k.dx, k.dy, k.x0, k.y0, k.prec, k.sgnd, k.factor, k.resno_decoded};
    if (c < outcome->o.planes.size()) {
        *samples = outcome->o.planes[c].data();
        *count = outcome->o.planes[c].size();
    } else {
        *samples = nullptr;
        *count = 0;
    }
    return 1;
}

const uint8_t* nj2k_shim_outcome_text(const nj2k_shim_outcome_t* outcome, uint32_t which, uint64_t* len)
{
    if (!outcome || !len) {
        return nullptr;
    }
    const std::string* s = which == NJ2K_SHIM_TEXT_STAGE        ? &outcome->o.stage
                           : which == NJ2K_SHIM_TEXT_LAST_ERROR ? &outcome->o.last_error
                           : which == NJ2K_SHIM_TEXT_WARNING_KINDS ? &outcome->o.warning_kinds
                                                                   : nullptr;
    if (!s) {
        *len = 0;
        return nullptr;
    }
    *len = s->size();
    return reinterpret_cast<const uint8_t*>(s->data());
}

int64_t nj2k_shim_viewer_data_size_j2c(int32_t w, int32_t h, int32_t discard)
{
    return nj2k::test::viewer::data_size_j2c(w, h, discard);
}

int32_t nj2k_shim_viewer_grid_side(uint64_t tiles, uint32_t tile_size)
{
    return nj2k::test::viewer::grid_side(tiles, tile_size);
}

uint32_t nj2k_shim_viewer_components_supported(uint64_t components)
{
    return nj2k::test::viewer::components_supported(components) ? 1u : 0u;
}

int32_t nj2k_shim_viewer_reduce_factor(int32_t requested_discard, int32_t resolutions, int32_t width, int32_t height,
                                       int32_t max_image_size)
{
    return nj2k::test::viewer::reduce_factor(requested_discard, resolutions, width, height, max_image_size);
}

uint32_t nj2k_shim_viewer_decode_allowed(uint64_t width, uint64_t height, uint64_t components, uint32_t reduce)
{
    return nj2k::test::viewer::decode_allowed(width, height, components, reduce) ? 1u : 0u;
}

}  // extern "C"

namespace {

// printf onto s, however long the text (a tile-part's PLT lengths and packet starts can be long)
void line(std::string& s, const char* fmt, ...)
{
    va_list ap, again;
    va_start(ap, fmt);
    va_copy(again, ap);
    const int n = std::vsnprintf(nullptr, 0, fmt, ap);
    va_end(ap);
    if (n > 0) {
        const std::size_t at = s.size();
        s.resize(at + static_cast<std::size_t>(n) + 1);
        std::vsnprintf(s.data() + at, static_cast<std::size_t>(n) + 1, fmt, again);
        s.resize(at + static_cast<std::size_t>(n));
    }
    va_end(again);
}

std::string hex_bytes(const std::vector<std::uint8_t>& v)
{
    if (v.empty()) {
        return "-";
    }
    std::string s;
    for (std::uint8_t b : v) {
        char buf[3];
        std::snprintf(buf, sizeof(buf), "%02x", b);
        s += buf;
    }
    return s;
}

template <class T>
std::string list(const std::vector<T>& v)
{
    if (v.empty()) {
        return "-";
    }
    std::string s;
    for (std::size_t i = 0; i < v.size(); ++i) {
        s += (i ? "," : "") + std::to_string(v[i]);
    }
    return s;
}

void coding(std::string& s, const char* tag, const nj2k::test::Coding& c)
{
    line(s, "%s %u %u %u %u %u prec %s\n", tag, c.is_cod ? 1u : 0u, unsigned(c.layers), unsigned(c.levels),
         unsigned(c.xcb), unsigned(c.ycb), hex_bytes(c.precincts).c_str());
}

// The scan's summary, line for line as rust/neo-j2k-testkit/src/scan.rs writes it.
std::string scan_summary(std::span<const std::uint8_t> bytes)
{
    std::string s;
    std::string why;
    const auto mh = nj2k::test::parse_main_header(bytes, &why);
    if (!mh) {
        line(s, "main none %s\n", why.c_str());
    } else {
        line(s, "siz %u %u %u %u %u %u %u %u rsiz %u csiz %u\n", mh->xsiz, mh->ysiz, mh->xosiz, mh->yosiz, mh->xtsiz,
             mh->ytsiz, mh->xtosiz, mh->ytosiz, unsigned(mh->rsiz), unsigned(mh->csiz));
        for (std::size_t c = 0; c < mh->ssiz.size(); ++c) {
            line(s, "comp %u %u %u\n", unsigned(mh->ssiz[c]), unsigned(mh->xrsiz[c]), unsigned(mh->yrsiz[c]));
        }
        line(s, "cod %u %u %u %u %u %u %u %u %u prec %s\n", unsigned(mh->scod), unsigned(mh->progression),
             unsigned(mh->layers), unsigned(mh->mct), unsigned(mh->levels), unsigned(mh->cblk_w_exp),
             unsigned(mh->cblk_h_exp), unsigned(mh->cblk_style), unsigned(mh->transform),
             hex_bytes(mh->precincts).c_str());
        for (const auto& c : mh->codings) {
            coding(s, "coding", c);
        }
        line(s, "flags %d %u %u %u %u %u %u %u %u end %zu tiles %llu %llu\n", mh->min_resolutions,
             unsigned(mh->has_coc), unsigned(mh->has_qcc), unsigned(mh->has_poc), unsigned(mh->has_ppm),
             unsigned(mh->has_tlm), unsigned(mh->has_plm), unsigned(mh->has_com), unsigned(mh->unknown_marker),
             mh->end, static_cast<unsigned long long>(mh->tiles_x()), static_cast<unsigned long long>(mh->tiles_y()));
        for (const auto& tp : nj2k::test::scan_tile_parts(bytes, *mh)) {
            line(s, "tp %zu %zu %zu %u %u %u %u ppt %u plt %u %s\n", tp.sot, tp.data, tp.end, unsigned(tp.tile),
                 tp.psot, unsigned(tp.tpsot), unsigned(tp.tnsot), unsigned(tp.has_ppt), unsigned(tp.has_plt),
                 list(tp.plt).c_str());
            for (const auto& c : tp.codings) {
                coding(s, "tpcoding", c);
            }
            const auto starts = nj2k::test::packet_starts(bytes, *mh, tp);
            line(s, "packets %s\n", starts ? list(*starts).c_str() : "none");
        }
    }
    const auto layers = nj2k::test::layer_boundaries(bytes);
    line(s, "layers %s\n", layers ? list(*layers).c_str() : "none");
    return s;
}

}  // namespace

extern "C" {

size_t nj2k_shim_scan_summary(const uint8_t* data, size_t size, char* out, size_t cap)
{
    if (!data && size) {
        return 0;
    }
    try {
        const std::string s = scan_summary(std::span<const std::uint8_t>(data, size));
        if (out && cap) {
            std::memcpy(out, s.data(), s.size() < cap ? s.size() : cap);
        }
        return s.size();
    } catch (...) {
        return 0;
    }
}

nj2k_shim_pgx_t* nj2k_shim_read_pgx(const char* path_utf8, char* why, size_t why_cap)
{
    if (!path_utf8) {
        return nullptr;
    }
    try {
        std::string reason;
        auto p = nj2k::test::read_pgx(path_utf8, &reason);
        if (!p) {
            if (why && why_cap) {
                const std::size_t n = reason.size() < why_cap - 1 ? reason.size() : why_cap - 1;
                std::memcpy(why, reason.data(), n);
                why[n] = '\0';
            }
            return nullptr;
        }
        auto out = std::make_unique<nj2k_shim_pgx>();
        out->p = std::move(*p);
        return out.release();
    } catch (...) {
        return nullptr;
    }
}

void nj2k_shim_pgx_info(const nj2k_shim_pgx_t* pgx, int32_t* width, int32_t* height, int32_t* depth,
                        uint32_t* is_signed, const int32_t** samples, uint64_t* count)
{
    if (!pgx || !width || !height || !depth || !is_signed || !samples || !count) {
        return;
    }
    *width = pgx->p.width;
    *height = pgx->p.height;
    *depth = pgx->p.depth;
    *is_signed = pgx->p.is_signed ? 1u : 0u;
    *samples = pgx->p.samples.data();
    *count = pgx->p.samples.size();
}

void nj2k_shim_pgx_free(nj2k_shim_pgx_t* pgx)
{
    delete pgx;
}

void nj2k_shim_dc_shift_53_scalar(int32_t* p, size_t n, int32_t shift, int32_t lo, int32_t hi)
{
    nj2k::detail::kernels::dc_shift_53_scalar(p, n, shift, lo, hi);
}

void nj2k_shim_dc_shift_97_scalar(int32_t* p, size_t n, int32_t shift, int32_t lo, int32_t hi)
{
    nj2k::detail::kernels::dc_shift_97_scalar(p, n, shift, lo, hi);
}

void nj2k_shim_rct_scalar(const int32_t* c0, const int32_t* c1, const int32_t* c2, int32_t* o0, int32_t* o1,
                          int32_t* o2, size_t n)
{
    nj2k::detail::kernels::rct_scalar(c0, c1, c2, o0, o1, o2, n);
}

void nj2k_shim_ict_scalar(const int32_t* c0, const int32_t* c1, const int32_t* c2, int32_t* o0, int32_t* o1,
                          int32_t* o2, size_t n)
{
    nj2k::detail::kernels::ict_scalar(c0, c1, c2, o0, o1, o2, n);
}

uint32_t nj2k_shim_codec_set_max_memory(nj2k_codec_t* codec, uint64_t max_memory)
{
    nj2k_decoder_limits_t limits;
    nj2k_decoder_limits_init(&limits, NJ2K_DECODER_LIMITS_NONE);
    limits.max_memory = max_memory;
    return codec && nj2k_decoder_set_limits(codec, &limits) ? 1u : 0u;
}

nj2k_codec_t* nj2k_shim_codec_new(uint32_t format, uint32_t reduce, uint32_t reduce_policy,
                                  const nj2k_decoder_limits_t* limits)
{
    if (format != NJ2K_CODEC_J2K && format != NJ2K_CODEC_JP2) {
        return nullptr;
    }
    nj2k_codec_t* codec = nj2k_create_decompress(static_cast<NJ2K_CODEC_FORMAT>(format));
    if (!codec) {
        return nullptr;
    }
    nj2k_dparameters_t p;
    nj2k_set_default_decoder_parameters(&p);
    p.cp_reduce = reduce;
    if (!nj2k_setup_decoder(codec, &p) || !nj2k_decoder_set_strict_mode(codec, NJ2K_FALSE) ||
        !nj2k_decoder_set_reduce_policy(codec, reduce_policy) || !nj2k_decoder_set_limits(codec, limits)) {
        nj2k_destroy_codec(codec);
        return nullptr;
    }
    return codec;
}

void nj2k_shim_codec_free(nj2k_codec_t* codec)
{
    if (codec) {
        nj2k_destroy_codec(codec);
    }
}

int32_t nj2k_shim_codec_header(nj2k_codec_t* codec, const uint8_t* data, size_t size, const uint32_t* reduce_after_header,
                               uint32_t capture_diagnostics, nj2k_shim_header_info_t* info, nj2k_shim_header_t** out)
{
    auto* decoder = nj2k::detail::Decoder::from(codec);
    if (!decoder || (!data && size) || !info || !out) {
        return NJ2K_E_ARGUMENT;
    }
    *out = nullptr;
    try {
        auto header = std::make_unique<nj2k_shim_header>();
        nj2k_image_t image{};
        int32_t header_status;
        {
            HeaderDiagnostics capture(decoder->messages, header->diagnostics, capture_diagnostics != 0);
            nj2k_read_header_memory(codec, data, size, &image);
            header_status = nj2k_get_status(codec);
            if (capture.failed()) {
                return NJ2K_E_NO_MEMORY;
            }
        }
        if (header_status != NJ2K_OK) {
            *out = header.release();
            return header_status;
        }
        nj2k_codestream_layout_t layout{};
        layout.struct_size = sizeof layout;
        if (!nj2k_get_codestream_layout(codec, &layout)) {
            return nj2k_get_status(codec);
        }
        header->original = copy_components(image);
        int32_t reduce_status = NJ2K_OK;
        if (reduce_after_header) {
            nj2k_set_decoded_resolution_factor(codec, *reduce_after_header);
            reduce_status = nj2k_get_status(codec);
        }
        if (nj2k_get_decoded_dimensions(codec, &image)) {
            header->sized = copy_components(image);
        }
        *info = nj2k_shim_header_info_t{image.x0,      image.y0,         image.x1,              image.y1,
                                        image.numcomps, static_cast<int32_t>(image.color_space),
                                        layout.tx0,     layout.ty0,        layout.tdx,             layout.tdy,
                                        layout.tw,      layout.th,         layout.numlayers,       layout.min_resolutions,
                                        decoder->header->cod.prg,          reduce_status,          nj2k_get_status(codec)};
        *out = header.release();
        return NJ2K_OK;
    } catch (...) {
        return NJ2K_E_NO_MEMORY;
    }
}

const nj2k_shim_component_t* nj2k_shim_header_components(const nj2k_shim_header_t* header, uint32_t sized)
{
    if (!header) {
        return nullptr;
    }
    const auto& components = sized ? header->sized : header->original;
    return components.empty() ? nullptr : components.data();
}

uint32_t nj2k_shim_header_diagnostic(const nj2k_shim_header_t* header, size_t index, uint32_t* severity,
                                     const uint8_t** text, size_t* len)
{
    if (!header || index >= header->diagnostics.size() || !severity || !text || !len) {
        return 0;
    }
    const auto& entry = header->diagnostics[index];
    *severity = static_cast<uint32_t>(entry.severity);
    *text = reinterpret_cast<const uint8_t*>(entry.text.data());
    *len = entry.text.size();
    return 1;
}

void nj2k_shim_header_free(nj2k_shim_header_t* header)
{
    delete header;
}

int32_t nj2k_shim_codec_decode_i32(nj2k_codec_t* codec, const uint8_t* data, size_t size, int32_t* out, uint64_t cap,
                                   uint64_t* count)
{
    if (!codec || (!data && size) || !count || (!out && cap)) {
        return NJ2K_E_ARGUMENT;
    }
    *count = 0;
    nj2k_image_t image;
    if (!nj2k_read_header_memory(codec, data, size, &image)) {
        return nj2k_get_status(codec);
    }
    try {
        std::vector<nj2k_plane_t> planes(image.numcomps);
        std::uint64_t at = 0;
        for (std::uint32_t c = 0; c < image.numcomps; ++c) {
            planes[c] = nj2k_plane_t{out ? out + at : nullptr, std::int64_t(image.comps[c].w) * 4, 4, c};
            at += std::uint64_t(image.comps[c].w) * image.comps[c].h;
        }
        *count = at;
        if (at > cap) {
            return NJ2K_E_OUTPUT;
        }
        const nj2k_output_t output{sizeof(nj2k_output_t), NJ2K_SAMPLE_I32, image.numcomps, planes.data()};
        nj2k_decode_memory(codec, data, size, &output);
        return nj2k_get_status(codec);
    } catch (...) {
        return NJ2K_E_NO_MEMORY;
    }
}

uint32_t nj2k_shim_ledger(nj2k_codec_t* codec, nj2k_shim_ledger_t* ledger)
{
    nj2k::detail::Decoder* d = nj2k::detail::Decoder::from(codec);
    if (!d || !ledger) {
        return 0;
    }
    *ledger = nj2k_shim_ledger_t{d->arena.reserved(), d->tile_arena.reserved(), d->work.reserved(),
                                 d->work.high_water(), d->kept_bytes()};
    return 1;
}

uint32_t nj2k_shim_set_parallel_thresholds(nj2k_codec_t* codec, uint64_t tile_samples, uint64_t level_area,
                                           uint64_t output_samples, uint64_t clear_bytes)
{
    return nj2k::detail::set_parallel_thresholds(codec, tile_samples, level_area, output_samples, clear_bytes) ? 1u
                                                                                                                : 0u;
}

}  // extern "C"
