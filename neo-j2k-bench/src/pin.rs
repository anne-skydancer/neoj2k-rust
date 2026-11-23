// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Pins the measuring thread to one logical CPU. No dependency: the system calls are declared here (the
//! workspace bans the `libc` crate).

#[cfg(windows)]
mod os {
    unsafe extern "system" {
        fn GetCurrentThread() -> isize;
        fn GetCurrentProcess() -> isize;
        fn SetThreadAffinityMask(thread: isize, mask: usize) -> usize;
        fn SetPriorityClass(process: isize, class: u32) -> i32;
        fn SetThreadPriority(thread: isize, priority: i32) -> i32;
    }

    const HIGH_PRIORITY_CLASS: u32 = 0x80;
    const THREAD_PRIORITY_HIGHEST: i32 = 2;

    pub(super) fn pin(cpu: usize) -> Result<&'static str, String> {
        if cpu >= usize::BITS as usize {
            return Err(format!("CPU {cpu} is outside the first processor group"));
        }
        // SAFETY: no arguments; returns the calling thread's pseudo-handle.
        let thread = unsafe { GetCurrentThread() };
        // SAFETY: the pseudo-handle is valid for this thread; the mask names one CPU.
        let previous = unsafe { SetThreadAffinityMask(thread, 1usize << cpu) };
        if previous == 0 {
            return Err(format!("SetThreadAffinityMask refused CPU {cpu}"));
        }
        // SAFETY: no arguments; returns the process pseudo-handle.
        let process = unsafe { GetCurrentProcess() };
        // SAFETY: the pseudo-handle is valid; a priority class is a plain value.
        let class = unsafe { SetPriorityClass(process, HIGH_PRIORITY_CLASS) };
        // SAFETY: the pseudo-handle is valid for this thread; a thread priority is a plain value.
        let priority = unsafe { SetThreadPriority(thread, THREAD_PRIORITY_HIGHEST) };
        Ok(if class == 0 || priority == 0 {
            "not raised"
        } else {
            "raised"
        })
    }
}

#[cfg(target_os = "linux")]
mod os {
    unsafe extern "C" {
        fn sched_setaffinity(pid: i32, cpusetsize: usize, mask: *const u64) -> i32;
    }

    pub(super) fn pin(cpu: usize) -> Result<&'static str, String> {
        // glibc's cpu_set_t: 1,024 bits
        let mut set = [0u64; 16];
        if cpu >= 1024 {
            return Err(format!("CPU {cpu} is above 1,023"));
        }
        set[cpu / 64] = 1 << (cpu % 64);
        // SAFETY: pid 0 is the calling thread; the mask is a live array of the size passed.
        let status = unsafe { sched_setaffinity(0, size_of_val(&set), set.as_ptr()) };
        if status == 0 {
            Ok("unchanged")
        } else {
            Err(format!("sched_setaffinity refused CPU {cpu}"))
        }
    }
}

#[cfg(not(any(windows, target_os = "linux")))]
mod os {
    pub(super) fn pin(_cpu: usize) -> Result<&'static str, String> {
        Err("pinning is implemented for Windows and Linux only".to_owned())
    }
}

/// Pins the calling thread to the logical CPU (Windows: the first processor group, and a raised priority). The thread
/// is pinned when this returns `Ok`; the string says whether the priority was `raised`, `not raised` (the pin holds)
/// or `unchanged` (not attempted).
///
/// # Errors
/// What the system refused: the thread is not pinned.
pub(crate) fn pin_to_cpu(cpu: usize) -> Result<&'static str, String> {
    os::pin(cpu)
}
