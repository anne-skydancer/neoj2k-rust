// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The MQ decoder's select decision and the cleanup pass's run of AGG decisions against plain models, with no oracle
//! (so that Miri runs them): the decision against ISO 15444-1 C.3.2 as the standard writes it (branches, a
//! renormalisation bit by bit), the run against the decisions taken one at a time.

use super::{CONTEXT_AGGREGATE, Decoder, PACKED_STATES};
use crate::t1::tables::MQ_STATES;

/// The flag words of a row as the cells the passes read them through.
fn cells(words: &[u32]) -> Vec<core::cell::Cell<u32>> {
    words.iter().map(|&w| core::cell::Cell::new(w)).collect()
}

/// The registers and the bytes of one decision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Model {
    a: u32,
    c: u32,
    bits: u32,
    current: u32,
    consumed: usize,
}

/// C.3.2 DECODE, then RENORMD of C.3.3 one bit at a time with BYTEIN of C.3.4, for the state at `index`: the bit, the
/// next state's index and the registers.
fn model_decision(index: usize, mut r: Model, tail: &[u8]) -> (u32, usize, Model) {
    let (qe, mps, next_mps, next_lps) = MQ_STATES[index];
    let (bit, next);
    r.a -= qe;
    if r.c >> 16 < qe {
        // the LPS sub-interval, with the conditional exchange
        if r.a < qe {
            (bit, next) = (mps, usize::from(next_mps));
        } else {
            (bit, next) = (mps ^ 1, usize::from(next_lps));
        }
        r.a = qe;
    } else {
        r.c -= qe << 16;
        if r.a & 0x8000 != 0 {
            return (mps, index, r);
        }
        if r.a < qe {
            (bit, next) = (mps ^ 1, usize::from(next_lps));
        } else {
            (bit, next) = (mps, usize::from(next_mps));
        }
    }
    loop {
        if r.bits == 0 {
            // BYTEIN: the byte after the current one, the marker 0xff 0xff past the data
            let upcoming = tail.get(r.consumed).copied().map_or(0xff, u32::from);
            if r.current == 0xff {
                if upcoming > 0x8f {
                    r.c += 0xff00;
                    r.bits = 8;
                } else {
                    r.current = upcoming;
                    r.consumed += 1;
                    r.c += upcoming << 9;
                    r.bits = 7;
                }
            } else {
                r.current = upcoming;
                r.consumed += 1;
                r.c += upcoming << 8;
                r.bits = 8;
            }
        }
        r.a <<= 1;
        r.c <<= 1;
        r.bits -= 1;
        if r.a & 0x8000 != 0 {
            break;
        }
    }
    (bit, next, r)
}

const BYTES: [u8; 6] = [0x00, 0x7f, 0x8f, 0x90, 0xfe, 0xff];

/// A decoder from the registers, followed by `tail` and then the marker.
fn decoder_from<'a>(tail: &'a [u8], index: usize, r: Model) -> Decoder<'a> {
    let mut decoder = Decoder::new();
    decoder.source.data = &tail[r.consumed..];
    decoder.source.current = r.current;
    decoder.registers.a = r.a;
    decoder.registers.c = r.c;
    decoder.registers.bits = r.bits;
    decoder.contexts[0] = PACKED_STATES[index];
    decoder
}

fn check_decision(index: usize, start: Model, tail: &[u8]) {
    let (want_bit, want_next, want) = model_decision(index, start, tail);
    let mut decoder = decoder_from(tail, index, start);
    let bit = decoder.symbol(0);
    let got = Model {
        a: decoder.registers.a,
        c: decoder.registers.c,
        bits: decoder.registers.bits,
        current: decoder.source.current,
        consumed: tail.len() - decoder.source.data.len(),
    };
    let context = format!("state {index}, from {start:?}, bytes {tail:02x?}");
    assert_eq!(bit, want_bit, "bit: {context}");
    assert_eq!(decoder.contexts[0], PACKED_STATES[want_next], "next state: {context}");
    assert_eq!(got, want, "registers: {context}");
}

#[test]
fn the_select_decision_equals_the_standards_branches() {
    let step = if cfg!(miri) { 4099 } else { 211 };
    let mut checked = 0u32;
    for (index, &(qe, ..)) in MQ_STATES.iter().enumerate() {
        // the intervals: the ends, around the exchange (qe and 2 qe) and the renormalisation (0x8000 + qe) edges, and a spread
        let mut intervals = vec![0x8000, 0x8001, 0xffff, 0x8000 + qe - 1, 0x8000 + qe, 0x8000 + qe + 1];
        intervals.extend([qe + 1, 2 * qe - 1, 2 * qe, 2 * qe + 1, 0x7fff + qe].map(|a: u32| a.clamp(0x8000, 0xffff)));
        intervals.extend((0x8000..=0xffffu32).step_by(step));
        for a in intervals {
            for chi in [0, qe.saturating_sub(1), qe, qe + 1, a - qe, a, 0xffff] {
                for bits in 0..=8 {
                    let q = a.wrapping_mul(31) ^ (u32::try_from(index).unwrap() * 7) ^ (bits * 13);
                    let bytes = [0, 1, 2, 3].map(|k| BYTES[((q >> (3 * k)) % 6) as usize]);
                    let clo = if q & 1 == 0 {
                        q.wrapping_mul(2_246_822_519) & 0xffff
                    } else {
                        0xffff
                    };
                    for current in [0x12, 0xff] {
                        let start = Model {
                            a,
                            c: ((chi & 0xffff) << 16) | clo,
                            bits,
                            current,
                            consumed: 0,
                        };
                        check_decision(index, start, &bytes);
                        checked += 1;
                    }
                }
            }
        }
    }
    assert!(checked > if cfg!(miri) { 5_000 } else { 500_000 });
}

#[test]
fn a_run_of_aggregate_columns_equals_the_decisions_one_at_a_time() {
    let step = if cfg!(miri) { 4099 } else { 97 };
    let mut runs = 0u32;
    let mut longest = 0;
    for (index, &(qe, ..)) in MQ_STATES.iter().enumerate() {
        for a in (0x8000..=0xffffu32).step_by(step).chain([0x8000, 0x8000 + qe, 0xffff]) {
            for chi in
                [0, 1, qe - 1, qe, qe + 1, 2 * qe, 3 * qe + 1, a - 0x8000, 0x7fff, 0xffff].map(|v: u32| v & 0xffff)
            {
                for columns in [1usize, 2, 3, 7, 16, 63] {
                    // the columns after the first: empty (flag word 0) up to `columns`, then a non-empty one
                    let mut flags = vec![0u32; columns];
                    flags.push(1);
                    let make = || {
                        let mut decoder = Decoder::new();
                        decoder.registers.a = a;
                        decoder.registers.c = (chi << 16) | (a.wrapping_mul(2_654_435_761) & 0xffff);
                        decoder.registers.bits = 5;
                        decoder.source.current = 0x12;
                        decoder.source.data = &[0x34, 0x56, 0xff, 0xff];
                        decoder.contexts[CONTEXT_AGGREGATE] = PACKED_STATES[index];
                        decoder
                    };
                    let mut at_once = make();
                    let run = at_once.aggregate_run(&cells(&flags[..columns]));

                    // one decision at a time: the longest prefix of decisions that are an MPS of 0 with no renormalisation,
                    // no change of state and no byte
                    let mut one_by_one = make();
                    let mut done = 0;
                    while done < columns {
                        let before = one_by_one;
                        let bit = one_by_one.symbol(CONTEXT_AGGREGATE);
                        let plain = bit == 0
                            && one_by_one.registers.a == before.registers.a - qe
                            && one_by_one.registers.c == before.registers.c - (qe << 16)
                            && one_by_one.registers.bits == before.registers.bits
                            && one_by_one.source.data.len() == before.source.data.len()
                            && one_by_one.contexts == before.contexts;
                        if !plain {
                            one_by_one = before;
                            break;
                        }
                        done += 1;
                    }
                    assert_eq!(run, done, "state {index}, a {a:#x}, c_hi {chi:#x}, {columns} columns");
                    assert_eq!(
                        (
                            at_once.registers.a,
                            at_once.registers.c,
                            at_once.registers.bits,
                            at_once.contexts
                        ),
                        (
                            one_by_one.registers.a,
                            one_by_one.registers.c,
                            one_by_one.registers.bits,
                            one_by_one.contexts
                        ),
                        "registers: state {index}, a {a:#x}, c_hi {chi:#x}, {columns} columns"
                    );
                    longest = longest.max(run);
                    runs += 1;
                }
            }
        }
    }
    assert!(longest >= 16 || cfg!(miri), "a run of 16 columns was never taken");
    assert!(runs > if cfg!(miri) { 1_000 } else { 100_000 });
}

#[test]
fn a_run_stops_at_the_first_column_that_is_not_empty() {
    // qe 1 at state 90 (index of an MPS 0): room is large, so only the flag words end the run
    let index = 90;
    assert_eq!(MQ_STATES[index], (1, 0, 90, 86));
    let mut decoder = Decoder::new();
    decoder.registers.a = 0xffff;
    decoder.registers.c = 0xffff_0000;
    decoder.registers.bits = 8;
    decoder.contexts[CONTEXT_AGGREGATE] = PACKED_STATES[index];
    let flags = [0, 0, 0, 4, 0, 0];
    assert_eq!(decoder.aggregate_run(&cells(&flags)), 3);
    assert_eq!(
        (decoder.registers.a, decoder.registers.c),
        (0xffff - 3, 0xffff_0000 - (3 << 16))
    );
    // a run of the last column of a row
    let mut decoder = Decoder::new();
    decoder.registers.a = 0xffff;
    decoder.registers.c = 0xffff_0000;
    decoder.contexts[CONTEXT_AGGREGATE] = PACKED_STATES[index];
    assert_eq!(decoder.aggregate_run(&cells(&[0])), 1);
}

#[test]
fn no_run_when_the_mps_is_one_or_the_room_is_short() {
    // the AGG context's initial state (state 3, index 6: MPS 0): c_hi below qe, or a - 0x8000 below qe
    let qe = MQ_STATES[6].0;
    assert_eq!(MQ_STATES[6].1, 0);
    let mut decoder = Decoder::new();
    decoder.registers.a = 0xffff;
    decoder.registers.c = (qe - 1) << 16;
    assert_eq!(decoder.aggregate_run(&cells(&[0, 0])), 0);
    decoder.registers.c = qe << 16;
    decoder.registers.a = 0x8000 + qe - 1;
    assert_eq!(decoder.aggregate_run(&cells(&[0, 0])), 0);
    // the same registers with the room exactly qe: one column, two with room for two
    decoder.registers.a = 0x8000 + qe;
    assert_eq!(decoder.aggregate_run(&cells(&[0, 0])), 1);
    decoder.registers.a = 0x8000 + 2 * qe;
    decoder.registers.c = (2 * qe) << 16;
    assert_eq!(decoder.aggregate_run(&cells(&[0, 0])), 2);
    // an MPS of 1 (index 7)
    decoder.contexts[CONTEXT_AGGREGATE] = PACKED_STATES[7];
    decoder.registers.a = 0xffff;
    decoder.registers.c = 0xffff_0000;
    assert_eq!(decoder.aggregate_run(&cells(&[0, 0])), 0);
}
