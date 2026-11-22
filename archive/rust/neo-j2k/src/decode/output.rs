// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use super::DecodedComponent;
#[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
use crate::simd::output as vector;
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

/// A writable plane of contiguous, top-down I32 samples.
#[derive(Debug)]
pub struct I32Plane<'a> {
    /// The original codestream component index.
    pub component: u32,
    /// Storage for at least `width * height` samples. Extra samples are left untouched.
    pub samples: &'a mut [i32],
}

pub(crate) fn validate(
    targets: &[DecodeTarget<'_>],
    sample: SampleType,
    components: &[ComponentInfo],
    slots: &[Option<usize>],
) -> Result<(), Error> {
    let planes = targets
        .iter()
        .flat_map(|target| target.planes.iter().map(|plane| (plane, target.bytes.len())));
    for (index, (plane, len)) in planes.enumerate() {
        let component = plane.component as usize;
        if plane.sample_stride < sample.bytes() || slots.get(component).is_none_or(Option::is_none) {
            return Err(Error::new(
                ErrorKind::Output,
                format!("nj2k_decode_memory: plane {index} does not fit the image\n"),
            ));
        }
        let component = &components[component];
        if component.width == 0 || component.height == 0 {
            continue;
        }
        let row_bytes = ((component.width - 1) as usize)
            .checked_mul(plane.sample_stride)
            .and_then(|width| width.checked_add(sample.bytes()))
            .ok_or_else(out_of_bounds)?;
        let last_row = row_start(*plane, (component.height - 1) as usize).ok_or_else(out_of_bounds)?;
        let end = last_row
            .max(plane.offset)
            .checked_add(row_bytes)
            .ok_or_else(out_of_bounds)?;
        if end > len {
            return Err(out_of_bounds());
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

/// Write a one- to four-channel U8 target in one pass. Other layouts fall back to plane-by-plane
/// writes, preserving their write order where rows or planes overlap.
pub(super) fn write_interleaved(
    components: &[DecodedComponent],
    slots: &[Option<usize>],
    sample: SampleType,
    target: &mut DecodeTarget<'_>,
) -> bool {
    sample == SampleType::U8 && interleaved(components, slots, target).is_some()
}

fn interleaved(components: &[DecodedComponent], slots: &[Option<usize>], target: &mut DecodeTarget<'_>) -> Option<()> {
    let channels = target.planes.len();
    if !(1..=4).contains(&channels) {
        return None;
    }
    let first = *target.planes.iter().min_by_key(|plane| plane.offset)?;
    let source = |plane: OutputPlane| {
        let slot = slots.get(usize::try_from(plane.component).ok()?)?.as_ref()?;
        components.get(*slot)
    };
    let first_source = source(first)?;
    let width = usize::try_from(first_source.info.width).ok()?;
    let height = usize::try_from(first_source.info.height).ok()?;
    let count = width.checked_mul(height)?;
    let mut sources = [None; 4];
    for &plane in target.planes {
        let index = plane.offset - first.offset;
        if index >= channels || plane.sample_stride != channels || plane.row_stride != first.row_stride {
            return None;
        }
        let component = source(plane)?;
        if component.info.width != first_source.info.width
            || component.info.height != first_source.info.height
            || component.samples.len() != count
            || sources[index].replace(component.samples.as_slice()).is_some()
        {
            return None;
        }
    }
    if width == 0 || height == 0 {
        return Some(());
    }
    let row_bytes = width.checked_mul(channels)?;
    if height > 1 && first.row_stride.unsigned_abs() < row_bytes {
        return None;
    }
    let last = row_start(first, height - 1)?;
    let end = first.offset.max(last).checked_add(row_bytes)?;
    if end > target.bytes.len() {
        return None;
    }

    let [a, b, c, d] = sources.map(Option::unwrap_or_default);
    match channels {
        1 => interleaved_rows([a], width, height, first, target.bytes),
        2 => interleaved_rows([a, b], width, height, first, target.bytes),
        3 => interleaved_rows([a, b, c], width, height, first, target.bytes),
        4 => interleaved_rows([a, b, c, d], width, height, first, target.bytes),
        _ => unreachable!("the channel count was validated"),
    }
    Some(())
}

fn interleaved_rows<const CHANNELS: usize>(
    sources: [&[i32]; CHANNELS],
    width: usize,
    height: usize,
    first: OutputPlane,
    bytes: &mut [u8],
) {
    for y in 0..height {
        let input = y * width;
        let rows = sources.map(|plane| &plane[input..input + width]);
        let output = row_start(first, y).expect("validated interleaved row");
        let dest = &mut bytes[output..output + width * CHANNELS];
        #[cfg(not(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2")))]
        let pixels_done = 0;
        #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "avx2"))]
        let pixels_done = vector::write(&rows, dest);
        for (x, pixel) in dest[pixels_done * CHANNELS..]
            .as_chunks_mut::<CHANNELS>()
            .0
            .iter_mut()
            .enumerate()
        {
            for (sample, row) in pixel.iter_mut().zip(rows) {
                *sample = row[pixels_done + x].to_le_bytes()[0];
            }
        }
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
            SampleType::I32 if plane.sample_stride == size_of::<i32>() => {
                let end = start + size_of_val(row);
                let (packed, _) = buffer[start..end].as_chunks_mut::<{ size_of::<i32>() }>();
                for (bytes, &value) in packed.iter_mut().zip(row) {
                    *bytes = value.to_ne_bytes();
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

#[cfg(test)]
mod tests {
    use super::*;

    fn components(width: u32, height: u32) -> [DecodedComponent; 4] {
        std::array::from_fn(|component| DecodedComponent {
            info: ComponentInfo {
                width,
                height,
                ..ComponentInfo::default()
            },
            samples: (0..width * height)
                .map(|pixel| {
                    let salt = u32::try_from(component + 1).unwrap();
                    (pixel.wrapping_mul(0x9e37_79b9) ^ salt.wrapping_mul(0x712a_019d)).cast_signed()
                })
                .collect(),
            ..DecodedComponent::default()
        })
    }

    #[test]
    fn packed_i32_rows_allow_unaligned_bottom_up_output_and_padding() {
        let [component, ..] = components(3, 2);
        let mut bytes = [0xcd; 33];
        let plane = OutputPlane {
            component: 0,
            offset: 19,
            row_stride: -16,
            sample_stride: 4,
        };
        write(&component, SampleType::I32, &mut bytes, plane);

        let encoded = |row: &[i32]| row.iter().flat_map(|sample| sample.to_ne_bytes()).collect::<Vec<_>>();
        assert_eq!(bytes[3..15], encoded(&component.samples[3..]));
        assert_eq!(bytes[19..31], encoded(&component.samples[..3]));
        assert_eq!(bytes[..3], [0xcd; 3]);
        assert_eq!(bytes[15..19], [0xcd; 4]);
        assert_eq!(bytes[31..], [0xcd; 2]);
    }

    #[test]
    fn interleaved_targets_match_plane_writes_at_every_channel_count() {
        let slots = [Some(2), Some(0), Some(3), Some(1)];
        for width in [1_u32, 7, 8, 9, 10, 15, 16, 17, 63] {
            for height in [1_u32, 3] {
                let components = components(width, height);
                let width = usize::try_from(width).unwrap();
                let height = usize::try_from(height).unwrap();
                for channels in 1..=4 {
                    for flip in [false, true] {
                        for padding in [0, 13] {
                            let stride = width * channels + padding;
                            let signed_stride = isize::try_from(stride).unwrap() * if flip { -1 } else { 1 };
                            let first = 5 + if flip { stride * (height - 1) } else { 0 };
                            // The descriptor order need not match the order of bytes in a pixel.
                            let planes: Vec<_> = (0..channels)
                                .rev()
                                .map(|channel| OutputPlane {
                                    component: u32::try_from(channel).unwrap(),
                                    offset: first + channel,
                                    row_stride: signed_stride,
                                    sample_stride: channels,
                                })
                                .collect();
                            let len = 5 + stride * height + 11;
                            let mut fast = vec![0xb6; len];
                            let mut fallback = fast.clone();
                            for plane in &planes {
                                let slot = slots[usize::try_from(plane.component).unwrap()].unwrap();
                                write(&components[slot], SampleType::U8, &mut fallback, *plane);
                            }
                            assert!(write_interleaved(
                                &components,
                                &slots,
                                SampleType::U8,
                                &mut DecodeTarget {
                                    bytes: &mut fast,
                                    planes: &planes
                                },
                            ));
                            assert_eq!(
                                fast, fallback,
                                "{width}x{height}x{channels}, flip={flip}, padding={padding}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn incompatible_targets_are_rejected_before_any_byte_changes() {
        let mut components = components(7, 2);
        let slots = [Some(0), Some(1), None];
        let first = OutputPlane {
            component: 0,
            offset: 0,
            row_stride: 20,
            sample_stride: 2,
        };
        let second = OutputPlane {
            component: 1,
            offset: 1,
            ..first
        };
        let mut bytes = [0x8b; 48];
        for (sample, planes) in [
            (SampleType::U16, [first, second]),
            (
                SampleType::U8,
                [
                    first,
                    OutputPlane {
                        sample_stride: 3,
                        ..second
                    },
                ],
            ),
            (
                SampleType::U8,
                [
                    first,
                    OutputPlane {
                        row_stride: 21,
                        ..second
                    },
                ],
            ),
            (SampleType::U8, [first, OutputPlane { offset: 2, ..second }]),
            (SampleType::U8, [first, OutputPlane { offset: 0, ..second }]),
            (SampleType::U8, [first, OutputPlane { component: 2, ..second }]),
            (
                SampleType::U8,
                [
                    OutputPlane {
                        row_stride: 13,
                        ..first
                    },
                    OutputPlane {
                        row_stride: 13,
                        ..second
                    },
                ],
            ),
            (
                SampleType::U8,
                [
                    OutputPlane {
                        row_stride: -20,
                        ..first
                    },
                    OutputPlane {
                        row_stride: -20,
                        ..second
                    },
                ],
            ),
            (
                SampleType::U8,
                [
                    OutputPlane { offset: 15, ..first },
                    OutputPlane { offset: 16, ..second },
                ],
            ),
        ] {
            assert!(!write_interleaved(
                &components,
                &slots,
                sample,
                &mut DecodeTarget {
                    bytes: &mut bytes,
                    planes: &planes
                }
            ));
            assert_eq!(bytes, [0x8b; 48]);
        }
        components[1].info.height = 1;
        let planes = [first, second];
        assert!(!write_interleaved(
            &components,
            &slots,
            SampleType::U8,
            &mut DecodeTarget {
                bytes: &mut bytes,
                planes: &planes
            }
        ));
        assert_eq!(bytes, [0x8b; 48]);
    }
}
