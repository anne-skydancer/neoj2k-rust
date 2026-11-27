// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! The test kit against the C++ harness it transcribes, and stage R0's gate: the digest functions
//! reproduce the oracle's reference table (three named lines, then every whole-file line), the
//! viewer's rules, the scanner and the PGX reader equal the C++'s on the same inputs, and the
//! oracle's own entry points work.

use std::path::PathBuf;

use neo_j2k_oracle::{BackendKind, Codec, Format, OracleBackend, kernels, layout_table, viewer};
use neo_j2k_testkit::digest::{cut_digest, outcome_hash, viewer_u8_hash, whole_value};
use neo_j2k_testkit::files::{base_name, corpus_dir, expand_inputs, repo_root, test_tmp_dir, testdata_dir};
use neo_j2k_testkit::reference::{Line, parse_table};
use neo_j2k_testkit::request::{Mode, Request};
use neo_j2k_testkit::{pgx, scan};

fn reference_table() -> Vec<Line> {
    let text = std::fs::read_to_string(corpus_dir().join("oracle-reference.tsv")).expect("the reference table");
    parse_table(&text).expect("a well-formed reference table")
}

fn corpus_file(name: &str) -> Vec<u8> {
    std::fs::read(corpus_dir().join(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
}

#[test]
fn openjpeg_strictness_controls_partial_downloads() {
    let data = corpus_file("sl_32_rgba_53.j2k");
    let prefix = &data[..data.len() - 16];
    let mut strict = neo_j2k_oracle::openjpeg::Decoder::with_strict(0, 1, true).unwrap();
    let mut partial = neo_j2k_oracle::openjpeg::Decoder::with_strict(0, 1, false).unwrap();
    let mut default = neo_j2k_oracle::openjpeg::Decoder::new(0, 1).unwrap();
    assert!(strict.decode(&data).is_ok());
    assert!(partial.decode(&data).is_ok());
    assert!(strict.decode(prefix).is_err());
    assert!(partial.decode(prefix).is_ok());
    assert!(default.decode(prefix).is_ok());
}

/// The R0 gate: three lines of `oracle-reference.tsv`, each recomputed from the oracle's planes by
/// the test kit's digests (a whole file in plain mode, the same in viewer mode, a cut digest).
#[test]
fn digests_reproduce_three_reference_lines() {
    let named = [
        "whole\tcblk_32x16_97.j2k\tplain\t0\t1529\t1\taef9eee61540d8b9",
        "whole\tcblk_32x16_97.j2k\tviewer\t0\t1529\t1\t6f85c07cefc9dd31",
        "cuts\tcblk_32x16_97.j2k\tplain\t0\t1\t13\t118\t107\ta087b885da04c19a",
    ];
    let table = reference_table();
    let mut oracle = OracleBackend::new(BackendKind::Oracle);
    for text in named {
        let line = neo_j2k_testkit::reference::parse_line(text).unwrap().unwrap();
        assert!(table.contains(&line), "'{text}' is not in the committed table any more");
        match line {
            Line::Whole {
                file,
                mode,
                reduce,
                bytes,
                ok,
                hash,
            } => {
                let data = corpus_file(&file);
                assert_eq!(data.len(), bytes);
                let (o, cpp) = oracle.decode_with_hashes(&Request::new(&data, reduce, mode));
                assert_eq!(o.ok, ok);
                assert_eq!(outcome_hash(&o), cpp.hash, "{text}: the outcome hash");
                if mode == Mode::Viewer {
                    assert_eq!(viewer_u8_hash(&o), cpp.u8_hash, "{text}: the 8-bit image's hash");
                }
                assert_eq!(whole_value(&o, mode), hash, "{text}");
            }
            Line::Cuts {
                file,
                mode,
                reduce,
                from,
                step,
                count,
                decoded,
                digest,
            } => {
                let data = corpus_file(&file);
                let d = cut_digest(&mut oracle, &data, mode, reduce, from, step);
                assert_eq!((d.count, d.decoded, d.digest), (count, decoded, digest), "{text}");
            }
        }
    }
}

/// Every whole-file line of the reference table (every corpus file in plain and viewer mode at
/// reduce 0-5), its value recomputed by the test kit from the oracle's planes.
#[test]
fn digests_reproduce_every_whole_line() {
    let mut oracle = OracleBackend::new(BackendKind::Oracle);
    let mut lines = 0;
    for line in reference_table() {
        if let Line::Whole {
            file,
            mode,
            reduce,
            ok,
            hash,
            ..
        } = line
        {
            let data = corpus_file(&file);
            let (o, cpp) = oracle.decode_with_hashes(&Request::new(&data, reduce, mode));
            assert_eq!(o.ok, ok, "{file} {mode} r{reduce}");
            assert_eq!(
                outcome_hash(&o),
                if o.ok { cpp.hash } else { 0 },
                "{file} {mode} r{reduce}"
            );
            assert_eq!(whole_value(&o, mode), hash, "{file} {mode} r{reduce}");
            lines += 1;
        }
    }
    assert!(lines >= 700, "{lines} whole-file lines");
}

/// neo-j2k through its compatibility API gives the oracle's outcome on a file with no departure.
#[test]
fn backends_agree_on_a_corpus_file() {
    let data = corpus_file("sl_64_rgb_97.j2k");
    let mut a = OracleBackend::new(BackendKind::Oracle);
    let mut b = OracleBackend::new(BackendKind::NeoJ2k);
    let mut c = OracleBackend::new(BackendKind::NeoJ2kMemory);
    for mode in [Mode::Plain, Mode::Viewer] {
        let x = a.decode_with_hashes(&Request::new(&data, 1, mode));
        let y = b.decode_with_hashes(&Request::new(&data, 1, mode));
        assert!(x.0.ok);
        assert_eq!(x.1, y.1, "{mode}");
        assert_eq!(whole_value(&x.0, mode), whole_value(&y.0, mode), "{mode}");
    }
    // the memory backend writes the viewer's 8-bit image straight into its buffer: no planes, its
    // 8-bit image's hash only
    for reduce in 0..=5 {
        let x = a.decode_with_hashes(&Request::new(&data, reduce, Mode::Viewer));
        let z = c.decode_with_hashes(&Request::new(&data, reduce, Mode::Viewer));
        assert_eq!(x.0.ok, z.0.ok, "r{reduce}");
        assert_eq!(viewer_u8_hash(&x.0), z.1.u8_hash, "r{reduce}");
        assert!(z.0.planes.is_empty());
    }
}

#[test]
fn viewer_rules_equal_the_cpp() {
    use neo_j2k_testkit::viewer as rs;
    let sides = [
        -1,
        0,
        1,
        2,
        3,
        63,
        64,
        65,
        100,
        127,
        128,
        129,
        1000,
        1024,
        2047,
        2048,
        4096,
        16384,
        65535,
        1 << 20,
    ];
    for &w in &sides {
        for &h in &sides {
            for discard in -1..=8 {
                assert_eq!(
                    rs::data_size_j2c(w, h, discard),
                    viewer::data_size_j2c(w, h, discard),
                    "{w} {h} {discard}"
                );
            }
        }
    }
    // every side the viewer can store, at discard 0 and 5 (the float log2 path)
    for side in 1..=70_000 {
        for discard in [0, 5] {
            assert_eq!(
                rs::data_size_j2c(side, 1, discard),
                viewer::data_size_j2c(side, 1, discard),
                "{side}"
            );
        }
    }
    for tiles in [0u64, 1, 2, 7, 1 << 20, 1 << 32, u64::MAX] {
        for size in [0u32, 1, 64, 1024, 0x8000_0000, u32::MAX] {
            assert_eq!(
                rs::grid_side(tiles, size),
                viewer::grid_side(tiles, size),
                "{tiles} {size}"
            );
        }
    }
    for c in 0..=10 {
        assert_eq!(rs::components_supported(c), viewer::components_supported(c));
    }
    for requested in -1..=32 {
        for res in -1..=33 {
            for &(w, h) in &[(1, 1), (64, 64), (1024, 768), (4096, 4096), (-5, 7), (i32::MAX, 3)] {
                for max in [4096, 2048, 1, 0] {
                    assert_eq!(
                        rs::reduce_factor(requested, res, w, h, max),
                        viewer::reduce_factor(requested, res, w, h, max),
                        "{requested} {res} {w} {h} {max}"
                    );
                }
            }
        }
    }
    let big = [
        0u64,
        1,
        2,
        16383,
        16384,
        16385,
        32768,
        1 << 31,
        1 << 40,
        u64::MAX - 1,
        u64::MAX,
    ];
    for &w in &big {
        for &h in &big {
            for c in [0u64, 1, 3, 4, 8, 9] {
                for r in [0u32, 1, 5, 31, 32] {
                    assert_eq!(
                        rs::decode_allowed(w, h, c, r),
                        viewer::decode_allowed(w, h, c, r),
                        "{w} {h} {c} {r}"
                    );
                }
            }
        }
    }
}

fn scan_inputs() -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    let root = repo_root();
    for dir in [
        corpus_dir(),
        root.join("fuzz/seeds"),
        root.join("fuzz/regressions"),
        root.join("fuzz/bombs"),
    ] {
        for p in expand_inputs(&dir, true) {
            out.push((base_name(&p), std::fs::read(&p).unwrap()));
        }
        // the decode fuzz target's inputs, less their four option bytes (fuzz/fuzz_common.h)
        for e in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let p = e.path();
            if p.extension().is_some_and(|x| x == "bin") {
                let b = std::fs::read(&p).unwrap();
                if b.len() > 4 && b[..2] != [0xFF, 0x4F] {
                    out.push((base_name(&p), b[4..].to_vec()));
                }
            }
        }
    }
    if let Some(data) = testdata_dir() {
        for p in expand_inputs(&data.join("input/conformance"), false) {
            out.push((base_name(&p), std::fs::read(&p).unwrap()));
        }
    }
    out
}

/// The scanner's summary equals the C++'s on every corpus file, fuzz input and (with the test
/// data) conformance codestream, and on every prefix of the inputs up to 4 KB.
#[test]
fn scanner_equals_the_cpp() {
    let mut checked = 0usize;
    for (name, data) in scan_inputs() {
        assert_eq!(scan::summary(&data), neo_j2k_oracle::scan_summary(&data), "{name}");
        checked += 1;
        if data.len() <= 4096 {
            for n in 0..data.len() {
                let (a, b) = (scan::summary(&data[..n]), neo_j2k_oracle::scan_summary(&data[..n]));
                assert_eq!(a, b, "{name}, first {n} bytes");
                checked += 1;
            }
        }
    }
    assert!(checked > 10_000, "{checked} scans");
}

/// The PGX reader equals the C++'s on written files (every depth, both byte orders and signs, and
/// broken headers) and, with the test data, on the conformance reference images.
#[test]
fn pgx_reader_equals_the_cpp() {
    let dir = test_tmp_dir().join(format!("nj2k-rust-pgx-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut files: Vec<PathBuf> = Vec::new();
    let mut k = 0;
    let mut write = |bytes: Vec<u8>| {
        let p = dir.join(format!("{k}.pgx"));
        k += 1;
        std::fs::write(&p, bytes).unwrap();
        files.push(p);
    };
    for depth in [1, 7, 8, 9, 12, 16, 17, 24, 31, 32] {
        for order in ["ML", "LM"] {
            for sign in ["", "+", "-"] {
                let per = if depth <= 8 {
                    1
                } else if depth <= 16 {
                    2
                } else {
                    4
                };
                let mut b = format!("PG {order} {sign}{depth} 3 2\n").into_bytes();
                b.extend((0..6 * per).map(|i| (i * 37 + depth * 11_u32).to_le_bytes()[0] ^ 0x80));
                write(b);
            }
        }
    }
    for broken in [
        &b"PG"[..],
        b"P5 3 2 255\n",
        b"PG ML 8 3 2",
        b"PG MX 8 1 1 \x00",
        b"PG ML 0 1 1 \x00",
        b"PG ML 8 0 1 \x00",
        b"PG ML 8 1 1x\x00",
        b"PG ML 8 4 4 \x00\x01",
        b"PG\tLM\t-16\t1\t1\r\xff\x7f",
        b"PG ML 8 1234567890 1 \x00",
    ] {
        write(broken.to_vec());
    }
    if let Some(data) = testdata_dir() {
        let refs = data.join("input/conformance");
        for e in std::fs::read_dir(&refs).into_iter().flatten().flatten() {
            if e.path().extension().is_some_and(|x| x == "pgx") {
                files.push(e.path());
            }
        }
    }
    for p in &files {
        let ours = pgx::read_pgx(p).map_err(str::to_owned);
        assert_eq!(ours, neo_j2k_oracle::read_pgx(p), "{}", p.display());
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_size_table_holds_the_arena_types() {
    let t = layout_table();
    assert!(t.iter().any(|r| r.name == "CodeBlock") && t.iter().any(|r| r.name == "EncTagNode"));
    assert!(t.iter().all(|r| r.size > 0 && r.align.is_power_of_two()));
}

#[test]
fn kernels_run() {
    let nan = f32::NAN.to_bits().cast_signed();
    let inf = f32::INFINITY.to_bits().cast_signed();
    let mut row = [
        nan,
        inf,
        1.5f32.to_bits().cast_signed(),
        2.5f32.to_bits().cast_signed(),
        (-300.0f32).to_bits().cast_signed(),
    ];
    kernels::dc_shift_97_scalar(&mut row, 128, 0, 255);
    // NaN gives lo, +inf hi, ties to even, then the shift and the clamp
    assert_eq!(row, [0, 255, 130, 130, 0]);
    let mut row = [-200, 0, 127, 1000];
    kernels::dc_shift_53_scalar(&mut row, 128, 0, 255);
    assert_eq!(row, [0, 128, 255, 255]);
    let [r, g, b] = kernels::rct_scalar([&[10, 0], &[0, 5], &[0, -5]]);
    assert_eq!((r.len(), g.len(), b.len()), (2, 2, 2));
    let f = |v: f32| v.to_bits().cast_signed();
    let [r, _, _] = kernels::ict_scalar([&[f(100.0)], &[f(0.0)], &[f(0.0)]]);
    assert_eq!(f32::from_bits(r[0].cast_unsigned()), 100.0);
}

#[test]
fn a_codec_decodes_and_reports_its_arenas() {
    let data = corpus_file("sl_64_rgb_97.j2k");
    let mut codec = Codec::new(Format::J2k, 0, 0).expect("a codec");
    assert!(codec.set_parallel_thresholds(0, 0, 0, 0));
    let (st, samples) = codec.decode_i32(&data);
    assert_eq!(st, 0, "NJ2K_OK");
    assert_eq!(samples.len(), 64 * 64 * 3);
    let l = codec.ledger();
    assert!(
        l.header_reserved > 0 && l.work_reserved > 0 && l.kept_bytes > 0,
        "{l:?}"
    );
    // the decode's trim resets the work arena's mark: stage R2 needs a hook to read it (README)
    assert_eq!(l.work_high_water, 0);
    // the same bytes decode the same on a codec that warmed up
    let (st2, again) = codec.decode_i32(&data);
    assert_eq!((st2, &again), (0, &samples));
    // a limit far below the decode's needs refuses it (NJ2K_E_LIMIT)
    let mut small = Codec::new(Format::J2k, 0, 4096).expect("a codec");
    assert_eq!(small.decode_i32(&data).0, -6);
}

#[test]
fn decode_i32_rejects_oversized_header_before_allocating_output() {
    let mut data = corpus_file("sl_64_rgb_97.j2k");
    let siz = data
        .windows(2)
        .position(|marker| marker == [0xff, 0x51])
        .expect("the fixture has a SIZ marker");
    // Xsiz is six bytes after the SIZ marker. This is the shape found by the differential fuzzer:
    // the header is parseable, but its decoded width is far above the configured limit.
    data[siz + 6..siz + 10].copy_from_slice(&u32::MAX.to_be_bytes());

    let mut codec = Codec::with_options(
        Format::J2k,
        0,
        neo_j2k_oracle::ReducePolicy::Exact,
        neo_j2k_oracle::Limits {
            max_tiles: 256,
            max_components: 8,
            max_layers: 64,
            max_decoded_side: 1_024,
            max_memory: 32 << 20,
        },
        0,
        false,
    )
    .expect("a bounded codec");
    let (status, samples) = codec.decode_i32(&data);
    assert_eq!(status, -4, "NJ2K_E_UNSUPPORTED");
    assert!(samples.is_empty());
}
