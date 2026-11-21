// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use crate::error::Diagnostics;
use crate::geometry::{self, Dimensions};
use crate::header::{HeaderSettings, MainHeader, parse_main_header};
use crate::jp2::{self, ColourSpec, Jp2Header};
use crate::mem::{CType, HeaderLedger};
use crate::{Diagnostic, Error, ErrorKind, Limits};

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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

/// Options for a reusable decoder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecoderConfig {
    pub format: Format,
    pub limits: Limits,
    pub reduce: u32,
    pub reduce_policy: ReducePolicy,
    /// Header arena reservations above this value are discarded before reading a new header.
    pub retain_bytes: u64,
    /// Save ordered diagnostics for retrieval after a call.
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
    info: Info,
    prefix: Vec<u8>,
    reduce: u32,
    estimate: Option<(u32, u64)>,
}

/// Reusable decoder state for one image at a time.
#[derive(Debug)]
pub struct Decoder {
    config: DecoderConfig,
    header: Option<HeaderState>,
    ledger: HeaderLedger,
    diagnostics: Diagnostics,
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

    /// Reads the input through the first SOT marker.
    ///
    /// A later call starting with the same header prefix reuses the validated header.
    pub fn read_header(&mut self, data: &[u8]) -> Result<&Info, Error> {
        self.diagnostics.clear();
        let cached = self
            .header
            .as_mut()
            .filter(|state| !state.prefix.is_empty() && data.starts_with(&state.prefix));

        let result = if let Some(state) = cached {
            let factor = choose_factor(&state.main, self.config, true, &mut self.diagnostics);
            factor.map(|reduce| {
                state.reduce = reduce;
                fill_components(&state.main, reduce, &mut state.info.components);
            })
        } else {
            self.load_header(data).map(|state| self.header = Some(state))
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

    fn load_header(&mut self, data: &[u8]) -> Result<HeaderState, Error> {
        let retain_buffers = self.ledger.reserved() <= self.config.retain_bytes;
        self.ledger
            .reset(self.config.retain_bytes, self.config.limits.max_memory);
        let jp2 = match self.config.format {
            Format::J2k => None,
            Format::Jp2 => Some(jp2::read_boxes(data, &mut self.ledger, &mut self.diagnostics)?),
        };
        let settings = HeaderSettings {
            limits: self.config.limits,
            ihdr_width: jp2.as_ref().map_or(0, |container| container.width),
            ihdr_height: jp2.as_ref().map_or(0, |container| container.height),
            allow_varying_depth: jp2.as_ref().is_none_or(|container| container.bpc == 255),
        };
        let start = jp2.as_ref().map_or(0, |container| container.codestream_start);
        let main = parse_main_header(data, start, settings, &mut self.ledger, &mut self.diagnostics)?;
        let reduce = choose_factor(&main, self.config, true, &mut self.diagnostics)?;
        let count = main.comps.len();
        self.ledger
            .array(CType::ImageComp, count)
            .map_err(|error| Error::new(error.kind(), "Not enough memory for the image header\n"))?;

        let (mut components, mut prefix) = self
            .header
            .take()
            .filter(|_| retain_buffers)
            .map_or_else(|| (Vec::new(), Vec::new()), |old| (old.info.components, old.prefix));
        components.clear();
        components
            .try_reserve_exact(count)
            .map_err(|_| Error::new(ErrorKind::NoMemory, "Not enough memory for the image header\n"))?;
        fill_components(&main, reduce, &mut components);

        prefix.clear();
        let end = usize::try_from(main.end)
            .ok()
            .and_then(|offset| offset.checked_add(2))
            .expect("the main header consumed its SOT marker");
        if let Some(bytes) = data.get(..end)
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
            codestream_offset: main.start,
            codestream_length: jp2.and_then(|container| container.codestream_len),
            first_tile_offset: main.end,
        };
        Ok(HeaderState {
            main,
            info,
            prefix,
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
            .and_then(|state| validate_dimensions(state, self.config.limits));
        if let Err(error) = &result {
            self.diagnostics.error(error.message());
        }
        result
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
                .map_err(|_| Error::new(ErrorKind::NoMemory, "Not enough memory for the ICC profile\n"))?;
            profile.extend_from_slice(contents);
            Ok(Some(ColorProfile::Icc(profile)))
        }
        Some(ColourSpec::Enumerated {
            cielab: Some(values), ..
        }) => Ok(Some(ColorProfile::Cielab(*values))),
        _ => Ok(None),
    }
}

fn validate_dimensions(state: &mut HeaderState, limits: Limits) -> Result<&[ComponentInfo], Error> {
    let resize = state.reduce > 0 && state.info.components[0].reduce == 0;
    let main = &state.main;
    for (index, (&stored, &component)) in state.info.components.iter().zip(&main.comps).enumerate() {
        let (width, height) = if resize {
            let dims = geometry::resized_dimensions(main, component, state.reduce).ok_or_else(|| {
                Error::new(
                    ErrorKind::Unsupported,
                    "Image coordinates above INT_MAX are not supported\n",
                )
            })?;
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
        let need = match state.estimate {
            Some((reduce, bytes)) if reduce == state.reduce => bytes,
            _ => {
                let bytes = geometry::working_memory(main, state.reduce);
                state.estimate = Some((state.reduce, bytes));
                bytes
            }
        };
        if need > max {
            return Err(Error::new(
                ErrorKind::Limit,
                format!("The decode needs {need} bytes of working memory, above the decoder's limit of {max}\n"),
            ));
        }
    }

    if resize {
        for (stored, &component) in state.info.components.iter_mut().zip(&main.comps) {
            let dims = geometry::resized_dimensions(main, component, state.reduce)
                .expect("the dimensions were validated above");
            stored.set_dimensions(dims, state.reduce);
        }
    }
    Ok(&state.info.components)
}
