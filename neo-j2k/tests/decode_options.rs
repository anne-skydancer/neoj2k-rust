// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use std::{collections::BTreeSet, path::PathBuf};

use neo_j2k::{DecodeParams, Decoder, DecoderConfig, ErrorKind};
use neo_j2k_oracle::{Codec, Format, Limits, ReducePolicy};
use neo_j2k_testkit::files::{base_name, corpus_dir, expand_inputs};
use neo_j2k_testkit::scan::{packet_starts, parse_main_header, scan_tile_parts};

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(corpus_dir().join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn codecs(reduce: u32, max_layers: u32, strict: bool) -> (Decoder, Codec, DecodeParams<'static>) {
    configured_codecs(reduce, max_layers, strict, neo_j2k::Limits::default())
}

fn configured_codecs(
    reduce: u32,
    max_layers: u32,
    strict: bool,
    limits: neo_j2k::Limits,
) -> (Decoder, Codec, DecodeParams<'static>) {
    let decoder = Decoder::new(DecoderConfig {
        reduce,
        limits,
        capture_diagnostics: true,
        ..DecoderConfig::default()
    });
    let oracle = Codec::with_options(
        Format::J2k,
        reduce,
        ReducePolicy::Exact,
        Limits {
            max_tiles: limits.max_tiles,
            max_components: limits.max_components,
            max_layers: limits.max_layers,
            max_decoded_side: limits.max_decoded_side,
            max_memory: limits.max_memory,
        },
        max_layers,
        strict,
    )
    .expect("a C++ memory decoder");
    let params = DecodeParams {
        max_layers,
        strict,
        ..DecodeParams::default()
    };
    (decoder, oracle, params)
}

fn compare(
    decoder: &mut Decoder,
    oracle: &mut Codec,
    params: DecodeParams<'_>,
    data: &[u8],
    case: &str,
) -> (i32, Vec<i32>) {
    let (status, expected) = oracle.decode_i32(data);
    match decoder.decode(data, params) {
        Ok(image) => {
            assert_eq!(i32::from(image.truncated), status, "{case}: status");
            let actual = image
                .components
                .iter()
                .flat_map(|component| component.samples().iter().copied());
            let count = image
                .components
                .iter()
                .map(|component| component.samples().len())
                .sum::<usize>();
            assert_eq!(count, expected.len(), "{case}: sample count");
            for (index, (actual, expected)) in actual.zip(&expected).enumerate() {
                assert_eq!(actual, *expected, "{case}: sample {index}");
            }
        }
        Err(error) => assert_eq!(
            error.kind() as i32,
            status,
            "{case}: {error:?}, {:?}",
            decoder.diagnostics()
        ),
    }
    (status, expected)
}

#[test]
fn layer_limits_match_cpp_and_remove_late_contributions() {
    let mut changed = false;
    for name in [
        "layers_8_97.j2k",
        "layers_4_53.j2k",
        "layers_2_97.j2k",
        "poc_sop_97.j2k",
    ] {
        let input = fixture(name);
        for reduce in [0, 2] {
            let mut full = Vec::new();
            for max_layers in [0, 1, 2, 4, u32::MAX] {
                let (mut decoder, mut oracle, params) = codecs(reduce, max_layers, false);
                let case = format!("{name}, reduce {reduce}, layers {max_layers}");
                let (status, samples) = compare(&mut decoder, &mut oracle, params, &input, &case);
                assert_eq!(status, 0, "{case}");
                match max_layers {
                    0 => full = samples,
                    u32::MAX => assert_eq!(samples, full, "{case}: an oversized limit keeps all layers"),
                    _ => changed |= samples != full,
                }
            }
        }
    }
    assert!(changed, "at least one layer-limited decode differs from full quality");
}

#[test]
fn strict_cuts_match_cpp_with_in_band_and_packed_headers() {
    let mut rejected = 0;
    for name in ["mark_sop_eph_97.j2k", "packed_ppm_sop_eph_97.j2k", "packed_ppt.j2k"] {
        let input = fixture(name);
        let header = parse_main_header(&input).unwrap();
        let parts = scan_tile_parts(&input, &header);
        let part = &parts[0];
        let packet_offsets = packet_starts(&input, &header, part).unwrap_or_default();
        let mut cuts = vec![header.end, part.data, part.end - 1, input.len() - 2, input.len()];
        cuts.extend((part.data..part.end.min(part.data + 31)).step_by(2));
        for start in packet_offsets.into_iter().take(3) {
            cuts.extend(start..part.end.min(start + 9));
        }
        cuts.sort_unstable();
        cuts.dedup();

        for reduce in [0, 2] {
            let (mut decoder, mut oracle, params) = codecs(reduce, 1, true);
            for &cut in &cuts {
                let case = format!("{name}, strict, reduce {reduce}, first {cut} bytes");
                let (status, _) = compare(&mut decoder, &mut oracle, params, &input[..cut], &case);
                if cut == input.len() {
                    assert_eq!(status, 0, "{case}");
                }
                rejected += usize::from(status < 0);
            }
        }
    }
    assert!(rejected > 0, "strict cuts include refused codestreams");
}

fn damaged_inputs(input: &[u8]) -> Vec<(String, Vec<u8>)> {
    let header = parse_main_header(input).expect("a generated codestream header");
    let parts = scan_tile_parts(input, &header);
    let part = &parts[0];
    let mut cases = Vec::new();
    // SOC, then the SIZ marker's length, profile, geometry and component count.
    let mut offsets = vec![0, 1, 4, 5, 6, 7, 11, 15, 19, 23, 27, 31, 35, 39];
    for (offset, marker) in input[..header.end].windows(2).enumerate() {
        if matches!(marker, [0xff, 0x52 | 0x5c]) {
            offsets.extend(offset + 2..(offset + 10).min(header.end));
        }
    }
    offsets.extend(header.end..part.data);
    offsets.extend([
        part.data,
        part.data + 1,
        part.data.midpoint(part.end),
        part.end - 1,
        input.len() - 1,
    ]);
    offsets.sort_unstable();
    offsets.dedup();
    for offset in offsets.into_iter().filter(|&offset| offset < input.len()) {
        for mask in [1, 0x80] {
            let mut bytes = input.to_vec();
            bytes[offset] ^= mask;
            cases.push((format!("byte {offset} xor {mask:02x}"), bytes));
        }
    }
    for cut in [2, header.end - 1, header.end, part.data, part.end - 1, input.len() - 1] {
        cases.push((format!("prefix {cut}"), input[..cut].to_vec()));
    }
    cases
}

#[test]
fn generated_damage_matches_cpp_and_allows_reuse() {
    let files = expand_inputs(&corpus_dir(), false);
    let mut codecs: Vec<_> = [0, 5]
        .into_iter()
        .flat_map(|reduce| [false, true].map(move |strict| (reduce, strict)))
        .map(|(reduce, strict)| {
            (
                reduce,
                strict,
                configured_codecs(reduce, 0, strict, neo_j2k::Limits::SECOND_LIFE),
            )
        })
        .collect();
    let mut cases = 0;
    let mut statuses = BTreeSet::new();
    for path in files {
        let input = std::fs::read(&path).expect("a generated codestream");
        if input.len() > 16 * 1024 {
            continue;
        }
        let name = base_name(&path);
        for (description, damaged) in damaged_inputs(&input) {
            cases += 1;
            for (reduce, strict, (rust, cpp, params)) in &mut codecs {
                let case = format!("{name}, {description}, reduce {reduce}, strict {strict}");
                statuses.insert(compare(rust, cpp, *params, &damaged, &case).0);
            }
        }

        // Reuse the same decoders for a clean request after the damaged requests.
        for (reduce, strict, (rust, cpp, params)) in &mut codecs {
            compare(
                rust,
                cpp,
                *params,
                &input,
                &format!("{name}, retry, reduce {reduce}, strict {strict}"),
            );
        }
    }
    assert!(cases > 1000, "{cases} damaged inputs");
    for status in [0, 1, ErrorKind::NeedData as i32, ErrorKind::Data as i32] {
        assert!(
            statuses.contains(&status),
            "damaged inputs never produced status {status}"
        );
    }
    eprintln!(
        "Compared {cases} generated damaged streams with {} decode configurations",
        codecs.len()
    );
}

#[test]
fn local_damaged_corpus_matches_cpp_when_requested() {
    let Some(path) = std::env::var_os("NJ2K_DAMAGED_FILES").map(PathBuf::from) else {
        return;
    };
    let files = expand_inputs(&path, true);
    assert!(!files.is_empty(), "no damaged codestreams in {}", path.display());
    for reduce in [0, 5] {
        for strict in [false, true] {
            let (mut rust, mut cpp, params) = configured_codecs(reduce, 0, strict, neo_j2k::Limits::SECOND_LIFE);
            for file in &files {
                let input = std::fs::read(file).unwrap_or_else(|error| panic!("{}: {error}", file.display()));
                let case = format!("{}, reduce {reduce}, strict {strict}", base_name(file));
                compare(&mut rust, &mut cpp, params, &input, &case);
            }
        }
    }
    eprintln!(
        "Compared {} local damaged streams with 4 decode configurations",
        files.len()
    );
}
