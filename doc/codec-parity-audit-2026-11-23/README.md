<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Evidence reproduction

The parent report is [codec-parity-audit-2026-11-23.md](../codec-parity-audit-2026-11-23.md). These are investigation artifacts, not new shipping benchmarks or production code.

## Local API experiment

`local-results/byte-1.txt` through `byte-3.txt` are the original small-image suite. `typed-1.txt` through `typed-3.txt` use the modified helper in `typed-probe.rs`. `encode-1.txt` through `encode-3.txt` are the original cold encoder suite. Each process retained all of its exact output assertions.

To rebuild the original suites from the repository root in PowerShell:

```powershell
$env:NJ2K_CPP_SOURCE = 'C:\Users\annes\source\repos\neo-j2k'
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'codec-parity-byte-repro'
$env:RUSTFLAGS = '-C target-cpu=x86-64-v3'
cargo test --locked --release -p neo-j2k --test small_image_gates --no-run
cargo test --locked --release -p neo-j2k --test small_image_gates small_image_median_latency -- --exact --nocapture --test-threads=1
cargo test --locked --release -p neo-j2k --test small_image_gates small_encoder_median_latency -- --exact --nocapture --test-threads=1
```

For typed reproduction, create a standalone scratch package outside the workspace, copy `typed-probe-Cargo.toml` to its `Cargo.toml`, and `typed-probe.rs` to `tests/typed.rs`. Adjust the three absolute path dependencies to the local checkout. Build with the same `RUSTFLAGS`, C++ source and release profile, using its own target directory. Run `cargo test --release --test typed small_image_median_latency -- --exact --nocapture --test-threads=1` there. Generate a fresh lockfile in that scratch package if needed. This probe intentionally duplicates the original harness to preserve its schedule and assertions; it is not a proposed code organization for the repository.

The recorded window ran the built executables directly with their processes pinned to CPU 7; do not claim the commands above reproduce that pin by themselves. Restrict the benchmark process/thread affinity using the host's native facilities, and choose a valid CPU on that host. The original measured window used three repetitions with byte/typed order BT, TB, BT, followed by encode per repetition. It sampled other-process CPU load before the window. Compilation was finished before timing.

Typed uses eight stack plane descriptors initialized per call; byte uses prebuilt descriptors. The binaries differ and run at different times. The experiment establishes remaining typed-API deficits, not an isolated estimate of output-copy cost. Raw files contain Rust/C++ absolute times rounded by the existing harness; the parent report recomputes percentage changes from those columns and takes their median across repetitions.

## Compiler evidence

Build the library at each CPU level with `--emit=asm`, with separate target directories, as documented in `inspect-assembly.py`. Find the generated `.s` under the custom Cargo `release/build/neo-j2k/HASH/out/` directory and run:

```text
python inspect-assembly.py PATH_TO_ASSEMBLY
```

`assembly-baseline.txt` and `assembly-v3.txt` were produced by that script on rustc 1.100.0, x86_64-pc-windows-msvc. Symbols and assembly line numbers change with toolchain/code changes. These are static optimized library emissions before downstream executable LTO, not sampled profiles. Bounds-panic call-site counts include cold paths, stack-reference counts include prologues/epilogues, and neither count establishes dynamic cost. Inspect the final timed executable before accepting a generated-code optimization.

## Existing evidence

The full CI run, four cells of three repetitions, is recorded in [the earlier performance tables](../audit-2026-11-23/performance.md). The [earlier allocation probe](../audit-2026-11-23/allocation-repro.rs) counts warmed codec allocations with a valid inline executor, excluding pool allocations. Its 14-versus-zero result is an allocation observation, not parallel speed evidence.
