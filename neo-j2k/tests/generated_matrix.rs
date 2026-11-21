// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use neo_j2k::{DecodeParams, Decoder, DecoderConfig};
use neo_j2k_oracle::{BackendKind, OracleBackend, generate_codestream};
use neo_j2k_testkit::request::{Backend, Component, Mode, Request};
use neo_j2k_testkit::scan::{MainHeader, parse_main_header};

const CI_SEED: u32 = 0xf72a_149b;

#[test]
fn generated_coding_matrix_matches_openjpeg() {
    compare_matrix(CI_SEED, 48);
}

#[test]
#[ignore = "larger generated matrix for nightly or local verification"]
fn extended_generated_coding_matrix_matches_openjpeg() {
    compare_matrix(0x4d89_a743, 96);
    compare_matrix(0xc3d1_e5b9, 96);
}

fn all_components_visible(siz: &MainHeader, reduce: u32) -> bool {
    // A coarsest level can have no samples for an offset or subsampled component.
    // This matrix makes requests where every component has a nonempty image plane.
    let step = 1_u32 << reduce;
    siz.xrsiz.iter().zip(&siz.yrsiz).all(|(&dx, &dy)| {
        let dx = u32::from(dx) * step;
        let dy = u32::from(dy) * step;
        siz.xsiz.div_ceil(dx) > siz.xosiz.div_ceil(dx) && siz.ysiz.div_ceil(dy) > siz.yosiz.div_ceil(dy)
    })
}

fn compare_matrix(seed: u32, cases: u32) {
    let mut oracle = OracleBackend::new(BackendKind::Oracle);
    let mut completed = 0;
    for index in 0..cases * 2 {
        let label = format!("OpenJPEG seed {seed:08x}, case {index}");
        let input = match generate_codestream(seed, index) {
            Ok(input) => input,
            // OpenJPEG can refuse the generated tile when packet headers exhaust its output
            // buffer. Keep drawing cases so the matrix still checks its requested count.
            Err(-8) => continue,
            Err(status) => panic!("{label}: encode status {status}"),
        };
        let mut decoder = Decoder::new(DecoderConfig::default());
        let header = decoder
            .read_header(&input)
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        let bounds = (header.x0, header.y0, header.x1, header.y1, header.color_space as i32);
        let siz = parse_main_header(&input).unwrap();
        let resolutions = u32::try_from(siz.min_resolutions).unwrap();
        assert_eq!(header.min_resolutions, resolutions, "{label}: resolution count");
        let last_reduce = (0..resolutions)
            .rev()
            .find(|&reduce| all_components_visible(&siz, reduce))
            .expect("the generated source has nonempty full-resolution planes");
        let mut previous = None;
        for reduce in [0, last_reduce.min(2), last_reduce] {
            if previous == Some(reduce) {
                continue;
            }
            previous = Some(reduce);
            let mut decoder = Decoder::new(DecoderConfig {
                reduce,
                ..DecoderConfig::default()
            });
            for layers in [0, 1] {
                let case = format!("{label}, reduce {reduce}, layers {layers}");
                let mut request = Request::new(&input, i32::try_from(reduce).unwrap(), Mode::Plain);
                request.layers = layers;
                let expected = oracle.decode(&request);
                assert!(
                    expected.ok,
                    "{case}: OpenJPEG refused its own output at {}: {}",
                    String::from_utf8_lossy(&expected.stage),
                    String::from_utf8_lossy(&expected.last_error),
                );
                assert_eq!(
                    bounds,
                    (expected.x0, expected.y0, expected.x1, expected.y1, expected.color_space),
                    "{case}: image geometry",
                );
                let params = DecodeParams {
                    max_layers: u32::try_from(layers).unwrap(),
                    ..DecodeParams::default()
                };
                let actual = decoder
                    .decode(&input, params)
                    .unwrap_or_else(|error| panic!("{case}: {error}"));
                assert!(!actual.truncated, "{case}: truncated");
                assert_eq!(actual.components.len(), expected.comps.len(), "{case}: component count");
                assert_eq!(
                    expected.comps.len(),
                    expected.planes.len(),
                    "{case}: oracle plane count"
                );
                for (plane_index, ((plane, dimensions), samples)) in actual
                    .components
                    .iter()
                    .zip(&expected.comps)
                    .zip(&expected.planes)
                    .enumerate()
                {
                    let info = plane.info();
                    assert_eq!(
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
                        },
                        *dimensions,
                        "{case}: component {plane_index} dimensions",
                    );
                    assert_eq!(
                        plane.samples().len(),
                        samples.len(),
                        "{case}: component {plane_index} sample count"
                    );
                    if let Some(at) = plane
                        .samples()
                        .iter()
                        .zip(samples)
                        .position(|(actual, expected)| actual != expected)
                    {
                        panic!(
                            "{case}: component {plane_index}, sample {at}: Rust {}, OpenJPEG {}",
                            plane.samples()[at],
                            samples[at],
                        );
                    }
                }
            }
        }
        completed += 1;
        if completed == cases {
            break;
        }
    }
    assert_eq!(
        completed, cases,
        "too many OpenJPEG encoder refusals for seed {seed:08x}"
    );
}
