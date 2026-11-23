// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The Tier-1 half of the test-only C interface (nj2k_rust_shim.h): the decoder's code-block (t1_decode_cblk and the store
// into a tile buffer), the encoder's (enc_t1_encode and its commit), and single MQ decisions of the decoder (the macro of
// t1_mq.h) and the encoder (MqEnc of enc_mq.h) on registers and byte strings the caller gives. Each entry is a thin
// call of the C++ the library itself compiled; a driver adds nothing the decoder's or encoder's own code does not.
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
    if (!t1 || !rq || !out || rq->w == 0 || rq->h == 0 || rq->w > 1024 || rq->h > 1024 || rq->w * rq->h > d::T1::kMaxData ||
        out_len < std::size_t(rq->w) * rq->h || (rq->nsegs && (!rq->seg_len || !rq->seg_passes)) ||
        (rq->nchunks && !rq->chunk_len)) {
        return 0;
    }
    try {
        std::size_t total = 0;
        std::vector<d::Chunk> chunks;
        for (std::size_t i = 0; i < rq->nchunks; ++i) {
            chunks.push_back({rq->bytes + total, rq->chunk_len[i]});
            total += rq->chunk_len[i];
        }
        std::vector<d::Segment> segs(rq->nsegs, d::Segment{});
        for (std::size_t i = 0; i < rq->nsegs; ++i) {
            segs[i].len = rq->seg_len[i];
            segs[i].numpasses = segs[i].real_num_passes = rq->seg_passes[i];
        }
        if (!t1->t1.reserve(t1->alloc, total)) {
            return 0;
        }
        const d::T1Block b{rq->w, rq->h, rq->numbps, rq->orient, rq->roishift, rq->cblksty, chunks.data(),
                           static_cast<std::uint32_t>(chunks.size()), segs.data(), static_cast<std::uint32_t>(segs.size())};
        if (!d::t1_decode_cblk(t1->t1, b, t1->quiet)) {
            return 0;
        }
        d::t1_store(t1->t1.data, rq->w, rq->h, rq->roishift, rq->reversible != 0, rq->stepsize, out, rq->w);
        return 1;
    } catch (...) {
        return 0;
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
    if (!t1 || !rq || !result || !rq->samples || rq->w == 0 || rq->h == 0 || rq->w > 64 || rq->h > 64) {
        return 0;
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
        result->ok = ok ? 1u : 0u;
        result->numbps = c.numbps;
        if (!ok) {
            return 1;
        }
        if (c.totalpasses == 0) {
            return 1;
        }
        d::Arena arena;
        arena.init(&t1->alloc);
        if (c.totalpasses > pass_cap || !d::enc_t1_commit(t1->t1, arena, c)) {
            return 0;
        }
        for (std::uint32_t i = 0; i < c.totalpasses; ++i) {
            passes[i] = {c.passes[i].rate, c.passes[i].len, c.passes[i].term, c.passes[i].distortiondec};
        }
        // the bytes up to the last pass's rate (the committed block holds the coder's count, a byte more when the last
        // pass's rate gave up a trailing 0xFF; the block does not say how many it holds, so no byte past the rate is read)
        const std::uint32_t len = c.passes[c.totalpasses - 1].rate;
        if (len > byte_cap) {
            return 0;
        }
        std::memcpy(bytes, c.data, len);
        result->totalpasses = c.totalpasses;
        result->byte_len = len;
        return 1;
    } catch (...) {
        return 0;
    }
}

uint32_t nj2k_shim_mq_dec_run(const nj2k_shim_mq_dec_regs_t* init, const uint8_t* data, size_t len, const uint8_t* ctx,
                              size_t n, uint8_t* bits, nj2k_shim_mq_dec_regs_t* out)
{
    if (!out || (n && (!ctx || !bits)) || (len && !data)) {
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
        if (init) {
            if (init->bp > len + 1) {
                return 0;
            }
            for (std::uint32_t k = 0; k < 19; ++k) {
                if (init->state[k] >= d::kMqWordCount) {
                    return 0;
                }
                m.ctx.slot[k] = d::kMqWords.w[init->state[k]];
            }
            a = init->a, c = init->c, ct = init->ct;
            m.bp = buf.data() + init->bp;
        } else {
            // opj_mqc_init_dec (ISO 15444-1 C.3.5 INITDEC): the first byte (0xFF when there is none) into c, a byte-in,
            // seven shifts
            m.ctx = d::kMqContextsInit;
            m.bp = buf.data();
            c = len == 0 ? 0xffu << 16 : static_cast<std::uint32_t>(*m.bp) << 16;
            opj_mqc_bytein_macro(&m, c, ct);
            c <<= 7;
            ct -= 7;
            a = 0x8000;
        }
        decode_decisions(m, a, c, ct, ctx, n, bits);
        save_dec(m, buf.data(), a, c, ct, out);
        return 1;
    } catch (...) {
        return 0;
    }
}

void nj2k_shim_mq_dec_cases(const nj2k_shim_mq_dec_case_t* cases, size_t n, nj2k_shim_mq_dec_result_t* results)
{
    for (std::size_t i = 0; i < n; ++i) {
        const nj2k_shim_mq_dec_case_t& k = cases[i];
        const std::uint8_t buf[6] = {k.b[0], k.b[1], k.b[2], k.b[3], 0xFF, 0xFF};
        d::Mqc m{};
        m.ctx.slot[0] = d::kMqWords.w[k.state % d::kMqWordCount];
        m.bp = const_cast<std::uint8_t*>(buf);  // the macro reads the bytes and moves bp; it writes none
        std::uint32_t a = k.a, c = k.c, ct = k.ct;
        const std::uint8_t ctx0 = 0;
        std::uint8_t bit = 0;
        decode_decisions(m, a, c, ct, &ctx0, 1, &bit);
        results[i] = {bit, a, c, ct, static_cast<std::uint32_t>(m.bp - buf), state_index(m.ctx.slot[0])};
    }
}

uint32_t nj2k_shim_mq_enc_run(const nj2k_shim_mq_enc_regs_t* init, const uint8_t* ctxbit, size_t n, uint32_t flush,
                              uint8_t* buf, size_t cap, size_t limit, nj2k_shim_mq_enc_regs_t* out,
                              nj2k_shim_mq_enc_regs_t* trace)
{
    if (!out || !buf || cap < 2 || limit >= cap || (n && !ctxbit)) {
        return 0;
    }
    for (std::size_t i = 0; i < n; ++i) {
        if (ctxbit[i] >= 38) {
            return 0;
        }
    }
    d::MqEnc m{};
    if (init) {
        if (init->bp >= cap) {
            return 0;
        }
        for (std::uint32_t k = 0; k < 19; ++k) {
            if (init->state[k] >= d::kMqStateCount) {
                return 0;
            }
            m.ctxs[k] = init->state[k];
        }
        m.a = init->a, m.c = init->c, m.ct = init->ct;
        m.bp = buf + init->bp;
        m.start = buf + 1;
        m.limit = buf + limit;
        m.overflow = init->overflow != 0;
        m.setcurctx(0);
    } else {
        m.resetstates();
        m.setstate(T1_CTXNO_UNI, 0, 46);
        m.setstate(T1_CTXNO_AGG, 0, 3);
        m.setstate(T1_CTXNO_ZC, 0, 4);
        m.init_enc(buf + 1, buf + limit);
    }
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

void nj2k_shim_mq_enc_cases(const nj2k_shim_mq_enc_case_t* cases, size_t n, nj2k_shim_mq_enc_result_t* results)
{
    for (std::size_t i = 0; i < n; ++i) {
        const nj2k_shim_mq_enc_case_t& k = cases[i];
        std::uint8_t buf[8];
        std::memset(buf, static_cast<int>(k.prev), sizeof buf);
        d::MqEnc m{};
        m.resetstates();
        m.setstate(0, k.state & 1u, static_cast<std::int32_t>(k.state >> 1));
        m.setcurctx(0);
        m.a = k.a;
        m.c = k.c;
        m.ct = k.ct;
        m.bp = buf + 2;
        m.start = buf + 3;
        m.limit = buf + 2 + k.limit_off;
        m.overflow = false;
        m.encode(k.bit);
        nj2k_shim_mq_enc_result_t& r = results[i];
        r.a = m.a;
        r.c = m.c;
        r.ct = m.ct;
        r.state = m.ctxs[0];
        r.bp = static_cast<std::uint32_t>(m.bp - buf);
        r.overflow = m.overflow ? 1u : 0u;
        std::memcpy(r.bytes, buf, sizeof buf);
    }
}

}  // extern "C"
