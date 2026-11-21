// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Synchronous jobs borrowed from the host for one decode.

use std::sync::Mutex;

pub(crate) const MAX_JOBS: usize = 64;

/// A host scheduler that can run independent parts of a large JPEG 2000 decode.
///
/// The codec calls the scheduler only during the decode and creates no threads of its own.
pub trait Executor: Sync {
    /// Maximum simultaneous jobs, including the calling thread if it participates.
    /// Returning zero or one keeps the work on the calling thread.
    fn concurrency(&self) -> usize;

    /// Run each job index in `0..count` once and return after all jobs finish.
    /// The batch can contain more jobs than `concurrency()` allows to run simultaneously.
    /// Jobs may run in any order, including on the calling thread.
    fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync));
}

#[derive(Clone, Copy)]
pub(crate) struct Jobs<'a> {
    host: &'a dyn Executor,
    width: usize,
}

impl<'a> Jobs<'a> {
    pub(crate) fn new(host: &'a dyn Executor) -> Option<Self> {
        let width = host.concurrency().min(MAX_JOBS);
        (width > 1).then_some(Self { host, width })
    }

    pub(crate) const fn width(self) -> usize {
        self.width
    }

    // Hand distinct mutable parts to a borrowed Fn callback without sharing sample writes.
    pub(crate) fn run<T: Send>(self, parts: impl IntoIterator<Item = T>, work: impl Fn(T) + Sync) {
        let mut parts = parts.into_iter();
        let Some(first) = parts.next() else { return };
        let Some(second) = parts.next() else {
            work(first);
            return;
        };
        let mut slots: [Mutex<Option<T>>; MAX_JOBS] = std::array::from_fn(|_| Mutex::new(None));
        let mut count = 0;
        for (index, part) in [first, second].into_iter().chain(parts).enumerate() {
            *slots[index].get_mut().expect("the job slot is not poisoned") = Some(part);
            count = index + 1;
        }
        {
            let slots = &slots[..count];
            self.host.parallel_for(count, &|index| {
                if let Some(slot) = slots.get(index) {
                    let part = slot.lock().expect("the job slot is not poisoned").take();
                    if let Some(part) = part {
                        work(part);
                    }
                }
            });
        }
        // The contract runs every index once. Complete any remaining work on the caller if a host omits one.
        for slot in &mut slots[..count] {
            if let Some(part) = slot.get_mut().expect("the job slot is not poisoned").take() {
                work(part);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TwoThreads;

    impl Executor for TwoThreads {
        fn concurrency(&self) -> usize {
            2
        }

        fn parallel_for(&self, count: usize, job: &(dyn Fn(usize) + Sync)) {
            assert_eq!(count, 2);
            std::thread::scope(|scope| {
                scope.spawn(|| job(1));
                job(0);
            });
        }
    }

    #[test]
    fn scoped_workers_write_distinct_rows() {
        let mut rows = [[0; 4]; 4];
        let jobs = Jobs::new(&TwoThreads).unwrap();
        jobs.run(rows.chunks_mut(2).enumerate(), |(index, rows)| {
            for row in rows {
                row.fill(index + 1);
            }
        });
        assert_eq!(rows, [[1; 4], [1; 4], [2; 4], [2; 4]]);
    }
}
