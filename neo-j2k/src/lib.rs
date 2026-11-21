// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! neo-j2k, a JPEG 2000 Part 1 codec: the native Rust twin of the C++ library in this repository,
//! transcribed module for module and bit-identical to it (rust/README.md).
//!
//! Read JPEG 2000 codestreams and JP2 headers with [`Decoder::read_header`] and check
//! decoded dimensions and resource limits with [`Decoder::decoded_dimensions`].
//!
//! Native Rust: no build script, no C or C++ and no FFI in this crate or its dependencies, and no
//! `unsafe` outside the `simd` module (stage R3). The C++ is only the test oracle, linked by the
//! dev-dependency `neo-j2k-oracle`.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]

#[cfg(not(feature = "std"))]
compile_error!("neo-j2k needs the std feature in this version");

mod decode;
mod decoder;
mod dwt;
#[cfg(feature = "encode")]
mod encoder;
mod error;
mod geometry;
mod header;
mod jp2;
mod limits;
mod mct;
mod mem;
mod t1;
mod t2;
mod tile;

pub use decode::{
    DecodeParams, DecodeStatistics, DecodeTarget, DecodedComponent, DecodedImage, Jp2Channels, OutputPlane, SampleType,
};
#[cfg(feature = "encode")]
pub use encoder::{EncodeParams, Encoder, ImageSamples, ImageView, Plane};
pub use decoder::{
    ColorProfile, ColorSpace, ComponentInfo, Decoder, DecoderConfig, Format, Info, Progression, ReducePolicy,
};
pub use error::{Diagnostic, Error, ErrorKind};
pub use limits::Limits;
