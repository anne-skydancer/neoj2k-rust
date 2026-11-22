// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Windows aborts the process on a panic before libFuzzer can persist the input. Print the
    // sequence first so that the exact failure can be replayed from the job log.
    if std::panic::catch_unwind(|| neo_j2k_fuzz::api_sequence(data)).is_err() {
        eprintln!("decoder API sequence: {data:?}");
        std::process::abort();
    }
});
