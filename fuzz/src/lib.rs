// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use neo_j2k::{
    DecodeParams, DecodeTarget, DecodedImage, Decoder, DecoderConfig, EncodeParams, Encoder, Format, ImageSamples,
    ImageView, Jp2Channels, Limits, OutputPlane, ReducePolicy, SampleType,
};
#[cfg(feature = "oracle")]
use neo_j2k_oracle::{Codec, Format as OracleFormat, Header, Limits as OracleLimits, ReducePolicy as OracleReduce};

#[cfg(feature = "oracle")]
mod encode_options;
#[cfg(feature = "oracle")]
pub use encode_options::encode_options;

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
        // JP2 palette metadata can declare up to 128 bits for a column.
        assert!((1..=128).contains(&info.precision));
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
        jp2_channels: match (flags >> 4) & 3 {
            1 => Jp2Channels::Apply,
            2 => Jp2Channels::Ignore,
            _ => Jp2Channels::Reject,
        },
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
    include_bytes!("../../corpus/generated/sl_64_rgb_97.j2k"),
    include_bytes!("../../corpus/generated/sl_32_rgba_53.j2k"),
    include_bytes!("../../corpus/generated/packed_ppm.j2k"),
    include_bytes!("../../corpus/generated/packed_ppt.j2k"),
    include_bytes!("../../corpus/generated/tile_origin_odd_97.j2k"),
    include_bytes!("../../corpus/generated/sub_420.j2k"),
    include_bytes!("../../corpus/generated/poc_lrcp_rlcp.j2k"),
];

#[cfg(feature = "oracle")]
pub fn differential(data: &[u8]) {
    if data.len() > 1 << 17 {
        return;
    }
    let raw_j2k = data.starts_with(&[0xff, 0x4f]);
    let raw_jp2 = data.starts_with(&JP2_SIGNATURE[..8]);
    let raw = raw_j2k || raw_jp2;
    // The first three codestream bytes are always the SOC and the start of SIZ.
    let options = if raw { &data[data.len() / 2..] } else { data };
    let control = |index| options.get(index).copied().unwrap_or(0);
    let mut mutated;
    let candidate = if raw {
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
    let format = if raw_jp2 { Format::Jp2 } else { Format::J2k };
    let reduce = u32::from(control(2) % 6);
    let (policy, params) = decode_options(control(6));
    let (mut native, mut oracle) = differential_codecs(format, reduce, policy, params, LIMITS);
    compare_decode(&mut native, &mut oracle, candidate, params);

    let cut = usize::from(u16::from_le_bytes([control(7), control(8)])) % (candidate.len() + 1);
    compare_decode(&mut native, &mut oracle, &candidate[..cut], params);
    compare_decode(&mut native, &mut oracle, candidate, params);
}

#[cfg(feature = "oracle")]
fn decode_options(settings: u8) -> (ReducePolicy, DecodeParams<'static>) {
    let policy = if settings & 1 == 0 {
        ReducePolicy::Exact
    } else {
        ReducePolicy::Clamp
    };
    let params = DecodeParams {
        strict: settings & 2 != 0,
        max_layers: u32::from(settings >> 2 & 7),
        ..DecodeParams::default()
    };
    (policy, params)
}

#[cfg(feature = "oracle")]
fn differential_codecs(
    format: Format,
    reduce: u32,
    reduce_policy: ReducePolicy,
    params: DecodeParams<'_>,
    limits: Limits,
) -> (Decoder, Codec) {
    let native = Decoder::new(DecoderConfig {
        format,
        reduce,
        reduce_policy,
        limits,
        ..DecoderConfig::default()
    });
    let oracle = Codec::with_options(
        match format {
            Format::J2k => OracleFormat::J2k,
            Format::Jp2 => OracleFormat::Jp2,
        },
        reduce,
        match reduce_policy {
            ReducePolicy::Exact => OracleReduce::Exact,
            ReducePolicy::Clamp => OracleReduce::Clamp,
        },
        OracleLimits {
            max_tiles: limits.max_tiles,
            max_components: limits.max_components,
            max_layers: limits.max_layers,
            max_decoded_side: limits.max_decoded_side,
            max_memory: limits.max_memory,
        },
        params.max_layers,
        params.strict,
    )
    .expect("the C++ oracle decoder can be configured");
    (native, oracle)
}

#[cfg(feature = "oracle")]
fn compare_decode(native: &mut Decoder, oracle: &mut Codec, data: &[u8], params: DecodeParams<'_>) -> i32 {
    let (status, samples) = oracle.decode_i32(data);
    match native.decode(data, params) {
        Ok(image) => {
            assert_eq!(
                i32::from(image.truncated),
                status,
                "decode status for {} bytes",
                data.len()
            );
            let planes = image.components.iter().map(|component| component.samples());
            assert_eq!(planes.clone().map(<[i32]>::len).sum::<usize>(), samples.len());
            for (index, (&ours, &theirs)) in planes.flatten().zip(&samples).enumerate() {
                assert_eq!(ours, theirs, "sample {index} for {} bytes", data.len());
            }
        }
        Err(error) => assert_eq!(error.kind() as i32, status, "{error} for {} bytes", data.len()),
    }
    status
}

#[cfg(feature = "oracle")]
fn compare_header(native: &mut Decoder, oracle: &mut Codec, data: &[u8]) -> bool {
    let expected = oracle.read_header(data);
    match native.read_header(data) {
        Ok(header) => {
            let expected = expected.expect("C++ accepts the header");
            assert_eq!([header.x0, header.y0, header.x1, header.y1], expected.bounds);
            assert_eq!(header.min_resolutions, expected.min_resolutions);
            assert_eq!(header.components.len(), expected.components.len());
            compare_dimensions(native, expected);
            true
        }
        Err(error) => {
            assert_eq!(error.kind() as i32, expected.expect_err("C++ refuses the header"));
            assert_eq!(
                native.decoded_dimensions().unwrap_err().kind(),
                neo_j2k::ErrorKind::State
            );
            false
        }
    }
}

#[cfg(feature = "oracle")]
fn compare_dimensions(native: &mut Decoder, expected: Header) {
    match native.decoded_dimensions() {
        Ok(actual) => {
            let expected = expected.dimensions.expect("C++ accepts the dimensions");
            assert_eq!(actual.len(), expected.len());
            for (a, b) in actual.iter().zip(expected) {
                assert_eq!((a.x0, a.y0, a.width, a.height), (b.x0, b.y0, b.w, b.h));
                assert_eq!((a.dx, a.dy, a.precision, a.reduce), (b.dx, b.dy, b.prec, b.factor));
                assert_eq!(u32::from(a.signed), b.sgnd);
            }
        }
        Err(error) => assert_eq!(
            error.kind() as i32,
            expected.dimensions.expect_err("C++ refuses the dimensions")
        ),
    }
}

#[cfg(feature = "oracle")]
pub fn api_sequence(data: &[u8]) {
    let Some(([selector, settings], commands)) = data.split_first_chunk::<2>() else {
        return;
    };
    let seed = usize::from(*selector) % DIFFERENTIAL_SEEDS.len();
    let (policy, params) = decode_options(*settings);
    // The oracle's setter changes the memory limit and leaves the other limits unrestricted.
    let mut limits = Limits {
        max_memory: LIMITS.max_memory,
        ..Limits::default()
    };
    let (mut native, mut oracle) = differential_codecs(Format::J2k, 0, policy, params, limits);
    let mut header_input = None;
    for &command in commands.iter().take(16) {
        let fixture = DIFFERENTIAL_SEEDS[(seed + usize::from(command >> 7)) % DIFFERENTIAL_SEEDS.len()];
        let input = match command >> 5 & 3 {
            0 => fixture,
            1 => &fixture[..2],
            2 => &fixture[..fixture.len() - 2],
            _ => &fixture[..fixture.len() / 2],
        };
        match command & 3 {
            0 => header_input = compare_header(&mut native, &mut oracle, input).then_some(input),
            1 => {
                let reduce = u32::from(command >> 2 & 7);
                if let Some(data) = header_input {
                    // The oracle helper reads the header before applying the factor. Match that
                    // state transition, including resetting cached component geometry.
                    native.read_header(data).expect("Rust retains the header");
                    let result = native.set_reduce(reduce);
                    let expected = oracle
                        .read_header_then_reduce(data, reduce)
                        .expect("C++ retains the header");
                    assert_eq!(
                        result.as_ref().err().map_or(0, |error| error.kind() as i32),
                        expected.reduce_status.unwrap()
                    );
                    compare_dimensions(&mut native, expected);
                } else {
                    let result = native.set_reduce(reduce);
                    assert_eq!(result.unwrap_err().kind() as i32, oracle.set_reduce(reduce));
                }
            }
            2 => {
                header_input = compare_header(&mut native, &mut oracle, input).then_some(input);
                compare_decode(&mut native, &mut oracle, input, params);
            }
            _ => {
                let next_memory = if command & 4 == 0 { 1 } else { LIMITS.max_memory };
                if limits.max_memory != next_memory {
                    limits.max_memory = next_memory;
                    assert!(oracle.set_max_memory(next_memory));
                    native.set_limits(limits);
                    if let Some(data) = header_input {
                        header_input = compare_header(&mut native, &mut oracle, data).then_some(data);
                    }
                }
            }
        }
    }
}

#[cfg(all(test, feature = "oracle"))]
mod tests {
    use super::*;

    #[test]
    fn decoder_sequence_seeds_cover_failure_and_recovery() {
        let (policy, params) = decode_options(0);
        let (mut native, mut oracle) = differential_codecs(Format::J2k, 0, policy, params, LIMITS);
        let input = DIFFERENTIAL_SEEDS[0];
        for (data, status) in [
            (input, 0),
            (&input[..input.len() - 2], 1),
            (&input[..2], neo_j2k::ErrorKind::NeedData as i32),
            (input, 0),
        ] {
            assert_eq!(compare_decode(&mut native, &mut oracle, data, params), status);
        }
        for sequence in [
            &include_bytes!("../seeds/api_sequence/retries")[..],
            &include_bytes!("../seeds/api_sequence/options")[..],
            &[0, 0, 0, 3, 32][..],
        ] {
            api_sequence(sequence);
        }
    }

    #[test]
    fn differential_replays_j2k_and_jp2_seeds() {
        for seed in DIFFERENTIAL_SEEDS.into_iter().chain([
            &include_bytes!("../seeds/jp2/palette-3x5.jp2")[..],
            &include_bytes!("../seeds/jp2/rgb-64.jp2")[..],
        ]) {
            differential(seed);
        }
    }

    #[test]
    fn decoder_reuse_with_a_reduced_header_budget() {
        for selector in 0..7 {
            for cut in [0x20, 0x40, 0x60] {
                for first in [0, 1, 2, 3] {
                    let sequence = [selector, 0, first, 3, cut, 7, 0];
                    let result = std::panic::catch_unwind(|| api_sequence(&sequence));
                    assert!(result.is_ok(), "sequence {sequence:?}");
                }
            }
        }
        let mut state = 137_u32;
        for _ in 0..10_000 {
            let mut sequence = [0_u8; 18];
            for byte in &mut sequence {
                state ^= state << 13;
                state ^= state >> 17;
                state ^= state << 5;
                *byte = state.to_le_bytes()[0];
            }
            let result = std::panic::catch_unwind(|| api_sequence(&sequence));
            assert!(result.is_ok(), "sequence {sequence:?}");
        }
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
        if flags & 0x10 != 0 {
            let bottom_up = flags & 0x20 != 0;
            let row_bytes = stride + usize::from(*selector >> 4);
            let row_stride = isize::try_from(row_bytes).unwrap();
            let mut expected = vec![0xa5; row_bytes * height as usize];
            for (channel, component) in image.components.iter().enumerate() {
                for (row, values) in component.samples().chunks_exact(width as usize).enumerate() {
                    let y = if bottom_up { height as usize - 1 - row } else { row };
                    for (x, &sample) in values.iter().enumerate() {
                        expected[y * row_bytes + x * usize::from(components) + channel] = sample.to_le_bytes()[0];
                    }
                }
            }
            let planes = [0, 1, 2, 3].map(|component| OutputPlane {
                component,
                offset: (if bottom_up {
                    (height as usize - 1) * row_bytes
                } else {
                    0
                }) + component as usize,
                row_stride: if bottom_up { -row_stride } else { row_stride },
                sample_stride: usize::from(components),
            });
            let mut actual = vec![0xa5; expected.len()];
            let truncated = decoder
                .decode_into(
                    bytes,
                    DecodeParams::default(),
                    SampleType::U8,
                    &mut [DecodeTarget {
                        bytes: &mut actual,
                        planes: &planes[..usize::from(components)],
                    }],
                )
                .expect("the output buffer fits the decoded image");
            assert!(!truncated);
            assert_eq!(actual, expected, "packed pixels and padding");
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
        resolutions: params
            .resolutions
            .map_or_else(|| (view.width.min(view.height).ilog2() + 1).min(6), u32::from),
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

/// Every file of `fuzz/regressions` and the checked-in seeds, through the targets: a defect a
/// fuzzer found stays fixed. The inputs are named `<target>-<finding>.bin` after the C++ targets
/// they came from; the Rust targets read the same byte formats or take any bytes, so every input
/// goes through every target that takes it.
#[cfg(test)]
mod replay {
    use std::fs;
    use std::path::{Path, PathBuf};

    fn inputs(dir: &str) -> Vec<(String, Vec<u8>)> {
        let dir: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir);
        let mut files: Vec<_> = fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
            .map(|entry| entry.expect("directory entry").path())
            .filter(|path| path.is_file() && path.file_name().is_some_and(|n| n != "README.md"))
            .collect();
        files.sort();
        files
            .into_iter()
            .map(|path| {
                let data = fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
                (
                    path.file_name().expect("file name").to_string_lossy().into_owned(),
                    data,
                )
            })
            .collect()
    }

    #[test]
    fn regressions_are_not_empty_and_named_after_a_target() {
        let regressions = inputs("regressions");
        assert!(!regressions.is_empty());
        for (name, _) in regressions {
            assert!(
                ["header-", "decode-", "roundtrip-", "encode-"]
                    .iter()
                    .any(|p| name.starts_with(p)),
                "{name}: not named <target>-<finding>.bin"
            );
        }
    }

    #[test]
    fn regressions_and_seeds_replay_through_the_native_targets() {
        for dir in ["regressions", "seeds/jp2", "seeds/roundtrip"] {
            for (name, data) in inputs(dir) {
                eprintln!("header_decode {dir}/{name}");
                super::header_decode(&data);
            }
        }
        for dir in ["regressions", "seeds/roundtrip"] {
            for (name, data) in inputs(dir) {
                eprintln!("roundtrip {dir}/{name}");
                super::roundtrip(&data);
            }
        }
    }

    #[cfg(feature = "oracle")]
    #[test]
    fn regressions_and_seeds_replay_against_the_oracle() {
        for dir in ["regressions", "seeds/jp2"] {
            for (name, data) in inputs(dir) {
                eprintln!("differential {dir}/{name}");
                super::differential(&data);
            }
        }
        for dir in ["regressions", "seeds/encode"] {
            for (name, data) in inputs(dir) {
                eprintln!("encode_options {dir}/{name}");
                super::encode_options(&data);
            }
        }
        for (name, data) in inputs("seeds/api_sequence") {
            eprintln!("api_sequence {name}");
            super::api_sequence(&data);
        }
    }
}
