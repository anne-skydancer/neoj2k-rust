// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Stage timers for the decoder, a measurement aid and nothing else.
//!
//! `timed!(Stage::X, expression)` evaluates the expression. Without the off-by-default feature `bench-internals`
//! the macro expands to the expression alone: no timer, no state, no code. With the feature it also adds the
//! wall-clock time the expression took to the calling thread's total for that stage, so a benchmark can read where
//! one decode's time went (`crate::bench::stage_reset`, `crate::bench::stage_totals`). Timers are per thread, so only
//! work on the calling thread is counted: a decode that hands work to an executor counts the calling thread's share.
//! The stages never nest, so the totals add up to less than the call; the rest is the code between them.

/// The stages of one decode call, in the order the code runs them.
#[cfg(feature = "bench-internals")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum Stage {
    /// `read_header`: the main header parse, or the reuse of the cached one.
    Header,
    /// `decode_planes` up to the tile loop: selection, target and limit checks.
    Request,
    /// `Work::decode` before the first tile: budget reset, image planes, tile scan setup.
    Begin,
    /// `Walk::next_tile`: find the next tile's SOT and tile-part data.
    TileScan,
    /// Tile setup: coding parameters, resolutions, bands, precincts, code-blocks (`Tile::configure`) and the
    /// tile's packed-header and data gathering.
    TileSetup,
    /// The Tier-2 packet walk (`t2::read_packets`): packet headers and code-block segment lists.
    Packets,
    /// Zeroing or sizing of each component's coefficient plane before Tier-1.
    PlanePrep,
    /// Tier-1: the code-block decoder.
    Tier1,
    /// The inverse wavelet transform.
    Wavelet,
    /// The inverse colour transform and the fused colour finish.
    Colour,
    /// Per component: DC shift, rounding and clamp, and the copy into the output plane.
    Finish,
    /// After the tile: the copy of the planes into the caller's buffers (`write_targets`, `write_i32_targets`).
    Output,
    /// The trim of retained storage after the call.
    Trim,
}

#[cfg(feature = "bench-internals")]
impl Stage {
    /// Every stage, in order.
    pub const ALL: [Stage; 13] = [
        Stage::Header,
        Stage::Request,
        Stage::Begin,
        Stage::TileScan,
        Stage::TileSetup,
        Stage::Packets,
        Stage::PlanePrep,
        Stage::Tier1,
        Stage::Wavelet,
        Stage::Colour,
        Stage::Finish,
        Stage::Output,
        Stage::Trim,
    ];

    /// A short name for tables.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Stage::Header => "header",
            Stage::Request => "request checks",
            Stage::Begin => "begin (budget, planes)",
            Stage::TileScan => "tile scan",
            Stage::TileSetup => "tile setup",
            Stage::Packets => "packet walk (Tier-2)",
            Stage::PlanePrep => "plane prep (zero)",
            Stage::Tier1 => "Tier-1",
            Stage::Wavelet => "wavelet",
            Stage::Colour => "colour transform",
            Stage::Finish => "finish (per component)",
            Stage::Output => "output copy",
            Stage::Trim => "trim",
        }
    }
}

#[cfg(feature = "bench-internals")]
mod clock {
    use super::Stage;
    use std::cell::Cell;
    use std::time::{Duration, Instant};

    thread_local! {
        static TOTALS: [Cell<Duration>; Stage::ALL.len()] = const { [const { Cell::new(Duration::ZERO) }; Stage::ALL.len()] };
    }

    pub(crate) fn add(stage: Stage, start: Instant) {
        let spent = start.elapsed();
        let _ = TOTALS.try_with(|totals| {
            let cell = &totals[stage as usize];
            cell.set(cell.get() + spent);
        });
    }

    pub(crate) fn reset() {
        let _ = TOTALS.try_with(|totals| totals.iter().for_each(|cell| cell.set(Duration::ZERO)));
    }

    pub(crate) fn totals() -> [Duration; Stage::ALL.len()] {
        TOTALS
            .try_with(|totals| std::array::from_fn(|index| totals[index].get()))
            .unwrap_or([Duration::ZERO; Stage::ALL.len()])
    }
}

#[cfg(feature = "bench-internals")]
pub(crate) use clock::{reset, totals};

/// Evaluates the expression and, with `bench-internals`, charges its time to the stage.
#[cfg(feature = "bench-internals")]
macro_rules! timed {
    ($stage:expr, $body:expr) => {{
        let started = ::std::time::Instant::now();
        let value = $body;
        $crate::stage::clock_add($stage, started);
        value
    }};
}

/// Evaluates the expression: without `bench-internals` there is no timer.
#[cfg(not(feature = "bench-internals"))]
macro_rules! timed {
    ($stage:expr, $body:expr) => {
        $body
    };
}

#[cfg(feature = "bench-internals")]
pub(crate) use clock::add as clock_add;

/// Starts a timer for a region with early exits, closed by `stage_stop!`.
#[cfg(feature = "bench-internals")]
macro_rules! stage_start {
    () => {
        ::std::time::Instant::now()
    };
}

/// Without `bench-internals` there is no timer.
#[cfg(not(feature = "bench-internals"))]
macro_rules! stage_start {
    () => {
        ()
    };
}

/// Charges the time since `stage_start!` to the stage.
#[cfg(feature = "bench-internals")]
macro_rules! stage_stop {
    ($stage:expr, $started:expr) => {
        $crate::stage::clock_add($stage, $started)
    };
}

/// Without `bench-internals` there is no timer.
#[cfg(not(feature = "bench-internals"))]
macro_rules! stage_stop {
    ($stage:expr, $started:expr) => {
        let () = $started;
    };
}
