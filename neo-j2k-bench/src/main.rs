// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! One measurement of the Rust twin's Tier-1: TSC cycles of the decoder's and the encoder's code-block entry
//! points, on one pinned logical CPU. One run of this program is one round of one build; `tools/bench_tier1.py`
//! builds the base, the head and a copy of the base, runs them alternately and interleaved, and writes the table
//! (`doc/bench.md`).
//!
//! ```text
//! neo-j2k-bench [--pin CPU] [--dumps DIR] [--min-mcycles N] [--units N] [--blocks N] [--digest-only]
//! ```
//!
//! Every set prints one line,
//! `RESULT set=<name> blocks=<n> inner=<passes in a unit> per_pass=<clock units> digest=<hash of the output>`,
//! after a header line. `--dumps` is the folder of the six decoder dumps (default: `tests/vectors/t1_bench` of
//! `NJ2K_CPP_SOURCE`); `--digest-only` prints the digests without timing anything.

mod clock;
mod pin;
mod sets;

use std::path::PathBuf;
use std::process::ExitCode;

struct Options {
    pin: Option<usize>,
    dumps: Option<PathBuf>,
    min_mcycles: f64,
    units: usize,
    blocks: u32,
    digest_only: bool,
}

fn parse(args: &[String]) -> Result<Options, String> {
    let default_pin = std::thread::available_parallelism().map_or(0, |n| if n.get() >= 4 { 2 } else { 0 });
    let mut options = Options {
        pin: Some(default_pin),
        dumps: None,
        min_mcycles: 30.0,
        units: 3,
        blocks: 256,
        digest_only: false,
    };
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        let mut value = |name: &str| it.next().ok_or_else(|| format!("{name} needs a value"));
        match arg.as_str() {
            "--pin" => {
                let v = value("--pin")?;
                options.pin = if v == "none" {
                    None
                } else {
                    Some(
                        v.parse()
                            .map_err(|_| format!("--pin takes a CPU number or none, not {v}"))?,
                    )
                };
            }
            "--dumps" => options.dumps = Some(PathBuf::from(value("--dumps")?)),
            "--min-mcycles" => {
                let v = value("--min-mcycles")?;
                options.min_mcycles = v
                    .parse()
                    .map_err(|_| format!("--min-mcycles takes a number, not {v}"))?;
            }
            "--units" => {
                let v = value("--units")?;
                options.units = v.parse().map_err(|_| format!("--units takes a count, not {v}"))?;
            }
            "--blocks" => {
                let v = value("--blocks")?;
                options.blocks = v.parse().map_err(|_| format!("--blocks takes a count, not {v}"))?;
            }
            "--digest-only" => options.digest_only = true,
            other => return Err(format!("unknown argument {other}")),
        }
    }
    if options.units == 0 || options.blocks == 0 || options.min_mcycles < 0.0 {
        return Err("--units and --blocks take 1 or more, --min-mcycles 0 or more".to_owned());
    }
    Ok(options)
}

fn run(options: &Options) -> Result<(), String> {
    let dumps = options.dumps.clone().map_or_else(
        || {
            std::env::var_os("NJ2K_CPP_SOURCE")
                .map(|d| PathBuf::from(d).join("tests/vectors/t1_bench"))
                .ok_or_else(|| "set NJ2K_CPP_SOURCE (the C++ neo-j2k checkout) or pass --dumps DIR".to_owned())
        },
        Ok,
    )?;
    let mut all = sets::decode_sets(&dumps)?;
    all.extend(sets::encode_sets(options.blocks));
    if let Some(cpu) = options.pin
        && !options.digest_only
        && let Err(why) = pin::pin_to_cpu(cpu)
    {
        eprintln!("neo-j2k-bench: not pinned: {why}");
    }
    println!(
        "neo-j2k-bench {}: clock {}, pin {}, units {}, min {} Mcycles, build {}",
        neo_j2k::version(),
        clock::UNIT,
        options.pin.map_or_else(|| "none".to_owned(), |c| c.to_string()),
        options.units,
        options.min_mcycles,
        if cfg!(target_feature = "avx2") {
            "x86-64-v3"
        } else {
            "baseline"
        }
    );
    for set in &mut all {
        let digest = set.digest()?;
        if options.digest_only {
            println!("DIGEST set={} blocks={} digest={digest:016x}", set.name(), set.blocks());
            continue;
        }
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "a positive count of cycles"
        )]
        let min_units = (options.min_mcycles * 1e6) as u64;
        let m = sets::measure(set, min_units, options.units);
        println!(
            "RESULT set={} blocks={} inner={} per_pass={} digest={digest:016x}",
            set.name(),
            set.blocks(),
            m.inner,
            m.per_pass
        );
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let options = match parse(&args) {
        Ok(o) => o,
        Err(why) => {
            eprintln!("neo-j2k-bench: {why}");
            eprintln!(
                "usage: neo-j2k-bench [--pin CPU|none] [--dumps DIR] [--min-mcycles N] [--units N] [--blocks N] [--digest-only]"
            );
            return ExitCode::from(2);
        }
    };
    match run(&options) {
        Ok(()) => ExitCode::SUCCESS,
        Err(why) => {
            eprintln!("neo-j2k-bench: {why}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn arguments_parse_and_bad_ones_are_refused() {
        let o = parse(&args(&[
            "--pin",
            "3",
            "--units",
            "5",
            "--min-mcycles",
            "2.5",
            "--digest-only",
        ]))
        .unwrap();
        assert_eq!((o.pin, o.units, o.digest_only), (Some(3), 5, true));
        assert!((o.min_mcycles - 2.5).abs() < f64::EPSILON);
        assert_eq!(parse(&args(&["--pin", "none"])).unwrap().pin, None);
        for bad in [
            &["--pin"][..],
            &["--units", "0"],
            &["--units", "x"],
            &["--frobnicate"],
            &["--min-mcycles", "-1"],
        ] {
            assert!(parse(&args(bad)).is_err(), "{bad:?}");
        }
    }
}
