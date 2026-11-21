// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use core::fmt;

/// Why a codec operation failed. The discriminants match neo-j2k's C++ status codes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The input ended before the operation could complete.
    NeedData = -1,
    /// An argument was invalid.
    Argument = -2,
    /// The input does not have the expected JPEG 2000 signature.
    NotJpeg2000 = -3,
    /// A header was invalid.
    Header = -4,
    /// The input uses a feature the codec does not support.
    Unsupported = -5,
    /// A resource limit or a reduce factor refused the input.
    Limit = -6,
    /// An allocation failed.
    NoMemory = -7,
    /// The packet or code-block data was damaged.
    Data = -8,
    /// The output cannot hold the decoded image.
    Output = -9,
    /// The call requires an earlier successful codec operation.
    State = -10,
}

/// A codec failure with its status and diagnostic message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    /// The category of this failure.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// The diagnostic text associated with this failure.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message.trim_end())
    }
}

impl std::error::Error for Error {}

/// A diagnostic emitted while processing a codestream.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Diagnostic {
    Info(String),
    Warning(String),
    Error(String),
}

#[derive(Debug)]
pub(crate) struct Diagnostics {
    enabled: bool,
    entries: Vec<Diagnostic>,
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self::new(true)
    }
}

impl Diagnostics {
    pub(crate) const fn new(enabled: bool) -> Self {
        Self {
            enabled,
            entries: Vec::new(),
        }
    }

    pub(crate) fn entries(&self) -> &[Diagnostic] {
        &self.entries
    }

    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }

    pub(crate) fn info(&mut self, message: impl Into<String>) {
        if self.enabled {
            self.entries.push(Diagnostic::Info(message.into()));
        }
    }

    pub(crate) fn warn(&mut self, message: impl Into<String>) {
        if self.enabled {
            self.entries.push(Diagnostic::Warning(message.into()));
        }
    }

    pub(crate) fn warn_fmt(&mut self, message: fmt::Arguments<'_>) {
        if self.enabled {
            self.entries.push(Diagnostic::Warning(message.to_string()));
        }
    }

    pub(crate) fn error(&mut self, message: impl Into<String>) {
        if self.enabled {
            self.entries.push(Diagnostic::Error(message.into()));
        }
    }
}
