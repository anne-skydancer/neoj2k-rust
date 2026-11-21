// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use neo_j2k::{DecodeParams, Decoder, DecoderConfig, Diagnostic, ErrorKind, Executor, I32Plane};
use neo_j2k_oracle::{Codec, Format};
use neo_j2k_testkit::files::corpus_dir;
use neo_j2k_testkit::pool::Pool;

thread_local! {
    static LARGEST: Cell<Option<usize>> = const { Cell::new(None) };
    static FIRST_PLAN_ALLOCATION: Cell<PlanAllocation> = const { Cell::new(PlanAllocation::Idle) };
}

struct TrackAllocations;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PlanAllocation {
    Idle,
    AwaitingConcurrency { refuse: bool },
    BeforeDispatch { refuse: bool },
    Attempted { size: usize, denied: usize },
}

struct PlanAllocationProbe;

impl PlanAllocationProbe {
    fn new(state: PlanAllocation) -> Self {
        assert_eq!(FIRST_PLAN_ALLOCATION.replace(state), PlanAllocation::Idle);
        Self
    }
}

impl Drop for PlanAllocationProbe {
    fn drop(&mut self) {
        FIRST_PLAN_ALLOCATION.set(PlanAllocation::Idle);
    }
}

fn allocation_allowed(size: usize) -> bool {
    let _ = LARGEST.try_with(|largest| {
        if let Some(previous) = largest.get() {
            largest.set(Some(previous.max(size)));
        }
    });
    FIRST_PLAN_ALLOCATION
        .try_with(|probe| match probe.get() {
            PlanAllocation::BeforeDispatch { refuse } => {
                probe.set(PlanAllocation::Attempted {
                    size,
                    denied: usize::from(refuse),
                });
                !refuse
            }
            PlanAllocation::Attempted { size, denied } if denied != 0 => {
                probe.set(PlanAllocation::Attempted {
                    size,
                    denied: denied + 1,
                });
                false
            }
            _ => true,
        })
        .unwrap_or(true)
}

// SAFETY: forwards valid operations unchanged or returns null for a refused allocation.
unsafe impl GlobalAlloc for TrackAllocations {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if !allocation_allowed(layout.size()) {
            return std::ptr::null_mut();
        }
        // SAFETY: the caller supplies a valid layout.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if !allocation_allowed(layout.size()) {
            return std::ptr::null_mut();
        }
        // SAFETY: the caller supplies a valid layout.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if !allocation_allowed(size) {
            return std::ptr::null_mut();
        }
        // SAFETY: the caller supplies a system allocation with its current layout.
        unsafe { System.realloc(ptr, layout, size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: the caller supplies a system allocation with its current layout.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: TrackAllocations = TrackAllocations;

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(corpus_dir().join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

#[test]
fn large_i32_output_avoids_a_plane_sized_allocation() {
    let input = fixture("large_4096_g_97.j2k");
    let mut decoder = Decoder::new(DecoderConfig::default());
    let mut samples = vec![-1; 4096 * 4096];
    let mut targets = [I32Plane {
        component: 0,
        samples: &mut samples,
    }];
    LARGEST.set(Some(0));
    let result = decoder.decode_into_i32(&input, DecodeParams::default(), &mut targets);
    let largest = LARGEST.replace(None).unwrap();
    assert!(!result.unwrap());
    assert!(
        largest < size_of_val(samples.as_slice()),
        "largest internal allocation was {largest} bytes"
    );
    assert!(samples.iter().all(|&sample| (0..=255).contains(&sample)));
}

#[test]
fn typed_planes_match_cpp_for_mct_cuts_and_multi_tile_images() {
    for (name, reduce, cut) in [
        ("sl_64_rgb_97.j2k", 0, 0),
        ("sl_64_rgb_97.j2k", 2, 2),
        ("sl_128_rgba_97.j2k", 0, 0),
        ("sl_32_rgba_53.j2k", 0, 0),
        ("tile_100x80_t64_97.j2k", 0, 0),
    ] {
        let input = fixture(name);
        let data = &input[..input.len() - cut];
        let mut decoder = Decoder::new(DecoderConfig {
            reduce,
            ..DecoderConfig::default()
        });
        decoder.read_header(data).unwrap();
        let lengths: Vec<_> = decoder
            .decoded_dimensions()
            .unwrap()
            .iter()
            .map(|c| (c.width * c.height) as usize)
            .collect();
        let (status, expected) = Codec::new(Format::J2k, reduce, 0).unwrap().decode_i32(data);
        assert!(status == 0 || status == 1, "{name}: C++ status {status}");
        let mut samples: Vec<_> = lengths.iter().map(|&len| vec![-1; len + 4]).collect();
        let mut planes: Vec<_> = samples
            .iter_mut()
            .enumerate()
            .rev()
            .map(|(component, samples)| I32Plane {
                component: u32::try_from(component).unwrap(),
                samples,
            })
            .collect();
        assert_eq!(
            i32::from(
                decoder
                    .decode_into_i32(data, DecodeParams::default(), &mut planes)
                    .unwrap()
            ),
            status,
            "{name}: truncation status"
        );
        assert!(
            samples
                .iter()
                .zip(&lengths)
                .flat_map(|(p, &len)| &p[..len])
                .eq(&expected),
            "{name}: decoded samples"
        );
        assert!(samples.iter().zip(&lengths).all(|(p, &len)| p[len..] == [-1; 4]));
    }
}

#[test]
fn selected_planes_can_use_direct_output_or_repeat_a_target() {
    let input = fixture("sl_64_rgb_97.j2k");
    let mut decoder = Decoder::new(DecoderConfig::default());
    let params = DecodeParams {
        components: Some(&[2, 0]),
        ..DecodeParams::default()
    };
    let image = decoder.decode(&input, params).unwrap();
    let expected: Vec<_> = image.components.iter().map(|plane| plane.samples().to_vec()).collect();
    let mut first = vec![-1; expected[0].len()];
    let mut second = vec![-1; expected[1].len()];
    let mut decode = |component| {
        let mut planes = [
            I32Plane {
                component: 2,
                samples: &mut first,
            },
            I32Plane {
                component,
                samples: &mut second,
            },
        ];
        decoder.decode_into_i32(&input, params, &mut planes)
    };
    assert_eq!(decode(1).unwrap_err().kind(), ErrorKind::Output);
    assert!(!decode(0).unwrap());
    assert_eq!(first, expected[0]);
    assert_eq!(second, expected[1]);

    let mut planes = [
        I32Plane {
            component: 2,
            samples: &mut first,
        },
        I32Plane {
            component: 2,
            samples: &mut second,
        },
    ];
    assert!(!decoder.decode_into_i32(&input, params, &mut planes).unwrap());
    assert_eq!(first, expected[0]);
    assert_eq!(second, expected[0]);
}

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

struct PlanningHost {
    width: usize,
    batches: AtomicUsize,
}

impl Executor for PlanningHost {
    fn concurrency(&self) -> usize {
        if let PlanAllocation::AwaitingConcurrency { refuse } = FIRST_PLAN_ALLOCATION.get() {
            FIRST_PLAN_ALLOCATION.set(PlanAllocation::BeforeDispatch { refuse });
        }
        self.width
    }

    fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync)) {
        if let PlanAllocation::BeforeDispatch { .. } = FIRST_PLAN_ALLOCATION.get() {
            FIRST_PLAN_ALLOCATION.set(PlanAllocation::Idle);
        }
        self.batches.fetch_add(1, Relaxed);
        for index in 0..count {
            job(index);
        }
    }
}

#[test]
fn tier1_planning_allocation_refusal_decodes_serially_and_can_retry() {
    let input = fixture("large_4096_g_97.j2k");
    let mut missing_eoc = input.clone();
    assert!(missing_eoc.ends_with(&[0xff, 0xd9]));
    *missing_eoc.last_mut().unwrap() = 0x92;
    let reduce = 1;
    let (status, expected) = Codec::new(Format::J2k, reduce, 0).unwrap().decode_i32(&missing_eoc);
    assert_eq!(status, 1);
    let mut decoder = Decoder::new(DecoderConfig {
        reduce,
        capture_diagnostics: true,
        ..DecoderConfig::default()
    });
    decoder.set_retained_bytes(128 << 20);
    let mut samples = vec![-1; expected.len()];
    let mut decode = |input: &[u8], truncated, width, refuse| {
        samples.fill(-1);
        let mut planes = [I32Plane {
            component: 0,
            samples: &mut samples,
        }];
        let host = PlanningHost {
            width,
            batches: AtomicUsize::new(0),
        };
        let params = DecodeParams {
            executor: Some(&host),
            ..DecodeParams::default()
        };
        let probe = PlanAllocationProbe::new(PlanAllocation::AwaitingConcurrency { refuse });
        let result = decoder.decode_into_i32(input, params, &mut planes);
        let allocation = FIRST_PLAN_ALLOCATION.get();
        drop(probe);
        assert_eq!(result.unwrap(), truncated);
        assert!(samples == expected, "width {width}, allocation {allocation:?}");
        let warning = decoder
            .diagnostics()
            .iter()
            .any(|event| matches!(event, Diagnostic::Warning(text) if text == "Stream does not end with EOC\n"));
        (allocation, host.batches.load(Relaxed), warning)
    };

    // Warm the serial fallback and the executor path, including the packet tables.
    decode(&input, false, 1, false);
    decode(&input, false, 4, false);
    let (serial, serial_batches, _) = decode(&input, false, 1, false);
    assert_eq!(serial, PlanAllocation::BeforeDispatch { refuse: false });
    assert_eq!(serial_batches, 0);

    // The first allocation between the concurrency query and the first job is specific
    // to the parallel Tier-1 plan. Calibrate the request size without using private types.
    let (scheduled, batches, warning) = decode(&missing_eoc, true, 4, false);
    let PlanAllocation::Attempted { size, denied: 0 } = scheduled else {
        panic!("missing planning allocation: {scheduled:?}");
    };
    assert!(batches > 1);
    assert!(warning);
    // Refuse allocations through the rest of this attempt. The warmed serial scratch can
    // finish the decode even when the warning and optional fallback allocations are refused.
    let (refused, fallback_batches, warning) = decode(&missing_eoc, true, 4, true);
    let PlanAllocation::Attempted {
        size: refused_size,
        denied,
    } = refused
    else {
        panic!("missing refused planning allocation: {refused:?}");
    };
    assert_eq!(refused_size, size);
    assert!(denied > 1);
    assert!(fallback_batches < batches);
    assert!(!warning);
    assert_eq!(decode(&missing_eoc, true, 4, false), (scheduled, batches, true));
}

struct RefusingHost {
    target_batch: usize,
    first_job: usize,
    batches: AtomicUsize,
    refused_batch_size: AtomicUsize,
}

impl Executor for RefusingHost {
    fn concurrency(&self) -> usize {
        4
    }

    fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync)) {
        let batch = self.batches.fetch_add(1, Relaxed);
        for index in 0..count {
            if batch == self.target_batch && index >= self.first_job && self.refused_batch_size.load(Relaxed) == 0 {
                let probe = PlanAllocationProbe::new(PlanAllocation::BeforeDispatch { refuse: true });
                job(index);
                let allocation = FIRST_PLAN_ALLOCATION.get();
                drop(probe);
                if matches!(allocation, PlanAllocation::Attempted { denied: 1.., .. }) {
                    self.refused_batch_size.store(count, Relaxed);
                }
            } else {
                job(index);
            }
        }
    }
}

fn check_rgb_job_allocation_fallback(target_batch: usize, first_job: usize) -> usize {
    let input = fixture("large_1024_rgb_97.j2k");
    let (status, expected) = Codec::new(Format::J2k, 0, 0).unwrap().decode_i32(&input);
    assert_eq!(status, 0);
    let mut decoder = Decoder::new(DecoderConfig {
        capture_diagnostics: true,
        ..DecoderConfig::default()
    });
    decoder.set_retained_bytes(128 << 20);
    let mut samples = vec![0; expected.len()];
    let (planes, remainder) = samples.as_chunks_mut::<{ 1024 * 1024 }>();
    assert!(remainder.is_empty());
    let mut planes: Vec<_> = planes
        .iter_mut()
        .enumerate()
        .map(|(component, samples)| I32Plane {
            component: u32::try_from(component).unwrap(),
            samples,
        })
        .collect();
    let mut decode = |executor: Option<&dyn Executor>| {
        for plane in &mut planes {
            plane.samples.fill(-1);
        }
        let params = DecodeParams {
            executor,
            ..DecodeParams::default()
        };
        assert!(!decoder.decode_into_i32(&input, params, &mut planes).unwrap());
        assert!(planes.iter().flat_map(|plane| plane.samples.iter()).eq(&expected));
        assert!(decoder.diagnostics().is_empty());
    };

    // The serial fallback is warm, but each worker still has to allocate its own scratch.
    decode(None);
    let host = RefusingHost {
        target_batch,
        first_job,
        batches: AtomicUsize::new(0),
        refused_batch_size: AtomicUsize::new(0),
    };
    decode(Some(&host));
    let refused_batch_size = host.refused_batch_size.load(Relaxed);
    assert!(refused_batch_size > 0, "the allocator must refuse worker scratch");

    let retry = PlanningHost {
        width: 4,
        batches: AtomicUsize::new(0),
    };
    decode(Some(&retry));
    assert!(retry.batches.load(Relaxed) > 1);
    refused_batch_size
}

#[test]
fn grouped_tier1_refuses_scratch_after_a_job_has_written_coefficients() {
    assert!(check_rgb_job_allocation_fallback(0, 1) > 3);
}

#[test]
fn coarse_wavelet_scratch_refusal_replays_only_the_failed_plane() {
    assert_eq!(check_rgb_job_allocation_fallback(1, 0), 3);
}

#[test]
fn direct_rgb_planes_match_cpp_at_different_executor_widths() {
    let input = fixture("large_1024_rgb_97.j2k");
    let (status, expected) = Codec::new(Format::J2k, 0, 0).unwrap().decode_i32(&input);
    assert_eq!(status, 0);
    let mut decoder = Decoder::new(DecoderConfig::default());
    let [mut red, mut green, mut blue] = std::array::from_fn(|_| vec![-1; 1024 * 1024]);
    let mut planes = [
        I32Plane {
            component: 2,
            samples: &mut blue,
        },
        I32Plane {
            component: 1,
            samples: &mut green,
        },
        I32Plane {
            component: 0,
            samples: &mut red,
        },
    ];
    for workers in [1, 3, 7] {
        let host = Host(Pool::new(workers));
        let params = DecodeParams {
            executor: Some(&host),
            ..DecodeParams::default()
        };
        assert!(!decoder.decode_into_i32(&input, params, &mut planes).unwrap());
        assert!(host.0.batches() > 1);
        assert!(planes.iter().rev().flat_map(|plane| plane.samples.iter()).eq(&expected));
    }
}
