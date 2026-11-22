// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use neo_j2k::{DecodeParams, Decoder, DecoderConfig, ErrorKind, Limits};
use neo_j2k_oracle::{Codec, Format};
use neo_j2k_testkit::files::corpus_dir;

fn cpp_memory_status(data: &[u8], reduce: u32, limit: u64) -> i32 {
    Codec::new(Format::J2k, reduce, limit).unwrap().decode_i32(data).0
}

fn rust_memory_status(decoder: &mut Decoder, data: &[u8]) -> i32 {
    match decoder.decode(data, DecodeParams::default()) {
        Ok(image) => i32::from(image.truncated),
        Err(error) => error.kind() as i32,
    }
}

#[test]
fn generated_decodes_match_the_cpp_budget_at_the_limit() {
    let denied = ErrorKind::Limit as i32;
    for name in [
        "sl_64_rgb_97.j2k",
        "tile_origin_odd_97.j2k",
        "packed_ppm.j2k",
        "large_1024_rgb_97.j2k",
    ] {
        let input = std::fs::read(corpus_dir().join(name)).unwrap();
        for reduce in [0, 2] {
            let mut high = 64 << 20;
            let mut low = 1;
            assert_eq!(cpp_memory_status(&input, reduce, high), 0, "{name}, reduce {reduce}");
            assert_eq!(
                cpp_memory_status(&input, reduce, low),
                denied,
                "{name}, reduce {reduce}"
            );
            while high - low > 1 {
                let limit = u64::midpoint(high, low);
                match cpp_memory_status(&input, reduce, limit) {
                    0 => high = limit,
                    status if status == denied => low = limit,
                    status => panic!("{name} with reduce {reduce} and limit {limit}: unexpected C++ status {status}"),
                }
            }
            let mut rust = Decoder::new(DecoderConfig {
                reduce,
                ..DecoderConfig::default()
            });
            let mut cpp = Codec::new(Format::J2k, reduce, 0).unwrap();
            // Exercise the same adjacent limits before and after the arenas hold a successful decode.
            for (limit, fresh) in [(low, denied), (high, 0), (high, 0), (low, denied), (high, 0)] {
                rust.set_limits(Limits {
                    max_memory: limit,
                    ..Limits::default()
                });
                assert!(cpp.set_max_memory(limit));
                let expected = cpp.decode_i32(&input).0;
                assert_eq!(
                    expected, fresh,
                    "{name} with reduce {reduce}, C++ reused at limit {limit}"
                );
                assert_eq!(
                    rust_memory_status(&mut rust, &input),
                    expected,
                    "{name} with reduce {reduce}, limit {limit}"
                );
            }
        }
    }
}
