// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Reference-grid geometry and the decoder's preflight working-memory estimate.

use crate::header::{CodingStyle, MainHeader, SizComponent};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Rect {
    pub(crate) x0: u32,
    pub(crate) y0: u32,
    pub(crate) x1: u32,
    pub(crate) y1: u32,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Dimensions {
    pub(crate) x0: u32,
    pub(crate) y0: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

pub(crate) fn ceil_div_pow2(value: u32, shift: u32) -> u32 {
    match 1_u32.checked_shl(shift) {
        Some(divisor) => value.div_ceil(divisor),
        None => u32::from(value != 0),
    }
}

fn component_rect(rect: Rect, component: SizComponent) -> Rect {
    let dx = u32::from(component.dx);
    let dy = u32::from(component.dy);
    Rect {
        x0: rect.x0.div_ceil(dx),
        y0: rect.y0.div_ceil(dy),
        x1: rect.x1.div_ceil(dx),
        y1: rect.y1.div_ceil(dy),
    }
}

fn image_rect(main: &MainHeader) -> Rect {
    Rect {
        x0: main.x0,
        y0: main.y0,
        x1: main.x1,
        y1: main.y1,
    }
}

pub(crate) fn header_dimensions(main: &MainHeader, component: SizComponent, reduce: u32) -> Dimensions {
    // SIZ validation ensures the tile grid covers the entire image.
    let bounds = component_rect(image_rect(main), component);
    Dimensions {
        x0: bounds.x0,
        y0: bounds.y0,
        width: ceil_div_pow2(bounds.x1 - bounds.x0, reduce),
        height: ceil_div_pow2(bounds.y1 - bounds.y0, reduce),
    }
}

pub(crate) fn resized_dimensions(main: &MainHeader, component: SizComponent, reduce: u32) -> Option<Dimensions> {
    if main.x1 > i32::MAX.unsigned_abs() || main.y1 > i32::MAX.unsigned_abs() {
        return None;
    }
    let bounds = component_rect(image_rect(main), component);
    Some(Dimensions {
        x0: bounds.x0,
        y0: bounds.y0,
        width: ceil_div_pow2(bounds.x1, reduce) - ceil_div_pow2(bounds.x0, reduce),
        height: ceil_div_pow2(bounds.y1, reduce) - ceil_div_pow2(bounds.y0, reduce),
    })
}

pub(crate) fn tile_rect(main: &MainHeader, tile: u32) -> Rect {
    let x = u64::from(main.tx0) + u64::from(tile % main.tw) * u64::from(main.tdx);
    let y = u64::from(main.ty0) + u64::from(tile / main.tw) * u64::from(main.tdy);
    Rect {
        x0: u32::try_from(x.max(u64::from(main.x0))).expect("tile starts inside the grid"),
        y0: u32::try_from(y.max(u64::from(main.y0))).expect("tile starts inside the grid"),
        x1: u32::try_from((x + u64::from(main.tdx)).min(u64::from(main.x1))).expect("tile clipped to the image"),
        y1: u32::try_from((y + u64::from(main.tdy)).min(u64::from(main.y1))).expect("tile clipped to the image"),
    }
}

// Cells of the grid of side 2^exponent, plus one to bound every tile alignment.
fn cells(start: u32, end: u32, exponent: u32) -> u64 {
    if start >= end {
        return 0;
    }
    u64::from(ceil_div_pow2(end - start, exponent)) + 1
}

fn structure_bytes(style: &CodingStyle, reduce: u32, bounds: Rect, most_precincts: &mut u64) -> u64 {
    let levels = u32::from(style.numresolutions) - 1;
    let mut bytes = 0_u64;
    for (resolution, &precinct) in (0..=levels).zip(&style.precincts) {
        let down = levels - resolution;
        let rx0 = ceil_div_pow2(bounds.x0, down);
        let rx1 = ceil_div_pow2(bounds.x1, down);
        let ry0 = ceil_div_pow2(bounds.y0, down);
        let ry1 = ceil_div_pow2(bounds.y1, down);
        let ppx = u32::from(precinct & 15);
        let ppy = u32::from(precinct >> 4);
        let precincts = cells(rx0, rx1, ppx).saturating_mul(cells(ry0, ry1, ppy));
        *most_precincts = (*most_precincts).max(precincts);
        bytes = bytes.saturating_add(precincts.saturating_mul(64));

        let band_shift = u32::from(resolution != 0);
        let cbw = u32::from(style.cblkw).min(ppx.saturating_sub(band_shift));
        let cbh = u32::from(style.cblkh).min(ppy.saturating_sub(band_shift));
        let each = if resolution + reduce <= levels { 128 } else { 32 };
        let blocks = if resolution == 0 {
            cells(rx0, rx1, cbw).saturating_mul(cells(ry0, ry1, cbh))
        } else {
            let band_width = (rx1 - rx0).div_ceil(2) + 1;
            let band_height = (ry1 - ry0).div_ceil(2) + 1;
            cells(0, band_width, cbw)
                .saturating_mul(cells(0, band_height, cbh))
                .saturating_mul(3)
        };
        bytes = bytes.saturating_add(blocks.saturating_mul(each));
    }
    bytes
}

fn image_bytes(main: &MainHeader, reduce: u32) -> u64 {
    let mut bytes = 256_u64 * 1024;
    let mut longest = 1;
    for &component in &main.comps {
        let header = header_dimensions(main, component, reduce);
        let resized = resized_dimensions(main, component, reduce).unwrap_or(header);
        let width = header.width.max(resized.width);
        let height = header.height.max(resized.height);
        bytes = bytes.saturating_add(sample_bytes(width, height));
        longest = longest.max(width).max(height);
    }
    bytes.saturating_add(u64::from(longest) * 64)
}

fn sample_bytes(width: u32, height: u32) -> u64 {
    (u64::from(width) * u64::from(height)).saturating_mul(4)
}

fn tile_bytes(main: &MainHeader, tile: u32, reduce: u32) -> u64 {
    let rect = tile_rect(main, tile);
    let several_tiles = main.tile_count() > 1;
    let mut bytes = 0_u64;
    let mut most_precincts = 0;
    let mut most_resolutions = 0;
    for (index, &component) in (0..main.numcomps).zip(&main.comps) {
        let bounds = component_rect(rect, component);
        let style = main.main_style(index);
        most_resolutions = most_resolutions.max(style.numresolutions);
        let structures = structure_bytes(style, reduce, bounds, &mut most_precincts);
        // The working arena reserves blocks in a doubling series.
        bytes = bytes.saturating_add(structures.saturating_mul(2));
        if several_tiles {
            let width = ceil_div_pow2(bounds.x1, reduce) - ceil_div_pow2(bounds.x0, reduce);
            let height = ceil_div_pow2(bounds.y1, reduce) - ceil_div_pow2(bounds.y0, reduce);
            bytes = bytes.saturating_add(sample_bytes(width, height));
        }
    }
    let packet_bits = (u64::from(main.cod.numlayers) + 1)
        .saturating_mul(u64::from(most_resolutions))
        .saturating_mul(u64::from(main.numcomps))
        .saturating_mul(most_precincts);
    bytes.saturating_add((packet_bits / 8 + 16).saturating_mul(2))
}

pub(crate) fn working_memory(main: &MainHeader, reduce: u32) -> u64 {
    let indices = |len: u32| [Some(0), (len > 2).then_some(1), (len > 1).then_some(len - 1)];
    // The largest tile is bounded by sampling the first, inner, and last rows and columns.
    let mut largest = 0;
    for row in indices(main.th).into_iter().flatten() {
        for col in indices(main.tw).into_iter().flatten() {
            largest = largest.max(tile_bytes(main, row * main.tw + col, reduce));
        }
    }
    image_bytes(main, reduce).saturating_add(largest)
}
