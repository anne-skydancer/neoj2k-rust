// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(feature = "encode")]

use neo_j2k::{EncodeParams, Encoder, Executor, ImageSamples, ImageView};
use neo_j2k_testkit::pool::Pool;

struct Host(Pool);

impl Executor for Host {
    fn concurrency(&self) -> usize {
        self.0.concurrency() as usize
    }

    fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync)) {
        self.0
            .parallel_for(u32::try_from(count).unwrap(), &|index| job(index as usize));
    }
}

fn pixels(side: usize) -> Vec<u8> {
    let stride = side * 3;
    (0..side * stride)
        .map(|index| {
            let x = (index % stride) / 3;
            let y = index / stride;
            let component = index % 3;
            (3 * x + 5 * y + 13 * (x ^ y) + 23 * component).to_le_bytes()[0]
        })
        .collect()
}

fn view(samples: &[u8], side: usize) -> ImageView<'_> {
    let side = u32::try_from(side).expect("the test image side fits u32");
    ImageView {
        width: side,
        height: side,
        flip_vertical: false,
        samples: ImageSamples::Interleaved {
            samples,
            components: 3,
            row_stride: usize::try_from(side).expect("u32 fits usize") * 3,
        },
    }
}

#[test]
fn small_encode_stays_serial() {
    let samples = pixels(128);
    let image = view(&samples, 128);
    let expected = Encoder::new()
        .encode(&image, &EncodeParams::default())
        .unwrap()
        .to_vec();
    let host = Host(Pool::new(7));
    let mut params = EncodeParams::default();
    params.executor = Some(&host);
    let actual = Encoder::new().encode(&image, &params).unwrap().to_vec();
    assert_eq!(actual, expected);
    assert_eq!(host.0.batches(), 0, "128x128 must not query the executor");
}

#[test]
fn threaded_encode_is_byte_identical_at_every_width() {
    let samples = pixels(256);
    let image = view(&samples, 256);
    for reversible in [true, false] {
        let mut params = EncodeParams::default();
        params.reversible = reversible;
        params.mct = true;
        let expected = Encoder::new().encode(&image, &params).unwrap().to_vec();
        for concurrency in [2, 4, 8, 16, 64] {
            let host = Host(Pool::new(concurrency - 1));
            let mut threaded = EncodeParams::default();
            threaded.reversible = reversible;
            threaded.mct = true;
            threaded.executor = Some(&host);
            let actual = Encoder::new().encode(&image, &threaded).unwrap().to_vec();
            assert_eq!(actual, expected, "concurrency {concurrency}, reversible {reversible}");
            assert!(
                host.0.batches() > 0,
                "concurrency {concurrency} did not use the executor"
            );
        }
    }
}
