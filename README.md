<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# neo-j2k in Rust

A JPEG 2000 codec library written natively in Rust: Part 1 decoding of J2K, JP2 and JPH, and encoding
with lossless 5/3 and irreversible 9/7 wavelets. It is the Rust counterpart of the C++ library
[neo-j2k](https://github.com/anne-skydancer/neo-j2k) and implements the same core codec. It is not a
wrapper: the crate `neo-j2k` has no build script, no C or C++, no `-sys` crate and no FFI, in itself or in
its dependency tree; it has no runtime dependency at all. `unsafe` is allowed only in its `simd` module.
The C++ library is the crate's oracle: the tests compare this crate's output against it, and the port
targets the same samples, statuses, messages and encoded bytes for the same inputs, on every executor and
at both CPU levels. The C++ appears only in the test-only crate `neo-j2k-oracle`, a dev-dependency that is
never published. The native-Rust check below verifies this directly.

The decoder's limits (`Limits`) bound the work and the memory a codestream can ask for; a limit set to zero
is unlimited. The encoder is a one-tile
encoder that the host can parallelize with a borrowed executor; the crate creates no private threads.

Licence: BSD-2-Clause ([License](#license)). Paths below are relative to the repository root.

## What it covers

| Crate | What | Published |
|---|---|---|
| `neo-j2k/` | native J2K and JP2/JPH decoding and the one-tile JPEG 2000 encoder | release crate |
| `neo-j2k-testkit/` | the Rust side of the C++ harness in `tests/common`: requests and outcomes, the host application's request rules, `nj2k_diff`'s digests and reference tables, the codestream scanner, the PGX reader, a test pool executor. Pure Rust, no dependency | never |
| `neo-j2k-bench/` | the Tier-1 benchmark: TSC cycles of the decoder's and the encoder's code-block entry points, driven by `tools/bench_tier1.py` ([doc/bench.md](doc/bench.md)). It holds the workspace's `unsafe` outside the codec's `simd` module (the cycle counter, thread affinity) and no dependency but the codec and the test kit | never |
| `neo-j2k-oracle/` | the C++ neo-j2k, OpenJPEG at the oracle's pin and the C++ harness, built by CMake and linked through the test-only shim (`neo-j2k-oracle/shim`), behind a safe interface | never |

`archive/rust/` preserves the final embedded Rust workspace moved from the C++ repository. It is a
historical snapshot, not a workspace member; the maintained and publishable crate is `neo-j2k/`.

The decoder supports the five progression orders, POC, packed packet headers, reduced and
layer-limited output, and partial inputs. It can return reusable planar samples or write
into planar or interleaved caller buffers. JP2 palette and channel metadata can be applied
to planar output through `Jp2Channels::Apply`.
`Decoder::decode_into_i32` can decode a matching single tile directly into tight, top-down
caller planes, one per selected component, without allocating another image-sized plane.

`Decoder::decode_from` reads from `std::io::Read`. It validates the header and working-memory
limits before buffering the tile data. Set `StreamOptions::length` to the advertised byte length
and `StreamOptions::seekable` when the source allows codestream look-ahead. The buffer ends at
the declared stream length or the end of the JP2/JPH codestream box, whichever comes first.
Use `Decoder::read_header_from` to inspect the header or select a reduce factor before resuming
from the same reader with `DecodeStream::decode` or `DecodeStream::decode_into`.

The encoder takes planar or interleaved unsigned 8-bit samples. It writes a J2K codestream
with one tile, RLCP progression and 64 × 64 code-blocks, using lossless 5/3 or irreversible
9/7 wavelets with optional quality layers. Use `Encoder::encode_to` to write the codestream to a
`std::io::Write`, with a caller-chosen buffer size and support for short writes. A host executor can
parallelize component wavelets and Tier-1 code-block encoding; the crate creates no private threads.

## Build and use

### Toolchain

The workspace pins its compiler in `rust-toolchain.toml`:

```toml
[toolchain]
channel = "1.100.0"
components = ["rustfmt", "clippy"]
profile = "minimal"
```

Edition 2024, `rust-version = "1.100"`. From the workspace root, `rustup toolchain install` installs it.

Only Miri and the fuzzers use a nightly, pinned by date in `nightly-toolchain`
(`nightly-2026-11-03`, rustc 1.101.0-nightly 1c3a65069, Miri available for Linux and Windows); never
the library or its tests. The fuzz workspace uses cargo-fuzz 0.13.2.

```
rustup toolchain install nightly-2026-11-03 --profile minimal --component miri,rust-src
```

### Using the library

```rust
use neo_j2k::{Decoder, DecoderConfig, Error, Limits, ReducePolicy};

fn dimensions(codestream: &[u8]) -> Result<(u32, u32), Error> {
    let mut decoder = Decoder::new(DecoderConfig {
        limits: Limits {
            max_tiles: 1024,
            max_decoded_side: 2048,
            max_memory: 256 << 20,
            ..Limits::default()
        },
        reduce_policy: ReducePolicy::Clamp,
        ..DecoderConfig::default()
    });
    decoder.read_header(codestream)?;
    let components = decoder.decoded_dimensions()?;
    Ok((components[0].width, components[0].height))
}
```

The decoder reads up to the first tile-part and caches the validated header prefix. Use
`Format::Jp2` for JP2 and JPH containers. An incomplete header reports `ErrorKind::NeedData`
so the caller can retry with more bytes.

For a `Read` input, use `Decoder::read_header_from` with `StreamOptions` to obtain a
`DecodeStream`. It keeps the reader positioned after the header and lets the caller inspect
dimensions or choose a reduce factor before decoding. Supply the byte length for a complete
decode or a JP2/JPH header read. `DecodeStream::decode_into` writes to caller buffers.

Use `decoded_dimensions_for` on `Decoder` or `DecodeStream` to check the working-memory estimate
for a component selection. It returns geometry for every codestream component. Pass the selection
again in `DecodeParams::components` when decoding.

After reading the header, `Decoder::codestream_coding` borrows the tile defaults and component
coding settings from the parsed header. Use `component(index)` to inspect the effective
code-block and precinct sizes, quantization, wavelet, and ROI shift for that codestream component.

```rust,no_run
use neo_j2k::{DecodeParams, Decoder, DecoderConfig, Error};

fn decode_first_component(data: &[u8]) -> Result<Vec<i32>, Error> {
    let mut decoder = Decoder::new(DecoderConfig::default());
    let image = decoder.decode(data, DecodeParams::default())?;
    Ok(image.components[0].samples().to_vec())
}
```

`Decoder::decode_into_i32` accepts tightly packed `I32Plane` buffers and can use them for
reconstruction of a single tile that matches the image. Those buffers may be partially modified
if decoding fails. `Decoder::decode_into` also accepts byte buffers with explicit row and
sample strides for U8, U16, and I32 output.

```rust,no_run
use neo_j2k::{EncodeParams, Encoder, Error, ImageSamples, ImageView};

fn encode_rgb(width: u32, height: u32, rgb: &[u8]) -> Result<Vec<u8>, Error> {
    let mut encoder = Encoder::new();
    let image = ImageView {
        width,
        height,
        flip_vertical: false,
        samples: ImageSamples::Interleaved {
            samples: rgb,
            components: 3,
            row_stride: width as usize * 3,
        },
    };
    Ok(encoder.encode(&image, &EncodeParams::default())?.to_vec())
}
```

Use `Encoder::encode_to` for a `Write` output. `encode_and_update` and `encode_to_and_update`
also return the effective layer ratios and codestream size cap through the caller's `EncodeParams`.

`EncodeParams::extra_options` accepts the core encoder's `PLT=NO`, `TLM=NO`, and `GUARD_BITS=2`.
The encoder checks these options after setup, before writing the main header.
Set `EncodeParams::executor` to a host scheduler to enable multi-core encoding. Images below
256 × 256 samples per component remain serial because dispatch overhead is not beneficial there.

`Decoder::set_limits` changes the resource limits for subsequent requests. The decoder checks the
header again under the new limits and can reuse its working buffers.
`Decoder::statistics` exposes the codec's budget and arena accounting. Use
`Decoder::set_retained_bytes` to cap the reusable native buffers that survive a decode and
`Decoder::retained_work_bytes` to read their live capacity. A host can lend its executor through
`DecodeParams::executor` for large tiles. Small requests stay serial.

`Limits` fields set to zero are unlimited, and `Limits::default()` has no limit. The crate also provides
named limit profiles; see [Example implementations](#example-implementations).

### Features

```
cargo build --release -p neo-j2k
cargo build --release -p neo-j2k --no-default-features --features std
```

| Feature | Default | What it does |
|---|---|---|
| `std` | yes | required in this version (reserved for a later `no_std` + `alloc` build) |
| `encode` | yes | the native JPEG 2000 encoder, including borrowed-executor parallelism |
| `simd` | yes | x86-64 instruction intrinsics, including the AVX2 kernels on AVX2 builds |
| `opensim` | yes | the OpenSim limit profile, as the C++'s `NJ2K_PROFILE_OPENSIM` |
| `bench-internals` | no | not API: the two Tier-1 entry points (decode a code-block, encode a code-block) that `neo-j2k-bench` times, hidden from the documentation; implies `encode` ([doc/bench.md](doc/bench.md)) |

The workspace's release profile (`lto = "fat"`, `codegen-units = 1`, `panic = "unwind"`) applies to
its tests, tools and benches; a program that depends on the crate uses its own profile.

### Target CPU

The crate selects kernels at compile time, as in the C++ (decision D8).

| Build | How | Kernels |
|---|---|---|
| baseline x86-64 | the default | scalar with SSE sample rounding |
| x86-64-v3 (AVX2) | `RUSTFLAGS="-C target-cpu=x86-64-v3"`, or in the consumer's `.cargo/config.toml`: `[build] rustflags = ["-C", "target-cpu=x86-64-v3"]` | AVX2 in `mod simd` |

The parity suite runs at both CPU levels. The port preserves the C++ floating-point operation order.
Calls into Rust, including jobs passed to a host executor, require the standard floating-point
control settings for the Rust target. On x86-64 this includes round-to-nearest-even and gradual
underflow.
`f32::mul_add` and `f64::mul_add` are denied (`clippy.toml`). Use separate target folders so the two
builds do not rebuild each other:

```
cargo test --release --workspace                                                    # baseline
RUSTFLAGS="-C target-cpu=x86-64-v3" CARGO_TARGET_DIR=target-v3 cargo test --release --workspace
```

(PowerShell: `$env:RUSTFLAGS = "-C target-cpu=x86-64-v3"; $env:CARGO_TARGET_DIR = "target-v3"`.)

### Windows and Linux

| | Windows | Linux |
|---|---|---|
| Rust target | `x86_64-pc-windows-msvc` | `x86_64-unknown-linux-gnu` |
| C++ for the oracle | Visual Studio 2026 (MSVC 14.5x) and CMake 4.2+ (the `windows-msvc` preset's generator); no developer prompt needed | GCC 15 (`CC=gcc-15 CXX=g++-15`), or Clang with `linux-clang`; Ninja; the LTS job: Ubuntu 24.04 with GCC 14 |
| C runtime | the dynamic CRT (`/MD`), as Rust's MSVC target; never `+crt-static` with the oracle | glibc; the link manifest adds `stdc++`, `m`, `pthread`, `dl` |
| Temporary files | `TEMP`/`TMP` and `NJ2K_TEST_TMP` | `TMPDIR` and `NJ2K_TEST_TMP` |
| Miri | works (it interprets; no oracle) | the CI job runs here |

CI (`.github/workflows/ci.yml`) runs formatting, Clippy, documentation, native-package checks, and the tests that do not need
the external C++ oracle. Differential tests use `NJ2K_ORACLE_DIR` or `NJ2K_CPP_SOURCE` as described
above. See the [Rust port design](doc/rust.md) for implementation notes.

## Correctness evidence

### Tests and the C++ oracle

The tests that compare with the C++ link it through `neo-j2k-oracle`. Its `build.rs` finds the C++ in
this order:

1. `NJ2K_ORACLE_DIR`: an existing CMake build folder of the C++
   [neo-j2k repository](https://github.com/anne-skydancer/neo-j2k), configured with
   `-DNJ2K_BUILD_RUST_SHIM=ON` (off by default) and this repository's
   `neo-j2k-oracle/shim` supplied as `NJ2K_RUST_SHIM_SOURCE`, with the target `nj2k_rust_shim` built. The build
   folder's link manifest, `nj2k-rust-shim-<config>.txt`, names the libraries (`nj2k_rust_shim`,
   `nj2k_test_common`, `nj2k_oracle`, `neoj2k_static`) and, on Linux, the C++ runtime:

   ```
   cmake --preset windows-msvc -DNJ2K_BUILD_RUST_SHIM=ON -DNJ2K_RUST_SHIM_SOURCE=/path/to/neoj2k-rust/neo-j2k-oracle/shim
   cmake --build --preset windows-msvc --target nj2k_rust_shim    # /fp:precise /arch:AVX2; -ffp-contract=off -fno-fast-math
   NJ2K_ORACLE_DIR=/path/to/neo-j2k/build/windows-msvc cargo test --release --workspace
   ```

2. Otherwise set `NJ2K_CPP_SOURCE` to a checkout of the C++ repository. CMake configures and
   builds that checkout into the build script's `OUT_DIR`
   (`cmake-<preset>`), with the preset `NJ2K_ORACLE_PRESET` (default `windows-msvc` on Windows,
   `linux-gcc` elsewhere) and only the static library, the oracle, the harness and the shim
   (`-DNJ2K_BUILD_TESTS=OFF -DNJ2K_BUILD_TOOLS=OFF -DNJ2K_BUILD_BENCH=OFF -DNJ2K_BUILD_SHARED=OFF
   -DNJ2K_BUILD_RUST_SHIM=ON`). A change under `src/`, `include/`, `oracle/`, `tests/common/`,
   `cmake/` or the shim runs CMake again; it rebuilds what changed.

**The twin is pinned to one C++ release.** This crate is version 0.1.3 and its oracle is the C++ neo-j2k
v0.1.3 (commit `00f273d502d211905adb756d2f75ef74321983d0`). CI checks out exactly that commit, so a later
C++ `main` never reddens it. The test `cpp_release` fails when the checkout (or the build folder of
`NJ2K_ORACLE_DIR`) reports another version than the crate's, and the size table test fails when the C++
arena types differ. The pinned SHA is the commit, not the annotated tag object (`git rev-parse v0.1.3^{commit}`). The version
test compares only major.minor.patch, so a local `NJ2K_CPP_SOURCE` pointing at C++ `main` can pass it and
still redden the size table test; only CI's `ref:` pins the exact commit.
Moving to a newer C++ release is a deliberate change: bump the version, the `ref:`
of the checkouts in `.github/workflows/` and this note together.

The C++ is always built in `RelWithDebInfo` (`NJ2K_ORACLE_CONFIG`), with the dynamic C runtime
(`/MD`) that Rust's MSVC target uses.

```
cargo test --release --workspace                                    # every test, the differential ones included
cargo test --release -p neo-j2k-testkit                             # the test kit alone: no C++
cargo test --release -p neo-j2k-oracle --test differential          # the test kit against the C++ harness, R0's gates
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check
```

| Variable | Use |
|---|---|
| `NJ2K_ORACLE_DIR` | a CMake build folder holding the oracle libraries (above) |
| `NJ2K_CPP_SOURCE` | a checkout of the external C++ neo-j2k repository at **v0.1.3** (commit `00f273d502d211905adb756d2f75ef74321983d0`), used when `NJ2K_ORACLE_DIR` is not set |
| `NJ2K_ORACLE_PRESET` | the CMake preset `build.rs` uses when it builds the oracle itself |
| `NJ2K_ORACLE_CONFIG` | the configuration of the oracle's build (default `RelWithDebInfo`) |
| `NJ2K_ORACLE_NO_BUILD` | `1`: no C++ build and no link, for commands that compile without linking (`cargo clippy`, `cargo doc`, `cargo metadata`); a test build then fails to link |
| `NJ2K_TESTDATA` | the fetched conformance data (`python -I tools/fetch_testdata.py`). Add `--all` to fetch the non-regression data too. |
| `NJ2K_DAMAGED_FILES` | an optional local directory or text file list of damaged codestreams for differential replay |
| `NJ2K_TEST_TMP` | the folder for the tests' temporary files, e.g. the repository's Defender-excluded `.fixture-tmp`; also set `TEMP`/`TMP` (Windows) or `TMPDIR` (Linux) to it for the C++ |

The tests never open a dialog: panics and assertion output go to stderr, and the oracle sends the C
runtime's reports to the console (`console_reports()`) before its first call.

#### Header parity

`neo-j2k/tests/header_parity.rs` compares header and sizing results with the C++ on the generated
codestream corpus, selected prefixes and JP2 containers, including the conformance files when
`NJ2K_TESTDATA` is set. It checks the reference-grid bounds, tile layout, component geometry,
reduce policies, limits, and header arena reservations. Targeted cases also compare diagnostics
in order.

#### Pixel and byte parity

The release tests compare native Rust pixel planes and statuses against the C++ decoder on whole
codestreams, small prefixes, reduced and layer-limited output, and packed packet headers. They
compare the encoder's bytes against the C++ on the generated image and settings matrix.
The local [fuzz targets](fuzz/README.md) cover decode, round trips and differential comparison.

The native reference replay compares hashes from Rust decoding against the committed OpenJPEG
tables. It checks the documented empty packed-header tile departure separately. Set
`NJ2K_ORACLE_NO_BUILD=1` to run the standard table without linking the C++ oracle.

```sh
cargo test --release --locked -p neo-j2k --test reference_replay
```

Use `-- --ignored --exact nightly_reference_cuts` with the same command for the extended table.

When `NJ2K_TESTDATA` points at the fetched OpenJPEG data, the pixel tests compare every conformance
and available non-regression codestream at reduce levels 0 to 5. The damage tests mutate the
generated corpus's headers and packet bodies, cut streams at decoder boundaries, and retry with
clean input in the same decoder. They compare exact statuses and decoded samples at reduce 0
and 5 in both strict and non-strict mode under the `Limits::SECOND_LIFE` profile (see [Example implementations](#example-implementations)).

Set `NJ2K_DAMAGED_FILES` to replay additional local inputs with the same options. Run from the workspace root:

```sh
cargo test --release --locked -p neo-j2k --test decode_options -- --nocapture
```

#### Tier-1 against the C++

The C++ Tier-1 is the oracle block by block and decision by decision, through the shim's Tier-1 entries
(`neo-j2k-oracle/shim/shim_t1.cpp`, `neo-j2k-oracle/src/tier1.rs`): the decoder's `t1_decode_cblk` and the store into a
tile buffer, the encoder's `enc_t1_encode` and its commit, and the MQ decoder's decision (`t1_mq.h`) and the MQ encoder
(`enc_mq.h`) driven on registers and byte strings the test sets. The tests are unit tests of the crate
(`cargo test --release -p neo-j2k --lib`) and need `NJ2K_CPP_SOURCE`, which also supplies the vectors:

- every committed decoder vector (3,927 code-blocks, `tests/vectors/t1`) and the six bench dumps (2,034 blocks,
  `tests/vectors/t1_bench`) of the C++ checkout, decoded by Rust and by the C++ and compared coefficient by coefficient,
  in both stores (5/3 and 9/7); the checkout is read, never copied here (`neo-j2k-testkit/src/mqng.rs` reads the
  "MQNG" files); with `NJ2K_T1_VECTORS` naming the C++'s full dump set (37,424 blocks, not committed) that is run too;
- generated code-blocks as `tests/unit/t1_unit.cpp` and `tests/unit/enc_t1_unit.cpp` draw them: every size 1..64 x 1..64
  (partial stripes included), the edge sizes in all 64 code-block styles and 4 orientations, the long shapes (1024 x 4
  and the like), hostile bit-planes and ROI shifts, segments longer than the bytes, split chunks, both wavelets; for the
  encoder blocks of every bit-plane count 0 to 25 with INT32_MIN, compared pass by pass (rate, length, termination,
  distortion bit for bit) and byte by byte;
- the MQ coders decision by decision: random streams (the decoder's bits and registers, the encoder's registers after
  every decision, at short buffer capacities too) and the single decisions of the C++ unit tests, from every state at the
  intervals around the exchange and renormalisation edges. The normal run takes the trimmed enumeration: 6.5 million
  decoder decisions (every 17th interval and the edges, as the C++ unit test's default) and 19 million encoder decisions
  (every 4th interval, and every combination at the edges), in a few seconds.

The long runs are `#[ignore]`d tests named `nightly_*`, run by `nightly.yml` (job `tier1-sweeps`, at both CPU levels) or by hand:

```
NJ2K_CPP_SOURCE=/path/to/neo-j2k cargo test --release --locked -p neo-j2k --lib -- --ignored nightly_
```

They take every interval of the MQ decoder's single decisions (443 million; the C++'s count of 887 million visits each
interval twice) and every combination of the MQ encoder's (4.4 billion), on every thread, and many more generated
blocks. The fuzz target `t1` compares the decoder's Tier-1 with the C++ on inputs of any bytes
([fuzz/README.md](fuzz/README.md)). Not compared: the encoder's coefficient preprocessing alone over all 2^32 inputs
(the C++'s `enc_preprocess` is not in the shim; the extremes go through the encoder's blocks).

#### Oracle and harness checks

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
- **The harness's rules and readers**: the host application's request rules (the viewer's, see [Example implementations](#example-implementations)), the scanner (every corpus file, fuzz seed,
  regression and bomb, and every prefix of those up to 4 KB) and the PGX reader give the C++'s
  results.

#### The C++ harness's FNV basis

The harness's `Fnv64` starts from 1469598103934665603, FNV-1a's published offset basis
(14695981039346656037) with its last digit dropped. Every reference table and digest is built on it,
so the test kit keeps it; it is not a published FNV-1a hash.

### The native-Rust check

```
python -I tools/check_rust_native.py
```

From `cargo metadata` and `cargo package --list` it checks that `neo-j2k`'s normal and build
dependency graph (dev-dependencies left out) holds no crate with a build script, a `links` key, a
`-sys` name or a C/C++ build or binding helper, and not the oracle crate; that the crate itself has
no build script; that the package holds Rust sources, manifests, readme and licence files only; and
that the crate's sources hold no FFI (`extern "C"`, `#[link]`, `#[no_mangle]`, `#[export_name]`), no
`unsafe` outside `src/simd/`, no `static mut` and no `thread_local!`. `cargo deny check` (with
`deny.toml`, cargo-deny 0.20.2 in CI) checks the licences, advisories and sources of the whole
workspace's graph and bans the C/C++ helper crates outright.

### Miri

Miri checks the Rust unit tests, the host job dispatcher with two threads, and small decoder
requests across all reduce levels, including retries after incomplete input. The AVX2 pass
exercises the wavelet, colour, Tier-1 sample stores and packed output kernels. The oracle's
build script does nothing under Miri.

```
cargo +nightly-2026-11-03 miri setup
NJ2K_ORACLE_NO_BUILD=1 cargo +nightly-2026-11-03 miri test -p neo-j2k-testkit -p neo-j2k --lib
NJ2K_ORACLE_NO_BUILD=1 cargo +nightly-2026-11-03 miri test -p neo-j2k --test miri_decode --locked
NJ2K_ORACLE_NO_BUILD=1 RUSTFLAGS="-C target-feature=+avx2" CARGO_TARGET_DIR=target-miri-avx2 \
  cargo +nightly-2026-11-03 miri test -p neo-j2k --lib --locked -- avx2_ interleaved_targets_match_plane_writes_at_every_channel_count
```

## Performance

"OpenJPEG" below means the pinned OpenJPEG 2.5.4 oracle. There are two direct, paired benchmark suites:
the C++ repository measures OpenJPEG against neo-j2k C++, and this repository measures native Rust against
the same neo-j2k C++ implementation. The fixtures and output APIs differ between suites, so the tables keep
the comparisons separate instead of multiplying them into an invented Rust/OpenJPEG result. In every table
the change is the time of the named build against its stated baseline (minus is faster); every comparison
produces identical decoded samples or encoded bytes. No pass/fail gate is stated for these numbers, so the
tables have no gate column.

### neo-j2k C++ versus OpenJPEG 2.5.4

Whole-image decode and encode, measured by the C++ repository's suites. The baseline is OpenJPEG.

| Operation | Suite | Threads | OpenJPEG | neo-j2k C++ | Change vs OpenJPEG |
|---|---|---:|---:|---:|---:|
| Decode 64² whole | C++ viewer suite | 1 | 373 µs | 316 µs | -15.0 % |
| Decode 1,024² whole | C++ viewer suite | 1 | 37.176 ms | 26.158 ms | -30.0 % |
| Decode 4,096² one-tile 9/7 | C++ large suite | 8 | 136 ms | 46 ms | -66.0 % |
| Encode 1,024² RGB lossy | C++ upload suite | 1 | 240.4 ms | 183.7 ms | -24.0 % |
| Encode 1,024² RGB lossless | C++ upload suite | 1 | 253.9 ms | 178.8 ms | -30.0 % |

- neo-j2k C++ is faster than OpenJPEG on every listed decode, from 1.18× to 2.96×.
- One-thread C++ encoding is 24-30% faster than OpenJPEG at 1,024² in the upload benchmark, measured after the
  C++ encoder's MQ-coder redesign (the C++ README's encode rows; later encoder steps take a few percent more off).

The complete C++/OpenJPEG tables are in the
[neo-j2k C++ README](https://github.com/anne-skydancer/neo-j2k#performance).

### Native Rust versus neo-j2k C++

Direct paired measurements; the baseline is neo-j2k C++.

| Operation | Suite | Threads | neo-j2k C++ | neo-j2k Rust | Change vs C++ |
|---|---|---:|---:|---:|---:|
| Decode 1,024² RGB 9/7 | direct I32 AVX2 suite | 1 | 4.780 ms | 4.725 ms | -1.1 % |
| Decode 1,024² RGB 9/7 | direct I32 AVX2 suite | 8 | 1.413 ms | 1.372 ms | -2.9 % |
| Decode 4,096² grayscale 9/7 | direct I32 AVX2 suite | 8 | 3.158 ms | 3.017 ms | -4.4 % |
| Encode 32²-128² RGB | paired AVX2 suite | 1 | baseline | within -2.1 % to +1.3 % | Rust is within about 2% of C++ |

- Native Rust is slightly faster than C++ on the listed large AVX2 decodes, by 1-4% while preserving the same
  multi-core scaling model.
- Rust small-image encoding is approximately equal to C++. Native Rust encoding scales by 4.27-5.47× lossless and
  4.36-5.38× lossy at eight workers on 256²-1,024² inputs. A direct three-way large-encode ratio is not
  claimed because the OpenJPEG/C++ and Rust scaling suites use different inputs and harnesses.

The small-image release tests compare whole, reduced, and partial decodes against C++ and check
that a warmed Rust decoder performs no heap calls on the selected fixtures. The opt-in timing
tests use matching inputs and settings, and compare lossless and lossy encodes byte for byte:

```sh
cargo test --release -p neo-j2k --test small_image_gates -- --ignored --nocapture --test-threads=1
```

### Large-image decode scaling

The large-image benchmark compares direct caller-owned I32 planes using the same persistent
executor at widths 1, 2, 4 and 8. Use a C++ oracle configured with `-DNJ2K_SIMD=SCALAR` for
scalar timing and the AVX2 oracle for an `x86-64-v3` Rust build.

```sh
cargo test --release -p neo-j2k --test small_image_gates -- --ignored --exact large_image_executor_latency --nocapture
```

The same 21 November 2026 system produced the following large-image scaling results. Both codecs
wrote caller-owned I32 planes through the same persistent pool; worker count includes the calling
thread. Each latency is the median of nine alternating rounds with two decodes per round.

Rust against C++ (the change is Rust's time against C++'s):

| Build | Fixture | Workers | Rust ms | C++ ms | Change vs C++ |
|---|---|---:|---:|---:|---:|
| Scalar | 1024² RGB 9/7 | 1 | 7.694 | 8.459 | -9.0 % |
| Scalar | 1024² RGB 9/7 | 2 | 4.452 | 4.887 | -8.9 % |
| Scalar | 1024² RGB 9/7 | 4 | 2.720 | 2.940 | -7.5 % |
| Scalar | 1024² RGB 9/7 | 8 | 1.815 | 1.881 | -3.5 % |
| Scalar | 4096² grayscale 9/7 | 1 | 23.240 | 28.279 | -17.8 % |
| Scalar | 4096² grayscale 9/7 | 2 | 12.301 | 14.983 | -17.9 % |
| Scalar | 4096² grayscale 9/7 | 4 | 7.229 | 9.064 | -20.2 % |
| Scalar | 4096² grayscale 9/7 | 8 | 4.516 | 5.298 | -14.8 % |
| AVX2 | 1024² RGB 9/7 | 1 | 4.725 | 4.780 | -1.1 % |
| AVX2 | 1024² RGB 9/7 | 2 | 2.999 | 3.041 | -1.4 % |
| AVX2 | 1024² RGB 9/7 | 4 | 1.865 | 1.984 | -6.0 % |
| AVX2 | 1024² RGB 9/7 | 8 | 1.372 | 1.413 | -2.9 % |
| AVX2 | 4096² grayscale 9/7 | 1 | 11.025 | 11.960 | -7.8 % |
| AVX2 | 4096² grayscale 9/7 | 2 | 6.520 | 6.992 | -6.7 % |
| AVX2 | 4096² grayscale 9/7 | 4 | 4.138 | 4.840 | -14.5 % |
| AVX2 | 4096² grayscale 9/7 | 8 | 3.017 | 3.158 | -4.4 % |

Scaling of each codec against its own one-worker time (speed-up factors, as measured):

| Build | Fixture | Workers | Rust speedup | C++ speedup |
|---|---|---:|---:|---:|
| Scalar | 1024² RGB 9/7 | 1 | 1.00× | 1.00× |
| Scalar | 1024² RGB 9/7 | 2 | 1.73× | 1.73× |
| Scalar | 1024² RGB 9/7 | 4 | 2.83× | 2.88× |
| Scalar | 1024² RGB 9/7 | 8 | 4.24× | 4.50× |
| Scalar | 4096² grayscale 9/7 | 1 | 1.00× | 1.00× |
| Scalar | 4096² grayscale 9/7 | 2 | 1.89× | 1.89× |
| Scalar | 4096² grayscale 9/7 | 4 | 3.21× | 3.12× |
| Scalar | 4096² grayscale 9/7 | 8 | 5.15× | 5.34× |
| AVX2 | 1024² RGB 9/7 | 1 | 1.00× | 1.00× |
| AVX2 | 1024² RGB 9/7 | 2 | 1.58× | 1.57× |
| AVX2 | 1024² RGB 9/7 | 4 | 2.53× | 2.41× |
| AVX2 | 1024² RGB 9/7 | 8 | 3.44× | 3.38× |
| AVX2 | 4096² grayscale 9/7 | 1 | 1.00× | 1.00× |
| AVX2 | 4096² grayscale 9/7 | 2 | 1.69× | 1.71× |
| AVX2 | 4096² grayscale 9/7 | 4 | 2.66× | 2.47× |
| AVX2 | 4096² grayscale 9/7 | 8 | 3.65× | 3.79× |

### Encode scaling

The native encoder uses the same borrowed-executor model. It divides component wavelets and the
flattened Tier-1 code-block sequence into bounded worker-local batches while retaining codestream
order and byte-for-byte serial output. On 22 November 2026, the release benchmark on the same system
measured these median end-to-end encode latencies:

| Fixture | Workers 1 | Workers 2 | Workers 4 | Workers 8 | 8-worker speedup |
|---|---:|---:|---:|---:|---:|
| 256² RGB lossless 5/3 | 15.818 ms | 9.463 ms | 5.669 ms | 3.706 ms | 4.27× |
| 256² RGB lossy 9/7 | 19.992 ms | 11.147 ms | 6.602 ms | 4.586 ms | 4.36× |
| 512² RGB lossless 5/3 | 59.154 ms | 32.906 ms | 18.070 ms | 10.895 ms | 5.43× |
| 512² RGB lossy 9/7 | 78.967 ms | 42.944 ms | 23.293 ms | 14.771 ms | 5.35× |
| 1,024² RGB lossless 5/3 | 222.519 ms | 124.848 ms | 67.604 ms | 40.648 ms | 5.47× |
| 1,024² RGB lossy 9/7 | 319.312 ms | 180.977 ms | 95.287 ms | 59.399 ms | 5.38× |

Every worker count emits exactly the serial codestream. Tests cover widths 2, 4, 8, 16, and 64,
and enforce that 128² stays serial while 256² uses the supplied executor. Reproduce the table with:

```sh
cargo test --release -p neo-j2k --test small_image_gates -- --ignored --exact large_encoder_executor_latency --nocapture --test-threads=1
```

An `x86-64-v3` assessment produced no consistent encoder improvement: eight-worker speedups ranged
from 4.15-5.61× lossless and 4.28-5.51× lossy, but absolute times moved within noise in both
directions. The encoder therefore retains the profitable SIMD-friendly eight-column 5/3 wavelet
layout and compile-time AVX2 target support without an encoder-only intrinsic path. Tier-1's MQ coder
is sequential within each code-block, so independent code-block parallelism supplies the material gain.

### Small images and the parallel crossover

The decoder deliberately does not query the host executor below 2^18 component samples: the C++
measurements found small decodes about 2% slower when dispatched across threads. This keeps the
measured small-image path serial while applying parallel Tier-1, wavelet, colour/output, and clearing
work to images large enough to benefit. The `decode_executor` tests enforce both sides of that
crossover and compare scheduled output with serial output at job widths 2, 4, 8, 16, and 64.

On 21 November 2026, paired runs on a Ryzen 9 9950X3D under Windows gave these changes in median
Rust latency against C++ for warmed, full-resolution, whole-image caller-owned I32 decodes.
Each reported latency is the median of 21 alternating Rust/C++ rounds with 16 decodes per
round. Rust 1.100.0 and MSVC 19.51 release builds used the same scalar or x86-64-v3/AVX2
instruction-set floor.

| Fixture | Scalar | AVX2 |
|---|---:|---:|
| 32 × 32 RGBA, 5/3 | +2.8 % | +0.9 % |
| 64 × 64 RGB, 9/7 | -11.5 % | +5.0 % |
| 128 × 128 RGBA, 9/7 | -13.8 % | 0.0 % |

Across those fixtures at full resolution, reduce 2, and with a missing EOC marker, the change in
caller-owned decode time ranged from -21.7 % to +8.2 % in the scalar build and -3.8 % to +11.1 % with
AVX2. The change in encoder time over 32², 64², and 128² RGB inputs using both 5/3 and 9/7 ranged from -0.2 % to +5.8 % scalar
and -2.1 % to +1.3 % with AVX2. A negative change means Rust was faster. This is a small-image latency
assessment, not a throughput claim; the C++ measurement includes its test shim constructing plane
descriptors, and encoder calls include the shim's input/output copies. Reproduce it with the ignored
`small_image_gates` timing tests above on an otherwise quiet system.

The warmed allocation test covers both output APIs on seven fixtures, including packed headers,
16-bit samples and 4:2:0 subsampling.

## Example implementations

The Second Life and OpenSim viewers are the first applications that use the library; the library's
built-in limit profiles and parts of its test harness follow their requests.

- **Decoder limit profiles.** `Limits::SECOND_LIFE` holds the Second Life viewer's texture limits (1,024
  tiles, 8 components, 64 layers, 2,048 px a side at the decoded resolution, 256 MiB of working memory).
  `Limits::OPENSIM`, behind the default `opensim` feature (as the C++'s `NJ2K_PROFILE_OPENSIM`), is the same
  with 16,384 px a side.
- **The encoder.** The one-tile JPEG 2000 encoder is the one used for viewer uploads.
- **The harness.** The test kit's request rules are the viewer's. The "viewer" and "upload" suites named in
  the Performance section are the C++ library's benchmark suites for the viewers' requests.
- **Target CPU.** The x86-64-v3 (AVX2) build is the viewer's floor.

### Performance

This repository holds no viewer-specific benchmark. The viewer measurements of the C++ library are in the
[neo-j2k C++ README](https://github.com/anne-skydancer/neo-j2k#performance).

## Scope and roadmap

Part 1 of JPEG 2000 is the core today. The other parts are intended to arrive bit by bit as modules compiled
in or out, as optional Cargo features that are off by default (the C++ library does this with its
`NJ2K_MODULE_<NAME>` CMake options). No such module feature exists yet. HTJ2K (Part 15) is a candidate first
module. No dates, no promise.

## License

`neoj2k-rust` is licensed under the [BSD 2-Clause License](LICENSE). Portions derived from
OpenJPEG retain their notices in [LICENSE.OpenJPEG](neo-j2k/LICENSE.OpenJPEG) and
[NOTICE.md](neo-j2k/NOTICE.md).
