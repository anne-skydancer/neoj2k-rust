<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# neo-j2k in Rust

The native Rust port of neo-j2k uses the C++ library as its test oracle. The port targets the same
samples, statuses, messages and encoded bytes for the same inputs, on every executor and at both CPU
levels.

**Native Rust.** The crate `neo-j2k` has no build script, no C or C++, no `-sys` crate and no FFI, in
itself or in its dependency tree; it has no runtime dependency at all. `unsafe` is allowed only in its
`simd` module. The C++ appears only in the test-only crate `neo-j2k-oracle`, a
dev-dependency that is never published. The native-Rust check below verifies this directly.

Paths below are relative to the repository root.

## Codec

| Crate | What | Published |
|---|---|---|
| `neo-j2k/` | native J2K and JP2/JPH decoding and the one-tile JPEG 2000 encoder used for viewer uploads | release crate |
| `neo-j2k-testkit/` | the Rust side of the C++ harness in `tests/common`: requests and outcomes, the viewer's rules, `nj2k_diff`'s digests and reference tables, the codestream scanner, the PGX reader, a test pool executor. Pure Rust, no dependency | never |
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

## Toolchain

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

## The library

```rust
use neo_j2k::{Decoder, DecoderConfig, Error, Limits, ReducePolicy};

fn dimensions(codestream: &[u8]) -> Result<(u32, u32), Error> {
    let mut decoder = Decoder::new(DecoderConfig {
        limits: Limits::SECOND_LIFE,
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

## OpenJPEG 2.5.4 versus neo-j2k C++ versus native Rust

“OpenJPEG2000” below means the pinned OpenJPEG 2.5.4 oracle. There are two direct, paired benchmark suites:
the C++ repository measures OpenJPEG against neo-j2k C++, and this repository measures native Rust against
the same neo-j2k C++ implementation. The fixtures and output APIs differ between suites, so the table keeps
the two ratios separate instead of multiplying them into an invented Rust/OpenJPEG result. Lower ratios are
faster; every comparison produces identical decoded samples or encoded bytes.

| Direct paired measurement | Threads | OpenJPEG | neo-j2k C++ | neo-j2k Rust | Measured result |
|---|---:|---:|---:|---:|---|
| Decode 64² whole, C++ viewer suite | 1 | 373 µs | 316 µs | — | C++ / OpenJPEG 0.85 |
| Decode 1,024² whole, C++ viewer suite | 1 | 37.176 ms | 26.158 ms | — | C++ / OpenJPEG 0.70 |
| Decode 4,096² one-tile 9/7, C++ large suite | 8 | 136 ms | 46 ms | — | C++ / OpenJPEG 0.34 |
| Decode 1,024² RGB 9/7, direct I32 AVX2 suite | 1 | — | 4.780 ms | 4.725 ms | Rust / C++ 0.989 |
| Decode 1,024² RGB 9/7, direct I32 AVX2 suite | 8 | — | 1.413 ms | 1.372 ms | Rust / C++ 0.971 |
| Decode 4,096² grayscale 9/7, direct I32 AVX2 suite | 8 | — | 3.158 ms | 3.017 ms | Rust / C++ 0.956 |
| Encode 1,024² RGB lossy, C++ upload suite | 1 | 250.0 ms | 284.5 ms | — | C++ / OpenJPEG 1.14 |
| Encode 1,024² RGB lossless, C++ upload suite | 1 | 262.8 ms | 295.9 ms | — | C++ / OpenJPEG 1.13 |
| Encode 32²-128² RGB, paired AVX2 suite | 1 | — | baseline | within 0.979-1.013× | Rust is within about 2% of C++ |

What this establishes directly:

- neo-j2k C++ is faster than OpenJPEG on every listed decode, from 1.18× to 2.96×.
- Native Rust is slightly faster than C++ on the listed large AVX2 decodes, by 1-4% while preserving the same
  multi-core scaling model.
- One-thread C++ encoding is 13-14% slower than OpenJPEG at 1,024² in the upload benchmark. Rust small-image
  encoding is approximately equal to C++. Native Rust encoding scales by 4.27-5.47× lossless and
  4.36-5.38× lossy at eight workers on 256²-1,024² inputs. A direct three-way large-encode ratio is not
  claimed because the OpenJPEG/C++ and Rust scaling suites use different inputs and harnesses.

The complete C++/OpenJPEG tables are in the
[neo-j2k C++ README](https://github.com/anne-skydancer/neo-j2k#how-fast). The complete paired Rust/C++ scaling
table and reproduction commands are below.

The small-image release tests compare whole, reduced, and partial decodes against C++ and check
that a warmed Rust decoder performs no heap calls on the selected fixtures. The opt-in timing
tests use matching inputs and settings, and compare lossless and lossy encodes byte for byte:

```sh
cargo test --release -p neo-j2k --test small_image_gates -- --ignored --nocapture --test-threads=1
```

The large-image benchmark compares direct caller-owned I32 planes using the same persistent
executor at widths 1, 2, 4 and 8. Use a C++ oracle configured with `-DNJ2K_SIMD=SCALAR` for
scalar timing and the AVX2 oracle for an `x86-64-v3` Rust build.

```sh
cargo test --release -p neo-j2k --test small_image_gates -- --ignored --exact large_image_executor_latency --nocapture
```

The same 21 November 2026 system produced the following large-image scaling results. Both codecs
wrote caller-owned I32 planes through the same persistent pool; worker count includes the calling
thread. Each latency is the median of nine alternating rounds with two decodes per round.

| Build | Fixture | Workers | Rust ms | C++ ms | Rust speedup | C++ speedup | Rust/C++ |
|---|---|---:|---:|---:|---:|---:|---:|
| Scalar | 1024² RGB 9/7 | 1 | 7.694 | 8.459 | 1.00× | 1.00× | 0.910 |
| Scalar | 1024² RGB 9/7 | 2 | 4.452 | 4.887 | 1.73× | 1.73× | 0.911 |
| Scalar | 1024² RGB 9/7 | 4 | 2.720 | 2.940 | 2.83× | 2.88× | 0.925 |
| Scalar | 1024² RGB 9/7 | 8 | 1.815 | 1.881 | 4.24× | 4.50× | 0.965 |
| Scalar | 4096² grayscale 9/7 | 1 | 23.240 | 28.279 | 1.00× | 1.00× | 0.822 |
| Scalar | 4096² grayscale 9/7 | 2 | 12.301 | 14.983 | 1.89× | 1.89× | 0.821 |
| Scalar | 4096² grayscale 9/7 | 4 | 7.229 | 9.064 | 3.21× | 3.12× | 0.798 |
| Scalar | 4096² grayscale 9/7 | 8 | 4.516 | 5.298 | 5.15× | 5.34× | 0.852 |
| AVX2 | 1024² RGB 9/7 | 1 | 4.725 | 4.780 | 1.00× | 1.00× | 0.989 |
| AVX2 | 1024² RGB 9/7 | 2 | 2.999 | 3.041 | 1.58× | 1.57× | 0.986 |
| AVX2 | 1024² RGB 9/7 | 4 | 1.865 | 1.984 | 2.53× | 2.41× | 0.940 |
| AVX2 | 1024² RGB 9/7 | 8 | 1.372 | 1.413 | 3.44× | 3.38× | 0.971 |
| AVX2 | 4096² grayscale 9/7 | 1 | 11.025 | 11.960 | 1.00× | 1.00× | 0.922 |
| AVX2 | 4096² grayscale 9/7 | 2 | 6.520 | 6.992 | 1.69× | 1.71× | 0.933 |
| AVX2 | 4096² grayscale 9/7 | 4 | 4.138 | 4.840 | 2.66× | 2.47× | 0.855 |
| AVX2 | 4096² grayscale 9/7 | 8 | 3.017 | 3.158 | 3.65× | 3.79× | 0.956 |

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

The decoder deliberately does not query the host executor below 2^18 component samples: the C++
measurements found small decodes about 2% slower when dispatched across threads. This keeps the
measured small-image path serial while applying parallel Tier-1, wavelet, colour/output, and clearing
work to images large enough to benefit. The `decode_executor` tests enforce both sides of that
crossover and compare scheduled output with serial output at job widths 2, 4, 8, 16, and 64.

`Decoder::set_limits` changes the resource limits for subsequent requests. The decoder checks the
header again under the new limits and can reuse its working buffers.
`Decoder::statistics` exposes the codec's budget and arena accounting. Use
`Decoder::set_retained_bytes` to cap the reusable native buffers that survive a decode and
`Decoder::retained_work_bytes` to read their live capacity. A host can lend its executor through
`DecodeParams::executor` for large tiles. Small requests stay serial.

On 21 November 2026, paired runs on a Ryzen 9 9950X3D under Windows gave these median
Rust/C++ latency ratios for warmed, full-resolution, whole-image caller-owned I32 decodes.
Each reported latency is the median of 21 alternating Rust/C++ rounds with 16 decodes per
round. Rust 1.100.0 and MSVC 19.51 release builds used the same scalar or x86-64-v3/AVX2
instruction-set floor.

| Fixture | Scalar | AVX2 |
|---|---:|---:|
| 32 × 32 RGBA, 5/3 | 1.028 | 1.009 |
| 64 × 64 RGB, 9/7 | 0.885 | 1.050 |
| 128 × 128 RGBA, 9/7 | 0.862 | 1.000 |

Across those fixtures at full resolution, reduce 2, and with a missing EOC marker, caller-owned
decode ratios ranged from 0.783 to 1.082 in the scalar build and 0.962 to 1.111 with AVX2. Encoder
ratios over 32², 64², and 128² RGB inputs using both 5/3 and 9/7 ranged from 0.998 to 1.058 scalar
and 0.979 to 1.013 with AVX2. A ratio below 1 means Rust was faster. This is a small-image latency
assessment, not a throughput claim; the C++ measurement includes its test shim constructing plane
descriptors, and encoder calls include the shim's input/output copies. Reproduce it with the ignored
`small_image_gates` timing tests above on an otherwise quiet system.

The warmed allocation test covers both output APIs on seven fixtures, including packed headers,
16-bit samples and 4:2:0 subsampling.

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

The workspace's release profile (`lto = "fat"`, `codegen-units = 1`, `panic = "unwind"`) applies to
its tests, tools and benches; a program that depends on the crate uses its own profile.

## Target CPU

The crate selects kernels at compile time, as in the C++ (decision D8).

| Build | How | Kernels |
|---|---|---|
| baseline x86-64 | the default | scalar with SSE sample rounding |
| x86-64-v3 (AVX2, the viewer's floor) | `RUSTFLAGS="-C target-cpu=x86-64-v3"`, or in the consumer's `.cargo/config.toml`: `[build] rustflags = ["-C", "target-cpu=x86-64-v3"]` | AVX2 in `mod simd` |

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

## Tests and the C++ oracle

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
| `NJ2K_CPP_SOURCE` | a checkout of the external C++ neo-j2k repository, used when `NJ2K_ORACLE_DIR` is not set |
| `NJ2K_ORACLE_PRESET` | the CMake preset `build.rs` uses when it builds the oracle itself |
| `NJ2K_ORACLE_CONFIG` | the configuration of the oracle's build (default `RelWithDebInfo`) |
| `NJ2K_ORACLE_NO_BUILD` | `1`: no C++ build and no link, for commands that compile without linking (`cargo clippy`, `cargo doc`, `cargo metadata`); a test build then fails to link |
| `NJ2K_TESTDATA` | the fetched conformance data (`python -I tools/fetch_testdata.py`). Add `--all` to fetch the non-regression data too. |
| `NJ2K_DAMAGED_FILES` | an optional local directory or text file list of damaged codestreams for differential replay |
| `NJ2K_TEST_TMP` | the folder for the tests' temporary files, e.g. the repository's Defender-excluded `.fixture-tmp`; also set `TEMP`/`TMP` (Windows) or `TMPDIR` (Linux) to it for the C++ |

The tests never open a dialog: panics and assertion output go to stderr, and the oracle sends the C
runtime's reports to the console (`console_reports()`) before its first call.

### Header parity

`neo-j2k/tests/header_parity.rs` compares header and sizing results with the C++ on the generated
codestream corpus, selected prefixes and JP2 containers, including the conformance files when
`NJ2K_TESTDATA` is set. It checks the reference-grid bounds, tile layout, component geometry,
reduce policies, limits, and header arena reservations. Targeted cases also compare diagnostics
in order.

### Pixel and byte parity

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
and 5 in both strict and non-strict mode under the Second Life resource limits.

Set `NJ2K_DAMAGED_FILES` to replay additional local inputs with the same options. Run from the workspace root:

```sh
cargo test --release --locked -p neo-j2k --test decode_options -- --nocapture
```

### Oracle and harness checks

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
`deny.toml`, cargo-deny 0.20.2 in CI) checks the licences, advisories and sources of the whole
workspace's graph and bans the C/C++ helper crates outright.

## Miri

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

## Windows and Linux

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

## License

`neoj2k-rust` is licensed under the [BSD 2-Clause License](LICENSE). Portions derived from
OpenJPEG retain their notices in [LICENSE.OpenJPEG](neo-j2k/LICENSE.OpenJPEG) and
[NOTICE.md](neo-j2k/NOTICE.md).
