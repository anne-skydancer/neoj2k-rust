// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Memory limits are charged against the C++ arenas' reservations and type layouts.

mod header;
mod layout;
mod work;

pub(crate) use header::HeaderLedger;
pub(crate) use layout::CType;
pub(crate) use work::WorkBudget;

pub(crate) fn capacity_bytes<T>(storage: &Vec<T>) -> u64 {
    storage.capacity() as u64 * size_of::<T>() as u64
}
