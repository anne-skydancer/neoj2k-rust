// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#[cfg(target_feature = "avx2")]
pub(super) mod dwt;
#[cfg(target_feature = "avx2")]
pub(super) mod mct;
#[cfg(target_feature = "avx2")]
pub(super) mod output;
#[cfg(target_feature = "avx2")]
pub(super) mod t1;

use core::arch::x86_64::{_mm_cvtss_si32, _mm_set_ss};

/// Round one sample in the target's default floating-point environment.
/// An unrepresentable result, including NaN, produces `i32::MIN`.
#[inline]
pub(super) fn round_i32(value: f32) -> i32 {
    // SAFETY: the module is compiled only for targets with SSE enabled.
    let sample = unsafe { _mm_set_ss(value) };
    // SAFETY: the same SSE requirement applies to the conversion.
    unsafe { _mm_cvtss_si32(sample) }
}
