// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The harness's requests and outcomes (`tests/common/decoder.h`): one request decodes a whole file
//! or a prefix of it in one of three modes, and its outcome holds the image's geometry, its sample
//! planes and the decoder's messages. The identity hashes are computed from an outcome by
//! [`crate::digest`].

use std::fmt;

/// How a request drives the decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    /// One codec, non-strict (partial codestreams decode), the reduce factor and layer limit.
    Plain,
    /// The same in strict mode (OpenJPEG's default).
    Strict,
    /// The viewer's request (`llimagej2coj.cpp`): a header pass, then a second codec with the
    /// reduce factor set before the header, non-strict, the viewer's size guard, its geometry
    /// checks and its 8-bit bottom-up copy of up to four components.
    Viewer,
}

impl Mode {
    /// Every mode, in the C++'s order.
    pub const ALL: [Mode; 3] = [Mode::Plain, Mode::Strict, Mode::Viewer];

    /// The name the reference tables and `nj2k_diff` use.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Mode::Plain => "plain",
            Mode::Strict => "strict",
            Mode::Viewer => "viewer",
        }
    }

    /// The mode of a name; `None` for any other text.
    #[must_use]
    pub fn parse(s: &str) -> Option<Mode> {
        Mode::ALL.into_iter().find(|m| m.name() == s)
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// One request: the bytes the decoder sees (a whole file, or a prefix of it) and how to decode
/// them. The C++'s `capture` and `inspect` are always on here: an outcome keeps its planes and the
/// digests are computed from them.
#[derive(Clone, Copy, Debug)]
pub struct Request<'a> {
    /// The bytes the decoder sees.
    pub data: &'a [u8],
    /// The discard level.
    pub reduce: i32,
    /// Layers to decode, 0 for all (`cp_layer`); plain and strict only.
    pub layers: i32,
    /// How the decoder is driven.
    pub mode: Mode,
    /// A JP2 or JPH file, read by the JP2 codec.
    pub jp2: bool,
    /// Plain and strict: the reduce factor set after the header, not in the parameters.
    pub reduce_after_header: bool,
    /// The Second Life texture limits on every codec of the request, with `max_memory` in place
    /// of the profile's own when it is not 0.
    pub limits: bool,
    /// See `limits`.
    pub max_memory: u64,
}

impl<'a> Request<'a> {
    /// A request for the bytes at a reduce level, in a mode, every other field at its default.
    #[must_use]
    pub const fn new(data: &'a [u8], reduce: i32, mode: Mode) -> Self {
        Self {
            data,
            reduce,
            layers: 0,
            mode,
            jp2: false,
            reduce_after_header: false,
            limits: false,
            max_memory: 0,
        }
    }
}

/// One component's geometry, as the decoder reported it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Component {
    pub w: u32,
    pub h: u32,
    pub dx: u32,
    pub dy: u32,
    pub x0: u32,
    pub y0: u32,
    pub prec: u32,
    pub sgnd: u32,
    pub factor: u32,
    pub resno_decoded: u32,
}

/// What a request gave. Texts are bytes: the decoders' messages are C text of any encoding.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Outcome {
    /// The image decoded and passed the mode's checks.
    pub ok: bool,
    /// Where a failure happened: `codec`, `header`, `guard`, `decode` or `geometry`.
    pub stage: Vec<u8>,
    /// The image area.
    pub x0: u32,
    #[allow(missing_docs)]
    pub y0: u32,
    #[allow(missing_docs)]
    pub x1: u32,
    #[allow(missing_docs)]
    pub y1: u32,
    /// The components of the image.
    pub numcomps: u32,
    /// The image's colour space, as the decoder's enum value.
    pub color_space: i32,
    /// Every component's geometry.
    pub comps: Vec<Component>,
    /// Every component's samples, `w x h` in rows.
    pub planes: Vec<Vec<i32>>,
    /// Warnings and errors the decoder gave.
    pub warnings: u32,
    #[allow(missing_docs)]
    pub errors: u32,
    /// Every error message, oldest first, joined by ` | ` (at most about 400 bytes).
    pub last_error: Vec<u8>,
    /// Each kind of warning once, in the order they came.
    pub warning_kinds: Vec<u8>,
}

/// A decoder under test, driven by requests: the C++ oracle's backends now, the twin later.
pub trait Backend {
    /// The backend's name, as `nj2k_diff` gives it (`oracle`, `neoj2k`, ...).
    fn name(&self) -> &str;
    /// One request.
    fn decode(&mut self, request: &Request<'_>) -> Outcome;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_names_round_trip() {
        for m in Mode::ALL {
            assert_eq!(Mode::parse(m.name()), Some(m));
        }
        assert_eq!(Mode::parse("Plain"), None);
        assert_eq!(Mode::Viewer.to_string(), "viewer");
    }
}
