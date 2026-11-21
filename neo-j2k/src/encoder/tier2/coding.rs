// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Packet bit and tag-tree writers, from OpenJPEG through src/enc_t2.cpp.

use crate::Error;

use super::super::{memory_error, resize_with};

pub(super) struct Bio<'a> {
    output: &'a mut [u8],
    written: usize,
    word: u16,
    left: u8,
}

impl<'a> Bio<'a> {
    pub(super) fn new(output: &'a mut [u8]) -> Self {
        Self {
            output,
            written: 0,
            word: 0,
            left: 8,
        }
    }

    fn byte_out(&mut self) -> bool {
        self.word <<= 8;
        self.left = if self.word == 0xff00 { 7 } else { 8 };
        let Some(out) = self.output.get_mut(self.written) else {
            return false;
        };
        *out = self.word.to_be_bytes()[0];
        self.written += 1;
        true
    }

    pub(super) fn put(&mut self, bit: bool) {
        if self.left == 0 {
            self.byte_out();
        }
        self.left -= 1;
        self.word |= u16::from(bit) << self.left;
    }

    pub(super) fn write(&mut self, value: u32, width: u32) {
        for bit in (0..width).rev() {
            self.put(value.checked_shr(bit).unwrap_or(0) & 1 != 0);
        }
    }

    pub(super) fn comma(&mut self, ones: u32) {
        for _ in 0..ones {
            self.put(true);
        }
        self.put(false);
    }

    pub(super) fn passes(&mut self, count: u32) {
        match count {
            1 => self.put(false),
            2 => self.write(2, 2),
            3..=5 => self.write(0xc | (count - 3), 4),
            6..=36 => self.write(0x1e0 | (count - 6), 9),
            37..=164 => self.write(0xff80 | (count - 37), 16),
            _ => {}
        }
    }

    pub(super) fn finish(mut self) -> Option<usize> {
        if !self.byte_out() || (self.left == 7 && !self.byte_out()) {
            return None;
        }
        Some(self.written)
    }
}

#[derive(Clone, Copy, Debug)]
struct TagNode {
    value: i32,
    low: i32,
    known: bool,
}

const UNKNOWN: TagNode = TagNode {
    value: 999,
    low: 0,
    known: false,
};
const MAX_PATH: usize = usize::BITS as usize + 1;

#[derive(Debug, Default)]
pub(super) struct TagTree {
    width: usize,
    height: usize,
    nodes: Vec<TagNode>,
}

impl TagTree {
    pub(super) fn configure(&mut self, width: usize, height: usize) -> Result<(), Error> {
        let (mut columns, mut rows) = (width, height);
        let mut size = columns.checked_mul(rows).ok_or_else(memory_error)?;
        let mut total = size;
        while size > 1 {
            columns = columns.div_ceil(2);
            rows = rows.div_ceil(2);
            size = columns * rows;
            total = total.checked_add(size).ok_or_else(memory_error)?;
        }
        resize_with(&mut self.nodes, total, || UNKNOWN)?;
        self.width = width;
        self.height = height;
        self.reset();
        Ok(())
    }

    pub(super) fn reset(&mut self) {
        self.nodes.fill(UNKNOWN);
    }

    fn path(&self, leaf: usize) -> ([usize; MAX_PATH], usize) {
        let mut path = [0; MAX_PATH];
        let mut depth = 0;
        let mut at = 0;
        let (mut width, mut height) = (self.width, self.height);
        let (mut x, mut y) = (leaf % width, leaf / width);
        loop {
            path[depth] = at + y * width + x;
            depth += 1;
            let size = width * height;
            if size == 1 {
                return (path, depth);
            }
            at += size;
            width = width.div_ceil(2);
            height = height.div_ceil(2);
            x /= 2;
            y /= 2;
        }
    }

    pub(super) fn set_value(&mut self, leaf: usize, value: i32) {
        let (path, depth) = self.path(leaf);
        for index in &path[..depth] {
            let node = &mut self.nodes[*index];
            if node.value <= value {
                break;
            }
            node.value = value;
        }
    }

    pub(super) fn write(&mut self, bio: &mut Bio<'_>, leaf: usize, threshold: i32) {
        let (path, depth) = self.path(leaf);
        let mut low = 0;
        for index in path[..depth].iter().rev() {
            let node = &mut self.nodes[*index];
            low = low.max(node.low);
            while low < threshold {
                if low >= node.value {
                    if !node.known {
                        bio.put(true);
                        node.known = true;
                    }
                    break;
                }
                bio.put(false);
                low += 1;
            }
            node.low = low;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writer_stuffs_after_ff_on_flush() {
        let mut out = [0xaa; 3];
        let mut bio = Bio::new(&mut out);
        bio.write(255, 8);
        assert_eq!(bio.finish(), Some(2));
        assert_eq!(out, [0xff, 0, 0xaa]);

        let mut out = [0; 1];
        let mut bio = Bio::new(&mut out);
        bio.write(255, 8);
        assert_eq!(bio.finish(), None);
    }
}
