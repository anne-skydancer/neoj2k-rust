// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! A native Rust implementation of the neo-j2k JPEG 2000 Part 1 codec.
//!
//! [`Decoder::decode`] returns planar samples from J2K, JP2 or JPH. [`Decoder::decode_into`]
//! writes into caller-owned buffers. [`Decoder::read_header`] checks limits and dimensions
//! before decoding the image. The `encode` feature enables the 8-bit viewer encoder.
//!
//! The codec has zero dependencies and uses the C++ library as a test oracle.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]

#[cfg(not(feature = "std"))]
compile_error!("neo-j2k needs the std feature in this version");

#[macro_use]
mod stage;
#[cfg(feature = "bench-internals")]
#[doc(hidden)]
pub mod bench;
mod decode;
mod decoder;
mod dwt;
#[cfg(feature = "encode")]
mod encoder;
mod error;
mod executor;
mod geometry;
mod header;
mod jp2;
mod limits;
mod mct;
mod mem;
#[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "sse"))]
#[allow(unsafe_code)]
mod simd;
mod stream;
mod t1;
mod t2;
mod tile;

pub use decode::{
    DecodeParams, DecodeStatistics, DecodeTarget, DecodedComponent, DecodedImage, I32Plane, Jp2Channels, OutputPlane,
    SampleType,
};
pub use decoder::{
    CodestreamCoding, ColorProfile, ColorSpace, ComponentCoding, ComponentInfo, DecodeStream, Decoder, DecoderConfig,
    Format, Info, Progression, QuantizationStep, QuantizationStyle, ReducePolicy, Wavelet,
};
#[cfg(feature = "encode")]
pub use encoder::{EncodeParams, Encoder, ImageSamples, ImageView, Plane};
pub use error::{Diagnostic, Error, ErrorKind};
pub use executor::Executor;
pub use limits::Limits;
pub use stream::StreamOptions;

/// The codec's full release version, including the patch number.
#[must_use]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// The codec's name and major/minor release line for display and logging.
/// A host that caches decoded samples can version those separately.
#[must_use]
pub const fn engine_string() -> &'static str {
    concat!(
        "neo-j2k ",
        env!("CARGO_PKG_VERSION_MAJOR"),
        ".",
        env!("CARGO_PKG_VERSION_MINOR")
    )
}
