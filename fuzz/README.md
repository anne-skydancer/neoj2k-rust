<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Native Rust fuzz targets

The fuzz crate has its own Cargo workspace. libFuzzer and the optional C++ test oracle are dependencies of this workspace. The published codec has no runtime dependencies.

| Target | Property |
|---|---|
| `header_decode` | Reads J2K and JP2 headers and decodes whole inputs and prefixes. Exercises reduce policies, layer limits, strictness, component selection, JP2 palette and channel handling, and shared decoder state with a 32 MiB codec budget. |
| `roundtrip` | Encodes bounded 8-bit images with both wavelets. Checks reversible samples, irreversible sample bounds, geometry, quality layers, reduced previews, and padded packed output in both row directions. |
| `differential` | Compares statuses and decoded samples on bounded J2K and JP2 inputs with different reduce policies, layer limits and strictness. Reuses the decoder for a prefix and a retry of the original input. Requires `oracle`. |
| `api_sequence` | Replays up to 16 header reads, reduce and memory-limit changes, and decodes against C++. Compares statuses, dimensions and samples as inputs are cut, retried or replaced. Requires `oracle`. |
| `t1` | Decodes a code-block of any bytes (shape, code-block style, ROI shift, bit-planes, segments and bytes from the input, the C++ `fuzz_t1` format) with the codec's Tier-1 entry point and with the C++'s `t1_decode_cblk`, 5/3 and 9/7 store alike, and compares the refusal and every coefficient. Needs the `bench-internals` entry point of the codec (enabled by `oracle`). Requires `oracle`. |
| `encode_options` | Uses the shared C++ seed format to compare encoder statuses, codestream bytes, and setup write-backs for the options expressible by the Rust API. Requires `oracle`. |

Run from the workspace root with the nightly pinned in `nightly-toolchain` and cargo-fuzz 0.13.2. The first corpus directory receives generated inputs. The checked-in fixture directories are seed sources.

The JP2 seeds wrap the generated `sl_64_rgb_97.j2k` and `size_3x5_g.j2k` fixtures. The latter adds a
palette, component mapping and channel definitions.

```sh
nightly=$(cat nightly-toolchain)
rustup toolchain install "$nightly" --profile minimal
cargo install cargo-fuzz --version 0.13.2 --locked
mkdir -p fuzz/corpus/header_decode fuzz/corpus/roundtrip
cargo +"$nightly" fuzz run header_decode fuzz/corpus/header_decode ../corpus/generated fuzz/seeds/jp2 -- -max_len=1048576 -max_total_time=120 -timeout=5 -print_final_stats=1
cargo +"$nightly" fuzz run roundtrip fuzz/corpus/roundtrip fuzz/seeds/roundtrip -- -max_len=16400 -max_total_time=120 -timeout=5 -print_final_stats=1
```

On Windows, use an x64 Visual Studio Developer PowerShell with the C++ toolchain and AddressSanitizer installed. The MSVC tools directory must be on `PATH` so the fuzz executable can load the ASan runtime. [Rust Fuzz Book Windows setup](https://rust-fuzz.github.io/book/cargo-fuzz/windows/setup.html)

```powershell
$nightly = (Get-Content nightly-toolchain -Raw).Trim()
rustup toolchain install $nightly --profile minimal
cargo install cargo-fuzz --version 0.13.2 --locked
New-Item -ItemType Directory -Force fuzz/corpus/header_decode, fuzz/corpus/roundtrip
cargo "+$nightly" fuzz run header_decode fuzz/corpus/header_decode ../corpus/generated fuzz/seeds/jp2 -- -max_len=1048576 -max_total_time=120 -timeout=5 -print_final_stats=1
cargo "+$nightly" fuzz run roundtrip fuzz/corpus/roundtrip fuzz/seeds/roundtrip -- -max_len=16400 -max_total_time=120 -timeout=5 -print_final_stats=1
```

For the differential target, use the C++ oracle build described in the [Rust workspace README](../README.md#tests-and-the-c-oracle). Enabling `oracle` on `roundtrip` also compares the entire encoded stream to the C++ encoder. Cargo-fuzz instruments the Rust target. The C++ oracle uses its own CMake sanitizer settings.

On Windows, configure and build `nj2k_rust_shim` with the `windows-msvc-asan` preset and point
`NJ2K_ORACLE_DIR` to that build folder. Both libraries then use the same MSVC AddressSanitizer
annotations.

```sh
mkdir -p fuzz/corpus/differential fuzz/corpus/encode_options fuzz/corpus/api_sequence
cargo +"$nightly" fuzz run --features oracle differential fuzz/corpus/differential ../corpus/generated fuzz/seeds/jp2 -- -max_len=131072 -max_total_time=120 -timeout=5 -print_final_stats=1
cargo +"$nightly" fuzz run --features oracle encode_options fuzz/corpus/encode_options fuzz/seeds/encode -- -max_len=16400 -max_total_time=120 -timeout=5 -print_final_stats=1
cargo +"$nightly" fuzz run --features oracle api_sequence fuzz/corpus/api_sequence fuzz/seeds/api_sequence -- -max_len=18 -max_total_time=45 -timeout=10 -seed=137 -print_final_stats=1
mkdir -p fuzz/corpus/t1
cargo +"$nightly" fuzz run --features oracle t1 fuzz/corpus/t1 fuzz/seeds/t1 -- -max_len=8192 -max_total_time=120 -timeout=5 -print_final_stats=1
```

The API sequence seeds exercise calls before a successful header read, shortened codestreams,
changes to the memory limit, and a successful decode on retry. Run
`cargo test --release --manifest-path fuzz/Cargo.toml --features oracle --lib --locked` to check
those seeds and the differential seeds without the fuzz runner.

Artifacts from local runs stay under `fuzz/artifacts/`. Replay a saved input by passing its path in place of the corpus directories. Use `-runs=0` with the corpus directories for a seed-only replay.

## Seeds, regressions and CI

- `seeds/jp2`, `seeds/api_sequence`: the JP2 and API-sequence seeds. `seeds/roundtrip` and `seeds/encode`: the byte format the C++ round-trip and encode targets read (twelve and eighteen parameter bytes, then samples), shared with the C++ repository. `corpus/generated` at the repository root seeds `header_decode` and `differential`. `j2k.dict` is the libFuzzer dictionary of the JPEG 2000 markers and JP2 boxes.
- `regressions/`: an input a fuzzer found a defect with, kept once the code is fixed, named `<target>-<finding>.bin`. `cargo test --release --manifest-path fuzz/Cargo.toml --lib` replays it and the seeds through `header_decode` and `roundtrip`; with `--features oracle` also through `differential`, `encode_options`, `api_sequence` and `t1`.
- `ci.yml` replays the seeds and regressions on Linux and Windows, at the baseline and at `x86-64-v3`. `oracle.yml` (weekly) replays them against the C++ oracle.
- `cflite-pr.yml` fuzzes `header_decode` and `roundtrip` with AddressSanitizer for five minutes on every pull request that touches the codec or the fuzz crate; `cflite-batch.yml` does the same for an hour a day on `main`, `cflite-prune.yml` minimises the stored corpus and `cflite-coverage.yml` stores the coverage report the pull request runs aim by. `.clusterfuzzlite/` holds the build image (Rust base builder pinned by digest, the nightly of `nightly-toolchain`, cargo-fuzz 0.13.2) and `build.sh`.
- `nightly.yml` runs all six targets for ten minutes each, one job per target, from the corpus the earlier nights kept in the Actions cache. The four oracle targets check out the C++ repository and build it with GCC 15. Findings are uploaded as the artifact `fuzz-findings-<target>`. Start it by hand with `workflow_dispatch` and a number of seconds.
- Rust has no UndefinedBehaviorSanitizer or MemorySanitizer fuzz build here: safe code has no undefined behaviour, the fuzz builds keep debug assertions (arithmetic overflow panics), and Miri (`ci.yml`) covers the `unsafe` of `src/simd`.
