// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The clock: time stamp counter cycles on x86-64 (the C++ benches' clock), nanoseconds elsewhere.

/// The unit of [`start`] and [`stop`].
#[cfg(target_arch = "x86_64")]
pub(crate) const UNIT: &str = "TSC cycles";
/// The unit of [`start`] and [`stop`].
#[cfg(not(target_arch = "x86_64"))]
pub(crate) const UNIT: &str = "ns";

/// The counter at the start of a timed region: nothing earlier is still in flight.
#[cfg(target_arch = "x86_64")]
#[inline]
pub(crate) fn start() -> u64 {
    // SAFETY: SSE2 is part of x86-64; the fence reads and writes nothing.
    unsafe { core::arch::x86_64::_mm_lfence() };
    // SAFETY: the time stamp counter is readable by user code on every x86-64 CPU the benchmark runs on (CR4.TSD is
    // clear on Windows and Linux).
    let t = unsafe { core::arch::x86_64::_rdtsc() };
    // SAFETY: as the first fence.
    unsafe { core::arch::x86_64::_mm_lfence() };
    t
}

/// The counter at the end of a timed region: everything before it has finished.
#[cfg(target_arch = "x86_64")]
#[inline]
pub(crate) fn stop() -> u64 {
    // SAFETY: as `start`'s fence.
    unsafe { core::arch::x86_64::_mm_lfence() };
    // SAFETY: as `start`'s counter read.
    let t = unsafe { core::arch::x86_64::_rdtsc() };
    // SAFETY: as `start`'s fence.
    unsafe { core::arch::x86_64::_mm_lfence() };
    t
}

#[cfg(not(target_arch = "x86_64"))]
fn nanoseconds() -> u64 {
    use std::sync::OnceLock;
    use std::time::Instant;
    static ORIGIN: OnceLock<Instant> = OnceLock::new();
    u64::try_from(ORIGIN.get_or_init(Instant::now).elapsed().as_nanos()).unwrap_or(u64::MAX)
}

/// The clock at the start of a timed region.
#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn start() -> u64 {
    nanoseconds()
}

/// The clock at the end of a timed region.
#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn stop() -> u64 {
    nanoseconds()
}
