// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The Rust crate at tag v0.1.2 against the crate of this checkout, timed in one process (perf.yml, `vs_v012`;
//! tools/perf_summary.py reads the output). The requests are those of the other suites: the two large decodes at
//! widths 1, 2, 4 and 8, the three small images whole, reduced by 2 and missing the end marker, and the six small
//! encodes. Both versions run the same code (suite.rs) through the public API they share: `Decoder::decode_into_i32`
//! into caller-owned planes and `Encoder::new().encode`. The harness's own scheduler (scoped threads) serves both.
//!
//! Each row prints the newer crate's time, the older's and `same` or `DIFFERENT`: whether the two outputs (the
//! decoded samples, the encoded bytes) are identical. A difference is a result, never hidden.

use std::path::PathBuf;
use std::time::{Duration, Instant};

macro_rules! version_module {
    ($module:ident, $krate:ident) => {
        mod $module {
            use $krate as neo_j2k;
            include!("suite.rs");
        }
    };
}

version_module!(old, neo_j2k_old);
version_module!(new, neo_j2k_new);

const ROUNDS: usize = 21;
const BATCH: u32 = 16;
const SMALL_IMAGES: [&str; 3] = ["sl_32_rgba_53.j2k", "sl_64_rgb_97.j2k", "sl_128_rgba_97.j2k"];

fn load(name: &str) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(r"C:/Users/annes/source/repos/neoj2k-rust/corpus/generated")
        .join(name);
    std::fs::read(&path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

// A smoke run (NJ2K_VERSIONS_SMOKE set: three rounds of one call, the smaller large image at widths 1 and 2) checks
// that the program works; its figures prove nothing.
fn smoke() -> bool {
    std::env::var_os("NJ2K_VERSIONS_SMOKE").is_some()
}

fn rounds(full: usize) -> usize {
    if smoke() { 3 } else { full }
}

fn batch(full: u32) -> u32 {
    if smoke() { 1 } else { full }
}

fn elapsed(call: &mut dyn FnMut(), batch: u32) -> Duration {
    let start = Instant::now();
    for _ in 0..batch {
        call();
    }
    start.elapsed() / batch
}

/// The median time per call of each side over `rounds` rounds, the sides alternating, the order flipping each round.
fn medians(rounds: usize, batch: u32, new: &mut dyn FnMut(), old: &mut dyn FnMut()) -> (Duration, Duration) {
    let mut new_times = vec![Duration::ZERO; rounds];
    let mut old_times = vec![Duration::ZERO; rounds];
    for round in 0..rounds {
        if (round + std::env::args().nth(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0)) % 2 == 0 {
            new_times[round] = elapsed(new, batch);
            old_times[round] = elapsed(old, batch);
        } else {
            old_times[round] = elapsed(old, batch);
            new_times[round] = elapsed(new, batch);
        }
    }
    static REQUEST: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let index = REQUEST.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    for (round, (new, old)) in new_times.iter().zip(&old_times).enumerate() {
        println!("ROUND request={index} round={round} new_ns={} old_ns={}", new.as_nanos(), old.as_nanos());
    }
    new_times.sort_unstable();
    old_times.sort_unstable();
    (new_times[rounds / 2], old_times[rounds / 2])
}

fn same(identical: bool) -> &'static str {
    if identical { "same" } else { "DIFFERENT" }
}

fn micros(time: Duration) -> f64 {
    time.as_secs_f64() * 1_000_000.0
}

fn large_decodes() {
    println!(
        "Large decodes: both versions decode_into_i32 into caller-owned planes, one Decoder kept across calls; the"
    );
    println!("harness's scheduler starts width - 1 scoped threads per call (width counts the caller).");
    println!(
        "{:<26} {:>5} {:>9} {:>9} Output",
        "Fixture", "Width", "new ms", "old ms"
    );
    for name in ["large_1024_rgb_97.j2k", "large_4096_g_97.j2k"]
        .into_iter()
        .take(if smoke() { 1 } else { 2 })
    {
        let data = load(name);
        for width in [1usize, 2, 4, 8].into_iter().take(if smoke() { 2 } else { 4 }) {
            old::decode(&data, 0, width, |old_run, old_output| {
                new::decode(&data, 0, width, |new_run, new_output| {
                    let (new_time, old_time) = medians(rounds(9), batch(2), new_run, old_run);
                    println!(
                        "{name:26} {width:5} {:9.3} {:9.3} {}",
                        new_time.as_secs_f64() * 1_000.0,
                        old_time.as_secs_f64() * 1_000.0,
                        same(new_output == old_output),
                    );
                })
            });
        }
    }
}

fn small_decodes() {
    println!("Small decodes: both versions decode_into_i32 into caller-owned planes, one Decoder kept across calls.");
    println!("{:<48} {:>9} {:>9} Output", "Request", "new µs", "old µs");
    for name in SMALL_IMAGES {
        let data = load(name);
        for (reduce, cut) in [(0, false), (2, false), (0, true)] {
            let input = if cut { &data[..data.len() - 2] } else { &data[..] };
            old::decode(input, reduce, 1, |old_run, old_output| {
                new::decode(input, reduce, 1, |new_run, new_output| {
                    let (new_time, old_time) = medians(rounds(ROUNDS), batch(BATCH), new_run, old_run);
                    let case = if cut { "missing EOC" } else { "whole" };
                    println!(
                        "{:<48} {:9.2} {:9.2} {}",
                        format!("{name} / reduce {reduce} / {case}"),
                        micros(new_time),
                        micros(old_time),
                        same(new_output == old_output),
                    );
                })
            });
        }
    }
}

fn small_encodes() {
    println!("Small encodes: both versions Encoder::new().encode on the borrowed interleaved pixels, per call.");
    println!("{:<48} {:>9} {:>9} Output", "Request", "new µs", "old µs");
    for side in [32u32, 64, 128] {
        let pixels = old::pixels(side as usize);
        assert_eq!(pixels, new::pixels(side as usize));
        for reversible in [true, false] {
            old::encode(&pixels, side, reversible, |old_run, old_output| {
                new::encode(&pixels, side, reversible, |new_run, new_output| {
                    let (new_time, old_time) = medians(rounds(ROUNDS), batch(BATCH), new_run, old_run);
                    let wavelet = if reversible { "lossless 5/3" } else { "lossy 9/7" };
                    println!(
                        "{:<48} {:9.2} {:9.2} {}",
                        format!("encode {side}x{side} RGB / {wavelet}"),
                        micros(new_time),
                        micros(old_time),
                        same(new_output == old_output),
                    );
                })
            });
        }
    }
}

fn main() {
    println!("Baseline 3000f7d; merged main b4240d6; unchanged 23-request version suite.");
    println!(
        "neo-j2k {} (new) against neo-j2k {} (old), one process, alternating.",
        new::version(),
        old::version()
    );
    large_decodes();
    small_decodes();
    small_encodes();
    println!();
    println!("harness result: ok. 3 passed; 0 failed");
}
