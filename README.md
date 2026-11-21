<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# neo-j2k in Rust

The native Rust twin of neo-j2k: a transcription of the C++ library in this repository, module for
module, bit-identical to it. Same samples, statuses, messages and encoded bytes for the same inputs,
on every executor and at both CPU levels. The C++ is the oracle: the differential tests build and link
it, and nothing else does.

**Native Rust.** The crate `neo-j2k` has no build script, no C or C++, no `-sys` crate and no FFI, in
itself or in its dependency tree; it has no runtime dependency at all. `unsafe` is allowed only in its
`simd` module (stage R3). The C++ appears only in the test-only crate `neo-j2k-oracle`, a
dev-dependency that is never published. CI proves both on every pull request
(`tools/check_rust_native.py`, below).

Paths below are relative to the repository root.

## State: stage R0, the scaffolding

| Crate | What | Published |
|---|---|---|
| `neo-j2k/` | the twin; in R0 the C++ size table its byte ledger will charge (`src/mem/layout.rs`) | later, on the owner's approval |
| `neo-j2k-testkit/` | the Rust side of the C++ harness in `tests/common`: requests and outcomes, the viewer's rules, `nj2k_diff`'s digests and reference tables, the codestream scanner, the PGX reader, a test pool executor. Pure Rust, no dependency | never |
| `neo-j2k-oracle/` | the C++ neo-j2k, OpenJPEG at the oracle's pin and the C++ harness, built by CMake and linked through the test-only shim (`neo-j2k-oracle/shim`), behind a safe interface | never |

The header, the decoder, the SIMD kernels, threads and the encoder follow in stages R1 to R5, the
fuzz targets and benches in R6 (`doc/stages.md`).

## Toolchain

The workspace pins its compiler in `rust/rust-toolchain.toml`:

```toml
[toolchain]
channel = "1.100.0"
components = ["rustfmt", "clippy"]
profile = "minimal"
```

Edition 2024, `rust-version = "1.100"`. In `rust/`, `rustup toolchain install` installs it.

Only Miri and, from R6, the fuzzers use a nightly, pinned by date in `rust/nightly-toolchain`
(`nightly-2026-11-03`, rustc 1.101.0-nightly 1c3a65069, Miri available for Linux and Windows); never
the library or its tests. The fuzz targets will use cargo-fuzz 0.13.2.

```
rustup toolchain install nightly-2026-11-03 --profile minimal --component miri,rust-src
```

## The library

```
cd rust
cargo build --release -p neo-j2k                                                    # decode + encode, std, simd, opensim
cargo build --release -p neo-j2k --no-default-features --features std               # decoder only, scalar, no OpenSim profile
cargo build --release -p neo-j2k --no-default-features --features std,encode,simd   # Vulkanstorm's set (Second Life only)
```

| Feature | Default | What it does |
|---|---|---|
| `std` | yes | required in this version (reserved for a later `no_std` + `alloc` build) |
| `encode` | yes | the encoder (stage R5); without it `Encoder::new` returns `Error::Unsupported` |
| `simd` | yes | the AVX2 kernels when the target has AVX2 (stage R3); without it, or on a target without AVX2, the scalar kernels |
| `opensim` | yes | the OpenSim limit profile (stage R1), as the C++'s `NJ2K_PROFILE_OPENSIM` |

The workspace's release profile (`lto = "fat"`, `codegen-units = 1`, `panic = "unwind"`) applies to
its tests, tools and benches; a program that depends on the crate uses its own profile.

## Target CPU

The kernels are chosen at compile time, as in the C++ (decision D8):

| Build | How | Kernels |
|---|---|---|
| baseline x86-64 | the default | scalar |
| x86-64-v3 (AVX2, the viewer's floor) | `RUSTFLAGS="-C target-cpu=x86-64-v3"`, or in the consumer's `.cargo/config.toml`: `[build] rustflags = ["-C", "target-cpu=x86-64-v3"]` | AVX2 in `mod simd` |

Both builds give identical results, bit for bit, and identical to the C++. No FMA is ever used: Rust
never fuses on its own, `f32::mul_add` and `f64::mul_add` are denied (`rust/clippy.toml`), and no FMA
intrinsic is called. Use separate target folders so the two builds do not rebuild each other:

```
cargo test --release --workspace                                                    # baseline
RUSTFLAGS="-C target-cpu=x86-64-v3" CARGO_TARGET_DIR=target-v3 cargo test --release --workspace
```

(PowerShell: `$env:RUSTFLAGS = "-C target-cpu=x86-64-v3"; $env:CARGO_TARGET_DIR = "target-v3"`.)

## Tests and the C++ oracle

The tests that compare with the C++ link it through `neo-j2k-oracle`. Its `build.rs` finds the C++ in
this order:

1. `NJ2K_ORACLE_DIR`: an existing CMake build folder of this repository configured with
   `-DNJ2K_BUILD_RUST_SHIM=ON` (off by default), with the target `nj2k_rust_shim` built. The build
   folder's link manifest, `nj2k-rust-shim-<config>.txt`, names the libraries (`nj2k_rust_shim`,
   `nj2k_test_common`, `nj2k_oracle`, `neoj2k_static`) and, on Linux, the C++ runtime:

   ```
   cmake --preset windows-msvc -DNJ2K_BUILD_RUST_SHIM=ON          # or linux-gcc; the presets' flags:
   cmake --build --preset windows-msvc --target nj2k_rust_shim    # /fp:precise /arch:AVX2; -ffp-contract=off -fno-fast-math
   cd rust
   NJ2K_ORACLE_DIR=../build/windows-msvc cargo test --release --workspace
   ```

2. Otherwise CMake configures and builds the C++ into the build script's `OUT_DIR`
   (`cmake-<preset>`), with the preset `NJ2K_ORACLE_PRESET` (default `windows-msvc` on Windows,
   `linux-gcc` elsewhere) and only the static library, the oracle, the harness and the shim
   (`-DNJ2K_BUILD_TESTS=OFF -DNJ2K_BUILD_TOOLS=OFF -DNJ2K_BUILD_BENCH=OFF -DNJ2K_BUILD_SHARED=OFF
   -DNJ2K_BUILD_RUST_SHIM=ON`). A change under `src/`, `include/`, `oracle/`, `tests/common/`,
   `cmake/` or the shim runs CMake again; it rebuilds what changed.

The C++ is always built in `RelWithDebInfo` (`NJ2K_ORACLE_CONFIG`), with the dynamic C runtime
(`/MD`) that Rust's MSVC target uses.

```
cd rust
cargo test --release --workspace                                    # every test, the differential ones included
cargo test --release -p neo-j2k-testkit                             # the test kit alone: no C++
cargo test --release -p neo-j2k-oracle --test differential          # the test kit against the C++ harness, R0's gates
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check
```

| Variable | Use |
|---|---|
| `NJ2K_ORACLE_DIR` | a CMake build folder holding the oracle libraries (above) |
| `NJ2K_ORACLE_PRESET` | the CMake preset `build.rs` uses when it builds the oracle itself |
| `NJ2K_ORACLE_CONFIG` | the configuration of the oracle's build (default `RelWithDebInfo`) |
| `NJ2K_ORACLE_NO_BUILD` | `1`: no C++ build and no link, for commands that compile without linking (`cargo clippy`, `cargo doc`, `cargo metadata`); a test build then fails to link |
| `NJ2K_TESTDATA` | the fetched conformance data (`python -I tools/fetch_testdata.py`); the tests that use it (the scanner and PGX differentials on the conformance files) cover it only when it is set, as in CTest |
| `NJ2K_TEST_TMP` | the folder for the tests' temporary files, e.g. the repository's Defender-excluded `.fixture-tmp`; also set `TEMP`/`TMP` (Windows) or `TMPDIR` (Linux) to it for the C++ |

The tests never open a dialog: panics and assertion output go to stderr, and the oracle sends the C
runtime's reports to the console (`console_reports()`) before its first call.

### What R0's tests prove

- **The size table** (`neo-j2k/src/mem/layout.rs`): the twin's copy of `sizeof` and `alignof` of the
  35 C++ types the arenas hold equals the C++ compiler's, row for row, on every CI platform (MSVC,
  GCC 15, GCC 14). Types defined inside a `.cpp` file (`decode.cpp`'s `TileWalk`, `t2.cpp`'s
  `PiResolution` and `PiComp`, `encode.cpp`'s `T1Job`) cannot be measured from outside it; the stage
  that transcribes their unit adds them with a C++ accessor.
- **The digests** (`neo-j2k-testkit/src/digest.rs`): three named lines of
  `corpus/generated/oracle-reference.tsv` (`cblk_32x16_97.j2k` whole in plain and viewer mode, and
  its cut digest at step 13), then every whole-file line of the table (every corpus file in plain and
  viewer mode at reduce 0-5), recomputed in Rust from the oracle's planes, and each hash equal to the
  C++ harness's own.
- **The harness's rules and readers**: the viewer's rules, the scanner (every corpus file, fuzz seed,
  regression and bomb, and every prefix of those up to 4 KB) and the PGX reader give the C++'s
  results.

### The C++ harness's FNV basis

The harness's `Fnv64` starts from 1469598103934665603, FNV-1a's published offset basis
(14695981039346656037) with its last digit dropped. Every reference table and digest is built on it,
so the test kit keeps it; it is not a published FNV-1a hash.

## The native-Rust check

```
python -I tools/check_rust_native.py
```

From `cargo metadata` and `cargo package --list` it checks that `neo-j2k`'s normal and build
dependency graph (dev-dependencies left out) holds no crate with a build script, a `links` key, a
`-sys` name or a C/C++ build or binding helper, and not the oracle crate; that the crate itself has
no build script; that the package holds Rust sources, manifests, readme and licence files only; and
that the crate's sources hold no FFI (`extern "C"`, `#[link]`, `#[no_mangle]`, `#[export_name]`), no
`unsafe` outside `src/simd/`, no `static mut` and no `thread_local!`. `cargo deny check` (with
`rust/deny.toml`, cargo-deny 0.20.2 in CI) checks the licences, advisories and sources of the whole
workspace's graph and bans the C/C++ helper crates outright.

## Miri

Miri runs the crates' own unit tests; they never call the oracle (`cfg(not(miri))` keeps the
differential tests out, and the oracle's build script does nothing under Miri):

```
cd rust
cargo +nightly-2026-11-03 miri setup
NJ2K_ORACLE_NO_BUILD=1 cargo +nightly-2026-11-03 miri test -p neo-j2k-testkit -p neo-j2k --lib
```

From stage R3 the `simd` module's tests also run with `RUSTFLAGS="-C target-feature=+avx2"`.

## Windows and Linux

| | Windows | Linux |
|---|---|---|
| Rust target | `x86_64-pc-windows-msvc` | `x86_64-unknown-linux-gnu` |
| C++ for the oracle | Visual Studio 2026 (MSVC 14.5x) and CMake 4.2+ (the `windows-msvc` preset's generator); no developer prompt needed | GCC 15 (`CC=gcc-15 CXX=g++-15`), or Clang with `linux-clang`; Ninja; the LTS job: Ubuntu 24.04 with GCC 14 |
| C runtime | the dynamic CRT (`/MD`), as Rust's MSVC target; never `+crt-static` with the oracle | glibc; the link manifest adds `stdc++`, `m`, `pthread`, `dl` |
| Temporary files | `TEMP`/`TMP` and `NJ2K_TEST_TMP` | `TMPDIR` and `NJ2K_TEST_TMP` |
| Miri | works (it interprets; no oracle) | the CI job runs here |

CI (`.github/workflows/rust.yml`) runs the lint job (fmt, clippy at both CPU levels, docs,
cargo-deny, the native-Rust check), Windows (MSVC), Linux (GCC 15), Ubuntu 24.04 (GCC 14, the oracle
built by `build.rs` itself) each at both CPU levels, and Miri.
