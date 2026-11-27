// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//! Single-thread, fresh-codec API latency, identical inputs and owned/caller-owned outputs.
#![forbid(unsafe_code)]
use neo_j2k::{
    DecodeParams, Decoder, DecoderConfig, EncodeParams, Encoder, I32Plane, ImageSamples, ImageView, ReducePolicy,
};
use neo_j2k_oracle::{self as oracle, openjpeg};
use std::{hint::black_box, path::PathBuf, time::Instant};

const ROUNDS: usize = 12;
const WARM: usize = 32;
const ORDERS: [[usize; 3]; 6] = [[0, 1, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0], [1, 0, 2], [0, 2, 1]];
const NAMES: [&str; 3] = ["neo-j2k", "neoj2k-rust", "openjpeg"];

fn measure(label: &str, batch: usize, shift: usize, calls: &mut [&mut dyn FnMut() -> usize; 3]) {
    let mut times: [Vec<f64>; 3] = std::array::from_fn(|_| Vec::new());
    for round in 0..ROUNDS {
        for actor in ORDERS[(round + shift) % ORDERS.len()] {
            for _ in 0..WARM {
                black_box(calls[actor]());
            }
            let start = Instant::now();
            for _ in 0..batch {
                black_box(calls[actor]());
            }
            let us = start.elapsed().as_secs_f64() * 1e6 / batch as f64;
            times[actor].push(us);
            println!(
                "{{\"type\":\"round\",\"case\":\"{label}\",\"codec\":\"{}\",\"round\":{round},\"us\":{us:.6}}}",
                NAMES[actor]
            );
        }
    }
    for (actor, values) in times.iter_mut().enumerate() {
        values.sort_by(f64::total_cmp);
        let median = (values[ROUNDS / 2 - 1] + values[ROUNDS / 2]) / 2.0;
        println!(
            "{{\"type\":\"result\",\"case\":\"{label}\",\"codec\":\"{}\",\"us\":{median:.6}}}",
            NAMES[actor]
        );
    }
}

fn rust_decode(data: &[u8], sizes: &[usize], reduce: u32, strict: bool, output: &mut [i32]) -> usize {
    let mut codec = Decoder::new(DecoderConfig {
        reduce,
        reduce_policy: ReducePolicy::Clamp,
        ..DecoderConfig::default()
    });
    codec.read_header(data).expect("Rust header");
    let mut tail = output;
    let mut planes = Vec::with_capacity(sizes.len());
    for (component, &size) in sizes.iter().enumerate() {
        let (samples, rest) = std::mem::take(&mut tail).split_at_mut(size);
        tail = rest;
        planes.push(I32Plane {
            component: component as u32,
            samples,
        });
    }
    codec
        .decode_into_i32(
            black_box(data),
            DecodeParams {
                strict,
                ..DecodeParams::default()
            },
            &mut planes,
        )
        .expect("Rust decode");
    black_box(planes.as_slice());
    sizes.iter().sum()
}
fn cpp_decode(data: &[u8], reduce: u32, strict: bool, output: &mut [i32]) -> usize {
    let mut codec = oracle::Codec::with_options(
        oracle::Format::J2k,
        reduce,
        oracle::ReducePolicy::Clamp,
        oracle::Limits::default(),
        0,
        strict,
    )
    .expect("C++ codec");
    let mut prepared = codec.prepare_i32(data);
    let (status, count) = prepared.decode_into(output);
    assert!((0..=1).contains(&status), "C++ status {status}");
    assert_eq!(count as usize, output.len(), "C++ descriptor capacity");
    count as usize
}
fn opj_decode(data: &[u8], reduce: u32, strict: bool, output: &mut [i32]) -> usize {
    let mut codec = openjpeg::Decoder::with_strict(reduce, 1, strict).expect("OpenJPEG wrapper");
    let count = codec.decode_into(data, output).expect("OpenJPEG decode");
    assert_eq!(count as usize, output.len());
    count as usize
}
fn fixture(name: &str) -> Vec<u8> {
    if name == "generated_1024_rgb_97.j2k" {
        let samples = pixels(1024);
        let view = ImageView {
            width: 1024,
            height: 1024,
            flip_vertical: false,
            samples: ImageSamples::Interleaved {
                samples: &samples,
                components: 3,
                row_stride: 3072,
            },
        };
        let mut params = EncodeParams::default();
        params.reversible = false;
        params.mct = true;
        params.resolutions = Some(6);
        let data = Encoder::new().encode(&view, &params).unwrap().to_vec();
        let folder = PathBuf::from(std::env::var_os("NJ2K_THREE_WAY_GENERATED").expect("generated fixture directory"));
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join(name), &data).unwrap();
        return data;
    }
    let corpus = PathBuf::from(std::env::var_os("NJ2K_THREE_WAY_CORPUS").expect("corpus directory"));
    std::fs::read(corpus.join(name)).expect("fixture")
}
fn decode(name: &str, reduce: u32, strict: bool, cut: Option<usize>, shift: usize) {
    let mut data = fixture(name);
    if let Some(bytes) = cut {
        assert!(data.len() > bytes);
        data.truncate(bytes);
    }
    let mut header = Decoder::new(DecoderConfig {
        reduce,
        reduce_policy: ReducePolicy::Clamp,
        ..DecoderConfig::default()
    });
    header.read_header(&data).unwrap();
    let sizes: Vec<usize> = header
        .decoded_dimensions()
        .unwrap()
        .iter()
        .map(|c| c.width as usize * c.height as usize)
        .collect();
    let total: usize = sizes.iter().sum();
    drop(header);
    let mut buffers: [Vec<i32>; 3] = std::array::from_fn(|_| vec![i32::MIN; total]);
    buffers.rotate_left(shift % 3);
    let [mut cpp, mut rust, mut opj] = buffers;
    cpp_decode(&data, reduce, strict, &mut cpp);
    rust_decode(&data, &sizes, reduce, strict, &mut rust);
    opj_decode(&data, reduce, strict, &mut opj);
    assert_eq!(cpp, rust, "C++/Rust samples");
    assert_eq!(cpp, opj, "C++/OpenJPEG samples");
    let expected = cpp.clone();
    let label = format!("decode/{name}/reduce{reduce}/strict{strict}/cut{}", cut.unwrap_or(0));
    {
        let mut c = || cpp_decode(&data, reduce, strict, &mut cpp);
        let mut r = || rust_decode(&data, &sizes, reduce, strict, &mut rust);
        let mut o = || opj_decode(&data, reduce, strict, &mut opj);
        measure(
            &label,
            if total > 1_000_000 { 1 } else { 8 },
            shift,
            &mut [&mut c, &mut r, &mut o],
        );
    }
    // Poison and repeat outside timing: ensure every required sample is actually written.
    cpp.fill(i32::MIN);
    rust.fill(i32::MIN);
    opj.fill(i32::MIN);
    cpp_decode(&data, reduce, strict, &mut cpp);
    rust_decode(&data, &sizes, reduce, strict, &mut rust);
    opj_decode(&data, reduce, strict, &mut opj);
    assert_eq!(cpp, expected);
    assert_eq!(rust, expected);
    assert_eq!(opj, expected);
    println!("{{\"type\":\"check\",\"case\":\"{label}\",\"samples\":{total},\"equal\":true}}");
}
fn pixels(side: usize) -> Vec<u8> {
    (0..side * side * 3)
        .map(|i| {
            let x = i % (side * 3) / 3;
            let y = i / (side * 3);
            let c = i % 3;
            (3 * x + 5 * y + 13 * (x ^ y) + 23 * c) as u8
        })
        .collect()
}

fn check_strict_partial_rejection() {
    let data = fixture("generated_1024_rgb_97.j2k");
    let prefix = &data[..32768];
    let mut rust = Decoder::new(DecoderConfig {
        reduce: 5,
        reduce_policy: ReducePolicy::Clamp,
        ..DecoderConfig::default()
    });
    assert!(
        rust.decode(
            prefix,
            DecodeParams {
                strict: true,
                ..DecodeParams::default()
            }
        )
        .is_err()
    );
    let mut cpp = oracle::Codec::with_options(
        oracle::Format::J2k,
        5,
        oracle::ReducePolicy::Clamp,
        oracle::Limits::default(),
        0,
        true,
    )
    .expect("C++ codec");
    assert!(cpp.decode_i32(prefix).0 < 0);
    let mut opj = openjpeg::Decoder::with_strict(5, 1, true).expect("OpenJPEG wrapper");
    assert!(opj.decode(prefix).is_err());
    println!("{{\"type\":\"strict-cut-check\",\"cut_bytes\":32768,\"reduce\":5,\"all_rejected\":true}}");
}
fn encode(side: usize, reversible: bool, shift: usize) {
    let pixels = pixels(side);
    let view = ImageView {
        width: side as u32,
        height: side as u32,
        flip_vertical: false,
        samples: ImageSamples::Interleaved {
            samples: &pixels,
            components: 3,
            row_stride: side * 3,
        },
    };
    let mut params = EncodeParams::default();
    params.reversible = reversible;
    params.mct = true;
    params.resolutions = Some(6);
    let cpp = oracle::EncodeRequest {
        pixels: &pixels,
        row_stride: side * 3,
        width: side as u32,
        height: side as u32,
        components: 3,
        flip_vertical: false,
        reversible,
        resolutions: 6,
        mct: true,
        max_codestream_size: 0,
        layer_ratios: &[],
        comment: None,
    };
    let opj = openjpeg::EncodeRequest {
        pixels: &pixels,
        row_stride: side * 3,
        width: side as u32,
        height: side as u32,
        components: 3,
        reversible,
        resolutions: 6,
        mct: true,
    };
    let expected = Encoder::new().encode(&view, &params).expect("Rust encode").to_vec();
    assert_eq!(oracle::encode_u8(&cpp).expect("C++ encode"), expected);
    assert_eq!(openjpeg::encode_u8(&opj).expect("OpenJPEG encode"), expected);
    let label = format!("encode/{side}x{side}/{}", if reversible { "53" } else { "97" });
    {
        let mut c = || {
            let bytes = oracle::encode_u8(black_box(&cpp)).unwrap();
            black_box(&bytes);
            bytes.len()
        };
        let mut r = || {
            let bytes = Encoder::new().encode(black_box(&view), &params).unwrap().to_vec();
            black_box(&bytes);
            bytes.len()
        };
        let mut o = || {
            let bytes = openjpeg::encode_u8(black_box(&opj)).unwrap();
            black_box(&bytes);
            bytes.len()
        };
        measure(&label, 4, shift, &mut [&mut c, &mut r, &mut o]);
    }
    assert_eq!(Encoder::new().encode(&view, &params).unwrap(), expected.as_slice());
    assert_eq!(oracle::encode_u8(&cpp).unwrap(), expected);
    assert_eq!(openjpeg::encode_u8(&opj).unwrap(), expected);
    println!(
        "{{\"type\":\"check\",\"case\":\"{label}\",\"bytes\":{},\"equal\":true}}",
        expected.len()
    );
}
fn main() {
    assert_eq!(oracle::cpp_version(), (0, 1, 3));
    assert_eq!(openjpeg::version(), "2.5.4");
    let shift = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "0".into())
        .parse::<usize>()
        .unwrap();
    println!(
        "{{\"type\":\"protocol\",\"rounds\":{ROUNDS},\"prewarm\":{WARM},\"single_thread\":true,\"fresh_codecs\":true,\"shift\":{shift}}}"
    );
    for (file, reduce) in [
        ("sl_32_rgba_53.j2k", 0),
        ("sl_64_rgb_97.j2k", 0),
        ("sl_128_rgba_97.j2k", 0),
        ("sl_64_rgb_97.j2k", 2),
        ("large_1024_rgb_97.j2k", 0),
        ("large_4096_g_97.j2k", 0),
        ("large_2048x64_rgba_53.j2k", 0),
    ] {
        for strict in [false, true] {
            decode(file, reduce, strict, None, shift);
        }
    }
    check_strict_partial_rejection();
    decode("generated_1024_rgb_97.j2k", 5, false, Some(32768), shift);
    for side in [32, 64, 128] {
        for reversible in [true, false] {
            encode(side, reversible, shift);
        }
    }
}
