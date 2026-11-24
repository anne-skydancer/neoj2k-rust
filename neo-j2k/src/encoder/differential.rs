// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use neo_j2k_oracle::{BackendKind, EncodeRequest, OracleBackend, encode_u8};
use neo_j2k_testkit::files::{corpus_dir, expand_inputs};
use neo_j2k_testkit::request::{Backend, Mode, Request};

use super::{EncodeParams, Encoder, ImageSamples, ImageView};

fn compare(encoder: &mut Encoder, image: &ImageView<'_>, params: &EncodeParams<'_>, what: &str) -> bool {
    let ImageSamples::Interleaved {
        samples,
        components,
        row_stride,
    } = image.samples
    else {
        unreachable!("the C++ oracle accepts interleaved inputs");
    };
    let side = image.width.min(image.height);
    let default_resolution = side.checked_ilog2().map_or(1, |log| (log + 1).min(6));
    let cpp = encode_u8(&EncodeRequest {
        pixels: samples,
        row_stride,
        width: image.width,
        height: image.height,
        components: u32::from(components),
        flip_vertical: image.flip_vertical,
        reversible: params.reversible,
        resolutions: params.resolutions.map_or(default_resolution, u32::from),
        mct: params.mct,
        max_codestream_size: params.max_codestream_size,
        layer_ratios: &params.layer_ratios,
        comment: params.comment.as_deref(),
    });
    let rust = encoder.encode(image, params);
    match (rust, cpp) {
        (Ok(rust), Ok(cpp)) => {
            assert_eq!(rust.len(), cpp.len(), "{what}: differing codestream size");
            if let Some((offset, (ours, theirs))) = rust.iter().zip(&cpp).enumerate().find(|(_, (a, b))| a != b) {
                panic!("{what}: codestream byte {offset} is {ours:#04x} in Rust and {theirs:#04x} in C++");
            }
            true
        }
        (Err(rust), Err(cpp)) => {
            assert_eq!(rust.kind() as i32, cpp, "{what}: {rust}");
            false
        }
        (Ok(rust), Err(cpp)) => panic!("{what}: Rust encoded {} bytes but C++ returned {cpp}", rust.len()),
        (Err(rust), Ok(cpp)) => panic!("{what}: Rust returned {rust} but C++ encoded {} bytes", cpp.len()),
    }
}

fn patterned_data(stride: usize, height: u32) -> Vec<u8> {
    let len = stride * usize::try_from(height).unwrap();
    let mut state = 0xc81f_19ab_u32;
    (0..len)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            state.to_le_bytes()[0]
        })
        .collect()
}

#[test]
fn encoded_bytes_match_cpp_across_core_settings() {
    let mut encoder = Encoder::new();
    let mut tested = 0;
    for (width, height) in [(1_u32, 1_u32), (2, 3), (8, 5), (17, 67), (65, 64), (129, 47)] {
        for components in [1_u16, 3, 5, 8] {
            let stride = usize::try_from(width).unwrap() * usize::from(components) + 3;
            let pixels = patterned_data(stride, height);
            let image = ImageView {
                width,
                height,
                flip_vertical: width % 2 == 1,
                samples: ImageSamples::Interleaved {
                    samples: &pixels,
                    components,
                    row_stride: stride,
                },
            };
            // The all-zero cases keep every pass; the positive and mixed ones take the rate allocator's normal path.
            let cases = [
                (true, vec![]),
                (true, vec![24.0, 0.0]),
                (false, vec![]),
                (false, vec![0.0, 0.0]),
                (false, vec![32.0, 8.0, 0.0]),
            ];
            for (reversible, layer_ratios) in cases {
                let params = EncodeParams {
                    reversible,
                    mct: components >= 3 && height % 2 == 1,
                    layer_ratios,
                    comment: (width == 8).then(|| b"\xe9Rust\0ignored".to_vec()),
                    ..EncodeParams::default()
                };
                let what = format!(
                    "{width}x{height}x{components}, reversible={reversible}, ratios={:?}, mct={}",
                    params.layer_ratios, params.mct
                );
                tested += usize::from(compare(&mut encoder, &image, &params, &what));
            }
        }
    }
    assert!(tested >= 45, "only {tested} settings encoded successfully");
}

#[test]
fn reused_encoder_matches_cpp_after_caps_and_refusals() {
    let (width, height, stride) = (63, 37, 63 * 3);
    let pixels = patterned_data(stride, height);
    let image = ImageView {
        width,
        height,
        flip_vertical: false,
        samples: ImageSamples::Interleaved {
            samples: &pixels,
            components: 3,
            row_stride: stride,
        },
    };
    let mut encoder = Encoder::new();
    for resolutions in [0, 33, 4, 2, 8, 3] {
        for cap in [900, 4_000, 0] {
            let params = EncodeParams {
                reversible: false,
                resolutions: Some(resolutions),
                mct: true,
                layer_ratios: vec![40.0, 20.0, 0.0],
                max_codestream_size: cap,
                ..EncodeParams::default()
            };
            let what = format!("{resolutions} resolutions and cap {cap}");
            assert_eq!(
                compare(&mut encoder, &image, &params, &what),
                matches!(resolutions, 2..=5)
            );
        }
    }
    for ratio in [f32::NAN, f32::INFINITY, -5.0, f32::NEG_INFINITY, 1.0e30, 1.0e-30] {
        let params = EncodeParams {
            reversible: true,
            resolutions: Some(4),
            mct: true,
            layer_ratios: vec![ratio, 8.0, 0.0],
            ..EncodeParams::default()
        };
        assert!(compare(&mut encoder, &image, &params, &format!("rate {ratio}")));
    }
    let invalid = ImageView {
        samples: ImageSamples::Interleaved {
            samples: &pixels,
            components: 3,
            row_stride: 1,
        },
        ..image
    };
    assert!(!compare(
        &mut encoder,
        &invalid,
        &EncodeParams::default(),
        "short stride"
    ));
    let invalid = EncodeParams {
        layer_ratios: vec![0.0; 101],
        ..EncodeParams::default()
    };
    assert!(!compare(&mut encoder, &image, &invalid, "layer limit"));
    let invalid = EncodeParams {
        comment: Some(vec![1; 65_532]),
        ..EncodeParams::default()
    };
    assert!(!compare(&mut encoder, &image, &invalid, "comment limit"));
    assert!(compare(&mut encoder, &image, &EncodeParams::default(), "retry"));
}

#[test]
fn corpus_images_reencode_byte_for_byte() {
    let mut encoder = Encoder::new();
    let mut decode = OracleBackend::new(BackendKind::Oracle);
    let mut count = 0;
    for path in expand_inputs(&corpus_dir(), false) {
        let bytes = std::fs::read(&path).unwrap();
        let image = decode.decode(&Request::new(&bytes, 0, Mode::Plain));
        let Some(first) = image.comps.first() else {
            continue;
        };
        let (width, height) = (first.w, first.h);
        if !image.ok
            || width < 4
            || height < 4
            || u64::from(width) * u64::from(height) > 360_000
            || image.comps.len() > 5
            || image.comps.len() != image.planes.len()
            || image
                .comps
                .iter()
                .any(|c| c.w != width || c.h != height || c.prec != 8 || c.sgnd != 0 || c.dx != 1 || c.dy != 1)
        {
            continue;
        }
        let components = image.planes.len();
        let row_stride = usize::try_from(width).unwrap() * components;
        let mut input = vec![0; usize::try_from(height).unwrap() * row_stride];
        for (c, plane) in image.planes.iter().enumerate() {
            for (sample, &value) in input[c..].iter_mut().step_by(components).zip(plane) {
                *sample = u8::try_from(value.clamp(0, 255)).unwrap();
            }
        }
        let image = ImageView {
            width,
            height,
            flip_vertical: false,
            samples: ImageSamples::Interleaved {
                samples: &input,
                components: u16::try_from(components).unwrap(),
                row_stride,
            },
        };
        for reversible in [true, false] {
            let params = EncodeParams {
                reversible,
                mct: components >= 3,
                layer_ratios: if reversible { vec![0.0] } else { vec![32.0, 8.0] },
                ..EncodeParams::default()
            };
            let what = format!("{}, reversible={reversible}", path.display());
            count += usize::from(compare(&mut encoder, &image, &params, &what));
        }
    }
    assert!(count >= 20, "only {count} corpus images could be re-encoded");
}
