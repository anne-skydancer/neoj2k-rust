// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use neo_j2k::{DecodeParams, Decoder, DecoderConfig};
use neo_j2k_oracle::{Codec, Format};
use neo_j2k_testkit::files::{base_name, corpus_dir, expand_inputs};

#[test]
fn generated_decodes_match_cpp_arena_reservations() {
    let files = expand_inputs(&corpus_dir(), false);
    assert!(!files.is_empty(), "generated JPEG 2000 corpus");
    let inputs: Vec<_> = files
        .iter()
        .map(|path| (base_name(path), std::fs::read(path).unwrap()))
        .collect();
    let mut successful = 0;
    for reduce in 0..=5 {
        let (mut decoder, mut oracle) = pair(reduce);
        for (name, input) in &inputs {
            let (mut fresh, mut fresh_oracle) = pair(reduce);
            let expected = compare(&mut fresh, &mut fresh_oracle, input, name, reduce, "fresh");
            successful += usize::from(expected >= 0);
            assert_eq!(
                compare(&mut decoder, &mut oracle, input, name, reduce, "reused"),
                expected,
                "{name}, reduce {reduce}: reuse status"
            );
        }
    }
    assert!(successful >= inputs.len(), "successful whole generated codestreams");
}

fn pair(reduce: u32) -> (Decoder, Codec) {
    let decoder = Decoder::new(DecoderConfig {
        reduce,
        retain_bytes: u64::MAX,
        ..DecoderConfig::default()
    });
    let mut oracle = Codec::new(Format::J2k, reduce, 0).unwrap();
    assert!(oracle.set_retained_bytes(usize::MAX));
    (decoder, oracle)
}

fn compare(decoder: &mut Decoder, oracle: &mut Codec, input: &[u8], name: &str, reduce: u32, reuse: &str) -> i32 {
    let (status, _) = oracle.decode_i32(input);
    let actual = decoder.decode(input, DecodeParams::default());
    let outcome = actual.map_or_else(|error| error.kind() as i32, |image| i32::from(image.truncated));
    let case = format!("{name}, reduce {reduce}, {reuse} decoder");
    assert_eq!(outcome, status, "{case}: decode status");
    if status < 0 {
        return status;
    }
    let actual = decoder.statistics();
    let expected = oracle.ledger();
    assert_eq!(actual.header_reserved, expected.header_reserved, "{case}: header arena");
    assert_eq!(actual.tile_reserved, expected.tile_reserved, "{case}: tile arena");
    assert_eq!(actual.work_reserved, expected.work_reserved, "{case}: work arena");

    // The C++ codec resets its work cursor during end-of-request trimming. Rust still exposes
    // the request's peak, which must fit in the unreleased work reservation.
    assert_eq!(expected.work_high_water, 0, "{case}: C++ post-decode peak");
    assert!(
        (1..=actual.work_reserved).contains(&actual.work_high_water),
        "{case}: Rust decode peak"
    );
    status
}
