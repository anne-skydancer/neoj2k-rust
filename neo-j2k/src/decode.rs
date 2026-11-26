// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The native memory decoder's tile walk and output assembly.

mod blocks;
mod output;
mod walk;

pub(crate) use output::validate as validate_targets;
pub use output::{DecodeTarget, I32Plane, OutputPlane, SampleType};

use crate::error::Diagnostics;
use crate::executor::{Executor, Jobs};
use crate::geometry::{self, Rect, ceil_div_pow2};
use crate::header::{
    MainHeader,
    tile::{TileComponentParams, TileOverrides, resolve_tile, resolve_tile_component},
};
use crate::mem::{CType, HeaderLedger, WorkBudget, capacity_bytes};
use crate::t1;
use crate::t2::{self, PacketInput};
use crate::tile::{Resolution, Tile, TileComp, resize};
use crate::{ComponentInfo, Error, ErrorKind, Limits, dwt, mct};
use walk::{Pages, Walk, data_error, need_data};

use std::fmt;

// Let the Miri integration test exercise the same job paths on a tiny image.
const TILE_SAMPLE_FLOOR: u64 = if cfg!(miri) { 1 } else { 1 << 18 };
const OUTPUT_SAMPLE_FLOOR: usize = if cfg!(miri) { 1 } else { 1 << 16 };
const CLEAR_BYTE_FLOOR: usize = if cfg!(miri) { size_of::<i32>() } else { 1 << 22 };

/// Options for one decode. A zero layer limit decodes every quality layer.
#[derive(Clone, Copy, Default)]
pub struct DecodeParams<'a> {
    pub max_layers: u32,
    /// Reject truncated codestreams. Defaults to `false` for partial texture downloads.
    pub strict: bool,
    /// Original component indices. Supplying a selection disables the colour transform.
    pub components: Option<&'a [u32]>,
    /// How to handle a JP2 palette or channel definitions when decoding every component.
    pub jp2_channels: Jp2Channels,
    /// A scheduler borrowed for independent work on large images.
    pub executor: Option<&'a dyn Executor>,
}

impl fmt::Debug for DecodeParams<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DecodeParams")
            .field("max_layers", &self.max_layers)
            .field("strict", &self.strict)
            .field("components", &self.components)
            .field("jp2_channels", &self.jp2_channels)
            .field("has_executor", &self.executor.is_some())
            .finish()
    }
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

/// Arena accounting using the C++ decoder's reservation model.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DecodeStatistics {
    /// Bytes reserved for the validated main header.
    pub header_reserved: u64,
    /// Bytes reserved for tile-part metadata.
    pub tile_reserved: u64,
    /// Bytes reserved for the modeled work arena.
    pub work_reserved: u64,
    /// Highest modeled work-arena use during the decode.
    pub work_high_water: u64,
}

pub(crate) struct Input<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) stream: Option<crate::stream::StreamBounds>,
    pub(crate) header: &'a MainHeader,
    pub(crate) components: &'a [ComponentInfo],
    pub(crate) reduce: u32,
    pub(crate) limits: Limits,
    pub(crate) retain: u64,
}

pub(crate) struct TypedOutput<'targets, 'samples> {
    pub(crate) targets: &'targets mut [I32Plane<'samples>],
    direct: bool,
}

impl<'targets, 'samples> TypedOutput<'targets, 'samples> {
    pub(crate) fn new(targets: &'targets mut [I32Plane<'samples>]) -> Self {
        Self { targets, direct: false }
    }

    pub(crate) fn validate(&self, components: &[ComponentInfo], slots: &[Option<usize>]) -> Result<(), Error> {
        for (index, target) in self.targets.iter().enumerate() {
            let component = target.component as usize;
            if slots.get(component).is_none_or(Option::is_none) {
                return Err(Error::new(
                    ErrorKind::Output,
                    format!("nj2k_decode_memory: plane {index} does not fit the image\n"),
                ));
            }
            let info = components[component];
            if target.samples.len() < sample_count(info.width, info.height)? {
                return Err(Error::new(
                    ErrorKind::Output,
                    "The output plane does not fit its buffer\n",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(crate) struct Workspace {
    pub(crate) tile_ledger: HeaderLedger,
    pub(crate) budget: WorkBudget,
    pages: Pages,
    tile: Tile,
    serial: ComponentScratch,
    workers: Vec<WorkerScratch>,
    tier2: t2::Workspace,
    packed: Vec<u8>,
    combined: Vec<u8>,
    image: Vec<DecodedComponent>,
    pub(crate) selection: Vec<u32>,
    slot: Vec<Option<usize>>,
    written: Vec<bool>,
    pub(crate) truncated: bool,
    pub(crate) retained_work_bytes: u64,
}

impl Workspace {
    /// Return excess native buffer capacity, keeping the image only when it is borrowed by a caller.
    pub(crate) fn trim_retained(&mut self, limit: u64, keep_image: bool) {
        for worker in &mut self.workers {
            worker.diagnostics.clear();
            worker.result = None;
        }
        let image_bytes = if keep_image {
            0
        } else {
            image_capacity_bytes(&self.image)
        };
        let worker_bytes = capacity_bytes(&self.workers)
            + self
                .workers
                .iter()
                .map(|worker| worker.component.retained_bytes() + worker.diagnostics.retained_bytes())
                .sum::<u64>();
        let mut bytes = image_bytes
            + worker_bytes
            + self.serial.retained_bytes()
            + self.budget.retained_bytes()
            + self.pages.retained_bytes()
            + tile_capacity_bytes(&self.tile)
            + self.tier2.retained_bytes()
            + capacity_bytes(&self.packed)
            + capacity_bytes(&self.combined)
            + capacity_bytes(&self.selection)
            + capacity_bytes(&self.slot)
            + capacity_bytes(&self.written);

        if bytes > limit {
            release_vec(&mut self.combined, &mut bytes);
            release_vec(&mut self.packed, &mut bytes);
        }
        if bytes > limit {
            if !keep_image {
                self.image = Vec::new();
                bytes -= image_bytes;
            }
            for component in &mut self.tile.comps {
                release_vec(&mut component.samples, &mut bytes);
            }
        }
        if bytes > limit {
            bytes -= self.serial.wavelet.retained_bytes();
            self.serial.wavelet = dwt::Workspace::default();
        }
        if bytes > limit {
            self.workers = Vec::new();
            bytes -= worker_bytes;
        }
        if bytes > limit {
            bytes -= self.serial.tier1.retained_bytes();
            self.serial.tier1 = t1::Scratch::default();
        }
        if bytes > limit {
            self.pages = Pages::default();
            self.tile = Tile::default();
            self.tier2 = t2::Workspace::default();
            self.budget = WorkBudget::default();
            self.tile_ledger = HeaderLedger::default();
            self.selection = Vec::new();
            self.slot = Vec::new();
            self.written = Vec::new();
            bytes = 0;
        }
        self.retained_work_bytes = bytes;
    }

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
            if output::write_interleaved(&self.image, &self.slot, sample, target) {
                continue;
            }
            for plane in target.planes {
                if let Some(slot) = self.slot[plane.component as usize] {
                    output::write(&self.image[slot], sample, target.bytes, *plane);
                }
            }
        }
    }

    pub(crate) fn write_i32_targets(&self, output: &mut TypedOutput<'_, '_>) {
        if !output.direct {
            for target in &mut *output.targets {
                let slot = self.slot[target.component as usize].expect("the target component was validated");
                let samples = self.image[slot].samples();
                target.samples[..samples.len()].copy_from_slice(samples);
            }
        }
    }

    pub(crate) fn reset_selection(&mut self, count: usize, selection: Option<&[u32]>) -> Result<(), Error> {
        resize(&mut self.slot, count)?;
        self.selection.clear();
        self.selection
            .try_reserve_exact(selection.map_or(count, <[u32]>::len))
            .map_err(|_| no_memory())?;
        if let Some(indices) = selection {
            self.slot.fill(None);
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
        mut output: Option<&mut TypedOutput<'_, '_>>,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Error> {
        let begin = stage_start!();
        let main = input.header;
        // Account for the previous decode's retained working memory before charging this one.
        self.budget.reset(input.retain, input.limits.max_memory);
        self.tile_ledger.reset(input.retain, input.limits.max_memory);
        self.budget.array(CType::OutComp, main.comps.len())?;
        let single = single_tile_image(main);
        self.prepare_image(input.components, single, params.executor)?;
        self.truncated = false;

        let mut walk = Walk::new(
            input.data,
            main,
            &mut self.pages,
            input.limits,
            input.reduce,
            params.strict,
            input.stream,
        );
        let mut ppm = (!main.ppm_data.is_empty()).then_some(main.ppm_data.as_slice());
        let mut decoded = 0;
        stage_stop!(crate::stage::Stage::Begin, begin);
        while let Some(index) = timed!(
            crate::stage::Stage::TileScan,
            walk.next_tile(&mut self.budget, &mut self.tile_ledger, diagnostics)
        )? {
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
                serial: &mut self.serial,
                workers: &mut self.workers,
                tier2: &mut self.tier2,
                packed: &mut self.packed,
                combined: &mut self.combined,
                budget: &mut self.budget,
                slot: &self.slot,
                image: &mut self.image,
                written: &mut self.written,
            };
            let result = target.decode(work, &mut walk, ppm.as_mut(), output.as_deref_mut(), diagnostics);
            self.budget.rewind(mark);
            if let Err(error) = result {
                diagnostics.error(error.message());
                return Err(Error::formatted(
                    error.kind(),
                    format_args!("Failed to decode tile {}/{}\n", index + 1, main.tile_count()),
                    "Failed to decode tile\n",
                ));
            }
            if single {
                self.truncated = walk.cut_or_incomplete();
                return Ok(());
            }
            diagnostics.info_fmt(format_args!(
                "Tile {}/{} has been decoded.\n",
                index + 1,
                main.tile_count()
            ));
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
            return Err(if walk.saw_cut() {
                need_data(message)
            } else {
                data_error(message)
            });
        }
        self.truncated = walk.cut_or_incomplete();
        Ok(())
    }

    fn prepare_image(
        &mut self,
        components: &[ComponentInfo],
        single: bool,
        executor: Option<&dyn Executor>,
    ) -> Result<(), Error> {
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
                clear_plane(&mut plane.samples, plane.info.height as usize, executor);
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

#[derive(Debug, Default)]
struct ComponentScratch {
    tier1: t1::Scratch,
    wavelet: dwt::Workspace,
}

impl ComponentScratch {
    fn retained_bytes(&self) -> u64 {
        self.tier1.retained_bytes() + self.wavelet.retained_bytes()
    }
}

fn release_vec<T>(storage: &mut Vec<T>, total: &mut u64) {
    *total -= capacity_bytes(storage);
    *storage = Vec::new();
}

fn image_capacity_bytes(image: &Vec<DecodedComponent>) -> u64 {
    capacity_bytes(image)
        + image
            .iter()
            .map(|component| capacity_bytes(&component.samples))
            .sum::<u64>()
}

fn tile_capacity_bytes(tile: &Tile) -> u64 {
    let mut bytes = capacity_bytes(&tile.comps);
    for component in &tile.comps {
        bytes += capacity_bytes(&component.samples) + capacity_bytes(&component.res);
        for resolution in &component.res {
            bytes += capacity_bytes(&resolution.bands);
            for band in &resolution.bands {
                bytes += capacity_bytes(&band.precincts);
                for precinct in &band.precincts {
                    bytes += capacity_bytes(&precinct.cblks)
                        + precinct.incl.retained_bytes()
                        + precinct.imsb.retained_bytes();
                    for block in &precinct.cblks {
                        bytes += capacity_bytes(&block.segs) + capacity_bytes(&block.chunks);
                    }
                }
            }
        }
    }
    bytes
}

#[derive(Debug, Default)]
struct WorkerScratch {
    component: ComponentScratch,
    diagnostics: Diagnostics,
    result: Option<Result<(), Error>>,
}

struct Components<'a, 'samples> {
    parts: &'a mut [TileComp],
    resolutions: &'a [u32],
    slots: &'a [Option<usize>],
    direct: Option<&'a mut [Option<&'samples mut [i32]>]>,
}

impl Components<'_, '_> {
    fn take(&mut self, selected: usize) -> Self {
        let end = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| slot.is_some())
            .nth(selected - 1)
            .map_or(self.slots.len(), |(index, _)| index + 1);
        let (parts, remaining) = core::mem::take(&mut self.parts).split_at_mut(end);
        self.parts = remaining;
        let (resolutions, remaining) = self.resolutions.split_at(end);
        self.resolutions = remaining;
        let (slots, remaining) = self.slots.split_at(end);
        self.slots = remaining;
        let direct = self.direct.take().map(|planes| {
            let (planes, remaining) = planes.split_at_mut(end);
            self.direct = Some(remaining);
            planes
        });
        Self {
            parts,
            resolutions,
            slots,
            direct,
        }
    }

    fn inverse(
        &mut self,
        data: &[u8],
        scratch: &mut ComponentScratch,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Error> {
        for (index, ((component, &resolution), slot)) in
            self.parts.iter_mut().zip(self.resolutions).zip(self.slots).enumerate()
        {
            if slot.is_some() {
                let samples = self
                    .direct
                    .as_deref_mut()
                    .and_then(|planes| planes[index].as_deref_mut());
                let mut plane = timed!(
                    crate::stage::Stage::PlanePrep,
                    Reconstruction::prepare(component, samples)
                )?;
                inverse_component(&mut plane, resolution, data, scratch, diagnostics)?;
            }
        }
        Ok(())
    }

    fn parallel_inverse(
        &mut self,
        jobs: Jobs<'_>,
        workers: &mut Vec<WorkerScratch>,
        serial: &mut ComponentScratch,
        data: &[u8],
        diagnostics: &mut Diagnostics,
    ) -> Option<Result<(), Error>> {
        let selected = self.slots.iter().filter(|slot| slot.is_some()).count();
        // Independent components fill a small pool. A wider pool benefits from
        // splitting the code-block and wavelet work within large components.
        let staged_limit = (2 * jobs.width()).min(crate::executor::MAX_JOBS);
        if (2..=staged_limit).contains(&selected) {
            return self.inverse_staged(selected, jobs, workers, serial, data, diagnostics);
        }
        if selected < jobs.width() {
            return Some(self.inverse_components_with_jobs(jobs, workers, serial, data, diagnostics));
        }
        let max_jobs = selected.min(jobs.width());
        if max_jobs < 2 {
            return None;
        }
        let each = selected.div_ceil(max_jobs);
        let count = selected.div_ceil(each);
        if workers.len() < count && resize(workers, count).is_err() {
            *workers = Vec::new();
            serial.wavelet.release_parallel_scratch();
            return None;
        }
        let mut components = Components {
            parts: self.parts,
            resolutions: self.resolutions,
            slots: self.slots,
            direct: self.direct.as_deref_mut(),
        };
        let tasks = workers[..count].iter_mut().map(|worker| {
            worker.diagnostics.configure_like(diagnostics);
            (components.take(each), worker)
        });
        jobs.run(tasks, |(mut batch, worker)| {
            worker.result = Some(batch.inverse(data, &mut worker.component, &mut worker.diagnostics));
        });

        if workers[..count].iter().any(|worker| {
            worker
                .result
                .as_ref()
                .and_then(|result| result.as_ref().err())
                .is_some_and(|error| error.kind() == ErrorKind::NoMemory)
        }) {
            *workers = Vec::new();
            serial.wavelet.release_parallel_scratch();
            return None;
        }
        // Replay jobs in component order, stopping at the first failure as the serial walk does.
        for worker in &mut workers[..count] {
            diagnostics.append(&mut worker.diagnostics);
            let result = worker.result.take().expect("each component job completed");
            if result.is_err() {
                return Some(result);
            }
        }
        Some(Ok(()))
    }

    fn inverse_staged(
        &mut self,
        count: usize,
        jobs: Jobs<'_>,
        workers: &mut Vec<WorkerScratch>,
        serial: &mut ComponentScratch,
        data: &[u8],
        diagnostics: &mut Diagnostics,
    ) -> Option<Result<(), Error>> {
        let mut planes = Vec::new();
        if planes.try_reserve_exact(count).is_err() {
            *workers = Vec::new();
            serial.wavelet.release_parallel_scratch();
            return None;
        }
        let direct = self
            .direct
            .as_deref_mut()
            .into_iter()
            .flatten()
            .map(Option::as_deref_mut)
            .chain(core::iter::repeat_with(|| None));
        for (((component, &resolution), slot), samples) in
            self.parts.iter_mut().zip(self.resolutions).zip(self.slots).zip(direct)
        {
            if slot.is_some() {
                match Reconstruction::prepare(component, samples) {
                    Ok(plane) => planes.push((plane, resolution)),
                    Err(error) if error.kind() == ErrorKind::NoMemory => {
                        *workers = Vec::new();
                        serial.wavelet.release_parallel_scratch();
                        return None;
                    }
                    Err(error) => return Some(Err(error)),
                }
            }
        }

        // All coded regions share an executor batch. A threaded refusal is replayed in
        // codestream order for the same diagnostic and status as a serial Tier-1 decode.
        let mut starts = [0; crate::executor::MAX_JOBS];
        if blocks::parallel_decode_tile(&mut planes, jobs, workers, data) {
            if planes.len() <= jobs.width()
                && let Err(error) =
                    inverse_coarse_wavelets(&mut planes, &mut starts, jobs, workers, &mut serial.wavelet)
            {
                return Some(Err(error));
            }
        } else {
            serial.wavelet.release_parallel_scratch();
            for (plane, _) in &mut planes {
                if let Err(error) = blocks::decode(plane, data, &mut serial.tier1, diagnostics) {
                    return Some(Err(error));
                }
            }
        }
        Some(
            planes
                .iter_mut()
                .zip(starts)
                .try_for_each(|((plane, resolution), start)| {
                    inverse_scheduled_wavelet(
                        plane,
                        start..(*resolution as usize + 1),
                        &mut serial.wavelet,
                        jobs,
                        workers,
                    )
                }),
        )
    }

    fn inverse_components_with_jobs(
        &mut self,
        jobs: Jobs<'_>,
        workers: &mut Vec<WorkerScratch>,
        serial: &mut ComponentScratch,
        data: &[u8],
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Error> {
        for (index, ((component, &resolution), slot)) in
            self.parts.iter_mut().zip(self.resolutions).zip(self.slots).enumerate()
        {
            if slot.is_none() {
                continue;
            }
            let rect = component.selected_resolution();
            let samples = self
                .direct
                .as_deref_mut()
                .and_then(|planes| planes[index].as_deref_mut());
            let mut plane = Reconstruction::prepare(component, samples)?;
            if u64::from(rect.width()) * u64::from(rect.height()) < TILE_SAMPLE_FLOOR {
                inverse_component(&mut plane, resolution, data, serial, diagnostics)?;
                continue;
            }
            if !blocks::parallel_decode(&mut plane, jobs, workers, data) {
                serial.wavelet.release_parallel_scratch();
                blocks::decode(&mut plane, data, &mut serial.tier1, diagnostics)?;
            }
            inverse_scheduled_wavelet(
                &mut plane,
                0..(resolution as usize + 1),
                &mut serial.wavelet,
                jobs,
                workers,
            )?;
        }
        Ok(())
    }
}

struct TileOutput<'a> {
    tile: &'a mut Tile,
    serial: &'a mut ComponentScratch,
    workers: &'a mut Vec<WorkerScratch>,
    tier2: &'a mut t2::Workspace,
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
        output: Option<&mut TypedOutput<'_, '_>>,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Error> {
        let setup = stage_start!();
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
                    format!(
                        "Tile {index} needs {need} bytes of working memory with its own coding, above the decoder's limit of {limit}\n"
                    ),
                ));
            }
        }
        let coding = *settings[0].coding;
        self.tile
            .configure(main, index, settings, input.reduce, self.budget)
            .map_err(|error| {
                diagnostics.error(error.message());
                Error::static_message(error.kind(), "Cannot decode tile, memory error\n")
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
        stage_stop!(crate::stage::Stage::TileSetup, setup);
        let whole = single_tile_image(main);
        let mut direct = if let Some(output) = output.filter(|_| whole) {
            let planes = DirectPlanes::new(output.targets, self.slot, &self.tile.comps, self.image)?;
            output.direct = planes.is_some();
            planes
        } else {
            None
        };
        let jobs = input
            .params
            .executor
            .filter(|_| {
                self.tile.comps.iter().zip(self.slot).any(|(component, slot)| {
                    let rect = component.selected_resolution();
                    slot.is_some() && u64::from(rect.width()) * u64::from(rect.height()) >= TILE_SAMPLE_FLOOR
                })
            })
            .and_then(Jobs::new);
        let mut fused_mct = None;
        let decoded = timed!(
            crate::stage::Stage::Packets,
            t2::read_packets(self.tier2, self.tile, packet_input, self.budget, diagnostics)
        )
        .and_then(|decoded| {
            if direct.is_some() {
                // The destination replaces both usual locations for this tile's sample storage.
                for (component, slot) in self.tile.comps.iter_mut().zip(self.slot) {
                    if let Some(slot) = *slot {
                        component.samples = Vec::new();
                        self.image[slot].samples = Vec::new();
                    }
                }
            } else if whole {
                for (component, slot) in self.tile.comps.iter_mut().zip(self.slot) {
                    if let Some(slot) = *slot
                        && self.image[slot].samples.capacity() > component.samples.capacity()
                    {
                        // The tile is the image. Keep one allocation per component across requests.
                        core::mem::swap(&mut component.samples, &mut self.image[slot].samples);
                    }
                }
            }
            let mut components = Components {
                parts: &mut self.tile.comps,
                resolutions: decoded,
                slots: self.slot,
                direct: direct.as_mut().map(DirectPlanes::as_mut),
            };
            if let Some(result) =
                jobs.and_then(|jobs| components.parallel_inverse(jobs, self.workers, self.serial, data, diagnostics))
            {
                result?;
            } else {
                components.inverse(data, self.serial, diagnostics)?;
            }
            Ok(decoded)
        })
        .and_then(|decoded| {
            if coding.mct != 0 && input.params.components.is_none() {
                if whole {
                    fused_mct = whole_mct_rect(&self.tile.comps, decoded).filter(|rect| {
                        self.slot[..3].iter().all(|slot| {
                            slot.is_some_and(|slot| {
                                let info = self.image[slot].info;
                                info.width == rect.width() && info.height == rect.height()
                            })
                        })
                    });
                }
                if fused_mct.is_none() {
                    timed!(
                        crate::stage::Stage::Colour,
                        transform_colour(
                            &mut self.tile.comps,
                            direct.as_mut().map(DirectPlanes::as_mut),
                            decoded,
                            diagnostics,
                            jobs,
                        )
                    )?;
                }
            }
            Ok(decoded)
        })
        .map_err(|error| {
            if !error.message().is_empty() {
                diagnostics.error(error.message());
            }
            Error::static_message(error.kind(), "Failed to decode.\n")
        })?;
        walk.after_tile(index, diagnostics)?;
        if let Some(rect) = fused_mct {
            let reversible = self.tile.comps[0].reversible;
            let finishes = std::array::from_fn(|index| {
                let sampling = main.comps[index];
                mct::Finish::new(sampling.prec, sampling.sgnd != 0, self.tile.comps[index].reversible)
            });
            let planes: [&mut [i32]; 3] = if let Some(planes) = &mut direct {
                let [Some(y), Some(u), Some(v), ..] = planes.as_mut() else {
                    unreachable!("the three MCT components have validated direct planes")
                };
                [y, u, v]
            } else {
                let [y, u, v, ..] = self.tile.comps.as_mut_slice() else {
                    unreachable!("the tile has three compatible MCT components")
                };
                [&mut y.samples, &mut u.samples, &mut v.samples]
            };
            timed!(
                crate::stage::Stage::Colour,
                finish_whole_mct(planes, reversible, finishes, rect, jobs)
            );
        }
        for (index, (component, (&resolution, slot))) in self
            .tile
            .comps
            .iter_mut()
            .zip(decoded.iter().zip(self.slot))
            .enumerate()
        {
            let Some(slot) = *slot else { continue };
            if fused_mct.is_some() && index < 3 {
                if direct.is_none() {
                    self.image[slot].resolution = resolution;
                    core::mem::swap(&mut component.samples, &mut self.image[slot].samples);
                }
                self.written[slot] = true;
                continue;
            }
            let sampling = main.comps[index];
            let finish = mct::Finish::new(sampling.prec, sampling.sgnd != 0, component.reversible);
            self.written[slot] |= if let Some(planes) = &mut direct {
                let samples = planes.as_mut()[index]
                    .as_deref_mut()
                    .expect("selected components have direct planes");
                timed!(
                    crate::stage::Stage::Finish,
                    finish_tile_component(component, resolution, samples, finish, jobs)
                )
            } else {
                timed!(
                    crate::stage::Stage::Finish,
                    copy_component(component, resolution, finish, &mut self.image[slot], whole, jobs)
                )?
            };
        }
        Ok(())
    }
}

// The Second Life and OpenSim profiles both admit eight components at most.
const INLINE_COMPONENTS: usize = 8;

#[allow(clippy::large_enum_variant, reason = "the normal component counts fit on the stack")]
enum DirectPlanes<'a> {
    Inline {
        planes: [Option<&'a mut [i32]>; INLINE_COMPONENTS],
        count: usize,
    },
    Heap(Vec<Option<&'a mut [i32]>>),
}

impl<'a> DirectPlanes<'a> {
    fn new(
        targets: &'a mut [I32Plane<'_>],
        slots: &[Option<usize>],
        components: &[TileComp],
        image: &[DecodedComponent],
    ) -> Result<Option<Self>, Error> {
        if targets.len() != image.len()
            || slots.iter().zip(components).any(|(slot, component)| {
                slot.is_some_and(|slot| {
                    let bounds = component.selected_resolution();
                    let info = image[slot].info;
                    bounds.width() != info.width || bounds.height() != info.height
                })
            })
        {
            return Ok(None);
        }
        let count = slots.len();
        let mut map = if count <= INLINE_COMPONENTS {
            Self::Inline {
                planes: std::array::from_fn(|_| None),
                count,
            }
        } else {
            let mut planes = Vec::new();
            resize(&mut planes, count)?;
            Self::Heap(planes)
        };
        let planes = map.as_mut();
        for target in targets {
            let index = target.component as usize;
            let plane = &mut planes[index];
            if plane.is_some() {
                return Ok(None);
            }
            let slot = slots[index].expect("output components are validated before attaching samples");
            let info = image[slot].info;
            let len = sample_count(info.width, info.height)?;
            *plane = Some(&mut target.samples[..len]);
        }
        Ok(Some(map))
    }

    fn as_mut(&mut self) -> &mut [Option<&'a mut [i32]>] {
        match self {
            Self::Inline { planes, count } => &mut planes[..*count],
            Self::Heap(planes) => planes,
        }
    }
}

#[allow(
    clippy::large_enum_variant,
    reason = "the small-image path keeps its component parameters on the stack"
)]
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
        && main.x0 == 0
        && main.y0 == 0
        && main.tx0 == 0
        && main.ty0 == 0
        && main.x1 == main.tdx
        && main.y1 == main.tdy
}

fn inverse_component(
    component: &mut Reconstruction<'_>,
    decoded: u32,
    data: &[u8],
    scratch: &mut ComponentScratch,
    diagnostics: &mut Diagnostics,
) -> Result<(), Error> {
    timed!(
        crate::stage::Stage::Tier1,
        blocks::decode(component, data, &mut scratch.tier1, diagnostics)
    )?;
    timed!(
        crate::stage::Stage::Wavelet,
        inverse_wavelet(component, 0..(decoded as usize + 1), &mut scratch.wavelet, None)
    )
}

struct Reconstruction<'a> {
    res: &'a [Resolution],
    stride: usize,
    reversible: bool,
    roishift: u8,
    cblksty: u8,
    samples: &'a mut [i32],
}

impl<'a> Reconstruction<'a> {
    fn prepare(component: &'a mut TileComp, direct: Option<&'a mut [i32]>) -> Result<Self, Error> {
        let finest = component.selected_resolution();
        let count = sample_count(finest.width(), finest.height())?;
        let samples = if let Some(samples) = direct {
            debug_assert_eq!(samples.len(), count);
            // The block walk covers the plane, writing zeros for blocks with no packet data.
            samples
        } else {
            resize(&mut component.samples, count)?;
            component.samples.fill(0);
            &mut component.samples
        };
        Ok(Self {
            res: &component.res[..component.minimum_num_resolutions as usize],
            stride: finest.width() as usize,
            reversible: component.reversible,
            roishift: component.roishift,
            cblksty: component.cblksty,
            samples,
        })
    }
}

fn inverse_wavelet(
    component: &mut Reconstruction<'_>,
    levels: core::ops::Range<usize>,
    workspace: &mut dwt::Workspace,
    jobs: Option<Jobs<'_>>,
) -> Result<(), Error> {
    let mut rects = [Rect::default(); 33];
    let end = levels.end.min(component.res.len());
    let resolutions = &component.res[levels.start.min(end)..end];
    let rects = &mut rects[..resolutions.len()];
    for (rect, res) in rects.iter_mut().zip(resolutions) {
        *rect = res.rect;
    }
    if let Some(jobs) = jobs {
        return dwt::inverse_parallel(
            component.samples,
            component.stride,
            rects,
            component.reversible,
            workspace,
            jobs,
        );
    }
    dwt::inverse(
        component.samples,
        component.stride,
        rects,
        component.reversible,
        workspace,
    )
}

fn inverse_scheduled_wavelet(
    component: &mut Reconstruction<'_>,
    levels: core::ops::Range<usize>,
    workspace: &mut dwt::Workspace,
    jobs: Jobs<'_>,
    workers: &mut Vec<WorkerScratch>,
) -> Result<(), Error> {
    match inverse_wavelet(component, levels.clone(), workspace, Some(jobs)) {
        Err(error) if error.kind() == ErrorKind::NoMemory => {
            // Required wavelet scratch is reserved before changing coefficients.
            *workers = Vec::new();
            inverse_wavelet(component, levels, workspace, None)
        }
        result => result,
    }
}

fn inverse_coarse_wavelets(
    planes: &mut [(Reconstruction<'_>, u32)],
    starts: &mut [usize],
    jobs: Jobs<'_>,
    workers: &mut Vec<WorkerScratch>,
    serial: &mut dwt::Workspace,
) -> Result<(), Error> {
    for ((plane, decoded), start) in planes.iter().zip(&mut *starts) {
        let count = (*decoded as usize + 1).min(plane.res.len());
        *start = plane.res[..count]
            .iter()
            .take_while(|level| {
                let area = u64::from(level.rect.width()) * u64::from(level.rect.height());
                area < dwt::PARALLEL_LEVEL_AREA as u64
            })
            .count()
            .saturating_sub(1);
    }
    if starts.iter().filter(|&&start| start > 0).count() < 2 {
        starts.fill(0);
        return Ok(());
    }

    // The lower levels can use different workers without synchronizing between rows and columns.
    let mut results = [const { Ok(()) }; crate::executor::MAX_JOBS];
    let tasks = planes
        .iter_mut()
        .zip(starts.iter().copied())
        .zip(&mut *workers)
        .zip(&mut results)
        .filter_map(|((((plane, _), start), worker), result)| {
            (start > 0).then_some((plane, start, &mut worker.component.wavelet, result))
        });
    jobs.run(tasks, |(plane, start, workspace, result)| {
        *result = inverse_wavelet(plane, 0..(start + 1), workspace, None);
    });
    if results.iter().any(|result| result.is_err()) {
        *workers = Vec::new();
        serial.release_parallel_scratch();
        for (((plane, _), &start), result) in planes.iter_mut().zip(starts.iter()).zip(results) {
            match result {
                Err(error) if error.kind() == ErrorKind::NoMemory => {
                    inverse_wavelet(plane, 0..(start + 1), serial, None)?;
                }
                result => result?,
            }
        }
    }
    Ok(())
}

fn whole_mct_rect(components: &[TileComp], resolutions: &[u32]) -> Option<Rect> {
    let [a, b, c, ..] = components else { return None };
    let [ra, rb, rc, ..] = resolutions else { return None };
    let rect = a.selected_resolution();
    let requested = a.minimum_num_resolutions - 1;
    // A reduced or differently shaped plane needs the existing separate finish path.
    (*ra == *rb
        && *ra == *rc
        && *ra >= requested
        && a.minimum_num_resolutions == b.minimum_num_resolutions
        && a.minimum_num_resolutions == c.minimum_num_resolutions
        && rect == b.selected_resolution()
        && rect == c.selected_resolution()
        && rect.width() != 0
        && rect.height() != 0)
        .then_some(rect)
}

fn finish_whole_mct(
    [y, u, v]: [&mut [i32]; 3],
    reversible: bool,
    finishes: [mct::Finish; 3],
    rect: Rect,
    jobs: Option<Jobs<'_>>,
) {
    let apply = |planes| mct::inverse_and_finish(planes, reversible, finishes);
    let (width, rows) = (rect.width() as usize, rect.height() as usize);
    if let Some(jobs) = output_jobs(jobs, width, rows) {
        let span = rows.div_ceil(jobs.width()) * width;
        jobs.run(
            y.chunks_mut(span)
                .zip(u.chunks_mut(span))
                .zip(v.chunks_mut(span))
                .map(|((y, u), v)| [y, u, v]),
            apply,
        );
    } else {
        apply([y, u, v]);
    }
}

fn transform_colour(
    components: &mut [TileComp],
    direct: Option<&mut [Option<&mut [i32]>]>,
    resolutions: &[u32],
    diagnostics: &mut Diagnostics,
    jobs: Option<Jobs<'_>>,
) -> Result<(), Error> {
    let count = components.len();
    let [a, b, c, ..] = components else {
        diagnostics.error_fmt(format_args!(
            "Number of components ({count}) is inconsistent with a MCT. Skip the MCT step.\n"
        ));
        return Ok(());
    };
    let rect = a.selected_resolution();
    let [y, u, v]: [&mut [i32]; 3] = if let Some([Some(y), Some(u), Some(v), ..]) = direct {
        [y, u, v]
    } else {
        [&mut a.samples, &mut b.samples, &mut c.samples]
    };
    if resolutions[0] != resolutions[1]
        || resolutions[0] != resolutions[2]
        || a.minimum_num_resolutions != b.minimum_num_resolutions
        || a.minimum_num_resolutions != c.minimum_num_resolutions
        || y.len() != u.len()
        || y.len() != v.len()
    {
        return Err(data_error(
            "Tiles don't all have the same dimension. Skip the MCT step.\n",
        ));
    }
    let reversible = a.reversible;
    let apply = |planes| mct::inverse(planes, reversible);
    let rows = rect.height() as usize;
    if let Some(jobs) = jobs.filter(|_| y.len() >= OUTPUT_SAMPLE_FLOOR && rows > 1) {
        let span = rows.div_ceil(jobs.width()) * rect.width() as usize;
        jobs.run(
            y.chunks_mut(span)
                .zip(u.chunks_mut(span))
                .zip(v.chunks_mut(span))
                .map(|((y, u), v)| [y, u, v]),
            apply,
        );
    } else {
        apply([y, u, v]);
    }
    Ok(())
}

fn finish_tile_component(
    source: &TileComp,
    resolution: u32,
    samples: &mut [i32],
    finish: mct::Finish,
    jobs: Option<Jobs<'_>>,
) -> bool {
    if samples.is_empty() {
        return false;
    }
    let actual = source.res[resolution.min(source.minimum_num_resolutions - 1) as usize].rect;
    finish_whole(
        samples,
        source.selected_resolution().width() as usize,
        actual,
        finish,
        jobs,
    );
    true
}

fn finish_whole(samples: &mut [i32], stride: usize, actual: Rect, finish: mct::Finish, jobs: Option<Jobs<'_>>) {
    let rows = actual.height() as usize;
    let width = actual.width() as usize;
    let apply = |band: &mut [i32]| {
        if width == stride {
            finish.apply(band);
        } else {
            for row in band.chunks_exact_mut(stride) {
                finish.apply(&mut row[..width]);
            }
        }
    };
    let samples = &mut samples[..rows * stride];
    if let Some(jobs) = output_jobs(jobs, width, rows) {
        jobs.run(samples.chunks_mut(rows.div_ceil(jobs.width()) * stride), apply);
    } else {
        apply(samples);
    }
}

fn copy_component(
    source: &mut TileComp,
    resolution: u32,
    finish: mct::Finish,
    target: &mut DecodedComponent,
    whole: bool,
    jobs: Option<Jobs<'_>>,
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
        finish_whole(&mut source.samples, stride, actual, finish, jobs);
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
    let rows = area.height() as usize;
    let input = &source.samples[source_y * stride..(source_y + rows) * stride];
    let output = &mut target.samples[target_y * target_stride..(target_y + rows) * target_stride];
    let copy = |(input, output): (&[i32], &mut [i32])| {
        for (src, dst) in input.chunks_exact(stride).zip(output.chunks_exact_mut(target_stride)) {
            let src = &src[source_x..source_x + width];
            let dst = &mut dst[target_x..target_x + width];
            for (output, &input) in dst.iter_mut().zip(src) {
                *output = finish.sample(input);
            }
        }
    };
    if let Some(jobs) = output_jobs(jobs, width, rows) {
        let band_rows = rows.div_ceil(jobs.width());
        jobs.run(
            input
                .chunks(band_rows * stride)
                .zip(output.chunks_mut(band_rows * target_stride)),
            copy,
        );
    } else {
        copy((input, output));
    }
    Ok(true)
}

fn output_jobs(jobs: Option<Jobs<'_>>, width: usize, rows: usize) -> Option<Jobs<'_>> {
    jobs.filter(|_| rows > 1 && width * rows >= OUTPUT_SAMPLE_FLOOR)
}

fn clear_plane(samples: &mut [i32], rows: usize, executor: Option<&dyn Executor>) {
    if rows > 1
        && samples.len() >= CLEAR_BYTE_FLOOR / size_of::<i32>()
        && let Some(jobs) = executor.and_then(Jobs::new)
    {
        let span = rows.div_ceil(jobs.width()) * (samples.len() / rows);
        jobs.run(samples.chunks_mut(span), |rows| rows.fill(0));
        return;
    }
    samples.fill(0);
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
    Error::static_message(ErrorKind::NoMemory, "Not enough memory to decode\n")
}

#[cfg(all(test, not(miri)))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

    #[derive(Default)]
    struct Host {
        queries: AtomicUsize,
        calls: AtomicUsize,
    }

    impl Executor for Host {
        fn concurrency(&self) -> usize {
            self.queries.fetch_add(1, Relaxed);
            4
        }

        fn parallel_for(&self, count: usize, work: &(dyn Fn(usize) + Sync)) {
            self.calls.fetch_add(1, Relaxed);
            for index in (0..count).rev() {
                work(index);
            }
        }
    }

    #[test]
    fn plane_clear_uses_the_host_at_four_mebibytes() {
        let host = Host::default();
        let mut samples = vec![17; 1024 * 1024];
        let below = &mut samples[..1024 * 1023];
        clear_plane(below, 1023, Some(&host));
        assert!(below.iter().all(|&sample| sample == 0));
        assert_eq!(host.queries.load(Relaxed), 0);

        samples.fill(19);
        clear_plane(&mut samples, 1024, Some(&host));
        assert!(samples.iter().all(|&sample| sample == 0));
        assert_eq!(host.queries.load(Relaxed), 1);
        assert_eq!(host.calls.load(Relaxed), 1);
    }
}
