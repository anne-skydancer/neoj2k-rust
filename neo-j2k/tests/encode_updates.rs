// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(all(feature = "encode", not(miri)))]

use std::io::{self, Write};
use std::num::NonZeroUsize;

use neo_j2k::{EncodeParams, Encoder, ErrorKind, ImageSamples, ImageView};
use neo_j2k_oracle::{EncodeRequest, encode_u8_with_updates};

const SIDE: u32 = 48;

fn image(pixels: &[u8]) -> ImageView<'_> {
    ImageView {
        width: SIDE,
        height: SIDE,
        flip_vertical: false,
        samples: ImageSamples::Interleaved {
            samples: pixels,
            components: 1,
            row_stride: SIDE as usize,
        },
    }
}

fn request<'a>(pixels: &'a [u8], params: &'a EncodeParams<'_>) -> EncodeRequest<'a> {
    EncodeRequest {
        pixels,
        row_stride: SIDE as usize,
        width: SIDE,
        height: SIDE,
        components: 1,
        flip_vertical: false,
        reversible: params.reversible,
        resolutions: params.resolutions.unwrap().into(),
        mct: false,
        max_codestream_size: params.max_codestream_size,
        layer_ratios: &params.layer_ratios,
        comment: None,
    }
}

#[test]
fn setup_writebacks_and_reuse_match_cpp() {
    let pixels: Vec<_> = (0..SIDE * SIDE)
        .map(|index| ((index * 29) ^ ((index / SIDE) * 7)).to_le_bytes()[0])
        .collect();
    let image = image(&pixels);
    let mut encoder = Encoder::new();
    for (reversible, cap, ratios) in [(true, 0, &[][..]), (false, 0, &[16.0, 7.3]), (false, 500, &[16.0, 4.0])] {
        let mut params = EncodeParams::default();
        params.reversible = reversible;
        params.resolutions = Some(5);
        params.max_codestream_size = cap;
        params.layer_ratios = ratios.to_vec();
        for _ in 0..2 {
            let (expected, updated) = encode_u8_with_updates(&request(&pixels, &params));
            let updated = updated.expect("C++ accepts the setup");
            let actual = encoder.encode_and_update(&image, &mut params).unwrap();
            assert_eq!(actual, expected.expect("C++ encodes the request"));
            assert_eq!(params.layer_ratios, updated.layer_ratios);
            assert_eq!(params.max_codestream_size, updated.max_codestream_size);
        }
    }
}

struct RefusingWriter;

impl Write for RefusingWriter {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::ErrorKind::BrokenPipe.into())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn writer_failure_preserves_accepted_setup_updates() {
    let pixels = [127; (SIDE * SIDE) as usize];
    let image = image(&pixels);
    let mut encoder = Encoder::new();
    let mut params = EncodeParams::default();
    params.resolutions = Some(13);
    let (expected, updated) = encode_u8_with_updates(&request(&pixels, &params));
    assert!(updated.is_none());
    assert_eq!(
        encoder
            .encode_to_and_update(&image, &mut params, &mut RefusingWriter, NonZeroUsize::MIN)
            .unwrap_err()
            .kind() as i32,
        expected.unwrap_err()
    );
    assert!(params.layer_ratios.is_empty());
    assert_eq!(params.max_codestream_size, 0);

    params.resolutions = Some(5);
    params.layer_ratios = vec![10.3];
    let mut original = params.clone();
    let (expected, updated) = encode_u8_with_updates(&request(&pixels, &original));
    let updated = updated.expect("C++ accepts the setup");
    let error = encoder
        .encode_to_and_update(&image, &mut params, &mut RefusingWriter, NonZeroUsize::MIN)
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Output);
    assert_eq!(params.layer_ratios, updated.layer_ratios);
    assert_eq!(params.max_codestream_size, updated.max_codestream_size);

    let mut output = Vec::new();
    encoder
        .encode_to_and_update(&image, &mut original, &mut output, NonZeroUsize::MIN)
        .unwrap();
    assert_eq!(output, expected.expect("C++ encodes the request"));
    assert_eq!(original.layer_ratios, params.layer_ratios);
    assert_eq!(original.max_codestream_size, params.max_codestream_size);
}
