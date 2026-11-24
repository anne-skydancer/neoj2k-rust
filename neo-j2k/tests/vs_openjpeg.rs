// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The native Rust codec timed against OpenJPEG 2.5.4 (the oracle at its pin), in one process, the two sides
//! alternating (tools/perf_summary.py reads the output; .github/workflows/perf.yml runs it with `vs_openjpeg`).
//! The same 23 requests as `small_image_gates.rs` times against the C++ neo-j2k: the large decodes at widths 1, 2, 4
//! and 8, the small decodes (whole, reduced by 2, missing its end marker) and the small encodes.
//!
//! The two sides do not use the same API, and the printed header says how:
//!
//! - decode, Rust: one `Decoder` kept across calls, `decode_into_i32` into caller-owned planes (no allocation per call);
//!   threads are the host executor of the Rust API (here the test harness's persistent pool, width counting the
//!   caller).
//! - decode, OpenJPEG: a codec and a memory stream made and destroyed in every call (OpenJPEG keeps none between
//!   decodes), the sample planes allocated by OpenJPEG in every call and not copied out of it; threads are OpenJPEG's
//!   own (`opj_codec_set_threads`, a pool of that many workers made by every call); no threads at width 1.
//! - encode, Rust: `Encoder::new().encode` on the borrowed pixels, per call; OpenJPEG: `opj_image_create`, the pixels
//!   copied into its planes, `opj_encode` into a growing memory buffer, per call.
//!
//! Every row also checks the two outputs against each other (the decoded samples, the encoded bytes) and prints
//! `same` or `DIFFERENT`: a difference is a result in the table, never hidden.

#![cfg(not(miri))]

use std::time::{Duration, Instant};

use neo_j2k::{DecodeParams, Decoder, DecoderConfig, Executor, I32Plane};
use neo_j2k_oracle::BenchmarkPool;
use neo_j2k_oracle::openjpeg::{self, Decoder as OpjDecoder};
use neo_j2k_testkit::files::corpus_dir;

const SMALL_IMAGES: [&str; 3] = ["sl_32_rgba_53.j2k", "sl_64_rgb_97.j2k", "sl_128_rgba_97.j2k"];
const ROUNDS: usize = 21;
const BATCH: u32 = 16;

fn load(name: &str) -> Vec<u8> {
    std::fs::read(corpus_dir().join(name)).unwrap_or_else(|error| panic!("cannot read {name}: {error}"))
}

fn decoder(reduce: u32) -> Decoder {
    Decoder::new(DecoderConfig {
        reduce,
        ..DecoderConfig::default()
    })
}

fn elapsed(mut call: impl FnMut(), batch: u32) -> Duration {
    let start = Instant::now();
    for _ in 0..batch {
        call();
    }
    start.elapsed() / batch
}

/// The median time per call of each side over `ROUNDS` rounds, the sides alternating, the order flipping each round.
fn medians<const ROUNDS: usize>(batch: u32, mut rust: impl FnMut(), mut opj: impl FnMut()) -> (Duration, Duration) {
    let mut rust_times = [Duration::ZERO; ROUNDS];
    let mut opj_times = [Duration::ZERO; ROUNDS];
    for round in 0..ROUNDS {
        if round.is_multiple_of(2) {
            rust_times[round] = elapsed(&mut rust, batch);
            opj_times[round] = elapsed(&mut opj, batch);
        } else {
            opj_times[round] = elapsed(&mut opj, batch);
            rust_times[round] = elapsed(&mut rust, batch);
        }
    }
    rust_times.sort_unstable();
    opj_times.sort_unstable();
    (rust_times[ROUNDS / 2], opj_times[ROUNDS / 2])
}

fn same(identical: bool) -> &'static str {
    if identical { "same" } else { "DIFFERENT" }
}

fn report(label: &str, rust: Duration, opj: Duration, identical: bool) {
    println!(
        "{label:48} {:9.2} {:9.2} {}",
        rust.as_secs_f64() * 1_000_000.0,
        opj.as_secs_f64() * 1_000_000.0,
        same(identical),
    );
}

/// The planes of a decoded image carved from one buffer, one after the other (the layout of OpenJPEG's copy).
fn planes_of<'a>(decoder: &mut Decoder, buffer: &'a mut [i32]) -> Vec<I32Plane<'a>> {
    let mut remaining = buffer;
    let planes: Vec<_> = decoder
        .decoded_dimensions()
        .expect("the header was read")
        .iter()
        .enumerate()
        .map(|(component, info)| {
            let (samples, rest) =
                core::mem::take(&mut remaining).split_at_mut(info.width as usize * info.height as usize);
            remaining = rest;
            I32Plane {
                component: u32::try_from(component).expect("a component index fits in u32"),
                samples,
            }
        })
        .collect();
    assert!(remaining.is_empty(), "the buffer holds exactly the planes");
    planes
}

/// The number of samples the image has once decoded (all planes), after the header read.
fn samples_needed(decoder: &mut Decoder) -> usize {
    decoder
        .decoded_dimensions()
        .expect("the header was read")
        .iter()
        .map(|info| info.width as usize * info.height as usize)
        .sum()
}

#[test]
#[ignore = "run with --release and --nocapture on a quiet machine"]
fn small_image_vs_openjpeg() {
    use std::hint::black_box;

    println!("Rust: one Decoder kept across calls, decode_into_i32 into caller-owned planes, no allocation per call.");
    println!(
        "OpenJPEG {} (the oracle): a codec and a memory stream made and destroyed in every call, the planes allocated \
         by OpenJPEG in every call and not copied out, no threads.",
        openjpeg::version()
    );
    println!("{:<48} {:>9} {:>11} Output", "Request", "Rust µs", "OpenJPEG µs");
    for name in SMALL_IMAGES {
        let data = load(name);
        for (reduce, cut) in [(0, false), (2, false), (0, true)] {
            let input = if cut { &data[..data.len() - 2] } else { &data };
            let mut rust = decoder(reduce);
            let mut opj = OpjDecoder::new(reduce, 0).expect("make the OpenJPEG decoder");
            rust.read_header(input).expect("the benchmark fixture has a header");
            let need = samples_needed(&mut rust);
            assert!(need > 0);
            let mut rust_output = vec![0; need];
            let mut opj_output = vec![0; need];
            let mut planes = planes_of(&mut rust, &mut rust_output);
            let params = DecodeParams::default();
            let truncated = rust
                .decode_into_i32(input, params, &mut planes)
                .expect("the Rust decode of the fixture");
            assert_eq!(truncated, cut, "{name}, reduce {reduce}");
            let count = opj
                .decode_into(input, &mut opj_output)
                .unwrap_or_else(|status| panic!("OpenJPEG status {status} on {name}, reduce {reduce}, cut {cut}"));
            assert_eq!(count, need as u64, "{name}, reduce {reduce}: the sample count");
            drop(planes);
            let identical = rust_output == opj_output;
            let mut planes = planes_of(&mut rust, &mut rust_output);
            let (rust_median, opj_median) = medians::<ROUNDS>(
                BATCH,
                || {
                    let truncated = rust
                        .decode_into_i32(black_box(input), params, &mut planes)
                        .expect("warmed Rust decode");
                    assert_eq!(truncated, cut);
                    black_box(planes.as_slice());
                },
                || {
                    black_box(opj.decode(black_box(input)).expect("OpenJPEG decode"));
                },
            );
            let case = if cut { "missing EOC" } else { "whole" };
            report(
                &format!("{name} / reduce {reduce} / {case}"),
                rust_median,
                opj_median,
                identical,
            );
        }
    }
}

struct SharedPool(BenchmarkPool);

impl Executor for SharedPool {
    fn concurrency(&self) -> usize {
        self.0.concurrency() as usize
    }

    fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync)) {
        self.0.parallel_for(
            u32::try_from(count).expect("the benchmark batch fits in u32"),
            &|index| {
                job(index as usize);
            },
        );
    }
}

#[test]
#[ignore = "run with --release and --nocapture on a quiet machine"]
fn large_image_vs_openjpeg() {
    use std::hint::black_box;

    const RETAINED_BYTES: u64 = 256 << 20;
    println!(
        "Rust: one Decoder kept across calls, decode_into_i32 into caller-owned planes; threads are the host executor \
         (the test harness's persistent pool, width counts the caller)."
    );
    println!(
        "OpenJPEG {} (the oracle): a codec and a memory stream made and destroyed in every call, the planes allocated \
         by OpenJPEG in every call and not copied out; width 1 has no threads, width N asks opj_codec_set_threads(N), \
         a pool of N workers made in every call.",
        openjpeg::version()
    );
    println!(
        "{:<26} {:>5} {:>9} {:>11} Output",
        "Fixture", "Width", "Rust ms", "OpenJPEG ms"
    );
    for name in ["large_1024_rgb_97.j2k", "large_4096_g_97.j2k"] {
        let data = load(name);
        for width in [1u32, 2, 4, 8] {
            let host = (width > 1).then(|| SharedPool(BenchmarkPool::new(width).expect("start the shared test pool")));
            let params = DecodeParams {
                executor: host.as_ref().map(|host| host as &dyn Executor),
                ..DecodeParams::default()
            };
            let mut rust = decoder(0);
            rust.set_retained_bytes(RETAINED_BYTES);
            let mut opj = OpjDecoder::new(0, width).expect("make the OpenJPEG decoder");
            rust.read_header(&data).expect("the benchmark fixture has a header");
            let need = samples_needed(&mut rust);
            assert!(need > 0);
            let mut rust_output = vec![0; need];
            let mut opj_output = vec![0; need];
            let mut planes = planes_of(&mut rust, &mut rust_output);
            // two warm-up decodes each, then the check (as the Rust-versus-C++ rows)
            for _ in 0..2 {
                assert!(!rust.decode_into_i32(&data, params, &mut planes).unwrap());
                opj.decode(&data).expect("OpenJPEG warm-up decode");
            }
            let count = opj.decode_into(&data, &mut opj_output).expect("OpenJPEG decode");
            assert_eq!(count, need as u64, "{name}, width {width}: the sample count");
            drop(planes);
            let identical = rust_output == opj_output;
            let mut planes = planes_of(&mut rust, &mut rust_output);
            let (rust_time, opj_time) = medians::<9>(
                2,
                || {
                    assert!(!rust.decode_into_i32(black_box(&data), params, &mut planes).unwrap());
                    black_box(planes.as_slice());
                },
                || {
                    black_box(opj.decode(black_box(&data)).expect("OpenJPEG decode"));
                },
            );
            println!(
                "{name:26} {width:5} {:9.3} {:11.3} {}",
                rust_time.as_secs_f64() * 1_000.0,
                opj_time.as_secs_f64() * 1_000.0,
                same(identical),
            );
        }
    }
}

#[cfg(feature = "encode")]
#[test]
#[ignore = "run with --release and --nocapture on a quiet machine"]
fn small_encoder_vs_openjpeg() {
    use std::hint::black_box;

    use neo_j2k::{EncodeParams, Encoder, ImageSamples, ImageView};
    use neo_j2k_oracle::openjpeg::EncodeRequest;

    println!("Rust: Encoder::new().encode on the borrowed interleaved pixels, one encoder per call.");
    println!(
        "OpenJPEG {} (the oracle): opj_image_create, the pixels copied into its int32 planes, opj_encode into a \
         growing memory buffer, a codec per call, one layer, no threads.",
        openjpeg::version()
    );
    println!("{:<48} {:>9} {:>11} Output", "Request", "Rust µs", "OpenJPEG µs");
    for side in [32usize, 64, 128] {
        let stride = side * 3;
        let pixels: Vec<u8> = (0..side * stride)
            .map(|index| {
                let x = (index % stride) / 3;
                let y = index / stride;
                let component = index % 3;
                (3 * x + 5 * y + 13 * (x ^ y) + 23 * component).to_le_bytes()[0]
            })
            .collect();
        let side = u32::try_from(side).expect("a small image side fits in u32");
        let view = ImageView {
            width: side,
            height: side,
            flip_vertical: false,
            samples: ImageSamples::Interleaved {
                samples: &pixels,
                components: 3,
                row_stride: stride,
            },
        };
        for reversible in [true, false] {
            let mut params = EncodeParams::default();
            params.reversible = reversible;
            params.mct = true;
            params.resolutions = Some(6);
            let request = EncodeRequest {
                pixels: &pixels,
                row_stride: stride,
                width: side,
                height: side,
                components: 3,
                reversible,
                resolutions: 6,
                mct: true,
            };
            let expected = openjpeg::encode_u8(&request).expect("OpenJPEG encodes the fixture");
            let identical = Encoder::new()
                .encode(&view, &params)
                .expect("the Rust encoder accepts the fixture")
                == expected.as_slice();
            let rust_call = || {
                black_box(Encoder::new().encode(black_box(&view), &params).expect("Rust encode"));
            };
            let opj_call = || {
                black_box(openjpeg::encode_u8_len(black_box(&request)).expect("OpenJPEG encode"));
            };
            let (rust_median, opj_median) = medians::<ROUNDS>(BATCH, rust_call, opj_call);
            let wavelet = if reversible { "lossless 5/3" } else { "lossy 9/7" };
            report(
                &format!("encode {side}x{side} RGB / {wavelet}"),
                rust_median,
                opj_median,
                identical,
            );
        }
    }
}
