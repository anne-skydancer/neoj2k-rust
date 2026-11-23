// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// OpenJPEG's MQ and raw readers, ported through src/t1.cpp. See the notice in t1.rs.

use super::tables::MQ_STATES;

#[cfg(all(test, not(miri)))]
mod differential;

pub(super) const CONTEXT_AGGREGATE: usize = 17;
pub(super) const CONTEXT_UNIFORM: usize = 18;
const MPS_BIT: u32 = 16;
const NEXT_MPS_SHIFT: u32 = 17;
const NEXT_LPS_SHIFT: u32 = 24;
const STATE_COUNT: usize = 128;

// Cache the active state in each context. A symbol that leaves its context unchanged
// can use Qe and MPS immediately, with no separate state-table lookup.
static PACKED_STATES: [u32; STATE_COUNT] = {
    let mut table = [0; STATE_COUNT];
    let mut i = 0;
    while i < MQ_STATES.len() {
        let (qeval, mps, next_mps, next_lps) = MQ_STATES[i];
        assert!(qeval <= 0xffff && mps <= 1);
        assert!((next_mps as usize) < MQ_STATES.len() && (next_lps as usize) < MQ_STATES.len());
        table[i] =
            qeval | (mps << MPS_BIT) | ((next_mps as u32) << NEXT_MPS_SHIFT) | ((next_lps as u32) << NEXT_LPS_SHIFT);
        i += 1;
    }
    table
};

const INITIAL_CONTEXTS: [u32; 19] = {
    let mut contexts = [PACKED_STATES[0]; 19];
    contexts[0] = PACKED_STATES[8];
    contexts[CONTEXT_AGGREGATE] = PACKED_STATES[6];
    contexts[CONTEXT_UNIFORM] = PACKED_STATES[92];
    contexts
};

/// One code-block's arithmetic registers and contexts, reused across its codeword segments.
#[derive(Debug)]
pub(super) struct Decoder<'a> {
    data: &'a [u8],
    current: u32,
    a: u32,
    c: u32,
    bits: u32,
    contexts: [u32; 19],
}

impl<'a> Decoder<'a> {
    pub(super) fn new() -> Self {
        Self {
            data: &[],
            current: 0,
            a: 0x8000,
            c: 0,
            bits: 0,
            contexts: INITIAL_CONTEXTS,
        }
    }

    pub(super) fn reset_contexts(&mut self) {
        self.contexts = INITIAL_CONTEXTS;
    }

    pub(super) fn start_segment(&mut self, data: &'a [u8], raw: bool) {
        self.data = data;
        if raw {
            self.c = 0;
            self.bits = 0;
        } else {
            (self.current, self.data) = Self::next_byte(data);
            self.c = self.current << 16;
            self.byte_in();
            self.c <<= 7;
            self.bits -= 7;
            self.a = 0x8000;
        }
    }

    fn next_byte(data: &[u8]) -> (u32, &[u8]) {
        // The two artificial 0xff bytes form a marker. The existing byte-in logic stops on it.
        data.split_first()
            .map_or((0xff, data), |(&byte, rest)| (u32::from(byte), rest))
    }

    fn byte_in(&mut self) {
        let (next, rest) = Self::next_byte(self.data);
        let stuffed = self.current == 0xff;
        let contribution = if stuffed && next > 0x8f {
            self.bits = 8;
            0xff00
        } else {
            self.data = rest;
            self.current = next;
            self.bits = 8 - u32::from(stuffed);
            next << (16 - self.bits)
        };
        self.c = self.c.wrapping_add(contribution);
    }

    #[inline]
    fn renormalize(&mut self) {
        let total = self.a.leading_zeros() - 16;
        let mut shifts = total;
        while shifts > self.bits {
            self.c <<= self.bits;
            shifts -= self.bits;
            self.byte_in();
        }
        self.a <<= total;
        self.c <<= shifts;
        self.bits -= shifts;
    }

    #[inline]
    pub(super) fn symbol(&mut self, context: usize) -> u32 {
        let current = self.contexts[context];
        let qeval = current & 0xffff;
        let mps = (current >> MPS_BIT) & 1;
        let next_mps = current >> NEXT_MPS_SHIFT;
        let next_lps = current >> NEXT_LPS_SHIFT;
        self.a -= qeval;
        let (bit, state) = if self.c >> 16 < qeval {
            let less = self.a < qeval;
            self.a = qeval;
            if less { (mps, next_mps) } else { (mps ^ 1, next_lps) }
        } else {
            self.c -= qeval << 16;
            if self.a & 0x8000 != 0 {
                return mps;
            }
            if self.a < qeval {
                (mps ^ 1, next_lps)
            } else {
                (mps, next_mps)
            }
        };
        self.contexts[context] = PACKED_STATES[state as usize & (STATE_COUNT - 1)];
        self.renormalize();
        bit
    }

    #[inline]
    pub(super) fn raw(&mut self) -> u32 {
        if self.bits == 0 {
            let (next, rest) = Self::next_byte(self.data);
            if self.c == 0xff && next > 0x8f {
                self.bits = 8;
            } else {
                self.bits = if self.c == 0xff { 7 } else { 8 };
                self.c = next;
                self.data = rest;
            }
        }
        self.bits -= 1;
        (self.c >> self.bits) & 1
    }
}
