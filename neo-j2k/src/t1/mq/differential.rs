// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The MQ decoder decision by decision against the C++'s (`opj_mqc_decode_macro` of `t1_mq.h`, driven through
//! the oracle's shim):
//!
//! - streams: random byte strings decoded from INITDEC on random context sequences, the bits and the registers
//!   at the end equal;
//! - single decisions from registers the test sets, over the C++ unit test's enumeration (`test_single` of
//!   `tests/unit/t1_unit.cpp`: every interval and state, the code register around `qe`, every bit counter, the next
//!   bytes from 00, 7F, 8F, 90, FE, FF). The normal suite takes the trimmed enumeration (every 17th interval and the
//!   intervals at each state's exchange and renormalisation edges, 6.5 million decisions);
//!   `nightly_mq_decoder_every_decision` takes every interval (443 million decisions: the C++'s count of 887 million
//!   takes each interval twice with identical inputs) on every thread.

use neo_j2k_oracle::tier1::{MqDecodeCase, MqDecodeResult, MqDecoderRegs, mq_decode, mq_decode_cases};
use neo_j2k_testkit::generate::SplitMix64 as Rng;

use super::{Decoder, PACKED_STATES};
use crate::t1::tables::MQ_STATES;

/// A context's state index (`state * 2 + MPS`) from the decoder's packed word.
fn index_of(word: u32) -> u8 {
    let index = PACKED_STATES[..MQ_STATES.len()]
        .iter()
        .position(|&w| w == word)
        .expect("a context holds one of the 94 states");
    u8::try_from(index).unwrap()
}

fn registers(decoder: &Decoder<'_>, bp: u32) -> MqDecoderRegs {
    let mut state = [0u8; 19];
    for (s, &word) in state.iter_mut().zip(&decoder.contexts) {
        *s = index_of(word);
    }
    MqDecoderRegs {
        a: decoder.registers.a,
        c: decoder.registers.c,
        ct: decoder.registers.bits,
        bp,
        state,
    }
}

/// One decision of Rust from the case's registers: the four bytes are followed by an explicit 0xFF 0xFF, so
/// the read position is exact.
fn rust_case(case: &MqDecodeCase) -> MqDecodeResult {
    let tail = [case.bytes[1], case.bytes[2], case.bytes[3], 0xff, 0xff];
    let mut decoder = Decoder::new();
    decoder.source.data = &tail;
    decoder.source.current = u32::from(case.bytes[0]);
    decoder.registers.a = case.a;
    decoder.registers.c = case.c;
    decoder.registers.bits = case.ct;
    decoder.contexts[0] = PACKED_STATES[case.state as usize];
    let bit = decoder.symbol(0);
    MqDecodeResult {
        bit,
        a: decoder.registers.a,
        c: decoder.registers.c,
        ct: decoder.registers.bits,
        bp: u32::try_from(tail.len() - decoder.source.data.len()).unwrap(),
        state: u32::from(index_of(decoder.contexts[0])),
    }
}

fn fill_stream(rng: &mut Rng, len: usize, kind: u32) -> Vec<u8> {
    (0..len)
        .map(|i| {
            let r = u8::try_from(rng.below(256)).unwrap();
            match kind {
                0 => r,
                1 => 0xff,
                2 => 0,
                3 => {
                    if i % 2 == 1 {
                        0x90 + u8::try_from(rng.below(0x70)).unwrap()
                    } else {
                        0xff
                    }
                }
                4 => {
                    if rng.below(8) == 0 {
                        0xff
                    } else {
                        u8::try_from(rng.below(0x90)).unwrap()
                    }
                }
                _ => {
                    if rng.below(2) == 1 {
                        0xff
                    } else {
                        0x8f - u8::try_from(rng.below(3)).unwrap()
                    }
                }
            }
        })
        .collect()
}

/// `count` random streams: the decisions of Rust and of the C++ on the same bytes and contexts.
fn run_streams(seed: u64, count: u32) -> u64 {
    let mut rng = Rng(seed);
    let mut decisions = 0;
    for stream in 0..count {
        let len = if stream % 7 == 0 {
            rng.below(4) as usize
        } else {
            rng.below(400) as usize
        };
        let kind = rng.below(6);
        let data = fill_stream(&mut rng, len, kind);
        // more decisions than the bytes hold: the decoder runs into the end and its marker
        let n = rng.below(2_000) as usize + 1;
        let mut contexts = Vec::with_capacity(n);
        let mut previous = 0u8;
        for _ in 0..n {
            // in runs, as the passes use them
            if rng.below(4) == 0 {
                previous = u8::try_from(rng.below(19)).unwrap();
            }
            contexts.push(previous);
        }
        let (want_bits, want) = mq_decode(&data, &contexts);

        // the decoder as the library runs it: the segment's bytes alone, the end implied
        let mut decoder = Decoder::new();
        decoder.start_segment(&data, false);
        let bits: Vec<u8> = contexts
            .iter()
            .map(|&c| u8::try_from(decoder.symbol(usize::from(c))).unwrap())
            .collect();
        assert_eq!(
            bits, want_bits,
            "stream {stream} (seed {seed:#x}): decoded bits, {len} bytes, kind {kind}"
        );
        // the read position past the data is not kept without the sentinel: compare it where it is real
        let got = registers(&decoder, want.bp);
        assert_eq!(
            got, want,
            "stream {stream} (seed {seed:#x}): registers at the end, {len} bytes, kind {kind}"
        );

        // and with the sentinel as bytes, where the position is exact too
        let mut with_marker = data.clone();
        with_marker.extend([0xff, 0xff]);
        let mut decoder = Decoder::new();
        decoder.start_segment(&with_marker, false);
        for &c in &contexts {
            decoder.symbol(usize::from(c));
        }
        let consumed = u32::try_from(with_marker.len() - 1 - decoder.source.data.len()).unwrap();
        assert_eq!(consumed, want.bp, "stream {stream} (seed {seed:#x}): read position");
        decisions += n as u64;
    }
    decisions
}

#[test]
fn streams_decode_as_the_cpp_does() {
    let decisions = run_streams(0x4d51_5354, 600);
    assert!(decisions > 500_000);
}

const BYTES: [u8; 6] = [0x00, 0x7f, 0x8f, 0x90, 0xfe, 0xff];

/// The cases of one interval and state (`at` of the C++ test): the code register's high word around `qe`, every bit
/// counter, a few low words and byte quadruples.
fn cases_at(a: u32, state: u32, full: bool, out: &mut Vec<MqDecodeCase>) {
    let qe = MQ_STATES[state as usize].0;
    let chis = [
        qe.saturating_sub(1),
        qe,
        qe + 1,
        a.wrapping_mul(2_654_435_761).wrapping_add(state.wrapping_mul(40_503)) & 0xffff,
    ];
    for ct in 0..=8u32 {
        for (k, &chi) in (0u32..).zip(chis.iter()) {
            let pick = a
                .wrapping_mul(31)
                .wrapping_add(state * 7)
                .wrapping_add(ct * 13)
                .wrapping_add(k * 5);
            let clo = if k & 1 == 1 {
                0xffff
            } else {
                pick.wrapping_mul(2_246_822_519) & 0xffff
            };
            let pairs = if full { 4 } else { 1 };
            for p in 0..pairs {
                let mut q = pick.wrapping_mul(2_654_435_761).wrapping_add(p * 40_503);
                let mut bytes = [0u8; 4];
                for b in &mut bytes {
                    *b = BYTES[((q >> 8) % 6) as usize];
                    q = q.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                }
                out.push(MqDecodeCase {
                    a,
                    c: (chi << 16) | clo,
                    ct,
                    state,
                    bytes,
                });
            }
        }
    }
}

struct Sweep {
    cases: Vec<MqDecodeCase>,
    results: Vec<MqDecodeResult>,
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

    /// Runs and compares the cases collected so far.
    fn flush(&mut self) {
        mq_decode_cases(&self.cases, &mut self.results);
        for (case, want) in self.cases.iter().zip(&self.results) {
            let got = rust_case(case);
            assert_eq!(
                got, *want,
                "a decision differs: a={:04x} state={} c={:08x} ct={} bytes {:02x?}",
                case.a, case.state, case.c, case.ct, case.bytes
            );
        }
        self.decisions += self.cases.len() as u64;
        self.cases.clear();
    }

    fn at(&mut self, a: u32, state: u32, full: bool) {
        cases_at(a, state, full, &mut self.cases);
        if self.cases.len() >= 1 << 16 {
            self.flush();
        }
    }
}

/// The trimmed enumeration of the C++ test (`full` false) over the intervals a0 in `first..=last` (offsets from 0x8000),
/// stepping `step`; or every interval (`full`, step 1).
fn sweep_intervals(first: u32, last: u32, step: u32, full: bool, sweep: &mut Sweep) {
    let mut a0 = first;
    while a0 <= last {
        let a = 0x8000 + a0;
        for state in 0..94 {
            sweep.at(a, state, full);
        }
        a0 += step;
    }
}

/// The intervals where the choices change, state by state: `a - qe < qe` (the conditional exchange) and
/// `a - qe < 0x8000` (the renormalisation), each with its neighbours.
fn sweep_edges(sweep: &mut Sweep) {
    for state in 0..94u32 {
        let qe = MQ_STATES[state as usize].0;
        for edge in [2 * qe, 0x8000 + qe] {
            for delta in 0..5u32 {
                let a = (edge + delta).wrapping_sub(2);
                if (0x8000..=0xffff).contains(&a) {
                    sweep.at(a, state, false);
                }
            }
        }
    }
}

#[test]
fn single_decisions_decode_as_the_cpp_does() {
    let mut sweep = Sweep::new();
    // every 17th interval, the ends of the range
    sweep_intervals(0, 0x7fff, 17, false, &mut sweep);
    for state in 0..94 {
        sweep.at(0xffff, state, false);
    }
    sweep_edges(&mut sweep);
    sweep.flush();
    // 1,928 intervals and the top one at 94 states of 36 cases, and the edges of the 94 states
    assert_eq!(sweep.decisions, 6_547_824);
}

/// The nightly sweep: every interval and state, c_hi around qe, every bit counter, four low words and byte
/// quadruples a case, split over every thread there is.
#[test]
#[ignore = "nightly: 443 million single decisions of the MQ decoder against the C++"]
fn nightly_mq_decoder_every_decision() {
    let threads = std::thread::available_parallelism().map_or(4, |n| u32::try_from(n.get()).unwrap_or(1));
    let total: u64 = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                scope.spawn(move || {
                    let mut sweep = Sweep::new();
                    // thread t takes the intervals a0 = t, t + threads, ...
                    sweep_intervals(t, 0x7fff, threads, true, &mut sweep);
                    sweep.flush();
                    sweep.decisions
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().expect("a sweep thread")).sum()
    });
    assert_eq!(total, 32_768 * 94 * 9 * 4 * 4);
    eprintln!("{total} single decisions identical ({threads} threads)");
}
