// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use std::any::Any;
use std::ffi::c_void;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::ptr::NonNull;
use std::sync::Mutex;

use super::{ffi, init};

/// The existing C++ test harness pool, shared by Rust and C++ in paired benchmarks.
#[derive(Debug)]
pub struct BenchmarkPool {
    pool: NonNull<ffi::Pool>,
    width: u32,
}

// SAFETY: the C++ pool supports concurrent and nested calls, with no thread affinity. Each call
// borrows the pool, so its owner cannot free it during a batch.
unsafe impl Sync for BenchmarkPool {}

impl BenchmarkPool {
    /// Start a persistent pool for one to 64 participants, including the calling thread.
    #[must_use]
    pub fn new(width: u32) -> Option<Self> {
        let workers = width.checked_sub(1).filter(|&workers| workers < 64)?;
        init();
        // SAFETY: the worker count is passed by value. A non-null result is an owned pool.
        let pool = NonNull::new(unsafe { ffi::nj2k_shim_pool_new(workers) })?;
        Some(Self { pool, width })
    }

    /// Number of participants, including the calling thread.
    #[must_use]
    pub const fn concurrency(&self) -> u32 {
        self.width
    }

    /// The number of batches the pool has received from either implementation.
    #[must_use]
    pub fn batches(&self) -> u64 {
        // SAFETY: the pool is live and the counter is atomic.
        unsafe { ffi::nj2k_shim_pool_batches(self.as_ptr()) }
    }

    /// Run `job` once for every index, on the caller and workers, and wait for all jobs.
    /// A job's panic is rethrown on the caller after the batch has finished.
    pub fn parallel_for(&self, count: u32, job: &(dyn Fn(u32) + Sync)) {
        let batch = Batch {
            job,
            panics: Mutex::new(Vec::new()),
        };
        // SAFETY: the pool and batch live through this synchronous call. The callback is safe to
        // invoke concurrently and does not unwind into C++.
        let ok =
            unsafe { ffi::nj2k_shim_pool_run(self.as_ptr(), count, run_job, (&raw const batch).cast_mut().cast()) };
        assert_eq!(ok, 1, "the C++ harness could not queue the batch");
        if let Some(payload) = batch.panics.into_inner().unwrap().pop() {
            resume_unwind(payload);
        }
    }

    pub(crate) const fn as_ptr(&self) -> *mut ffi::Pool {
        self.pool.as_ptr()
    }
}

impl Drop for BenchmarkPool {
    fn drop(&mut self) {
        // SAFETY: the pool was allocated by the shim and is owned exclusively here.
        unsafe { ffi::nj2k_shim_pool_free(self.as_ptr()) };
    }
}

struct Batch<'a> {
    job: &'a (dyn Fn(u32) + Sync),
    panics: Mutex<Vec<Box<dyn Any + Send>>>,
}

extern "C" fn run_job(context: *mut c_void, index: u32) {
    // SAFETY: parallel_for passes a pointer to its live, shared batch for this synchronous call.
    let batch = unsafe { &*context.cast::<Batch<'_>>() };
    if let Err(payload) = catch_unwind(AssertUnwindSafe(|| (batch.job)(index))) {
        batch.panics.lock().unwrap().push(payload);
    }
}
