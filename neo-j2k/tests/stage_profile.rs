// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Where the time of a small decode goes (doc/codec-parity-audit-2026-11-23.md, D3). Needs the off-by-default
//! feature `bench-internals`, which turns on the decoder's stage timers; run by `.github/workflows/stage-profile.yml`.
//!
//! The three small images are decoded many times at a reduce factor, and each call's wall time is split into the
//! stages the decoder passes through. The stage totals are process-wide, and these decodes run one at a time on the
//! calling thread (no executor), so the stages plus the unattributed rest add up to the call.

#![cfg(all(feature = "bench-internals", not(miri)))]

use std::hint::black_box;
use std::time::{Duration, Instant};

use neo_j2k::bench::{Stage, stage_reset, stage_totals};
use neo_j2k::{DecodeParams, DecodeTarget, Decoder, DecoderConfig, I32Plane, OutputPlane, SampleType};
use neo_j2k_testkit::files::corpus_dir;

const IMAGES: [&str; 3] = ["sl_64_rgb_97.j2k", "sl_128_rgba_97.j2k", "sl_32_rgba_53.j2k"];
const ROUNDS: usize = 31;
const CALLS: u32 = 200;

fn load(name: &str) -> Vec<u8> {
    std::fs::read(corpus_dir().join(name)).unwrap_or_else(|error| panic!("cannot read {name}: {error}"))
}

fn median<T: Ord + Copy>(mut values: Vec<T>) -> T {
    values.sort_unstable();
    values[values.len() / 2]
}

/// The cost of one timer read pair, which every stage scope pays (a start and a stop).
fn scope_cost() -> Duration {
    const N: u32 = 200_000;
    let mut best = Duration::MAX;
    for _ in 0..5 {
        let begin = Instant::now();
        for _ in 0..N {
            let started = Instant::now();
            black_box(started.elapsed());
        }
        best = best.min(begin.elapsed() / N);
    }
    best
}

enum Api {
    /// `decode_into` into one byte buffer of native-endian I32 planes, as the latency gate does.
    Bytes,
    /// `decode_into_i32` into one I32 slice per component.
    Typed,
}

fn profile(api: &Api, name: &str, reduce: u32) {
    let data = load(name);
    let mut decoder = Decoder::new(DecoderConfig {
        reduce,
        ..DecoderConfig::default()
    });
    let params = DecodeParams::default();
    let image = decoder.decode(&data, params).expect("the fixture decodes");
    let infos: Vec<_> = image.components.iter().map(|plane| plane.info()).collect();
    let counts: Vec<usize> = infos
        .iter()
        .map(|info| info.width as usize * info.height as usize)
        .collect();
    let mut bytes = vec![0u8; counts.iter().sum::<usize>() * 4];
    let mut offset = 0;
    let views: Vec<OutputPlane> = counts
        .iter()
        .zip(&infos)
        .enumerate()
        .map(|(component, (&count, info))| {
            let view = OutputPlane {
                component: u32::try_from(component).expect("few components"),
                offset,
                row_stride: isize::try_from(info.width as usize * 4).expect("small"),
                sample_stride: 4,
            };
            offset += count * 4;
            view
        })
        .collect();
    let mut planes: Vec<Vec<i32>> = counts.iter().map(|&count| vec![0; count]).collect();
    let mut call = |decoder: &mut Decoder| {
        match api {
            Api::Bytes => decoder
                .decode_into(
                    black_box(&data),
                    params,
                    SampleType::I32,
                    &mut [DecodeTarget {
                        bytes: &mut bytes,
                        planes: &views,
                    }],
                )
                .expect("decodes"),
            Api::Typed => {
                let mut targets: Vec<I32Plane<'_>> = planes
                    .iter_mut()
                    .enumerate()
                    .map(|(component, samples)| I32Plane {
                        component: u32::try_from(component).expect("few components"),
                        samples,
                    })
                    .collect();
                decoder
                    .decode_into_i32(black_box(&data), params, &mut targets)
                    .expect("decodes")
            }
        };
    };
    for _ in 0..CALLS {
        call(&mut decoder);
    }
    let mut totals = Vec::new();
    let mut stages: Vec<Vec<Duration>> = vec![Vec::new(); Stage::ALL.len()];
    for _ in 0..ROUNDS {
        stage_reset();
        let begin = Instant::now();
        for _ in 0..CALLS {
            call(&mut decoder);
        }
        let total = begin.elapsed() / CALLS;
        totals.push(total);
        for (list, spent) in stages.iter_mut().zip(stage_totals()) {
            list.push(spent / CALLS);
        }
    }
    let total = median(totals);
    let stages: Vec<Duration> = stages.into_iter().map(median).collect();
    let attributed: Duration = stages.iter().sum();
    let micros = |value: Duration| value.as_secs_f64() * 1e6;
    let percent = |value: Duration| 100.0 * value.as_secs_f64() / total.as_secs_f64();
    let label = match api {
        Api::Bytes => "decode_into (bytes)",
        Api::Typed => "decode_into_i32",
    };
    println!(
        "\n{name} / reduce {reduce} / {label}: {:.2} us per call (median of {ROUNDS} x {CALLS})",
        micros(total)
    );
    println!("  {:<26} {:>9} {:>8}", "stage", "us", "share");
    for (stage, &spent) in Stage::ALL.iter().zip(&stages) {
        println!("  {:<26} {:>9.2} {:>7.1}%", stage.name(), micros(spent), percent(spent));
    }
    let rest = total.saturating_sub(attributed);
    println!(
        "  {:<26} {:>9.2} {:>7.1}%",
        "rest (between stages)",
        micros(rest),
        percent(rest)
    );
}

#[test]
#[ignore = "run with --release --features bench-internals and --nocapture; a measurement, not a check"]
fn small_decode_stage_shares() {
    let cost = scope_cost();
    println!(
        "One stage scope (a start and a stop of the clock) costs {:.0} ns; a decode passes about 14 of them.",
        cost.as_secs_f64() * 1e9
    );
    for reduce in [2, 0] {
        for name in IMAGES {
            for api in [Api::Bytes, Api::Typed] {
                profile(&api, name, reduce);
            }
        }
    }
}
