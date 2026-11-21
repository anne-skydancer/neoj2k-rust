// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// The packet coder and rate allocation derive from OpenJPEG's t2.c, bio.c, tgt.c and tcd.c
// through src/enc_t2.cpp. See LICENSE.OpenJPEG for OpenJPEG's copyright notices and licence.

//! Packet generation and rate allocation for the one-tile RLCP encoder.

use crate::{Error, ErrorKind};

use super::{Band, Block, Pass, TileLayout, memory_error, resize_with};

mod coding;

use coding::{Bio, TagTree};

#[derive(Clone, Copy, Debug, Default)]
struct Layer {
    passes: usize,
    start: usize,
    end: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct BlockState {
    /// Number of passes assigned to prior layers by the rate allocator.
    assigned: usize,
    /// Number of passes emitted so far during this packet-writing attempt.
    emitted: usize,
    length_bits: u32,
}

#[derive(Debug, Default)]
struct BandState {
    inclusion: TagTree,
    bitplanes: TagTree,
}

#[derive(Debug, Default)]
pub(super) struct Workspace {
    layers: Vec<Layer>,
    blocks: Vec<BlockState>,
    bands: Vec<BandState>,
}

impl Workspace {
    fn prepare(&mut self, layout: &TileLayout, blocks: &[Block], count: usize) -> Result<(), Error> {
        let layers = blocks.len().checked_mul(count).ok_or_else(memory_error)?;
        resize_with(&mut self.layers, layers, Layer::default)?;
        self.layers.fill(Layer::default());
        resize_with(&mut self.blocks, blocks.len(), BlockState::default)?;
        self.blocks.fill(BlockState::default());
        resize_with(&mut self.bands, layout.bands.len(), BandState::default)?;
        for (band, state) in layout.bands.iter().zip(&mut self.bands) {
            state.inclusion.configure(band.block_columns, band.block_rows)?;
            state.bitplanes.configure(band.block_columns, band.block_rows)?;
        }
        Ok(())
    }

    /// Builds one layer with the given slope threshold and reports whether the allocation changed.
    #[allow(
        clippy::float_equality_without_abs,
        reason = "OpenJPEG accepts all slopes above the threshold"
    )]
    fn layer(&mut self, layer: usize, blocks: &[Block], threshold: f64, commit: bool) -> bool {
        let start = layer * blocks.len();
        let layers = &mut self.layers[start..start + blocks.len()];
        let mut same = true;
        for ((block, state), layer) in blocks.iter().zip(&mut self.blocks).zip(layers) {
            let assigned = state.assigned;
            let mut end = assigned;
            if threshold < 0.0 {
                end = block.passes.len();
            } else {
                for (index, pass) in block.passes.iter().enumerate().skip(assigned) {
                    let previous = end.checked_sub(1).map(|previous| &block.passes[previous]);
                    let rate = pass.rate.wrapping_sub(previous.map_or(0, |previous| previous.rate));
                    let distortion = pass.distortion_dec - previous.map_or(0.0, |previous| previous.distortion_dec);
                    if rate == 0 {
                        if distortion != 0.0 {
                            end = index + 1;
                        }
                    } else if threshold - distortion / f64::from(rate) < f64::EPSILON {
                        end = index + 1;
                    }
                }
            }
            let count = end - assigned;
            same &= layer.passes == count;
            layer.passes = count;
            if count != 0 {
                layer.start = assigned
                    .checked_sub(1)
                    .map_or(0, |index| byte_count(block.passes[index].rate));
                layer.end = byte_count(block.passes[end - 1].rate);
                if commit {
                    state.assigned = end;
                }
            }
        }
        same
    }

    fn write(
        &mut self,
        layout: &TileLayout,
        blocks: &[Block],
        layers: usize,
        dest: &mut [u8],
        final_pass: bool,
    ) -> Result<usize, Error> {
        let mut written = 0;
        for resolution in 0..layout.resolutions {
            for layer in 0..layers {
                let start = layer * blocks.len();
                let layers = &self.layers[start..start + blocks.len()];
                for component in 0..layout.components {
                    let offset = component * (3 * layout.resolutions - 2) + (3 * resolution).saturating_sub(2);
                    let bands = layout.bands(component, resolution);
                    let mut packet = Packet {
                        component,
                        resolution,
                        layer,
                        bands,
                        tags: &mut self.bands[offset..offset + bands.len()],
                        block_states: &mut self.blocks,
                        blocks,
                        layers,
                    };
                    written += packet.encode(&mut dest[written..], final_pass)?;
                }
            }
        }
        Ok(written)
    }
}

fn limiting_error() -> Error {
    Error::new(ErrorKind::Limit, "Cannot encode tile\n")
}

fn byte_count(rate: u32) -> usize {
    usize::try_from(rate).expect("the encoder's byte count fits usize")
}

fn min_max_slope(blocks: &[Block]) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = 0.0;
    for block in blocks {
        let (mut previous_rate, mut previous_distortion) = (0, 0.0);
        for pass in &block.passes {
            let rate = pass.rate.wrapping_sub(previous_rate).cast_signed();
            let distortion = pass.distortion_dec - previous_distortion;
            previous_rate = pass.rate;
            previous_distortion = pass.distortion_dec;
            if rate != 0 {
                let slope = distortion / f64::from(rate);
                if slope < min {
                    min = slope;
                }
                if slope > max {
                    max = slope;
                }
            }
        }
    }
    (min, max)
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "positive byte targets are rounded up and capped at the available buffer"
)]
fn rate_limit(rate: f32, dest_len: usize) -> usize {
    (rate.ceil() as usize).min(dest_len)
}

fn allocate_rates(workspace: &mut Workspace, layout: &TileLayout, blocks: &[Block], rates: &[f32], dest: &mut [u8]) {
    let (min, max) = min_max_slope(blocks);
    for (layer, &rate) in rates.iter().enumerate() {
        let good_threshold = if rate > 0.0 {
            let (mut lo, mut hi) = (min, max);
            let max_len = rate_limit(rate, dest.len());
            let (mut threshold, mut stable_threshold): (f64, f64) = (0.0, 0.0);
            let mut last_ok = false;
            for iteration in 0..128 {
                let next = (lo + hi) / 2.0;
                if (next - threshold).abs() <= 0.5 * 1e-5 * threshold {
                    break;
                }
                threshold = next;
                let same = workspace.layer(layer, blocks, threshold, false) && iteration != 0;
                let fits = if same {
                    last_ok
                } else {
                    workspace
                        .write(layout, blocks, layer + 1, &mut dest[..max_len], false)
                        .is_ok()
                };
                last_ok = fits;
                if fits {
                    hi = threshold;
                    stable_threshold = threshold;
                } else {
                    lo = threshold;
                }
            }
            if stable_threshold == 0.0 {
                threshold
            } else {
                stable_threshold
            }
        } else {
            -1.0
        };
        workspace.layer(layer, blocks, good_threshold, true);
    }
}

/// Allocates codeword passes to layers and writes the complete SOD body.
/// Rates are cumulative byte targets. A zero requests all remaining passes in that layer.
pub(super) fn encode_packets(
    workspace: &mut Workspace,
    layout: &TileLayout,
    blocks: &[Block],
    rates: &[f32],
    dest: &mut [u8],
) -> Result<usize, Error> {
    workspace.prepare(layout, blocks, rates.len())?;
    allocate_rates(workspace, layout, blocks, rates, dest);
    workspace.write(layout, blocks, rates.len(), dest, true)
}

struct Packet<'a> {
    component: usize,
    resolution: usize,
    layer: usize,
    bands: &'a [Band],
    tags: &'a mut [BandState],
    block_states: &'a mut [BlockState],
    blocks: &'a [Block],
    layers: &'a [Layer],
}

impl Packet<'_> {
    fn reset(&mut self) -> Result<(), Error> {
        for (band, tags) in self.bands.iter().zip(&mut *self.tags) {
            tags.inclusion.reset();
            tags.bitplanes.reset();
            for (leaf, index) in band.blocks.clone().enumerate() {
                self.block_states[index].emitted = 0;
                let numbps = self.blocks[index].numbps;
                let bitplanes = numbps.cast_signed();
                if band.numbps < bitplanes {
                    return Err(Error::new(
                        ErrorKind::Limit,
                        format!(
                            "opj_t2_encode_packet(): a code-block of component {}, resolution {} needs {numbps} bit planes, \
                         its band holds {} with the core encoder's 2 guard bits: the image does not fit \
                         (more guard bits, GUARD_BITS, belong to the j2k-extra module)\n",
                            self.component, self.resolution, band.numbps,
                        ),
                    ));
                }
                tags.bitplanes.set_value(leaf, band.numbps - bitplanes);
            }
        }
        Ok(())
    }

    fn encode(&mut self, dest: &mut [u8], final_pass: bool) -> Result<usize, Error> {
        if self.layer == 0 {
            self.reset()?;
        }
        let layer = i32::try_from(self.layer).expect("the validated number of layers fits i32");
        let mut bio = Bio::new(dest);
        // As in the C++ encoder, every packet has its present bit set.
        bio.put(true);
        for (band, tags) in self.bands.iter().zip(&mut *self.tags) {
            for (leaf, index) in band.blocks.clone().enumerate() {
                if self.block_states[index].emitted == 0 && self.layers[index].passes != 0 {
                    tags.inclusion.set_value(leaf, layer);
                }
            }
            for (leaf, index) in band.blocks.clone().enumerate() {
                let state = &mut self.block_states[index];
                let count = self.layers[index].passes;
                if state.emitted == 0 {
                    tags.inclusion.write(&mut bio, leaf, layer + 1);
                } else {
                    bio.put(count != 0);
                }
                if count == 0 {
                    continue;
                }
                if state.emitted == 0 {
                    state.length_bits = 3;
                    tags.bitplanes.write(&mut bio, leaf, 999);
                }
                bio.passes(u32::try_from(count).map_err(|_| limiting_error())?);
                let first = state.emitted;
                let passes = &self.blocks[index].passes[first..first + count];
                let increment = segment_lengths(passes)
                    .map(|(length, passes)| {
                        let want = signed_log2(length) + 1;
                        want.saturating_sub(state.length_bits + signed_log2(passes))
                    })
                    .max()
                    .unwrap_or(0);
                bio.comma(increment);
                state.length_bits += increment;
                for (length, count) in segment_lengths(passes) {
                    bio.write(length, state.length_bits + signed_log2(count));
                }
            }
        }
        let mut written = bio.finish().ok_or_else(limiting_error)?;
        for band in self.bands {
            for index in band.blocks.clone() {
                let layer = self.layers[index];
                if layer.passes == 0 {
                    continue;
                }
                let length = layer.end - layer.start;
                let available = dest.len() - written;
                if length > available {
                    if final_pass {
                        return Err(Error::new(
                            ErrorKind::Limit,
                            format!(
                                "opj_t2_encode_packet(): only {available} bytes remaining in output buffer. {length} needed.\n",
                            ),
                        ));
                    }
                    return Err(limiting_error());
                }
                if final_pass {
                    dest[written..written + length].copy_from_slice(&self.blocks[index].data[layer.start..layer.end]);
                }
                self.block_states[index].emitted += layer.passes;
                written += length;
            }
        }
        Ok(written)
    }
}

fn signed_log2(value: u32) -> u32 {
    let signed = value.cast_signed();
    if signed > 1 { signed.unsigned_abs().ilog2() } else { 0 }
}

fn segment_lengths(mut remaining: &[Pass]) -> impl Iterator<Item = (u32, u32)> + '_ {
    std::iter::from_fn(move || {
        if remaining.is_empty() {
            return None;
        }
        let count = remaining
            .iter()
            .position(|pass| pass.terminated)
            .map_or(remaining.len(), |end| end + 1);
        let (segment, next) = remaining.split_at(count);
        let length = segment.iter().fold(0u32, |total, pass| total.wrapping_add(pass.len));
        remaining = next;
        Some((length, u32::try_from(count).expect("a code-block pass count fits u32")))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_layout() -> TileLayout {
        TileLayout {
            components: 1,
            resolutions: 1,
            bands: vec![Band {
                orientation: 0,
                numbps: 8,
                block_columns: 1,
                block_rows: 1,
                blocks: 0..1,
                x: 0,
                y: 0,
                width: 2,
                height: 2,
                step_size: 1.0,
            }],
        }
    }

    #[test]
    fn lossless_packet_and_later_empty_layer_match_the_packet_codes() {
        let layout = sample_layout();
        let blocks = [Block {
            numbps: 8,
            data: vec![0xaa, 0xbb],
            passes: vec![Pass {
                rate: 2,
                len: 2,
                distortion_dec: 6.0,
                terminated: true,
            }],
        }];
        let mut workspace = Workspace::default();
        let mut output = [0; 4];
        let n = encode_packets(&mut workspace, &layout, &blocks, &[0.0, 0.0], &mut output).unwrap();
        assert_eq!(n, 4);
        assert_eq!(output, [0xe2, 0xaa, 0xbb, 0x80]);

        let error = encode_packets(&mut workspace, &layout, &blocks, &[0.0], &mut output[..2]).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Limit);
        assert!(
            error
                .message()
                .contains("only 1 bytes remaining in output buffer. 2 needed.")
        );
    }

    #[test]
    fn rate_search_rounds_up_the_byte_target_and_preserves_the_remaining_pass() {
        let layout = sample_layout();
        let blocks = [Block {
            numbps: 8,
            data: (0x11..=0x1a).collect(),
            passes: vec![
                Pass {
                    rate: 2,
                    len: 2,
                    distortion_dec: 10.0,
                    terminated: false,
                },
                Pass {
                    rate: 10,
                    len: 8,
                    distortion_dec: 11.0,
                    terminated: true,
                },
            ],
        }];
        let mut workspace = Workspace::default();
        let mut output = [0; 20];
        let n = encode_packets(&mut workspace, &layout, &blocks, &[2.1, 0.0], &mut output).unwrap();
        assert_eq!(
            &output[..n],
            [
                0xe2, 0x11, 0x12, 0xd4, 0, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a
            ]
        );
    }
}
