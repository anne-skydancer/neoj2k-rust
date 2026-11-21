// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use neo_j2k::{DecodeParams, DecodedImage, Decoder, DecoderConfig, Executor};
use neo_j2k_testkit::files::corpus_dir;

struct Host {
    width: usize,
    queries: AtomicUsize,
    calls: AtomicUsize,
    background: AtomicUsize,
}

impl Host {
    fn new(width: usize) -> Self {
        Self {
            width,
            queries: AtomicUsize::new(0),
            calls: AtomicUsize::new(0),
            background: AtomicUsize::new(0),
        }
    }
}

impl Executor for Host {
    fn concurrency(&self) -> usize {
        self.queries.fetch_add(1, Relaxed);
        self.width
    }

    fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync)) {
        assert!((2..=self.width).contains(&count));
        self.calls.fetch_add(1, Relaxed);
        std::thread::scope(|scope| {
            for index in (1..count).rev() {
                scope.spawn(move || {
                    self.background.fetch_add(1, Relaxed);
                    job(index);
                });
            }
            job(0);
        });
    }
}

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(corpus_dir().join(name)).unwrap()
}

fn assert_image_eq(actual: &DecodedImage<'_>, expected: &DecodedImage<'_>) {
    assert_eq!(actual.truncated, expected.truncated);
    assert_eq!(actual.components.len(), expected.components.len());
    for (actual, expected) in actual.components.iter().zip(expected.components) {
        assert_eq!(actual.info(), expected.info());
        assert_eq!(actual.resolution(), expected.resolution());
        assert_eq!(actual.alpha(), expected.alpha());
        assert!(
            actual.samples() == expected.samples(),
            "a scheduled component changed samples"
        );
    }
}

#[test]
fn small_and_reduced_tiles_do_not_even_query_the_host() {
    let host = Host::new(4);
    let params = DecodeParams {
        executor: Some(&host),
        ..DecodeParams::default()
    };
    for (name, reduce) in [
        ("sl_128_rgba_97.j2k", 0),
        ("large_2048x64_rgba_53.j2k", 0),
        ("large_1024_rgb_97.j2k", 2),
    ] {
        let data = fixture(name);
        let config = DecoderConfig {
            reduce,
            ..DecoderConfig::default()
        };
        let mut serial = Decoder::new(config);
        let mut scheduled = Decoder::new(config);
        let expected = serial.decode(&data, DecodeParams::default()).unwrap();
        assert_image_eq(&scheduled.decode(&data, params).unwrap(), &expected);
    }
    assert_eq!(host.queries.load(Relaxed), 0);
    assert_eq!(host.calls.load(Relaxed), 0);
}

#[test]
fn large_images_match_serial_across_reuse_and_selected_components() {
    let data = fixture("large_1024_rgb_97.j2k");
    let config = DecoderConfig {
        capture_diagnostics: true,
        ..DecoderConfig::default()
    };
    let mut serial = Decoder::new(config);
    let mut scheduled = Decoder::new(config);
    let host = Host::new(4);
    for components in [None, None, Some(&[1][..]), Some(&[2, 0][..])] {
        let params = DecodeParams {
            components,
            ..DecodeParams::default()
        };
        let prior = host.calls.load(Relaxed);
        let expected = serial.decode(&data, params).unwrap();
        let threaded = scheduled
            .decode(
                &data,
                DecodeParams {
                    executor: Some(&host),
                    ..params
                },
            )
            .unwrap();
        assert_image_eq(&threaded, &expected);
        assert_eq!(scheduled.diagnostics(), serial.diagnostics());
        assert!(
            host.calls.load(Relaxed) > prior,
            "large output should use row jobs even for one component"
        );
    }
    assert!(host.background.load(Relaxed) > 0);

    let host = Host::new(1);
    let expected = serial.decode(&data, DecodeParams::default()).unwrap();
    let params = DecodeParams {
        executor: Some(&host),
        ..DecodeParams::default()
    };
    assert_image_eq(&scheduled.decode(&data, params).unwrap(), &expected);
    assert_eq!(host.calls.load(Relaxed), 0);
}
