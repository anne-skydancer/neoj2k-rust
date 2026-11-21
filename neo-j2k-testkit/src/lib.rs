// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The test kit of neo-j2k's Rust twin: the Rust counterpart of the C++ harness in `tests/common`,
//! transcribed module for module so that the twin's tests drive both products the same way.
//!
//! - [`request`]: the harness's requests, modes and outcomes ([`request::Backend`] is what the C++
//!   oracle crate and, later, the twin implement);
//! - [`viewer`]: the Second Life viewer's rules the requests follow;
//! - [`fnv`], [`digest`], [`reference`](mod@reference): `nj2k_diff`'s identity hashes, cut digests and reference
//!   tables;
//! - [`scan`]: the codestream scanner (`j2k_scan`), with a text summary both languages write;
//! - [`pgx`]: the PGX reader of the conformance images;
//! - [`pool`]: a test executor, `std::thread::scope` based, whose caller runs jobs too;
//! - [`files`]: inputs, the repository's folders and the tests' temporary folder.
//!
//! Pure Rust with no dependencies and no `unsafe`, so Miri runs everything here.

#![forbid(unsafe_code)]

pub mod digest;
pub mod files;
pub mod fnv;
pub mod pgx;
pub mod pool;
pub mod reference;
pub mod request;
pub mod scan;
pub mod viewer;
