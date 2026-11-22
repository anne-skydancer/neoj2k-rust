// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(all(feature = "encode", not(miri)))]

use std::io::{self, Write};
use std::num::NonZeroUsize;

use neo_j2k::{Decoder, DecoderConfig, Diagnostic, EncodeParams, Encoder, Error, ErrorKind, ImageSamples, ImageView};
use neo_j2k_oracle::{EncodeRequest, EncodeTrace, EncodeTraceOptions, HeaderDiagnostic, encode_u8_trace};

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

fn cpp_trace(image: &ImageView<'_>, params: &EncodeParams, writer: &Writer, buffer_size: NonZeroUsize) -> EncodeTrace {
    let ImageSamples::Interleaved {
        samples,
        components,
        row_stride,
    } = image.samples
    else {
        panic!("the oracle takes interleaved inputs");
    };
    let request = EncodeRequest {
        pixels: samples,
        row_stride,
        width: image.width,
        height: image.height,
        components: u32::from(components),
        flip_vertical: image.flip_vertical,
        reversible: params.reversible,
        // Rust's automatic setting picks four resolutions for an 8 × 8 image.
        resolutions: u32::from(params.resolutions.unwrap_or(4)),
        mct: params.mct,
        max_codestream_size: params.max_codestream_size,
        layer_ratios: &params.layer_ratios,
        comment: params.comment.as_deref(),
    };
    encode_u8_trace(
        &request,
        EncodeTraceOptions {
            buffer_size,
            max_write: writer.max_write,
            stop_at: writer.stop_at,
            fail_with_error: writer.fail_with_error,
            extra_options: &params.extra_options,
        },
    )
}

fn compare_trace(expected: &EncodeTrace, writer: &Writer, outcome: &Result<(), Error>, encoder: &Encoder) {
    assert_eq!(
        outcome.as_ref().err().map_or(0, |error| error.kind() as i32),
        expected.status,
        "Rust result {outcome:?}, C++ diagnostics {:?}",
        expected.diagnostics
    );
    assert_eq!(writer.offers, expected.offers, "offered write sizes");
    assert_eq!(writer.data, expected.bytes, "accepted output prefix");
    let diagnostics: Vec<_> = encoder
        .diagnostics()
        .iter()
        .map(|diagnostic| match diagnostic {
            Diagnostic::Info(text) => HeaderDiagnostic::Info(text.as_bytes().to_vec()),
            Diagnostic::Warning(text) => HeaderDiagnostic::Warning(text.as_bytes().to_vec()),
            Diagnostic::Error(text) => HeaderDiagnostic::Error(text.as_bytes().to_vec()),
            other => panic!("unexpected Rust diagnostic {other:?}"),
        })
        .collect();
    assert_eq!(diagnostics, expected.diagnostics);
    if let Err(error) = outcome {
        assert_eq!(
            expected.diagnostics.last(),
            Some(&HeaderDiagnostic::Error(error.message().as_bytes().to_vec()))
        );
    }
}

#[test]
fn writes_match_cpp_across_buffer_and_short_write_sizes() {
    let samples = std::array::from_fn(|i| u8::try_from(i * 4).unwrap());
    let image = image(&samples);
    let mut encoder = Encoder::new();
    let mut params = EncodeParams::default();
    for reversible in [true, false] {
        params.reversible = reversible;
        let expected = encoder.encode(&image, &params).unwrap().to_vec();
        for buffer_size in [1, 16, 4096] {
            let buffer_size = NonZeroUsize::new(buffer_size).unwrap();
            for max_write in [1, 7, usize::MAX] {
                let mut writer = Writer::new(max_write);
                writer.interrupt = true;
                writer.interrupt_flush = true;
                let trace = cpp_trace(&image, &params, &writer, buffer_size);
                let outcome = encoder.encode_to(&image, &params, &mut writer, buffer_size);
                compare_trace(&trace, &writer, &outcome, &encoder);
                outcome.unwrap();
                assert_eq!(writer.data, expected);
                assert_eq!(writer.flushes, 2);
            }
        }
    }
}

#[test]
fn full_4096_byte_buffers_and_the_final_remainder_match_cpp() {
    let samples: Vec<_> = (0..128 * 128)
        .scan(1_u32, |state, _| {
            *state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            Some(state.to_be_bytes()[0])
        })
        .collect();
    let image = ImageView {
        width: 128,
        height: 128,
        flip_vertical: false,
        samples: ImageSamples::Interleaved {
            samples: &samples,
            components: 1,
            row_stride: 128,
        },
    };
    let mut params = EncodeParams::default();
    params.resolutions = Some(4);
    let mut encoder = Encoder::new();
    let buffer_size = NonZeroUsize::new(4096).unwrap();
    let expected = encoder.encode(&image, &params).unwrap().to_vec();
    assert!(expected.len() > 8192);
    for stop_at in [usize::MAX, 4097] {
        let mut writer = Writer::new(701);
        writer.stop_at = stop_at;
        let trace = cpp_trace(&image, &params, &writer, buffer_size);
        let outcome = encoder.encode_to(&image, &params, &mut writer, buffer_size);
        compare_trace(&trace, &writer, &outcome, &encoder);
        assert_eq!(outcome.is_ok(), stop_at == usize::MAX);
        assert_eq!(writer.data, expected[..stop_at.min(expected.len())]);
    }
}

#[test]
fn extra_options_keep_default_bytes_and_refuse_before_writing() {
    let samples = [127; 64];
    let image = image(&samples);
    let mut encoder = Encoder::new();
    let mut params = EncodeParams::default();
    let expected = encoder.encode(&image, &params).unwrap().to_vec();
    params.extra_options = ["PLT=NO\0ignored", "TLM=NO", "GUARD_BITS=2"].map(String::from).into();
    assert_eq!(encoder.encode(&image, &params).unwrap(), expected);
    let mut writer = Writer::new(7);
    let buffer_size = NonZeroUsize::new(16).unwrap();
    let trace = cpp_trace(&image, &params, &writer, buffer_size);
    let outcome = encoder.encode_to(&image, &params, &mut writer, buffer_size);
    compare_trace(&trace, &writer, &outcome, &encoder);
    outcome.unwrap();

    for (option, kind) in [
        ("PLT=YES", ErrorKind::Unsupported),
        ("TLM=YES", ErrorKind::Unsupported),
        ("GUARD_BITS=+2tail", ErrorKind::Unsupported),
        ("GUARD_BITS=", ErrorKind::Unsupported),
        ("GUARD_BITS=9", ErrorKind::Argument),
        ("GUARD_BITS=-1", ErrorKind::Argument),
        ("PLT=MAYBE", ErrorKind::Argument),
        ("NOPE=1", ErrorKind::Argument),
    ] {
        params.layer_ratios.clear();
        params.extra_options = ["PLT=NO", option, "unknown after first failure"]
            .map(String::from)
            .into();
        let mut writer = Writer::new(usize::MAX);
        let trace = cpp_trace(&image, &params, &writer, buffer_size);
        let outcome = encoder.encode_to_and_update(&image, &mut params, &mut writer, buffer_size);
        compare_trace(&trace, &writer, &outcome, &encoder);
        let error = outcome.unwrap_err();
        assert_eq!(error.kind(), kind, "{option}");
        let message = if kind == ErrorKind::Unsupported {
            format!(
                "nj2k_encoder_set_extra_options: {option} is not supported by the core encoder (it belongs to the j2k-extra module)\n"
            )
        } else {
            format!("Invalid option: {option}.\n")
        };
        assert_eq!(error.message(), message);
        assert_eq!(encoder.diagnostics(), &[Diagnostic::Error(message)]);
        assert!(writer.offers.is_empty());
        assert_eq!(writer.flushes, 0);
        assert_eq!(params.layer_ratios, [0.0]);
    }

    params.extra_options = vec!["X".repeat(512)];
    let mut writer = Writer::new(usize::MAX);
    let trace = cpp_trace(&image, &params, &writer, buffer_size);
    let outcome = encoder.encode_to(&image, &params, &mut writer, buffer_size);
    compare_trace(&trace, &writer, &outcome, &encoder);
    let error = outcome.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Argument);
    assert_eq!(error.message().len(), 511);
    params.extra_options.clear();
    assert_eq!(encoder.encode(&image, &params).unwrap(), expected);
}

#[test]
fn refuses_failed_writes_in_each_phase_and_accepts_a_new_request() {
    let samples = [127; 64];
    let image = image(&samples);
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
        for buffer_size in [1, 16, 4096] {
            let buffer_size = NonZeroUsize::new(buffer_size).unwrap();
            for (max_write, fail_with_error) in [(7, false), (usize::MAX, true)] {
                let mut writer = Writer::new(max_write);
                writer.stop_at = stop_at;
                writer.fail_with_error = fail_with_error;
                let trace = cpp_trace(&image, &params, &writer, buffer_size);
                let outcome = encoder.encode_to(&image, &params, &mut writer, buffer_size);
                compare_trace(&trace, &writer, &outcome, &encoder);
                let error = outcome.unwrap_err();
                assert_eq!(error.kind(), ErrorKind::Output);
                if buffer_size == NonZeroUsize::MIN {
                    assert!(error.message().contains(message));
                }
                assert_eq!(writer.data, expected[..stop_at]);
                assert_eq!(writer.flushes, 0);
            }
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
