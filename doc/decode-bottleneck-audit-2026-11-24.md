<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Measured small-decode bottleneck and a narrow remedy

Audited Rust `07ba441`, C++ checkout `50f74d2` (its production codec remains the pinned `00f273d`), on 2026-11-24. All code changes were isolated experiments. Maintained production source is unchanged.

## Finding and recommendation

The remaining 64²/128² reduce-2 deficit is **not primarily an unidentified bottleneck elsewhere**. A prepared benchmark of the actual decoded code-blocks reproduces it inside Tier-1 decoding, after excluding wrapper allocation, output-buffer allocation and packet/header setup. Prejoining the real chunks does not remove it; coefficient storage is too small to explain it.

The concrete contributor found is the outlined MQ byte-renormalization helper's **aggregate argument/result transport in the observed Rust 1.100.0 Windows-target executable**. Its 12-byte `Registers` value is passed indirectly, and its result uses a hidden return pointer. The helper also saves/restores eight nonvolatile registers. Source-level by-value semantics did not produce the assumed register-only transport in this build.

A narrow scalar-return prototype keeps the same MQ arithmetic and cold outlined helper, returns only the needed code register/bit count in a `u64`, and computes the interval shift at the caller. Final executable inspection confirms that the aggregate transport disappears. This improves the captured 64²/128² Tier-1 corpora by approximately **6–8%**, and public reduced calls by approximately **4–6% at v3**. Full small requests also improve. Large single/multiworker checks show no robust material slowdown on the tested matrix, although controls and repetitions expose noise/layout sensitivity.

**Recommended first change:** review and validate the [one-file scalar-result patch](decode-bottleneck-audit-2026-11-24/packed-return.patch), then repeat the relevant Linux and release-build gates before adopting it broadly. This is an evidence-backed optimization, not a new arithmetic coder, image-size service, or wholesale decoder rewrite. It does **not yet establish whole-API parity in every row**: the direct public C++ comparison retains roughly 1–2.4% reduced-call deficits.

This report supersedes the earlier [small-decode audit's](small-decode-audit-2026-11-24.md) unperformed matched-benchmark recommendation and its hypothetical reference to another bottleneck. Earlier boundary/MQ experiments remain useful historical evidence; they are not silently reclassified as fixes.

## What was measured

A scratch capture hook at `decode/blocks.rs:99` recorded every real Tier-1 call from the three existing fixtures at reduce 2 and full resolution, including empty blocks. Each record preserves dimensions, numbps, orientation, ROI/style, transform/step, segment lengths/pass counts, **original chunk lengths**, and destination stride/offset. Captures and per-block hashes are saved in [the evidence directory](decode-bottleneck-audit-2026-11-24/README.md).

The primary reduced replay uses:

| Fixture | All blocks / active | Real chunks | Encoded contribution bytes |
|---|---:|---:|---:|
| 32² RGBA 5/3 | 40 / 39 | 39 | 308 |
| 64² RGB 9/7 | 30 / 30 | 80 | 444 |
| 128² RGBA 9/7 | 40 / 38 | 86 | 1,435 |

Rust calls the **original `Scratch::decode` full body** from its prepared runner. Capture hooks are in the tile walk and are not called by timed direct Scratch entries. C++ retains its `T1` scratch, vectors and output buffers and calls `t1_decode_cblk` plus `t1_store`. The original oracle's per-call vector/output allocation wrappers are excluded. Both sides use the same real chunk shapes and destination layouts, poisoned padding/tail buffers, and preallocated storage. Rust's warmed full-mode allocation/reallocation count was zero. Every block's complete output buffer, including padding, matched C++ and an unchanged-source Rust control before and after timing.

Three process repetitions per CPU level used 24 rounds and 128 corpus repetitions per timed batch, rotating all algorithm orders evenly. The packed-result A/B adds the candidate and uses all 24 orders of current/C++/control/candidate. Machine: Windows, Ryzen 9 9950X3D, eight reported available logical CPUs; serial kernel/small tests pinned to CPU 7; Rust 1.100.0, release fat LTO/CGU1; C++ MSVC 19.51.36260, `RelWithDebInfo`, no IPO. Baseline C++ is **SCALAR**, v3 C++ is **AVX2**, matching the Rust CPU level. Pre-window observed other-process load for the primary replay was about 1.74%; audit builds were idle during timing. There is no continuous unrelated-load trace.

This is still a separate measurement context from the whole decoder: caller/iterator monomorphization, downstream LTO and cache/layout can differ. Percentages compare inside their own process. The report does not subtract independent kernel/public medians into an exact measured cost of an unobserved stage.

## 1. The actual Tier-1 body reproduces the deficit

Primary replay, original real chunks, decode **and coefficient store**, median percentage of the repetitions; times are independently reported medians in microseconds:

| CPU | Reduced fixture | Rust µs | C++ µs | Rust vs C++ | Repetition range |
|---|---|---:|---:|---:|---:|
| baseline | 64² | 14.812 | 14.106 | +5.09% | +4.86 to +5.22% |
| baseline | 128² | 47.992 | 44.922 | +6.44% | +5.80 to +7.13% |
| v3 | 64² | 14.576 | 13.708 | +6.35% | +6.08 to +6.86% |
| v3 | 128² | 47.722 | 43.889 | +8.30% | +7.21 to +8.85% |

The unchanged-source replay control is close on these full-corpus rows (about −0.04 to +0.76% medians). The gap therefore survives moving below header/tile/packet setup and below the ordinary oracle wrapper.

The counterfactual with one already-joined chunk still has v3 deficits of +5.35% for 64² and +7.58% for 128². Joining is not the sole cause. These counterfactual cases are separate timed cases, not a precise subtraction of gathering cost.

Store-only prepared raw coefficients take about **0.18 µs for the 64² corpus and 0.47 µs for 128² on Rust v3**, close to C++ in this run. The decode-without-store ablation still has approximately +7.7%/+9.3% gaps. That ablation uses a separately compiled copied body; it supports attribution but cannot make full-minus-no-store an exact store-cost calculation.

Stratification also matters. With original chunks at v3:

| Actual block subset | Rust vs C++, median |
|---|---:|
| 64² fixture, 2×2 blocks | −11.90% |
| 64² fixture, 4×4 blocks | +11.83% |
| 64² fixture, 8×8 blocks | +10.75% |
| 128² fixture, 4×4 blocks | +12.20% |
| 128² fixture, 8×8 blocks | +10.51% |
| 128² fixture, 16×16 blocks | +11.41% |

Grouping changes branch/cache history, so subset times need not add exactly to the mixed-corpus time. It nevertheless rejects a simple claim that all small work suffers equally: partial 2-row blocks can win while full-stripe work loses. Whole-image dimensions alone do not prove which MQ implementation should be selected.

## 2. Sampling and final code identify a specific helper boundary

Kernel WPR profiling was blocked by Windows policy (`0xc5585011`). A preserved user-mode sampler instead created the benchmark process and periodically suspended/read/resumed **its own primary thread**, resolving program-counter samples through the executable's PDB. It collected approximately 6,700 samples for each 20-second 128² reduced run. This perturbs execution and measures instruction-pointer residency, not hardware counters, complete stacks or unbiased per-stage durations.

Rust samples place approximately 80.77% in the inlined pass/driver body and **16.83% in `renormalize_with_bytes`**, with coefficient storage about 0.96%. C++ samples place approximately 40.60% in significance, 34.76% in cleanup and 20.39% in refinement; renormalization is inlined into those C++ pass bodies. Thus Rust's 16.83% must **not** be compared with an imagined zero C++ renormalization cost. The common finding is that arithmetic/pass execution dominates, not allocation or copies.

Rust source locations: `neo-j2k/src/t1/mq.rs:91–113` (`Registers` and helper) and `:237–251` (call); C++ comparison: `src/t1_mq.h` decision/renormalization macros expanded in `src/t1.cpp` significance/refinement/cleanup.

PDB-resolved final executable disassembly supplies the stronger ABI evidence:

| Observed helper property | Current | Scalar-result candidate |
|---|---|---|
| Code size | 283 bytes | 202 bytes |
| Inputs | hidden result pointer; pointer to 12-byte input Registers; Source pointer; shift count | direct scalar c/bits; Source pointer; shift count |
| Result | 12-byte copy through hidden pointer | packed 64-bit result in RAX |
| Callee-saved push/pop pairs | 8, 64 bytes | 5, 40 bytes |

The original code reads c/bits from `[RDX+4]`/`[RDX+8]`, receives the result address in RCX and copies the result through it. The candidate returns c/bits with a shift/OR into RAX and has no Registers input pointer or hidden result pointer. Private helper ranges were resolved from PDB records; objdump's nearest export label misleadingly names `nj2k_audit_run`. The [disassembly, PDB excerpts and hashes](decode-bottleneck-audit-2026-11-24/traces/) preserve that distinction.

This is an **observed internal Rust ABI lowering for this compiler/target/build**, not a permanent language guarantee or proof that the same lowering occurs on Linux. The source comment about by-value register transport is an optimization intention; it should not substitute for final-code verification. The helper's sampled time includes necessary byte work, so the report does not count its entire 16.83% as waste.

## 3. Narrow scalar-result A/B: no new MQ algorithm

The prototype changes only `t1/mq.rs` in a clean public candidate:

- Keep `#[cold]` and `#[inline(never)]`; avoid replicating a byte-input loop into every lane.
- Pass c, bit count, Source and shift count as the required inputs.
- Perform exactly the same byte-input/shift sequence and pack c/bits into one `u64`.
- Compute `a << total` at the caller; restore c and bits from the scalar result.

It does not change context selection, probability states, arithmetic decisions, marker/sentinel rules, RAW behavior, pass scheduling or decoded samples. It does not add per-image/per-decision routing, allocations or unsafe codec operations.

A/B with original chunks and all real blocks:

| CPU | Reduced fixture | Candidate vs current | Range | Candidate vs C++ |
|---|---|---:|---:|---:|
| baseline | 32² | −5.04% | −5.34 to −4.71% | −5.20% |
| baseline | 64² | −6.59% | −6.88 to −6.47% | −2.99% |
| baseline | 128² | −7.57% | −8.37 to −7.23% | −3.73% |
| v3 | 32² | −5.58% | −6.30 to −5.11% | +0.30% |
| v3 | 64² | −5.87% | −6.01 to −4.67% | +0.47% |
| v3 | 128² | −6.73% | −6.91 to −6.56% | +0.21% |

The v3 64² candidate/C++ range is +0.42 to +1.71%, so even captured-kernel parity is not a guarantee for every repetition. Control medians are generally within about half a percent; baseline 64² has a +1.11% control movement. The candidate gains are materially larger. Removing the aggregate boundary is a measured contributor to the deficit, not a claim that every remaining cycle has been explained.

## 4. Public-path and large-image regression checks

The public candidate is a clean copy of maintained source with **only the MQ file changed**, with `bench-internals` disabled. Captured-kernel improvements were then checked through actual decoder APIs.

Three-repetition paired public byte-I32 small requests:

| Reduced request | Baseline candidate vs current | v3 candidate vs current |
|---|---:|---:|
| 32² RGBA | −2.90% | −3.57% |
| 64² RGB | −3.60% | −4.64% |
| 128² RGBA | −5.35% | −6.23% |

All three full small requests also improve at both CPU levels. The separate original C++ gate retains exact sample/status checks and includes missing-EOC cases. Full and missing-EOC requests are faster than C++ in that gate, but reduced whole-API parity is **not complete**:

| Reduced request | Candidate vs C++, v3 median | Range |
|---|---:|---:|
| 32² | +1.26% | +1.15 to +1.50% |
| 64² | +2.36% | +1.88 to +2.54% |
| 128² | +1.06% | +0.62 to +1.41% |

The paired Rust executable and C++-comparison executable have noticeably different absolute full-32 timings despite equivalent intended work. Caller/LTO/code layout is material. Do not multiply percentages from these executables or treat their difference as an exact measurement of the remaining API-stage cost.

Large typed-I32 checks use one persistent shared executor, widths 1/2/4/8, actual eight available CPUs, current/candidate/renamed-control, three repetitions. Medians against current:

| Image | Workers | v3 | baseline |
|---|---:|---:|---:|
| 1,024² RGB | 1 | −4.11% | −0.19% |
| 1,024² RGB | 2 | −4.16% | +0.61% |
| 1,024² RGB | 4 | −5.41% | −1.25% |
| 1,024² RGB | 8 | −4.23% | −1.38% |
| 4,096² gray | 1 | −0.52% | −1.26% |
| 4,096² gray | 2 | −0.75% | −0.82% |
| 4,096² gray | 4 | −4.33% | −2.38% |
| 4,096² gray | 8 | −5.53% | −2.52% |

Those numbers require their controls: several unchanged-source controls also improve 1–4%. For example, v3 gray at width 4 is +0.63% for candidate versus control, not a demonstrated 4.33% algorithmic gain. Baseline RGB width 2 ranges −2.56 to +1.50% versus current; baseline gray candidate/control medians are about +0.07 to +0.91% at widths 1/2/4. Exact results and all ranges are retained. The tested matrix does not show a robust material slowdown, but it does **not** prove zero regression on every platform, content type or build.

## Validation and acceptance

The clean scalar-result candidate passed:

- 66 native tests selected by `--cfg miri`, including MQ model comparisons; this was not an interpreter/Miri run.
- 21 native MQ/Tier-1 C++ differential tests, with three long nightly tests ignored.
- Exact prepared-block samples and poisoned padding/tail equality for original/joined chunks and full/decode-only/store-only modes.
- Public small full/reduced/missing-EOC sample/status checks and large-image equality at all four worker widths.

Recommended implementation sequence:

1. Review the small scalar-result change, intentional packing casts and updated performance comment. Keep the outlined cold path and current full/partial pass kernels; do not bundle unrelated boundary or service work.
2. Run the repository's full required checks, actual Miri, mutation/nightly arithmetic coverage, and Linux baseline/v3 comparison before adopting it there. If an unmeasured ABI/platform regresses, retain its existing helper representation until a measured alternative is established. No architecture-independent speed promise follows from a Windows result.
3. Recheck the residual public 1–2.4% rows in one quiet release build with matched API/lifecycle and an unchanged-code control. If they remain outside the agreed noise band, profile the **post-fix public path** before the next code change. Do not presume a fresh bottleneck elsewhere or repeat already-rejected boundary trims.
4. Consider the earlier short-only path only as a separate, measured candidate after this change; its effects are not additive by assumption. Preserve the existing larger/full-image wins and rerun the same large regression matrix.

Image-size engine categories, additional 256–4,096 size sweeps, and service/batching architectures remain follow-up ideas. No IPC/queue/copy cost was measured, and no service was deployed. They should not obscure the concrete bottleneck and lower-risk in-process remedy demonstrated here.

Maintained production source, dependencies and CI remain unchanged. [Reproduction sources, real captures, raw timing logs, controls, profile evidence and prototype patch](decode-bottleneck-audit-2026-11-24/README.md) accompany the report. No global no-regression or complete whole-API parity claim is made.
