// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The viewer's rules the requests follow (`tests/common/viewer.cpp`, itself restated from the
//! viewer's behaviour: `calcDataSizeJ2C`, `getMetadata`, `LLJ2CReduce`, `LLJ2CDecodeGuard`). Same
//! types and arithmetic as the C++; where the C++ would overflow a signed `int` (undefined), these
//! wrap, as the C++ compiles on x86-64. The oracle crate's tests compare each with the C++.

/// `MAX_IMAGE_COMPONENTS` (`llimage.h`).
pub const MAX_COMPONENTS: u64 = 8;
/// The bytes of a texture's first request (`lltexturefetch`).
pub const FIRST_REQUEST_BYTES: usize = 600;

/// `LLImageJ2C::calcDataSizeJ2C`: the bytes the fetcher asks for at a discard level, from the image
/// size `getMetadata` stores.
#[must_use]
pub fn data_size_j2c(w: i32, h: i32, discard: i32) -> i64 {
    // 64x64 blocks at discard 5, 4 components of 8 bits, compression rate 1/8, then a 600-byte
    // header
    const BASE_AREA: i64 = 64 * 64;
    const BITS_PER_TILE: i64 = 4 * 8;
    const HEADER_BYTES: i64 = 600;
    const HARD_CAP: i32 = 12;
    const RATE: f64 = 1.0 / 8.0;
    // std::llround: halves away from zero, as f64::round; the products are exact in f64
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    let scaled = |area: i64| ((area * BITS_PER_TILE) as f64 * RATE).round() as i64;
    let discard_layers = 5i32.wrapping_sub(discard).max(0);
    let mut bits = scaled(BASE_AREA);
    if w > 0 && h > 0 {
        let surface = i64::from(w) * i64::from(h);
        let mut area = BASE_AREA;
        let mut layers = 1i32;
        while surface > area && layers < HARD_CAP {
            if layers <= discard_layers {
                bits += scaled(area);
            }
            layers += 1;
            area *= 4;
        }
        let max_dim = w.max(h);
        // float arithmetic, as the C++: float(max_dim) / 64.0f, std::log2(float), std::floor
        #[allow(clippy::cast_precision_loss)]
        let ratio = max_dim as f32 / 64.0f32;
        #[allow(clippy::cast_possible_truncation)]
        let dim_layers = if ratio > 0.0 {
            (ratio.log2().floor() as i32 + 1).max(1)
        } else {
            1
        };
        if dim_layers > layers {
            let mut extra = dim_layers.min(HARD_CAP) - layers;
            while extra > 0 {
                extra -= 1;
                if layers <= discard_layers {
                    bits += scaled(area);
                }
                layers += 1;
                area *= 4;
            }
        }
    }
    bits / 8 + HEADER_BYTES
}

/// The side `getMetadata` stores: tiles x tile size, in 32 bits, as a signed int.
#[must_use]
pub fn grid_side(tiles: u64, tile_size: u32) -> i32 {
    // S32(tilesW * tileDimX): the 64-bit product cut to 32 bits, then reinterpreted
    #[allow(clippy::cast_possible_truncation)]
    let low = tiles.wrapping_mul(u64::from(tile_size)) as u32;
    low.cast_signed()
}

/// `LLJ2CDecodeGuard::componentsSupported`: the component counts `getMetadata` takes (1 to 8).
#[must_use]
pub fn components_supported(components: u64) -> bool {
    (1..=MAX_COMPONENTS).contains(&components)
}

/// `LLJ2CReduce::reduceFactor`: never ask for a coarser level than the codestream has.
#[must_use]
pub fn reduce_factor(requested_discard: i32, resolutions: i32, width: i32, height: i32, max_image_size: i32) -> i32 {
    if resolutions <= 0 || requested_discard < resolutions || requested_discard > 30 {
        return requested_discard;
    }
    let coarsest = resolutions - 1;
    let side = i64::from(max_image_size) >> requested_discard;
    let round = |v: i32| (i64::from(v) + (1i64 << coarsest) - 1) >> coarsest;
    let pixels = round(width) * round(height);
    if pixels <= side * side {
        coarsest
    } else {
        requested_discard
    }
}

/// `LLJ2CDecodeGuard::decodeAllowed` with `MAX_IMAGE_COMPONENTS` (8).
#[must_use]
pub fn decode_allowed(width: u64, height: u64, components: u64, reduce: u32) -> bool {
    const MAX_SIDE: u64 = 16384;
    const MAX_BYTES: u64 = 1 << 30;
    const BYTES_PER_SAMPLE: u64 = 4;
    if !components_supported(components) || reduce > 31 {
        return false;
    }
    // unsigned arithmetic, wrapping as the C++'s
    let at_reduce = |v: u64| v.wrapping_add((1u64 << reduce) - 1) >> reduce;
    let (w, h) = (at_reduce(width), at_reduce(height));
    w <= MAX_SIDE && h <= MAX_SIDE && w * h * BYTES_PER_SAMPLE <= MAX_BYTES / components
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_size_at_known_points() {
        // 64 x 64 x 4 x 8 bits at 1/8 = 2048 bytes, then the header
        assert_eq!(data_size_j2c(0, 0, 0), 2048 + 600);
        assert!(data_size_j2c(1024, 1024, 0) > data_size_j2c(1024, 1024, 3));
        assert_eq!(data_size_j2c(1024, 1024, 5), data_size_j2c(64, 64, 5));
    }

    #[test]
    fn grid_side_wraps_to_32_bits() {
        assert_eq!(grid_side(2, 1024), 2048);
        assert_eq!(grid_side(1, 0x8000_0000), i32::MIN);
        assert_eq!(grid_side(1 << 32, 7), 0);
    }

    #[test]
    fn guards() {
        assert!(!components_supported(0) && components_supported(8) && !components_supported(9));
        assert!(decode_allowed(16384, 16384, 1, 0));
        assert!(!decode_allowed(16385, 1, 1, 0));
        assert!(decode_allowed(16385, 1, 1, 1));
        assert!(!decode_allowed(1, 1, 1, 32));
        assert!(!decode_allowed(u64::MAX, 1, 1, 0));
    }

    #[test]
    fn reduce_clamps_to_the_coarsest_level() {
        assert_eq!(reduce_factor(6, 6, 1024, 1024, 4096), 5);
        assert_eq!(reduce_factor(3, 6, 1024, 1024, 4096), 3);
        assert_eq!(reduce_factor(31, 6, 1024, 1024, 4096), 31);
    }
}
