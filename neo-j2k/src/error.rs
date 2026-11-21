// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

use core::fmt;
use std::borrow::Cow;

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
    message: Cow<'static, str>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: Cow::Owned(message.into()),
        }
    }

    /// Keep static error text available when the allocator has refused a request.
    pub(crate) const fn static_message(kind: ErrorKind, message: &'static str) -> Self {
        Self {
            kind,
            message: Cow::Borrowed(message),
        }
    }

    pub(crate) fn formatted(kind: ErrorKind, args: fmt::Arguments<'_>, fallback: &'static str) -> Self {
        match try_format(args) {
            Some(message) => Self::new(kind, message),
            None => Self::static_message(kind, fallback),
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

fn try_format(args: fmt::Arguments<'_>) -> Option<String> {
    struct Text(String);

    impl fmt::Write for Text {
        fn write_str(&mut self, part: &str) -> fmt::Result {
            self.0.try_reserve(part.len()).map_err(|_| fmt::Error)?;
            self.0.push_str(part);
            Ok(())
        }
    }

    let mut text = Text(String::new());
    fmt::write(&mut text, args).ok()?;
    Some(text.0)
}

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

    pub(crate) fn configure_like(&mut self, other: &Self) {
        self.enabled = other.enabled;
        self.entries.clear();
    }

    pub(crate) fn append(&mut self, other: &mut Self) {
        if self.entries.try_reserve(other.entries.len()).is_ok() {
            self.entries.append(&mut other.entries);
        } else {
            let available = self.entries.capacity() - self.entries.len();
            for entry in other.entries.drain(..).take(available) {
                self.entries.push(entry);
            }
        }
    }

    pub(crate) fn retained_bytes(&self) -> u64 {
        crate::mem::capacity_bytes(&self.entries)
            + self
                .entries
                .iter()
                .map(|entry| match entry {
                    Diagnostic::Info(text) | Diagnostic::Warning(text) | Diagnostic::Error(text) => {
                        text.capacity() as u64
                    }
                })
                .sum::<u64>()
    }

    pub(crate) fn info(&mut self, message: &str) {
        self.info_fmt(format_args!("{message}"));
    }

    pub(crate) fn info_fmt(&mut self, message: fmt::Arguments<'_>) {
        self.push(Diagnostic::Info, || try_format(message));
    }

    pub(crate) fn warn(&mut self, message: &str) {
        self.warn_fmt(format_args!("{message}"));
    }

    pub(crate) fn warn_fmt(&mut self, message: fmt::Arguments<'_>) {
        self.push(Diagnostic::Warning, || try_format(message));
    }

    pub(crate) fn error<'a>(&mut self, message: impl Into<Cow<'a, str>>) {
        self.push(Diagnostic::Error, || match message.into() {
            Cow::Borrowed(message) => try_format(format_args!("{message}")),
            Cow::Owned(message) => Some(message),
        });
    }

    pub(crate) fn error_fmt(&mut self, message: fmt::Arguments<'_>) {
        self.push(Diagnostic::Error, || try_format(message));
    }

    fn push(&mut self, entry: fn(String) -> Diagnostic, message: impl FnOnce() -> Option<String>) {
        if self.enabled
            && self.entries.try_reserve(1).is_ok()
            && let Some(message) = message()
        {
            self.entries.push(entry(message));
        }
    }
}
