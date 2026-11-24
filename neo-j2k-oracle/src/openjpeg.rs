// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! OpenJPEG's own API at the oracle's pin (OpenJPEG 2.5.4), as the timing of the Rust twin against
//! OpenJPEG calls it (`neo-j2k/tests/vs_openjpeg.rs`; the shim's `shim/opj_shim.h`).
//!
//! OpenJPEG keeps no codec between decodes: every [`Decoder::decode`] creates and destroys a codec
//! and a memory stream, and OpenJPEG allocates the image's sample planes itself. Threads are
//! OpenJPEG's own (`opj_codec_set_threads`: a pool of that many worker threads made by every
//! decode), not the host executor the Rust decoder borrows.

use std::ffi::CStr;
use std::ptr::NonNull;

use crate::{ffi, init};

/// What a failed call of OpenJPEG returns: the shim's status (1 codec setup, 2 header, 3 decode,
/// 4 output too small for a decode, for an encode 1 bad request, 2 allocation, 3 setup, 4 encode,
/// 5 exception).
pub type Status = i32;

/// OpenJPEG's version text (`opj_version`).
#[must_use]
pub fn version() -> String {
    init();
    // SAFETY: no arguments; the result is a static NUL-terminated string.
    let text = unsafe { ffi::nj2k_opj_version() };
    if text.is_null() {
        return String::new();
    }
    // SAFETY: non-null and NUL-terminated, valid for the whole run.
    unsafe { CStr::from_ptr(text) }.to_string_lossy().into_owned()
}

/// The settings of OpenJPEG's J2K decodes: the reduce factor and the thread count.
#[derive(Debug)]
pub struct Decoder(NonNull<ffi::OpjDecoder>);

// SAFETY: the handle holds two integers and no thread affinity; `&mut self` serves one call at a time.
unsafe impl Send for Decoder {}

impl Decoder {
    /// A decoder at a reduce factor. `threads` of 0 or 1 decodes serially; more asks every decode for
    /// that many OpenJPEG worker threads.
    #[must_use]
    pub fn new(reduce: u32, threads: u32) -> Option<Self> {
        init();
        // SAFETY: plain values in; the result is a new handle or null.
        NonNull::new(unsafe { ffi::nj2k_opj_decoder_new(reduce, threads) }).map(Self)
    }

    /// One decode in non-strict mode, nothing copied out: the samples the image holds (all components),
    /// or OpenJPEG's status. The image is destroyed before the call returns.
    pub fn decode(&mut self, data: &[u8]) -> Result<u64, Status> {
        self.run(data, None)
    }

    /// One decode whose planes are copied, one after the other, into `out`: the sample count. Fails
    /// with status 4 when `out` is too small.
    pub fn decode_into(&mut self, data: &[u8], out: &mut [i32]) -> Result<u64, Status> {
        self.run(data, Some(out))
    }

    fn run(&mut self, data: &[u8], out: Option<&mut [i32]>) -> Result<u64, Status> {
        let mut count = 0u64;
        let (ptr, cap) = out.map_or((std::ptr::null_mut(), 0), |out| (out.as_mut_ptr(), out.len() as u64));
        // SAFETY: the handle is live; data is valid for its length; out is null with cap 0, or valid for cap
        // samples; count is a writable local. The shim keeps none of them after the call.
        let status = unsafe {
            ffi::nj2k_opj_decode(
                self.0.as_ptr(),
                data.as_ptr(),
                data.len() as u64,
                ptr,
                cap,
                &raw mut count,
            )
        };
        if status == 0 { Ok(count) } else { Err(status) }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        // SAFETY: the handle came from nj2k_opj_decoder_new and is freed once.
        unsafe { ffi::nj2k_opj_decoder_free(self.0.as_ptr()) };
    }
}

/// One OpenJPEG encode of an eight-bit interleaved image: reversible or not, `resolutions` levels,
/// the colour transform or not; one quality layer with no rate limit, 64 by 64 code-blocks, RLCP,
/// OpenJPEG's own default comment.
#[derive(Clone, Copy, Debug)]
pub struct EncodeRequest<'a> {
    pub pixels: &'a [u8],
    pub row_stride: usize,
    pub width: u32,
    pub height: u32,
    pub components: u32,
    pub reversible: bool,
    pub resolutions: u32,
    pub mct: bool,
}

struct Bytes(NonNull<ffi::OpjBytes>);

impl Bytes {
    fn as_slice(&self) -> &[u8] {
        let mut len = 0;
        // SAFETY: the handle is live and len is a writable local.
        let data = unsafe { ffi::nj2k_opj_bytes_data(self.0.as_ptr(), &raw mut len) };
        if len == 0 || data.is_null() {
            return &[];
        }
        // SAFETY: the shim's buffer holds len initialized bytes for as long as the handle lives, which the
        // returned borrow of self covers.
        unsafe { std::slice::from_raw_parts(data, len) }
    }
}

impl Drop for Bytes {
    fn drop(&mut self) {
        // SAFETY: the handle came from nj2k_opj_encode_u8 and is freed once.
        unsafe { ffi::nj2k_opj_bytes_free(self.0.as_ptr()) };
    }
}

/// The number of bytes the shim reads: `height` rows of `width * components` bytes, a row stride apart (the last
/// row has no padding after it). `None` when a size does not fit in `usize`, the stride is shorter than a row, or
/// there is no row: the arithmetic is checked, so a huge stride cannot wrap to a small count.
fn bytes_read(row_stride: usize, width: u32, height: u32, components: u32) -> Option<usize> {
    let row = usize::try_from(width)
        .ok()?
        .checked_mul(usize::try_from(components).ok()?)?;
    if row_stride < row || height == 0 {
        return None;
    }
    usize::try_from(height - 1)
        .ok()?
        .checked_mul(row_stride)?
        .checked_add(row)
}

fn encode(request: &EncodeRequest<'_>) -> Result<Bytes, Status> {
    init();
    let mut out = std::ptr::null_mut();
    let needed = bytes_read(request.row_stride, request.width, request.height, request.components);
    assert!(
        needed.is_some_and(|needed| request.pixels.len() >= needed),
        "the pixels hold the image"
    );
    // SAFETY: the pixels hold every byte the shim reads (checked above); out is a writable local; the
    // shim keeps no pointer to the pixels after the call.
    let status = unsafe {
        ffi::nj2k_opj_encode_u8(
            request.pixels.as_ptr(),
            request.row_stride,
            request.width,
            request.height,
            request.components,
            u32::from(request.reversible),
            request.resolutions,
            u32::from(request.mct),
            &raw mut out,
        )
    };
    if status != 0 {
        return Err(status);
    }
    NonNull::new(out).map(Bytes).ok_or(2)
}

/// The codestream OpenJPEG writes.
pub fn encode_u8(request: &EncodeRequest<'_>) -> Result<Vec<u8>, Status> {
    encode(request).map(|bytes| bytes.as_slice().to_vec())
}

/// One encode for timing: the codestream's length, with no copy of it out of OpenJPEG's buffer.
pub fn encode_u8_len(request: &EncodeRequest<'_>) -> Result<usize, Status> {
    encode(request).map(|bytes| bytes.as_slice().len())
}

#[cfg(test)]
mod tests {
    use super::bytes_read;

    #[test]
    fn the_bytes_read_are_the_rows_without_the_last_padding() {
        assert_eq!(bytes_read(10, 3, 4, 3), Some(3 * 10 + 9));
        assert_eq!(bytes_read(9, 3, 1, 3), Some(9));
    }

    #[test]
    fn a_stride_shorter_than_a_row_or_no_rows_is_refused() {
        assert_eq!(bytes_read(8, 3, 4, 3), None);
        assert_eq!(bytes_read(9, 3, 0, 3), None);
    }

    #[test]
    fn a_huge_stride_cannot_wrap_to_a_small_count() {
        // twice the stride wraps to zero on a 64-bit target: the count must be refused, not 1
        assert_eq!(bytes_read(1usize << 63, 1, 3, 1), None);
        assert_eq!(bytes_read(usize::MAX, 1, 2, 1), None);
    }
}
