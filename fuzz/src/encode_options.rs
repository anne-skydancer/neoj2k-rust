// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use neo_j2k::{EncodeParams, Encoder, ImageSamples, ImageView};
use neo_j2k_oracle::{EncodeRequest, encode_u8_with_updates};

fn side(low: u8, high: u8) -> u32 {
    if high & 0x80 == 0 {
        u32::from(low % 80) + 1
    } else {
        u32::from(u16::from_le_bytes([low, high & 0x7f]) % 5000) + 1
    }
}

/// Compare the options expressible by the Rust encoder using the C++ harness's input format.
pub fn encode_options(data: &[u8]) {
    let Some((options, source)) = data.split_first_chunk::<18>() else {
        return;
    };
    let [
        w0,
        w1,
        h0,
        h1,
        count,
        precision,
        image_flags,
        levels,
        layers,
        rate,
        rate_flags,
        toggles,
        comment,
        length,
        sample_flags,
        _,
        _,
        _,
    ] = *options;
    if (precision & 0xf0 == 0 && precision & 0x0f != 7)
        || image_flags & 0xe0 != 0
        || toggles & 0x7f != 0
        || sample_flags & 1 != 0
    {
        // This API accepts U8 planes and the core's one-tile RLCP coding settings.
        return;
    }
    let width = side(w0, w1);
    let height = side(h0, h1);
    let components = u16::from(count % 10) + 1;
    let stride = width as usize * usize::from(components);
    let size = stride * height as usize;
    let resolutions = if levels & 0x80 == 0 { levels % 9 } else { levels % 40 } + 1;
    let layer_count = if layers >= 200 { 101 } else { layers % 9 };
    let mct = image_flags & 2 != 0;
    // The test shim rejects an oversized rate array before invoking C++ setup. Isolate that
    // limit from the MCT and resolution errors that Rust setup can report first.
    if layer_count > 100 && ((mct && components < 3) || resolutions > 33) {
        return;
    }
    let exceeds_limits = width > 4096 || height > 4096 || components > 8 || size > 1 << 26 || layer_count > 100;
    if !exceeds_limits && size > 1 << 16 {
        return;
    }
    let pixels: Vec<_> = if exceeds_limits {
        vec![0]
    } else if source.is_empty() {
        vec![0; size]
    } else {
        source.iter().copied().cycle().take(size).collect()
    };

    let base = [0.0, 1.0, 2.0, 4.0, 8.0, 16.0, 100.0, 1.0e5][usize::from(rate % 8)];
    let mut params = EncodeParams::default();
    params.reversible = image_flags & 1 != 0;
    params.resolutions = Some(resolutions);
    params.mct = mct;
    params.max_codestream_size = if toggles & 0x80 == 0 { 0 } else { 5000 };
    params.layer_ratios = (0..layer_count)
        .map(|layer| {
            if rate_flags & 1 != 0 && layer & 1 != 0 {
                base * f32::from(layer + 1)
            } else {
                base * (1u32 << (2 * ((layer_count - 1 - layer) % 12))) as f32
            }
        })
        .collect();
    let length = usize::from(length % 200);
    params.comment = match comment % 5 {
        0 => None,
        1 => Some(Vec::new()),
        2 => Some(b"no comment".to_vec()),
        3 => Some(vec![b'x'; length]),
        _ => Some(vec![b'y'; 65532 + length]),
    };

    let (expected, updates) = encode_u8_with_updates(&EncodeRequest {
        pixels: &pixels,
        row_stride: stride,
        width,
        height,
        components: u32::from(components),
        flip_vertical: false,
        reversible: params.reversible,
        resolutions: u32::from(resolutions),
        mct: params.mct,
        max_codestream_size: params.max_codestream_size,
        layer_ratios: &params.layer_ratios,
        comment: params.comment.as_deref(),
    });
    let image = ImageView {
        width,
        height,
        flip_vertical: false,
        samples: ImageSamples::Interleaved {
            samples: &pixels,
            components,
            row_stride: stride,
        },
    };
    let mut encoder = Encoder::new();
    let result = encoder
        .encode_and_update(&image, &mut params)
        .map_err(|error| error.kind() as i32);
    assert_eq!(result, expected.as_deref().map_err(|&status| status), "{options:?}");
    if let Some(updates) = updates {
        assert_eq!(params.layer_ratios, updates.layer_ratios, "{options:?}");
        assert_eq!(params.max_codestream_size, updates.max_codestream_size, "{options:?}");
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::encode_options;

    #[test]
    fn shared_encoder_seeds() {
        let fuzz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fuzz");
        for entry in std::fs::read_dir(fuzz.join("seeds/encode")).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|extension| extension == "bin") {
                encode_options(&std::fs::read(path).unwrap());
            }
        }
        encode_options(include_bytes!("../regressions/encode-size-cap-lossless.bin"));
    }
}
