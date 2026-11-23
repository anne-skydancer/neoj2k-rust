// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The Tier-1 half of the test-only C interface (nj2k_rust_shim.h): the decoder's code-block (t1_decode_cblk and the store
// into a tile buffer), the encoder's (enc_t1_encode and its commit), and single MQ decisions of the decoder (the macro of
// t1_mq.h) and the encoder (MqEnc of enc_mq.h) on registers and byte strings the caller gives.
//
// What is the C++'s own and what is restated here: the code-block entries call t1_decode_cblk, t1_store, enc_t1_encode and
// enc_t1_commit themselves. The MQ entries call the decoder's macro and MqEnc's members themselves, but not the sequences
// that start them: INITDEC is local to src/t1.cpp, so nj2k_shim_mq_dec_run restates it (the first byte into c, a byte-in,
// seven shifts, a = 0x8000, from opj_mqc_init_dec), and nj2k_shim_mq_enc_run restates enc_t1.cpp's start (resetstates,
// setstate of UNI, AGG and ZC to 46, 3 and 4, init_enc). The stream tests therefore compare Rust with a copy of that
// initialisation at the pinned tag; the code-block tests compare it with the real one.
//
// Every entry checks its request and returns 0 (INVALID) instead of running the C++ on a value it is not meant to take.
#include "nj2k_rust_shim.h"

#include <algorithm>
#include <cstring>
#include <memory>
#include <new>
#include <vector>

// The C++ checkout exposes its internals to this test-only target through its src include path.
#include "arena.h"
#include "enc_mq.h"
#include "enc_tile.h"
#include "memory.h"
#include "messages.h"
#include "t1.h"
#include "t1_flags.h"
#include "t1_mq.h"

namespace d = nj2k::detail;

// the records the Rust bindings (neo-j2k-oracle/src/tier1.rs) mirror: their sizes are asserted there too
static_assert(sizeof(nj2k_shim_t1_decode_request_t) == 88, "mirrored in tier1.rs");
static_assert(sizeof(nj2k_shim_enc_t1_request_t) == 48, "mirrored in tier1.rs");
static_assert(sizeof(nj2k_shim_enc_pass_t) == 24, "mirrored in tier1.rs");
static_assert(sizeof(nj2k_shim_enc_t1_result_t) == 12, "mirrored in tier1.rs");
static_assert(sizeof(nj2k_shim_mq_dec_regs_t) == 36, "mirrored in tier1.rs");
static_assert(sizeof(nj2k_shim_mq_dec_case_t) == 20, "mirrored in tier1.rs");
static_assert(sizeof(nj2k_shim_mq_dec_result_t) == 24, "mirrored in tier1.rs");
static_assert(sizeof(nj2k_shim_mq_enc_regs_t) == 40, "mirrored in tier1.rs");
static_assert(sizeof(nj2k_shim_mq_enc_case_t) == 28, "mirrored in tier1.rs");
static_assert(sizeof(nj2k_shim_mq_enc_result_t) == 32, "mirrored in tier1.rs");
static_assert(sizeof(nj2k_shim_mq_flush_case_t) == 20, "mirrored in tier1.rs");

#if defined(_MSC_VER)
#pragma warning(push)
#pragma warning(disable : 4324)  // the decoder's scratch is padded to a whole cache line, as intended
#endif
struct nj2k_shim_t1 {
    d::Allocator alloc = d::Allocator::system();
    d::T1 t1{};
    d::Messages quiet;
    ~nj2k_shim_t1() { t1.release(alloc); }
};
#if defined(_MSC_VER)
#pragma warning(pop)
#endif

struct nj2k_shim_enc_t1 {
    d::Allocator alloc = d::Allocator::system();
    d::EncT1* t1 = nullptr;
    ~nj2k_shim_enc_t1() { d::enc_t1_destroy(t1, alloc); }
};

namespace {

// the interval of a renormalised decision or flush, and the bounds of the single-decision buffers
constexpr std::uint32_t kAMin = 0x8000U, kAMax = 0xFFFFU;
constexpr std::uint32_t kMaxLimitOffset = 5;  // the buffers hold eight bytes, the write position starts at 2
constexpr std::size_t kMaxSegments = 4096;

// The index of a context's state word in the decoder's table of 94 (the trace of t1.cpp finds it the same way).
std::uint8_t state_index(std::uint32_t word) noexcept
{
    std::uint32_t i = 0;
    while (i < d::kMqWordCount && d::kMqWords.w[i] != word) {
        ++i;
    }
    return static_cast<std::uint8_t>(i);
}

// The decisions of the decoder's macro on the registers a, c, ct and the context slots of m (bp inside m): what the
// passes of t1.cpp expand on their locals.
void decode_decisions(d::Mqc& m, std::uint32_t& a, std::uint32_t& c, std::uint32_t& ct, const std::uint8_t* ctx,
                      std::size_t n, std::uint8_t* bits) noexcept
{
    for (std::size_t i = 0; i < n; ++i) {
        std::uint32_t* const curctx = &m.ctx.slot[ctx[i]];
        std::uint32_t bit;
        opj_mqc_decode_macro(bit, &m, curctx, a, c, ct);
        bits[i] = static_cast<std::uint8_t>(bit);
    }
}

void save_dec(const d::Mqc& m, const std::uint8_t* base, std::uint32_t a, std::uint32_t c, std::uint32_t ct,
              nj2k_shim_mq_dec_regs_t* out) noexcept
{
    out->a = a;
    out->c = c;
    out->ct = ct;
    out->bp = static_cast<std::uint32_t>(m.bp - base);
    for (std::uint32_t k = 0; k < 19; ++k) {
        out->state[k] = state_index(m.ctx.slot[k]);
    }
}

void save_enc(const d::MqEnc& m, const std::uint8_t* buf, nj2k_shim_mq_enc_regs_t* out) noexcept
{
    out->a = m.a;
    out->c = m.c;
    out->ct = m.ct;
    out->bp = static_cast<std::uint32_t>(m.bp - buf);
    out->overflow = m.overflow ? 1u : 0u;
    std::memcpy(out->state, m.ctxs, 19);
}

bool valid_interval(std::uint32_t a) noexcept
{
    return a >= kAMin && a <= kAMax;
}

// an encoder case's registers and buffer: valid_interval(a), ct 1 to 12, prev a byte, limit_off 0 to 5
bool valid_coder(std::uint32_t a, std::uint32_t ct, std::uint32_t prev, std::uint32_t limit_off) noexcept
{
    return valid_interval(a) && ct >= 1 && ct <= 12 && prev <= 0xFF && limit_off <= kMaxLimitOffset;
}

// the eight-byte buffer of a single-decision case, on which the coder starts at byte 2
void start_case(d::MqEnc& m, std::uint8_t (&buf)[8], std::uint32_t a, std::uint32_t c, std::uint32_t ct,
                std::uint32_t prev, std::uint32_t limit_off) noexcept
{
    std::memset(buf, static_cast<int>(prev), sizeof buf);
    m = d::MqEnc{};
    m.resetstates();
    m.setcurctx(0);
    m.a = a;
    m.c = c;
    m.ct = ct;
    m.bp = buf + 2;
    m.start = buf + 3;
    m.limit = buf + 2 + limit_off;
    m.overflow = false;
}

void save_case(const d::MqEnc& m, const std::uint8_t (&buf)[8], nj2k_shim_mq_enc_result_t& r) noexcept
{
    r.a = m.a;
    r.c = m.c;
    r.ct = m.ct;
    r.state = m.ctxs[0];
    r.bp = static_cast<std::uint32_t>(m.bp - buf);
    r.overflow = m.overflow ? 1u : 0u;
    std::memcpy(r.bytes, buf, sizeof buf);
}

}  // namespace

extern "C" {

nj2k_shim_t1_t* nj2k_shim_t1_new(void)
{
    try {
        auto h = std::make_unique<nj2k_shim_t1>();
        return h->t1.ready(h->alloc) ? h.release() : nullptr;
    } catch (...) {
        return nullptr;
    }
}

void nj2k_shim_t1_free(nj2k_shim_t1_t* t1)
{
    delete t1;
}

uint32_t nj2k_shim_t1_decode(nj2k_shim_t1_t* t1, const nj2k_shim_t1_decode_request_t* rq, int32_t* out, size_t out_len)
{
    if (!t1 || !rq || !out || rq->w == 0 || rq->h == 0 || rq->w > 1024 || rq->h > 1024 ||
        std::uint64_t(rq->w) * rq->h > d::T1::kMaxData || rq->orient > 3 || rq->cblksty > 63 ||
        rq->nsegs > kMaxSegments || rq->nchunks > kMaxSegments || (rq->nsegs && (!rq->seg_len || !rq->seg_passes)) ||
        (rq->nchunks && !rq->chunk_len) || rq->out_stride < rq->w) {
        return NJ2K_SHIM_T1_INVALID;
    }
    // the tile buffer the block goes into: offset + (h - 1) * stride + w samples
    if (std::uint64_t(rq->out_offset) + std::uint64_t(rq->h - 1) * rq->out_stride + rq->w > out_len) {
        return NJ2K_SHIM_T1_INVALID;
    }
    std::uint64_t total = 0;
    for (std::size_t i = 0; i < rq->nchunks; ++i) {
        total += rq->chunk_len[i];
    }
    if (total > (1u << 26) || (total && !rq->bytes)) {
        return NJ2K_SHIM_T1_INVALID;
    }
    try {
        std::vector<d::Chunk> chunks;
        std::size_t at = 0;
        for (std::size_t i = 0; i < rq->nchunks; ++i) {
            chunks.push_back({rq->bytes + at, rq->chunk_len[i]});
            at += rq->chunk_len[i];
        }
        std::vector<d::Segment> segs(rq->nsegs, d::Segment{});
        for (std::size_t i = 0; i < rq->nsegs; ++i) {
            segs[i].len = rq->seg_len[i];
            segs[i].numpasses = segs[i].real_num_passes = rq->seg_passes[i];
        }
        if (!t1->t1.reserve(t1->alloc, static_cast<std::size_t>(total))) {
            return NJ2K_SHIM_T1_INVALID;
        }
        const d::T1Block b{rq->w, rq->h, rq->numbps, rq->orient, rq->roishift, rq->cblksty, chunks.data(),
                           static_cast<std::uint32_t>(chunks.size()), segs.data(), static_cast<std::uint32_t>(segs.size())};
        if (!d::t1_decode_cblk(t1->t1, b, t1->quiet)) {
            return NJ2K_SHIM_T1_REFUSED;
        }
        d::t1_store(t1->t1.data, rq->w, rq->h, rq->roishift, rq->reversible != 0, rq->stepsize, out + rq->out_offset,
                    rq->out_stride);
        return NJ2K_SHIM_T1_DONE;
    } catch (...) {
        return NJ2K_SHIM_T1_INVALID;
    }
}

uint32_t nj2k_shim_t1_store(const int32_t* src, uint32_t w, uint32_t h, uint32_t roishift, uint32_t reversible,
                            float stepsize, int32_t* dst, size_t dst_len, uint32_t dst_stride)
{
    if (!src || !dst || w == 0 || h == 0 || w > 1024 || h > 1024 || std::uint64_t(w) * h > d::T1::kMaxData ||
        dst_stride < w || std::uint64_t(h - 1) * dst_stride + w > dst_len) {
        return NJ2K_SHIM_T1_INVALID;
    }
    try {
        // t1_store undoes the ROI shift in its source: on a copy
        std::vector<std::int32_t> coefficients(src, src + std::size_t(w) * h);
        d::t1_store(coefficients.data(), w, h, roishift, reversible != 0, stepsize, dst, dst_stride);
        return NJ2K_SHIM_T1_DONE;
    } catch (...) {
        return NJ2K_SHIM_T1_INVALID;
    }
}

nj2k_shim_enc_t1_t* nj2k_shim_enc_t1_new(void)
{
    try {
        auto h = std::make_unique<nj2k_shim_enc_t1>();
        h->t1 = d::enc_t1_create(h->alloc);
        return h->t1 ? h.release() : nullptr;
    } catch (...) {
        return nullptr;
    }
}

void nj2k_shim_enc_t1_free(nj2k_shim_enc_t1_t* t1)
{
    delete t1;
}

uint32_t nj2k_shim_enc_t1_encode(nj2k_shim_enc_t1_t* t1, const nj2k_shim_enc_t1_request_t* rq,
                                 nj2k_shim_enc_t1_result_t* result, nj2k_shim_enc_pass_t* passes, size_t pass_cap,
                                 uint8_t* bytes, size_t byte_cap)
{
    if (!t1 || !rq || !result || !rq->samples || rq->w == 0 || rq->h == 0 || rq->w > 64 || rq->h > 64 || rq->orient > 3 ||
        (pass_cap && !passes) || (byte_cap && !bytes)) {
        return NJ2K_SHIM_T1_INVALID;
    }
    *result = {};
    try {
        std::memcpy(d::enc_t1_data(t1->t1), rq->samples, std::size_t(rq->w) * rq->h * sizeof(std::int32_t));
        d::EncCblk c{};
        c.x1 = static_cast<std::int32_t>(rq->w);
        c.y1 = static_cast<std::int32_t>(rq->h);
        const double* norms = rq->mct ? (rq->reversible ? d::kEncMctNorms : d::kEncMctNormsReal) : nullptr;
        const bool ok = d::enc_t1_encode(t1->t1, c, rq->orient, rq->compno, rq->level, rq->reversible != 0, rq->stepsize,
                                         norms, rq->mct ? 3u : 0u);
        result->numbps = c.numbps;
        if (!ok) {
            return NJ2K_SHIM_T1_REFUSED;
        }
        if (c.totalpasses == 0) {
            return NJ2K_SHIM_T1_DONE;
        }
        d::Arena arena;
        arena.init(&t1->alloc);
        if (c.totalpasses > pass_cap || !d::enc_t1_commit(t1->t1, arena, c)) {
            return NJ2K_SHIM_T1_INVALID;
        }
        for (std::uint32_t i = 0; i < c.totalpasses; ++i) {
            passes[i] = {c.passes[i].rate, c.passes[i].len, c.passes[i].term, c.passes[i].distortiondec};
        }
        // the bytes up to the last pass's rate (the committed block holds the coder's count, a byte more when the last
        // pass's rate gave up a trailing 0xFF; the block does not say how many it holds, so no byte past the rate is read)
        const std::uint32_t len = c.passes[c.totalpasses - 1].rate;
        if (len > byte_cap) {
            return NJ2K_SHIM_T1_INVALID;
        }
        std::memcpy(bytes, c.data, len);
        result->totalpasses = c.totalpasses;
        result->byte_len = len;
        return NJ2K_SHIM_T1_DONE;
    } catch (...) {
        return NJ2K_SHIM_T1_INVALID;
    }
}

uint32_t nj2k_shim_mq_dec_run(const uint8_t* data, size_t len, const uint8_t* ctx, size_t n, uint8_t* bits,
                              nj2k_shim_mq_dec_regs_t* out)
{
    if (!out || (n && (!ctx || !bits)) || (len && !data) || len > (1u << 26)) {
        return 0;
    }
    for (std::size_t i = 0; i < n; ++i) {
        if (ctx[i] >= 19) {
            return 0;
        }
    }
    try {
        std::vector<std::uint8_t> buf(len + 2, 0xFF);  // the 0xFF 0xFF opj_mqc_init_dec_common puts after the segment
        if (len) {
            std::memcpy(buf.data(), data, len);
        }
        d::Mqc m{};
        std::uint32_t a, c, ct;
        // the restatement of opj_mqc_init_dec (ISO 15444-1 C.3.5 INITDEC): the first byte (0xFF when there is none) into
        // c, a byte-in, seven shifts, a = 0x8000, every context at its initial state
        m.ctx = d::kMqContextsInit;
        m.bp = buf.data();
        c = len == 0 ? 0xffu << 16 : static_cast<std::uint32_t>(*m.bp) << 16;
        opj_mqc_bytein_macro(&m, c, ct);
        c <<= 7;
        ct -= 7;
        a = 0x8000;
        decode_decisions(m, a, c, ct, ctx, n, bits);
        save_dec(m, buf.data(), a, c, ct, out);
        return 1;
    } catch (...) {
        return 0;
    }
}

uint32_t nj2k_shim_mq_dec_cases(const nj2k_shim_mq_dec_case_t* cases, size_t n, nj2k_shim_mq_dec_result_t* results)
{
    if (n && (!cases || !results)) {
        return 0;
    }
    for (std::size_t i = 0; i < n; ++i) {
        const nj2k_shim_mq_dec_case_t& k = cases[i];
        if (!valid_interval(k.a) || k.state >= d::kMqWordCount) {
            return 0;
        }
        const std::uint8_t buf[6] = {k.b[0], k.b[1], k.b[2], k.b[3], 0xFF, 0xFF};
        d::Mqc m{};
        m.ctx.slot[0] = d::kMqWords.w[k.state];
        m.bp = const_cast<std::uint8_t*>(buf);  // the macro reads the bytes and moves bp; it writes none
        std::uint32_t a = k.a, c = k.c, ct = k.ct;
        const std::uint8_t ctx0 = 0;
        std::uint8_t bit = 0;
        decode_decisions(m, a, c, ct, &ctx0, 1, &bit);
        results[i] = {bit, a, c, ct, static_cast<std::uint32_t>(m.bp - buf), state_index(m.ctx.slot[0])};
    }
    return 1;
}

uint32_t nj2k_shim_mq_enc_run(const uint8_t* ctxbit, size_t n, uint32_t flush, uint8_t* buf, size_t cap, size_t limit,
                              nj2k_shim_mq_enc_regs_t* out, nj2k_shim_mq_enc_regs_t* trace)
{
    if (!out || !buf || cap < 2 || limit < 1 || limit >= cap || (n && !ctxbit)) {
        return 0;
    }
    for (std::size_t i = 0; i < n; ++i) {
        if (ctxbit[i] >= 38) {
            return 0;
        }
    }
    d::MqEnc m{};
    // the restatement of enc_t1.cpp's start of a code-block's coder
    m.resetstates();
    m.setstate(T1_CTXNO_UNI, 0, 46);
    m.setstate(T1_CTXNO_AGG, 0, 3);
    m.setstate(T1_CTXNO_ZC, 0, 4);
    m.init_enc(buf + 1, buf + limit);
    for (std::size_t i = 0; i < n; ++i) {
        m.setcurctx(ctxbit[i] >> 1);
        m.encode(ctxbit[i] & 1u);
        if (trace) {
            save_enc(m, buf, &trace[i]);
        }
    }
    if (flush) {
        m.flush();
    }
    save_enc(m, buf, out);
    return 1;
}

uint32_t nj2k_shim_mq_enc_cases(const nj2k_shim_mq_enc_case_t* cases, size_t n, nj2k_shim_mq_enc_result_t* results)
{
    if (n && (!cases || !results)) {
        return 0;
    }
    for (std::size_t i = 0; i < n; ++i) {
        const nj2k_shim_mq_enc_case_t& k = cases[i];
        if (!valid_coder(k.a, k.ct, k.prev, k.limit_off) || k.state >= d::kMqStateCount || k.bit > 1) {
            return 0;
        }
        std::uint8_t buf[8];
        d::MqEnc m;
        start_case(m, buf, k.a, k.c, k.ct, k.prev, k.limit_off);
        m.setstate(0, k.state & 1u, static_cast<std::int32_t>(k.state >> 1));
        m.encode(k.bit);
        save_case(m, buf, results[i]);
    }
    return 1;
}

uint32_t nj2k_shim_mq_flush_cases(const nj2k_shim_mq_flush_case_t* cases, size_t n, nj2k_shim_mq_enc_result_t* results)
{
    if (n && (!cases || !results)) {
        return 0;
    }
    for (std::size_t i = 0; i < n; ++i) {
        const nj2k_shim_mq_flush_case_t& k = cases[i];
        if (!valid_coder(k.a, k.ct, k.prev, k.limit_off)) {
            return 0;
        }
        std::uint8_t buf[8];
        d::MqEnc m;
        start_case(m, buf, k.a, k.c, k.ct, k.prev, k.limit_off);
        m.flush();
        save_case(m, buf, results[i]);
    }
    return 1;
}

}  // extern "C"
