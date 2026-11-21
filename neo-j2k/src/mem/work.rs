// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The C++ working arena's block accounting, independent of Rust's storage layout.

use super::CType;
use crate::{Error, ErrorKind};

const HEADER: usize = 64;
const FIRST: usize = 16 * 1024;
const LARGEST: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Mark {
    block: Option<usize>,
    offset: usize,
    preceding: u64,
}

#[derive(Debug)]
pub(crate) struct WorkBudget {
    blocks: Vec<usize>,
    cursor: Mark,
    next_block: usize,
    reserved: u64,
    high_water: u64,
    maximum: u64,
}

impl Default for WorkBudget {
    fn default() -> Self {
        Self {
            blocks: Vec::new(),
            cursor: Mark::default(),
            next_block: FIRST,
            reserved: 0,
            high_water: 0,
            maximum: 0,
        }
    }
}

impl WorkBudget {
    pub(crate) fn retained_bytes(&self) -> u64 {
        super::capacity_bytes(&self.blocks)
    }

    pub(crate) fn reset(&mut self, retain: u64, maximum: u64) {
        self.maximum = maximum;
        self.cursor = Mark::default();
        self.high_water = 0;
        if self.reserved > retain {
            self.blocks.clear();
            self.next_block = FIRST;
            self.reserved = 0;
        } else if self.blocks.len() > 1 {
            if let Ok(total) = usize::try_from(self.reserved) {
                self.blocks.truncate(1);
                self.blocks[0] = total;
            } else {
                self.blocks.clear();
                self.reserved = 0;
            }
            self.next_block = FIRST;
        }
    }

    pub(crate) const fn mark(&self) -> Mark {
        self.cursor
    }

    pub(crate) fn rewind(&mut self, mark: Mark) {
        self.cursor = mark;
    }

    pub(crate) const fn reserved(&self) -> u64 {
        self.reserved
    }

    pub(crate) const fn high_water(&self) -> u64 {
        self.high_water
    }

    pub(crate) fn array(&mut self, kind: CType, count: usize) -> Result<(), Error> {
        let size = kind.csize().checked_mul(count).ok_or_else(limit)?;
        self.charge(size, kind.calign().max(16))
    }

    pub(crate) fn charge(&mut self, size: usize, alignment: usize) -> Result<(), Error> {
        if size == 0 {
            return Ok(());
        }
        if !alignment.is_power_of_two() || alignment > 64 || size > usize::MAX / 2 {
            return Err(limit());
        }
        loop {
            if let Some(index) = self.cursor.block {
                let start = self.cursor.offset.checked_add(alignment - 1).ok_or_else(limit)? & !(alignment - 1);
                let end = start.checked_add(size).ok_or_else(limit)?;
                if end <= self.blocks[index] {
                    self.cursor.offset = end;
                    self.high_water = self.high_water.max(self.cursor.preceding + end as u64);
                    return Ok(());
                }
                if index + 1 < self.blocks.len() {
                    self.enter(index + 1, self.cursor.preceding + self.blocks[index] as u64);
                    continue;
                }
            } else if !self.blocks.is_empty() {
                self.enter(0, 0);
                continue;
            }

            let minimum = size.checked_add(HEADER + alignment).ok_or_else(limit)?;
            let block_size = minimum.max(self.next_block);
            if minimum <= self.next_block {
                self.next_block = (self.next_block * 2).min(LARGEST);
            }
            let reserved = self.reserved.checked_add(block_size as u64).ok_or_else(limit)?;
            if self.maximum != 0 && reserved > self.maximum {
                return Err(limit());
            }
            self.blocks
                .try_reserve(1)
                .map_err(|_| Error::new(ErrorKind::NoMemory, "Not enough memory to decode\n"))?;
            self.blocks.push(block_size);
            self.enter(self.blocks.len() - 1, self.reserved);
            self.reserved = reserved;
        }
    }

    fn enter(&mut self, index: usize, preceding: u64) {
        self.cursor = Mark {
            block: Some(index),
            offset: HEADER,
            preceding,
        };
    }
}

fn limit() -> Error {
    Error::new(ErrorKind::Limit, "Not enough memory to decode\n")
}
