// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use super::round_i32;

fn check(value: f32) {
    let rounded = value.round_ties_even();
    #[allow(
        clippy::cast_possible_truncation,
        reason = "the reference result is rounded and range checked"
    )]
    let expected = if (-2_147_483_648.0..2_147_483_648.0).contains(&rounded) {
        rounded as i32
    } else {
        i32::MIN
    };
    assert_eq!(round_i32(value), expected, "float bits {:08x}", value.to_bits());
}

#[test]
fn round_matches_ties_even_at_halfway_and_range_boundaries() {
    for magnitude in [
        0.0f32,
        0.5,
        1.5,
        2.5,
        127.5,
        128.5,
        255.5,
        256.5,
        65_535.5,
        65_536.5,
        4_194_303.5,
        4_194_304.5,
        8_388_607.5,
        8_388_608.0,
        2_147_483_648.0,
        f32::MIN_POSITIVE,
        f32::MAX,
    ] {
        for value in [magnitude, -magnitude] {
            for neighbor in [value.next_down(), value, value.next_up()] {
                check(neighbor);
            }
        }
    }
    for bits in [
        1,
        0x8000_0001,
        0x7f80_0000,
        0xff80_0000,
        0x7fc0_0000,
        0x7f80_0001,
        0xffc0_0001,
    ] {
        check(f32::from_bits(bits));
    }
}

#[test]
fn round_matches_ties_even_across_float_bit_patterns() {
    let exponents: Vec<u32> = if cfg!(miri) {
        vec![0, 1, 125, 126, 127, 148, 149, 150, 157, 158, 159, 254, 255]
    } else {
        (0..=255).collect()
    };
    for sign in [0, 1 << 31] {
        for &exponent in &exponents {
            for fraction in [0, 1, 0x3f_ffff, 0x40_0000, 0x40_0001, 0x7f_fffe, 0x7f_ffff] {
                check(f32::from_bits(sign | (exponent << 23) | fraction));
            }
        }
    }
    let mut state = 0x92d6_8ca2u32;
    for _ in 0..if cfg!(miri) { 128 } else { 32_768 } {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        check(f32::from_bits(state));
    }
}
