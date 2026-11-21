// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use super::CType;
use crate::{Error, ErrorKind};

const FIRST_BLOCK: u64 = 2048;
const LARGEST_BLOCK: u64 = 64 * 1024;
const BLOCK_HEADER: u64 = 16;

/// Tracks block reservations as the C++ header arena would.
#[derive(Debug)]
pub(crate) struct HeaderLedger {
    budget: u64,
    reserved: u64,
    current: u64,
    end: u64,
    next_block: u64,
}

impl Default for HeaderLedger {
    fn default() -> Self {
        Self {
            budget: 0,
            reserved: 0,
            current: 0,
            end: 0,
            next_block: FIRST_BLOCK,
        }
    }
}

impl HeaderLedger {
    pub(crate) const fn reserved(&self) -> u64 {
        self.reserved
    }

    pub(crate) fn reset(&mut self, retain: u64, budget: u64) {
        self.budget = budget;
        if self.reserved == 0 {
            return;
        }
        if self.reserved > retain {
            *self = Self {
                budget,
                ..Self::default()
            };
        } else {
            self.current = BLOCK_HEADER;
            self.end = self.reserved;
            self.next_block = FIRST_BLOCK;
        }
    }

    pub(crate) fn array(&mut self, ty: CType, count: usize) -> Result<(), Error> {
        let size = ty.csize().checked_mul(count).ok_or_else(limit)?;
        self.charge(size, ty.calign().max(16))
    }

    pub(crate) fn charge(&mut self, size: usize, alignment: usize) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        debug_assert!(alignment.is_power_of_two());
        let size = size as u64;
        let alignment = alignment as u64;
        let align_up = |at: u64| at.checked_add(alignment - 1).map(|n| n & !(alignment - 1));
        if let Some(at) = align_up(self.current)
            && let Some(next) = at.checked_add(size)
            && next <= self.end
        {
            self.current = next;
            return Ok(());
        }

        let header = align_up(BLOCK_HEADER).ok_or_else(limit)?;
        let needed = header.checked_add(size).ok_or_else(limit)?;
        let dedicated = needed > self.next_block / 2;
        let block = if dedicated {
            needed
        } else {
            let block = self.next_block;
            self.next_block = (block * 2).min(LARGEST_BLOCK);
            block
        };
        let reserved = self.reserved.checked_add(block).ok_or_else(limit)?;
        if self.budget != 0 && reserved > self.budget {
            return Err(limit());
        }
        self.reserved = reserved;
        if !dedicated {
            self.current = needed;
            self.end = block;
        }
        Ok(())
    }
}

fn limit() -> Error {
    Error::new(
        ErrorKind::Limit,
        "The headers need more memory than the decoder's limit\n",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedicated_blocks_leave_the_current_block_available() {
        let mut ledger = HeaderLedger::default();
        ledger.charge(256, 16).unwrap();
        ledger.charge(4096, 16).unwrap();
        ledger.charge(1024, 16).unwrap();
        assert_eq!(ledger.reserved(), 2048 + 4096 + 16);
    }

    #[test]
    fn reset_combines_retained_blocks_and_preserves_the_budget() {
        let mut ledger = HeaderLedger::default();
        ledger.charge(256, 16).unwrap();
        ledger.charge(4096, 16).unwrap();
        ledger.reset(8192, 6200);
        ledger.charge(6000, 16).unwrap();
        assert_eq!(ledger.charge(300, 4).unwrap_err().kind(), ErrorKind::Limit);
        assert_eq!(ledger.reserved(), 6160);
    }
}
