// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Memory limits are charged against the C++ arenas' reservations and type layouts.

mod header;
mod layout;

pub(crate) use header::HeaderLedger;
pub(crate) use layout::CType;
