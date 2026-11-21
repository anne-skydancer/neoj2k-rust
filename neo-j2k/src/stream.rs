// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#[cfg(feature = "encode")]
use core::num::NonZeroUsize;
#[cfg(feature = "encode")]
use std::io::Write;
use std::io::{ErrorKind as IoErrorKind, Read};

use crate::{Error, ErrorKind};

/// Input properties used by the decoder's buffered streaming API.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StreamOptions {
    /// Length in bytes from the reader's current position. Required to decode a stream or read a
    /// JP2/JPH header.
    pub length: Option<u64>,
    /// Enable codec look-ahead that requires seeking in the C++ stream API.
    /// The Rust decoder performs look-ahead on buffered bytes without moving the reader.
    pub seekable: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct StreamBounds {
    pub(crate) claimed_end: Option<u64>,
    pub(crate) has_seek: bool,
}

pub(crate) enum Source<'a> {
    Memory(&'a [u8]),
    Reader(StreamReader<'a>),
}

pub(crate) struct StreamReader<'a> {
    reader: &'a mut dyn Read,
    options: StreamOptions,
    buffered: Vec<u8>,
    skipped: u64,
    eof: bool,
}

impl<'a> Source<'a> {
    pub(crate) fn reader(reader: &'a mut dyn Read, options: StreamOptions) -> Self {
        Self::Reader(StreamReader {
            reader,
            options,
            buffered: Vec::new(),
            skipped: 0,
            eof: false,
        })
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        match self {
            Self::Memory(data) => data,
            Self::Reader(stream) => &stream.buffered,
        }
    }

    pub(crate) fn ensure(&mut self, end: usize) -> Result<(), Error> {
        if let Self::Reader(stream) = self {
            stream.fill_to(end)?;
        }
        Ok(())
    }

    // JP2 parses the header in one pass. Discarding a box at the buffer's end preserves prior
    // payload ranges and leaves the next reader box at the same buffered offset.
    pub(crate) fn skip(&mut self, start: usize, count: usize) -> Result<Option<usize>, Error> {
        let Some(end) = start.checked_add(count) else {
            return Ok(None);
        };
        match self {
            Self::Memory(data) => Ok((end <= data.len()).then_some(end)),
            Self::Reader(stream) => stream.skip(start, end).map(|complete| complete.then_some(start)),
        }
    }

    // All skips precede the codestream. Report its public offsets in the input's coordinates.
    pub(crate) fn input_offset(&self, offset: u64) -> u64 {
        match self {
            Self::Memory(_) => offset,
            Self::Reader(stream) => offset + stream.skipped,
        }
    }

    pub(crate) fn read(&mut self, start: usize, count: usize) -> Result<Option<&[u8]>, Error> {
        let Some(end) = start.checked_add(count) else {
            return Ok(None);
        };
        self.ensure(end)?;
        Ok(self.bytes().get(start..end))
    }

    pub(crate) fn read_segment(
        &mut self,
        start: usize,
        count: usize,
        reserve: impl FnOnce() -> Result<(), Error>,
    ) -> Result<Option<&[u8]>, Error> {
        let Some(end) = start.checked_add(count) else {
            return Ok(None);
        };
        // A stream allocates for a marker or known box before it reads its body if the declared
        // length allows it. With a slice, the supplied bytes establish the length directly.
        match self {
            Self::Memory(data) => {
                let bytes = data.get(start..end);
                if bytes.is_some() {
                    reserve()?;
                }
                Ok(bytes)
            }
            Self::Reader(stream) => {
                if stream.compact_length().is_none_or(|length| end as u64 <= length) {
                    reserve()?;
                }
                stream.fill_to(end)?;
                Ok(stream.buffered.get(start..end))
            }
        }
    }

    pub(crate) fn bounds(&self, codestream_end: Option<u64>) -> Option<StreamBounds> {
        let Self::Reader(stream) = self else {
            return None;
        };
        Some(StreamBounds {
            claimed_end: stream
                .compact_length()
                .map(|length| codestream_end.map_or(length, |end| end.min(length))),
            has_seek: stream.options.seekable,
        })
    }
}

impl StreamReader<'_> {
    fn compact_length(&self) -> Option<u64> {
        self.options.length.map(|length| length - self.skipped)
    }

    fn skip(&mut self, start: usize, end: usize) -> Result<bool, Error> {
        let buffered = self.buffered.len();
        debug_assert!((start..=end).contains(&buffered));
        let unread = end - buffered;
        self.buffered.truncate(start);
        self.skipped += (buffered - start) as u64;

        let mut left = self.compact_length().map_or(unread, |length| {
            usize::try_from(length - start as u64).map_or(unread, |length| length.min(unread))
        });
        let requested = left;
        let mut scratch = [0; 8192];
        while !self.eof && left != 0 {
            let count = left.min(scratch.len());
            let read = read_some(self.reader, &mut scratch[..count])?;
            self.skipped += read as u64;
            left -= read;
            self.eof = read == 0;
        }
        Ok(left == 0 && requested == unread)
    }

    fn fill_to(&mut self, end: usize) -> Result<(), Error> {
        let declared = self.compact_length().and_then(|length| usize::try_from(length).ok());
        let end = declared.map_or(end, |declared| end.min(declared));
        while !self.eof && self.buffered.len() < end {
            let start = self.buffered.len();
            let count = (end - start).min(64 * 1024);
            self.buffered.try_reserve(count).map_err(|_| {
                Error::static_message(ErrorKind::NoMemory, "Not enough memory to buffer the input stream\n")
            })?;
            self.buffered.resize(start + count, 0);
            match read_some(self.reader, &mut self.buffered[start..]) {
                Ok(read) => {
                    self.buffered.truncate(start + read);
                    self.eof = read == 0;
                }
                Err(error) => {
                    self.buffered.truncate(start);
                    return Err(error);
                }
            }
        }
        Ok(())
    }
}

fn read_some(reader: &mut dyn Read, buffer: &mut [u8]) -> Result<usize, Error> {
    loop {
        match reader.read(buffer) {
            Ok(read) if read <= buffer.len() => return Ok(read),
            Err(error) if error.kind() == IoErrorKind::Interrupted => {}
            _ => {
                return Err(Error::static_message(
                    ErrorKind::Data,
                    "The input stream could not be read\n",
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skipping_reader_bytes_keeps_the_buffer_capacity() {
        let prefix = b"keptskip";
        let omitted = 1024 * 1024;
        let padding = std::io::repeat(0).take(omitted);
        let mut reader = prefix.as_slice().chain(padding).chain(b"tail".as_slice());
        let options = StreamOptions {
            length: Some(12 + omitted),
            seekable: false,
        };
        let mut source = Source::reader(&mut reader, options);
        source.ensure(prefix.len()).unwrap();
        let Source::Reader(stream) = &source else {
            unreachable!()
        };
        let capacity = stream.buffered.capacity();

        assert_eq!(source.skip(4, 4 + usize::try_from(omitted).unwrap()).unwrap(), Some(4));
        assert_eq!(source.bytes(), b"kept");
        let Source::Reader(stream) = &source else {
            unreachable!()
        };
        assert_eq!(stream.buffered.capacity(), capacity);
        assert_eq!(source.read(4, 4).unwrap(), Some(b"tail".as_slice()));
        assert_eq!(source.input_offset(4), 8 + omitted);
    }
}

#[cfg(feature = "encode")]
#[derive(Clone, Copy)]
pub(crate) enum EncodePart {
    Header,
    Tile,
    End,
}

#[cfg(feature = "encode")]
impl EncodePart {
    fn write_error(self) -> Error {
        let text = match self {
            Self::Header => "nj2k_start_compress: the output stream refused the main header\n",
            Self::Tile => "nj2k_encode: the output stream refused the tile\n",
            Self::End => "nj2k_end_compress: the output stream refused the end of the codestream\n",
        };
        Error::static_message(ErrorKind::Output, text)
    }
}

#[cfg(feature = "encode")]
pub(crate) struct StreamWriter<'a, W: ?Sized> {
    writer: &'a mut W,
    buffer_size: NonZeroUsize,
    written: usize,
}

#[cfg(feature = "encode")]
impl<'a, W: Write + ?Sized> StreamWriter<'a, W> {
    pub(crate) const fn new(writer: &'a mut W, buffer_size: NonZeroUsize) -> Self {
        Self {
            writer,
            buffer_size,
            written: 0,
        }
    }

    pub(crate) fn write(&mut self, part: EncodePart, encoded: &[u8]) -> Result<(), Error> {
        // The encoder owns the accumulated bytes, so the unflushed tail needs no separate copy.
        // As in the C++ stream, a full buffer is sent when more data arrives or at the end.
        let buffer_size = self.buffer_size.get();
        let final_part = matches!(part, EncodePart::End);
        let end = if final_part {
            encoded.len()
        } else {
            let pending = encoded.len() - self.written;
            self.written + pending.saturating_sub(1) / buffer_size * buffer_size
        };
        for chunk in encoded[self.written..end].chunks(buffer_size) {
            self.writer.write_all(chunk).map_err(|_| part.write_error())?;
        }
        self.written = end;
        if final_part {
            loop {
                match self.writer.flush() {
                    Ok(()) => break,
                    Err(error) if error.kind() == IoErrorKind::Interrupted => {}
                    Err(_) => return Err(part.write_error()),
                }
            }
        }
        Ok(())
    }
}
