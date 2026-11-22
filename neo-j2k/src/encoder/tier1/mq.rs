// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// OpenJPEG's MQ encoder, ported through src/enc_t1.cpp. See LICENSE.OpenJPEG.

use crate::t1::tables::MQ_STATES;

#[derive(Clone, Copy)]
struct State {
    qeval: u16,
    next_mps: u8,
    next_lps: u8,
}

/// `MQ_STATES` without the most probable symbol, which is the low bit of the state index.
#[allow(clippy::cast_possible_truncation, reason = "Qe is asserted to fit sixteen bits")]
static STATES: [State; 94] = {
    let mut states = [State {
        qeval: 0,
        next_mps: 0,
        next_lps: 0,
    }; 94];
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

    pub(super) fn rate(&self) -> u32 {
        u32::try_from(self.position)
            .expect("MQ buffer length fits u32")
            .wrapping_sub(1)
    }

    pub(super) fn overflowed(&self) -> bool {
        self.overflow
    }

    #[inline(always)]
    fn byte_out(&mut self) {
        if self.position >= self.bytes.len() - 1 {
            self.overflow = true;
            self.position = self.bytes.len() - 2;
        }
        let last = &mut self.bytes[self.position];
        let last_is_ff = if *last == 0xff {
            true
        } else if self.c & 0x800_0000 != 0 {
            *last += 1;
            let became_ff = *last == 0xff;
            if became_ff {
                self.c &= 0x7ff_ffff;
            }
            became_ff
        } else {
            false
        };
        self.position += 1;
        if last_is_ff {
            self.bytes[self.position] = (self.c >> 20).to_le_bytes()[0];
            self.c &= 0xf_ffff;
            self.bits = 7;
        } else {
            self.bytes[self.position] = (self.c >> 19).to_le_bytes()[0];
            self.c &= 0x7_ffff;
            self.bits = 8;
        }
    }

    /// Shift `c` left by `shift` bits, emitting a byte at each byte boundary crossed. Chunked
    /// shifts hand `byte_out` the same `c` as shifting one bit at a time.
    #[cold]
    #[inline(never)]
    fn shift_with_bytes(&mut self, mut shift: u32) {
        while shift >= self.bits {
            shift -= self.bits;
            self.c <<= self.bits;
            self.byte_out();
        }
        self.c <<= shift;
        self.bits -= shift;
    }

    /// Branch-free form of OpenJPEG's `opj_mqc_encode`: the outcome of a symbol is data
    /// dependent and mispredicts badly as a branch, so the conditional exchange, the state
    /// transition and the renormalisation shift are all computed without one.
    #[inline(always)]
    pub(super) fn symbol(&mut self, context: usize, bit: u32) {
        let state = self.contexts[context];
        let entry = STATES[usize::from(state)];
        let qeval = u32::from(entry.qeval);
        let lps = u32::from(state & 1) ^ bit;
        let a = self.a - qeval;
        // MPS: a < qe takes the exchange (a = qe), else c += qe. LPS: the opposite test.
        let exchange = u32::from(a < qeval) ^ lps != 0;
        self.c = self.c.wrapping_add(if exchange { 0 } else { qeval });
        let renormalize = lps != 0 || a < 0x8000;
        let a = if exchange { qeval } else { a };
        let next = if lps != 0 { entry.next_lps } else { entry.next_mps };
        self.contexts[context] = if renormalize { next } else { state };
        let shift = a.leading_zeros() - 16;
        self.a = a << shift;
        if shift >= self.bits {
            self.shift_with_bytes(shift);
        } else {
            self.c <<= shift;
            self.bits -= shift;
        }
    }

    pub(super) fn flush(&mut self) {
        let top = self.c.wrapping_add(self.a);
        self.c |= 0xffff;
        if self.c >= top {
            self.c = self.c.wrapping_sub(0x8000);
        }
        self.c <<= self.bits;
        self.byte_out();
        self.c <<= self.bits;
        self.byte_out();
        if self.bytes[self.position] != 0xff {
            self.position += 1;
        }
    }
}
