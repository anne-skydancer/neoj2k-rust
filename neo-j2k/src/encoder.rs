// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The one-tile JPEG 2000 encoder and its caller-owned work buffers.

use core::{num::NonZeroUsize, ops::Range};
use std::io::Write;

use crate::error::Diagnostics;
use crate::executor::Jobs;
use crate::stream::{EncodePart, StreamWriter};
use crate::{Diagnostic, Error, ErrorKind, Executor};

#[cfg(feature = "bench-internals")]
pub(crate) mod bench;
#[cfg(all(test, not(miri)))]
mod differential;
mod tier1;
mod tier2;
mod wavelet;

const MAX_SIDE: u32 = 4096;
const MAX_SAMPLES: u64 = 1 << 26;
const MAX_COMPONENTS: usize = 8;
const MAX_LAYERS: usize = 100;
const MAX_RESOLUTIONS: usize = 13;
const MAX_BANDS: usize = 3 * MAX_RESOLUTIONS - 2;
const BLOCK_SIDE: usize = 64;
const PARALLEL_ENCODE_SAMPLES: u64 = 1 << 16;
const DEFAULT_COMMENT: &[u8] = b"Created by OpenJPEG version 2.5.4";

/// One eight-bit unsigned plane of a planar input image.
#[derive(Clone, Copy, Debug)]
pub struct Plane<'a> {
    pub samples: &'a [u8],
    pub row_stride: usize,
}

/// Eight-bit unsigned samples, with optional padding at the end of each row.
#[derive(Clone, Copy, Debug)]
pub enum ImageSamples<'a> {
    Interleaved {
        samples: &'a [u8],
        components: u16,
        row_stride: usize,
    },
    Planar(&'a [Plane<'a>]),
}

/// The encoder's input. Components share the image dimensions and are never subsampled.
#[derive(Clone, Copy, Debug)]
pub struct ImageView<'a> {
    pub width: u32,
    pub height: u32,
    /// When true, the first supplied row is the bottom row of the image.
    pub flip_vertical: bool,
    pub samples: ImageSamples<'a>,
}

/// Settings for the one-tile RLCP codestream with 64-by-64 code-blocks.
#[derive(Clone)]
#[non_exhaustive]
pub struct EncodeParams<'a> {
    /// True selects the reversible 5/3 wavelet. False selects the irreversible 9/7 wavelet.
    pub reversible: bool,
    /// Number of resolutions. The default is six or the most that fit, whichever is smaller.
    pub resolutions: Option<u8>,
    /// Apply the standard colour transform to the first three components.
    pub mct: bool,
    /// Requested compression ratio per quality layer, coarsest first. Empty, zero or a ratio of
    /// at most one requests all of the corresponding layer's passes.
    pub layer_ratios: Vec<f32>,
    /// A size cap expressed as a minimum compression ratio. Zero leaves the ratios unchanged.
    pub max_codestream_size: u32,
    /// The COM marker's text. As in the C++ encoder, only bytes before the first NUL are written.
    pub comment: Option<Vec<u8>>,
    /// OpenJPEG-style `KEY=VALUE` options. The core accepts `PLT=NO`, `TLM=NO`, and `GUARD_BITS=2`.
    /// Options are checked in order after setup and before any output is written. NUL ends an option.
    pub extra_options: Vec<String>,
    /// A host scheduler for images of at least 256 by 256 samples per component. The encoder
    /// creates no threads of its own; smaller images remain on the calling thread.
    pub executor: Option<&'a dyn Executor>,
}

impl core::fmt::Debug for EncodeParams<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EncodeParams")
            .field("reversible", &self.reversible)
            .field("resolutions", &self.resolutions)
            .field("mct", &self.mct)
            .field("layer_ratios", &self.layer_ratios)
            .field("max_codestream_size", &self.max_codestream_size)
            .field("comment", &self.comment)
            .field("extra_options", &self.extra_options)
            .field("has_executor", &self.executor.is_some())
            .finish()
    }
}

impl Default for EncodeParams<'_> {
    fn default() -> Self {
        Self {
            reversible: true,
            resolutions: None,
            mct: false,
            layer_ratios: Vec::new(),
            max_codestream_size: 0,
            comment: None,
            extra_options: Vec::new(),
            executor: None,
        }
    }
}

/// A reusable JPEG 2000 encoder.
#[derive(Debug, Default)]
pub struct Encoder {
    buffers: EncodingBuffers,
    tier1: tier1::Workspace,
    tier1_workers: Vec<EncodeWorker>,
    tier2: tier2::Workspace,
    diagnostics: Diagnostics,
}

#[derive(Debug, Default)]
struct EncodingBuffers {
    layout: TileLayout,
    blocks: Vec<Block>,
    block_jobs: Vec<BlockJob>,
    samples: Vec<i32>,
    wavelet_line: Vec<i32>,
    wavelet_lines: Vec<Vec<i32>>,
    output: Vec<u8>,
}

#[derive(Debug, Default)]
struct EncodeWorker {
    tier1: tier1::Workspace,
    zigzag: Vec<i32>,
    error: Option<Error>,
}

#[derive(Clone, Copy, Debug)]
struct BlockJob {
    sample_start: usize,
    width: usize,
    height: usize,
    params: T1Params,
}

impl Encoder {
    /// Construct an encoder. The first call allocates its working buffers.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Messages collected during the last encode request.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.diagnostics.entries()
    }

    /// Encode one image. The codestream is borrowed from the encoder until its next use.
    pub fn encode(&mut self, image: &ImageView<'_>, params: &EncodeParams<'_>) -> Result<&[u8], Error> {
        self.encode_with(image, params, |_, _| {}, |_, _| Ok(()))?;
        Ok(&self.buffers.output)
    }

    /// Encodes one image and writes back the effective layer ratios and codestream size cap.
    ///
    /// Accepted settings are updated even if writing or coding fails later. A size cap inferred
    /// from the last requested layer applies only when the updated settings are used again.
    pub fn encode_and_update(&mut self, image: &ImageView<'_>, params: &mut EncodeParams<'_>) -> Result<&[u8], Error> {
        self.encode_with_updates(image, params, |_, _| Ok(()))?;
        Ok(&self.buffers.output)
    }

    /// Encodes one image to a writer, offering chunks of up to `buffer_size` bytes.
    ///
    /// Retries short writes and `Interrupted`, then flushes the writer. On an output error, the
    /// writer may already have received part of the codestream. The encoder can encode another
    /// image after a failure.
    pub fn encode_to(
        &mut self,
        image: &ImageView<'_>,
        params: &EncodeParams<'_>,
        writer: &mut (impl Write + ?Sized),
        buffer_size: NonZeroUsize,
    ) -> Result<(), Error> {
        let mut output = StreamWriter::new(writer, buffer_size);
        self.encode_with(image, params, |_, _| {}, |part, bytes| output.write(part, bytes))
    }

    /// Encodes to a writer and updates the settings as [`Self::encode_and_update`] does.
    pub fn encode_to_and_update(
        &mut self,
        image: &ImageView<'_>,
        params: &mut EncodeParams<'_>,
        writer: &mut (impl Write + ?Sized),
        buffer_size: NonZeroUsize,
    ) -> Result<(), Error> {
        let mut output = StreamWriter::new(writer, buffer_size);
        self.encode_with_updates(image, params, |part, bytes| output.write(part, bytes))
    }

    fn encode_with_updates(
        &mut self,
        image: &ImageView<'_>,
        params: &mut EncodeParams<'_>,
        write: impl FnMut(EncodePart, &[u8]) -> Result<(), Error>,
    ) -> Result<(), Error> {
        // Only the implicit lossless layer can grow the caller's vector. Reserve it before any
        // codestream bytes can be written, so propagating accepted settings cannot fail later.
        if params.layer_ratios.is_empty() && params.layer_ratios.try_reserve_exact(1).is_err() {
            self.diagnostics.clear();
            let error = memory_error();
            self.diagnostics.error(error.message());
            return Err(error);
        }
        let mut ratios = [0.0; MAX_LAYERS];
        let mut layers = 0;
        let mut cap = 0;
        let result = self.encode_with(
            image,
            params,
            |resolved, size| {
                layers = resolved.len();
                ratios[..layers].copy_from_slice(resolved);
                cap = size;
            },
            write,
        );
        if layers != 0 {
            params.layer_ratios.clear();
            params.layer_ratios.extend_from_slice(&ratios[..layers]);
            params.max_codestream_size = cap;
        }
        result
    }

    fn encode_with(
        &mut self,
        image: &ImageView<'_>,
        params: &EncodeParams<'_>,
        update: impl FnOnce(&[f32], u32),
        write: impl FnMut(EncodePart, &[u8]) -> Result<(), Error>,
    ) -> Result<(), Error> {
        self.diagnostics.clear();
        if let Err(error) = self.encode_inner(image, params, update, write) {
            self.diagnostics.error(error.message());
            return Err(error);
        }
        Ok(())
    }

    fn encode_inner(
        &mut self,
        image: &ImageView<'_>,
        params: &EncodeParams<'_>,
        update: impl FnOnce(&[f32], u32),
        mut write: impl FnMut(EncodePart, &[u8]) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let jobs = if u64::from(image.width) * u64::from(image.height) >= PARALLEL_ENCODE_SAMPLES {
            params.executor.and_then(Jobs::new)
        } else {
            None
        };
        let settings = self
            .buffers
            .prepare(image, params, jobs, &mut self.diagnostics, update)?;
        write(EncodePart::Header, &self.buffers.output)?;
        if let Some(jobs) = jobs {
            self.buffers
                .encode_blocks_parallel(&settings, jobs, &mut self.tier1_workers)?;
        } else {
            let code_blocks = &mut self.tier1;
            let encode = if settings.all_passes {
                tier1::encode_all_passes
            } else {
                tier1::encode
            };
            self.buffers
                .encode_blocks(&settings, |samples, width, height, params, block| {
                    encode(code_blocks, samples, width, height, params, block)
                })?;
        }
        let packets = &mut self.tier2;
        let output = self
            .buffers
            .finish_packets(&settings, |layout, blocks, rates, output| {
                tier2::encode_packets(packets, layout, blocks, rates, output)
            })?;
        write(EncodePart::Tile, &output[..output.len() - 2])?;
        write(EncodePart::End, output)
    }
}

impl EncodingBuffers {
    fn prepare<'p>(
        &mut self,
        image: &ImageView<'_>,
        params: &'p EncodeParams<'_>,
        jobs: Option<Jobs<'_>>,
        diagnostics: &mut Diagnostics,
        update: impl FnOnce(&[f32], u32),
    ) -> Result<Settings<'p>, Error> {
        let settings = Settings::new(image, params, diagnostics, update)?;
        let plane_len = settings.width * settings.height;
        resize_with(&mut self.samples, plane_len * settings.components, || 0)?;
        image.copy_samples(&settings, &mut self.samples);
        if settings.mct {
            wavelet::colour_transform(&mut self.samples, plane_len, settings.reversible);
        }
        if settings.resolutions > 1 {
            let line_len = wavelet::scratch_len(settings.width, settings.height);
            if let Some(jobs) = jobs.filter(|_| settings.components > 1) {
                resize_with(&mut self.wavelet_lines, settings.components, Vec::new)?;
                for line in &mut self.wavelet_lines {
                    resize_with(line, line_len, || 0)?;
                }
                let parts = self
                    .samples
                    .chunks_exact_mut(plane_len)
                    .zip(self.wavelet_lines.iter_mut());
                jobs.run(parts, |(plane, line)| {
                    wavelet::forward(
                        plane,
                        settings.width,
                        settings.height,
                        settings.resolutions,
                        settings.reversible,
                        line,
                    );
                });
            } else {
                resize_with(&mut self.wavelet_line, line_len, || 0)?;
                for plane in self.samples.chunks_exact_mut(plane_len) {
                    wavelet::forward(
                        plane,
                        settings.width,
                        settings.height,
                        settings.resolutions,
                        settings.reversible,
                        &mut self.wavelet_line,
                    );
                }
            }
        }
        let count = self.layout.reset(&settings)?;
        resize_with(&mut self.blocks, count, Block::default)?;
        self.reset_block_jobs(&settings, count)?;

        self.output.clear();
        self.output
            .try_reserve_exact(settings.header_size() + settings.tile_capacity + 2)
            .map_err(|_| memory_error())?;
        settings.write_header(&mut self.output);
        Ok(settings)
    }

    fn encode_blocks(
        &mut self,
        settings: &Settings<'_>,
        mut encode_block: impl FnMut(&[i32], usize, usize, T1Params, &mut Block) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let mut zigzag = [0; BLOCK_SIDE * BLOCK_SIDE];
        let stride = settings.width;
        for (component, plane) in self.samples.chunks_exact(stride * settings.height).enumerate() {
            for resolution in 0..settings.resolutions {
                let level = settings.resolutions - 1 - resolution;
                for band in self.layout.bands(component, resolution) {
                    let params = T1Params {
                        orientation: band.orientation,
                        level,
                        component,
                        reversible: settings.reversible,
                        step_size: band.step_size,
                        mct: settings.mct,
                    };
                    for (index, block) in self.blocks[band.blocks.clone()].iter_mut().enumerate() {
                        let x = (index % band.block_columns) * BLOCK_SIDE;
                        let y = (index / band.block_columns) * BLOCK_SIDE;
                        let width = BLOCK_SIDE.min(band.width - x);
                        let height = BLOCK_SIDE.min(band.height - y);
                        let start = (band.y + y) * stride + band.x + x;
                        let zigzag = &mut zigzag[..width * height];
                        gather_samples(&plane[start..], stride, width, height, params, zigzag);
                        encode_block(zigzag, width, height, params, block)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn encode_blocks_parallel(
        &mut self,
        settings: &Settings<'_>,
        jobs: Jobs<'_>,
        workers: &mut Vec<EncodeWorker>,
    ) -> Result<(), Error> {
        let width = jobs.width();
        resize_with(workers, width, EncodeWorker::default)?;
        for worker in &mut *workers {
            resize_with(&mut worker.zigzag, BLOCK_SIDE * BLOCK_SIDE, || 0)?;
        }

        let encode = if settings.all_passes {
            tier1::encode_all_passes
        } else {
            tier1::encode
        };
        let samples = &self.samples;
        let blocks = &mut self.blocks;
        let block_jobs = &self.block_jobs;
        for (batch, descriptors) in blocks.chunks_mut(width).zip(block_jobs.chunks(width)) {
            let active = batch.len();
            for worker in &mut workers[..active] {
                worker.error = None;
            }
            let parts = workers[..active]
                .iter_mut()
                .zip(batch.iter_mut())
                .zip(descriptors.iter().copied());
            jobs.run(parts, |((worker, block), descriptor)| {
                let zigzag = &mut worker.zigzag[..descriptor.width * descriptor.height];
                gather_samples(
                    &samples[descriptor.sample_start..],
                    settings.width,
                    descriptor.width,
                    descriptor.height,
                    descriptor.params,
                    zigzag,
                );
                worker.error = encode(
                    &mut worker.tier1,
                    zigzag,
                    descriptor.width,
                    descriptor.height,
                    descriptor.params,
                    block,
                )
                .err();
            });
            // Report the first failure in code-block order, independent of scheduling.
            for worker in &mut workers[..active] {
                if let Some(error) = worker.error.take() {
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    fn reset_block_jobs(&mut self, settings: &Settings<'_>, count: usize) -> Result<(), Error> {
        self.block_jobs.clear();
        self.block_jobs.try_reserve_exact(count).map_err(|_| memory_error())?;
        let stride = settings.width;
        let plane_len = stride * settings.height;
        for component in 0..settings.components {
            for resolution in 0..settings.resolutions {
                let level = settings.resolutions - 1 - resolution;
                for band in self.layout.bands(component, resolution) {
                    let params = T1Params {
                        orientation: band.orientation,
                        level,
                        component,
                        reversible: settings.reversible,
                        step_size: band.step_size,
                        mct: settings.mct,
                    };
                    for index in 0..band.blocks.len() {
                        let x = (index % band.block_columns) * BLOCK_SIDE;
                        let y = (index / band.block_columns) * BLOCK_SIDE;
                        let width = BLOCK_SIDE.min(band.width - x);
                        let height = BLOCK_SIDE.min(band.height - y);
                        self.block_jobs.push(BlockJob {
                            sample_start: component * plane_len + (band.y + y) * stride + band.x + x,
                            width,
                            height,
                            params,
                        });
                    }
                }
            }
        }
        debug_assert_eq!(self.block_jobs.len(), count);
        Ok(())
    }

    fn finish_packets(
        &mut self,
        settings: &Settings<'_>,
        encode_packets: impl FnOnce(&TileLayout, &[Block], &[f32], &mut [u8]) -> Result<usize, Error>,
    ) -> Result<&[u8], Error> {
        let tile_start = self.output.len();
        let tile_end = tile_start + settings.tile_capacity;
        self.output.resize(tile_end, 0);
        let packets_start = tile_start + 14;
        let packets = &mut self.output[packets_start..tile_end - 2];
        let len = encode_packets(&self.layout, &self.blocks, &settings.rates[..settings.layers], packets)?;
        if len > packets.len() {
            return Err(Error::new(ErrorKind::Limit, "Cannot encode tile\n"));
        }
        let psot = u32::try_from(14 + len).expect("the validated tile capacity fits in u32");
        self.output[tile_start..tile_start + 6].copy_from_slice(&[0xff, 0x90, 0, 10, 0, 0]); // SOT, Lsot, Isot.
        self.output[tile_start + 6..tile_start + 10].copy_from_slice(&psot.to_be_bytes());
        self.output[tile_start + 10..packets_start].copy_from_slice(&[0, 1, 0xff, 0x93]); // TPsot, TNsot, SOD.
        self.output.truncate(packets_start + len);
        put16(&mut self.output, 0xffd9); // EOC.
        Ok(&self.output)
    }
}

impl ImageView<'_> {
    fn component_count(&self) -> usize {
        match self.samples {
            ImageSamples::Interleaved { components, .. } => usize::from(components),
            ImageSamples::Planar(planes) => planes.len(),
        }
    }

    fn validate_samples(&self, size: &Settings<'_>) -> Result<(), Error> {
        let fits = |samples: &[u8], row_stride: usize, row_len: usize| {
            row_stride >= row_len
                && (size.height - 1)
                    .checked_mul(row_stride)
                    .and_then(|offset| offset.checked_add(row_len))
                    .is_some_and(|required| required <= samples.len())
        };
        let ok = match self.samples {
            ImageSamples::Interleaved {
                samples, row_stride, ..
            } => fits(samples, row_stride, size.width * size.components),
            ImageSamples::Planar(planes) => planes.iter().all(|p| fits(p.samples, p.row_stride, size.width)),
        };
        if !ok {
            return Err(Error::new(
                ErrorKind::Argument,
                "The samples do not cover the image dimensions and stride\n",
            ));
        }
        Ok(())
    }

    fn copy_samples(&self, settings: &Settings<'_>, target: &mut [i32]) {
        let size = settings.width * settings.height;
        for (component, plane) in target.chunks_exact_mut(size).enumerate() {
            let (data, stride, step, shift) = match self.samples {
                ImageSamples::Interleaved {
                    samples, row_stride, ..
                } => (samples, row_stride, settings.components, component),
                ImageSamples::Planar(planes) => (planes[component].samples, planes[component].row_stride, 1, 0),
            };
            for (y, row) in plane.chunks_exact_mut(settings.width).enumerate() {
                let row_number = if self.flip_vertical { settings.height - 1 - y } else { y };
                let start = row_number * stride + shift;
                for (x, out) in row.iter_mut().enumerate() {
                    *out = wavelet::shift_sample(data[start + x * step], settings.reversible);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Settings<'p> {
    width: usize,
    height: usize,
    components: usize,
    resolutions: usize,
    layers: usize,
    reversible: bool,
    mct: bool,
    steps: [wavelet::Step; MAX_BANDS],
    rates: [f32; MAX_LAYERS],
    /// Every layer's effective rate is zero, so every pass is kept and no distortion is read.
    all_passes: bool,
    comment: &'p [u8],
    tile_capacity: usize,
}

impl<'p> Settings<'p> {
    fn new(
        image: &ImageView<'_>,
        params: &'p EncodeParams<'_>,
        diagnostics: &mut Diagnostics,
        update: impl FnOnce(&[f32], u32),
    ) -> Result<Self, Error> {
        let components = image.component_count();
        if components == 0 {
            return Err(Error::new(ErrorKind::Argument, "nj2k_setup_encoder: no image\n"));
        }
        if components > MAX_COMPONENTS {
            return Err(Error::new(
                ErrorKind::Limit,
                format!("nj2k_setup_encoder: {components} components, at most {MAX_COMPONENTS}\n"),
            ));
        }
        let (w, h) = (image.width, image.height);
        if w == 0 || h == 0 {
            return Err(Error::new(ErrorKind::Argument, "nj2k_setup_encoder: an empty image\n"));
        }
        if w > MAX_SIDE || h > MAX_SIDE || u64::from(w) * u64::from(h) * components as u64 > MAX_SAMPLES {
            return Err(Error::new(
                ErrorKind::Limit,
                format!(
                    "nj2k_setup_encoder: a {w} x {h} image of {components} components is over the encoder's limits ({MAX_SIDE} per side, {MAX_SAMPLES} samples)\n"
                ),
            ));
        }
        if params.mct && components < 3 {
            return Err(Error::new(
                ErrorKind::Argument,
                "nj2k_setup_encoder: the colour transform needs three components\n",
            ));
        }
        let max_levels = w.min(h).ilog2() + 1;
        let levels = params.resolutions.map_or(max_levels.min(6), u32::from);
        if !(1..=33).contains(&levels) {
            return Err(Error::new(
                ErrorKind::Argument,
                format!("Invalid number of resolutions : {levels} not in range [1,33]\n"),
            ));
        }
        let mut layers = params.layer_ratios.len();
        if layers > MAX_LAYERS {
            return Err(Error::new(
                ErrorKind::Limit,
                format!("nj2k_setup_encoder: {layers} layers, at most {MAX_LAYERS}\n"),
            ));
        }
        let comment = params.comment.as_deref().map_or(DEFAULT_COMMENT, |bytes| {
            bytes.split(|&byte| byte == 0).next().unwrap_or_default()
        });
        if comment.len() > usize::from(u16::MAX) - 4 {
            return Err(Error::new(
                ErrorKind::Argument,
                "nj2k_setup_encoder: the comment is longer than a marker segment holds\n",
            ));
        }
        if levels > max_levels {
            return Err(Error::new(
                ErrorKind::Argument,
                "Number of resolutions is too high in comparison to the size of tiles\n",
            ));
        }
        if params.max_codestream_size > i32::MAX.unsigned_abs() {
            return Err(Error::new(
                ErrorKind::Argument,
                "The codestream size cap must fit in a signed 32-bit integer\n",
            ));
        }

        let resolutions = usize::try_from(levels).expect("encoder resolution count fits in usize");
        let width = usize::try_from(w).expect("encoder width fits in usize");
        let height = usize::try_from(h).expect("encoder height fits in usize");
        let samples = w * h * u32::try_from(components).expect("encoder component count fits in u32");
        let mut rates = [0.0; MAX_LAYERS];
        rates[..layers].copy_from_slice(&params.layer_ratios);
        layers = layers.max(1);
        let mut settings = Self {
            width,
            height,
            components,
            resolutions,
            layers,
            reversible: params.reversible,
            mct: params.mct,
            steps: [wavelet::Step::default(); MAX_BANDS],
            rates,
            all_passes: false,
            comment,
            tile_capacity: tile_capacity(samples, components),
        };
        let written_cap = settings.setup_rates(samples, params.max_codestream_size, diagnostics);
        update(&settings.rates[..settings.layers], written_cap);
        validate_extra_options(&params.extra_options)?;
        image.validate_samples(&settings)?;
        settings.update_rates(samples);
        settings.all_passes = tier2::takes_all_passes(&settings.rates[..settings.layers]);
        for (index, step) in settings.steps[..3 * resolutions - 2].iter_mut().enumerate() {
            let resolution = if index == 0 { 0 } else { (index - 1) / 3 + 1 };
            let level = resolutions - 1 - resolution;
            let orientation = if index == 0 {
                0
            } else {
                u8::try_from((index - 1) % 3 + 1).unwrap()
            };
            *step = wavelet::step(level, orientation, settings.reversible);
        }
        Ok(settings)
    }

    fn header_size(&self) -> usize {
        let band_count = 3 * self.resolutions - 2;
        let qcd = if self.reversible { band_count } else { 2 * band_count };
        2 + 40 + 3 * self.components + 14 + 5 + qcd + 6 + self.comment.len()
    }

    fn write_header(&self, dest: &mut Vec<u8>) {
        let width = u32::try_from(self.width).expect("validated encoder width");
        let height = u32::try_from(self.height).expect("validated encoder height");
        let components = u16::try_from(self.components).expect("validated encoder component count");
        let layers = u16::try_from(self.layers).expect("validated encoder layer count");
        let decompositions = u8::try_from(self.resolutions - 1).expect("validated encoder resolution count");
        let band_count = 3 * usize::from(decompositions) + 1;
        let comment_length = u16::try_from(self.comment.len()).expect("validated comment length") + 4;

        put16(dest, 0xff4f); // SOC
        put16(dest, 0xff51); // SIZ
        put16(dest, 38 + 3 * components);
        put16(dest, 0);
        for value in [width, height, 0, 0, width, height, 0, 0] {
            dest.extend(value.to_be_bytes());
        }
        put16(dest, components);
        for _ in 0..components {
            dest.extend([7, 1, 1]); // Eight-bit unsigned, no subsampling.
        }

        put16(dest, 0xff52); // COD
        put16(dest, 12);
        dest.extend([0, 1]); // No style switches, RLCP progression.
        put16(dest, layers);
        dest.extend([u8::from(self.mct), decompositions, 4, 4, 0, u8::from(self.reversible)]);

        put16(dest, 0xff5c); // QCD
        let qcd_bytes = if self.reversible { band_count } else { 2 * band_count };
        put16(dest, u16::try_from(3 + qcd_bytes).expect("QCD length fits in u16"));
        dest.push(if self.reversible { 64 } else { 66 }); // Two guard bits.
        for step in &self.steps[..band_count] {
            if self.reversible {
                dest.push(step.exponent << 3);
            } else {
                put16(dest, (u16::from(step.exponent) << 11) | step.mantissa);
            }
        }

        put16(dest, 0xff64); // COM
        put16(dest, comment_length);
        put16(dest, 1);
        dest.extend(self.comment);
    }

    #[allow(
        clippy::cast_possible_truncation,
        reason = "the C++ rate targets are rounded to float32"
    )]
    fn setup_rates(&mut self, samples: u32, cap: u32, diagnostics: &mut Diagnostics) -> u32 {
        let ratios = &mut self.rates[..self.layers];
        if cap == 0 {
            let last = ratios[self.layers - 1];
            if last > 0.0 {
                let size = (f64::from(samples) / f64::from(last)) as f32;
                #[allow(clippy::cast_sign_loss, reason = "the quotient is nonnegative")]
                return (size.floor() as u32).min(i32::MAX.unsigned_abs());
            }
            return 0;
        }
        let minimum_ratio = (f64::from(samples) / f64::from(cap)) as f32;
        let mut capped = false;
        for ratio in ratios.iter_mut().filter(|ratio| **ratio < minimum_ratio) {
            *ratio = minimum_ratio;
            capped = true;
        }
        if capped {
            diagnostics
                .warn("The desired maximum codestream size has limited\nat least one of the desired quality layers\n");
        }
        cap
    }

    #[allow(
        clippy::cast_possible_truncation,
        reason = "the C++ rate targets are rounded to float32"
    )]
    fn update_rates(&mut self, samples: u32) {
        let header = u32::try_from(self.header_size()).expect("encoder header fits in u32") as f32;
        let ratios = &mut self.rates[..self.layers];
        for ratio in ratios.iter_mut() {
            if *ratio <= 1.0 {
                *ratio = 0.0;
            } else if *ratio > 0.0 {
                *ratio = ((f64::from(samples) * 8.0) / f64::from(*ratio * 8.0)) as f32;
            }
        }
        if ratios[0] > 0.0 {
            ratios[0] = (ratios[0] - header).max(30.0);
        }
        for index in 1..ratios.len() {
            if ratios[index] > 0.0 {
                let remove = if index + 1 == ratios.len() {
                    header + 2.0
                } else {
                    header
                };
                ratios[index] -= remove;
                if ratios[index] < ratios[index - 1] + 10.0 {
                    ratios[index] = ratios[index - 1] + 20.0;
                }
            }
        }
    }
}

fn validate_extra_options(options: &[String]) -> Result<(), Error> {
    for option in options {
        let option = option.split('\0').next().unwrap_or_default();
        if matches!(option, "PLT=NO" | "TLM=NO" | "GUARD_BITS=2") {
            continue;
        }
        let (kind, prefix, suffix) = if matches!(option, "PLT=YES" | "TLM=YES")
            || option.strip_prefix("GUARD_BITS=").is_some_and(guard_bits_in_range)
        {
            (
                ErrorKind::Unsupported,
                "nj2k_encoder_set_extra_options: ",
                " is not supported by the core encoder (it belongs to the j2k-extra module)\n",
            )
        } else {
            (ErrorKind::Argument, "Invalid option: ", ".\n")
        };
        // The C++ message has 511 payload bytes. Preserve a UTF-8 boundary if one spans the limit.
        let limit = 511 - prefix.len();
        let end = option.floor_char_boundary(limit);
        let suffix = if end == option.len() {
            &suffix[..suffix.len().min(limit - end)]
        } else {
            ""
        };
        return Err(Error::formatted(
            kind,
            format_args!("{prefix}{}{suffix}", &option[..end]),
            "Invalid or unsupported encoder option\n",
        ));
    }
    Ok(())
}

fn guard_bits_in_range(value: &str) -> bool {
    // The C++ option parser uses atoi, which accepts a sign and stops at the first non-digit.
    let value = value.trim_start_matches([' ', '\t', '\n', '\r', '\u{b}', '\u{c}']);
    let (negative, digits) = match value.as_bytes() {
        [b'-', rest @ ..] => (true, rest),
        [b'+', rest @ ..] => (false, rest),
        digits => (false, digits),
    };
    let magnitude = digits
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .fold(0_u8, |value, &digit| {
            value.saturating_mul(10).saturating_add(digit - b'0')
        });
    magnitude <= if negative { 0 } else { 7 }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C++ tile byte estimate is floored in the validated encoder range"
)]
fn tile_capacity(samples: u32, components: usize) -> usize {
    (f64::from(samples) * 1.4) as usize + 500 + 12 + (components - 1) * 22 + 13
}

#[derive(Debug, Default)]
pub(super) struct TileLayout {
    pub(super) components: usize,
    pub(super) resolutions: usize,
    bands: Vec<Band>,
}

#[derive(Debug)]
pub(super) struct Band {
    pub(super) orientation: u8,
    pub(super) numbps: i32,
    pub(super) block_columns: usize,
    pub(super) block_rows: usize,
    pub(super) blocks: Range<usize>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    step_size: f32,
}

impl TileLayout {
    pub(super) fn bands(&self, component: usize, resolution: usize) -> &[Band] {
        let component_start = component * (3 * self.resolutions - 2);
        if resolution == 0 {
            &self.bands[component_start..component_start + 1]
        } else {
            let start = component_start + 1 + (resolution - 1) * 3;
            &self.bands[start..start + 3]
        }
    }

    fn reset(&mut self, settings: &Settings<'_>) -> Result<usize, Error> {
        self.components = settings.components;
        self.resolutions = settings.resolutions;
        self.bands.clear();
        self.bands
            .try_reserve_exact(self.components * (3 * self.resolutions - 2))
            .map_err(|_| memory_error())?;
        let mut blocks = 0;
        for _ in 0..self.components {
            let mut step = settings.steps.iter();
            for res in 0..self.resolutions {
                let divisor = 1 << (self.resolutions - 1 - res);
                let cols = settings.width.div_ceil(divisor);
                let rows = settings.height.div_ceil(divisor);
                let orientations = if res == 0 { 0..1 } else { 1..4 };
                for orientation in orientations {
                    let (x, width) = band_axis(cols, orientation & 1 != 0, res != 0);
                    let (y, height) = band_axis(rows, orientation & 2 != 0, res != 0);
                    let block_columns = width.div_ceil(BLOCK_SIDE);
                    let block_rows = height.div_ceil(BLOCK_SIDE);
                    let count = block_columns * block_rows;
                    let quant = *step.next().expect("one step per coder band");
                    self.bands.push(Band {
                        orientation,
                        numbps: i32::from(quant.exponent) + 1,
                        block_columns,
                        block_rows,
                        blocks: blocks..blocks + count,
                        x,
                        y,
                        width,
                        height,
                        step_size: quant.size(orientation),
                    });
                    blocks += count;
                }
            }
        }
        Ok(blocks)
    }
}

fn band_axis(length: usize, high: bool, split: bool) -> (usize, usize) {
    if !split {
        (0, length)
    } else if high {
        (length.div_ceil(2), length / 2)
    } else {
        (0, length.div_ceil(2))
    }
}

#[derive(Debug, Default)]
pub(super) struct Block {
    pub(super) numbps: u32,
    pub(super) data: Vec<u8>,
    pub(super) passes: Vec<Pass>,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Pass {
    pub(super) rate: u32,
    pub(super) len: u32,
    pub(super) distortion_dec: f64,
    pub(super) terminated: bool,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct T1Params {
    pub(super) orientation: u8,
    pub(super) level: usize,
    pub(super) component: usize,
    pub(super) reversible: bool,
    pub(super) step_size: f32,
    pub(super) mct: bool,
}

fn gather_samples(plane: &[i32], stride: usize, width: usize, height: usize, params: T1Params, dst: &mut [i32]) {
    let mut dst = dst.iter_mut();
    for band in (0..height).step_by(4) {
        for column in 0..width {
            for row in band..(band + 4).min(height) {
                let sample = plane[row * stride + column];
                let value = if params.reversible {
                    sample.wrapping_shl(6)
                } else {
                    quantize(sample, params.step_size)
                };
                *dst.next().expect("one destination per code-block sample") = value;
            }
        }
    }
}

#[allow(
    clippy::cast_possible_truncation,
    reason = "the bounds check mirrors the C++ float-to-int conversion"
)]
fn quantize(sample: i32, step_size: f32) -> i32 {
    let value = ((wavelet::from_sample(sample) / step_size) * 64.0).round_ties_even();
    if (-2_147_483_648.0..2_147_483_648.0).contains(&value) {
        value as i32
    } else {
        i32::MIN
    }
}

fn put16(dest: &mut Vec<u8>, value: u16) {
    dest.extend(value.to_be_bytes());
}

pub(super) fn memory_error() -> Error {
    Error::static_message(ErrorKind::NoMemory, "Not enough memory to encode\n")
}

fn resize_with<T>(data: &mut Vec<T>, len: usize, value: impl FnMut() -> T) -> Result<(), Error> {
    if len > data.len() {
        data.try_reserve_exact(len - data.len()).map_err(|_| memory_error())?;
    }
    data.resize_with(len, value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_padded_interleaved_and_planar_rows_with_vertical_flip() {
        let interleaved = [10, 11, 20, 21, 99, 30, 31, 40, 41];
        let planar0 = [10, 20, 99, 99, 30, 40];
        let planar1 = [11, 21, 31, 41];
        let planar = [
            Plane {
                samples: &planar0,
                row_stride: 4,
            },
            Plane {
                samples: &planar1,
                row_stride: 2,
            },
        ];
        let mut encoder = EncodingBuffers::default();
        let mut diagnostics = Diagnostics::default();
        let params = EncodeParams {
            resolutions: Some(1),
            ..EncodeParams::default()
        };
        let mut image = ImageView {
            width: 2,
            height: 2,
            flip_vertical: true,
            samples: ImageSamples::Interleaved {
                samples: &interleaved,
                components: 2,
                row_stride: 5,
            },
        };
        encoder
            .prepare(&image, &params, None, &mut diagnostics, |_, _| {})
            .unwrap();
        let expected = [-98, -88, -118, -108, -97, -87, -117, -107];
        assert_eq!(encoder.samples, expected);
        image.samples = ImageSamples::Planar(&planar);
        encoder
            .prepare(&image, &params, None, &mut diagnostics, |_, _| {})
            .unwrap();
        assert_eq!(encoder.samples, expected);
    }

    #[test]
    fn reversible_wavelet_transforms_columns_then_rows() {
        let mut plane = [1, 2, 3, 4];
        wavelet::forward(&mut plane, 2, 2, 2, true, &mut [0; 2]);
        assert_eq!(plane, [3, 1, 2, 0]);
    }

    #[test]
    fn prep_builds_striped_code_blocks_and_counts_the_tile_part() {
        let data = [128, 129, 130, 131, 132, 133, 134, 135, 136, 137];
        let image = ImageView {
            width: 2,
            height: 5,
            flip_vertical: false,
            samples: ImageSamples::Interleaved {
                samples: &data,
                components: 1,
                row_stride: 2,
            },
        };
        let mut encoder = EncodingBuffers::default();
        let params = EncodeParams {
            resolutions: Some(1),
            comment: Some(b"test\0ignored".to_vec()),
            ..EncodeParams::default()
        };
        let settings = encoder
            .prepare(&image, &params, None, &mut Diagnostics::default(), |_, _| {})
            .unwrap();
        assert_eq!(encoder.output.len(), settings.header_size());
        assert!(
            encoder
                .output
                .ends_with(&[0xff, 0x64, 0, 8, 0, 1, b't', b'e', b's', b't'])
        );
        encoder
            .encode_blocks(&settings, |samples, width, height, options, _| {
                assert_eq!((width, height), (2, 5));
                assert_eq!(options.orientation, 0);
                let samples: Vec<_> = samples.iter().map(|v| v >> 6).collect();
                assert_eq!(samples, [0, 2, 4, 6, 1, 3, 5, 7, 8, 9]);
                Ok(())
            })
            .unwrap();
        let tile = settings.header_size();
        let output = encoder
            .finish_packets(&settings, |_, _, rates, dst| {
                assert_eq!(rates, &[0.0]);
                dst[..3].copy_from_slice(&[0x12, 0x34, 0x56]);
                Ok(3)
            })
            .unwrap();
        assert_eq!(
            &output[tile..],
            &[
                0xff, 0x90, 0, 10, 0, 0, 0, 0, 0, 17, 0, 1, 0xff, 0x93, 0x12, 0x34, 0x56, 0xff, 0xd9
            ]
        );
    }

    #[test]
    fn cap_lifts_layers_in_order_using_the_actual_main_header_size() {
        let input = [128; 64 * 64];
        let image = ImageView {
            width: 64,
            height: 64,
            flip_vertical: false,
            samples: ImageSamples::Interleaved {
                samples: &input,
                components: 1,
                row_stride: 64,
            },
        };
        let params = EncodeParams {
            resolutions: Some(1),
            max_codestream_size: 512,
            layer_ratios: vec![2.0, 4.0, 0.0],
            ..EncodeParams::default()
        };
        let mut diagnostics = Diagnostics::default();
        let settings = Settings::new(&image, &params, &mut diagnostics, |_, _| {}).unwrap();
        let first = f32::from(512 - u16::try_from(settings.header_size()).unwrap());
        assert_eq!(&settings.rates[..3], &[first, first + 20.0, first + 40.0]);
        assert_eq!(diagnostics.entries().len(), 1);
    }
}
