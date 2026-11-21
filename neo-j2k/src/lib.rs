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
#[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
#[allow(unsafe_code)]
mod simd;
mod t1;
mod t2;
mod tile;

pub use decode::{
    DecodeParams, DecodeStatistics, DecodeTarget, DecodedComponent, DecodedImage, Jp2Channels, OutputPlane, SampleType,
};
pub use decoder::{
    CodestreamCoding, ColorProfile, ColorSpace, ComponentCoding, ComponentInfo, Decoder, DecoderConfig, Format, Info,
    Progression, QuantizationStep, QuantizationStyle, ReducePolicy, Wavelet,
};
#[cfg(feature = "encode")]
pub use encoder::{EncodeParams, Encoder, ImageSamples, ImageView, Plane};
pub use error::{Diagnostic, Error, ErrorKind};
pub use executor::Executor;
pub use limits::Limits;
