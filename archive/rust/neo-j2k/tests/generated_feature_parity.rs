// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

#![cfg(not(miri))]

use neo_j2k::{DecodeParams, Decoder, DecoderConfig};
use neo_j2k_oracle::{BackendKind, OracleBackend};
use neo_j2k_testkit::files::corpus_dir;
use neo_j2k_testkit::request::{Backend, Component, Mode, Request};

#[test]
fn odd_origin_signed_subsampling_matches_openjpeg() {
    let mut oracle = OracleBackend::new(BackendKind::Oracle);
    for fixture in ["mix_odd_sub420_signed12_53.j2k", "mix_odd_sub420_signed12_97.j2k"] {
        let input = std::fs::read(corpus_dir().join(fixture)).expect("a generated OpenJPEG fixture");
        for reduce in 0..=2 {
            let mut rust = Decoder::new(DecoderConfig {
                reduce,
                ..DecoderConfig::default()
            });
            for layers in [0, 1, 2] {
                let case = format!("{fixture}, reduce {reduce}, layers {layers}");
                let mut request = Request::new(&input, i32::try_from(reduce).unwrap(), Mode::Strict);
                request.layers = i32::try_from(layers).unwrap();
                let expected = oracle.decode(&request);
                assert!(
                    expected.ok,
                    "{case}: OpenJPEG refused the fixture: {:?}",
                    expected.last_error
                );
                let params = DecodeParams {
                    max_layers: layers,
                    strict: true,
                    ..DecodeParams::default()
                };
                let actual = rust
                    .decode(&input, params)
                    .unwrap_or_else(|error| panic!("{case}: {error}"));
                assert!(!actual.truncated, "{case}: truncated result");
                assert_eq!(actual.components.len(), expected.comps.len(), "{case}: component count");
                assert_eq!(
                    expected.comps.len(),
                    expected.planes.len(),
                    "{case}: oracle plane count"
                );
                for (index, ((component, geometry), plane)) in actual
                    .components
                    .iter()
                    .zip(&expected.comps)
                    .zip(&expected.planes)
                    .enumerate()
                {
                    let info = component.info();
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
                            resno_decoded: component.resolution(),
                        },
                        *geometry,
                        "{case}: component {index} geometry"
                    );
                    assert_eq!(
                        component.samples().len(),
                        plane.len(),
                        "{case}: component {index} length"
                    );
                    for (sample, (actual, expected)) in component.samples().iter().zip(plane).enumerate() {
                        assert_eq!(actual, expected, "{case}: component {index}, sample {sample}");
                    }
                }
            }
        }
    }
}
