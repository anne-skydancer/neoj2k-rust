// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Replay the committed OpenJPEG reference digests using the native codec alone.

#![cfg(not(miri))]

use std::collections::{HashMap, HashSet};

use neo_j2k::{DecodeParams, DecodedComponent, Decoder, DecoderConfig, Diagnostic, Format, Jp2Channels, Limits};
use neo_j2k_testkit::digest::{CutDigest, cut_digest, whole_value};
use neo_j2k_testkit::files::{corpus_dir, repo_root};
use neo_j2k_testkit::reference::{Line, parse_table};
use neo_j2k_testkit::request::{Backend, Component, Mode, Outcome, Request};
use neo_j2k_testkit::{scan, viewer};

struct Native;

impl Backend for Native {
    fn name(&self) -> &str {
        "rust"
    }

    fn decode(&mut self, request: &Request<'_>) -> Outcome {
        let viewer = request.mode == Mode::Viewer;
        let mut config = DecoderConfig {
            format: if request.jp2 { Format::Jp2 } else { Format::J2k },
            limits: if request.limits {
                Limits::SECOND_LIFE
            } else {
                Limits::default()
            },
            ..DecoderConfig::default()
        };
        if request.limits && request.max_memory != 0 {
            config.limits.max_memory = request.max_memory;
        }
        let mut outcome = Outcome::default();
        let reduce = if viewer {
            let Some(reduce) = viewer_reduce(request, config) else {
                outcome.stage = b"header".to_vec();
                return outcome;
            };
            reduce
        } else {
            request.reduce.cast_unsigned()
        };

        let reduce_after_header = !viewer && request.reduce_after_header;
        config.reduce = if reduce_after_header { 0 } else { reduce };
        config.capture_diagnostics = true;
        let mut decoder = Decoder::new(config);
        let Ok(info) = decoder.read_header(request.data) else {
            record(&mut outcome, decoder.diagnostics());
            outcome.stage = b"header".to_vec();
            return outcome;
        };
        let bounds = (info.x0, info.y0, info.x1, info.y1, info.color_space as i32);
        let allowed = !viewer
            || viewer::decode_allowed(
                u64::from(info.width()),
                u64::from(info.height()),
                info.components.len() as u64,
                reduce,
            );
        record(&mut outcome, decoder.diagnostics());
        if !allowed {
            outcome.stage = b"guard".to_vec();
            return outcome;
        }
        if reduce_after_header && request.reduce > 0 {
            let result = decoder.set_reduce(reduce);
            record(&mut outcome, decoder.diagnostics());
            if result.is_err() {
                outcome.stage = b"header".to_vec();
                return outcome;
            }
        }

        let params = DecodeParams {
            max_layers: if viewer { 0 } else { request.layers.cast_unsigned() },
            strict: request.mode == Mode::Strict,
            jp2_channels: Jp2Channels::Apply,
            ..DecodeParams::default()
        };
        match decoder.decode(request.data, params) {
            Ok(image) if !image.components.is_empty() => {
                if viewer && !viewer_geometry(image.components) {
                    outcome.stage = b"geometry".to_vec();
                } else {
                    (outcome.x0, outcome.y0, outcome.x1, outcome.y1, outcome.color_space) = bounds;
                    outcome.numcomps = u32::try_from(image.components.len()).unwrap();
                    for plane in image.components {
                        outcome.comps.push(component(plane));
                        outcome.planes.push(plane.samples().to_vec());
                    }
                    outcome.ok = true;
                }
            }
            _ => outcome.stage = b"decode".to_vec(),
        }
        record(&mut outcome, decoder.diagnostics());
        outcome
    }
}

// The viewer reads metadata on its own codec before asking a second codec to decode.
fn viewer_reduce(request: &Request<'_>, config: DecoderConfig) -> Option<u32> {
    let mut metadata = Decoder::new(config);
    let info = metadata.read_header(request.data).ok()?;
    if !viewer::components_supported(info.components.len() as u64) {
        return None;
    }
    let (across, down) = (u64::from(info.tiles_across), u64::from(info.tiles_down));
    let (tile_w, tile_h) = (info.tile_width, info.tile_height);
    let resolutions = if across * u64::from(tile_w) <= 0xffff && down * u64::from(tile_h) <= 0xffff {
        i32::try_from(info.min_resolutions).unwrap()
    } else {
        0
    };
    Some(
        viewer::reduce_factor(
            request.reduce,
            resolutions,
            viewer::grid_side(across, tile_w),
            viewer::grid_side(down, tile_h),
            4096,
        )
        .cast_unsigned(),
    )
}

fn viewer_geometry(planes: &[DecodedComponent]) -> bool {
    let base = planes[0].info();
    (1..=65535).contains(&base.width)
        && (1..=65535).contains(&base.height)
        && planes.iter().take(4).all(|plane| {
            let info = plane.info();
            (info.width, info.height, info.reduce) == (base.width, base.height, base.reduce)
                && plane.samples().len() as u64 == u64::from(info.width) * u64::from(info.height)
        })
}

fn component(plane: &DecodedComponent) -> Component {
    let info = plane.info();
    Component {
        w: info.width,
        h: info.height,
        dx: info.dx,
        dy: info.dy,
        x0: info.x0,
        y0: info.y0,
        prec: info.precision,
        sgnd: u32::from(info.signed),
        factor: info.reduce,
        resno_decoded: plane.resolution(),
    }
}

fn record(outcome: &mut Outcome, diagnostics: &[Diagnostic]) {
    fn append(texts: &mut Vec<u8>, text: &str) {
        if !texts.is_empty() {
            texts.extend_from_slice(b" | ");
        }
        texts.extend_from_slice(text.as_bytes());
    }

    for diagnostic in diagnostics {
        match diagnostic {
            Diagnostic::Warning(text) => {
                outcome.warnings += 1;
                let text = text.trim_end_matches(['\r', '\n']);
                let kind = text.split_once(" (").map_or(text, |(kind, _)| kind);
                if !kind.is_empty()
                    && outcome.warning_kinds.len() < 400
                    && !outcome
                        .warning_kinds
                        .windows(kind.len())
                        .any(|part| part == kind.as_bytes())
                {
                    append(&mut outcome.warning_kinds, kind);
                }
            }
            Diagnostic::Error(text) => {
                outcome.errors += 1;
                if outcome.last_error.len() < 400 {
                    append(&mut outcome.last_error, text.trim_end_matches(['\r', '\n']));
                }
            }
            _ => {}
        }
    }
}

fn fixture(file: &str) -> Vec<u8> {
    std::fs::read(corpus_dir().join(file)).unwrap_or_else(|error| panic!("{file}: {error}"))
}

fn verify(table: &str, use_departures: bool, select: impl Fn(&Line) -> bool) {
    let text = std::fs::read_to_string(corpus_dir().join(table)).expect("the committed table");
    let table_lines = parse_table(&text).expect("valid reference rows");
    let departure_text = if use_departures {
        std::fs::read_to_string(repo_root().join("tests/neoj2k-departures.txt")).expect("the C++ departure list")
    } else {
        String::new()
    };
    let departures: HashSet<_> = departure_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_whitespace().next())
        .collect();
    let mut skipped = HashSet::new();
    let mut files = HashMap::new();
    let mut backend = Native;
    let mut checked = 0;
    for line in table_lines.iter().filter(|line| select(line)) {
        let file = match line {
            Line::Whole { file, .. } | Line::Cuts { file, .. } => file.as_str(),
        };
        if departures.contains(file) {
            skipped.insert(file);
            continue;
        }
        let data = files.entry(file).or_insert_with(|| fixture(file));
        match *line {
            Line::Whole {
                mode,
                reduce,
                bytes,
                ok,
                hash,
                ..
            } => {
                assert_eq!(data.len(), bytes, "{file} byte count");
                let outcome = backend.decode(&Request::new(data, reduce, mode));
                assert_eq!(
                    (outcome.ok, whole_value(&outcome, mode)),
                    (ok, hash),
                    "{file} {mode} r{reduce}"
                );
            }
            Line::Cuts {
                mode,
                reduce,
                from,
                step,
                count,
                decoded,
                digest,
                ..
            } => {
                let actual = cut_digest(&mut backend, data, mode, reduce, from, step);
                assert_eq!(
                    actual,
                    CutDigest { count, decoded, digest },
                    "{file} {mode} r{reduce} / from {from} step {step}"
                );
            }
        }
        checked += 1;
    }
    assert_eq!(
        skipped, departures,
        "the departure list must match this table selection"
    );
    assert!(checked > 0, "no table rows selected for {}", backend.name());
    println!("checked {checked} rows from {table}");
}

#[test]
fn generated_reference_whole() {
    verify("oracle-reference.tsv", true, |line| matches!(line, Line::Whole { .. }));
}

#[test]
fn generated_reference_cuts() {
    verify("oracle-reference.tsv", true, |line| matches!(line, Line::Cuts { .. }));
}

#[test]
#[ignore = "exhaustive nightly replay of generated inputs up to 16 KiB"]
fn nightly_reference_cuts() {
    verify("oracle-reference-nightly.tsv", false, |line| {
        matches!(line, Line::Cuts { .. })
    });
}

#[test]
fn dp3_matches_the_cpp_departure_reference() {
    verify("neoj2k-reference-departures.tsv", false, |_| true);
}

#[test]
fn dp3_empty_ppt_tiles_decode_to_the_sample_midpoint() {
    let data = fixture("edge_ppt_empty_tiles_97.j2k");
    let main = scan::parse_main_header(&data).expect("DP-3 main header");
    let parts = scan::scan_tile_parts(&data, &main);
    let mut decoder = Decoder::new(DecoderConfig::default());
    let info = decoder.read_header(&data).expect("DP-3 header").clone();
    let image = decoder.decode(&data, DecodeParams::default()).expect("DP-3 decode");
    let mut checked = 0;
    for tile in (0..info.tiles_across * info.tiles_down).filter(|&tile| {
        let same_tile = || parts.iter().filter(move |part| u32::from(part.tile) == tile);
        same_tile().any(|part| part.has_ppt) && same_tile().all(|part| part.end == part.data)
    }) {
        let x = info.tile_x0 + tile % info.tiles_across * info.tile_width;
        let y = info.tile_y0 + tile / info.tiles_across * info.tile_height;
        for plane in image.components {
            let c = plane.info();
            assert_eq!((c.dx, c.dy), (1, 1), "the DP-3 fixture is unsubsampled");
            let mid = if c.signed { 0 } else { 1 << (c.precision - 1) };
            for row in y.max(info.y0)..(y + info.tile_height).min(info.y1) {
                let start = (row - info.y0) * c.width + x.max(info.x0) - info.x0;
                let width = (x + info.tile_width).min(info.x1) - x.max(info.x0);
                assert!(
                    plane.samples()[start as usize..(start + width) as usize]
                        .iter()
                        .all(|&value| value == mid)
                );
                checked += width;
            }
        }
    }
    assert!(checked > 0, "the fixture must contain an empty PPT tile");
}
