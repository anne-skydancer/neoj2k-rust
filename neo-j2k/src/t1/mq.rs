// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// OpenJPEG's MQ and raw readers, ported through src/t1.cpp. See the notice in t1.rs.

use super::tables::MQ_STATES;

pub(super) const CONTEXT_AGGREGATE: usize = 17;
pub(super) const CONTEXT_UNIFORM: usize = 18;

/// One code-block's arithmetic registers and contexts, reused across its codeword segments.
#[derive(Debug)]
pub(super) struct Decoder<'a> {
    data: &'a [u8],
    current: u32,
    a: u32,
    c: u32,
    bits: u32,
    contexts: [u8; 19],
}

impl<'a> Decoder<'a> {
    pub(super) fn new() -> Self {
        let mut decoder = Self {
            data: &[],
            current: 0,
            a: 0x8000,
            c: 0,
            bits: 0,
            contexts: [0; 19],
        };
        decoder.reset_contexts();
        decoder
    }

    pub(super) fn reset_contexts(&mut self) {
        self.contexts.fill(0);
        self.contexts[0] = 8;
        self.contexts[CONTEXT_AGGREGATE] = 6;
        self.contexts[CONTEXT_UNIFORM] = 92;
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
        let (qeval, mps, next_mps, next_lps) = MQ_STATES[usize::from(self.contexts[context])];
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
        self.contexts[context] = state;
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
