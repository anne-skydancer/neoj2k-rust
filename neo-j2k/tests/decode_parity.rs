// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use neo_j2k::{DecodeParams, DecodeTarget, Decoder, DecoderConfig, ErrorKind, OutputPlane, SampleType};
use neo_j2k_oracle::{Codec, Format};
use neo_j2k_testkit::files::{base_name, corpus_dir, expand_inputs};

fn codecs(reduce: u32) -> (Decoder, Codec) {
    (
        Decoder::new(DecoderConfig {
            reduce,
            capture_diagnostics: true,
            ..DecoderConfig::default()
        }),
        Codec::new(Format::J2k, reduce, 0).expect("a C++ decoder"),
    )
}

fn compare(rust: &mut Decoder, cpp: &mut Codec, input: &[u8], label: &str) {
    let (status, samples) = cpp.decode_i32(input);
    match rust.decode(input, DecodeParams::default()) {
        Ok(image) => {
            assert_eq!(i32::from(image.truncated), status, "{label}: status");
            let actual = image.components.iter().flat_map(|comp| comp.samples().iter().copied());
            assert_eq!(image.components.iter().map(|c| c.samples().len()).sum::<usize>(), samples.len(), "{label}: sample count");
            for (index, (actual, expected)) in actual.zip(samples).enumerate() {
                assert_eq!(actual, expected, "{label}: sample {index}");
            }
        }
        Err(error) => {
            assert_eq!(error.kind() as i32, status, "{label}: {error:?}, {:?}", rust.diagnostics());
        }
    }
}

#[test]
fn whole_memory_decodes_match_cpp_at_every_reduce_level() {
    let corpus = corpus_dir();
    let files = expand_inputs(&corpus, false);
    assert!(!files.is_empty());
    let inputs: Vec<_> = files
        .iter()
        .map(|file| (base_name(file), std::fs::read(file).expect("the generated corpus")))
        .collect();
    for reduce in 0..=5 {
        let (mut rust, mut cpp) = codecs(reduce);
        for (name, input) in &inputs {
            compare(&mut rust, &mut cpp, input, &format!("{name}, reduce {reduce}"));
        }
    }
}

#[test]
fn selected_cut_memory_decodes_match_cpp() {
    for name in ["sl_64_rgb_97.j2k", "packed_ppm.j2k", "tile_origin_odd_97.j2k"] {
        let data = std::fs::read(corpus_dir().join(name)).expect("a generated codestream");
        for reduce in [0, 2] {
            let (mut rust, mut cpp) = codecs(reduce);
            for prefix in (0..data.len().min(600)).step_by(7) {
                compare(
                    &mut rust,
                    &mut cpp,
                    &data[..prefix],
                    &format!("{name}, reduce {reduce}, first {prefix} bytes"),
                );
            }
        }
    }
}

#[test]
fn small_codestream_prefixes_match_cpp() {
    let files = expand_inputs(&corpus_dir(), false);
    for reduce in [0, 2, 5] {
        let (mut rust, mut cpp) = codecs(reduce);
        for path in &files {
            let data = std::fs::read(path).expect("a generated codestream");
            if data.len() > 4096 {
                continue;
            }
            let name = base_name(path);
            for prefix in 0..data.len() {
                if reduce != 0 && prefix % 7 != 0 {
                    continue;
                }
                compare(
                    &mut rust,
                    &mut cpp,
                    &data[..prefix],
                    &format!("{name}, reduce {reduce}, first {prefix} bytes"),
                );
            }
        }
    }
}

#[test]
fn u8_interleaved_output_supports_bottom_up_rows_and_preserves_padding() {
    let data = std::fs::read(corpus_dir().join("sl_64_rgb_97.j2k")).unwrap();
    let (mut rust, mut cpp) = codecs(0);
    let width = 64;
    let height = 64;
    let row_stride = 3 * width + 16;
    let packing = [0_u32, 1, 2].map(|component| OutputPlane {
        component,
        offset: (height - 1) * row_stride + component as usize,
        row_stride: -isize::try_from(row_stride).unwrap(),
        sample_stride: 3,
    });
    let mut bytes = vec![0x8b; height * row_stride];
    let (status, expected) = cpp.decode_i32(&data);
    assert_eq!(status, 0);
    assert!(
        !rust
            .decode_into(
                &data,
                DecodeParams::default(),
                SampleType::U8,
                &mut [DecodeTarget { bytes: &mut bytes, planes: &packing }],
            )
            .unwrap()
    );
    for y in 0..height {
        for x in 0..width {
            for component in 0..3 {
                let source = component * width * height + y * width + x;
                let destination = (height - 1 - y) * row_stride + 3 * x + component;
                assert_eq!(bytes[destination], expected[source].to_le_bytes()[0]);
            }
        }
        assert_eq!(&bytes[y * row_stride + 3 * width..(y + 1) * row_stride], &[0x8b; 16]);
    }

    let original = bytes.clone();
    let packing = [OutputPlane { offset: 0, ..packing[0] }];
    let error = rust
        .decode_into(
            &data,
            DecodeParams::default(),
            SampleType::U8,
            &mut [DecodeTarget { bytes: &mut bytes, planes: &packing }],
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Output);
    assert_eq!(original, bytes);
}
