// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// OpenJPEG's MQ encoder, ported through src/enc_t1.cpp. See LICENSE.OpenJPEG.

use crate::t1::tables::MQ_STATES;

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

    fn renormalize(&mut self) {
        while self.a & 0x8000 == 0 {
            self.a <<= 1;
            self.c <<= 1;
            self.bits -= 1;
            if self.bits == 0 {
                self.byte_out();
            }
        }
    }

    #[inline]
    pub(super) fn symbol(&mut self, context: usize, bit: u32) {
        let (qeval, mps, next_mps, next_lps) = MQ_STATES[usize::from(self.contexts[context])];
        self.a -= qeval;
        if mps == bit {
            if self.a & 0x8000 != 0 {
                self.c = self.c.wrapping_add(qeval);
                return;
            }
            if self.a < qeval {
                self.a = qeval;
            } else {
                self.c = self.c.wrapping_add(qeval);
            }
            self.contexts[context] = next_mps;
        } else {
            if self.a < qeval {
                self.c = self.c.wrapping_add(qeval);
            } else {
                self.a = qeval;
            }
            self.contexts[context] = next_lps;
        }
        self.renormalize();
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
