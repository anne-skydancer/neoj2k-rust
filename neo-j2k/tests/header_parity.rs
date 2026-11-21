// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The Rust memory decoder's header and sizing API against the C++ implementation.

#![cfg(not(miri))]

use neo_j2k::{
    ComponentInfo, DecodeParams, Decoder, DecoderConfig, Diagnostic, ErrorKind, Format, Limits, ReducePolicy,
};
use neo_j2k_oracle::{self as cpp, Codec};
use neo_j2k_testkit::files::{base_name, corpus_dir, expand_inputs, testdata_dir};
use neo_j2k_testkit::request::Component;
use neo_j2k_testkit::scan::parse_main_header;

const NEED_DATA: i32 = -1;
const LIMIT: i32 = -6;

fn codecs(config: DecoderConfig) -> (Decoder, Codec) {
    let format = match config.format {
        Format::J2k => cpp::Format::J2k,
        Format::Jp2 => cpp::Format::Jp2,
    };
    let policy = match config.reduce_policy {
        ReducePolicy::Exact => cpp::ReducePolicy::Exact,
        ReducePolicy::Clamp => cpp::ReducePolicy::Clamp,
    };
    let limits = cpp::Limits {
        max_tiles: config.limits.max_tiles,
        max_components: config.limits.max_components,
        max_layers: config.limits.max_layers,
        max_decoded_side: config.limits.max_decoded_side,
        max_memory: config.limits.max_memory,
    };
    let oracle = Codec::with_limits(format, config.reduce, policy, limits).expect("a C++ codec");
    (Decoder::new(config), oracle)
}

fn compare_components(rust: &[ComponentInfo], cpp: &[Component], case: &str) {
    assert_eq!(rust.len(), cpp.len(), "{case}: component count");
    for (index, (rust, cpp)) in rust.iter().zip(cpp).enumerate() {
        assert_eq!(
            [
                rust.x0,
                rust.y0,
                rust.width,
                rust.height,
                rust.dx,
                rust.dy,
                rust.precision,
                rust.reduce
            ],
            [cpp.x0, cpp.y0, cpp.w, cpp.h, cpp.dx, cpp.dy, cpp.prec, cpp.factor],
            "{case}: component {index} x0/y0/w/h/dx/dy/precision/factor",
        );
        assert_eq!(rust.signed, cpp.sgnd != 0, "{case}: component {index} signedness");
    }
}

// Returns the header and sizing statuses so selected cases can confirm which check ran.
fn compare(decoder: &mut Decoder, oracle: &mut Codec, input: &[u8], case: &str) -> (i32, Option<i32>) {
    let expected = oracle.read_header(input);
    let actual = decoder.read_header(input);
    let header_status = actual.as_ref().err().map_or(0, |error| error.kind() as i32);
    assert_eq!(
        header_status,
        expected.as_ref().err().copied().unwrap_or(0),
        "{case}: header status"
    );
    let Ok(expected) = expected else {
        assert_eq!(
            decoder.header_reserved_bytes(),
            oracle.ledger().header_reserved,
            "{case}: reserved header bytes"
        );
        return (header_status, None);
    };
    let actual = actual.expect("matching successful header");
    assert_eq!(
        [actual.x0, actual.y0, actual.x1, actual.y1],
        expected.bounds,
        "{case}: bounds"
    );
    assert_eq!(
        [
            actual.tile_x0,
            actual.tile_y0,
            actual.tile_width,
            actual.tile_height,
            actual.tiles_across,
            actual.tiles_down
        ],
        expected.tiles,
        "{case}: tile grid",
    );
    assert_eq!(actual.layers, expected.layers, "{case}: quality layers");
    assert_eq!(
        actual.min_resolutions, expected.min_resolutions,
        "{case}: minimum resolutions"
    );
    assert_eq!(actual.progression as u32, expected.progression, "{case}: progression");
    assert_eq!(actual.color_space as i32, expected.color_space, "{case}: color space");
    compare_components(&actual.components, &expected.components, case);
    assert_eq!(
        decoder.header_reserved_bytes(),
        oracle.ledger().header_reserved,
        "{case}: reserved header bytes"
    );

    let actual = decoder.decoded_dimensions();
    let dimensions_status = actual.as_ref().err().map_or(0, |error| error.kind() as i32);
    assert_eq!(
        dimensions_status,
        expected.dimensions.as_ref().err().copied().unwrap_or(0),
        "{case}: dimensions status",
    );
    if let Ok(expected) = expected.dimensions {
        compare_components(actual.expect("matching successful dimensions"), &expected, case);
    }
    (header_status, Some(dimensions_status))
}

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(corpus_dir().join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

#[test]
fn generated_headers_match_cpp_at_every_reduce_level() {
    let files = expand_inputs(&corpus_dir(), false);
    assert!(files.len() >= 60, "{} generated codestreams", files.len());
    let inputs: Vec<_> = files
        .iter()
        .map(|path| (base_name(path), std::fs::read(path).unwrap()))
        .collect();
    let mut exact_refusals = 0;
    let mut accepted = 0;
    for policy in [ReducePolicy::Exact, ReducePolicy::Clamp] {
        for reduce in 0..=5 {
            let (mut decoder, mut oracle) = codecs(DecoderConfig {
                reduce,
                reduce_policy: policy,
                ..DecoderConfig::default()
            });
            for (name, input) in &inputs {
                let case = format!("{name}, reduce {reduce}, {policy:?}");
                match compare(&mut decoder, &mut oracle, input, &case) {
                    (0, Some(0)) => accepted += 1,
                    (LIMIT, None) if policy == ReducePolicy::Exact => exact_refusals += 1,
                    _ => {}
                }
            }
        }
    }
    assert!(accepted >= inputs.len());
    assert!(exact_refusals > 0);
}

#[test]
fn conformance_file_headers_match_cpp_when_test_data_is_available() {
    let Some(directory) = testdata_dir().map(|root| root.join("input/conformance")) else {
        return;
    };
    let files = expand_inputs(&directory, true);
    assert!(
        !files.is_empty(),
        "no conformance codestreams in {}",
        directory.display()
    );
    for path in files {
        let format = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) if matches!(ext.to_ascii_lowercase().as_str(), "jp2" | "jph") => Format::Jp2,
            _ => Format::J2k,
        };
        let input = std::fs::read(&path).unwrap();
        let (mut decoder, mut oracle) = codecs(DecoderConfig {
            format,
            ..DecoderConfig::default()
        });
        compare(&mut decoder, &mut oracle, &input, &base_name(&path));
    }
}

#[test]
fn selected_header_prefixes_match_exact_statuses_and_can_be_retried() {
    let (mut decoder, mut oracle) = codecs(DecoderConfig::default());
    for name in [
        "sl_64_rgb_97.j2k",
        "sub_420.j2k",
        "tile_origin_odd_97.j2k",
        "packed_ppm.j2k",
    ] {
        let input = fixture(name);
        let end = parse_main_header(&input).unwrap().end + 2;
        for size in 0..end {
            let case = format!("{name}, first {size} bytes");
            assert_eq!(
                compare(&mut decoder, &mut oracle, &input[..size], &case),
                (NEED_DATA, None),
                "{case}"
            );
        }
        assert_eq!(
            compare(&mut decoder, &mut oracle, &input[..end], name),
            (0, Some(0)),
            "{name}"
        );
        assert_eq!(compare(&mut decoder, &mut oracle, &input, name), (0, Some(0)), "{name}");
    }
}

fn boxed(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let size = u32::try_from(data.len() + 8).unwrap().to_be_bytes();
    [&size[..], kind, data].concat()
}

fn jp2_rgb64(codestream: &[u8]) -> Vec<u8> {
    let mut image_header = Vec::new();
    image_header.extend_from_slice(&64u32.to_be_bytes());
    image_header.extend_from_slice(&64u32.to_be_bytes());
    image_header.extend_from_slice(&3u16.to_be_bytes());
    image_header.extend_from_slice(&[7, 7, 0, 0]);
    let mut header = boxed(b"ihdr", &image_header);
    header.extend_from_slice(&boxed(b"colr", &[1, 0, 0, 0, 0, 0, 16]));
    [
        boxed(b"jP  ", b"\r\n\x87\n"),
        boxed(b"ftyp", b"jp2 \0\0\0\0jp2 "),
        boxed(b"jp2h", &header),
        boxed(b"jp2c", codestream),
    ]
    .concat()
}

#[test]
fn jp2_header_and_prefixes_match_cpp() {
    let codestream = fixture("sl_64_rgb_97.j2k");
    let jp2 = jp2_rgb64(&codestream);
    let config = DecoderConfig {
        format: Format::Jp2,
        reduce: 1,
        ..DecoderConfig::default()
    };
    let (mut decoder, mut oracle) = codecs(config);
    let end = jp2.len() - codestream.len() + parse_main_header(&codestream).unwrap().end + 2;
    for size in 0..=end {
        let case = format!("JP2, first {size} bytes");
        compare(&mut decoder, &mut oracle, &jp2[..size], &case);
    }
    assert_eq!(compare(&mut decoder, &mut oracle, &jp2, "complete JP2"), (0, Some(0)));
}

#[test]
fn header_and_dimension_limits_refuse_at_the_same_stage() {
    let input = fixture("sl_64_rgb_97.j2k");
    let (mut decoder, mut oracle) = codecs(DecoderConfig {
        limits: Limits {
            max_components: 2,
            ..Limits::default()
        },
        ..DecoderConfig::default()
    });
    assert_eq!(
        compare(&mut decoder, &mut oracle, &input, "component limit"),
        (LIMIT, None)
    );

    for (reduce, expected) in [(0, LIMIT), (1, 0)] {
        let (mut decoder, mut oracle) = codecs(DecoderConfig {
            reduce,
            limits: Limits {
                max_decoded_side: 32,
                ..Limits::default()
            },
            ..DecoderConfig::default()
        });
        let case = format!("decoded-side limit at reduce {reduce}");
        assert_eq!(compare(&mut decoder, &mut oracle, &input, &case), (0, Some(expected)));
    }
}

#[test]
fn changing_header_limits_revalidates_the_same_bytes() {
    let input = fixture("sl_64_rgb_97.j2k");
    let mut decoder = Decoder::new(DecoderConfig::default());
    assert_eq!(decoder.read_header(&input).unwrap().components.len(), 3);

    decoder.set_limits(Limits {
        max_components: 2,
        ..Limits::default()
    });
    assert_eq!(decoder.read_header(&input).unwrap_err().kind(), ErrorKind::Limit);
    assert_eq!(decoder.codestream_coding().unwrap_err().kind(), ErrorKind::State);

    decoder.set_limits(Limits::default());
    assert_eq!(decoder.read_header(&input).unwrap().components.len(), 3);
}

#[test]
fn changing_memory_limits_on_a_reused_decoder_matches_cpp() {
    let input = fixture("sl_64_rgb_97.j2k");
    let (mut decoder, mut oracle) = codecs(DecoderConfig::default());
    for (max_memory, expected_status) in [(0, 0), (1, LIMIT), (0, 0)] {
        assert!(oracle.set_max_memory(max_memory));
        decoder.set_limits(Limits {
            max_memory,
            ..Limits::default()
        });
        let (status, expected) = oracle.decode_i32(&input);
        assert_eq!(status, expected_status, "C++ status at {max_memory} bytes");
        match decoder.decode(&input, DecodeParams::default()) {
            Ok(image) => {
                assert_eq!(i32::from(image.truncated), status);
                assert!(image.components.iter().flat_map(|plane| plane.samples()).eq(&expected));
            }
            Err(error) => assert_eq!(error.kind() as i32, status),
        }
    }
}

#[test]
fn changing_reduce_after_the_header_matches_cpp() {
    let input = fixture("tile_origin_odd_97.j2k");
    for (policy, reduce, status) in [
        (ReducePolicy::Exact, 1, 0),
        (ReducePolicy::Exact, 32, LIMIT),
        (ReducePolicy::Clamp, 32, 0),
    ] {
        let (mut decoder, mut oracle) = codecs(DecoderConfig {
            reduce_policy: policy,
            ..DecoderConfig::default()
        });
        let case = format!("reduce after header to {reduce}, {policy:?}");
        let expected = oracle.read_header_then_reduce(&input, reduce).unwrap();
        let header = decoder.read_header(&input).unwrap();
        compare_components(&header.components, &expected.components, &case);
        let result = decoder.set_reduce(reduce);
        assert_eq!(expected.reduce_status, Some(status), "{case}: expected C++ status");
        assert_eq!(
            result.err().map_or(0, |error| error.kind() as i32),
            status,
            "{case}: setter status"
        );
        compare_components(
            decoder.decoded_dimensions().unwrap(),
            &expected.dimensions.unwrap(),
            &case,
        );
        if reduce == 1 {
            // The odd image origin yields 18 samples for decode-time sizing and 19 in the header.
            assert_eq!(decoder.decoded_dimensions().unwrap()[0].width, 18);
            assert_eq!(decoder.read_header(&input).unwrap().components[0].width, 19);
            assert_eq!(decoder.read_header(&input).unwrap().components[0].width, 19);
        }
    }
}

#[test]
fn header_reservation_and_diagnostics_match_cpp_in_order() {
    let input = fixture("sl_64_rgb_97.j2k");
    let end = parse_main_header(&input).unwrap().end;
    // Reserved segmentless markers can be skipped with a warning. Replace the SOT marker after
    // both warnings to check that the later error is captured in the same order.
    let warnings = [0xFF, 0x30, 0xFF, 0x31];
    let warned = [&input[..end], &warnings, &input[end..]].concat();
    let malformed = [&input[..end], &warnings, &[0, 0]].concat();
    let (mut decoder, mut oracle) = codecs(DecoderConfig {
        capture_diagnostics: true,
        ..DecoderConfig::default()
    });
    for (case, bytes, message_count) in [
        ("valid", input.as_slice(), 0),
        ("warned", warned.as_slice(), 2),
        ("malformed", malformed.as_slice(), 3),
        ("recovered", input.as_slice(), 0),
    ] {
        let (expected, diagnostics) = oracle.read_header_with_diagnostics(bytes);
        let actual = decoder.read_header(bytes);
        assert_eq!(
            actual.err().map_or(0, |error| error.kind() as i32),
            expected.err().unwrap_or(0),
            "{case}: header status",
        );
        assert_eq!(diagnostics.len(), message_count, "{case}: C++ diagnostic count");
        let messages: Vec<_> = decoder
            .diagnostics()
            .iter()
            .map(|entry| match entry {
                Diagnostic::Info(text) => cpp::HeaderDiagnostic::Info(text.as_bytes().to_vec()),
                Diagnostic::Warning(text) => cpp::HeaderDiagnostic::Warning(text.as_bytes().to_vec()),
                Diagnostic::Error(text) => cpp::HeaderDiagnostic::Error(text.as_bytes().to_vec()),
                _ => panic!("unexpected Rust diagnostic {entry:?}"),
            })
            .collect();
        assert_eq!(messages, diagnostics, "{case}: ordered diagnostics");
        assert_eq!(
            decoder.header_reserved_bytes(),
            oracle.ledger().header_reserved,
            "{case}: header reservation"
        );
    }
}
