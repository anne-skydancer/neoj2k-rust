// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The native memory decoder's tile walk and output assembly.

mod output;
mod walk;

pub(crate) use output::validate as validate_targets;
pub use output::{DecodeTarget, OutputPlane, SampleType};

use crate::error::Diagnostics;
use crate::geometry::{self, Rect, ceil_div_pow2};
use crate::header::{MainHeader, tile::{TileComponentParams, TileOverrides, resolve_tile, resolve_tile_component}};
use crate::mem::{CType, HeaderLedger, WorkBudget};
use crate::t1::{self, Destination};
use crate::t2::{self, PacketInput};
use crate::tile::{Tile, TileComp, resize};
use crate::{ComponentInfo, Error, ErrorKind, Limits, dwt, mct};
use walk::{Pages, Walk, data_error, need_data};

/// Options for one decode. A zero layer limit decodes every quality layer.
#[derive(Clone, Copy, Debug, Default)]
pub struct DecodeParams<'a> {
    pub max_layers: u32,
    pub strict: bool,
    /// Original component indices. Supplying a selection disables the colour transform.
    pub components: Option<&'a [u32]>,
    /// How to handle a JP2 palette or channel definitions when decoding every component.
    pub jp2_channels: Jp2Channels,
}

/// How to handle JP2 container metadata that changes the decoded component list.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Jp2Channels {
    /// Return an unsupported error when the components would change.
    #[default]
    Reject,
    /// Apply palette and channel definitions to the decoder's planar output.
    Apply,
    /// Return the codestream components directly.
    Ignore,
}

/// One decoded component, in row-major order.
#[derive(Debug, Default)]
pub struct DecodedComponent {
    pub(crate) info: ComponentInfo,
    pub(crate) samples: Vec<i32>,
    pub(crate) resolution: u32,
    pub(crate) alpha: u16,
}

impl DecodedComponent {
    /// The component's geometry and sample representation.
    #[must_use]
    pub const fn info(&self) -> ComponentInfo {
        self.info
    }

    /// Its decoded samples, in row-major order.
    #[must_use]
    pub fn samples(&self) -> &[i32] {
        &self.samples
    }

    /// The highest resolution reached by the available packets of the last tile.
    #[must_use]
    pub const fn resolution(&self) -> u32 {
        self.resolution
    }

    /// The JP2 channel definition's alpha type, or zero for a colour channel.
    #[must_use]
    pub const fn alpha(&self) -> u16 {
        self.alpha
    }
}

/// The outcome of a decode, borrowed from the reusable decoder.
#[derive(Debug)]
pub struct DecodedImage<'a> {
    /// The decoded planes in the requested component order.
    pub components: &'a [DecodedComponent],
    /// The decoder reached the end of the available bytes before the end of the codestream.
    pub truncated: bool,
}

/// Work arena accounting as reported by the C++ memory decoder.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DecodeStatistics {
    pub header_reserved: u64,
    pub tile_reserved: u64,
    pub work_reserved: u64,
    pub work_high_water: u64,
}

pub(crate) struct Input<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) header: &'a MainHeader,
    pub(crate) components: &'a [ComponentInfo],
    pub(crate) reduce: u32,
    pub(crate) limits: Limits,
    pub(crate) retain: u64,
}

#[derive(Debug, Default)]
pub(crate) struct Workspace {
    pub(crate) tile_ledger: HeaderLedger,
    pub(crate) budget: WorkBudget,
    pages: Pages,
    tile: Tile,
    tier1: t1::Scratch,
    tier2: t2::Workspace,
    wavelet: dwt::Workspace,
    packed: Vec<u8>,
    combined: Vec<u8>,
    image: Vec<DecodedComponent>,
    pub(crate) selection: Vec<u32>,
    slot: Vec<Option<usize>>,
    written: Vec<bool>,
    pub(crate) truncated: bool,
}

impl Workspace {
    pub(crate) fn image(&self) -> DecodedImage<'_> {
        DecodedImage {
            components: &self.image,
            truncated: self.truncated,
        }
    }

    pub(crate) fn slots(&self) -> &[Option<usize>] {
        &self.slot
    }

    pub(crate) fn image_mut(&mut self) -> &mut Vec<DecodedComponent> {
        &mut self.image
    }

    pub(crate) fn write_targets(&self, sample: SampleType, targets: &mut [DecodeTarget<'_>]) {
        for target in targets {
            for plane in target.planes {
                if let Some(slot) = self.slot[plane.component as usize] {
                    output::write(&self.image[slot], sample, target.bytes, *plane);
                }
            }
        }
    }

    pub(crate) fn reset_selection(&mut self, count: usize, selection: Option<&[u32]>) -> Result<(), Error> {
        resize(&mut self.slot, count)?;
        self.slot.fill(None);
        self.selection.clear();
        self.selection
            .try_reserve_exact(selection.map_or(count, <[u32]>::len))
            .map_err(|_| no_memory())?;
        if let Some(indices) = selection {
            for &component in indices {
                let Some(slot) = self.slot.get_mut(component as usize) else {
                    let message = format!("Invalid component index: {component}\n");
                    return Err(Error::new(ErrorKind::Argument, message));
                };
                if slot.is_some() {
                    let message = format!("Component index {component} used several times\n");
                    return Err(Error::new(ErrorKind::Argument, message));
                }
                *slot = Some(self.selection.len());
                self.selection.push(component);
            }
        } else {
            for (component, slot) in (0_u32..).zip(&mut self.slot) {
                *slot = Some(component as usize);
                self.selection.push(component);
            }
        }
        Ok(())
    }

    pub(crate) fn decode(
        &mut self,
        input: Input<'_>,
        params: DecodeParams<'_>,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Error> {
        let main = input.header;
        // Account for the previous decode's retained working memory before charging this one.
        self.budget.reset(input.retain, input.limits.max_memory);
        self.tile_ledger.reset(input.retain, input.limits.max_memory);
        self.budget.array(CType::OutComp, main.comps.len())?;
        let single = single_tile_image(main);
        self.prepare_image(input.components, single)?;
        self.truncated = false;

        let mut walk = Walk::new(input.data, main, &mut self.pages, input.limits, input.reduce, params.strict);
        let mut ppm = (!main.ppm_data.is_empty()).then_some(main.ppm_data.as_slice());
        let mut decoded = 0;
        while let Some(index) = walk.next_tile(&mut self.budget, &mut self.tile_ledger, diagnostics)? {
            let mark = self.budget.mark();
            let work = TileDecode {
                header: main,
                reduce: input.reduce,
                limits: input.limits,
                index,
                params,
            };
            let mut target = TileOutput {
                tile: &mut self.tile,
                tier1: &mut self.tier1,
                tier2: &mut self.tier2,
                wavelet: &mut self.wavelet,
                packed: &mut self.packed,
                combined: &mut self.combined,
                budget: &mut self.budget,
                slot: &self.slot,
                image: &mut self.image,
                written: &mut self.written,
            };
            let result = target.decode(work, &mut walk, ppm.as_mut(), diagnostics);
            self.budget.rewind(mark);
            if let Err(error) = result {
                diagnostics.error(error.message());
                let message = format!("Failed to decode tile {}/{}\n", index + 1, main.tile_count());
                return Err(Error::new(error.kind(), message));
            }
            if single {
                self.truncated = walk.cut_or_incomplete();
                return Ok(());
            }
            diagnostics.info_fmt(format_args!("Tile {}/{} has been decoded.\n", index + 1, main.tile_count()));
            decoded += 1;
            if walk.past_cut() || decoded == main.tile_count() {
                break;
            }
        }
        if single {
            return Err(need_data("Failed to decode tile 1/1\n"));
        }
        if self.written.iter().any(|&written| !written) {
            for (&component, &written) in self.selection.iter().zip(&self.written) {
                if !written {
                    diagnostics.warn_fmt(format_args!("Failed to decode component {component}\n"));
                }
            }
            let message = "Failed to decode all used components\n";
            return Err(if walk.saw_cut() { need_data(message) } else { data_error(message) });
        }
        self.truncated = walk.cut_or_incomplete();
        Ok(())
    }

    fn prepare_image(&mut self, components: &[ComponentInfo], single: bool) -> Result<(), Error> {
        resize(&mut self.image, self.selection.len())?;
        resize(&mut self.written, self.selection.len())?;
        self.written.fill(false);
        for (plane, &component) in self.image.iter_mut().zip(&self.selection) {
            plane.info = components[component as usize];
            plane.resolution = 0;
            plane.alpha = 0;
            if !single {
                let count = sample_count(plane.info.width, plane.info.height)?;
                resize(&mut plane.samples, count)?;
                plane.samples.fill(0);
            }
        }
        Ok(())
    }
}

struct TileDecode<'a> {
    header: &'a MainHeader,
    reduce: u32,
    limits: Limits,
    index: u32,
    params: DecodeParams<'a>,
}

struct TileOutput<'a> {
    tile: &'a mut Tile,
    tier1: &'a mut t1::Scratch,
    tier2: &'a mut t2::Workspace,
    wavelet: &'a mut dwt::Workspace,
    packed: &'a mut Vec<u8>,
    combined: &'a mut Vec<u8>,
    budget: &'a mut WorkBudget,
    slot: &'a [Option<usize>],
    image: &'a mut [DecodedComponent],
    written: &'a mut [bool],
}

impl TileOutput<'_> {
    fn decode(
        &mut self,
        input: TileDecode<'_>,
        walk: &mut Walk<'_, '_>,
        ppm: Option<&mut &[u8]>,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Error> {
        let main = input.header;
        let index = input.index;
        self.budget.array(CType::TileComponentParams, main.comps.len())?;
        let override_settings = walk.overrides.find(index);
        let settings = TileParams::resolve(main, override_settings)?;
        let settings = settings.as_slice();
        if input.limits.max_memory != 0 && walk.overrides.find(index).is_some_and(|tile| tile.has_own_coding()) {
            let need = geometry::working_memory_for_tile(main, index, input.reduce, settings, self.slot);
            if need > input.limits.max_memory {
                let limit = input.limits.max_memory;
                return Err(Error::new(
                    ErrorKind::Limit,
                    format!("Tile {index} needs {need} bytes of working memory with its own coding, above the decoder's limit of {limit}\n"),
                ));
            }
        }
        let coding = *settings[0].coding;
        self.tile
            .configure(main, index, settings, input.reduce, self.budget)
            .map_err(|error| {
                diagnostics.error(error.message());
                Error::new(error.kind(), "Cannot decode tile, memory error\n")
            })?;

        if !walk.decodable(index) {
            return Err(data_error(""));
        }
        let ppt = walk.merge_ppt(index, self.packed, self.budget, diagnostics)?;
        let data = walk.tile_bytes(index, self.combined, self.budget)?;
        let pocs = walk.overrides.find(index).map(|tile| tile.poc.as_slice());
        let pocs = pocs.filter(|list| !list.is_empty()).unwrap_or(&main.poc);
        let packet_input = PacketInput {
            coding,
            pocs,
            layers: input.params.max_layers,
            strict: input.params.strict,
            cut: walk.tile_cut(index),
            data,
            ppm,
            ppt,
        };
        let whole = single_tile_image(main);
        let decoded = t2::read_packets(self.tier2, self.tile, packet_input, self.budget, diagnostics)
            .and_then(|decoded| {
                for ((component, &resolution), slot) in self.tile.comps.iter_mut().zip(decoded).zip(self.slot) {
                    if let Some(slot) = *slot {
                        if whole && self.image[slot].samples.capacity() > component.samples.capacity() {
                            // The tile is the image. Keep one allocation per component across requests.
                            core::mem::swap(&mut component.samples, &mut self.image[slot].samples);
                        }
                        inverse_component(component, resolution, data, self.tier1, self.wavelet, diagnostics)?;
                    }
                }
                Ok(decoded)
            })
            .and_then(|decoded| {
                if coding.mct != 0 && input.params.components.is_none() {
                    transform_colour(&mut self.tile.comps, decoded, diagnostics)?;
                }
                Ok(decoded)
            })
            .map_err(|error| {
                if !error.message().is_empty() {
                    diagnostics.error(error.message());
                }
                Error::new(error.kind(), "Failed to decode.\n")
            })?;
        walk.after_tile(index, diagnostics)?;
        for (index, (component, (&resolution, slot))) in self.tile.comps.iter_mut().zip(decoded.iter().zip(self.slot)).enumerate() {
            let Some(slot) = *slot else { continue };
            let sampling = main.comps[index];
            let finish = mct::Finish::new(sampling.prec, sampling.sgnd != 0, component.reversible);
            if copy_component(component, resolution, finish, &mut self.image[slot], whole)? {
                self.written[slot] = true;
            }
        }
        Ok(())
    }
}

// The Second Life and OpenSim profiles both admit eight components at most.
const INLINE_COMPONENTS: usize = 8;

#[allow(clippy::large_enum_variant, reason = "the small-image path keeps its component parameters on the stack")]
enum TileParams<'a> {
    Inline {
        params: [TileComponentParams<'a>; INLINE_COMPONENTS],
        len: usize,
    },
    Heap(Vec<TileComponentParams<'a>>),
}

impl<'a> TileParams<'a> {
    fn resolve(main: &'a MainHeader, overrides: Option<&'a TileOverrides<'_>>) -> Result<Self, Error> {
        let len = main.comps.len();
        if len > INLINE_COMPONENTS {
            return resolve_tile(main, overrides).map(Self::Heap);
        }
        let first = resolve_tile_component(main, overrides, 0)?;
        let mut params = [first; INLINE_COMPONENTS];
        for (index, target) in (1_u32..).zip(&mut params[1..len]) {
            *target = resolve_tile_component(main, overrides, index)?;
        }
        Ok(Self::Inline { params, len })
    }

    fn as_slice(&self) -> &[TileComponentParams<'a>] {
        match self {
            Self::Inline { params, len } => &params[..*len],
            Self::Heap(params) => params,
        }
    }
}

fn single_tile_image(main: &MainHeader) -> bool {
    main.tile_count() == 1
        && main.x0 == 0 && main.y0 == 0 && main.tx0 == 0 && main.ty0 == 0
        && main.x1 == main.tdx && main.y1 == main.tdy
}

fn inverse_component(
    component: &mut TileComp,
    decoded: u32,
    data: &[u8],
    scratch: &mut t1::Scratch,
    wavelet: &mut dwt::Workspace,
    diagnostics: &mut Diagnostics,
) -> Result<(), Error> {
    let finest = component.selected_resolution();
    let stride = finest.width() as usize;
    resize(&mut component.samples, sample_count(finest.width(), finest.height())?)?;
    component.samples.fill(0);
    for (index, resolution) in (0_u32..component.minimum_num_resolutions).zip(&component.res) {
        let coarser = index.checked_sub(1).map(|r| component.res[r as usize].rect);
        for band in &resolution.bands {
            let horizontal = coarser.map_or(0, |r| if band.bandno & 1 == 0 { 0 } else { r.width() });
            let vertical = coarser.map_or(0, |r| if band.bandno & 2 == 0 { 0 } else { r.height() });
            for precinct in &band.precincts {
                if !component.area_of_interest(index, band.bandno, precinct.rect) {
                    continue;
                }
                for block in &precinct.cblks {
                    if !component.area_of_interest(index, band.bandno, block.rect)
                        || block.rect.width() == 0 || block.rect.height() == 0
                    {
                        continue;
                    }
                    let x = block.rect.x0 - band.rect.x0 + horizontal;
                    let y = block.rect.y0 - band.rect.y0 + vertical;
                    let block_input = t1::Block {
                        width: block.rect.width(),
                        height: block.rect.height(),
                        numbps: block.numbps,
                        orient: band.bandno,
                        roishift: u32::from(component.roishift),
                        cblksty: u32::from(component.cblksty),
                        segments: &block.segs[..block.real_num_segs as usize],
                    };
                    scratch.decode(
                        block_input,
                        block.chunks.iter().map(|chunk| &data[chunk.clone()]),
                        Destination {
                            samples: &mut component.samples,
                            offset: y as usize * stride + x as usize,
                            stride,
                            reversible: component.reversible,
                            stepsize: band.stepsize,
                        },
                        diagnostics,
                    )?;
                }
            }
        }
    }
    let mut rects = [Rect::default(); 33];
    let count = (decoded + 1).min(component.minimum_num_resolutions) as usize;
    for (rect, res) in rects[..count].iter_mut().zip(&component.res) {
        *rect = res.rect;
    }
    dwt::inverse(&mut component.samples, stride, &rects[..count], component.reversible, wavelet)
}

fn transform_colour(
    components: &mut [TileComp],
    resolutions: &[u32],
    diagnostics: &mut Diagnostics,
) -> Result<(), Error> {
    let count = components.len();
    let [a, b, c, ..] = components else {
        diagnostics.error(format!("Number of components ({count}) is inconsistent with a MCT. Skip the MCT step.\n"));
        return Ok(());
    };
    if resolutions[0] != resolutions[1]
        || resolutions[0] != resolutions[2]
        || a.minimum_num_resolutions != b.minimum_num_resolutions
        || a.minimum_num_resolutions != c.minimum_num_resolutions
        || a.samples.len() != b.samples.len()
        || a.samples.len() != c.samples.len()
    {
        return Err(data_error("Tiles don't all have the same dimension. Skip the MCT step.\n"));
    }
    let transform = if a.reversible { mct::inverse_reversible } else { mct::inverse_irreversible };
    for ((y, u), v) in a.samples.iter_mut().zip(&mut b.samples).zip(&mut c.samples) {
        [*y, *u, *v] = transform([*y, *u, *v]);
    }
    Ok(())
}

fn copy_component(
    source: &mut TileComp,
    resolution: u32,
    finish: mct::Finish,
    target: &mut DecodedComponent,
    whole: bool,
) -> Result<bool, Error> {
    target.resolution = resolution;
    if source.samples.is_empty() {
        if whole {
            target.samples.clear();
        }
        return Ok(false);
    }
    let requested = source.selected_resolution();
    let actual = source.res[resolution.min(source.minimum_num_resolutions - 1) as usize].rect;
    let stride = requested.width() as usize;
    if whole && requested.width() == target.info.width && requested.height() == target.info.height {
        for row in source.samples.chunks_exact_mut(stride).take(actual.height() as usize) {
            for sample in &mut row[..actual.width() as usize] {
                *sample = finish.sample(*sample);
            }
        }
        core::mem::swap(&mut source.samples, &mut target.samples);
        return Ok(true);
    }

    let info = target.info;
    let x0 = ceil_div_pow2(info.x0, info.reduce);
    let y0 = ceil_div_pow2(info.y0, info.reduce);
    let bounds = Rect {
        x0,
        y0,
        x1: x0.checked_add(info.width).ok_or_else(invalid_output)?,
        y1: y0.checked_add(info.height).ok_or_else(invalid_output)?,
    };
    let area = actual.clipped_to(bounds);
    if area.width() == 0 || area.height() == 0 {
        return Ok(true);
    }
    let source_x = (area.x0 - actual.x0) as usize;
    let source_y = (area.y0 - actual.y0) as usize;
    let width = area.width() as usize;
    let target_x = (area.x0 - x0) as usize;
    let target_y = (area.y0 - y0) as usize;
    let target_stride = info.width as usize;
    for row in 0..area.height() as usize {
        let i = (source_y + row) * stride + source_x;
        let j = (target_y + row) * target_stride + target_x;
        let input = &source.samples[i..i + width];
        let output = &mut target.samples[j..j + width];
        for (output, &input) in output.iter_mut().zip(input) {
            *output = finish.sample(input);
        }
    }
    Ok(true)
}

fn sample_count(width: u32, height: u32) -> Result<usize, Error> {
    (width as usize)
        .checked_mul(height as usize)
        .filter(|&size| size <= isize::MAX as usize / size_of::<i32>())
        .ok_or_else(invalid_output)
}

fn invalid_output() -> Error {
    Error::new(ErrorKind::Output, "The decoded component does not fit in memory\n")
}

fn no_memory() -> Error {
    Error::new(ErrorKind::NoMemory, "Not enough memory to decode\n")
}
