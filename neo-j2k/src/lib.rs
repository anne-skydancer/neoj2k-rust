// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! neo-j2k, a JPEG 2000 Part 1 codec: the native Rust twin of the C++ library in this repository,
//! transcribed module for module and bit-identical to it (rust/README.md).
//!
//! Stage R0 holds the scaffolding only: the C++ size table the byte ledger mirrors
//! (`mem::layout`). The header, decoder and encoder follow in stages R1 to R5.
//!
//! Native Rust: no build script, no C or C++ and no FFI in this crate or its dependencies, and no
//! `unsafe` outside the `simd` module (stage R3). The C++ is only the test oracle, linked by the
//! dev-dependency `neo-j2k-oracle`.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]

#[cfg(not(feature = "std"))]
compile_error!("neo-j2k needs the std feature in this version");

mod mem;
