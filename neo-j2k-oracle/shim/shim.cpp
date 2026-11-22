// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The test-only C interface of nj2k_rust_shim.h: the harness's backends and requests, the viewer's
// rules, the scanner and the PGX reader of tests/common, the library's transform kernels, and a
// memory codec with its ledger numbers and thresholds.
#include "nj2k_rust_shim.h"

#include <algorithm>
#include <cstdarg>
#include <cstdio>
#include <cstring>
#include <exception>
#include <limits>
#include <memory>
#include <new>
#include <span>
#include <string>
#include <vector>

#include "common.h"
#include "decoder.h"
#include "generated.h"
#include "j2k_scan.h"
#include "pgx.h"
#include "thread_pool.h"

// The C++ checkout exposes its internals to this test-only target through its src include path.
#include "codec.h"
#include "dwt.h"
#include "encode.h"
#include "mct.h"
#include "parallel.h"

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

struct CapturedMessage {
    Messages::Kind severity;
    std::string text;
};

class CaptureDiagnostics {
public:
    CaptureDiagnostics(Messages& messages, std::vector<CapturedMessage>& entries, bool enabled) noexcept
        : messages_(messages), previous_(messages), entries_(entries)
    {
        if (enabled) {
            messages_.set(Messages::Info, record<Messages::Info>, this);
            messages_.set(Messages::Warning, record<Messages::Warning>, this);
            messages_.set(Messages::Error, record<Messages::Error>, this);
        }
    }

    ~CaptureDiagnostics() { messages_ = previous_; }

    bool failed() const noexcept { return failed_; }

private:
    template <Messages::Kind Kind>
    static void record(const char* text, void* cookie) noexcept
    {
        auto& capture = *static_cast<CaptureDiagnostics*>(cookie);
        try {
            capture.entries_.push_back(CapturedMessage{Kind, text});
        } catch (...) {
            capture.failed_ = true;
        }
    }

    Messages& messages_;
    Messages previous_;
    std::vector<CapturedMessage>& entries_;
    bool failed_ = false;
};

uint32_t diagnostic_at(const std::vector<CapturedMessage>& entries, size_t index, uint32_t* severity,
                       const uint8_t** text, size_t* len) noexcept
{
    if (index >= entries.size() || !severity || !text || !len) {
        return 0;
    }
    const auto& entry = entries[index];
    *severity = static_cast<uint32_t>(entry.severity);
    *text = reinterpret_cast<const uint8_t*>(entry.text.data());
    *len = entry.text.size();
    return 1;
}

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
    std::vector<CapturedMessage> diagnostics;
};

struct nj2k_shim_codestream {
    std::vector<uint8_t> bytes;
    std::vector<size_t> offers;
    std::vector<CapturedMessage> diagnostics;
};

struct nj2k_shim_pool {
    explicit nj2k_shim_pool(uint32_t workers) : threads(workers) {}
    nj2k::test::ThreadPool threads;
};

namespace {

class BorrowedExecutor {
public:
    BorrowedExecutor(nj2k_codec_t* codec, nj2k_shim_pool_t* pool) noexcept : codec_(pool ? codec : nullptr)
    {
        if (codec_) {
            const nj2k_executor_t executor = pool->threads.executor();
            nj2k_decoder_set_executor(codec_, &executor);
        }
    }

    ~BorrowedExecutor()
    {
        if (codec_) {
            nj2k_decoder_set_executor(codec_, nullptr);
        }
    }

    BorrowedExecutor(const BorrowedExecutor&) = delete;
    BorrowedExecutor& operator=(const BorrowedExecutor&) = delete;

private:
    nj2k_codec_t* codec_;
};

NJ2K_SIZE_T append_encoded(void* buffer, NJ2K_SIZE_T count, void* cookie) noexcept
{
    auto& output = *static_cast<nj2k_shim_codestream_t*>(cookie);
    const auto* bytes = static_cast<const uint8_t*>(buffer);
    try {
        output.bytes.insert(output.bytes.end(), bytes, bytes + count);
        return count;
    } catch (...) {
        return 0;
    }
}

struct TracedWriter {
    nj2k_shim_codestream_t& output;
    const nj2k_shim_encode_trace_request_t* trace;
    bool failed = false;

    static NJ2K_SIZE_T write(void* buffer, NJ2K_SIZE_T count, void* cookie) noexcept
    {
        auto& writer = *static_cast<TracedWriter*>(cookie);
        const auto& options = *writer.trace;
        try {
            writer.output.offers.push_back(count);
        } catch (...) {
            writer.failed = true;
            return 0;
        }
        count = std::min({count, options.max_write, options.stop_at - writer.output.bytes.size()});
        if (count == 0) {
            return options.fail_with_error ? std::numeric_limits<NJ2K_SIZE_T>::max() : 0;
        }
        const auto accepted = append_encoded(buffer, count, &writer.output);
        writer.failed = accepted != count;
        return accepted;
    }
};

}  // namespace

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

// The scan's summary, line for line as neo-j2k-testkit/src/scan.rs writes it.
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

uint32_t nj2k_shim_dwt_decode(int32_t* plane, size_t size, uint32_t stride, const nj2k_shim_wavelet_rect_t* resolutions,
                              size_t count, uint32_t reversible)
{
    using namespace nj2k::detail;
    if (!plane || !resolutions || count == 0 || count > kMaxResolutions) {
        return 0;
    }
    ResRect rects[kMaxResolutions];
    constexpr auto max_coordinate = static_cast<uint32_t>(std::numeric_limits<int32_t>::max());
    const auto half = [](uint32_t n) { return n / 2 + (n & 1u); };
    for (size_t i = 0; i < count; ++i) {
        const auto& r = resolutions[i];
        if (r.x0 > r.x1 || r.y0 > r.y1 || r.x1 > max_coordinate || r.y1 > max_coordinate) {
            return 0;
        }
        if (i > 0) {
            const auto& low = resolutions[i - 1];
            if (low.x0 != half(r.x0) || low.y0 != half(r.y0) || low.x1 != half(r.x1) || low.y1 != half(r.y1)) {
                return 0;
            }
        }
        rects[i] = ResRect{static_cast<int32_t>(r.x0), static_cast<int32_t>(r.y0), static_cast<int32_t>(r.x1),
                           static_cast<int32_t>(r.y1)};
    }
    const auto& last = resolutions[count - 1];
    const size_t width = last.x1 - last.x0, height = last.y1 - last.y0;
    if (width > stride || (stride && height > size / stride)) {
        return 0;
    }
    if (count == 1 || !width || !height) {
        return 1;
    }
    // dwt_scratch_bytes needs at most 64 bytes per sample of the longer side.
    constexpr size_t alignment = 64;
    constexpr size_t max_side = std::numeric_limits<size_t>::max() / alignment;
    if (width > max_side || height > max_side) {
        return 0;
    }
    const auto numres = static_cast<uint32_t>(count);
    const size_t bytes = dwt_scratch_bytes(rects, numres, reversible != 0);
    struct alignas(alignment) ScratchBlock {
        char bytes[alignment];
    };
    try {
        // Zero the unused lanes of partial column groups as well.
        std::vector<ScratchBlock> scratch((bytes + alignment - 1) / alignment);
        const DefaultFpEnv fp;
        dwt_decode(plane, stride, rects, numres, reversible != 0, scratch.data());
        return 1;
    } catch (...) {
        return 0;
    }
}

int32_t nj2k_shim_encode_u8(const nj2k_shim_encode_request_t* request, nj2k_shim_codestream_t** out)
{
    return nj2k_shim_encode_u8_with_updates(request, nullptr, out);
}

static int32_t encode_u8(const nj2k_shim_encode_request_t* request, nj2k_shim_encode_updates_t* updates,
                         const nj2k_shim_encode_trace_request_t* trace, nj2k_shim_codestream_t& output)
{
    using nj2k::detail::Encoder;
    static_assert(sizeof(nj2k_shim_encode_updates_t::ratios) == sizeof(float) * Encoder::kMaxLayers);
    if (updates) {
        *updates = {};
    }
    if (!request || !request->pixels || !request->components || !request->width || !request->height ||
        (request->rate_count && !request->rates) || (trace && !trace->buffer_size)) {
        return NJ2K_E_ARGUMENT;
    }
    const auto& r = *request;
    if (r.components > Encoder::kMaxComponents || r.width > Encoder::kMaxSide || r.height > Encoder::kMaxSide ||
        uint64_t(r.width) * r.height * r.components > Encoder::kMaxSamples || r.rate_count > Encoder::kMaxLayers) {
        return NJ2K_E_LIMIT;
    }
    const size_t row_size = size_t(r.width) * r.components;
    if (r.stride < row_size || r.size < row_size || r.height - 1 > (r.size - row_size) / r.stride ||
        r.resolutions > static_cast<uint32_t>(std::numeric_limits<int>::max()) ||
        r.max_codestream_size > static_cast<uint32_t>(std::numeric_limits<int>::max())) {
        return NJ2K_E_ARGUMENT;
    }

    try {
        nj2k_image_cmptparm_t components[Encoder::kMaxComponents]{};
        for (uint32_t c = 0; c < r.components; ++c) {
            components[c].dx = components[c].dy = 1;
            components[c].w = r.width;
            components[c].h = r.height;
            components[c].prec = 8;
        }
        const std::unique_ptr<nj2k_image_t, decltype(&nj2k_image_destroy)> image(
            nj2k_image_create(r.components, components, NJ2K_CLRSPC_SRGB), nj2k_image_destroy);
        const std::unique_ptr<nj2k_codec_t, decltype(&nj2k_destroy_codec)> codec(
            nj2k_create_compress(NJ2K_CODEC_J2K), nj2k_destroy_codec);
        if (!image || !codec) {
            return NJ2K_E_NO_MEMORY;
        }
        CaptureDiagnostics messages(Encoder::from(codec.get())->messages, output.diagnostics, trace != nullptr);
        image->x1 = r.width;
        image->y1 = r.height;

        nj2k_cparameters_t params{};
        nj2k_set_default_encoder_parameters(&params);
        params.prog_order = NJ2K_RLCP;
        params.cp_disto_alloc = 1;
        params.tcp_numlayers = static_cast<int>(r.rate_count);
        params.numresolution = static_cast<int>(r.resolutions);
        params.irreversible = r.reversible ? 0 : 1;
        params.tcp_mct = static_cast<char>(r.mct ? 1 : 0);
        params.max_cs_size = static_cast<int>(r.max_codestream_size);
        if (r.rate_count) {
            std::memcpy(params.tcp_rates, r.rates, r.rate_count * sizeof(float));
        }
        std::string comment;
        if (r.comment) {
            // The C++ parameter is a NUL-terminated string, although the FFI takes a byte slice.
            comment.assign(reinterpret_cast<const char*>(r.comment), r.comment_len);
            params.cp_comment = comment.data();
        }
        if (!nj2k_setup_encoder(codec.get(), &params, image.get())) {
            return messages.failed() ? NJ2K_E_NO_MEMORY : nj2k_get_status(codec.get());
        }
        if (updates) {
            updates->max_codestream_size = static_cast<uint32_t>(params.max_cs_size);
            updates->layer_count = static_cast<uint32_t>(params.tcp_numlayers);
            std::memcpy(updates->ratios, params.tcp_rates, updates->layer_count * sizeof(float));
        }
        if (trace && !nj2k_encoder_set_extra_options(codec.get(), trace->extra_options)) {
            return messages.failed() ? NJ2K_E_NO_MEMORY : nj2k_get_status(codec.get());
        }
        for (uint32_t c = 0; c < r.components; ++c) {
            for (uint32_t y = 0; y < r.height; ++y) {
                const size_t start = size_t(r.flip_vertical ? r.height - 1 - y : y) * r.stride + c;
                for (uint32_t x = 0; x < r.width; ++x) {
                    image->comps[c].data[size_t(y) * r.width + x] = r.pixels[start + size_t(x) * r.components];
                }
            }
        }

        const std::unique_ptr<nj2k_stream_t, decltype(&nj2k_stream_destroy)> stream(
            nj2k_stream_create(trace ? trace->buffer_size : 4096, NJ2K_FALSE), nj2k_stream_destroy);
        if (!stream) {
            return NJ2K_E_NO_MEMORY;
        }
        TracedWriter writer{output, trace};
        nj2k_stream_set_user_data(stream.get(), trace ? static_cast<void*>(&writer) : &output, nullptr);
        nj2k_stream_set_write_function(stream.get(), trace ? TracedWriter::write : append_encoded);
        const bool complete = nj2k_start_compress(codec.get(), image.get(), stream.get()) &&
                              nj2k_encode(codec.get(), stream.get()) && nj2k_end_compress(codec.get(), stream.get());
        if (writer.failed || messages.failed()) {
            return NJ2K_E_NO_MEMORY;
        }
        return complete ? NJ2K_OK : nj2k_get_status(codec.get());
    } catch (...) {
        return NJ2K_E_NO_MEMORY;
    }
}

static int32_t encode_u8_snapshot(const nj2k_shim_encode_request_t* request, nj2k_shim_encode_updates_t* updates,
                                  const nj2k_shim_encode_trace_request_t* trace, nj2k_shim_codestream_t** out)
{
    if (!out) {
        return NJ2K_E_ARGUMENT;
    }
    *out = nullptr;
    try {
        auto codestream = std::make_unique<nj2k_shim_codestream_t>();
        const int32_t status = encode_u8(request, updates, trace, *codestream);
        if (trace || status == NJ2K_OK) {
            *out = codestream.release();
        }
        return status;
    } catch (...) {
        return NJ2K_E_NO_MEMORY;
    }
}

int32_t nj2k_shim_encode_u8_with_updates(const nj2k_shim_encode_request_t* request,
                                       nj2k_shim_encode_updates_t* updates, nj2k_shim_codestream_t** out)
{
    return encode_u8_snapshot(request, updates, nullptr, out);
}

int32_t nj2k_shim_encode_u8_trace(const nj2k_shim_encode_request_t* request,
                                 const nj2k_shim_encode_trace_request_t* trace, nj2k_shim_codestream_t** out)
{
    return encode_u8_snapshot(request, nullptr, trace, out);
}

const size_t* nj2k_shim_codestream_offers(const nj2k_shim_codestream_t* codestream, size_t* count)
{
    *count = codestream->offers.size();
    return codestream->offers.data();
}

uint32_t nj2k_shim_codestream_diagnostic(const nj2k_shim_codestream_t* codestream, size_t index, uint32_t* severity,
                                         const uint8_t** text, size_t* len)
{
    return diagnostic_at(codestream->diagnostics, index, severity, text, len);
}

const uint8_t* nj2k_shim_codestream_bytes(const nj2k_shim_codestream_t* codestream, size_t* len)
{
    *len = codestream->bytes.size();
    return codestream->bytes.data();
}

void nj2k_shim_codestream_free(nj2k_shim_codestream_t* codestream)
{
    delete codestream;
}

int32_t nj2k_shim_generate_codestream(uint32_t seed, uint32_t index, nj2k_shim_codestream_t** out)
{
    if (!out) {
        return NJ2K_E_ARGUMENT;
    }
    *out = nullptr;
    try {
        const auto spec = nj2k::test::generated_case(seed, index);
        auto codestream = std::make_unique<nj2k_shim_codestream_t>();
        std::string error;
        if (!nj2k::test::encode_generated(spec, spec.sop, spec.eph, codestream->bytes, error)) {
            std::fprintf(stderr, "OpenJPEG generation %08x/%u: %s\n", seed, index, error.c_str());
            return NJ2K_E_DATA;
        }
        *out = codestream.release();
        return NJ2K_OK;
    } catch (...) {
        return NJ2K_E_NO_MEMORY;
    }
}

uint32_t nj2k_shim_codec_set_max_memory(nj2k_codec_t* codec, uint64_t max_memory)
{
    nj2k_decoder_limits_t limits;
    nj2k_decoder_limits_init(&limits, NJ2K_DECODER_LIMITS_NONE);
    limits.max_memory = max_memory;
    return codec && nj2k_decoder_set_limits(codec, &limits) ? 1u : 0u;
}

int32_t nj2k_shim_codec_set_reduce(nj2k_codec_t* codec, uint32_t reduce)
{
    if (!codec) {
        return NJ2K_E_ARGUMENT;
    }
    nj2k_set_decoded_resolution_factor(codec, reduce);
    return nj2k_get_status(codec);
}

nj2k_codec_t* nj2k_shim_codec_new(uint32_t format, uint32_t reduce, uint32_t reduce_policy,
                                  const nj2k_decoder_limits_t* limits, uint32_t max_layers, uint32_t strict)
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
    p.cp_layer = max_layers;
    if (!nj2k_setup_decoder(codec, &p) || !nj2k_decoder_set_strict_mode(codec, strict ? NJ2K_TRUE : NJ2K_FALSE) ||
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
            CaptureDiagnostics capture(decoder->messages, header->diagnostics, capture_diagnostics != 0);
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
    return header ? diagnostic_at(header->diagnostics, index, severity, text, len) : 0;
}

void nj2k_shim_header_free(nj2k_shim_header_t* header)
{
    delete header;
}

uint32_t nj2k_shim_codec_set_retained_bytes(nj2k_codec_t* codec, size_t retained_bytes)
{
    return nj2k_decoder_set_retained_bytes(codec, retained_bytes) ? 1u : 0u;
}

nj2k_shim_pool_t* nj2k_shim_pool_new(uint32_t workers)
{
    try {
        return new nj2k_shim_pool_t(workers);
    } catch (...) {
        return nullptr;
    }
}

void nj2k_shim_pool_free(nj2k_shim_pool_t* pool)
{
    delete pool;
}

uint64_t nj2k_shim_pool_batches(const nj2k_shim_pool_t* pool)
{
    return pool->threads.batches();
}

uint32_t nj2k_shim_pool_run(nj2k_shim_pool_t* pool, uint32_t count, nj2k_job_fn job, void* context)
{
    try {
        nj2k::test::ThreadPool::run(&pool->threads, count, job, context);
        return 1;
    } catch (const std::bad_alloc&) {
        // The only allocation queues the batch, before any worker can see the context.
        return 0;
    } catch (...) {
        // A system error after queuing a batch cannot return through the C boundary.
        std::terminate();
    }
}

int32_t nj2k_shim_codec_decode_i32(nj2k_codec_t* codec, const uint8_t* data, size_t size, int32_t* out, uint64_t cap,
                                   uint64_t* count, nj2k_shim_pool_t* pool)
{
    if (!codec || (!data && size) || !count || (!out && cap)) {
        return NJ2K_E_ARGUMENT;
    }
    *count = 0;
    nj2k_image_t image;
    if (!nj2k_read_header_memory(codec, data, size, &image)) {
        return nj2k_get_status(codec);
    }
    // The size-only call is used to allocate the Rust output buffer. Apply the same decoded-side
    // and working-memory checks as the subsequent decode before exposing that size; otherwise a
    // hostile but syntactically valid SIZ marker can make the harness allocate from unchecked
    // header dimensions even though nj2k_decode_memory() would reject them.
    if (!nj2k_get_decoded_dimensions(codec, &image)) {
        return nj2k_get_status(codec);
    }
    try {
        std::uint64_t at = 0;
        for (std::uint32_t c = 0; c < image.numcomps; ++c) {
            at += std::uint64_t(image.comps[c].w) * image.comps[c].h;
        }
        *count = at;
        if (!out) {
            return NJ2K_OK;
        }
        if (at > cap) {
            return NJ2K_E_OUTPUT;
        }
        std::vector<nj2k_plane_t> planes;
        planes.reserve(image.numcomps);
        at = 0;
        for (std::uint32_t c = 0; c < image.numcomps; ++c) {
            const auto& comp = image.comps[c];
            if (comp.w && comp.h) {
                planes.push_back(nj2k_plane_t{out + at, std::int64_t(comp.w) * 4, 4, c});
                at += std::uint64_t(comp.w) * comp.h;
            }
        }
        const nj2k_output_t output{sizeof(nj2k_output_t), NJ2K_SAMPLE_I32, static_cast<uint32_t>(planes.size()),
                                   planes.data()};
        const BorrowedExecutor executor(codec, pool);
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

int32_t nj2k_shim_codec_dimensions_for(nj2k_codec_t* codec, const uint8_t* data, size_t size,
                                      const uint32_t* indices, uint32_t count, uint32_t* numcomps,
                                      nj2k_shim_header_t** out)
{
    *out = nullptr;
    nj2k_image_t image{};
    if (!nj2k_read_header_memory(codec, data, size, &image) ||
        !nj2k_set_decoded_components(codec, count, indices, NJ2K_FALSE) || !nj2k_get_decoded_dimensions(codec, &image)) {
        return nj2k_get_status(codec);
    }
    try {
        auto header = std::make_unique<nj2k_shim_header>();
        header->sized = copy_components(image);
        *numcomps = image.numcomps;
        *out = header.release();
        return NJ2K_OK;
    } catch (...) {
        return NJ2K_E_NO_MEMORY;
    }
}

}  // extern "C"
