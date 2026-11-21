<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Native Rust fuzz targets

The fuzz crate has its own Cargo workspace. libFuzzer and the optional C++ test oracle are dependencies of this workspace. The published codec has no runtime dependencies.

| Target | Property |
|---|---|
| `header_decode` | Reads J2K and JP2 headers and decodes whole inputs and prefixes. Exercises reduce policies, layer limits, strictness, component selection, and shared decoder state with a 32 MiB codec budget. |
| `roundtrip` | Encodes bounded 8-bit images with the two wavelets. Verifies reversible samples exactly, checks irreversible sample bounds and geometry, and exercises quality layers and reduced previews. |
| `differential` | Compares status and decoded samples from the native and C++ memory decoders on bounded J2K inputs, including cuts and mutations of repository fixtures. Requires the `oracle` feature. |

Run from `rust/` with the nightly pinned in `nightly-toolchain` and cargo-fuzz 0.13.2. The first corpus directory receives generated inputs. The checked-in fixture directories are seed sources.

```sh
nightly=$(cat nightly-toolchain)
rustup toolchain install "$nightly" --profile minimal
cargo install cargo-fuzz --version 0.13.2 --locked
mkdir -p fuzz/corpus/header_decode fuzz/corpus/roundtrip
cargo +"$nightly" fuzz run header_decode fuzz/corpus/header_decode ../corpus/generated -- -max_len=1048576 -max_total_time=120 -timeout=5 -print_final_stats=1
cargo +"$nightly" fuzz run roundtrip fuzz/corpus/roundtrip ../fuzz/seeds/roundtrip -- -max_len=16400 -max_total_time=120 -timeout=5 -print_final_stats=1
```

On Windows, use an x64 Visual Studio Developer PowerShell with the C++ toolchain and AddressSanitizer installed. The MSVC tools directory must be on `PATH` so the fuzz executable can load the ASan runtime. [Rust Fuzz Book Windows setup](https://rust-fuzz.github.io/book/cargo-fuzz/windows/setup.html)

```powershell
$nightly = (Get-Content nightly-toolchain -Raw).Trim()
rustup toolchain install $nightly --profile minimal
cargo install cargo-fuzz --version 0.13.2 --locked
New-Item -ItemType Directory -Force fuzz/corpus/header_decode, fuzz/corpus/roundtrip
cargo "+$nightly" fuzz run header_decode fuzz/corpus/header_decode ../corpus/generated -- -max_len=1048576 -max_total_time=120 -timeout=5 -print_final_stats=1
cargo "+$nightly" fuzz run roundtrip fuzz/corpus/roundtrip ../fuzz/seeds/roundtrip -- -max_len=16400 -max_total_time=120 -timeout=5 -print_final_stats=1
```

For the differential target, use the C++ oracle build described in the [Rust workspace README](../README.md#tests-and-the-c-oracle). Enabling `oracle` on `roundtrip` also compares the entire encoded stream to the C++ encoder. Cargo-fuzz instruments the Rust target. The C++ oracle uses its own CMake sanitizer settings.

On Windows, configure and build `nj2k_rust_shim` with the `windows-msvc-asan` preset and point
`NJ2K_ORACLE_DIR` to that build folder. Both libraries then use the same MSVC AddressSanitizer
annotations.

```sh
mkdir -p fuzz/corpus/differential
cargo +"$nightly" fuzz run --features oracle differential fuzz/corpus/differential ../corpus/generated -- -max_len=131072 -max_total_time=120 -timeout=5 -print_final_stats=1
```

Artifacts from local runs stay under `rust/fuzz/artifacts/`. Replay a saved input by passing its path in place of the corpus directories. Use `-runs=0` with the corpus directories for a seed-only replay.
