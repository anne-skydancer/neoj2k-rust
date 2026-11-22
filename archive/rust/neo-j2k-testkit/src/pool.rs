// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! A host's executor for the tests (the counterpart of `tests/common/thread_pool.h`):
//! `parallel_for` runs `count` jobs on up to `workers` threads besides the caller, the caller
//! claiming jobs too, and returns when every job has returned. The threads are scoped to the call
//! (`std::thread::scope`), so jobs borrow freely and nothing outlives the call; that costs a thread
//! start a batch, which the tests can afford and the benches do not use. A panicking job panics
//! the call once every thread is joined.

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// A test executor.
#[derive(Debug)]
pub struct Pool {
    workers: u32,
    batches: AtomicU64,
}

impl Pool {
    /// A pool of `workers` threads besides the caller (0: every job runs on the caller).
    #[must_use]
    pub const fn new(workers: u32) -> Self {
        Self {
            workers,
            batches: AtomicU64::new(0),
        }
    }

    /// The threads that run a batch: the workers and the caller.
    #[must_use]
    pub const fn concurrency(&self) -> u32 {
        self.workers.saturating_add(1)
    }

    /// Calls of [`Pool::parallel_for`] so far (the tests check whether a decode used the pool).
    #[must_use]
    pub fn batches(&self) -> u64 {
        self.batches.load(Ordering::Relaxed)
    }

    /// Runs `job(0)` to `job(count - 1)`, each once, on the workers and the caller.
    pub fn parallel_for(&self, count: u32, job: &(dyn Fn(u32) + Sync)) {
        self.batches.fetch_add(1, Ordering::Relaxed);
        let next = AtomicU32::new(0);
        let drain = || {
            loop {
                // Relaxed: the index alone is shared; each job's data reaches the caller through
                // the scope's join, which synchronises with every thread's end.
                let i = next.fetch_add(1, Ordering::Relaxed);
                if i >= count {
                    break;
                }
                job(i);
            }
        };
        let helpers = self.workers.min(count.saturating_sub(1));
        if helpers == 0 {
            drain();
            return;
        }
        std::thread::scope(|s| {
            for _ in 0..helpers {
                s.spawn(drain);
            }
            drain();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn every_job_runs_once() {
        for workers in [0, 1, 3, 7] {
            for count in [0u32, 1, 2, 5, 64, 129] {
                let pool = Pool::new(workers);
                let seen = Mutex::new(vec![0u32; count as usize]);
                pool.parallel_for(count, &|i| seen.lock().unwrap()[i as usize] += 1);
                assert!(
                    seen.into_inner().unwrap().iter().all(|&n| n == 1),
                    "{workers} workers, {count} jobs"
                );
                assert_eq!(pool.batches(), 1);
                assert_eq!(pool.concurrency(), workers + 1);
            }
        }
    }

    #[test]
    fn jobs_write_disjoint_slots() {
        let pool = Pool::new(3);
        let slots: Vec<Mutex<u32>> = (0..32).map(|_| Mutex::new(0)).collect();
        pool.parallel_for(32, &|i| *slots[i as usize].lock().unwrap() = i * 2);
        assert!(
            slots
                .iter()
                .enumerate()
                .all(|(i, s)| *s.lock().unwrap() as usize == i * 2)
        );
    }
}
