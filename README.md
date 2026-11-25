<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# neo-j2k in Rust

A JPEG 2000 codec library written natively in Rust: Part 1 decoding of J2K, JP2 and JPH, and encoding
with lossless 5/3 and irreversible 9/7 wavelets. It is the Rust counterpart of the C++ library
[neo-j2k](https://github.com/anne-skydancer/neo-j2k) and implements the same core codec. It is not a
wrapper: the crate `neo-j2k` has no build script, no C or C++, no `-sys` crate and no FFI, in itself or in
its dependency tree; it has no runtime dependency at all. The crate forbids `unsafe` code, including in
its `simd` module; its lane kernels use safe Rust arrays and slices.
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
| `neo-j2k-bench/` | the Tier-1 benchmark: TSC cycles of the decoder's and the encoder's code-block entry points, driven by `tools/bench_tier1.py` ([doc/bench.md](doc/bench.md)). It is, besides the oracle and the fuzz crate, a crate that holds `unsafe` (the cycle counter, thread affinity), and has no dependency but the codec and the test kit | never |
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
| `simd` | yes | safe sample rounding and fixed-width lane kernels for x86-64 builds |
| `opensim` | yes | the OpenSim limit profile, as the C++'s `NJ2K_PROFILE_OPENSIM` |
| `bench-internals` | no | not API: the two Tier-1 entry points (decode a code-block, encode a code-block) that `neo-j2k-bench` times, hidden from the documentation; implies `encode` ([doc/bench.md](doc/bench.md)) |

The workspace's release profile (`lto = "fat"`, `codegen-units = 1`, `panic = "unwind"`) applies to
its tests, tools and benches; a program that depends on the crate uses its own profile.

### Target CPU

The crate selects kernels at compile time, as in the C++ (decision D8).

| Build | How | Kernels |
|---|---|---|
| baseline x86-64 | the default | scalar with safe sample rounding |
| x86-64-v3 (AVX2) | `RUSTFLAGS="-C target-cpu=x86-64-v3"`, or in the consumer's `.cargo/config.toml`: `[build] rustflags = ["-C", "target-cpu=x86-64-v3"]` | safe lane kernels in `mod simd`, optimized for the selected target |

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

The default C++ test configuration is `RelWithDebInfo` (`NJ2K_ORACLE_CONFIG`), with the dynamic
C runtime (`/MD`) that Rust's MSVC target uses. The three-way benchmark below selects `Release`.

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
- the C++ decoder tests' other block sets: segments longer than the bytes (`test_overlong`, lengths up to 0xFFFFFFFF),
  a raw segment holding a cleanup pass (`test_raw_cleanup`), tiny blocks over every sign and significance pattern of a
  column (`test_split`, 5.8 million blocks here, 3.84 million in the C++, trimmed in the normal run and complete in
  nightly with the count of the patterns it must see), blocks stored into a tile buffer of another stride and offset,
  and the store alone around every ROI threshold; not mirrored: the table and AGG-run tests of the C++ (`test_tables`,
  `test_agg_run`), which check its own reformulations;
- generated code-blocks as `tests/unit/t1_unit.cpp` and `tests/unit/enc_t1_unit.cpp` draw them: every size 1..64 x 1..64
  (partial stripes included), the edge sizes in all 64 code-block styles and 4 orientations, the long shapes (1024 x 4
  and the like), hostile bit-planes and ROI shifts, segments longer than the bytes, split chunks, both wavelets; for the
  encoder blocks of every bit-plane count 0 to 25 with INT32_MIN, compared pass by pass (rate, length, termination,
  distortion bit for bit) and byte by byte;
- the MQ coders decision by decision: random streams (the decoder's bits and registers, the encoder's registers after
  every decision, at short buffer capacities too), the single decisions of the C++ unit tests, from every state at the
  intervals around the exchange and renormalisation edges, and the encoder's flush from registers around its boundary
  (`c | 0xFFFF == c + a`). The streams are compared with the shim's restatement of the C++'s start of a coder (INITDEC is
  local to `src/t1.cpp`; `shim_t1.cpp` says so), the code-block tests with the real one. The normal run takes the
  trimmed enumeration: 6.5 million decoder decisions (every 17th interval and the edges, as the C++ unit test's default)
  and 19 million encoder decisions (every 4th interval, and every combination at the edges), in a few seconds.

The long runs are `#[ignore]`d tests named `nightly_*`, run by `nightly.yml` (job `tier1-sweeps`, at both CPU levels)
or by hand:

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
`unsafe` anywhere in the crate's sources, no `static mut` and no `thread_local!`. The crate also
enforces `#![forbid(unsafe_code)]`, so a module cannot relax the compiler's safety-code lint.
`cargo deny check` (with
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

Matched three-way measurements on **2026-11-25**, using an AMD Ryzen 9 9950X3D under
Linux/WSL2, one thread pinned to one logical CPU, and AVX2 (`x86-64-v3`) builds.
Inputs are the repository's generated 8-bit fixtures. **Latencies are in milliseconds;
lower is faster.** Each value is the median of three process medians.

| Operation and input | neo-j2k C++ | neoj2k-rust | OpenJPEG 2.5.4 oracle | Rust speedup vs OpenJPEG |
|---|---:|---:|---:|---:|
| Decode 32 × 32 RGBA, 5/3 | 0.166 | 0.149 | 0.375 | 2.52× |
| Decode 64 × 64 RGB, 9/7 | 0.0515 | 0.0565 | 0.105 | 1.86× |
| Decode 128 × 128 RGBA, 9/7 | 0.11 | 0.115 | 0.374 | 3.26× |
| Decode 64 × 64 RGB, 9/7 → 16 × 16 | 0.0208 | 0.0264 | 0.0316 | 1.20× |
| Decode 1,024 × 1,024 RGB, 9/7 | 4.27 | 4.01 | 14.1 | 3.52× |
| Decode 4,096 × 4,096 grayscale, 9/7 | 11.8 | 11 | 82.6 | 7.53× |
| Encode 32 × 32 RGB, lossless 5/3 | 0.11 | 0.113 | 0.282 | 2.51× |
| Encode 32 × 32 RGB, lossy 9/7 | 0.114 | 0.11 | 0.322 | 2.93× |
| Encode 64 × 64 RGB, lossless 5/3 | 0.606 | 0.617 | 0.995 | 1.61× |
| Encode 64 × 64 RGB, lossy 9/7 | 0.639 | 0.662 | 1.19 | 1.80× |
| Encode 128 × 128 RGB, lossless 5/3 | 2.31 | 2.35 | 3.41 | 1.45× |
| Encode 128 × 128 RGB, lossy 9/7 | 2.41 | 2.57 | 4.29 | 1.67× |

Rust speedup is **OpenJPEG time ÷ Rust time**, calculated from the unrounded matched medians.
A value above 1× means Rust completed the same request faster.

Sources: Rust `28d96e9` (the safe implementation, crate version 0.1.3), neo-j2k C++
0.1.3 at `81407c2`, and the repository's pinned OpenJPEG 2.5.4 fork at `66b16561`.
All three were built from source; Rust uses Release with fat LTO and one codegen unit,
and the native libraries use GCC Release/O3 with strict floating-point settings.

Every decode creates and destroys a fresh codec, including header setup, and writes
caller-owned I32 output. OpenJPEG allocates internal planes and copies them to that output;
Rust and C++ can reconstruct directly into it. Input loading and caller-buffer allocation
are outside timing. Every encode creates a fresh codec and returns an owned codestream;
input conversion, allocation, output copying and teardown are included. The 9/7 encodes
keep every coding pass, with no target rate. Decoded samples and encoded bytes matched
across all three libraries before and after every workload in all three repetitions.

The same executable runs all codecs with balanced order and 32 preconditioning calls
before each timed batch. The [measurement report](doc/benchmarks/2026-11-25-three-way/README.md)
contains process ranges, source/build hashes and raw results. These figures describe the
listed single-thread API workloads; they do not establish a universal ranking.

Reproduce on Linux with the [three-way benchmark](tools/bench_three_way/README.md):

```sh
python3 tools/bench_three_way/run.py --cpp-source ../neo-j2k --out /tmp/neoj2k-three-way
```

Earlier measurements, including parallel scaling and viewer-specific suites, are retained
in [historical performance tables](doc/performance-history.md). Other CI comparisons and
the Tier-1 benchmark are documented in [doc/bench.md](doc/bench.md).

## Example implementations

The Second Life and OpenSim viewers are the first applications that use the library; the library's
built-in limit profiles and parts of its test harness follow their requests.

- **Decoder limit profiles.** `Limits::SECOND_LIFE` holds the Second Life viewer's texture limits (1,024
  tiles, 8 components, 64 layers, 2,048 px a side at the decoded resolution, 256 MiB of working memory).
  `Limits::OPENSIM`, behind the default `opensim` feature (as the C++'s `NJ2K_PROFILE_OPENSIM`), is the same
  with 16,384 px a side.
- **The encoder.** The one-tile JPEG 2000 encoder is the one used for viewer uploads.
- **The harness.** The test kit's request rules are the viewer's. The C++ library's "viewer" and "upload"
  suites measure those application requests; they are linked below.
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
