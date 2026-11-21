// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use neo_j2k::{DecodeParams, Decoder, DecoderConfig};
use neo_j2k_oracle::{Codec, Format, Limits, ReducePolicy};
use neo_j2k_testkit::files::corpus_dir;
use neo_j2k_testkit::scan::{packet_starts, parse_main_header, scan_tile_parts};

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(corpus_dir().join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn codecs(reduce: u32, max_layers: u32, strict: bool) -> (Decoder, Codec, DecodeParams<'static>) {
    let decoder = Decoder::new(DecoderConfig {
        reduce,
        capture_diagnostics: true,
        ..DecoderConfig::default()
    });
    let oracle = Codec::with_options(
        Format::J2k,
        reduce,
        ReducePolicy::Exact,
        Limits::default(),
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
