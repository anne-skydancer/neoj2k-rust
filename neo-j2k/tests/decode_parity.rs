// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use neo_j2k::{DecodeParams, DecodeTarget, Decoder, DecoderConfig, ErrorKind, Limits, OutputPlane, SampleType};
use neo_j2k_oracle::{Codec, Format};
use neo_j2k_testkit::files::{base_name, corpus_dir, expand_inputs, testdata_dir};

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

fn compare(rust: &mut Decoder, cpp: &mut Codec, input: &[u8], label: &str) -> i32 {
    let (status, samples) = cpp.decode_i32(input);
    match rust.decode(input, DecodeParams::default()) {
        Ok(image) => {
            assert_eq!(i32::from(image.truncated), status, "{label}: status");
            let actual = image.components.iter().flat_map(|comp| comp.samples().iter().copied());
            assert_eq!(
                image.components.iter().map(|c| c.samples().len()).sum::<usize>(),
                samples.len(),
                "{label}: sample count"
            );
            for (index, (actual, expected)) in actual.zip(samples).enumerate() {
                assert_eq!(actual, expected, "{label}: sample {index}");
            }
        }
        Err(error) => {
            assert_eq!(
                error.kind() as i32,
                status,
                "{label}: {error:?}, {:?}",
                rust.diagnostics()
            );
        }
    }
    status
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
fn conformance_pixels_match_cpp_when_test_data_is_available() {
    let Some(root) = testdata_dir() else {
        return;
    };
    compare_openjpeg_files(&root.join("input/conformance"));
}

#[test]
fn nonregression_pixels_match_cpp_when_the_full_test_data_is_available() {
    if let Some(directory) = testdata_dir()
        .map(|root| root.join("input/nonregression"))
        .filter(|directory| directory.is_dir())
    {
        compare_openjpeg_files(&directory);
    }
}

fn compare_openjpeg_files(directory: &std::path::Path) {
    let files = expand_inputs(directory, true);
    assert!(!files.is_empty(), "no codestreams in {}", directory.display());
    let memory = 64 << 20;
    for path in files {
        let jp2 = path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("jp2") || ext.eq_ignore_ascii_case("jph"));
        let input = std::fs::read(&path).expect("a conformance file");
        for reduce in 0..=5 {
            let mut rust = Decoder::new(DecoderConfig {
                format: if jp2 {
                    neo_j2k::Format::Jp2
                } else {
                    neo_j2k::Format::J2k
                },
                reduce,
                limits: Limits {
                    max_memory: memory,
                    ..Limits::default()
                },
                capture_diagnostics: true,
                ..DecoderConfig::default()
            });
            let mut cpp = Codec::new(if jp2 { Format::Jp2 } else { Format::J2k }, reduce, memory).unwrap();
            compare(
                &mut rust,
                &mut cpp,
                &input,
                &format!("{}, reduce {reduce}", base_name(&path)),
            );
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
        let mut decoded_truncations = 0;
        let mut refusals = 0;
        for path in &files {
            let data = std::fs::read(path).expect("a generated codestream");
            if data.len() > 4096 {
                continue;
            }
            let name = base_name(path);
            for prefix in 0..data.len() {
                // At the deepest tested reduction, cover every byte around packet and
                // tile-part boundaries as well as headers that refuse the exact reduce factor.
                if reduce == 2 && prefix % 7 != 0 {
                    continue;
                }
                let status = compare(
                    &mut rust,
                    &mut cpp,
                    &data[..prefix],
                    &format!("{name}, reduce {reduce}, first {prefix} bytes"),
                );
                decoded_truncations += usize::from(status == 1);
                refusals += usize::from(status < 0);
            }
        }
        if reduce == 5 {
            assert!(
                decoded_truncations > 0,
                "reduce 5 must exercise successful truncated decodes"
            );
            assert!(refusals > 0, "reduce 5 must exercise refused prefixes");
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
                &mut [DecodeTarget {
                    bytes: &mut bytes,
                    planes: &packing
                }],
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
    let packing = [OutputPlane {
        offset: 0,
        ..packing[0]
    }];
    let error = rust
        .decode_into(
            &data,
            DecodeParams::default(),
            SampleType::U8,
            &mut [DecodeTarget {
                bytes: &mut bytes,
                planes: &packing,
            }],
        )
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Output);
    assert_eq!(original, bytes);
}

#[test]
fn caller_owned_targets_must_reference_selected_components() {
    let data = std::fs::read(corpus_dir().join("sl_64_rgb_97.j2k")).unwrap();
    let (mut rust, _) = codecs(0);
    let params = DecodeParams {
        components: Some(&[2, 0]),
        ..DecodeParams::default()
    };
    let image = rust.decode(&data, params).unwrap();
    let expected: Vec<u8> = image.components[0]
        .samples()
        .iter()
        .map(|sample| sample.to_le_bytes()[0])
        .collect();
    let mut bytes = vec![0xab; 64 * 64];
    let plane = OutputPlane {
        component: 1,
        offset: 0,
        row_stride: 64,
        sample_stride: 1,
    };
    let invalid = rust.decode_into(
        &data,
        params,
        SampleType::U8,
        &mut [DecodeTarget {
            bytes: &mut bytes,
            planes: &[plane],
        }],
    );
    assert_eq!(invalid.unwrap_err().kind(), ErrorKind::Output);
    assert!(bytes.iter().all(|&sample| sample == 0xab));

    let selected = OutputPlane { component: 2, ..plane };
    let partial = rust
        .decode_into(
            &data,
            params,
            SampleType::U8,
            &mut [DecodeTarget {
                bytes: &mut bytes,
                planes: &[selected],
            }],
        )
        .unwrap();
    assert!(!partial);
    assert_eq!(bytes, expected);
}
