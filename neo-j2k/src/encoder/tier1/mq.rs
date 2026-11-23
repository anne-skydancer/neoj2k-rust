// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// OpenJPEG's MQ encoder, ported through src/enc_t1.cpp. See LICENSE.OpenJPEG.

use core::hint::select_unpredictable;

use crate::t1::tables::MQ_STATES;

#[cfg(all(test, not(miri)))]
mod differential;

#[derive(Clone, Copy)]
struct State {
    qeval: u16,
    next_mps: u8,
    next_lps: u8,
}

/// `MQ_STATES` without the most probable symbol, which is the low bit of the state index. It has an entry for every
/// byte, so that indexing it by a context's state byte needs no bounds check; the states are the first 94.
#[allow(clippy::cast_possible_truncation, reason = "Qe is asserted to fit sixteen bits")]
static STATES: [State; 256] = {
    let mut states = [State {
        qeval: 0,
        next_mps: 0,
        next_lps: 0,
    }; 256];
    let mut index = 0;
    while index < 94 {
        let (qeval, mps, next_mps, next_lps) = MQ_STATES[index];
        assert!(mps as usize == index & 1 && qeval <= 0x5601);
        states[index] = State {
            qeval: qeval as u16,
            next_mps,
            next_lps,
        };
        index += 1;
    }
    states
};

pub(super) const CONTEXT_AGGREGATE: usize = 17;
pub(super) const CONTEXT_UNIFORM: usize = 18;

/// What a pass hands back of the coder's registers (the contexts it changed in place).
#[derive(Clone, Copy, Debug)]
pub(super) struct Kept {
    position: usize,
    a: u32,
    c: u32,
    bits: u32,
    overflow: bool,
}

/// The code register's part of the coder: what the byte-out path reads and writes.
#[derive(Clone, Copy)]
struct Tail {
    position: usize,
    c: u32,
    bits: u32,
    overflow: bool,
}

#[inline(always)]
fn byte_out(bytes: &mut [u8], t: &mut Tail) {
    if t.position >= bytes.len() - 1 {
        t.overflow = true;
        t.position = bytes.len() - 2;
    }
    let last = &mut bytes[t.position];
    let last_is_ff = if *last == 0xff {
        true
    } else if t.c & 0x800_0000 != 0 {
        *last += 1;
        let became_ff = *last == 0xff;
        if became_ff {
            t.c &= 0x7ff_ffff;
        }
        became_ff
    } else {
        false
    };
    t.position += 1;
    if last_is_ff {
        bytes[t.position] = (t.c >> 20).to_le_bytes()[0];
        t.c &= 0xf_ffff;
        t.bits = 7;
    } else {
        bytes[t.position] = (t.c >> 19).to_le_bytes()[0];
        t.c &= 0x7_ffff;
        t.bits = 8;
    }
}

/// Shift `c` left by `shift` bits, emitting a byte at each byte boundary crossed. Chunked shifts hand `byte_out` the
/// same `c` as shifting one bit at a time. A function of the tail alone, so that the coder itself is never borrowed
/// by it (in the passes it stays in machine registers).
#[cold]
#[inline(never)]
fn shift_with_bytes(bytes: &mut [u8], mut t: Tail, mut shift: u32) -> Tail {
    while shift >= t.bits {
        shift -= t.bits;
        t.c <<= t.bits;
        byte_out(bytes, &mut t);
    }
    t.c <<= shift;
    t.bits -= shift;
    t
}

#[derive(Debug)]
pub(super) struct Encoder<'a> {
    bytes: &'a mut [u8],
    position: usize,
    a: u32,
    c: u32,
    bits: u32,
    contexts: [u8; 19],
    overflow: bool,
}

impl<'a> Encoder<'a> {
    pub(super) fn new(bytes: &'a mut [u8]) -> Self {
        bytes[0] = 0; // The fake byte permits carry propagation before the first actual byte.
        let mut contexts = [0; 19];
        contexts[0] = 8;
        contexts[CONTEXT_AGGREGATE] = 6;
        contexts[CONTEXT_UNIFORM] = 92;
        Self {
            bytes,
            position: 0,
            a: 0x8000,
            c: 0,
            bits: 12,
            contexts,
            overflow: false,
        }
    }

    /// The coder's registers as locals for the length of a pass: nothing takes the run's address, so they live in
    /// machine registers (the contexts stay in the encoder's memory, which is why they are a reference: a variable
    /// index into an array in the same local would keep the whole struct in memory). Hand them back with
    /// `Run::finish` and `restore`.
    #[inline(always)]
    pub(super) fn lend(&mut self) -> Run<'_> {
        Run {
            bytes: &mut *self.bytes,
            contexts: &mut self.contexts,
            position: self.position,
            a: self.a,
            c: self.c,
            bits: self.bits,
            overflow: self.overflow,
        }
    }

    #[inline(always)]
    pub(super) fn restore(&mut self, kept: Kept) {
        self.position = kept.position;
        self.a = kept.a;
        self.c = kept.c;
        self.bits = kept.bits;
        self.overflow = kept.overflow;
    }

    pub(super) fn rate(&self) -> u32 {
        u32::try_from(self.position)
            .expect("MQ buffer length fits u32")
            .wrapping_sub(1)
    }

    pub(super) fn overflowed(&self) -> bool {
        self.overflow
    }

    #[cfg(all(test, not(miri)))]
    pub(super) fn symbol(&mut self, context: usize, bit: u32) {
        let mut run = self.lend();
        run.symbol(context, bit);
        let kept = run.finish();
        self.restore(kept);
    }

    pub(super) fn flush(&mut self) {
        let mut run = self.lend();
        run.flush();
        let kept = run.finish();
        self.restore(kept);
    }
}

/// The coder of a pass: see `Encoder::lend`.
#[derive(Debug)]
pub(super) struct Run<'b> {
    bytes: &'b mut [u8],
    contexts: &'b mut [u8; 19],
    position: usize,
    a: u32,
    c: u32,
    bits: u32,
    overflow: bool,
}

impl Run<'_> {
    #[inline(always)]
    pub(super) fn finish(self) -> Kept {
        Kept {
            position: self.position,
            a: self.a,
            c: self.c,
            bits: self.bits,
            overflow: self.overflow,
        }
    }

    /// OpenJPEG's `opj_mqc_encode`. The outcome of a symbol is data dependent and mispredicts badly as a branch,
    /// so the conditional exchange, the state transition and the renormalisation shift are all computed without
    /// one; the selects are marked unpredictable so that the code generator keeps them selects.
    #[inline(always)]
    pub(super) fn symbol(&mut self, context: usize, bit: u32) {
        let state = self.contexts[context];
        let entry = STATES[usize::from(state)];
        let qeval = u32::from(entry.qeval);
        let lps = u32::from(state & 1) ^ bit != 0;
        let a = self.a - qeval;
        // MPS: a < qe takes the exchange (a = qe), else c += qe. LPS: the opposite test.
        let exchange = (a < qeval) != lps;
        self.c = self.c.wrapping_add(select_unpredictable(exchange, 0, qeval));
        let renormalize = lps | (a < 0x8000);
        let a = select_unpredictable(exchange, qeval, a);
        let next = select_unpredictable(lps, entry.next_lps, entry.next_mps);
        self.contexts[context] = select_unpredictable(renormalize, next, state);
        let shift = a.leading_zeros() - 16;
        self.a = a << shift;
        if shift >= self.bits {
            let tail = shift_with_bytes(
                &mut *self.bytes,
                Tail {
                    position: self.position,
                    c: self.c,
                    bits: self.bits,
                    overflow: self.overflow,
                },
                shift,
            );
            self.position = tail.position;
            self.c = tail.c;
            self.bits = tail.bits;
            self.overflow = tail.overflow;
        } else {
            self.c <<= shift;
            self.bits -= shift;
        }
    }

    /// Codes a list of pairs, each a context in its high seven bits and the bit in its low one.
    #[inline(always)]
    pub(super) fn pairs(&mut self, list: &[u8]) {
        for &pair in list {
            self.symbol(usize::from(pair >> 1), u32::from(pair & 1));
        }
    }

    pub(super) fn flush(&mut self) {
        let top = self.c.wrapping_add(self.a);
        self.c |= 0xffff;
        if self.c >= top {
            self.c = self.c.wrapping_sub(0x8000);
        }
        let mut tail = Tail {
            position: self.position,
            c: self.c,
            bits: self.bits,
            overflow: self.overflow,
        };
        tail.c <<= tail.bits;
        byte_out(self.bytes, &mut tail);
        tail.c <<= tail.bits;
        byte_out(self.bytes, &mut tail);
        if self.bytes[tail.position] != 0xff {
            tail.position += 1;
        }
        self.position = tail.position;
        self.c = tail.c;
        self.bits = tail.bits;
        self.overflow = tail.overflow;
    }
}
