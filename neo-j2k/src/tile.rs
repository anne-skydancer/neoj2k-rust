// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
//
// Tile geometry and dequantization follow OpenJPEG's tcd.c through src/tile.cpp.
// See LICENSE.OpenJPEG for OpenJPEG's notices.

use crate::geometry::{self, Rect, ceil_div_pow2};
use crate::header::{MainHeader, SizComponent, tile::TileComponentParams};
use crate::mem::{CType, WorkBudget};
use crate::t2::TagTree;
use crate::{Error, ErrorKind};
use core::ops::Range;

const OPJ_PRECINCT_BYTES: u32 = 56;
const OPJ_BLOCK_BYTES: u32 = 88;
const HT_BLOCK: u8 = 0x40;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Segment {
    pub(crate) len: u32,
    pub(crate) numpasses: u32,
    pub(crate) real_num_passes: u32,
    pub(crate) maxpasses: u32,
    pub(crate) numnewpasses: u32,
    pub(crate) newlen: u32,
}

#[derive(Debug, Default)]
pub(crate) struct CodeBlock {
    pub(crate) rect: Rect,
    pub(crate) segs: Vec<Segment>,
    pub(crate) chunks: Vec<Range<usize>>,
    pub(crate) numbps: u32,
    pub(crate) numlenbits: u32,
    pub(crate) numsegs: u32,
    pub(crate) real_num_segs: u32,
    pub(crate) numnewpasses: u32,
    pub(crate) corrupted: bool,
}

impl CodeBlock {
    fn reset(&mut self, rect: Rect) {
        self.segs.clear();
        self.chunks.clear();
        self.rect = rect;
        self.numbps = 0;
        self.numlenbits = 0;
        self.numsegs = 0;
        self.real_num_segs = 0;
        self.numnewpasses = 0;
        self.corrupted = false;
    }
}

#[derive(Debug, Default)]
pub(crate) struct Precinct {
    pub(crate) rect: Rect,
    pub(crate) cw: u32,
    pub(crate) ch: u32,
    pub(crate) cblks: Vec<CodeBlock>,
    pub(crate) incl: TagTree,
    pub(crate) imsb: TagTree,
}

#[derive(Debug, Default)]
pub(crate) struct Band {
    pub(crate) rect: Rect,
    pub(crate) bandno: u32,
    pub(crate) numbps: i32,
    pub(crate) stepsize: f32,
    pub(crate) precincts: Vec<Precinct>,
}

impl Band {
    pub(crate) const fn empty(&self) -> bool {
        self.rect.width() == 0 || self.rect.height() == 0
    }
}

#[derive(Debug, Default)]
pub(crate) struct Resolution {
    pub(crate) rect: Rect,
    pub(crate) pw: u32,
    pub(crate) ph: u32,
    pub(crate) pdx: u32,
    pub(crate) pdy: u32,
    pub(crate) bands: Vec<Band>,
}

#[derive(Debug, Default)]
pub(crate) struct TileComp {
    pub(crate) dx: u32,
    pub(crate) dy: u32,
    pub(crate) rect: Rect,
    pub(crate) cblksty: u8,
    pub(crate) minimum_num_resolutions: u32,
    pub(crate) reversible: bool,
    pub(crate) roishift: u8,
    pub(crate) res: Vec<Resolution>,
    pub(crate) samples: Vec<i32>,
}

impl TileComp {
    pub(crate) fn area_of_interest(&self, resno: u32, bandno: u32, rect: Rect) -> bool {
        let margin = if self.reversible { 2 } else { 3 };
        let levels = u32::try_from(self.res.len()).expect("one component has at most 33 resolutions") - resno.max(1);
        let band = |coordinate: u32, high: u32| {
            if levels == 0 {
                coordinate
            } else {
                ceil_div_pow2(coordinate.saturating_sub(high << (levels - 1)), levels)
            }
        };
        let x = bandno & 1;
        let y = bandno >> 1;
        rect.x0 < band(self.rect.x1, x).saturating_add(margin)
            && rect.y0 < band(self.rect.y1, y).saturating_add(margin)
            && rect.x1 > band(self.rect.x0, x).saturating_sub(margin)
            && rect.y1 > band(self.rect.y0, y).saturating_sub(margin)
    }

    pub(crate) fn selected_resolution(&self) -> Rect {
        self.res[self.minimum_num_resolutions as usize - 1].rect
    }
}

#[derive(Debug, Default)]
pub(crate) struct Tile {
    pub(crate) rect: Rect,
    pub(crate) comps: Vec<TileComp>,
}

impl Tile {
    pub(crate) fn configure(
        &mut self,
        main: &MainHeader,
        index: u32,
        params: &[TileComponentParams<'_>],
        reduce: u32,
        budget: &mut WorkBudget,
    ) -> Result<(), Error> {
        let bounds = geometry::tile_rect(main, index);
        if bounds.x1 > i32::MAX.unsigned_abs() || bounds.width() == 0 {
            return Err(unsupported("Tile X coordinates are not supported\n"));
        }
        if bounds.y1 > i32::MAX.unsigned_abs() || bounds.height() == 0 {
            return Err(unsupported("Tile Y coordinates are not supported\n"));
        }
        if params.iter().any(|p| p.style.cblksty & HT_BLOCK != 0) {
            return Err(unsupported("HT code-blocks (JPEG 2000 Part 15) are not supported\n"));
        }
        self.rect = bounds;
        budget.array(CType::TileComp, main.comps.len())?;
        resize(&mut self.comps, main.comps.len())?;
        for (index, ((component, &sampling), settings)) in
            self.comps.iter_mut().zip(&main.comps).zip(params).enumerate()
        {
            configure_component(component, sampling, bounds, settings, reduce, index, budget)?;
        }
        Ok(())
    }
}

pub(crate) fn too_few_resolutions(index: usize, reduce: u32, resolutions: u32) -> Error {
    Error::new(
        ErrorKind::Limit,
        format!(
            "Error decoding component {index}.\nThe number of resolutions to remove ({reduce}) is greater or equal \
             than the number of resolutions of this component ({resolutions})\nModify the cp_reduce parameter.\n\n"
        ),
    )
}

fn configure_component(
    component: &mut TileComp,
    sampling: SizComponent,
    bounds: Rect,
    params: &TileComponentParams<'_>,
    reduce: u32,
    index: usize,
    budget: &mut WorkBudget,
) -> Result<(), Error> {
    let count = u32::from(params.style.numresolutions);
    if count <= reduce {
        return Err(too_few_resolutions(index, reduce, count));
    }
    component.dx = u32::from(sampling.dx);
    component.dy = u32::from(sampling.dy);
    component.rect = geometry::component_rect(bounds, sampling);
    component.cblksty = params.style.cblksty;
    component.minimum_num_resolutions = count - reduce;
    component.reversible = params.style.qmfbid == 1;
    component.roishift = params.roishift;
    budget.array(CType::Resolution, count as usize)?;
    resize(&mut component.res, count as usize)?;
    let context = ResContext {
        component: component.rect,
        precision: u32::from(sampling.prec),
        params,
    };
    for (resno, resolution) in (0..count).zip(&mut component.res) {
        context.configure_resolution(resolution, resno, count - 1 - resno, budget)?;
    }
    Ok(())
}

struct ResContext<'a> {
    component: Rect,
    precision: u32,
    params: &'a TileComponentParams<'a>,
}

impl ResContext<'_> {
    fn configure_resolution(
        &self,
        resolution: &mut Resolution,
        resno: u32,
        down: u32,
        budget: &mut WorkBudget,
    ) -> Result<(), Error> {
        let rect = band_bounds(self.component, down, 0);
        let precincts = self.params.style.precincts[resno as usize];
        let px = u32::from(precincts & 15);
        let py = u32::from(precincts >> 4);
        let gx = GridAxis::new(rect.x0, rect.x1, px)?;
        let gy = GridAxis::new(rect.y0, rect.y1, py)?;
        let count = from_count(gx.count(), gy.count(), OPJ_PRECINCT_BYTES, "Size of tile data exceeds system limits\n")?;
        resolution.rect = rect;
        resolution.pdx = px;
        resolution.pdy = py;
        resolution.pw = gx.count();
        resolution.ph = gy.count();

        let base = resno == 0;
        let band_count = if base { 1 } else { 3 };
        let precinct_grid = if base {
            [gx, gy]
        } else {
            [gx.halve(), gy.halve()]
        };
        let block_exp = [
            u32::from(self.params.style.cblkw).min(precinct_grid[0].exp),
            u32::from(self.params.style.cblkh).min(precinct_grid[1].exp),
        ];
        resize(&mut resolution.bands, band_count)?;
        for (index, band) in (0_u32..).zip(&mut resolution.bands) {
            let bandno = if base { 0 } else { index + 1 };
            let bounds = band_bounds(self.component, down, bandno);
            let step_index = if base { 0 } else { 3 * resno - 2 + index };
            let (exponent, mantissa) = self.params.quant.step(step_index);
            let gain = if self.params.style.qmfbid == 1 {
                [0, 1, 1, 2][bandno as usize]
            } else {
                0
            };
            band.rect = bounds;
            band.bandno = bandno;
            band.numbps = exponent.cast_signed() + i32::from(self.params.quant.numgbits) - 1;
            band.stepsize = step_size(self.precision + gain, exponent, mantissa);
            budget.array(CType::Precinct, count)?;
            resize(&mut band.precincts, count)?;
            for (index, precinct) in (0_u32..).zip(&mut band.precincts) {
                let column = index % gx.count();
                let row = index / gx.count();
                let bounds = grid_cell(precinct_grid, column, row).clipped_to(bounds);
                configure_precinct(precinct, bounds, block_exp, budget)?;
            }
        }
        Ok(())
    }
}

fn configure_precinct(
    precinct: &mut Precinct,
    bounds: Rect,
    block_exp: [u32; 2],
    budget: &mut WorkBudget,
) -> Result<(), Error> {
    let gx = GridAxis::new(bounds.x0, bounds.x1, block_exp[0])?;
    let gy = GridAxis::new(bounds.y0, bounds.y1, block_exp[1])?;
    let cw = gx.grid_count();
    let ch = gy.grid_count();
    let count = from_count(cw, ch, OPJ_BLOCK_BYTES, "Size of code block data exceeds system limits\n")?;
    precinct.rect = bounds;
    precinct.cw = cw;
    precinct.ch = ch;
    let nodes = TagTree::node_count(cw, ch)?;
    budget.array(CType::TagNode, nodes)?;
    precinct.incl.configure(cw, ch)?;
    budget.array(CType::TagNode, nodes)?;
    precinct.imsb.configure(cw, ch)?;

    budget.array(CType::CodeBlock, count)?;
    resize(&mut precinct.cblks, count)?;
    for (index, block) in (0_u32..).zip(&mut precinct.cblks) {
        let rect = grid_cell([gx, gy], index % cw, index / cw).clipped_to(bounds);
        block.reset(rect);
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct GridAxis {
    start: u32,
    end: u32,
    exp: u32,
    empty: bool,
}

impl GridAxis {
    fn new(start: u32, end: u32, exp: u32) -> Result<Self, Error> {
        let bottom = ceil_div_pow2(end, exp) << exp;
        if bottom > i32::MAX.unsigned_abs() {
            return Err(unsupported("Integer overflow\n"));
        }
        Ok(Self {
            start: (start >> exp) << exp,
            end: bottom,
            exp,
            empty: start == end,
        })
    }

    const fn halve(self) -> Self {
        Self {
            start: self.start >> 1,
            end: self.end >> 1,
            exp: self.exp - 1,
            empty: self.empty,
        }
    }

    const fn grid_count(self) -> u32 {
        (self.end - self.start) >> self.exp
    }

    const fn count(self) -> u32 {
        if self.empty { 0 } else { self.grid_count() }
    }
}

fn grid_cell([gx, gy]: [GridAxis; 2], column: u32, row: u32) -> Rect {
    let x0 = gx.start + (column << gx.exp);
    let y0 = gy.start + (row << gy.exp);
    Rect {
        x0,
        y0,
        x1: x0 + (1 << gx.exp),
        y1: y0 + (1 << gy.exp),
    }
}

fn band_bounds(rect: Rect, down: u32, bandno: u32) -> Rect {
    let axis = |coordinate: u32, high: u32| {
        if bandno == 0 {
            ceil_div_pow2(coordinate, down)
        } else {
            let divisor = 1_i64 << (down + 1);
            let offset = i64::from(high) << down;
            let bound = (i64::from(coordinate) - offset + divisor - 1) >> (down + 1);
            u32::try_from(bound).expect("a subband bound is nonnegative")
        }
    };
    let x = bandno & 1;
    let y = bandno >> 1;
    Rect {
        x0: axis(rect.x0, x),
        y0: axis(rect.y0, y),
        x1: axis(rect.x1, x),
        y1: axis(rect.y1, y),
    }
}

#[allow(clippy::cast_possible_truncation, reason = "the stepsize is rounded to f32 as in C++")]
fn step_size(precision: u32, exponent: u32, mantissa: u32) -> f32 {
    let power = u64::from(1023 + precision - exponent);
    ((1.0 + f64::from(mantissa) / 2048.0) * f64::from_bits(power << 52)) as f32
}

fn from_count(width: u32, height: u32, memory: u32, message: &'static str) -> Result<usize, Error> {
    let count = u64::from(width) * u64::from(height);
    if count > u64::from(u32::MAX / memory) {
        return Err(Error::new(ErrorKind::Limit, message));
    }
    usize::try_from(count).map_err(|_| Error::new(ErrorKind::Limit, message))
}

pub(crate) fn resize<T: Default>(items: &mut Vec<T>, count: usize) -> Result<(), Error> {
    if let Some(additional) = count.checked_sub(items.len()) {
        items.try_reserve_exact(additional).map_err(|_| no_memory())?;
    }
    items.resize_with(count, T::default);
    Ok(())
}

fn unsupported(message: &'static str) -> Error {
    Error::new(ErrorKind::Unsupported, message)
}

fn no_memory() -> Error {
    Error::new(ErrorKind::NoMemory, "Not enough memory to decode\n")
}
