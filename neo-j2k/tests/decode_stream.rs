// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use std::io::{self, Cursor, Read};

use neo_j2k::{
    DecodeParams, DecodeTarget, Decoder, DecoderConfig, ErrorKind, Format, Jp2Channels, Limits, OutputPlane,
    SampleType, StreamOptions,
};
use neo_j2k_oracle::{BackendKind, OracleBackend};
use neo_j2k_testkit::files::corpus_dir;
use neo_j2k_testkit::request::{Backend, Mode, Request};

struct ShortReader<'a> {
    data: &'a [u8],
    interrupt: bool,
    fail_at_end: bool,
}

impl Read for ShortReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if std::mem::take(&mut self.interrupt) {
            return Err(io::ErrorKind::Interrupted.into());
        }
        if self.data.is_empty() && self.fail_at_end {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        let len = buf.len().min(7);
        self.data.read(&mut buf[..len])
    }
}

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(corpus_dir().join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

#[test]
fn seekable_streams_match_cpp_on_whole_and_cut_files() {
    let mut oracle = OracleBackend::new(BackendKind::NeoJ2k);
    for name in [
        "sl_64_rgb_97.j2k",
        "packed_ppm_tiles_97.j2k",
        "psot0_sop_97.j2k",
        "psot0_sop_noeoc_97.j2k",
        "psot0_tiles_noeoc_97.j2k",
    ] {
        let file = fixture(name);
        for data in [&file[..], &file[..file.len() - 3], &file[..file.len() / 2]] {
            for mode in [Mode::Plain, Mode::Strict] {
                let expected = oracle.decode(&Request::new(data, 0, mode));
                let mut decoder = Decoder::new(DecoderConfig::default());
                let params = DecodeParams {
                    strict: mode == Mode::Strict,
                    ..DecodeParams::default()
                };
                let mut reader = ShortReader {
                    data,
                    interrupt: true,
                    fail_at_end: false,
                };
                let actual = decoder.decode_from(
                    &mut reader,
                    params,
                    StreamOptions {
                        length: Some(data.len() as u64),
                        seekable: true,
                    },
                );
                assert_eq!(actual.is_ok(), expected.ok, "{name}: {} bytes, {mode}", data.len());
                if let Ok(image) = actual {
                    assert_eq!(image.components.len(), expected.planes.len());
                    for (component, samples) in image.components.iter().zip(expected.planes) {
                        assert_eq!(component.samples(), samples, "{name}: {} bytes, {mode}", data.len());
                    }
                }
            }
        }
    }
}

#[test]
fn jp2_streams_apply_channels_and_stop_at_the_codestream_box_end() {
    let input = include_bytes!("../../fuzz/seeds/jp2/palette-3x5.jp2");
    let mut bytes = input.to_vec();
    bytes.extend_from_slice(b"\x00\x00\x00\x0cxml trailing");
    let mut reader = Cursor::new(&bytes);
    let mut decoder = Decoder::new(DecoderConfig {
        format: Format::Jp2,
        ..DecoderConfig::default()
    });
    let header = decoder.read_header(&bytes).unwrap();
    let codestream_end = header.codestream_offset + header.codestream_length.unwrap();
    let params = DecodeParams {
        jp2_channels: Jp2Channels::Apply,
        ..DecodeParams::default()
    };
    let image = decoder
        .decode_from(
            &mut reader,
            params,
            StreamOptions {
                length: Some(bytes.len() as u64),
                seekable: false,
            },
        )
        .unwrap();
    let mut request = Request::new(&bytes, 0, Mode::Plain);
    request.jp2 = true;
    let expected = OracleBackend::new(BackendKind::NeoJ2k).decode(&request);
    assert!(expected.ok);
    assert_eq!(image.components.len(), expected.planes.len());
    for (component, samples) in image.components.iter().zip(expected.planes) {
        assert_eq!(component.samples(), samples);
    }
    assert_eq!(reader.position(), codestream_end);

    let error = decoder
        .decode_from(&mut &bytes[..], params, StreamOptions::default())
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Unsupported);
}

struct SkippedPayload {
    left: usize,
    interrupt: bool,
    fail_at_end: bool,
}

impl Read for SkippedPayload {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        assert!(buffer.len() <= 8192, "discarded box should be read in bounded chunks");
        if std::mem::take(&mut self.interrupt) {
            return Err(io::ErrorKind::Interrupted.into());
        }
        if self.left == 0 && self.fail_at_end {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        let len = buffer.len().min(self.left).min(257);
        buffer[..len].fill(0);
        self.left -= len;
        Ok(len)
    }
}

fn push_box_header(prefix: &mut Vec<u8>, kind: &[u8; 4], payload_len: usize) {
    prefix.extend_from_slice(&u32::try_from(8 + payload_len).unwrap().to_be_bytes());
    prefix.extend_from_slice(kind);
}

#[test]
fn skipped_jp2_boxes_keep_physical_offsets_and_palette_samples() {
    let input = include_bytes!("../../fuzz/seeds/jp2/palette-3x5.jp2");
    let mut decoder = Decoder::new(DecoderConfig {
        format: Format::Jp2,
        ..DecoderConfig::default()
    });
    let info = decoder.read_header(input).unwrap();
    let (original_start, original_tile, codestream_len) = (
        info.codestream_offset,
        info.first_tile_offset,
        info.codestream_length.unwrap(),
    );
    let params = DecodeParams {
        jp2_channels: Jp2Channels::Apply,
        ..DecodeParams::default()
    };
    let expected: Vec<_> = decoder
        .decode(input, params)
        .unwrap()
        .components
        .iter()
        .map(|plane| plane.samples().to_vec())
        .collect();

    let ftyp_end = 12 + u32::from_be_bytes(input[12..16].try_into().unwrap()) as usize;
    let jp2c = usize::try_from(original_start).unwrap() - 8;
    let mut prefix = input[..ftyp_end].to_vec();
    push_box_header(&mut prefix, b"free", 3);
    prefix.extend_from_slice(b"pad");
    prefix.extend_from_slice(&input[ftyp_end..jp2c]);
    let large = 1024 * 1024;
    push_box_header(&mut prefix, b"junk", large);
    let body = SkippedPayload {
        left: large,
        interrupt: true,
        fail_at_end: false,
    };
    let added = (11 + 8 + large) as u64;
    let options = StreamOptions {
        length: Some(input.len() as u64 + added),
        seekable: true,
    };
    let mut remainder = Cursor::new(&input[jp2c..]);
    let mut reader = prefix.as_slice().chain(body).chain(&mut remainder);
    let stream = decoder.read_header_from(&mut reader, options).unwrap();
    assert_eq!(stream.info().codestream_offset, original_start + added);
    assert_eq!(stream.info().first_tile_offset, original_tile + added);
    assert_eq!(stream.info().codestream_length, Some(codestream_len));
    let image = stream.decode(params).unwrap();
    assert!(!image.truncated);
    assert!(
        image
            .components
            .iter()
            .map(|plane| plane.samples())
            .eq(expected.iter().map(Vec::as_slice))
    );
    assert_eq!(remainder.position(), 8 + codestream_len);
}

#[test]
fn eof_and_read_failure_inside_a_skipped_jp2_box_are_distinct() {
    let input = include_bytes!("../../fuzz/seeds/jp2/palette-3x5.jp2");
    let ftyp_end = 12 + u32::from_be_bytes(input[12..16].try_into().unwrap()) as usize;
    let mut prefix = input[..ftyp_end].to_vec();
    let declared = 1024 * 1024;
    push_box_header(&mut prefix, b"free", declared);
    let mut decoder = Decoder::new(DecoderConfig {
        format: Format::Jp2,
        ..DecoderConfig::default()
    });
    let options = StreamOptions {
        length: Some((prefix.len() + declared) as u64),
        seekable: false,
    };
    for (fail_at_end, expected) in [(false, ErrorKind::NeedData), (true, ErrorKind::Data)] {
        let body = SkippedPayload {
            left: 73,
            interrupt: true,
            fail_at_end,
        };
        let mut reader = prefix.as_slice().chain(body);
        let error = decoder.read_header_from(&mut reader, options).unwrap_err();
        assert_eq!(error.kind(), expected);
    }
    let mut body = SkippedPayload {
        left: 73,
        interrupt: false,
        fail_at_end: true,
    };
    let mut reader = prefix.as_slice().chain(&mut body);
    let bounded = StreamOptions {
        length: Some((prefix.len() + 17) as u64),
        ..options
    };
    let error = decoder.read_header_from(&mut reader, bounded).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::NeedData);
    assert_eq!(body.left, 56, "discard stops at the advertised input length");
}

#[test]
fn skipped_jp2_boxes_are_read_again_after_rewinding_a_failed_reader() {
    let input = include_bytes!("../../fuzz/seeds/jp2/palette-3x5.jp2");
    let ftyp_end = 12 + u32::from_be_bytes(input[12..16].try_into().unwrap()) as usize;
    let body_len = 257;
    let mut bytes = input[..ftyp_end].to_vec();
    push_box_header(&mut bytes, b"free", body_len);
    bytes.resize(bytes.len() + body_len, 0);
    let box_end = bytes.len();
    bytes.extend_from_slice(&input[ftyp_end..]);
    let options = StreamOptions {
        length: Some(bytes.len() as u64),
        seekable: false,
    };
    let mut decoder = Decoder::new(DecoderConfig {
        format: Format::Jp2,
        ..DecoderConfig::default()
    });
    let info = decoder.read_header(&bytes).unwrap();
    let (start, tile) = (info.codestream_offset, info.first_tile_offset);
    let params = DecodeParams {
        jp2_channels: Jp2Channels::Apply,
        ..DecodeParams::default()
    };

    for cut in [box_end - 3, box_end + 3] {
        let mut reader = ShortReader {
            data: &bytes[..cut],
            interrupt: true,
            fail_at_end: true,
        };
        let error = decoder.read_header_from(&mut reader, options).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Data);
        assert!(decoder.codestream_coding().is_err());

        reader.data = &bytes;
        reader.fail_at_end = false;
        let stream = decoder.read_header_from(&mut reader, options).unwrap();
        assert_eq!(stream.info().codestream_offset, start);
        assert_eq!(stream.info().first_tile_offset, tile);
        assert!(!stream.decode(params).unwrap().truncated);
    }
}

#[test]
fn limits_are_checked_before_the_reader_consumes_tile_data() {
    let input = fixture("sl_64_rgb_97.j2k");
    let mut reader = Cursor::new(&input);
    let mut decoder = Decoder::new(DecoderConfig::default());
    let tile_start = decoder.read_header(&input).unwrap().first_tile_offset;
    decoder.set_limits(Limits {
        max_decoded_side: 32,
        ..Limits::default()
    });
    let error = decoder
        .decode_from(
            &mut reader,
            DecodeParams::default(),
            StreamOptions {
                length: Some(input.len() as u64),
                seekable: true,
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Limit);
    assert_eq!(reader.position(), tile_start + 2);
}

#[test]
fn early_eof_is_distinct_from_a_short_advertised_length() {
    let input = fixture("sl_64_rgb_97.j2k");
    let mut decoder = Decoder::new(DecoderConfig::default());
    let first_tile = decoder.read_header(&input).unwrap().first_tile_offset;
    let cut = usize::try_from(first_tile + 7).unwrap();
    for (length, message) in [
        (cut, "Marker size inconsistent with stream length\n"),
        (input.len(), "Stream too short\n"),
    ] {
        let mut reader = &input[..cut];
        let error = decoder
            .decode_from(
                &mut reader,
                DecodeParams::default(),
                StreamOptions {
                    length: Some(length as u64),
                    seekable: true,
                },
            )
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::NeedData);
        assert_eq!(error.message(), message);
    }
}

#[test]
fn hard_read_errors_leave_the_decoder_available_for_a_fresh_reader() {
    let input = fixture("sl_64_rgb_97.j2k");
    let mut decoder = Decoder::new(DecoderConfig::default());
    let first_tile = usize::try_from(decoder.read_header(&input).unwrap().first_tile_offset).unwrap();
    let expected: Vec<_> = decoder
        .decode(&input, DecodeParams::default())
        .unwrap()
        .components
        .iter()
        .map(|component| component.samples().to_vec())
        .collect();
    let options = StreamOptions {
        length: Some(input.len() as u64),
        seekable: true,
    };

    for (prefix, header_remains) in [(2, false), (first_tile + 7, true)] {
        let mut reader = ShortReader {
            data: &input[..prefix],
            interrupt: false,
            fail_at_end: true,
        };
        let error = decoder
            .decode_from(&mut reader, DecodeParams::default(), options)
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Data);
        assert_eq!(error.message(), "The input stream could not be read\n");
        assert_eq!(decoder.codestream_coding().is_ok(), header_remains);

        let image = decoder
            .decode_from(&mut &input[..], DecodeParams::default(), options)
            .unwrap();
        assert!(!image.truncated);
        assert!(
            image
                .components
                .iter()
                .map(|component| component.samples())
                .eq(expected.iter().map(Vec::as_slice))
        );
    }
}

#[test]
fn unseekable_open_tile_reserves_two_bytes_for_eoc() {
    let input = fixture("psot0_sop_noeoc_97.j2k");
    let mut with_eoc = input.clone();
    let end = with_eoc.len();
    with_eoc[end - 2..].copy_from_slice(&[0xff, 0xd9]);
    let expected = OracleBackend::new(BackendKind::NeoJ2k).decode(&Request::new(&with_eoc, 0, Mode::Plain));
    assert!(expected.ok);
    let mut decoder = Decoder::new(DecoderConfig::default());
    let without_seek = decoder
        .decode_from(
            &mut &input[..],
            DecodeParams::default(),
            StreamOptions {
                length: Some(end as u64),
                seekable: false,
            },
        )
        .unwrap();
    assert!(without_seek.truncated);
    for (component, samples) in without_seek.components.iter().zip(&expected.planes) {
        assert_eq!(component.samples(), samples);
    }
    let with_seek = decoder
        .decode_from(
            &mut &input[..],
            DecodeParams::default(),
            StreamOptions {
                length: Some(end as u64),
                seekable: true,
            },
        )
        .unwrap();
    assert!(with_seek.truncated);
    assert!(
        with_seek
            .components
            .iter()
            .zip(&expected.planes)
            .any(|(component, samples)| component.samples() != samples),
        "the two final bytes carry samples when the stream can look for a missing EOC"
    );
}

#[test]
fn stream_header_can_be_inspected_without_reading_any_tile_data() {
    let input = fixture("sl_64_rgb_97.j2k");
    let mut reader = Cursor::new(&input);
    let mut decoder = Decoder::new(DecoderConfig::default());
    let after_header = {
        let stream = decoder
            .read_header_from(
                &mut reader,
                StreamOptions {
                    length: None,
                    seekable: false,
                },
            )
            .unwrap();
        assert_eq!(stream.info().components.len(), 3);
        stream.info().first_tile_offset + 2
    };
    assert_eq!(reader.position(), after_header);
}

#[test]
fn stream_can_resume_at_a_late_reduce_into_borrowed_or_reusable_planes() {
    let input = fixture("tile_origin_odd_97.j2k");
    let mut request = Request::new(&input, 2, Mode::Plain);
    request.reduce_after_header = true;
    let expected = OracleBackend::new(BackendKind::NeoJ2k).decode(&request);
    assert!(expected.ok);
    let options = StreamOptions {
        length: Some(input.len() as u64),
        seekable: true,
    };

    let mut decoder = Decoder::new(DecoderConfig::default());
    let mut reader = Cursor::new(&input);
    let mut stream = decoder.read_header_from(&mut reader, options).unwrap();
    stream.set_reduce(2).unwrap();
    let dims = stream.decoded_dimensions().unwrap()[0];
    let width = usize::try_from(dims.width).unwrap();
    let height = usize::try_from(dims.height).unwrap();
    let mut data = vec![0; width * height * size_of::<i32>()];
    let planes = [OutputPlane {
        component: 0,
        offset: 0,
        row_stride: isize::try_from(width * size_of::<i32>()).unwrap(),
        sample_stride: size_of::<i32>(),
    }];
    let truncated = stream
        .decode_into(
            DecodeParams::default(),
            SampleType::I32,
            &mut [DecodeTarget {
                bytes: &mut data,
                planes: &planes,
            }],
        )
        .unwrap();
    assert!(!truncated);
    assert!(
        data.as_chunks::<4>()
            .0
            .iter()
            .copied()
            .map(i32::from_ne_bytes)
            .eq(expected.planes[0].iter().copied())
    );
    assert_eq!(reader.position(), input.len() as u64);

    let mut decoder = Decoder::new(DecoderConfig::default());
    let mut reader = Cursor::new(&input);
    let mut stream = decoder.read_header_from(&mut reader, options).unwrap();
    stream.set_reduce(2).unwrap();
    let image = stream.decode(DecodeParams::default()).unwrap();
    assert!(!image.truncated);
    for (component, samples) in image.components.iter().zip(expected.planes) {
        assert_eq!(component.samples(), samples);
    }
}
