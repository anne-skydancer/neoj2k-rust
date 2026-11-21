// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use std::sync::Barrier;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use neo_j2k::{DecodeParams, DecodedImage, Decoder, DecoderConfig, Executor};
use neo_j2k_testkit::files::corpus_dir;
use neo_j2k_testkit::pool::Pool;

struct Host {
    pool: Pool,
    queries: AtomicUsize,
    background: AtomicUsize,
}

impl Host {
    fn new(width: u32) -> Self {
        Self {
            pool: Pool::new(width - 1),
            queries: AtomicUsize::new(0),
            background: AtomicUsize::new(0),
        }
    }
}

impl Executor for Host {
    fn concurrency(&self) -> usize {
        self.queries.fetch_add(1, Relaxed);
        self.pool.concurrency() as usize
    }

    fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync)) {
        assert!(count > 1 && self.pool.concurrency() > 1);
        let caller = std::thread::current().id();
        let started = Barrier::new(2);
        self.pool.parallel_for(u32::try_from(count).unwrap(), &|index| {
            // Make the worker-coverage assertion deterministic even if the caller claims job zero.
            if index < 2 {
                started.wait();
            }
            if std::thread::current().id() != caller {
                self.background.fetch_add(1, Relaxed);
            }
            job(index as usize);
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
    assert_eq!(host.pool.batches(), 0);
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
        let prior = host.pool.batches();
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
        let minimum = if components.is_some_and(|list| list.len() == 1) {
            4
        } else {
            2
        };
        assert!(
            host.pool.batches() - prior >= minimum,
            "expected the large tile to schedule reconstruction and output"
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
    assert_eq!(host.pool.batches(), 0);
}

#[test]
fn large_tiles_preserve_samples_at_every_supported_job_width() {
    let data = fixture("large_1024_rgb_97.j2k");
    let mut serial = Decoder::new(DecoderConfig::default());
    let mut scheduled = Decoder::new(DecoderConfig::default());
    for width in [2, 4, 8, 16, 64] {
        let host = Host::new(width);
        for components in [None, Some(&[1][..])] {
            let params = DecodeParams {
                components,
                ..DecodeParams::default()
            };
            let before = host.pool.batches();
            assert_image_eq(
                &scheduled
                    .decode(
                        &data,
                        DecodeParams {
                            executor: Some(&host),
                            ..params
                        },
                    )
                    .unwrap(),
                &serial.decode(&data, params).unwrap(),
            );
            assert!(host.pool.batches() - before >= 4);
        }
    }
}

#[test]
fn parallel_tier1_uses_serial_diagnostics_for_a_refused_code_block() {
    let mut data = fixture("large_1024_rgb_97.j2k");
    let mut sot = 2;
    while data[sot..sot + 2] != [0xff, 0x90] {
        let length = u16::from_be_bytes(data[sot + 2..sot + 4].try_into().unwrap());
        sot += 2 + usize::from(length);
    }
    // A Maxshift ROI on component 1 forces Tier-1 to reject its coded bit-planes.
    data.splice(sot..sot, [0xff, 0x5e, 0, 5, 1, 0, 31]);
    let config = DecoderConfig {
        capture_diagnostics: true,
        ..DecoderConfig::default()
    };
    let params = DecodeParams {
        components: Some(&[1]),
        ..DecodeParams::default()
    };
    let mut serial = Decoder::new(config);
    let expected = serial.decode(&data, params).unwrap_err();
    assert!(format!("{:?}", serial.diagnostics()).contains("unsupported bpno_plus_one"));

    let mut scheduled = Decoder::new(config);
    let host = Host::new(4);
    let actual = scheduled
        .decode(
            &data,
            DecodeParams {
                executor: Some(&host),
                ..params
            },
        )
        .unwrap_err();
    assert_eq!(actual.kind(), expected.kind());
    assert_eq!(actual.message(), expected.message());
    assert_eq!(scheduled.diagnostics(), serial.diagnostics());
    assert_eq!(host.pool.batches(), 1);
}
