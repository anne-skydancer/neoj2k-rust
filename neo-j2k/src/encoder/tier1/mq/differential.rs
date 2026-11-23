// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The MQ encoder decision by decision against the C++'s (`MqEnc` of `enc_mq.h`, driven through the oracle's
//! shim):
//!
//! - streams: random decision streams over the 19 contexts at full capacity and at short ones (the output clamp
//!   at every byte position of a short buffer and around the length of the stream); after every decision the
//!   registers, the write position, the overflow flag and the contexts equal, and after the flush every byte;
//! - single decisions from registers the test sets, over the C++ unit test's enumeration (`test_single` of
//!   `tests/unit/enc_t1_unit.cpp`): every interval, state, bit and bit counter 1 to 12, the code register's low
//!   word, the previous byte (00, FE, FF) and the output limit at the previous byte, one, two or three after it
//!   or out of reach. The normal suite takes every fourth interval with one pseudo-random combination of the last three
//!   for each (state, bit, counter), and every combination at the intervals where the choices change (18 million and
//!   a million decisions); `nightly_mq_encoder_every_decision` takes every interval and all 60, 4.4 billion, on every
//!   thread.

use neo_j2k_oracle::tier1::{MqEncodeCase, MqEncodeResult, MqEncoderRegs, mq_encode, mq_encode_cases};

use super::{CONTEXT_AGGREGATE, CONTEXT_UNIFORM, Encoder};

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: u32) -> u32 {
        u32::try_from((self.next() >> 11) % u64::from(n)).unwrap()
    }
}

fn registers(encoder: &Encoder<'_>) -> MqEncoderRegs {
    MqEncoderRegs {
        a: encoder.a,
        c: encoder.c,
        ct: encoder.bits,
        bp: u32::try_from(encoder.position).unwrap(),
        overflow: u32::from(encoder.overflow),
        state: encoder.contexts,
    }
}

/// One stream in Rust and in the C++ on a buffer whose last writable byte is `limit`.
fn run_stream(seed: u32, n: u32, limit: usize) -> usize {
    let mut rng = Rng(u64::from(seed) * 7919 + 13);
    // the probability of a 1 in each context: skewed, uniform or alternating, by stream
    let kind = seed % 4;
    let thresholds: Vec<u32> = (0..19)
        .map(|_| match kind {
            0 => 32_768,
            1 => 200 + rng.below(65_000),
            2 => {
                if rng.below(2) == 1 {
                    65_000
                } else {
                    500
                }
            }
            _ => rng.below(65_536),
        })
        .collect();
    let mut previous = 0u32;
    let mut decisions = Vec::new();
    for _ in 0..n {
        // contexts in runs, as the passes use them
        let ctx = if rng.below(4) == 0 { rng.below(19) } else { previous };
        previous = ctx;
        let bit = u32::from(rng.below(65_536) < thresholds[ctx as usize]);
        decisions.push(u8::try_from(ctx * 2 + bit).unwrap());
    }

    // the C++: the fake byte at 0, the coder's last byte at `limit`, two bytes of slack
    let mut cpp_buf = vec![0u8; limit + 2];
    let (want, trace) = mq_encode(None, &decisions, true, &mut cpp_buf, limit);

    let mut bytes = vec![0u8; limit + 1];
    let mut encoder = Encoder::new(&mut bytes);
    for (k, &ctxbit) in decisions.iter().enumerate() {
        encoder.symbol(usize::from(ctxbit >> 1), u32::from(ctxbit & 1));
        assert_eq!(
            registers(&encoder),
            trace[k],
            "stream {seed} limit {limit}: the coders differ after decision {k} (ctx {} bit {})",
            ctxbit >> 1,
            ctxbit & 1
        );
        // what a pass end records: the bytes written so far
        assert_eq!(
            encoder.rate(),
            trace[k].bp.wrapping_sub(1),
            "stream {seed} limit {limit}: rate after decision {k}"
        );
    }
    encoder.flush();
    let total = encoder.position;
    assert_eq!(
        registers(&encoder),
        want,
        "stream {seed} limit {limit}: after the flush"
    );
    // every byte of the buffer: the ones the coder wrote, and zeros past them (the C++ has two bytes of slack more)
    assert_eq!(bytes[..], cpp_buf[..=limit], "stream {seed} limit {limit}: bytes");
    total
}

fn run_streams(first: u32, count: u32) -> u32 {
    let mut runs = 0;
    for seed in first..first + count {
        let n = 200 + (seed * 37) % 4_000;
        // the capacity of a full block, then short ones: every byte position up to 40, around the length
        let length = run_stream(seed, n, 1 << 16);
        runs += 1;
        let mut limits: Vec<usize> = (1..=40).collect();
        limits.extend(length.saturating_sub(4).max(1)..=length + 4);
        limits.extend([length / 2 + 1, length / 3 + 1, length / 5 + 1]);
        for limit in limits {
            run_stream(seed, n, limit);
            runs += 1;
        }
    }
    runs
}

#[test]
fn streams_encode_as_the_cpp_does() {
    let runs = run_streams(0, 24);
    assert!(runs > 1_000);
}

#[test]
fn a_fresh_encoder_starts_where_the_cpp_does() {
    let mut buf = vec![0u8; 16];
    let (fresh, _) = mq_encode(None, &[], false, &mut buf, 8);
    let mut bytes = vec![0u8; 9];
    let encoder = Encoder::new(&mut bytes);
    assert_eq!(registers(&encoder), fresh);
    assert_eq!(
        (
            encoder.contexts[0],
            encoder.contexts[CONTEXT_AGGREGATE],
            encoder.contexts[CONTEXT_UNIFORM]
        ),
        (8, 6, 92)
    );
}

const COMBO_C: [u32; 4] = [0, 0x07ff_fff0, 0x0800_0000, 0x035a_5a5a];
const COMBO_PREV: [u32; 3] = [0x00, 0xfe, 0xff];

/// Combination `combo` of 60: the code register's low word, the previous byte and the limit offset.
fn combo(combo: u32) -> (u32, u32, u32) {
    (
        COMBO_C[(combo & 3) as usize],
        COMBO_PREV[((combo >> 2) % 3) as usize],
        combo / 12,
    )
}

/// One decision of Rust from the case's registers, in eight bytes of the previous byte as the C++'s.
fn rust_case(case: &MqEncodeCase) -> MqEncodeResult {
    // the buffer ends at the limit: the coder may write up to it
    let length = 3 + case.limit_off as usize;
    let prev = u8::try_from(case.prev).unwrap();
    let mut bytes = vec![prev; length];
    let mut encoder = Encoder::new(&mut bytes);
    encoder.bytes[0] = prev;
    encoder.position = 2;
    encoder.a = case.a;
    encoder.c = case.c;
    encoder.bits = case.ct;
    encoder.contexts = [0; 19];
    encoder.contexts[0] = u8::try_from(case.state).unwrap();
    encoder.overflow = false;
    encoder.symbol(0, case.bit);
    let (a, c, bits, position, overflow, state) = (
        encoder.a,
        encoder.c,
        encoder.bits,
        u32::try_from(encoder.position).unwrap(),
        u32::from(encoder.overflow),
        u32::from(encoder.contexts[0]),
    );
    let mut eight = [prev; 8];
    eight[..length].copy_from_slice(&bytes);
    MqEncodeResult {
        a,
        c,
        ct: bits,
        state,
        bp: position,
        overflow,
        bytes: eight,
    }
}

struct Sweep {
    cases: Vec<MqEncodeCase>,
    results: Vec<MqEncodeResult>,
    decisions: u64,
}

impl Sweep {
    fn new() -> Self {
        Self {
            cases: Vec::with_capacity(1 << 17),
            results: Vec::new(),
            decisions: 0,
        }
    }

    fn flush(&mut self) {
        mq_encode_cases(&self.cases, &mut self.results);
        for (case, want) in self.cases.iter().zip(&self.results) {
            let got = rust_case(case);
            // the C++ buffer holds eight bytes: the Rust one ends at the limit, the rest is the previous byte's
            assert_eq!(
                got, *want,
                "a decision differs: a={:04x} state={} bit={} ct={} c={:08x} prev={:02x} limit offset {}",
                case.a, case.state, case.bit, case.ct, case.c, case.prev, case.limit_off
            );
        }
        self.decisions += self.cases.len() as u64;
        self.cases.clear();
    }

    fn push(&mut self, a: u32, state: u32, bit: u32, ct: u32, which: u32) {
        let (c, prev, limit_off) = combo(which);
        self.cases.push(MqEncodeCase {
            a,
            c,
            ct,
            state,
            bit,
            prev,
            limit_off,
        });
        if self.cases.len() >= 1 << 16 {
            self.flush();
        }
    }
}

/// One interval's cases: all 60 combinations of the code register's low word, the previous byte and the limit for each
/// (state, bit, counter) when `full`, else the C++ test's one pseudo-random combination.
fn interval(a: u32, full: bool, sweep: &mut Sweep) {
    for state in 0..94u32 {
        for bit in 0..2u32 {
            let pick = a
                .wrapping_mul(2_654_435_761)
                .wrapping_add(state.wrapping_mul(40_503))
                .wrapping_add(bit.wrapping_mul(977))
                % 60;
            for ct in 1..=12u32 {
                if full {
                    for which in 0..60 {
                        sweep.push(a, state, bit, ct, which);
                    }
                } else {
                    sweep.push(a, state, bit, ct, (pick + ct * 5) % 60);
                }
            }
        }
    }
}

/// The intervals `first`, `first + step`, ... up to 0xFFFF.
fn sweep(first: u32, step: u32, full: bool, sweep: &mut Sweep) {
    let mut a = 0x8000 + first;
    while a <= 0xffff {
        interval(a, full, sweep);
        a += step;
    }
}

#[test]
fn single_decisions_encode_as_the_cpp_does() {
    let mut s = Sweep::new();
    // every fourth interval with the C++ test's sampled combination ...
    sweep(0, 4, false, &mut s);
    s.flush();
    assert_eq!(s.decisions, 8_192 * 94 * 2 * 12);
    // ... and, for the intervals where the choices change (the conditional exchange at a = 2 qe, the renormalisation
    // at a = 0x8000 + qe, each with its neighbours), every combination: 94 states x 10 intervals x 2 x 12 x 60
    let before = s.decisions;
    let mut edges: Vec<u32> = (0..94)
        .flat_map(|state| {
            let qe = crate::t1::tables::MQ_STATES[state].0;
            [2 * qe, 0x8000 + qe]
                .into_iter()
                .flat_map(|e| (0..5).map(move |d| (e + d).wrapping_sub(2)))
        })
        .filter(|a| (0x8000..=0xffff).contains(a))
        .collect();
    edges.sort_unstable();
    edges.dedup();
    for a in edges {
        interval(a, true, &mut s);
    }
    s.flush();
    assert!(
        s.decisions - before > 1_000_000,
        "{} edge decisions",
        s.decisions - before
    );
}

/// The nightly sweep: every combination, 4.4 billion decisions, split over every thread there is.
#[test]
#[ignore = "nightly: 4.4 billion single decisions of the MQ encoder against the C++"]
fn nightly_mq_encoder_every_decision() {
    let threads = std::thread::available_parallelism().map_or(4, |n| u32::try_from(n.get()).unwrap_or(1));
    let total: u64 = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                scope.spawn(move || {
                    let mut s = Sweep::new();
                    // thread t takes the intervals 0x8000 + t, + threads, ...
                    sweep(t, threads, true, &mut s);
                    s.flush();
                    s.decisions
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().expect("a sweep thread")).sum()
    });
    assert_eq!(total, 32_768 * 94 * 2 * 12 * 60);
    eprintln!("{total} single decisions identical ({threads} threads)");
}
