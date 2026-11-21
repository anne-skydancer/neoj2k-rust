// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(feature = "encode")]

use std::io::{self, Write};
use std::num::NonZeroUsize;

use neo_j2k::{Decoder, DecoderConfig, EncodeParams, Encoder, ErrorKind, ImageSamples, ImageView};

fn image(samples: &[u8; 64]) -> ImageView<'_> {
    ImageView {
        width: 8,
        height: 8,
        flip_vertical: false,
        samples: ImageSamples::Interleaved {
            samples,
            components: 1,
            row_stride: 8,
        },
    }
}

struct Writer {
    data: Vec<u8>,
    offers: Vec<usize>,
    flushes: usize,
    max_write: usize,
    stop_at: usize,
    interrupt: bool,
    interrupt_flush: bool,
    fail_with_error: bool,
}

impl Writer {
    fn new(max_write: usize) -> Self {
        Self {
            data: Vec::new(),
            offers: Vec::new(),
            flushes: 0,
            max_write,
            stop_at: usize::MAX,
            interrupt: false,
            interrupt_flush: false,
            fail_with_error: false,
        }
    }
}

impl Write for Writer {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if std::mem::take(&mut self.interrupt) {
            return Err(io::ErrorKind::Interrupted.into());
        }
        self.offers.push(data.len());
        let count = data.len().min(self.max_write).min(self.stop_at - self.data.len());
        if count == 0 && self.fail_with_error {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        self.data.extend_from_slice(&data[..count]);
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flushes += 1;
        if std::mem::take(&mut self.interrupt_flush) {
            return Err(io::ErrorKind::Interrupted.into());
        }
        Ok(())
    }
}

#[test]
fn writes_match_memory_encoding_across_buffer_and_short_write_sizes() {
    let samples = std::array::from_fn(|i| u8::try_from(i * 4).unwrap());
    let image = image(&samples);
    let mut encoder = Encoder::new();
    let mut params = EncodeParams::default();
    for reversible in [true, false] {
        params.reversible = reversible;
        let expected = encoder.encode(&image, &params).unwrap().to_vec();
        for buffer_size in [1, 16, 4096] {
            for max_write in [1, 7, usize::MAX] {
                let mut writer = Writer::new(max_write);
                writer.interrupt = true;
                writer.interrupt_flush = true;
                encoder
                    .encode_to(&image, &params, &mut writer, NonZeroUsize::new(buffer_size).unwrap())
                    .unwrap();
                let offers: Vec<_> = expected
                    .chunks(buffer_size)
                    .flat_map(|chunk| (1..=chunk.len()).rev().step_by(max_write))
                    .collect();
                assert_eq!(writer.offers, offers);
                assert_eq!(writer.data, expected);
                assert_eq!(writer.flushes, 2);
            }
        }
    }
}

#[test]
fn refuses_failed_writes_in_each_phase_and_accepts_a_new_request() {
    let image = image(&[127; 64]);
    let params = EncodeParams::default();
    let mut encoder = Encoder::new();
    let expected = encoder.encode(&image, &params).unwrap().to_vec();
    let mut decoder = Decoder::new(DecoderConfig::default());
    let header = decoder.read_header(&expected).unwrap();
    let tile_start = usize::try_from(header.first_tile_offset).unwrap();
    for (stop_at, message) in [
        (0, "main header"),
        (tile_start, "the tile"),
        (expected.len() - 2, "end of the codestream"),
    ] {
        for fail_with_error in [false, true] {
            let mut writer = Writer::new(usize::MAX);
            writer.stop_at = stop_at;
            writer.fail_with_error = fail_with_error;
            let error = encoder
                .encode_to(&image, &params, &mut writer, NonZeroUsize::MIN)
                .unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Output);
            assert!(error.message().contains(message));
            assert_eq!(writer.data, expected[..stop_at]);
            assert_eq!(writer.flushes, 0);
        }
    }

    let invalid = ImageView { width: 0, ..image };
    let mut writer = Writer::new(usize::MAX);
    assert_eq!(
        encoder
            .encode_to(&invalid, &params, &mut writer, NonZeroUsize::MIN)
            .unwrap_err()
            .kind(),
        ErrorKind::Argument
    );
    assert!(writer.offers.is_empty());
    assert_eq!(writer.flushes, 0);
    encoder
        .encode_to(&image, &params, &mut writer, NonZeroUsize::MIN)
        .unwrap();
    assert_eq!(writer.data, expected);
}
