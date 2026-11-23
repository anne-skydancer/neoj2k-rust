// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// Standalone allocation probe: use as src/main.rs in a temporary Cargo package with neo-j2k as a path dependency.
use neo_j2k::{DecodeParams, Decoder, DecoderConfig, Executor, I32Plane};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
static ON: AtomicBool = AtomicBool::new(false);
static ALLOC: AtomicUsize = AtomicUsize::new(0);
static REALLOC: AtomicUsize = AtomicUsize::new(0);
struct Count;
unsafe impl GlobalAlloc for Count {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if ON.load(Relaxed) {
            ALLOC.fetch_add(1, Relaxed);
        }
        unsafe { System.alloc(l) }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        if ON.load(Relaxed) {
            ALLOC.fetch_add(1, Relaxed);
        }
        unsafe { System.alloc_zeroed(l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        if ON.load(Relaxed) {
            REALLOC.fetch_add(1, Relaxed);
        }
        unsafe { System.realloc(p, l, n) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
}
#[global_allocator]
static A: Count = Count;
struct Inline(usize);
impl Executor for Inline {
    fn concurrency(&self) -> usize {
        self.0
    }
    fn parallel_for(&self, n: usize, f: &(dyn Fn(usize) + Sync)) {
        for i in 0..n {
            f(i)
        }
    }
}
fn main() {
    for width in [1, 2, 4, 8] {
        let data = std::fs::read(std::env::args().nth(1).expect("pass the large_1024_rgb_97.j2k path")).unwrap();
        let host = Inline(width);
        let mut d = Decoder::new(DecoderConfig {
            retain_bytes: 256 << 20,
            ..DecoderConfig::default()
        });
        d.read_header(&data).unwrap();
        let dims = d.decoded_dimensions().unwrap().to_vec();
        let mut samples: Vec<Vec<i32>> = dims.iter().map(|x| vec![0; (x.width * x.height) as usize]).collect();
        let mut planes: Vec<_> = samples
            .iter_mut()
            .enumerate()
            .map(|(i, s)| I32Plane {
                component: i as u32,
                samples: s,
            })
            .collect();
        let p = DecodeParams {
            executor: Some(&host),
            ..DecodeParams::default()
        };
        for _ in 0..3 {
            d.decode_into_i32(&data, p, &mut planes).unwrap();
        }
        ALLOC.store(0, Relaxed);
        REALLOC.store(0, Relaxed);
        ON.store(true, Relaxed);
        let result = d.decode_into_i32(&data, p, &mut planes);
        ON.store(false, Relaxed);
        result.unwrap();
        println!(
            "width {width}: allocations {}, reallocations {}",
            ALLOC.load(Relaxed),
            REALLOC.load(Relaxed)
        );
    }
}
