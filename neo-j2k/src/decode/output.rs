// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use super::DecodedComponent;
use crate::{ComponentInfo, Error, ErrorKind};

/// Format of the samples written into caller-owned buffers.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SampleType {
    /// Signed 32-bit samples in native byte order.
    #[default]
    I32,
    /// The low eight bits of each decoded sample.
    U8,
    /// The low sixteen bits of each decoded sample in native byte order.
    U16,
}

impl SampleType {
    const fn bytes(self) -> usize {
        match self {
            Self::I32 => 4,
            Self::U8 => 1,
            Self::U16 => 2,
        }
    }
}

/// The byte layout of one codestream component in a caller-owned buffer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutputPlane {
    pub component: u32,
    /// Byte offset of the first sample from the start of the buffer.
    pub offset: usize,
    /// Signed byte distance between two rows. Use a negative stride for bottom-up rows.
    pub row_stride: isize,
    /// Byte distance between two samples in the same row.
    pub sample_stride: usize,
}

/// A caller-owned buffer with planar or interleaved component layouts.
#[derive(Debug)]
pub struct DecodeTarget<'a> {
    pub bytes: &'a mut [u8],
    pub planes: &'a [OutputPlane],
}

pub(crate) fn validate(
    targets: &[DecodeTarget<'_>],
    sample: SampleType,
    components: &[ComponentInfo],
) -> Result<(), Error> {
    for target in targets {
        for plane in target.planes {
            let Some(component) = components.get(plane.component as usize) else {
                return Err(Error::new(ErrorKind::Argument, "Invalid output component index\n"));
            };
            if plane.sample_stride < sample.bytes() {
                return Err(Error::new(ErrorKind::Argument, "The output sample stride is too small\n"));
            }
            if component.width == 0 || component.height == 0 {
                continue;
            }
            let row_bytes = ((component.width - 1) as usize)
                .checked_mul(plane.sample_stride)
                .and_then(|width| width.checked_add(sample.bytes()))
                .ok_or_else(out_of_bounds)?;
            let last_row = row_start(*plane, (component.height - 1) as usize).ok_or_else(out_of_bounds)?;
            let end = last_row.max(plane.offset).checked_add(row_bytes).ok_or_else(out_of_bounds)?;
            if end > target.bytes.len() {
                return Err(out_of_bounds());
            }
        }
    }
    Ok(())
}

fn row_start(plane: OutputPlane, row: usize) -> Option<usize> {
    let movement = row.checked_mul(plane.row_stride.unsigned_abs())?;
    if plane.row_stride < 0 {
        plane.offset.checked_sub(movement)
    } else {
        plane.offset.checked_add(movement)
    }
}

pub(super) fn write(component: &DecodedComponent, sample: SampleType, buffer: &mut [u8], plane: OutputPlane) {
    let width = component.info.width as usize;
    if width == 0 {
        return;
    }
    for (y, row) in component.samples.chunks_exact(width).enumerate() {
        let start = row_start(plane, y).expect("validated output layout");
        let offsets = (start..buffer.len()).step_by(plane.sample_stride);
        match sample {
            SampleType::U8 => {
                for (offset, &value) in offsets.zip(row) {
                    buffer[offset] = value.to_le_bytes()[0];
                }
            }
            SampleType::U16 => {
                for (offset, &value) in offsets.zip(row) {
                    #[allow(clippy::cast_possible_truncation, reason = "keep the low sixteen sample bits")]
                    let low = value.cast_unsigned() as u16;
                    buffer[offset..offset + 2].copy_from_slice(&low.to_ne_bytes());
                }
            }
            SampleType::I32 => {
                for (offset, &value) in offsets.zip(row) {
                    buffer[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
                }
            }
        }
    }
}

fn out_of_bounds() -> Error {
    Error::new(ErrorKind::Output, "The output plane does not fit its buffer\n")
}
