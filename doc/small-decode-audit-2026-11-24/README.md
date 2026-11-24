<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->
# Reproduction and evidence

All source experiments began at Rust `07ba441`, C++ checkout `50f74d2`; the C++ production source is unchanged from pinned `00f273d`. Maintained source was not modified. The local machine was Windows, Ryzen 9 9950X3D, eight reported logical CPUs, rustc/cargo 1.100.0, MSVC oracle, release fat LTO and one code-generation unit for paired/public-API benchmarks. API benchmarks use default codec features and have no `bench-internals` timers. V3 uses `RUSTFLAGS=-C target-cpu=x86-64-v3`; baseline removes that variable.

## Files

- `short-path.patch`: current-code prototype, retaining current Row/Cell and MQ algorithms; specializes height<=4. Native unit and C++ differential checks passed. Not a production patch ready for approval without the report's remaining validation.
- `fast-mps.patch`: deliberately unshipped negative experiment. Global use badly regresses full 32² lossless.
- `short-probe.rs`: paired current/candidate public byte-API harness. Package aliases `current` and `before` name the unmodified and short-path codec crates.
- `four-way-probe.rs`: current, pre-eaea (`05e7fb3` Tier-1 files), that old source with SINGLE disabled, and current with fast-MPS shortcut. Package aliases `current`, `before`, `general`, `fast`.
- `candidate-gates.rs`: exact repository small-image-gates test with `use before as neo_j2k` added, so the ordinary C++ shim and its exact output checks compare against the scratch candidate.
- `three-way-v3-*`, `four-way-v3-*`: first historical isolation and subsequent MQ experiment. Three-way used 31 cyclic rounds; four-way 32 cyclic rounds; 128 calls per measured batch. Code layout changed between binaries, so compare inside one executable.
- `current-short-v3-*`, `current-short-baseline-*`: 32 alternating rounds x128 calls, three process repetitions, current and candidate in one executable.
- `candidate-cpp-v3-*`: three repetitions of 21 alternating rounds x16 calls from the normal gate, candidate versus C++ directly.
- `current-windows-v3-gates-*`: the same normal gate for unchanged current Rust versus C++, freshly built at current source.
- `current-windows-v3-stage-*`: two instrumented current-code profiles,31 rounds x200 calls. Stage scope cost 46 ns in the second repetition. First 32² byte profile noisy; retain both logs.
- `geometry.txt`: active block dimensions and real coding-pass counts. Instrumentation is at `decode_passes`, after an input chunk exists; empty blocks are excluded. `geometry-probe.rs` drives one owned decode per image/reduction.
- `native-selected-unit-tests.txt`:66 native tests compiled with `--cfg miri`, NOT a Miri interpreter run.
- `native-cpp-differential-tests.txt`:8 current Tier-1 native oracle comparisons passed;2 long nightly sweeps ignored.

The parent audit process measured other-process load 2.14% immediately before the unchanged-current C++ benchmark. Scratch timings ran with parent/compiler work idle; no contemporaneous load trace or unchanged-code byte-copy control was collected for every scratch repetition. Package/code placement and time drift may affect small differences. The report does not classify the 1% adverse cases as proved noise.

## Rebuild the direct candidate-versus-C++ experiment

Create an isolated checkout/worktree of Rust `07ba441`, apply `short-path.patch` there, and leave the maintained checkout untouched. For example, with paths adjusted to an unused scratch directory:

```powershell
git worktree add --detach C:/audit-scratch/neoj2k-short 07ba441
git -C C:/audit-scratch/neoj2k-short apply C:/Users/annes/source/repos/neoj2k-rust/doc/small-decode-audit-2026-11-24/short-path.patch
$env:NJ2K_CPP_SOURCE='C:/Users/annes/source/repos/neo-j2k'
$env:CARGO_TARGET_DIR='C:/audit-scratch/target-short'
$env:RUSTFLAGS='-C target-cpu=x86-64-v3'
cargo test --manifest-path C:/audit-scratch/neoj2k-short/Cargo.toml --release --locked -p neo-j2k --test small_image_gates --no-run
```

Run the resulting `small_image_gates-*.exe` three times with `--ignored --nocapture --exact small_image_median_latency`. The actual local runs restricted the dedicated benchmark shell and its children to CPU 7 with `[Diagnostics.Process]::GetCurrentProcess().ProcessorAffinity=128`. Use a valid CPU on your machine. Pinning this serial small benchmark is appropriate; do not use that one-CPU restriction for multithread scaling tests. The existing test alternates both codecs, warms reusable decoders/output buffers and verifies exact samples/status before timing.

Run the unmodified checkout identically for context, without multiplying independently obtained percentage changes to claim parity. Baseline-versus-scalar-C++ needs a scalar C++ build; the default Windows oracle is AVX2. Local baseline paired Rust-versus-Rust experiments are valid without a C++ baseline comparison.

## Rebuild a paired current/short probe

The checked-in `short-probe.rs` is `src/main.rs` of a temporary Cargo package. Its dependencies are path aliases of the original crate and a scratch copy with `short-path.patch`. For this paired probe, copy the codec into a standalone scratch package (expand workspace-inherited manifest values to their original values and omit test-only dev-dependencies). Rename that scratch package to `neo-j2k-current-short` to make both package identities distinct, keeping its library name `neo_j2k`; preserve features and source. The package manifest is:

```toml
[package]
name = "small-decode-short-probe"
version = "0.0.0"
edition = "2024"
[dependencies]
current = { package = "neo-j2k", path = "PATH/TO/ORIGINAL/neo-j2k" }
before = { package = "neo-j2k-current-short", path = "PATH/TO/SCRATCH/neo-j2k" }
[profile.release]
lto = "fat"
codegen-units = 1
panic = "unwind"
debug = "line-tables-only"
```

The probe source's fixture path is explicit for this workspace; adjust it on another computer. Build release once at each CPU level, then repeat the executable three times in a quiet pinned window. It compares all output bytes before timing and uses the same lifecycle/API for both implementations. It has no unchanged-code layout control; add one for a production decision near the noise band.

The historical comparison used current surrounding code and coherent `t1.rs`, `t1/mq.rs`, `t1/passes.rs` from `05e7fb3`. The `general` crate differs from that source only by `let single_stripe = false` instead of `height <= 4`. The `fast` crate is current code plus `fast-mps.patch`. These are source-isolation probes, not released historical builds.

## Validation commands

On the short scratch checkout with its regular oracle:

```powershell
cargo test --release --locked -p neo-j2k --lib t1::differential -- --test-threads=1
```

This ran 8 tests and ignored 2 nightly sweeps. The separate native selected-unit run used `RUSTFLAGS='--cfg miri -C target-cpu=x86-64-v3'`, `NJ2K_ORACLE_NO_BUILD=1`, a separate target, and `cargo test -p neo-j2k --lib`; 66 passed. It checks selected logic with native execution and overflow checks; it provides no interpreted UB/race verification. Actual Miri, mutation checks, Linux measurements and full supported geometry/style coverage remain required for a production change.

## Expanded boundary and larger-reduced investigation

The later user-requested extension is preserved under [boundary-work](boundary-work/README.md), [Tier-2 segment retention](tier2-trim/README.md), and [chunk inspection](chunk-inspection/README.md). Original results above remain unchanged. The extension adds baseline/v3 cyclic-order probes with renamed unchanged-source controls, boundary store/init/side/refinement variants, conditional MQ modes and bounded width kernels. It rejects the measured broad trims as parity fixes and gives an exact prepared real-block benchmark specification for the unresolved 64²/128² target. Prototype patches are evidence, not production recommendations.
