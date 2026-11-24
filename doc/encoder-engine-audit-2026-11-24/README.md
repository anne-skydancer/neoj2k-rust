<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Encoder audit evidence and reproduction

Source: Rust `07ba441`, adjacent C++ `50f74d2`. Maintained source was unchanged. Scratch root was `$env:TEMP/codex-encoder-engine-audit`.

`single.patch` adds only the one-node tag-tree shortcut. `bound.patch` changes the path array from 65 slots to 7. `combined.patch` contains both; it is the candidate used for the native oracle/stream/update/executor validation. These are experimental patches, not approval to merge them. The seven-node prototype should derive its bound from validated encoder geometry instead of shipping an unexplained magic constant.

## Timing

`tagtree-probe.rs` is the temporary Cargo package's `src/main.rs`; `probe-Cargo.toml` records exact path aliases and release profile. Copy the codec into standalone packages named `neo-j2k-control`, `neo-j2k-bound`, `neo-j2k-single`, and `neo-j2k-combined`, expanding workspace manifest fields without changing values. Keep library name `neo_j2k` and original features. Apply each patch only to its scratch copy; control is an unchanged-source renamed package. Adjust absolute paths in the manifest to your workspace.

Build release with Rust 1.100.0, fat LTO, one codegen unit, then repeat with `RUSTFLAGS=-C target-cpu=x86-64-v3`. The original current crate and four scratch packages are linked in one executable. All fixture codestream bytes are compared before timing. Each case has three warmup calls; 20 cyclic-order rounds rotate five implementations through five positions. Batch sizes are 8 calls for 32², 2 for 128², and 1 for 256²/512². Three process repetitions were taken at each CPU level. Cold means a new encoder per invocation with pixels/params already prepared; warm reuses the encoder. No executor is used in these timings.

Machine: Windows, AMD Ryzen 9 9950X3D, 8 reported logical CPUs. The dedicated shell/children were restricted to CPU 7 (`[Diagnostics.Process]::GetCurrentProcess().ProcessorAffinity=128`). Other audit compilation and profiling were idle during windows. No continuous system-load trace or unchanged executable byte-copy control was collected. The renamed unchanged-source package measures code-placement effects; it cannot perfectly remove them. Several rated control rows move by 1–2%, so results are leads rather than a certified regression gate.

`tagtree-{baseline,v3}-[123].txt` are raw per-case medians in microseconds. The JSON summaries calculate each repetition's candidate/current and candidate/control percentage, then report median/min/max; they do not divide separately aggregated medians. All full raw rows, including adverse ones, are retained.

## Validation

A scratch workspace copied `neo-j2k`, `neo-j2k-oracle`, `neo-j2k-testkit`, `neo-j2k-bench`, corpus and workspace manifests from 07ba441; applied `combined.patch`; linked the prebuilt current MSVC oracle with `NJ2K_ORACLE_DIR`; used `RUSTFLAGS=-C target-cpu=x86-64-v3` and a separate `CARGO_TARGET_DIR`.

```
cargo test --release --locked -p neo-j2k --lib encoder:: -- --test-threads=1
cargo test --release --locked -p neo-j2k --test encode_stream --test encode_updates --test encode_executor -- --test-threads=1
```

Logs `candidate-encoder-unit.txt` and `candidate-integration.txt`: 22 passed, 3 long ignored; then 8 passed. These cover current C++ byte/pass/decision comparisons, caps/refusals/reuse, stream offers/failures, parameter updates and threaded output identity. They are native tests, not Miri.

`bound-domain.rs` wraps the actual candidate coding module with minimal allocation helpers; `coding-domain.rs` adds a test of 144 representative width/height pairs, including 64×64 maximum leaf geometry. 60,516 leaf paths fit seven entries and remain within nodes. It does not test every possible geometry or constitute a full 4096² public API differential test. Adjust its explicit include path before running:

```
rustc --test bound-domain.rs -O -o bound-domain.exe
./bound-domain.exe --nocapture
```

`assembly-baseline-summary.txt` extracts the current optimized library's encoder Tier2 code shape. Reproduce emission with a separate target directory:

```
cargo rustc --release --locked -p neo-j2k --features bench-internals --lib -- --emit=asm
```

This is pre-final-LTO library emission; verify a proposed production change in the final executable too. Static memcpy/memset/check sites do not measure executed counts or wall-time contribution.
