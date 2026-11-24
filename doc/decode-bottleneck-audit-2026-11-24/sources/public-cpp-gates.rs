// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use packed as neo_j2k;

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::time::{Duration, Instant};

use neo_j2k::{
    DecodeParams, DecodeTarget, DecodedImage, Decoder, DecoderConfig, Error, Executor, I32Plane, OutputPlane,
    SampleType,
};
use neo_j2k_oracle::{BenchmarkPool, Codec, Format};
use neo_j2k_testkit::files::corpus_dir;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct HeapCalls {
    alloc: usize,
    realloc: usize,
    free: usize,
}

thread_local! {
    static HEAP_CALLS: Cell<Option<HeapCalls>> = const { Cell::new(None) };
}

fn count_call(update: impl FnOnce(&mut HeapCalls)) {
    let _ = HEAP_CALLS.try_with(|cell| {
        if let Some(mut calls) = cell.get() {
            update(&mut calls);
            cell.set(Some(calls));
        }
    });
}

struct CountingAllocator;

// SAFETY: every call forwards its original arguments to System. The thread-local counter is
// initialized without allocation, stores only counters, and is disabled outside the measurement.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count_call(|calls| calls.alloc = calls.alloc.saturating_add(1));
        // SAFETY: the caller supplied a valid allocation layout.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count_call(|calls| calls.alloc = calls.alloc.saturating_add(1));
        // SAFETY: the caller supplied a valid allocation layout.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        count_call(|calls| calls.realloc = calls.realloc.saturating_add(1));
        // SAFETY: the caller supplied a live System allocation and its layout, plus a valid size.
        unsafe { System.realloc(pointer, layout, new_size) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        count_call(|calls| calls.free = calls.free.saturating_add(1));
        // SAFETY: the caller supplied a live System allocation and its layout.
        unsafe { System.dealloc(pointer, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

struct Measurement;

impl Measurement {
    fn start() -> Self {
        HEAP_CALLS.with(|cell| assert!(cell.replace(Some(HeapCalls::default())).is_none()));
        Self
    }

    fn calls(&self) -> HeapCalls {
        HEAP_CALLS.with(|cell| cell.get().expect("the current thread owns the active measurement"))
    }
}

impl Drop for Measurement {
    fn drop(&mut self) {
        HEAP_CALLS.with(|cell| cell.set(None));
    }
}

const SMALL_IMAGES: [&str; 3] = ["sl_32_rgba_53.j2k", "sl_64_rgb_97.j2k", "sl_128_rgba_97.j2k"];

fn load(name: &str) -> Vec<u8> {
    std::fs::read(corpus_dir().join(name)).unwrap_or_else(|error| panic!("cannot read {name}: {error}"))
}

fn decoder(reduce: u32) -> Decoder {
    Decoder::new(DecoderConfig {
        reduce,
        ..DecoderConfig::default()
    })
}

struct PlanarI32 {
    bytes: Vec<u8>,
    planes: Vec<OutputPlane>,
}

impl PlanarI32 {
    fn for_image(image: &DecodedImage<'_>) -> Self {
        const SAMPLE_SIZE: usize = size_of::<i32>();
        let mut offset = 0;
        let planes = image
            .components
            .iter()
            .enumerate()
            .map(|(component, plane)| {
                let row_stride = plane.info().width as usize * SAMPLE_SIZE;
                let view = OutputPlane {
                    component: u32::try_from(component).expect("fixture component indices fit in u32"),
                    offset,
                    row_stride: isize::try_from(row_stride).expect("fixture strides fit in isize"),
                    sample_stride: SAMPLE_SIZE,
                };
                offset += plane.samples().len() * SAMPLE_SIZE;
                view
            })
            .collect();
        Self {
            bytes: vec![0; offset],
            planes,
        }
    }

    fn decode(&mut self, decoder: &mut Decoder, data: &[u8], params: DecodeParams<'_>) -> Result<bool, Error> {
        decoder.decode_into(
            data,
            params,
            SampleType::I32,
            &mut [DecodeTarget {
                bytes: &mut self.bytes,
                planes: &self.planes,
            }],
        )
    }

    fn matches_samples(&self, samples: &[i32]) -> bool {
        let (values, remainder) = self.bytes.as_chunks::<4>();
        remainder.is_empty()
            && values
                .iter()
                .copied()
                .map(i32::from_ne_bytes)
                .eq(samples.iter().copied())
    }
}

fn warmed_heap_calls(mut decode: impl FnMut() -> bool) -> (HeapCalls, bool) {
    let measurement = Measurement::start();
    let mut decoded = true;
    for _ in 0..4 {
        decoded &= decode();
    }
    (measurement.calls(), decoded)
}

#[test]
fn warmed_small_decodes_make_no_heap_calls() {
    use std::hint::black_box;

    let mut heap_regressions = Vec::new();
    for name in SMALL_IMAGES.into_iter().chain([
        "prec16_g_97.j2k",
        "sub_420.j2k",
        "packed_ppt.j2k",
        "packed_ppt_tiles.j2k",
    ]) {
        let data = load(name);
        for (reduce, cut) in [(0, false), (2, false), (0, true)] {
            let input = if cut { &data[..data.len() - 2] } else { &data };
            let mut rust = decoder(reduce);
            let params = DecodeParams::default();
            let first = rust.decode(input, params).expect("the fixture decodes");
            assert_eq!(first.truncated, cut, "{name}, reduce {reduce}");
            if name == "prec16_g_97.j2k" {
                assert_eq!(first.components[0].info().precision, 16);
            }
            if name == "sub_420.j2k" {
                assert_eq!(first.components.len(), 3);
                let chroma = first.components[1].info();
                assert_eq!((chroma.dx, chroma.dy), (2, 2));
            }
            let mut output = PlanarI32::for_image(&first);
            let mut cpp = Codec::new(Format::J2k, reduce, 0).expect("configure the C++ decoder");
            let (status, expected) = cpp.decode_i32(input);
            assert_eq!(status, i32::from(cut));
            assert!(first.components.iter().flat_map(|plane| plane.samples()).eq(&expected));
            assert_eq!(output.decode(&mut rust, input, params).unwrap(), cut);
            assert!(output.matches_samples(&expected));

            let planar = warmed_heap_calls(|| black_box(rust.decode(black_box(input), params)).is_ok());
            let borrowed = warmed_heap_calls(|| {
                let result = output.decode(&mut rust, black_box(input), params).is_ok();
                black_box(output.bytes.as_slice());
                result
            });
            for (path, (calls, decoded)) in [("owned", planar), ("caller-owned", borrowed)] {
                assert!(
                    decoded,
                    "{name}, reduce {reduce}, cut {cut}, {path}: warmed requests decode"
                );
                if calls != HeapCalls::default() {
                    heap_regressions.push((name, reduce, cut, path, calls));
                }
            }
        }
    }
    assert!(
        heap_regressions.is_empty(),
        "heap calls across four warmed repeats per request: {heap_regressions:?}"
    );
}

const ROUNDS: usize = 21;
const BATCH: u32 = 16;

fn elapsed(mut call: impl FnMut(), batch: u32) -> Duration {
    let start = Instant::now();
    for _ in 0..batch {
        call();
    }
    start.elapsed() / batch
}

fn medians<const ROUNDS: usize>(batch: u32, mut rust: impl FnMut(), mut cpp: impl FnMut()) -> (Duration, Duration) {
    let mut rust_times = [Duration::ZERO; ROUNDS];
    let mut cpp_times = [Duration::ZERO; ROUNDS];
    for round in 0..ROUNDS {
        if round.is_multiple_of(2) {
            rust_times[round] = elapsed(&mut rust, batch);
            cpp_times[round] = elapsed(&mut cpp, batch);
        } else {
            cpp_times[round] = elapsed(&mut cpp, batch);
            rust_times[round] = elapsed(&mut rust, batch);
        }
    }
    rust_times.sort_unstable();
    cpp_times.sort_unstable();
    (rust_times[ROUNDS / 2], cpp_times[ROUNDS / 2])
}

fn report(label: &str, rust: Duration, cpp: Duration) {
    println!(
        "{label:48} {:9.2} {:9.2} {:8.3}",
        rust.as_secs_f64() * 1_000_000.0,
        cpp.as_secs_f64() * 1_000_000.0,
        rust.as_secs_f64() / cpp.as_secs_f64(),
    );
}

#[test]
#[ignore = "run with --release and --nocapture on a quiet machine"]
fn small_image_median_latency() {
    benchmark_small_decodes(true);
}

#[test]
#[ignore = "run with --release and --nocapture on a quiet machine"]
fn small_owned_image_median_latency() {
    benchmark_small_decodes(false);
}

fn benchmark_small_decodes(caller_owned: bool) {
    use std::hint::black_box;

    if caller_owned {
        println!("Decode uses caller-owned native-endian planar I32 on both sides.");
    } else {
        println!("Rust decodes into reusable owned I32 planes. C++ writes to a reusable caller-owned I32 buffer.");
    }
    println!("The C++ shim builds plane descriptors per call.");
    println!("{:<48} {:>9} {:>9} {:>8}", "Request", "Rust µs", "C++ µs", "Rust/C++");
    for name in SMALL_IMAGES {
        let data = load(name);
        for (reduce, cut) in [(0, false), (2, false), (0, true)] {
            let input = if cut { &data[..data.len() - 2] } else { &data };
            let mut rust = decoder(reduce);
            let mut cpp = Codec::new(Format::J2k, reduce, 0).expect("configure the C++ decoder");
            let params = DecodeParams::default();
            let mut oracle = cpp.prepare_i32(input);
            let need = oracle.samples_needed();
            assert!(need > 0);
            let mut cpp_output = vec![0; usize::try_from(need).expect("fixture sample count fits in usize")];
            let (status, written) = oracle.decode_into(&mut cpp_output);
            assert_eq!(written, need);
            let warm = rust.decode(input, params).expect("the benchmark fixture decodes");
            assert_eq!(status, i32::from(warm.truncated));
            let mut rust_output = PlanarI32::for_image(&warm);
            assert_eq!(rust_output.decode(&mut rust, input, params).unwrap(), cut);
            assert!(rust_output.matches_samples(&cpp_output));
            let cpp_call = || {
                let (actual, written) = oracle.decode_into(&mut cpp_output);
                assert_eq!(actual, status);
                assert_eq!(written, need);
                black_box(cpp_output.as_slice());
            };
            let (rust_median, cpp_median) = if caller_owned {
                medians::<ROUNDS>(
                    BATCH,
                    || {
                        let truncated = rust_output
                            .decode(&mut rust, black_box(input), params)
                            .expect("warmed Rust decode");
                        assert_eq!(i32::from(truncated), status);
                        black_box(rust_output.bytes.as_slice());
                    },
                    cpp_call,
                )
            } else {
                medians::<ROUNDS>(
                    BATCH,
                    || {
                        let image = rust.decode(black_box(input), params).expect("warmed Rust decode");
                        assert_eq!(i32::from(image.truncated), status);
                        black_box(image);
                    },
                    cpp_call,
                )
            };
            let case = if cut { "missing EOC" } else { "whole" };
            report(&format!("{name} / reduce {reduce} / {case}"), rust_median, cpp_median);
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

fn warm_large_decode(pool: Option<&BenchmarkPool>, mut decode: impl FnMut()) {
    let before = pool.map(BenchmarkPool::batches);
    for _ in 0..2 {
        decode();
    }
    if let Some((pool, before)) = pool.zip(before) {
        assert!(pool.batches() > before, "a threaded decode must use the pool");
    }
}

#[test]
#[ignore = "run with --release and --nocapture on a quiet machine"]
fn large_image_executor_latency() {
    use std::hint::black_box;

    const RETAINED_BYTES: usize = 256 << 20;
    println!("Both codecs use caller-owned typed I32 planes and the same persistent pool. Width includes the caller.");
    println!(
        "{:<26} {:>5} {:>9} {:>9} {:>8} {:>8} {:>8}",
        "Fixture", "Width", "Rust ms", "C++ ms", "Rust/C++", "Rust ×", "C++ ×"
    );
    for name in ["large_1024_rgb_97.j2k", "large_4096_g_97.j2k"] {
        let data = load(name);
        let mut serial = None;
        for width in [1, 2, 4, 8] {
            let host = (width > 1).then(|| SharedPool(BenchmarkPool::new(width).expect("start the shared test pool")));
            let oracle_pool = host.as_ref().map(|host| &host.0);
            let params = DecodeParams {
                executor: host.as_ref().map(|host| host as &dyn Executor),
                ..DecodeParams::default()
            };
            let mut rust = decoder(0);
            rust.set_retained_bytes(RETAINED_BYTES as u64);
            let mut cpp = Codec::new(Format::J2k, 0, 0).expect("configure the C++ decoder");
            assert!(cpp.set_retained_bytes(RETAINED_BYTES));
            let mut prepared = cpp.prepare_i32(&data);
            let need = prepared.samples_needed();
            assert!(need > 0);
            let need_usize = usize::try_from(need).expect("fixture sample count fits in usize");
            let mut cpp_output = vec![0; need_usize];
            let mut rust_output = vec![0; need_usize];
            let mut remaining = rust_output.as_mut_slice();
            rust.read_header(&data).expect("the benchmark fixture has a header");
            let mut planes: Vec<_> = rust
                .decoded_dimensions()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(component, info)| {
                    let (samples, rest) =
                        core::mem::take(&mut remaining).split_at_mut(info.width as usize * info.height as usize);
                    remaining = rest;
                    I32Plane {
                        component: u32::try_from(component).unwrap(),
                        samples,
                    }
                })
                .collect();
            assert!(remaining.is_empty());
            warm_large_decode(oracle_pool, || {
                assert!(!rust.decode_into_i32(&data, params, &mut planes).unwrap());
            });
            warm_large_decode(oracle_pool, || {
                assert_eq!(prepared.decode_into_with_pool(&mut cpp_output, oracle_pool), (0, need));
            });
            assert!(
                planes.iter().flat_map(|plane| plane.samples.iter()).eq(&cpp_output),
                "{name}, width {width}"
            );

            let (rust_time, cpp_time) = medians::<9>(
                2,
                || {
                    assert!(!rust.decode_into_i32(black_box(&data), params, &mut planes).unwrap());
                    black_box(planes.as_slice());
                },
                || {
                    assert_eq!(prepared.decode_into_with_pool(&mut cpp_output, oracle_pool), (0, need));
                    black_box(cpp_output.as_slice());
                },
            );
            let (rust_serial, cpp_serial) = *serial.get_or_insert((rust_time, cpp_time));
            println!(
                "{name:26} {width:5} {:9.3} {:9.3} {:8.3} {:8.2} {:8.2}",
                rust_time.as_secs_f64() * 1_000.0,
                cpp_time.as_secs_f64() * 1_000.0,
                rust_time.as_secs_f64() / cpp_time.as_secs_f64(),
                rust_serial.as_secs_f64() / rust_time.as_secs_f64(),
                cpp_serial.as_secs_f64() / cpp_time.as_secs_f64(),
            );
        }
    }
}

#[cfg(feature = "encode")]
#[test]
#[ignore = "run with --release and --nocapture on a quiet machine"]
fn small_encoder_median_latency() {
    use std::hint::black_box;

    use neo_j2k::{EncodeParams, Encoder, ImageSamples, ImageView};
    use neo_j2k_oracle::{EncodeRequest, encode_u8};

    println!("Both sides create an encoder per call. The C++ shim copies input and encoded output.");
    println!("{:<48} {:>9} {:>9} {:>8}", "Request", "Rust µs", "C++ µs", "Rust/C++");
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
                flip_vertical: false,
                reversible,
                resolutions: 6,
                mct: true,
                max_codestream_size: 0,
                layer_ratios: &[],
                comment: None,
            };
            let expected = encode_u8(&request).expect("the C++ encoder accepts the fixture");
            assert_eq!(
                Encoder::new()
                    .encode(&view, &params)
                    .expect("the Rust encoder accepts the fixture"),
                expected
            );
            // The C++ shim creates an encoder per invocation. Match that lifecycle on Rust.
            let rust_call = || {
                black_box(Encoder::new().encode(black_box(&view), &params).expect("Rust encode"));
            };
            let cpp_call = || {
                black_box(encode_u8(black_box(&request)).expect("C++ encode"));
            };
            let (rust_median, cpp_median) = medians::<ROUNDS>(BATCH, rust_call, cpp_call);
            let wavelet = if reversible { "lossless 5/3" } else { "lossy 9/7" };
            report(
                &format!("encode {side}x{side} RGB / {wavelet}"),
                rust_median,
                cpp_median,
            );
        }
    }
}

#[cfg(feature = "encode")]
#[test]
#[ignore = "run with --release and --nocapture on a quiet machine"]
fn large_encoder_executor_latency() {
    use std::hint::black_box;

    use neo_j2k::{EncodeParams, Encoder, ImageSamples, ImageView};

    println!("Encoder is reused; executor workers include the calling thread. Times are medians of nine rounds.");
    println!("{:<34} {:>7} {:>10} {:>10}", "Request", "Workers", "Rust ms", "Speedup");
    for side in [256usize, 512, 1024] {
        let stride = side * 3;
        let pixels: Vec<u8> = (0..side * stride)
            .map(|index| {
                let x = (index % stride) / 3;
                let y = index / stride;
                let component = index % 3;
                (3 * x + 5 * y + 13 * (x ^ y) + 23 * component).to_le_bytes()[0]
            })
            .collect();
        let side_u32 = u32::try_from(side).expect("the benchmark image side fits u32");
        let image = ImageView {
            width: side_u32,
            height: side_u32,
            flip_vertical: false,
            samples: ImageSamples::Interleaved {
                samples: &pixels,
                components: 3,
                row_stride: stride,
            },
        };
        for reversible in [true, false] {
            let mut serial_params = EncodeParams::default();
            serial_params.reversible = reversible;
            serial_params.mct = true;
            let expected = Encoder::new().encode(&image, &serial_params).unwrap().to_vec();
            let mut baseline = Duration::ZERO;
            for width in [1u32, 2, 4, 8] {
                let pool = SharedPool(BenchmarkPool::new(width).unwrap());
                let mut params = EncodeParams::default();
                params.reversible = reversible;
                params.mct = true;
                params.executor = Some(&pool);
                let mut encoder = Encoder::new();
                assert_eq!(encoder.encode(&image, &params).unwrap(), expected);
                let before = pool.0.batches();
                let batch = if side == 256 {
                    4
                } else if side == 512 {
                    2
                } else {
                    1
                };
                let mut times = [Duration::ZERO; 9];
                for time in &mut times {
                    *time = elapsed(
                        || {
                            black_box(encoder.encode(black_box(&image), &params).unwrap());
                        },
                        batch,
                    );
                }
                times.sort_unstable();
                let median = times[times.len() / 2];
                if width == 1 {
                    baseline = median;
                    assert_eq!(pool.0.batches(), before, "one worker must remain serial");
                } else {
                    assert!(pool.0.batches() > before, "threaded encode must use its executor");
                }
                let wavelet = if reversible { "lossless" } else { "lossy" };
                println!(
                    "{side}x{side} RGB {wavelet:<17} {width:>7} {:>10.3} {:>9.2}x",
                    median.as_secs_f64() * 1000.0,
                    baseline.as_secs_f64() / median.as_secs_f64(),
                );
            }
        }
    }
}
