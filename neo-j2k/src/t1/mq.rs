// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// OpenJPEG's MQ and raw readers, ported through src/t1.cpp. See the notice in t1.rs.

use core::hint::select_unpredictable;

use super::tables::MQ_STATES;

#[cfg(all(test, not(miri)))]
mod differential;

pub(super) const CONTEXT_AGGREGATE: usize = 17;
pub(super) const CONTEXT_UNIFORM: usize = 18;
const MPS_BIT: u32 = 16;
const NEXT_MPS_SHIFT: u32 = 17;
const NEXT_LPS_SHIFT: u32 = 24;
const STATE_COUNT: usize = 128;
// A context index is masked to this many slots (the 19 contexts and padding), so that no access is out of bounds.
const CONTEXT_SLOTS: usize = 32;

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

const INITIAL_CONTEXTS: [u32; CONTEXT_SLOTS] = {
    let mut contexts = [PACKED_STATES[0]; CONTEXT_SLOTS];
    contexts[0] = PACKED_STATES[8];
    contexts[CONTEXT_AGGREGATE] = PACKED_STATES[6];
    contexts[CONTEXT_UNIFORM] = PACKED_STATES[92];
    contexts
};

/// The registers a renormalisation that needs a byte works on, passed by value so that the decoder's own fields stay in
/// registers (no address escapes) on the path that never needs one.
#[derive(Clone, Copy)]
struct Registers<'a> {
    data: &'a [u8],
    current: u32,
    a: u32,
    c: u32,
    bits: u32,
}

impl Registers<'_> {
    fn byte_in(&mut self) {
        let (next, rest) = Decoder::next_byte(self.data);
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
}

/// RENORMD when the code register has fewer bits than the interval needs shifts (a byte comes in: 0.07 to 0.11 of a
/// decision): the shifts taken as many at a time as the register has bits, a byte read whenever none is left.
#[cold]
#[inline(never)]
fn renormalize_with_bytes(mut r: Registers<'_>, total: u32) -> Registers<'_> {
    let mut shifts = total;
    while shifts > r.bits {
        r.c <<= r.bits;
        shifts -= r.bits;
        r.byte_in();
    }
    r.a <<= total;
    r.c <<= shifts;
    r.bits -= shifts;
    r
}

/// One code-block's arithmetic registers and contexts, reused across its codeword segments. A pass copies it into a
/// local for its length, so that the registers live in machine registers, and copies it back.
#[derive(Clone, Copy, Debug)]
pub(super) struct Decoder<'a> {
    data: &'a [u8],
    current: u32,
    a: u32,
    c: u32,
    bits: u32,
    contexts: [u32; CONTEXT_SLOTS],
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
        let mut r = Registers {
            data: self.data,
            current: self.current,
            a: self.a,
            c: self.c,
            bits: self.bits,
        };
        r.byte_in();
        (self.data, self.current, self.c, self.bits) = (r.data, r.current, r.c, r.bits);
    }

    /// The decision of ISO 15444-1 C.3.2 with its renormalisation, as the C++'s `opj_mqc_decode_macro`: the branches
    /// the decoded bit does not steer are selects, the renormalisation is always run (no shift when bit 15 of the
    /// interval is set) and only a byte coming in branches.
    ///
    /// `lps` says the code register is in the LPS sub-interval; `flip` that the symbol is the less probable one (an
    /// LPS-interval hit the conditional exchange leaves an MPS, or an MPS-interval hit it makes an LPS). The state
    /// changes, to the next-LPS state on a flip and else to the next-MPS state, when the interval is the LPS one or has
    /// lost bit 15.
    #[inline(always)]
    pub(super) fn symbol(&mut self, context: usize) -> u32 {
        let slot = context & (CONTEXT_SLOTS - 1);
        let word = self.contexts[slot];
        let qeval = word & 0xffff;
        let a = self.a - qeval;
        let lps = self.c >> 16 < qeval;
        let flip = lps ^ (a < qeval);
        let bit = ((word >> MPS_BIT) & 1) ^ u32::from(flip);
        // Selects the compiler is told are unpredictable (it otherwise turns them back into branches).
        let next_shift = select_unpredictable(flip, NEXT_LPS_SHIFT, NEXT_MPS_SHIFT);
        let next = PACKED_STATES[((word >> next_shift) as usize) & (STATE_COUNT - 1)];
        let changes = lps | (a < 0x8000);
        self.contexts[slot] = select_unpredictable(changes, next, word);
        self.c -= select_unpredictable(lps, 0, qeval << 16);
        let a = select_unpredictable(lps, qeval, a);
        // the renormalisation: the shifts at once when the code register has the bits
        let total = a.leading_zeros() - 16;
        if total <= self.bits {
            self.a = a << total;
            self.c <<= total;
            self.bits -= total;
        } else {
            let r = renormalize_with_bytes(
                Registers {
                    data: self.data,
                    current: self.current,
                    a,
                    c: self.c,
                    bits: self.bits,
                },
                total,
            );
            (self.data, self.current, self.a, self.c, self.bits) = (r.data, r.current, r.a, r.c, r.bits);
        }
        bit
    }

    #[inline(always)]
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
