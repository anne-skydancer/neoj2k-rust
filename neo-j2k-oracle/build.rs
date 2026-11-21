// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)

//! Finds or builds the C++ oracle and links it (rust/README.md, "Tests and the C++ oracle"):
//!
//! 1. `NJ2K_ORACLE_DIR`: an existing CMake build folder of this repository configured with
//!    `-DNJ2K_BUILD_RUST_SHIM=ON`, whose libraries are linked as they are (built beforehand);
//! 2. otherwise CMake configures and builds the C++ into `OUT_DIR` with the preset
//!    `NJ2K_ORACLE_PRESET` (default `windows-msvc` on Windows, `linux-gcc` elsewhere): the static
//!    library, the OpenJPEG oracle, the harness and the shim only.
//!
//! Either way the build folder's link manifest (`nj2k-rust-shim-<config>.txt`, written by
//! `rust/neo-j2k-oracle/shim/CMakeLists.txt`; config `NJ2K_ORACLE_CONFIG`, default
//! `RelWithDebInfo`) names the libraries in link order. `NJ2K_ORACLE_NO_BUILD=1` skips all of it
//! for commands that compile without linking (clippy, doc), and so does Miri, whose tests never
//! call the oracle; a linking build then fails with the oracle's symbols missing.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn var(name: &str) -> Option<String> {
    println!("cargo::rerun-if-env-changed={name}");
    env::var(name).ok().filter(|v| !v.is_empty())
}

fn run(cmd: &mut Command) {
    eprintln!("neo-j2k-oracle: running {cmd:?}");
    let status = cmd
        .status()
        .unwrap_or_else(|e| panic!("neo-j2k-oracle: cannot run {cmd:?}: {e}"));
    assert!(status.success(), "neo-j2k-oracle: {cmd:?} failed ({status})");
}

fn build_oracle(repo: &Path, preset: &str, config: &str) -> PathBuf {
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let dir = out.join(format!("cmake-{preset}"));
    // A change to the C++ re-runs this script; CMake then rebuilds what changed.
    for watched in [
        "CMakeLists.txt",
        "CMakePresets.json",
        "cmake",
        "include",
        "src",
        "oracle",
        "tests/common",
        "rust/neo-j2k-oracle/shim",
    ] {
        println!("cargo::rerun-if-changed={}", repo.join(watched).display());
    }
    run(Command::new("cmake")
        .arg("-S")
        .arg(repo)
        .arg("-B")
        .arg(&dir)
        .arg("--preset")
        .arg(preset)
        .args([
            "-DNJ2K_BUILD_RUST_SHIM=ON",
            "-DNJ2K_BUILD_TESTS=OFF",
            "-DNJ2K_BUILD_TOOLS=OFF",
            "-DNJ2K_BUILD_BENCH=OFF",
            "-DNJ2K_BUILD_FUZZERS=OFF",
            "-DNJ2K_BUILD_SHARED=OFF",
            "-DNJ2K_BUILD_STATIC=ON",
            "-DNJ2K_INSTALL=OFF",
        ]));
    run(Command::new("cmake").arg("--build").arg(&dir).args([
        "--config",
        config,
        "--target",
        "nj2k_rust_shim",
        "--parallel",
    ]));
    dir
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let no_build = var("NJ2K_ORACLE_NO_BUILD").is_some_and(|v| v != "0");
    if no_build || env::var_os("CARGO_CFG_MIRI").is_some() {
        return;
    }
    let repo = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR")).join("../..");
    let config = var("NJ2K_ORACLE_CONFIG").unwrap_or_else(|| "RelWithDebInfo".to_owned());
    let dir = if let Some(dir) = var("NJ2K_ORACLE_DIR") {
        PathBuf::from(dir)
    } else {
        let windows = env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "windows");
        let preset =
            var("NJ2K_ORACLE_PRESET").unwrap_or_else(|| if windows { "windows-msvc" } else { "linux-gcc" }.to_owned());
        build_oracle(&repo, &preset, &config)
    };
    let manifest = dir.join(format!("nj2k-rust-shim-{config}.txt"));
    println!("cargo::rerun-if-changed={}", manifest.display());
    let text = std::fs::read_to_string(&manifest).unwrap_or_else(|e| {
        panic!(
            "neo-j2k-oracle: no link manifest {} ({e}); configure that build folder with -DNJ2K_BUILD_RUST_SHIM=ON \
             and build the target nj2k_rust_shim in {config}",
            manifest.display()
        )
    });
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(path) = line.strip_prefix("lib ") {
            let path = Path::new(path);
            assert!(path.is_file(), "neo-j2k-oracle: {} is not built", path.display());
            println!("cargo::rerun-if-changed={}", path.display());
            let folder = path.parent().expect("a library's folder");
            let stem = path.file_stem().expect("a library's name").to_string_lossy();
            // libname.a on Unix, name.lib with MSVC
            let unix_archive = path.extension().is_some_and(|e| e == "a");
            let name = if unix_archive {
                stem.strip_prefix("lib").unwrap_or(&stem)
            } else {
                &stem[..]
            };
            println!("cargo::rustc-link-search=native={}", folder.display());
            println!("cargo::rustc-link-lib=static={name}");
        } else if let Some(name) = line.strip_prefix("sys ") {
            println!("cargo::rustc-link-lib={name}");
        } else {
            panic!("neo-j2k-oracle: unknown line '{line}' in {}", manifest.display());
        }
    }
}
