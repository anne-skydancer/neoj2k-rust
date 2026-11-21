// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! FNV-1a, 64 bits, and the hex form of the reference tables (`tests/common/common.h`: `Fnv64`,
//! `hex64`, `parse_hex64`). Integers enter little-endian, as the C++ writes them byte by byte.
//! The offset basis is the C++'s, which is not FNV's published one (see the test below).

/// FNV-1a over 64 bits: identity checks and digests (not cryptographic).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fnv64(u64);

impl Default for Fnv64 {
    fn default() -> Self {
        Self::new()
    }
}

impl Fnv64 {
    const OFFSET: u64 = 1_469_598_103_934_665_603;
    const PRIME: u64 = 1_099_511_628_211;

    /// The empty hash.
    #[must_use]
    pub const fn new() -> Self {
        Self(Self::OFFSET)
    }

    /// Adds bytes.
    pub fn bytes(&mut self, data: &[u8]) {
        let mut h = self.0;
        for &b in data {
            h = (h ^ u64::from(b)).wrapping_mul(Self::PRIME);
        }
        self.0 = h;
    }

    /// Adds a `u32`, low byte first.
    pub fn u32(&mut self, v: u32) {
        self.bytes(&v.to_le_bytes());
    }

    /// Adds a `u64` as two `u32`s, the low one first (the same bytes as little-endian).
    pub fn u64(&mut self, v: u64) {
        self.bytes(&v.to_le_bytes());
    }

    /// The hash so far.
    #[must_use]
    pub const fn value(&self) -> u64 {
        self.0
    }
}

/// Sixteen lowercase hex digits (`%016llx`).
#[must_use]
pub fn hex64(v: u64) -> String {
    format!("{v:016x}")
}

/// One to sixteen hex digits of either case; `None` otherwise.
#[must_use]
pub fn parse_hex64(s: &str) -> Option<u64> {
    if s.is_empty() || s.len() > 16 {
        return None;
    }
    let mut v = 0u64;
    for c in s.bytes() {
        let d = match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            b'A'..=b'F' => c - b'A' + 10,
            _ => return None,
        };
        v = (v << 4) | u64::from(d);
    }
    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cpp_offset_basis() {
        // The C++ harness's basis is FNV-1a's published one (14695981039346656037) with its last
        // digit dropped. Every reference table and digest is built on it, so the twin keeps it;
        // the oracle crate's tests check whole-file and cut lines of the tables against it.
        assert_eq!(Fnv64::new().value(), 1_469_598_103_934_665_603);
        let mut h = Fnv64::new();
        h.bytes(b"a");
        assert_eq!(
            h.value(),
            (1_469_598_103_934_665_603u64 ^ 0x61).wrapping_mul(1_099_511_628_211)
        );
    }

    #[test]
    fn integers_are_little_endian() {
        let mut a = Fnv64::new();
        a.u64(0x0102_0304_0506_0708);
        let mut b = Fnv64::new();
        b.u32(0x0506_0708);
        b.u32(0x0102_0304);
        assert_eq!(a, b);
    }

    #[test]
    fn hex_round_trip() {
        assert_eq!(hex64(0xaef9_eee6_1540_d8b9), "aef9eee61540d8b9");
        assert_eq!(hex64(0), "0000000000000000");
        assert_eq!(parse_hex64("aef9eee61540d8b9"), Some(0xaef9_eee6_1540_d8b9));
        assert_eq!(parse_hex64("FF"), Some(255));
        assert_eq!(parse_hex64(""), None);
        assert_eq!(parse_hex64("12345678901234567"), None);
        assert_eq!(parse_hex64("0x1"), None);
    }
}
