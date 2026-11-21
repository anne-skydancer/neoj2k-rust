// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Memory: the C++ arenas' accounting, which the decoder limits' refusals are decided on (plan
//! section 4.4). Stage R0 holds the size table; R1 adds the header arena's ledger, R2 the work
//! arena's.

pub(crate) mod layout;
