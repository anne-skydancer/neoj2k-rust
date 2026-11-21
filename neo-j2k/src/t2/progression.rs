// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! JPEG 2000 packet order (OpenJPEG's pi.c via src/t2.cpp; notice in the parent module).

use std::ops::Range;

use crate::geometry::Rect;
use crate::header::{PocEntry, TileCoding};
use crate::mem::{CType, WorkBudget};
use crate::tile::Tile;
use crate::{Error, ErrorKind};

// The work arena charges sizeof/alignof of C++ PiComp and PiResolution, respectively.
const PI_COMP: (usize, usize) = (24, 8);
const PI_RESOLUTION: (usize, usize) = (16, 4);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Packet {
    pub(super) comp: usize,
    pub(super) res: usize,
    pub(super) precinct: usize,
    pub(super) layer: u32,
}

#[derive(Clone, Copy, Debug, Default)]
struct Resolution {
    pdx: u32,
    pdy: u32,
    pw: u32,
    ph: u32,
}

impl Resolution {
    fn precincts(self) -> Option<usize> {
        usize::try_from(self.pw.checked_mul(self.ph)?).ok()
    }
}

#[derive(Debug, Default)]
struct Component {
    dx: u32,
    dy: u32,
    resolutions: Vec<Resolution>,
    step: (u32, u32),
}

impl Component {
    fn precinct_at(&self, rect: Rect, resno: usize, x: u32, y: u32) -> Option<usize> {
        let res = self.resolutions[resno];
        let levels = u32::try_from(self.resolutions.len() - 1 - resno).ok()?;
        let col = precinct_axis(rect.x0..rect.x1, x, self.dx, levels, res.pdx)?;
        let row = precinct_axis(rect.y0..rect.y1, y, self.dy, levels, res.pdy)?;
        if col >= res.pw || row >= res.ph {
            return None;
        }
        usize::try_from(row.checked_mul(res.pw)?.checked_add(col)?).ok()
    }
}

fn precinct_axis(bounds: Range<u32>, pos: u32, sampling: u32, levels: u32, exponent: u32) -> Option<u32> {
    let unit = u64::from(sampling).checked_shl(levels)?;
    let step_exp = levels.checked_add(exponent)?;
    let step = u64::from(sampling).checked_shl(step_exp)?;
    if unit == 0 || step == 0 || (step >> step_exp) != u64::from(sampling) {
        return None;
    }
    let begin = u64::from(bounds.start).div_ceil(unit);
    if begin == u64::from(bounds.end).div_ceil(unit) {
        return None;
    }
    // ISO 15444-1 B.12.1.3–5: the tile origin can begin an unaligned first precinct.
    let origin = pos == bounds.start && !(begin << levels).is_multiple_of(1u64 << step_exp);
    if !u64::from(pos).is_multiple_of(step) && !origin {
        return None;
    }
    let index = (u64::from(pos).div_ceil(unit) >> exponent).checked_sub(begin >> exponent)?;
    u32::try_from(index).ok()
}

fn smaller_step(previous: u32, step: u32) -> u32 {
    if previous == 0 { step } else { previous.min(step) }
}

fn update_step(step: &mut u32, sampling: u32, exponent: u32) {
    if let Some(value) = 1u32.checked_shl(exponent).and_then(|scale| sampling.checked_mul(scale)) {
        *step = smaller_step(*step, value);
    }
}

fn coordinates(bounds: Range<u32>, step: u32) -> impl Iterator<Item = u32> {
    std::iter::successors((bounds.start < bounds.end).then_some(bounds.start), move |&value| {
        value.checked_add(step - value % step).filter(|&next| next < bounds.end)
    })
}

#[derive(Debug, Default)]
pub(super) struct Grid {
    rect: Rect,
    components: Vec<Component>,
    active: usize,
    max_res: usize,
    max_prec: usize,
    step: (u32, u32),
}

fn no_memory() -> Error {
    Error::new(ErrorKind::NoMemory, "Not enough memory for the packet iterator\n")
}

fn grow<T: Default>(array: &mut Vec<T>, len: usize) -> Result<(), Error> {
    if len > array.len() {
        array.try_reserve_exact(len - array.len()).map_err(|_| no_memory())?;
        array.resize_with(len, T::default);
    }
    Ok(())
}

impl Grid {
    pub(super) fn configure(&mut self, tile: &Tile, budget: &mut WorkBudget) -> Result<(), Error> {
        budget.charge(
            PI_COMP.0.checked_mul(tile.comps.len()).ok_or_else(no_memory)?,
            PI_COMP.1,
        )?;
        self.rect = tile.rect;
        self.active = tile.comps.len();
        self.max_res = 0;
        self.max_prec = 0;
        self.step = (0, 0);
        // Keep the inactive component slots and their nested allocations for the next tile.
        grow(&mut self.components, self.active)?;
        for (comp, component) in tile.comps.iter().zip(&mut self.components) {
            let count = comp.res.len();
            budget.charge(
                PI_RESOLUTION.0.checked_mul(count).ok_or_else(no_memory)?,
                PI_RESOLUTION.1,
            )?;
            component.dx = comp.dx;
            component.dy = comp.dy;
            component.step = (0, 0);
            grow(&mut component.resolutions, count)?;
            component.resolutions.truncate(count);
            for (index, (res, resolution)) in comp.res.iter().zip(&mut component.resolutions).enumerate() {
                *resolution = Resolution {
                    pdx: res.pdx,
                    pdy: res.pdy,
                    pw: res.pw,
                    ph: res.ph,
                };
                let level = u32::try_from(count - index - 1).map_err(|_| no_memory())?;
                update_step(&mut component.step.0, comp.dx, resolution.pdx + level);
                update_step(&mut component.step.1, comp.dy, resolution.pdy + level);
                self.max_prec = self.max_prec.max(resolution.precincts().ok_or_else(no_memory)?);
            }
            if component.step.0 != 0 {
                self.step.0 = smaller_step(self.step.0, component.step.0);
            }
            if component.step.1 != 0 {
                self.step.1 = smaller_step(self.step.1, component.step.1);
            }
            self.max_res = self.max_res.max(count);
        }
        Ok(())
    }

    pub(super) fn prepare_seen(
        &self,
        seen: &mut Seen,
        coding: TileCoding,
        budget: &mut WorkBudget,
    ) -> Result<(), Error> {
        let count = (usize::from(coding.numlayers) + 1)
            .checked_mul(self.max_res)
            .and_then(|value| value.checked_mul(self.active))
            .and_then(|value| value.checked_mul(self.max_prec))
            .and_then(|value| u32::try_from(value).ok())
            .ok_or_else(no_memory)?;
        let len = usize::try_from(count.div_ceil(64) + 1).map_err(|_| no_memory())?;
        budget.array(CType::U64, len)?;
        grow(&mut seen.words, len)?;
        seen.words.truncate(len);
        seen.words.fill(0);
        seen.max_res = self.max_res;
        seen.max_prec = self.max_prec;
        seen.comps = self.active;
        Ok(())
    }

    pub(super) fn visit(
        &self,
        coding: TileCoding,
        poc: Option<&PocEntry>,
        mut visitor: impl FnMut(Packet) -> Result<bool, Error>,
    ) -> Result<bool, Error> {
        let window = if let Some(entry) = poc {
            Window {
                order: entry.prg,
                components: usize::from(entry.compno0)..usize::from(entry.compno1).min(self.active),
                resolutions: usize::from(entry.resno0)..usize::from(entry.resno1),
                layers: u32::from(entry.layno1.min(coding.numlayers)),
            }
        } else {
            Window {
                order: coding.prg,
                components: 0..self.active,
                resolutions: 0..self.max_res,
                layers: u32::from(coding.numlayers),
            }
        };
        match window.order {
            0 => {
                for layer in 0..window.layers {
                    for res in window.resolutions.clone() {
                        if self.visit_components(&window, layer, res, &mut visitor)? {
                            return Ok(true);
                        }
                    }
                }
            }
            1 => {
                for res in window.resolutions.clone() {
                    for layer in 0..window.layers {
                        if self.visit_components(&window, layer, res, &mut visitor)? {
                            return Ok(true);
                        }
                    }
                }
            }
            2 if self.step.0 != 0 && self.step.1 != 0 => {
                for res in window.resolutions.clone() {
                    for y in coordinates(self.rect.y0..self.rect.y1, self.step.1) {
                        for x in coordinates(self.rect.x0..self.rect.x1, self.step.0) {
                            for comp in window.components.clone() {
                                if self.visit_at(&window, comp, res, (x, y), &mut visitor)? {
                                    return Ok(true);
                                }
                            }
                        }
                    }
                }
            }
            3 if self.step.0 != 0 && self.step.1 != 0 => {
                for y in coordinates(self.rect.y0..self.rect.y1, self.step.1) {
                    for x in coordinates(self.rect.x0..self.rect.x1, self.step.0) {
                        for comp in window.components.clone() {
                            if self.visit_resolutions_at(&window, comp, (x, y), &mut visitor)? {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
            4 => {
                for comp in window.components.clone() {
                    let (dx, dy) = self.components[comp].step;
                    if dx == 0 || dy == 0 {
                        return Ok(false);
                    }
                    for y in coordinates(self.rect.y0..self.rect.y1, dy) {
                        for x in coordinates(self.rect.x0..self.rect.x1, dx) {
                            if self.visit_resolutions_at(&window, comp, (x, y), &mut visitor)? {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(false)
    }

    fn visit_components(
        &self,
        window: &Window,
        layer: u32,
        res: usize,
        visitor: &mut impl FnMut(Packet) -> Result<bool, Error>,
    ) -> Result<bool, Error> {
        for comp in window.components.clone() {
            let Some(grid) = self.components[comp].resolutions.get(res) else {
                continue;
            };
            let count = grid.precincts().ok_or_else(no_memory)?;
            for precinct in 0..count {
                if visitor(Packet {
                    comp,
                    res,
                    precinct,
                    layer,
                })? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn visit_resolutions_at(
        &self,
        window: &Window,
        comp: usize,
        point: (u32, u32),
        visitor: &mut impl FnMut(Packet) -> Result<bool, Error>,
    ) -> Result<bool, Error> {
        for res in window.resolutions.start..window.resolutions.end.min(self.components[comp].resolutions.len()) {
            if self.visit_at(window, comp, res, point, visitor)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn visit_at(
        &self,
        window: &Window,
        comp: usize,
        res: usize,
        (x, y): (u32, u32),
        visitor: &mut impl FnMut(Packet) -> Result<bool, Error>,
    ) -> Result<bool, Error> {
        let component = &self.components[comp];
        if res >= component.resolutions.len() {
            return Ok(false);
        }
        if let Some(precinct) = component.precinct_at(self.rect, res, x, y) {
            for layer in 0..window.layers {
                if visitor(Packet {
                    comp,
                    res,
                    precinct,
                    layer,
                })? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

struct Window {
    order: u8,
    components: Range<usize>,
    resolutions: Range<usize>,
    layers: u32,
}

#[derive(Debug, Default)]
pub(super) struct Seen {
    words: Vec<u64>,
    max_res: usize,
    max_prec: usize,
    comps: usize,
}

impl Seen {
    pub(super) fn insert(&mut self, packet: Packet) -> bool {
        let layer = usize::try_from(packet.layer).expect("a layer count fits usize");
        let index = ((layer * self.max_res + packet.res) * self.comps + packet.comp) * self.max_prec + packet.precinct;
        let word = &mut self.words[index / 64];
        let mask = 1u64 << (index % 64);
        let new = *word & mask == 0;
        *word |= mask;
        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_progressions_and_overlapping_pocs_visit_the_same_packets_in_order() {
        let rect = Rect {
            x0: 0,
            y0: 0,
            x1: 4,
            y1: 1,
        };
        let comp = || Component {
            dx: 1,
            dy: 1,
            resolutions: vec![
                Resolution {
                    pdx: 0,
                    pdy: 0,
                    pw: 2,
                    ph: 1,
                },
                Resolution {
                    pdx: 1,
                    pdy: 0,
                    pw: 2,
                    ph: 1,
                },
            ],
            step: (2, 1),
        };
        let grid = Grid {
            rect,
            components: vec![comp(), comp()],
            active: 2,
            max_res: 2,
            max_prec: 2,
            step: (2, 1),
        };
        let mut all = Vec::new();
        for layer in 0..2 {
            for comp in 0..2 {
                for res in 0..2 {
                    for precinct in 0..2 {
                        all.push(Packet {
                            comp,
                            res,
                            precinct,
                            layer,
                        });
                    }
                }
            }
        }
        for prg in 0..=4 {
            let coding = TileCoding {
                csty: 0,
                prg,
                numlayers: 2,
                mct: 0,
            };
            let mut expected = all.clone();
            expected.sort_by_key(|p| {
                let l = usize::try_from(p.layer).unwrap();
                match prg {
                    0 => [l, p.res, p.comp, p.precinct],
                    1 => [p.res, l, p.comp, p.precinct],
                    2 => [p.res, p.precinct, p.comp, l],
                    3 => [p.precinct, p.comp, p.res, l],
                    _ => [p.comp, p.precinct, p.res, l],
                }
            });
            let mut visited = Vec::new();
            grid.visit(coding, None, |packet| {
                visited.push(packet);
                Ok(false)
            })
            .unwrap();
            assert_eq!(visited, expected, "order {prg}");
        }
        let mut budget = WorkBudget::default();
        let coding = TileCoding {
            csty: 0,
            prg: 0,
            numlayers: 2,
            mct: 0,
        };
        let mut seen = Seen::default();
        grid.prepare_seen(&mut seen, coding, &mut budget).unwrap();
        let short = PocEntry {
            prg: 2,
            resno0: 0,
            resno1: 2,
            compno0: 0,
            compno1: 2,
            layno1: 1,
        };
        let long = PocEntry {
            prg: 4,
            layno1: 2,
            ..short
        };
        for (poc, layer) in [(short, 0), (long, 1)] {
            let mut visited = Vec::new();
            grid.visit(coding, Some(&poc), |packet| {
                if seen.insert(packet) {
                    visited.push(packet);
                }
                Ok(false)
            })
            .unwrap();
            assert_eq!(visited.len(), 8);
            assert!(visited.iter().all(|p| p.layer == layer));
        }
    }
}
