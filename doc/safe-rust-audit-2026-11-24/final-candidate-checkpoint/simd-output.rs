// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Safe packing of finished component rows. Fixed-size pixel stores and zipped
//! input slices expose interleaving to the compiler without pointer operations.
//! The same packing pattern is used in Aurora-Viewer's aurora-j2k::mct (1ea0749).

const PIXELS: usize = 8;

/// Write complete eight-pixel groups; the caller handles the remaining pixels.
pub(crate) fn write<const CHANNELS: usize>(rows: &[&[i32]; CHANNELS], dest: &mut [u8]) -> usize {
    assert!((1..=4).contains(&CHANNELS));
    let limit = rows.iter().fold(dest.len() / CHANNELS, |n, row| n.min(row.len()));
    let pixels = limit / PIXELS * PIXELS;
    let dest = &mut dest[..pixels * CHANNELS];
    if CHANNELS == 2 && pixels >= 128 {
        // Wide two-channel rows benefit from packing eight words before one byte copy.
        // Keep the slice loops below for short rows and the other channel counts.
        let a = rows[0][..pixels].as_chunks::<PIXELS>().0;
        let b = rows[1][..pixels].as_chunks::<PIXELS>().0;
        for ((out, a), b) in dest.as_chunks_mut::<16>().0.iter_mut().zip(a).zip(b) {
            let bytes: [[u8; 2]; PIXELS] = core::array::from_fn(|lane| {
                let word = (a[lane].cast_unsigned() & 0xff) | ((b[lane].cast_unsigned() & 0xff) << 8);
                let [low, high, _, _] = word.to_le_bytes();
                [low, high]
            });
            out.copy_from_slice(bytes.as_flattened());
        }
        return pixels;
    }
    match CHANNELS {
        1 => {
            for (out, &a) in dest.iter_mut().zip(&rows[0][..pixels]) {
                *out = a.to_le_bytes()[0];
            }
        }
        2 => {
            for ((out, &a), &b) in dest
                .as_chunks_mut::<2>()
                .0
                .iter_mut()
                .zip(&rows[0][..pixels])
                .zip(&rows[1][..pixels])
            {
                *out = [a.to_le_bytes()[0], b.to_le_bytes()[0]];
            }
        }
        3 => {
            for (((out, &a), &b), &c) in dest
                .as_chunks_mut::<3>()
                .0
                .iter_mut()
                .zip(&rows[0][..pixels])
                .zip(&rows[1][..pixels])
                .zip(&rows[2][..pixels])
            {
                let pixel =
                    (a.cast_unsigned() & 0xff) | ((b.cast_unsigned() & 0xff) << 8) | ((c.cast_unsigned() & 0xff) << 16);
                let [r, g, b, _] = pixel.to_le_bytes();
                *out = [r, g, b];
            }
        }
        4 => {
            for ((((out, &a), &b), &c), &d) in dest
                .as_chunks_mut::<4>()
                .0
                .iter_mut()
                .zip(&rows[0][..pixels])
                .zip(&rows[1][..pixels])
                .zip(&rows[2][..pixels])
                .zip(&rows[3][..pixels])
            {
                *out = ((a.cast_unsigned() & 0xff)
                    | ((b.cast_unsigned() & 0xff) << 8)
                    | ((c.cast_unsigned() & 0xff) << 16)
                    | (d.cast_unsigned() << 24))
                    .to_le_bytes();
            }
        }
        _ => unreachable!("the channel count was validated"),
    }
    pixels
}

#[cfg(test)]
mod tests {
    use super::write;

    fn check<const CHANNELS: usize>() {
        for width in [0usize, 8, 120, 127, 128, 129, 136, 257] {
            let source: [Vec<i32>; CHANNELS] = core::array::from_fn(|channel| {
                (0..width + channel)
                    .map(|pixel| i32::try_from(pixel + channel * 31).unwrap().wrapping_mul(0x1234_5678))
                    .collect()
            });
            let rows = source.each_ref().map(|row| row.as_slice());
            for capacity in [
                0,
                (width * CHANNELS).saturating_sub(1),
                width * CHANNELS,
                width * CHANNELS + 7,
            ] {
                let mut bytes = vec![0x3d; 3 + capacity + 11];
                let mut expected = bytes.clone();
                let count = width.min(capacity / CHANNELS) / 8 * 8;
                for pixel in 0..count {
                    for channel in 0..CHANNELS {
                        expected[3 + pixel * CHANNELS + channel] = rows[channel][pixel].to_le_bytes()[0];
                    }
                }
                assert_eq!(write(&rows, &mut bytes[3..3 + capacity]), count);
                assert_eq!(bytes, expected, "{CHANNELS} channels, {width} pixels, {capacity} bytes");
            }
        }
    }

    #[test]
    fn avx2_packed_groups_preserve_limits_and_padding_at_wide_row_boundaries() {
        check::<1>();
        check::<2>();
        check::<3>();
        check::<4>();
    }
}
