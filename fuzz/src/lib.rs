// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use neo_j2k::{
    DecodeParams, DecodedImage, Decoder, DecoderConfig, EncodeParams, Encoder, Format, ImageSamples, ImageView, Limits,
    ReducePolicy,
};

const LIMITS: Limits = Limits {
    max_tiles: 256,
    max_components: 8,
    max_layers: 64,
    max_decoded_side: 1_024,
    max_memory: 32 << 20,
};
const MAX_CODESTREAM: usize = 1 << 20;
const JP2_SIGNATURE: &[u8] = &[0, 0, 0, 12, b'j', b'P', b' ', b' ', 13, 10, 135, 10];

fn check_geometry(image: &DecodedImage<'_>) {
    assert!(!image.components.is_empty());
    for component in image.components {
        let info = component.info();
        assert!(info.dx > 0 && info.dy > 0);
        assert!((1..=31).contains(&info.precision));
        assert_eq!(component.samples().len(), info.width as usize * info.height as usize);
    }
}

fn tail_byte(data: &[u8], index: usize) -> u8 {
    data.iter().rev().nth(index).copied().unwrap_or(0)
}

pub fn header_decode(data: &[u8]) {
    if data.len() > MAX_CODESTREAM {
        return;
    }
    let flags = tail_byte(data, 0);
    let format = if JP2_SIGNATURE.starts_with(data)
        || data.starts_with(&JP2_SIGNATURE[..8])
        || !data.starts_with(&[0xff, 0x4f]) && flags & 1 != 0
    {
        Format::Jp2
    } else {
        Format::J2k
    };
    let reduce_policy = if flags & 2 != 0 {
        ReducePolicy::Clamp
    } else {
        ReducePolicy::Exact
    };
    let mut decoder = Decoder::new(DecoderConfig {
        format,
        reduce_policy,
        limits: LIMITS,
        ..DecoderConfig::default()
    });
    let components = decoder.read_header(data).ok().map_or(0, |info| info.components.len());
    let _ = decoder.decoded_dimensions();
    let _ = decoder.set_reduce(u32::from(tail_byte(data, 1) % 7));
    let index = u32::from(tail_byte(data, 2)) % u32::try_from(components.max(1)).unwrap();
    let selection = [index];
    let params = DecodeParams {
        max_layers: u32::from(tail_byte(data, 3) & 7),
        strict: flags & 4 != 0,
        components: if flags & 8 == 0 { None } else { Some(&selection) },
        ..DecodeParams::default()
    };
    if let Ok(image) = decoder.decode(data, params) {
        check_geometry(&image);
    }
    let cut = usize::from(u16::from_le_bytes([tail_byte(data, 4), tail_byte(data, 5)])) % (data.len() + 1);
    if let Ok(image) = decoder.decode(&data[..cut], params) {
        check_geometry(&image);
    }
}

#[cfg(feature = "oracle")]
const DIFFERENTIAL_SEEDS: [&[u8]; 7] = [
    include_bytes!("../../../corpus/generated/sl_64_rgb_97.j2k"),
    include_bytes!("../../../corpus/generated/sl_32_rgba_53.j2k"),
    include_bytes!("../../../corpus/generated/packed_ppm.j2k"),
    include_bytes!("../../../corpus/generated/packed_ppt.j2k"),
    include_bytes!("../../../corpus/generated/tile_origin_odd_97.j2k"),
    include_bytes!("../../../corpus/generated/sub_420.j2k"),
    include_bytes!("../../../corpus/generated/poc_lrcp_rlcp.j2k"),
];

#[cfg(feature = "oracle")]
pub fn differential(data: &[u8]) {
    use neo_j2k_oracle::{Codec, Format as OracleFormat, Limits as OracleLimits, ReducePolicy as OracleReduce};

    if data.len() > 1 << 17 {
        return;
    }
    let control = |index| data.get(index).copied().unwrap_or(0);
    let mut mutated;
    let candidate = if data.starts_with(&[0xff, 0x4f]) {
        data
    } else {
        let seed = DIFFERENTIAL_SEEDS[usize::from(control(0)) % DIFFERENTIAL_SEEDS.len()];
        let position = usize::from(u16::from_le_bytes([control(3), control(4)])) % seed.len();
        match control(1) % 4 {
            0 => seed,
            1 => &seed[..position],
            2 => &seed[..seed.len() - 1 - usize::from(control(5)) % seed.len()],
            _ => {
                mutated = seed.to_vec();
                mutated[position] ^= 1 << (control(5) & 7);
                &mutated
            }
        }
    };
    let reduce = u32::from(control(2) % 6);
    let mut native = Decoder::new(DecoderConfig {
        limits: LIMITS,
        reduce,
        ..DecoderConfig::default()
    });
    let mut oracle = Codec::with_limits(
        OracleFormat::J2k,
        reduce,
        OracleReduce::Exact,
        OracleLimits {
            max_tiles: LIMITS.max_tiles,
            max_components: LIMITS.max_components,
            max_layers: LIMITS.max_layers,
            max_decoded_side: LIMITS.max_decoded_side,
            max_memory: LIMITS.max_memory,
        },
    )
    .expect("the C++ oracle decoder can be configured");
    let (status, samples) = oracle.decode_i32(candidate);
    match native.decode(candidate, DecodeParams::default()) {
        Ok(image) => {
            assert_eq!(i32::from(image.truncated), status, "decode status at reduce {reduce}");
            let planes = image.components.iter().map(|component| component.samples());
            assert_eq!(planes.clone().map(<[i32]>::len).sum::<usize>(), samples.len());
            for (index, (&ours, &theirs)) in planes.flatten().zip(&samples).enumerate() {
                assert_eq!(ours, theirs, "coefficient {index} at reduce {reduce}");
            }
        }
        Err(error) => assert_eq!(error.kind() as i32, status, "{error} at reduce {reduce}"),
    }
}

pub fn roundtrip(data: &[u8]) {
    let Some(([width, height, selector, flags], input)) = data.split_first_chunk::<4>() else {
        return;
    };
    let width = u32::from(width % 64) + 1;
    let height = u32::from(height % 64) + 1;
    let components = [1u16, 2, 3, 4][usize::from(selector & 3)];
    let stride = width as usize * usize::from(components);
    let len = stride * height as usize;
    let pixels: Vec<u8> = if input.is_empty() {
        vec![*selector; len]
    } else {
        input.iter().copied().cycle().take(len).collect()
    };
    let flip_vertical = flags & 1 != 0;
    let view = ImageView {
        width,
        height,
        flip_vertical,
        samples: ImageSamples::Interleaved {
            samples: &pixels,
            components,
            row_stride: stride,
        },
    };
    let mut encoder = Encoder::new();
    let mut decoder = Decoder::new(DecoderConfig {
        limits: LIMITS,
        ..DecoderConfig::default()
    });
    for reversible in [true, false] {
        let mut params = EncodeParams::default();
        params.reversible = reversible;
        params.mct = components >= 3 && flags & 2 != 0;
        let layered = flags & 4 != 0;
        if layered {
            params.layer_ratios = vec![2.0, 0.0];
        }
        let bytes = encoder.encode(&view, &params).expect("bounded core input encodes");
        #[cfg(feature = "oracle")]
        compare_encoding(&pixels, &view, &params, bytes, stride, components);
        let image = decoder
            .decode(bytes, DecodeParams::default())
            .expect("encoded codestream decodes");
        assert!(!image.truncated);
        assert_eq!(image.components.len(), usize::from(components));
        check_geometry(&image);
        for (index, component) in image.components.iter().enumerate() {
            let info = component.info();
            assert_eq!(
                (info.width, info.height, info.precision, info.signed),
                (width, height, 8, false)
            );
            for (row, values) in component.samples().chunks_exact(width as usize).enumerate() {
                let source_row = if flip_vertical { height as usize - 1 - row } else { row };
                for (col, &value) in values.iter().enumerate() {
                    if reversible {
                        assert_eq!(
                            value,
                            i32::from(pixels[source_row * stride + col * usize::from(components) + index])
                        );
                    } else {
                        assert!((0..=255).contains(&value));
                    }
                }
            }
        }
        if layered {
            let preview = decoder
                .decode(
                    bytes,
                    DecodeParams {
                        max_layers: 1,
                        ..DecodeParams::default()
                    },
                )
                .expect("one quality layer decodes");
            check_geometry(&preview);
            assert!(!preview.truncated);
        }
        if flags & 8 != 0 {
            let mut reduced = Decoder::new(DecoderConfig {
                limits: LIMITS,
                reduce: u32::from((flags >> 4) % 6),
                reduce_policy: ReducePolicy::Clamp,
                ..DecoderConfig::default()
            });
            let preview = reduced
                .decode(bytes, DecodeParams::default())
                .expect("a coarser resolution decodes");
            check_geometry(&preview);
            assert!(!preview.truncated);
            assert!(
                preview
                    .components
                    .iter()
                    .all(|plane| plane.info().width <= width && plane.info().height <= height)
            );
        }
    }
}

#[cfg(feature = "oracle")]
fn compare_encoding(
    pixels: &[u8],
    view: &ImageView<'_>,
    params: &EncodeParams,
    bytes: &[u8],
    stride: usize,
    components: u16,
) {
    let request = neo_j2k_oracle::EncodeRequest {
        pixels,
        row_stride: stride,
        width: view.width,
        height: view.height,
        components: u32::from(components),
        flip_vertical: view.flip_vertical,
        reversible: params.reversible,
        resolutions: u32::from(params.resolutions.unwrap_or(0)),
        mct: params.mct,
        max_codestream_size: params.max_codestream_size,
        layer_ratios: &params.layer_ratios,
        comment: params.comment.as_deref(),
    };
    let oracle = neo_j2k_oracle::encode_u8(&request).expect("the bounded core input encodes in the C++ oracle");
    assert_eq!(bytes.len(), oracle.len(), "codestream length");
    let mismatch = bytes.iter().zip(&oracle).position(|(ours, theirs)| ours != theirs);
    assert_eq!(mismatch, None, "first different encoded byte");
}
