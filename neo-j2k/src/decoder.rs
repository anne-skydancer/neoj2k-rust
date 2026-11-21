// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use std::{fmt, io::Read};

use crate::decode::{
    self, DecodeParams, DecodeStatistics, DecodeTarget, DecodedImage, I32Plane, Jp2Channels, SampleType, TypedOutput,
};
use crate::error::Diagnostics;
use crate::geometry::{self, Dimensions};
use crate::header::{CodingStyle, HeaderSettings, MainHeader, Quantization, parse_main_header};
use crate::jp2::{self, ColourSpec, Jp2Header};
use crate::mem::{CType, HeaderLedger};
use crate::stream::Source;
use crate::{Diagnostic, Error, ErrorKind, Limits, StreamOptions};

/// The input format expected by a decoder.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Format {
    /// A bare JPEG 2000 codestream.
    #[default]
    J2k,
    /// A JP2 or JPH file containing a JPEG 2000 codestream.
    Jp2,
}

/// What to do when the requested reduction exceeds the available resolutions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ReducePolicy {
    /// Return a limit error.
    #[default]
    Exact,
    /// Use the coarsest resolution present in every component.
    Clamp,
}

/// Quality layer, resolution, component and position traversal order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Progression {
    Lrcp,
    Rlcp,
    Rpcl,
    Pcrl,
    Cprl,
}

impl Progression {
    fn from_marker(order: u8) -> Self {
        [Self::Lrcp, Self::Rlcp, Self::Rpcl, Self::Pcrl, Self::Cprl][usize::from(order)]
    }
}

/// Colour space reported by the codestream container.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum ColorSpace {
    Unknown = -1,
    Unspecified = 0,
    Srgb = 1,
    Gray = 2,
    Sycc = 3,
    Eycc = 4,
    Cmyk = 5,
}

impl ColorSpace {
    fn from_jp2(header: &Jp2Header) -> Self {
        match header.color_space() {
            1 => Self::Srgb,
            2 => Self::Gray,
            3 => Self::Sycc,
            4 => Self::Eycc,
            5 => Self::Cmyk,
            _ => Self::Unknown,
        }
    }
}

/// Colour metadata carried by a JP2 or JPH file.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ColorProfile {
    /// Embedded ICC profile bytes.
    Icc(Vec<u8>),
    /// The nine CIELab parameters in OpenJPEG's layout.
    Cielab([u32; 9]),
}

/// Geometry and sample representation of a codestream component.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct ComponentInfo {
    pub x0: u32,
    pub y0: u32,
    pub width: u32,
    pub height: u32,
    pub dx: u32,
    pub dy: u32,
    pub precision: u32,
    pub signed: bool,
    pub reduce: u32,
}

impl ComponentInfo {
    fn set_dimensions(&mut self, dimensions: Dimensions, reduce: u32) {
        self.x0 = dimensions.x0;
        self.y0 = dimensions.y0;
        self.width = dimensions.width;
        self.height = dimensions.height;
        self.reduce = reduce;
    }
}

/// Image bounds and coding metadata from the main header.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Info {
    pub x0: u32,
    pub y0: u32,
    pub x1: u32,
    pub y1: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub tile_x0: u32,
    pub tile_y0: u32,
    pub tiles_across: u32,
    pub tiles_down: u32,
    pub layers: u32,
    pub min_resolutions: u32,
    pub progression: Progression,
    pub components: Vec<ComponentInfo>,
    pub color_space: ColorSpace,
    pub color_profile: Option<ColorProfile>,
    /// Byte offset of the SOC marker from the beginning of the input.
    pub codestream_offset: u64,
    /// Length declared by the JP2 codestream box, when available.
    pub codestream_length: Option<u64>,
    /// Byte offset of the first SOT marker from the beginning of the input.
    pub first_tile_offset: u64,
}

impl Info {
    /// Image width on the reference grid.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.x1 - self.x0
    }

    /// Image height on the reference grid.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.y1 - self.y0
    }
}

/// The coding defaults and component overrides parsed from the main header.
#[derive(Clone, Copy, Debug)]
pub struct CodestreamCoding<'a> {
    main: &'a MainHeader,
}

impl<'a> CodestreamCoding<'a> {
    /// Number of codestream components, before any JP2 palette or channel mapping.
    #[must_use]
    pub const fn component_count(&self) -> u32 {
        self.main.numcomps
    }

    /// Default tile coding style flags from the COD marker.
    #[must_use]
    pub const fn tile_style_flags(&self) -> u8 {
        self.main.cod.csty
    }

    /// Default packet progression order from the COD marker.
    #[must_use]
    pub fn progression(&self) -> Progression {
        Progression::from_marker(self.main.cod.prg)
    }

    /// Default number of quality layers from the COD marker.
    #[must_use]
    pub const fn layers(&self) -> u16 {
        self.main.cod.numlayers
    }

    /// Whether the default tile uses a multiple component transform.
    #[must_use]
    pub const fn uses_mct(&self) -> bool {
        self.main.cod.mct != 0
    }

    /// Coding information for a zero-based codestream component index.
    #[must_use]
    pub fn component(&self, index: u32) -> Option<ComponentCoding<'a>> {
        if index >= self.component_count() {
            return None;
        }
        Some(ComponentCoding {
            style: self.main.main_style(index),
            quantization: self.main.main_quant(index),
            roi_shift: self.main.main_roishift(index),
        })
    }
}

/// Coding information for a component, including its main-header overrides.
#[derive(Clone, Copy, Debug)]
pub struct ComponentCoding<'a> {
    style: &'a CodingStyle,
    quantization: &'a Quantization,
    roi_shift: u8,
}

impl<'a> ComponentCoding<'a> {
    /// Number of resolutions, including the lowest-resolution subband.
    #[must_use]
    pub const fn resolutions(&self) -> u8 {
        self.style.numresolutions
    }

    /// Component coding style flags from COD or COC.
    #[must_use]
    pub const fn style_flags(&self) -> u8 {
        self.style.csty
    }

    /// Code-block width and height as base-2 exponents.
    #[must_use]
    pub const fn code_block_log2(&self) -> (u8, u8) {
        (self.style.cblkw, self.style.cblkh)
    }

    /// Coding mode flags for this component's code-blocks.
    #[must_use]
    pub const fn code_block_style(&self) -> u8 {
        self.style.cblksty
    }

    /// Precinct width and height as base-2 exponents at a zero-based resolution.
    #[must_use]
    pub fn precinct_log2(&self, resolution: u8) -> Option<(u8, u8)> {
        if resolution >= self.resolutions() {
            return None;
        }
        let packed = self.style.precincts[usize::from(resolution)];
        Some((packed & 0x0f, packed >> 4))
    }

    /// Wavelet transform for this component.
    #[must_use]
    pub const fn wavelet(&self) -> Wavelet {
        if self.style.qmfbid == 1 {
            Wavelet::Reversible53
        } else {
            Wavelet::Irreversible97
        }
    }

    /// Region-of-interest shift. Zero is the default.
    #[must_use]
    pub const fn roi_shift(&self) -> u8 {
        self.roi_shift
    }

    /// Quantization style from QCD or QCC.
    #[must_use]
    pub fn quantization_style(&self) -> QuantizationStyle {
        match self.quantization.qntsty {
            0 => QuantizationStyle::None,
            1 => QuantizationStyle::ScalarDerived,
            2 => QuantizationStyle::ScalarExpounded,
            _ => unreachable!("the main header parser validates quantization styles"),
        }
    }

    /// Number of quantization guard bits.
    #[must_use]
    pub const fn guard_bits(&self) -> u8 {
        self.quantization.numgbits
    }

    /// The marker's stored exponent and mantissa pairs in subband order.
    ///
    /// Scalar-derived quantization yields the single base step encoded in the marker.
    pub fn encoded_quantization_steps(self) -> impl ExactSizeIterator<Item = QuantizationStep> + 'a {
        self.quantization.steps[..usize::from(self.quantization.nbands)]
            .iter()
            .map(|&packed| QuantizationStep {
                exponent: packed.to_be_bytes()[0] >> 3,
                mantissa: packed & 0x07ff,
            })
    }
}

/// Wavelet transforms supported by the Part 1 codec.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Wavelet {
    Reversible53,
    Irreversible97,
}

/// How the marker represents a component's quantization steps.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuantizationStyle {
    None,
    ScalarDerived,
    ScalarExpounded,
}

/// An encoded quantization step from a QCD or QCC marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuantizationStep {
    pub exponent: u8,
    pub mantissa: u16,
}

/// Options for a reusable decoder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecoderConfig {
    pub format: Format,
    pub limits: Limits,
    pub reduce: u32,
    pub reduce_policy: ReducePolicy,
    /// Native working buffer capacity to retain after each decode, in bytes.
    /// The cached header, diagnostic messages and a borrowed decoded image have separate lifetimes.
    pub retain_bytes: u64,
    /// Save ordered diagnostics for retrieval after a call. A message may be dropped if memory is unavailable.
    pub capture_diagnostics: bool,
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self {
            format: Format::J2k,
            limits: Limits::default(),
            reduce: 0,
            reduce_policy: ReducePolicy::Exact,
            retain_bytes: 8 << 20,
            capture_diagnostics: false,
        }
    }
}

#[derive(Debug)]
struct HeaderState {
    main: MainHeader,
    jp2: Option<Jp2Header>,
    info: Info,
    prefix: Vec<u8>,
    limits: Limits,
    reduce: u32,
    estimate: Option<(u32, u64)>,
}

impl HeaderState {
    fn needs_resize(&self) -> bool {
        self.reduce > 0 && self.info.components[0].reduce == 0
    }

    fn resize_components(&mut self) -> Result<(), Error> {
        for (info, &component) in self.info.components.iter_mut().zip(&self.main.comps) {
            let dims =
                geometry::resized_dimensions(&self.main, component, self.reduce).ok_or_else(unsupported_coordinates)?;
            info.set_dimensions(dims, self.reduce);
        }
        Ok(())
    }
}

/// Reusable decoder state for one image at a time.
#[derive(Debug)]
pub struct Decoder {
    config: DecoderConfig,
    header: Option<HeaderState>,
    ledger: HeaderLedger,
    diagnostics: Diagnostics,
    work: decode::Workspace,
}

enum DecodeOutput<'targets, 'samples> {
    Planar,
    Bytes(SampleType, &'targets [DecodeTarget<'samples>]),
    I32(TypedOutput<'targets, 'samples>),
}

/// A decoder and reader positioned just after the first tile-part marker.
///
/// Inspect the header or choose a reduce factor before consuming the stream in a decode.
pub struct DecodeStream<'decoder, 'input> {
    decoder: &'decoder mut Decoder,
    source: Source<'input>,
}

impl fmt::Debug for DecodeStream<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DecodeStream")
            .field("info", self.info())
            .finish_non_exhaustive()
    }
}

impl<'decoder> DecodeStream<'decoder, '_> {
    /// Information parsed from the stream's main header.
    #[must_use]
    pub fn info(&self) -> &Info {
        &self
            .decoder
            .header
            .as_ref()
            .expect("the stream has a validated header")
            .info
    }

    /// Sets the requested reduce factor for decoding this stream.
    pub fn set_reduce(&mut self, reduce: u32) -> Result<(), Error> {
        let previous = self.decoder.header.as_ref().expect("the stream has a header").reduce;
        self.decoder.set_reduce(reduce)?;
        let state = self.decoder.header.as_mut().expect("the stream has a header");
        if state.reduce != previous && state.info.components[0].reduce != 0 {
            fill_components(&state.main, state.reduce, &mut state.info.components);
        }
        Ok(())
    }

    /// Validates and returns decoded component dimensions before reading the tile data.
    pub fn decoded_dimensions(&mut self) -> Result<&[ComponentInfo], Error> {
        self.decoder.decoded_dimensions()
    }

    /// Checks decoded dimensions and the working memory needed for the given component indices.
    ///
    /// Returns geometry for every codestream component without reading tile data. Pass the
    /// same indices in [`DecodeParams::components`] to decode the selection.
    pub fn decoded_dimensions_for(&mut self, components: &[u32]) -> Result<&[ComponentInfo], Error> {
        self.decoder.decoded_dimensions_for(components)
    }

    /// Consumes the stream and decodes into the decoder's reusable planar buffers.
    pub fn decode(mut self, params: DecodeParams<'_>) -> Result<DecodedImage<'decoder>, Error> {
        let result = self
            .decoder
            .decode_planes(&mut self.source, params, DecodeOutput::Planar);
        self.decoder.finish_planar_decode(result)
    }

    /// Consumes the stream and decodes into caller buffers. Returns whether the input was truncated.
    pub fn decode_into(
        mut self,
        params: DecodeParams<'_>,
        sample: SampleType,
        targets: &mut [DecodeTarget<'_>],
    ) -> Result<bool, Error> {
        let result = self
            .decoder
            .decode_planes(&mut self.source, params, DecodeOutput::Bytes(sample, targets));
        self.decoder.finish_target_decode(result, Some((sample, targets)))
    }
}

impl Decoder {
    /// Creates a decoder. The header is read on the first request.
    #[must_use]
    pub fn new(config: DecoderConfig) -> Self {
        Self {
            config,
            header: None,
            ledger: HeaderLedger::default(),
            diagnostics: Diagnostics::new(config.capture_diagnostics),
            work: decode::Workspace::default(),
        }
    }

    /// Diagnostic messages from the most recent call, if capture was enabled.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.diagnostics.entries()
    }

    /// Bytes reserved by the header arena according to the C++ decoder's accounting.
    #[must_use]
    pub const fn header_reserved_bytes(&self) -> u64 {
        self.ledger.reserved()
    }

    /// The working memory reservations charged by the most recent decode.
    #[must_use]
    pub const fn statistics(&self) -> DecodeStatistics {
        DecodeStatistics {
            header_reserved: self.ledger.reserved(),
            tile_reserved: self.work.tile_ledger.reserved(),
            work_reserved: self.work.budget.reserved(),
            work_high_water: self.work.budget.high_water(),
        }
    }

    /// Changes the resource limits for the next header read or decode.
    ///
    /// A cached header read under different limits is validated again, even when the input
    /// bytes match. Working buffers remain available for reuse.
    pub fn set_limits(&mut self, limits: Limits) {
        self.config.limits = limits;
    }

    /// Change how much native working storage is kept after the next and subsequent decodes.
    pub fn set_retained_bytes(&mut self, bytes: u64) {
        self.config.retain_bytes = bytes;
    }

    /// Capacity in bytes still allocated to working buffers after the last decode.
    ///
    /// This measures native storage, separately from the C++ arena accounting in [`Self::statistics`].
    /// Cached headers, diagnostic messages and the image returned by [`Self::decode`] are excluded.
    #[must_use]
    pub const fn retained_work_bytes(&self) -> u64 {
        self.work.retained_work_bytes
    }

    /// Decodes a memory codestream into this decoder's reusable planar buffers.
    pub fn decode(&mut self, data: &[u8], params: DecodeParams<'_>) -> Result<DecodedImage<'_>, Error> {
        if let Err(error) = self.read_header(data) {
            self.work.trim_retained(self.config.retain_bytes, false);
            return Err(error);
        }
        let result = self.decode_planes(&mut Source::Memory(data), params, DecodeOutput::Planar);
        self.finish_planar_decode(result)
    }

    /// Reads a J2K, JP2 or JPH stream into the decoder's reusable planar output.
    ///
    /// Reads and validates the header before buffering the codestream for decode. It reads at
    /// most the declared stream length and stops at the end of a JP2/JPH codestream box when the
    /// box gives its length. The stream buffer is outside the codec working-memory budget.
    /// Supply the length of the available bytes for whole codestreams and the full advertised
    /// length when decoding a stream that may end early.
    pub fn decode_from(
        &mut self,
        reader: &mut dyn Read,
        params: DecodeParams<'_>,
        options: StreamOptions,
    ) -> Result<DecodedImage<'_>, Error> {
        self.read_header_from(reader, options)?.decode(params)
    }

    /// Reads and validates a stream header through the first tile-part marker.
    ///
    /// The returned session borrows the reader at that position and can resume the decode without
    /// reading the header again. JP2 and JPH streams require an advertised input length.
    pub fn read_header_from<'decoder, 'input>(
        &'decoder mut self,
        reader: &'input mut dyn Read,
        options: StreamOptions,
    ) -> Result<DecodeStream<'decoder, 'input>, Error> {
        self.diagnostics.clear();
        let mut source = Source::reader(reader, options);
        let header = match self.load_header(&mut source) {
            Ok(header) => header,
            Err(error) => {
                self.header = None;
                self.work.trim_retained(self.config.retain_bytes, false);
                self.diagnostics.error(error.message());
                return Err(error);
            }
        };
        self.header = Some(header);
        Ok(DecodeStream { decoder: self, source })
    }

    fn finish_planar_decode(&mut self, result: Result<(), Error>) -> Result<DecodedImage<'_>, Error> {
        self.work.trim_retained(self.config.retain_bytes, result.is_ok());
        if let Err(error) = result {
            self.diagnostics.error(error.message());
            return Err(error);
        }
        Ok(self.work.image())
    }

    /// Decodes into borrowed, strided buffers. Returns whether the codestream was truncated.
    pub fn decode_into(
        &mut self,
        data: &[u8],
        params: DecodeParams<'_>,
        sample: SampleType,
        targets: &mut [DecodeTarget<'_>],
    ) -> Result<bool, Error> {
        if let Err(error) = self.read_header(data) {
            self.work.trim_retained(self.config.retain_bytes, false);
            return Err(error);
        }
        let result = self.decode_planes(&mut Source::Memory(data), params, DecodeOutput::Bytes(sample, targets));
        self.finish_target_decode(result, Some((sample, targets)))
    }

    /// Decodes into tight, top-down I32 planes. Returns whether the codestream was truncated.
    ///
    /// For a single tile matching the image, the decoder can use one plane per selected component
    /// directly as its coefficient storage. Output can be partially modified if decoding then fails.
    pub fn decode_into_i32(
        &mut self,
        data: &[u8],
        params: DecodeParams<'_>,
        targets: &mut [I32Plane<'_>],
    ) -> Result<bool, Error> {
        if let Err(error) = self.read_header(data) {
            self.work.trim_retained(self.config.retain_bytes, false);
            return Err(error);
        }
        let output = DecodeOutput::I32(TypedOutput::new(targets));
        let result = self.decode_planes(&mut Source::Memory(data), params, output);
        self.finish_target_decode(result, None)
    }

    fn finish_target_decode(
        &mut self,
        result: Result<(), Error>,
        output: Option<(SampleType, &mut [DecodeTarget<'_>])>,
    ) -> Result<bool, Error> {
        if let Err(error) = result {
            self.work.trim_retained(self.config.retain_bytes, false);
            self.diagnostics.error(error.message());
            return Err(error);
        }
        if let Some((sample, targets)) = output {
            self.work.write_targets(sample, targets);
        }
        self.work.trim_retained(self.config.retain_bytes, false);
        Ok(self.work.truncated)
    }

    fn decode_planes(
        &mut self,
        source: &mut Source<'_>,
        params: DecodeParams<'_>,
        output: DecodeOutput<'_, '_>,
    ) -> Result<(), Error> {
        let state = self.header.as_mut().expect("decode reads the header first");
        if matches!(source, Source::Reader(_)) && state.needs_resize() {
            state.resize_components()?;
        }
        let main = &state.main;
        let params = DecodeParams {
            components: params.components.filter(|indices| !indices.is_empty()),
            ..params
        };
        self.work.reset_selection(main.comps.len(), params.components)?;
        match &output {
            DecodeOutput::Planar => {}
            DecodeOutput::Bytes(sample, targets) => {
                decode::validate_targets(targets, *sample, &state.info.components, self.work.slots())?;
            }
            DecodeOutput::I32(typed) => typed.validate(&state.info.components, self.work.slots())?,
        }
        let channels = params.components.is_none() && state.jp2.as_ref().is_some_and(Jp2Header::changes_channels);
        if channels
            && (params.jp2_channels == Jp2Channels::Reject
                || !matches!(output, DecodeOutput::Planar) && params.jp2_channels == Jp2Channels::Apply)
        {
            return Err(Error::new(
                ErrorKind::Unsupported,
                "nj2k_decode_memory: the JP2 file's palette or channel definitions change the components; nj2k_decode() applies them\n",
            ));
        }
        let limits = self.config.limits;
        let side = limits.max_decoded_side;
        if side != 0 {
            for (index, info) in state.info.components.iter().enumerate() {
                let ComponentInfo { width, height, .. } = *info;
                if width > side || height > side {
                    let message = format!(
                        "Component {index} decodes to {width} x {height} samples, more than the decoder's limit of {side} per side\n"
                    );
                    return Err(Error::new(ErrorKind::Limit, message));
                }
            }
        }
        if limits.max_memory != 0 {
            let bytes = if params.components.is_some() {
                geometry::working_memory_selected(main, state.reduce, Some(self.work.slots()))
            } else {
                estimate(state)
            };
            if bytes > limits.max_memory {
                return Err(memory_limit(bytes, limits.max_memory));
            }
        }

        let codestream_end = state
            .info
            .codestream_length
            .and_then(|length| state.main.start.checked_add(length));
        let stream = source.bounds(codestream_end);
        let end = codestream_end.and_then(|end| usize::try_from(end).ok());
        source.ensure(end.unwrap_or(usize::MAX))?;
        let data = source.bytes();
        let end = end.map_or(data.len(), |end| end.min(data.len()));
        let input = decode::Input {
            data: &data[..end],
            stream,
            header: &state.main,
            components: &state.info.components,
            reduce: state.reduce,
            limits,
            retain: self.config.retain_bytes,
        };
        let mut typed = match output {
            DecodeOutput::I32(typed) => Some(typed),
            _ => None,
        };
        self.work.decode(input, params, typed.as_mut(), &mut self.diagnostics)?;
        if let Some(output) = &mut typed {
            self.work.write_i32_targets(output);
        }
        if let Some(jp2) = state
            .jp2
            .as_ref()
            .filter(|_| channels && params.jp2_channels == Jp2Channels::Apply)
        {
            jp2::apply_colour(jp2, data, self.work.image_mut(), limits, &mut self.diagnostics)?;
        }
        Ok(())
    }

    /// Reads the input through the first SOT marker.
    ///
    /// A later call with the same header prefix and limits reuses the validated header.
    pub fn read_header(&mut self, data: &[u8]) -> Result<&Info, Error> {
        self.diagnostics.clear();
        let cached = self.header.as_mut().filter(|state| {
            state.limits == self.config.limits && !state.prefix.is_empty() && data.starts_with(&state.prefix)
        });

        let result = if let Some(state) = cached {
            let factor = choose_factor(&state.main, self.config, true, &mut self.diagnostics);
            factor.map(|reduce| {
                state.reduce = reduce;
                fill_components(&state.main, reduce, &mut state.info.components);
            })
        } else {
            self.load_header(&mut Source::Memory(data))
                .map(|state| self.header = Some(state))
        };

        if let Err(error) = result {
            self.header = None;
            self.diagnostics.error(error.message());
            return Err(error);
        }
        Ok(&self
            .header
            .as_ref()
            .expect("a successful header read sets the state")
            .info)
    }

    fn load_header(&mut self, source: &mut Source<'_>) -> Result<HeaderState, Error> {
        let retain_buffers = self.ledger.reserved() <= self.config.retain_bytes;
        self.ledger
            .reset(self.config.retain_bytes, self.config.limits.max_memory);
        let jp2 = match self.config.format {
            Format::J2k => None,
            Format::Jp2 => Some(jp2::read_boxes(source, &mut self.ledger, &mut self.diagnostics)?),
        };
        let settings = HeaderSettings {
            limits: self.config.limits,
            ihdr_width: jp2.as_ref().map_or(0, |container| container.width),
            ihdr_height: jp2.as_ref().map_or(0, |container| container.height),
            allow_varying_depth: jp2.as_ref().is_none_or(|container| container.bpc == 255),
        };
        let start = jp2.as_ref().map_or(0, |container| container.codestream_start);
        let main = parse_main_header(source, start, settings, &mut self.ledger, &mut self.diagnostics)?;
        let data = source.bytes();
        let reduce = choose_factor(&main, self.config, true, &mut self.diagnostics)?;
        let count = main.comps.len();
        self.ledger
            .array(CType::ImageComp, count)
            .map_err(|error| Error::static_message(error.kind(), "Not enough memory for the image header\n"))?;

        let (mut components, mut prefix) = self
            .header
            .take()
            .filter(|_| retain_buffers)
            .map_or_else(|| (Vec::new(), Vec::new()), |old| (old.info.components, old.prefix));
        components.clear();
        components
            .try_reserve_exact(count)
            .map_err(|_| Error::static_message(ErrorKind::NoMemory, "Not enough memory for the image header\n"))?;
        fill_components(&main, reduce, &mut components);

        prefix.clear();
        let end = usize::try_from(main.end)
            .ok()
            .and_then(|offset| offset.checked_add(2))
            .expect("the main header consumed its SOT marker");
        if matches!(source, Source::Memory(_))
            && let Some(bytes) = data.get(..end)
            && self.ledger.charge(bytes.len(), 16).is_ok()
            && prefix.try_reserve_exact(bytes.len()).is_ok()
        {
            prefix.extend_from_slice(bytes);
        }

        let color_profile = color_profile(jp2.as_ref(), data)?;
        let info = Info {
            x0: main.x0,
            y0: main.y0,
            x1: main.x1,
            y1: main.y1,
            tile_width: main.tdx,
            tile_height: main.tdy,
            tile_x0: main.tx0,
            tile_y0: main.ty0,
            tiles_across: main.tw,
            tiles_down: main.th,
            layers: u32::from(main.cod.numlayers),
            min_resolutions: main.min_resolutions,
            progression: Progression::from_marker(main.cod.prg),
            components,
            color_space: jp2.as_ref().map_or(ColorSpace::Unspecified, ColorSpace::from_jp2),
            color_profile,
            codestream_offset: source.input_offset(main.start),
            codestream_length: jp2.as_ref().and_then(|container| container.codestream_len),
            first_tile_offset: source.input_offset(main.end),
        };
        Ok(HeaderState {
            main,
            jp2,
            info,
            prefix,
            limits: self.config.limits,
            reduce,
            estimate: None,
        })
    }

    /// Changes the requested reduce factor. A header is required to validate the request.
    ///
    /// When called before a header read, the requested factor is kept and this returns
    /// [`ErrorKind::State`]. The next header read validates it.
    pub fn set_reduce(&mut self, reduce: u32) -> Result<(), Error> {
        self.diagnostics.clear();
        let Some(state) = self.header.as_mut() else {
            self.config.reduce = reduce;
            return Err(Error::new(ErrorKind::State, ""));
        };
        let config = DecoderConfig { reduce, ..self.config };
        match choose_factor(&state.main, config, false, &mut self.diagnostics) {
            Ok(effective) => {
                self.config = config;
                state.reduce = effective;
                Ok(())
            }
            Err(error) => {
                self.diagnostics.error(error.message());
                Err(error)
            }
        }
    }

    /// Validates the decoded dimensions and the estimated working memory against the limits.
    ///
    /// A reduce factor set after reading at full resolution uses the decode-time geometry.
    pub fn decoded_dimensions(&mut self) -> Result<&[ComponentInfo], Error> {
        self.decoded_dimensions_for(&[])
    }

    /// Validates dimensions and the estimated working memory for a component selection.
    ///
    /// An empty selection checks the memory for all components. The decoded-side limit and the
    /// returned geometry cover every component in codestream order. The selection applies only
    /// to this check. Pass it in [`DecodeParams::components`] when decoding.
    pub fn decoded_dimensions_for(&mut self, components: &[u32]) -> Result<&[ComponentInfo], Error> {
        self.diagnostics.clear();
        let result = self
            .header
            .as_mut()
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::State,
                    "nj2k_get_decoded_dimensions: call it after nj2k_read_header\n",
                )
            })
            .and_then(|state| {
                let slots = if components.is_empty() {
                    None
                } else {
                    self.work.reset_selection(state.main.comps.len(), Some(components))?;
                    Some(self.work.slots())
                };
                validate_dimensions(state, self.config.limits, slots)
            });
        if let Err(error) = &result {
            self.diagnostics.error(error.message());
        }
        result
    }

    /// Borrows coding information after a successful header read or decode.
    ///
    /// Returns a state error when the decoder has no header. The view uses the main header's
    /// defaults and component overrides. Tile-part overrides belong to the individual tiles.
    pub fn codestream_coding(&self) -> Result<CodestreamCoding<'_>, Error> {
        self.header
            .as_ref()
            .map(|state| CodestreamCoding { main: &state.main })
            .ok_or_else(|| Error::new(ErrorKind::State, "codestream_coding: call it after read_header\n"))
    }
}

fn choose_factor(
    main: &MainHeader,
    config: DecoderConfig,
    at_header: bool,
    diagnostics: &mut Diagnostics,
) -> Result<u32, Error> {
    let reduce = config.reduce;
    if reduce < main.min_resolutions {
        return Ok(reduce);
    }
    if config.reduce_policy == ReducePolicy::Clamp {
        let factor = main.min_resolutions - 1;
        diagnostics.warn_fmt(format_args!(
            "The reduce factor {reduce} is lowered to {factor}, the coarsest level this codestream has\n"
        ));
        return Ok(factor);
    }
    let message = if at_header {
        let fewest = (0..main.numcomps)
            .find(|&index| u32::from(main.main_style(index).numresolutions) == main.min_resolutions)
            .unwrap_or(0);
        let min = main.min_resolutions;
        format!(
            "Error decoding component {fewest}.\nThe number of resolutions to remove ({reduce}) is greater or equal \
             than the number of resolutions of this component ({min})\nModify the cp_reduce parameter.\n\n"
        )
    } else {
        "Resolution factor is greater than the maximum resolution in the component.\n".to_owned()
    };
    Err(Error::new(ErrorKind::Limit, message))
}

fn fill_components(main: &MainHeader, reduce: u32, output: &mut Vec<ComponentInfo>) {
    output.clear();
    output.extend(main.comps.iter().map(|&component| {
        let dims = geometry::header_dimensions(main, component, reduce);
        ComponentInfo {
            x0: dims.x0,
            y0: dims.y0,
            width: dims.width,
            height: dims.height,
            dx: u32::from(component.dx),
            dy: u32::from(component.dy),
            precision: u32::from(component.prec),
            signed: component.sgnd != 0,
            reduce,
        }
    }));
}

fn color_profile(jp2: Option<&Jp2Header>, data: &[u8]) -> Result<Option<ColorProfile>, Error> {
    match jp2.and_then(|container| container.colour.as_ref()) {
        Some(ColourSpec::Icc(range)) if !range.is_empty() => {
            let contents = &data[range.clone()];
            let mut profile = Vec::new();
            profile
                .try_reserve_exact(contents.len())
                .map_err(|_| Error::static_message(ErrorKind::NoMemory, "Not enough memory for the ICC profile\n"))?;
            profile.extend_from_slice(contents);
            Ok(Some(ColorProfile::Icc(profile)))
        }
        Some(ColourSpec::Enumerated {
            cielab: Some(values), ..
        }) => Ok(Some(ColorProfile::Cielab(*values))),
        _ => Ok(None),
    }
}

fn validate_dimensions<'a>(
    state: &'a mut HeaderState,
    limits: Limits,
    slots: Option<&[Option<usize>]>,
) -> Result<&'a [ComponentInfo], Error> {
    let resize = state.needs_resize();
    let main = &state.main;
    for (index, (&stored, &component)) in state.info.components.iter().zip(&main.comps).enumerate() {
        let (width, height) = if resize {
            let dims =
                geometry::resized_dimensions(main, component, state.reduce).ok_or_else(unsupported_coordinates)?;
            (dims.width, dims.height)
        } else {
            (stored.width, stored.height)
        };
        let side = limits.max_decoded_side;
        if side != 0 && (width > side || height > side) {
            return Err(Error::new(
                ErrorKind::Limit,
                format!(
                    "Component {index} is {width} x {height} at the decoded resolution, above the decoder's limit of {side}\n"
                ),
            ));
        }
    }

    let max = limits.max_memory;
    if max != 0 {
        let need = match slots {
            Some(slots) => geometry::working_memory_selected(main, state.reduce, Some(slots)),
            None => estimate(state),
        };
        if need > max {
            return Err(memory_limit(need, max));
        }
    }

    if resize {
        state.resize_components()?;
    }
    Ok(&state.info.components)
}

fn unsupported_coordinates() -> Error {
    Error::new(
        ErrorKind::Unsupported,
        "Image coordinates above INT_MAX are not supported\n",
    )
}

fn estimate(state: &mut HeaderState) -> u64 {
    match state.estimate {
        Some((reduce, bytes)) if reduce == state.reduce => bytes,
        _ => {
            let bytes = geometry::working_memory(&state.main, state.reduce);
            state.estimate = Some((state.reduce, bytes));
            bytes
        }
    }
}

fn memory_limit(need: u64, max: u64) -> Error {
    Error::new(
        ErrorKind::Limit,
        format!("The decode needs {need} bytes of working memory, above the decoder's limit of {max}\n"),
    )
}

#[cfg(all(test, not(miri)))]
mod retention_tests {
    use super::*;
    use crate::OutputPlane;

    fn fixture() -> Vec<u8> {
        std::fs::read(neo_j2k_testkit::files::corpus_dir().join("packed_ppt_tiles.j2k")).unwrap()
    }

    #[test]
    fn retention_limits_apply_after_planar_and_caller_owned_decodes() {
        let data = fixture();
        let params = DecodeParams::default();
        let mut decoder = Decoder::new(DecoderConfig::default());
        let image = decoder.decode(&data, params).unwrap();
        let first = &image.components[0];
        let expected = first.samples().to_vec();
        let width = first.info().width as usize;
        let warm = decoder.retained_work_bytes();
        assert!(warm > 0);

        decoder.set_retained_bytes(0);
        assert_eq!(decoder.retained_work_bytes(), warm);
        let image = decoder.decode(&data, params).unwrap();
        assert_eq!(image.components[0].samples(), expected);
        assert_eq!(decoder.retained_work_bytes(), 0);
        assert!(decoder.header_reserved_bytes() > 0);

        decoder.set_retained_bytes(1024);
        let mut bytes = vec![0xff; expected.len() * size_of::<i32>()];
        let planes = [OutputPlane {
            component: 0,
            offset: 0,
            row_stride: isize::try_from(width * 4).unwrap(),
            sample_stride: 4,
        }];
        let mut targets = [DecodeTarget {
            bytes: &mut bytes,
            planes: &planes,
        }];
        assert!(
            !decoder
                .decode_into(&data, params, SampleType::I32, &mut targets)
                .unwrap()
        );
        let (samples, rest) = bytes.as_chunks::<4>();
        assert!(rest.is_empty());
        assert!(samples.iter().copied().map(i32::from_ne_bytes).eq(expected));
        assert!(decoder.retained_work_bytes() <= 1024);
    }

    #[test]
    fn failed_requests_trim_previous_storage_and_allow_retries() {
        let data = fixture();
        let params = DecodeParams::default();
        let mut decoder = Decoder::new(DecoderConfig::default());
        decoder.decode(&data, params).unwrap();
        assert!(decoder.retained_work_bytes() > 0);
        decoder.set_retained_bytes(0);

        let error = decoder.decode(&[], params).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::NeedData);
        assert_eq!(decoder.retained_work_bytes(), 0);
        assert!(!decoder.decode(&data, params).unwrap().truncated);
        assert_eq!(decoder.retained_work_bytes(), 0);
    }
}
