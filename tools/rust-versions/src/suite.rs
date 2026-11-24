// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

// The requests of one version of the crate, through its public whole-image API only. This file is included once per
// version (main.rs), inside a module that has `neo_j2k` aliased to that version's crate.

use std::sync::atomic::{AtomicUsize, Ordering};

use neo_j2k::{
    DecodeParams, Decoder, DecoderConfig, EncodeParams, Encoder, Executor, I32Plane, ImageSamples, ImageView,
};

/// The host scheduler of the Rust API, built on scoped threads: every call starts `width - 1` helpers, which with
/// the caller claim job indices from one counter. (The crate creates no threads of its own, so the host does.)
struct Pool {
    width: usize,
}

impl Executor for Pool {
    fn concurrency(&self) -> usize {
        self.width
    }

    fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync)) {
        let next = AtomicUsize::new(0);
        let work = || {
            loop {
                let index = next.fetch_add(1, Ordering::Relaxed);
                if index >= count {
                    break;
                }
                job(index);
            }
        };
        let work = &work;
        let helpers = self.width.min(count).saturating_sub(1);
        std::thread::scope(|scope| {
            for _ in 0..helpers {
                scope.spawn(work);
            }
            work();
        });
    }
}

/// One decode request, set up and warmed: `body` gets the call to time and the samples one decode gives (planes one
/// after the other).
pub fn decode<R>(
    data: &[u8],
    reduce: u32,
    width: usize,
    body: impl FnOnce(&mut dyn FnMut(), Vec<i32>) -> R,
) -> R {
    let pool = Pool { width };
    let params = DecodeParams {
        executor: (width > 1).then_some(&pool as &dyn Executor),
        ..DecodeParams::default()
    };
    let mut decoder = Decoder::new(DecoderConfig {
        reduce,
        ..DecoderConfig::default()
    });
    decoder.set_retained_bytes(256 << 20);
    decoder.read_header(data).expect("the fixture has a header");
    let sizes: Vec<usize> = decoder
        .decoded_dimensions()
        .expect("the header was read")
        .iter()
        .map(|info| info.width as usize * info.height as usize)
        .collect();
    let mut buffer = vec![0i32; sizes.iter().sum()];
    let mut planes = Vec::with_capacity(sizes.len());
    let mut rest = buffer.as_mut_slice();
    for (component, size) in sizes.iter().enumerate() {
        let (samples, tail) = core::mem::take(&mut rest).split_at_mut(*size);
        rest = tail;
        planes.push(I32Plane {
            component: u32::try_from(component).expect("a component index fits in u32"),
            samples,
        });
    }
    // two decodes before anything is measured; a codestream without its end marker is a result, not an error
    for _ in 0..2 {
        decoder
            .decode_into_i32(data, params, &mut planes)
            .expect("the fixture decodes");
    }
    let output: Vec<i32> = planes.iter().flat_map(|plane| plane.samples.iter().copied()).collect();
    body(
        &mut || {
            decoder
                .decode_into_i32(std::hint::black_box(data), params, &mut planes)
                .expect("a warmed decode");
            std::hint::black_box(planes.as_slice());
        },
        output,
    )
}

/// The test image of the small encodes: interleaved RGB, `side` by `side`.
pub fn pixels(side: usize) -> Vec<u8> {
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

/// One encode request: `body` gets the call to time (a new encoder per call, as the small encode rows of the other
/// suites do) and the codestream one encode gives.
pub fn encode<R>(pixels: &[u8], side: u32, reversible: bool, body: impl FnOnce(&mut dyn FnMut(), Vec<u8>) -> R) -> R {
    let view = ImageView {
        width: side,
        height: side,
        flip_vertical: false,
        samples: ImageSamples::Interleaved {
            samples: pixels,
            components: 3,
            row_stride: side as usize * 3,
        },
    };
    let mut params = EncodeParams::default();
    params.reversible = reversible;
    params.mct = true;
    params.resolutions = Some(6);
    let output = Encoder::new()
        .encode(&view, &params)
        .expect("the encoder accepts the fixture")
        .to_vec();
    body(
        &mut || {
            std::hint::black_box(
                Encoder::new()
                    .encode(std::hint::black_box(&view), &params)
                    .expect("an encode"),
            );
        },
        output,
    )
}

/// The crate's release version.
pub fn version() -> &'static str {
    neo_j2k::version()
}
