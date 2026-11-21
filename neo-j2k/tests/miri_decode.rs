// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(miri)]

use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use neo_j2k::{DecodeParams, Decoder, DecoderConfig, ErrorKind, Executor, Limits, ReducePolicy};

#[derive(Default)]
struct TwoParticipants(AtomicUsize);

impl Executor for TwoParticipants {
    fn concurrency(&self) -> usize {
        2
    }

    fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync)) {
        assert!(count > 1);
        self.0.fetch_add(1, Relaxed);
        std::thread::scope(|scope| {
            scope.spawn(|| {
                for index in (1..count).step_by(2) {
                    job(index);
                }
            });
            for index in (0..count).step_by(2) {
                job(index);
            }
        });
    }
}

#[test]
fn tiny_tiles_preserve_exact_samples_through_scoped_workers() {
    let host = TwoParticipants::default();
    let mut serial = Decoder::new(DecoderConfig::default());
    let mut parallel = Decoder::new(DecoderConfig::default());
    for data in [
        &include_bytes!("../../corpus/generated/sl_32_rgba_53.j2k")[..],
        &include_bytes!("../../corpus/generated/sl_32_rgb_97.j2k")[..],
    ] {
        let before = host.0.load(Relaxed);
        let expected = serial.decode(data, DecodeParams::default()).unwrap();
        let actual = parallel
            .decode(
                data,
                DecodeParams {
                    executor: Some(&host),
                    ..DecodeParams::default()
                },
            )
            .unwrap();
        assert_eq!(actual.truncated, expected.truncated);
        assert_eq!(actual.components.len(), expected.components.len());
        for (actual, expected) in actual.components.iter().zip(expected.components) {
            assert_eq!(actual.info(), expected.info());
            assert_eq!(actual.samples(), expected.samples());
        }
        assert!(host.0.load(Relaxed) - before >= 4);
    }
}

#[test]
fn small_decodes_at_every_reduce_level_can_be_retried() {
    let inputs: [&[u8]; 4] = [
        include_bytes!("../../corpus/generated/size_1x1_g.j2k"),
        include_bytes!("../../corpus/generated/size_3x5_g.j2k"),
        include_bytes!("../../corpus/generated/size_7x9_rgb_r1.j2k"),
        include_bytes!("../../corpus/generated/size_16x16_rgb_r5.j2k"),
    ];
    for reduce in 0..=5 {
        let mut decoder = Decoder::new(DecoderConfig {
            reduce,
            reduce_policy: ReducePolicy::Clamp,
            limits: Limits::SECOND_LIFE,
            ..DecoderConfig::default()
        });
        for input in inputs {
            let image = decoder.decode(input, DecodeParams::default()).unwrap();
            assert!(!image.truncated);
            assert!(!image.components.is_empty());
            let expected: Vec<_> = image
                .components
                .iter()
                .map(|component| {
                    let info = component.info();
                    assert_eq!(
                        component.samples().len(),
                        usize::try_from(info.width * info.height).unwrap()
                    );
                    (info, component.samples().to_vec())
                })
                .collect();

            assert_eq!(
                decoder.decode(&input[..2], DecodeParams::default()).unwrap_err().kind(),
                ErrorKind::NeedData
            );
            assert_eq!(decoder.decoded_dimensions().unwrap_err().kind(), ErrorKind::State);
            let retried = decoder.decode(input, DecodeParams::default()).unwrap();
            assert!(!retried.truncated);
            assert_eq!(retried.components.len(), expected.len());
            for (component, (info, samples)) in retried.components.iter().zip(expected) {
                assert_eq!(component.info(), info);
                assert_eq!(component.samples(), samples);
            }
        }
    }
}
