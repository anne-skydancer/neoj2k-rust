<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Safe Rust conversion checkpoint and performance audit 2026 11 24

The production `neo-j2k` crate in the isolated `safe-simd` worktree now compiles with `#![forbid(unsafe_code)]`, has no unsafe source constructs or module exceptions, and retains zero runtime dependencies. Its public API, stable Rust 1.100 requirement and default features are unchanged. **The performance requirement is not yet met:** the conversion is a working candidate, not an accepted replacement for main. Linux measurements retain small-image v3 slowdowns and a baseline lossy-encode concern despite substantial baseline decode improvements. Nothing was committed or pushed.

This is the **frozen initial conversion checkpoint**, before the subsequently tested cold `round_general` helper and packing refinements. The owner requested continued zero-dependency performance tuning. The live worktree is changing; the measurements, hashes and validation counts below describe the frozen checkpoint and do not certify later changes.

Reference: main `d13082f`, including PR #21's scalar-result MQ change. This comparison is against that Rust implementation, not C++ or Rust 0.1.2. Existing user reports and production main were left unchanged.

## Implementation

- [`lib.rs`](../neo-j2k/src/lib.rs) forbids unsafe code outright. [`check_rust_native.py`](../tools/check_rust_native.py) no longer exempts `src/simd/`; its self-test rejects unsafe blocks, functions and implementations there too. README and current architecture/benchmark documentation describe the new policy.
- [`simd/lanes.rs`](../neo-j2k/src/simd/lanes.rs) expresses fixed lane arithmetic, bit reinterpretation and permutations as safe arrays. DWT and Tier-1 stores use checked slices and array copies instead of raw-pointer intrinsic loads/stores. Wrapping integer arithmetic, floating operation order, tails and strided padding are retained. No unsafe code was moved into another crate or FFI wrapper.
- [`simd/mct.rs`](../neo-j2k/src/simd/mct.rs) uses safe fused colour/finish kernels. For representable component bounds, it clamps floats first, uses an exact nearest-even bias/mantissa conversion, and hoists fast-path eligibility outside the sample loops. Ordered comparisons map NaN to the lower endpoint. Wider precision retains the general sentinel/clamp rules. These safe lane kernels also run on the baseline SSE target, where final measurements show useful gains.
- [`simd/mod.rs`](../neo-j2k/src/simd/mod.rs) replaces scalar SSE conversion with a bounded bias/mantissa fast path and a general safe rounding fallback. It preserves `i32::MIN` for NaN and unrepresentable conversion, distinct from the decoder's positive-overflow clamp. [`round_tests.rs`](../neo-j2k/src/simd/round_tests.rs) checks half ties, adjacent floats, signed endpoints, exponents and random bit patterns.
- [`encoder.rs`](../neo-j2k/src/encoder.rs) quantizes four gathered coefficients together, checks their fast-path range without short-circuiting individual lanes, and falls back to the same scalar conversion when any lane needs it. [`quantize_tests.rs`](../neo-j2k/src/encoder/quantize_tests.rs) compares grouped and scalar conversion across ties, exceptional step sizes, mixed lanes and float bit patterns.
- [`simd/output.rs`](../neo-j2k/src/simd/output.rs) packs one to four channels through zipped slices and fixed-size pixel stores. The existing eight-pixel completion contract and caller tail handling remain intact.

The existing DWT level thresholds, public scheduling and tile layout are unchanged. The retained safe transpose is outlined and its lifting helper is inlined because measurements found this better than the initial fully flattened safe-array transpose. This still leaves a measured performance gap; it is not a claim that the compiler reproduced every original intrinsic instruction.

## Aurora reference

The supplied [Aurora Viewer aurora-j2k](https://github.com/Aurora-Viewer/Aurora-Viewer/tree/1ea0749c798daf81ccb059de74557b862c9e2340/crates/aurora-j2k) was inspected at pinned commit `1ea0749c798daf81ccb059de74557b862c9e2340`. It forbids unsafe code and has no runtime dependencies. Its [MCT source](https://github.com/Aurora-Viewer/Aurora-Viewer/blob/1ea0749c798daf81ccb059de74557b862c9e2340/crates/aurora-j2k/src/mct.rs) provides directly applicable patterns: hoisted `FastShift`, ordered clamp followed by the nearest-even mantissa bias, fused finishing and per-channel fixed-pixel packing. Those patterns informed the implementation above, with attribution in the affected modules and existing neo-j2k/OpenJPEG notices retained.

Aurora's DWT also changes where resolution rows live and avoids the old eight-row transpose through a different row schedule. That is a broader storage/decoder integration change, not a drop-in intrinsic wrapper. Its decoder-only scope does not resolve neo-j2k's encoder conversion by itself. This candidate retains the current codec API, encoder, JP2 handling and limits rather than substituting the narrower Aurora crate.

Compiler probes explain the implementation choice: on the pinned Rust compiler, globally enabling AVX2 does not make architecture-intrinsic calls from an ordinary function safe, and `std::simd` remains unstable. [Probe sources and diagnostics](safe-rust-audit-2026-11-24/compiler-probes.txt) are preserved. This establishes limitations of those direct approaches, not an impossibility proof for other safe algorithms.

## Controlled Linux measurements

Arch Linux under WSL2 on the existing Ryzen 9 9950X3D machine; Rust 1.100.0, release fat LTO, one codegen unit, default features, **no `bench-internals` stage clocks**. One executable links the candidate, untouched main and an independently named byte copy of main as a code-layout control. All three share one persistent test-harness pool for large requests. Each row uses 24 rounds, all six execution orders rotated, with warm decoders and caller buffers. Three independent process repetitions were run per CPU level.

The 32 rows cover eight large decode/worker cases, nine small whole/reduced/missing-EOC cases, six encodes, eight packed-U8 cases at one to four channels and two reductions, and a large packed-RGB case. Packed targets have an unaligned prefix and row padding. Warm decoded planes, complete packed buffers and encoded codestreams were identical across all three builds. Truncation, error recovery and broader layout correctness are additionally covered by the native test suites; the timing harness is not a complete semantic test.

Values below are candidate versus main, followed by unchanged control versus main. Minus is faster. Each figure is the median of the three processes' own median within-round percentage changes; it is not calculated from unrelated absolute-time medians.

| Request | Baseline candidate / control | v3 candidate / control |
|---|---:|---:|
| 1,024² RGB, one worker | −21.53% / −0.34% | −2.45% / +0.05% |
| 32² RGBA, reduce 2 | +0.49% / −0.78% | +0.32% / −0.29% |
| 64² RGB, whole | −6.96% / +1.96% | +7.28% / −2.90% |
| 128² RGBA, whole | −15.44% / −1.42% | +2.89% / +0.27% |
| 32² RGB, lossy encode | +4.38% / +0.09% | +0.30% / −0.99% |
| 128² RGBA packed, four channels | −12.84% / −0.12% | +1.40% / −1.51% |

[All 32 rows, ranges and controls](safe-rust-audit-2026-11-24/linux-summary.md) and [machine-readable results](safe-rust-audit-2026-11-24/linux-summary.json) accompany the individual raw logs. Some unchanged controls move materially, including roughly 10% on large gray v3 rows. Those rows are inconclusive for strict no-regression acceptance. Neither control movement nor a gain elsewhere excuses a losing target row or proves an algorithmic cause for a noisy row.

Three further seven-row residual runs pinned the process to Linux CPU 7, without multithreaded cases. The v3 64² whole slowdown persists at **+7.61%** (range +6.21 to +8.44%, control median −0.39%), and 128² whole at **+3.86%** (+2.87 to +5.95%, control median +0.60%). Baseline 32² lossy encode remains +3.26%, although its control is −2.41% and is not quiet. [Pinned results and all control ranges](safe-rust-audit-2026-11-24/linux-residual-pinned-summary.md) are retained. These results prevent a claim of preserved performance.

## Attribution and rejected experiments

A separate three-way stage-instrumented executable, used only for attribution, found the 64² whole v3 wavelet stage at about 11.61 µs for the retained safe candidate versus 8.75/8.86 µs for main/control. Colour finishing improved from about 0.98/0.99 to 0.66 µs. On 128² whole, wavelet was about 45.45 versus 41.28/41.37 µs, while colour improved from about 3.81/3.80 to 2.60 µs. Timers perturb these calls; stage differences must not be subtracted from separately compiled public medians as exact cost accounting.

Final-code inspection showed the first inline safe-array transpose flattening into scalar loads/spills. Outlining the eight-row transpose and inlining its lifting helper improved that version, but [the retained safe transpose assembly](safe-rust-audit-2026-11-24/linux-v3-safe-transpose.asm.txt) still contains a less efficient shuffle sequence than the intrinsic reference. This is concrete code-generation evidence for the remaining wavelet investigation.

The following isolated experiments were rejected and are absent from the retained worktree: lowering the row-wise 9/7 threshold to 32²; replacing larger horizontal lane groups with per-row lifting; outlining the four-row transpose stage; and routing small 9/7 levels through the existing portable row and eight-column kernels. The last variant increased 64² whole wavelet time to about 13.91 µs and 128² whole to 49.13 µs; its [stage log](safe-rust-audit-2026-11-24/stages-portable-experiment-v3.txt) is saved. No whole-image dimension switch or broad layout rewrite was adopted to conceal a regression.

## Validation and remaining work

Linux release C++ differential/unit suites passed: **99 baseline and 102 v3 tests**, with seven long/nightly tests ignored in each. [Baseline log](safe-rust-audit-2026-11-24/native-baseline.txt), [v3 log](safe-rust-audit-2026-11-24/native-v3.txt). Windows baseline and v3 full release workspace suites also passed with matched newly built MSVC SCALAR/AVX2 oracles. Formatting and all-target Clippy passed for baseline, v3 and decoder-only configuration. The native-policy checker confirms the production dependency closure remains one crate. Earlier actual-Miri tests passed for the safe-array conversion; final touched-kernel Miri revalidation belongs with the final candidate's checks, not the timing runs.

The remaining acceptance work is specific: reduce safe small-level wavelet transport/shuffle cost without losing the safe arithmetic and memory-budget behavior; settle the baseline lossy-encode concern with quiet controls and quantizer attribution; and repeat the full platform/CPU matrix after any change. The Aurora row-layout approach is a possible larger measured follow-up, not an already-validated fix for this crate. The strict zero-dependency and performance requirements have not both been demonstrated by the current candidate.

[The comparison harness](safe-rust-audit-2026-11-24/harness/) and [preparation script](safe-rust-audit-2026-11-24/prepare_compare.py) reproduce the public-API matrix outside the repository. The script uses a fresh destination and materializes the pinned main/control snapshots while pointing at the candidate checkout. Set `NJ2K_ORACLE_DIR`, use distinct target directories for baseline and `RUSTFLAGS="-C target-cpu=x86-64-v3"`, and run the release executable three times per cell. `SAFE_AUDIT_RESIDUAL=1` selects the seven focused residual rows. Keep stage profiling separate from acceptance timing. [Build/source hashes](safe-rust-audit-2026-11-24/linux-build-manifest.json), [timing provenance](safe-rust-audit-2026-11-24/linux-timing-manifest.json) and raw logs identify the measured snapshot.
