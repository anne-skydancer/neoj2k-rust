<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Audit of the remaining local Linux decode slowdowns (2026-11-24)

Audited Rust `dd7d3f4`; the codec is unchanged from the benchmark's `40ee9b4` (the intervening commit adds only its report). Scope: the slower rows in [the local Linux benchmark](local-linux-bench-2026-11-24.md), not an encoder rewrite or a general rollback. The adjacent C++ production reference is `00f273d`. No production code, existing audit, dependency or workflow was changed. No commit or push was made.

**First implementation recommendation:** validate the existing scalar-result MQ renormalization patch on the **current, post-short-block** decoder. The source still passes a 12-byte aggregate through the outlined helper, and this audit directly confirms indirect aggregate transport in retained Linux release executables at both CPU levels. That closes the earlier audit's Linux ABI evidence gap; it does **not** establish the candidate's Linux speedup. Separately investigate the repeatable four-worker RGB result through measured row-band balance and wavelet barriers. Do not bundle a scheduler change with the MQ patch.

## 1. Which slowdowns need resolving

The four original benchmark tables were read from `/home/dev/bench-out/` in WSL's `archlinux` distribution and copied unchanged into [this audit's evidence directory](local-linux-slowdown-audit-2026-11-24/). The default WSL account is root; the benchmark report's `~/bench-out` refers to the `dev` account.

| Request | Baseline vs C++ | v3 vs C++ | Disposition |
|---|---:|---:|---|
| 32² RGBA 5/3, reduce 2 | +2.9%, spread +0.5 to +3.4% | +3.4%, +3.0 to +3.5% | Persistent small deficit; highest-priority version regression too |
| 64² RGB 9/7, reduce 2 | +6.3%, +3.2 to +9.7% | +8.7%, +7.6 to +8.8% | Clearest larger small-request deficit |
| 128² RGBA 9/7, reduce 2 | +5.9%, +1.8 to +6.1% | +0.7%, −2.7 to +8.0% | Baseline deficit; v3 is within noise, not a proved regression |
| 1,024² RGB 9/7, four workers | −29.8% | +2.8%, +2.0 to +5.4% | Secondary v3 deficit; all three repetitions are positive |

Reduce 2 gives output sides 8, 16 and 32 respectively, not one quarter of the original sample count. Against Rust 0.1.2, only the 32² reduced request is a demonstrated small-decode regression: +9.6% baseline and +8.2% v3. The 64² +3.9%/+7.6% ranges cross zero; 128² is 30–34% faster. These are separate comparisons and must not be conflated.

The v3 four-worker row's independently reported median times are Rust 2.595 ms and C++ 2.515 ms. Its roughly 0.080 ms separation indicates the scale of the problem, not an exact stage cost or the derivation of the reported +2.8%: that percentage is the median of within-repetition ratios. A new quiet session and unchanged-source control are needed before attributing it to code, but the saved results do not support dismissing it as already within noise.

## 2. Source reconciliation: what is already fixed and what remains

| Area and source at audited HEAD | Source-verified observation | Consequence |
|---|---|---|
| [`t1.rs:212`](../neo-j2k/src/t1.rs#L212), [`passes.rs:87`](../neo-j2k/src/t1/passes.rs#L87) | PR #19 already dispatches height ≤ 4 to `decode_passes::<true>` and initializes only its real flag row | Do not propose restoring the short-block path again. Earlier audits' instruction to consider it is historical |
| [`mq.rs:82`](../neo-j2k/src/t1/mq.rs#L82), [`:93`](../neo-j2k/src/t1/mq.rs#L93), [`:237`](../neo-j2k/src/t1/mq.rs#L237) | `Registers { a, c, bits }` still enters and leaves `renormalize_with_bytes` as an aggregate | The packed scalar result remains a live, unlanded candidate |
| [`t1.rs:183`](../neo-j2k/src/t1.rs#L183) | Single incoming chunk is borrowed; multiple chunks are copied into retained `coded` storage | Empty-chunk filtering is not a solution to the actual fixtures; their captured chunks were all nonempty |
| [`decoder.rs:739`](../neo-j2k/src/decoder.rs#L739) | Cached headers are reused after prefix/limit checks, but reduction choice and component dimensions are recomputed | No basis for claiming every warmed request reparses the complete main header; residual checks are measurable candidates only |
| [`decode.rs:818`](../neo-j2k/src/decode.rs#L818), [`dwt.rs:209`](../neo-j2k/src/dwt.rs#L209) | Small requests have no executor by default; parallel inverse work additionally requires a component area ≥ 262,144 | Scheduler overhead does not explain these small reduce-2 rows |
| [`decode.rs:577`](../neo-j2k/src/decode.rs#L577) | Three-component large RGB selects `inverse_staged` at workers 2, 4 and 8 | It is incorrect to explain four-worker loss simply as three components leaving the fourth thread idle: Tier-1 is split across components |
| [`blocks.rs:197`](../neo-j2k/src/decode/blocks.rs#L197) | Multicomponent Tier-1 uses two row bands per worker through width 4, four bands per worker above width 4 | This discrete granularity change is a specific four-versus-eight-worker hypothesis worth testing |
| [`stage.rs:5`](../neo-j2k/src/stage.rs#L5) | `bench-internals` enables clocks and process-wide atomic totals; ordinary production builds contain neither | Use stage builds for attribution, then measure acceptance without the feature; totals across worker threads are not elapsed critical-path time |

`git apply --check doc/decode-bottleneck-audit-2026-11-24/packed-return.patch` succeeds against audited HEAD. The open-PR query returned no open PRs during this audit. No existing branch/PR was found that already ships that candidate.

## 3. New Linux final-code evidence for the MQ interface

Inspected the retained `small_image_gates` executables under `/home/dev/target-baseline/` and `/home/dev/target-v3/`, using `nm -S -C` and bounded `objdump -d -Mintel` ranges. [The manifest](local-linux-slowdown-audit-2026-11-24/binary-manifest.json) records full paths, SHA-256 hashes, file timestamps and symbol sizes. [Baseline disassembly](local-linux-slowdown-audit-2026-11-24/baseline-mq-renormalize.asm.txt) and [v3 disassembly](local-linux-slowdown-audit-2026-11-24/v3-mq-renormalize.asm.txt) are preserved.

These are retained binaries in the benchmark build directories, with timestamps consistent with their saved runs. The original run logs did not freeze executable hashes: the new hashes identify **what was inspected now**, and do not cryptographically prove those exact bytes produced the earlier timing tables.

| Property in retained Linux executable | Baseline | x86-64-v3 |
|---|---|---|
| Named helper | `neo_j2k::t1::mq::renormalize_with_bytes` | Same |
| Address / size | `0xd8910`, 266 bytes | `0xe13a0`, 260 bytes |
| Aggregate input | Loads/writes through RSI, fields at +0/+4/+8 | Same |
| Aggregate result | Writes through RDI, including +8 | Same |
| Saved nonvolatile registers on byte path | RBX, RBP, R12–R15: six push/pop pairs | Same six pairs |

Concrete v3 instructions include `e13aa: mov r8d,[rsi+8]`, `e1403: mov [rsi+4],r15d`, `e148f: mov [rdi+8],r8d` and `e1496: mov [rdi],rax`. Baseline likewise writes the result at `d8a0f`/`d8a16`. Thus both Linux builds materialize aggregate state in memory; the source comment that by-value transport keeps the copies in machine registers is not a valid description of these generated helpers. The necessary `Source` update remains mutable memory and is not claimed removable.

This supports the **mechanism**, not its entire cost: six saves, loads and copies are not all independently avoidable, and a call ABI alone does not predict total decode latency. The earlier [Windows bottleneck audit](decode-bottleneck-audit-2026-11-24.md) supplies the measured candidate experiment: approximately 6–8% faster captured 64²/128² Tier-1 corpora, and roughly 3–6% faster public reduced requests, depending on CPU level/request. Those experiments predate PR #19. Neither those gains nor the short-path gains are additive by assumption, and Windows percentages must not be multiplied into Linux C++ ratios to claim parity.

## 4. Prioritized actionable changes

### P1 — replace aggregate MQ transport with the narrow scalar-result interface

**Implementation:** start from [the existing one-file prototype](decode-bottleneck-audit-2026-11-24/packed-return.patch), applied to current main in an isolated change. Keep the helper cold and outlined. Pass `c`, `bits`, `Source` and shift count; return `(bits << 32) | c` in `u64`; update `a << total` in the caller. Keep arithmetic decisions, state tables, RAW handling, segment/pass scheduling, markers and sentinel behavior unchanged. Update the performance comment to describe observed intent instead of promising an ABI lowering.

**Why first:** this directly targets a demonstrated Windows contributor whose aggregate mechanism is now also seen on Linux. The actual reduced-block replay excluded normal header/packet setup and still reproduced the 64²/128² deficit; joining chunks first did not remove it. This is stronger evidence than a speculative sweep of allocations or DWT kernels.

**Proof obligations:** decoded bits, next context, all MQ registers and source position must match the original across boundary intervals, zero remaining bits, repeated byte-in, stuffed `0xff`, marker-adjacent values, exhausted input and malformed/overlong segments. The intentional `u64`/`u32` packing casts must be documented and reviewed under the crate's lint policy. Inspect the candidate's **final executable**, including callers, to establish that the input aggregate and hidden result pointer disappeared without simply introducing larger caller spills. Keep scalar-result portability separate from a guarantee of register allocation on every compiler/target.

**Expected scope and risk:** all MQ-coded blocks, not just reduced images; minimal source diff and no new API or unsafe operation. Performance/code-layout effects can therefore touch large and full small decodes too. Public parity is an acceptance result to measure, not a promised outcome. If Linux does not improve beyond controls, do not ship it there solely on Windows measurements; assess the simpler portable implementation and platform policy separately before introducing conditional code.

### P2 — make the performance decision cover all three reduced fixtures

[`tools/perf_summary.py:57`](../tools/perf_summary.py#L57) names only `small/sl_32_rgba_53_reduce2` in `SMALL_TARGET_ROWS`. [`small_verdict:546`](../tools/perf_summary.py#L546) requires the other small rows merely not to regress. A green `pair_side=small` gate therefore **does not prove the 64²/128² deficit was fixed**, and C++/Rust-version comparison tables are measurements rather than blanket parity gates.

For a general MQ proposal, explicitly select all three reduced rows as targets in the experimental gate, or make the target list an input with the existing short-path default retained. Add focused parser/verdict cases demonstrating that a flat 64²/128² target cannot be silently counted as improved. Preserve the existing per-row unchanged-source control and 1% floor; an out-of-band control makes a performance claim inconclusive and should cause a rerun, not a wider permissive band.

An optimization may be useful without closing every gap. If only some target rows improve, state that narrower achieved scope and require no regression elsewhere; do not label it a complete remedy for this audit. Require the direct Rust/C++ table separately before claiming parity, and the Rust-version table separately before claiming that the 32² regression is resolved.

### P3 — attribute and then tune the four-worker RGB path

The source-specific suspects are concrete but **not yet measured causes**:

1. `parallel_decode_tile` at [`blocks.rs:189`](../neo-j2k/src/decode/blocks.rs#L189) builds/zeros a fresh row-estimate vector, estimates cost as `100 * bytes + samples` for coded blocks, cuts at safe nonoverlapping row boundaries, and sorts tasks by predicted cost. At four workers it allows eight bands; at eight workers it allows 32. C++ `Tile::plan_t1` (adjacent C++ checkout, `src/tile.cpp:607`) cuts contiguous **code-block** runs using that byte/sample weight and two runs per worker. Rust gives empty blocks a lower sample weight and its row boundaries can constrain balancing differently; similar coded-block estimates do not imply identical schedules.
2. `inverse_staged` at [`decode.rs:631`](../neo-j2k/src/decode.rs#L631) creates a small plane vector each request, performs pooled Tier-1, invokes `inverse_coarse_wavelets` at [`:1167`](../neo-j2k/src/decode.rs#L1167), then iterates components for finer inverse DWT. Coarse work can run across three planes; larger levels use row/column batches with synchronization. At width four, tail work and barriers may matter differently than at width eight.

**First experiment:** wrap the existing persistent `Executor` in a benchmark-only recorder. Record each batch's count, elapsed wall time, per-job duration, predicted cost and component/row range. Store records in preallocated per-job slots and collect after the batch; avoid a shared logging lock/clock per MQ decision. Run a separate instrumented attribution executable, not the acceptance timer. Compare slowest-job/mean ratios, idle tails, batch counts and coarse/fine DWT time at workers 1/2/4/8. Sample allocations once per warmed request outside timing.

**First bounded candidate if Tier-1 tails dominate:** independently test 2/3/4 bands per worker at width four, preserving safe disjoint row slices, error replay and `MAX_JOBS` bounds. Retain larger/smaller image and worker-width controls. Do not change the byte weight too until recorded task durations show it mispredicts work; do not substitute raw per-block parallel writes that alias row slices. If barriers dominate instead, independently test batching independent component wavelet tasks at eligible levels, preserving each plane's horizontal/vertical dependency and fallback semantics. If setup allocations dominate, test retention of planning buffers with correct resource accounting. Each is a separate A/B, not three simultaneous changes.

**Acceptance:** first reproduce the +2.8% result in new sessions; then improve the four-worker row beyond the unchanged-source control without surrendering w1/w2/w8 or gray-image wins. Do not require all four workers busy in a tiny coarse level when serial execution is faster. The public report's C++ gate already shares a persistent pool (`small_image_gates.rs:363`); proposing persistent threads alone is not a new fix. The version comparator's per-call scoped-thread lifecycle is a different measurement.

### P4 — profile residual fixed overhead after P1; make only a measured trim

If small reduced requests remain outside the agreed band, rerun [`stage_profile.rs:159`](../neo-j2k/tests/stage_profile.rs#L159) on the **post-P1 current source**, for byte-I32 and typed-I32 APIs, reduce 2 and full. The 32² output contains only 256 samples across four components; even small fixed costs are worth measuring, but past independent kernel/public medians cannot be subtracted into exact setup time.

Two bounded implementation candidates, only if that attribution supports them:

- If cached-header/check work dominates: skip `fill_components` in `read_header_untimed` when the computed reduction and already-materialized component dimensions are unchanged. Continue validating the input prefix/limits and calling reduction policy as required for equivalent warnings. Re-test streaming resize, changed reduction/limits, selected components, malformed input, status/diagnostics and failed-request recovery. Do not cache whole decode results or bypass per-request limits.
- If output work dominates: inspect `finish_target_decode` (`decoder.rs:617`) and packed I32 rows (`decode/output.rs:198`). First measure whether the compiler already lowers the contiguous byte loop efficiently. Test a safe bounded bulk-copy representation only if generated code shows a real deficiency; preserve unaligned buffers, negative row stride, padding, component selection and output-on-error semantics. Switching callers to `decode_into_i32` can help suitable applications but changes the API under test and does not repair the byte-I32 comparison by itself.

DWT is lower priority unless fresh stages contradict the existing Tier-1 evidence. Reduced sides are at most 32; [`dwt.rs:762`](../neo-j2k/src/dwt.rs#L762) uses row-wise 9/7 only from area 16,384, while the 5/3 threshold is 262,144. These requests use small serial paths. A new SIMD/batched tiny-DWT kernel needs its own measured stage opportunity and exact floating/integer output validation; broad parallel-threshold changes cannot explain or fix a request that passes no executor.

## 5. Experiments not worth repeating as default solutions

The earlier [small-decode audit](small-decode-audit-2026-11-24.md) contains controlled counterexamples; its evidence is retained, not overridden by source-level intuition:

- Global predictable-MPS shortcut: full 32² lossless regressed about 71%; transform/≤8 geometry selection still hurt 128². Keep current MQ decision arithmetic for P1.
- General const boundary/side-padding trimming: tested variants regressed larger reduced fixtures roughly 6–15%; runtime trims also hurt full 32². Do not proliferate const kernels because stores look redundant.
- Segment-slot retention and refinement-only views: no robust target gain after unchanged-source controls. Do not spend the first performance patch on those cleanups or bypass modeled memory-budget charges with retained capacity.
- Width-8/16 specializations: about 1% v3 tuning lead in some cases, weak baseline evidence and code-layout sensitivity. Defer until after the ABI patch, with fresh final-code evidence.
- Copy avoidance: saved captures contain 80 chunks for 30 active 64² blocks and 86 for 38 active 128² blocks; all nonempty. C++ also gathers chunks, and joined-input replay still had a deficit. A chunk iterator adds per-byte boundary state and is not an evidenced first fix.

## 6. Implementation order and definition of done

1. **One MQ candidate commit**, based on current main, plus the minimum tests/comment changes needed. Run native MQ model and C++ differential suites, actual Miri (not only native `--cfg miri`), Tier-1 mutation checks, required native/full-package checks and the longer nightly arithmetic coverage. Keep short/general equivalence and scratch reuse checks.
2. **Same-process paired performance:** current/candidate/unchanged renamed control, matching release profile/fat LTO/one codegen unit and default features, warm decoders/caller buffers, alternating or rotating all orders. Keep `bench-internals` off for final timings. Run at least three quiet independent process repetitions initially, then increase repetitions/sessions where ranges straddle the decision threshold. Save compiler versions, CPU affinity/availability, exact commits/build flags and executable hashes alongside raw timings.
3. **All four platform/CPU cells:** Linux baseline versus SCALAR C++, Linux v3 versus AVX2 C++, Windows baseline versus SCALAR C++, Windows v3 versus AVX2 C++. Repeat local Arch/GCC16.2 separately from CI GCC16 snapshot; compare ratios within each run, never absolute times across machines.
4. **Rows:** all nine small decodes (whole/reduce2/missing EOC), all eight large decode/worker rows and six encode controls. Require exact samples, truncation status, layout padding/tails and diagnostics/limits in the relevant correctness probes. For paired improvement require the target gain to exceed the control band, with quiet controls and no material nontarget regression. For C++ parity classify per row using the existing spread and 1% floor; do not average away a losing row. Compare v0.1.2 independently to check whether the 32² regression actually closes.
5. **Scheduler experiment only after P1:** use the recorded four-worker attribution to select one bounded change, with its own commit, controls and full worker matrix. A parallel failure must still replay in codestream order for canonical diagnostics and status, as current `parallel_decode_tile` does.

Existing manual dispatch entry points (commands are proposals, not executions performed by this audit):

```text
gh workflow run perf.yml --ref CANDIDATE_BRANCH -f pair_base=BASE_SHA -f pair_head=CANDIDATE_SHA -f pair_side=small -f pair_name=mq-scalar-return -f rounds=24 -f vs_cpp=true
gh workflow run perf.yml --ref CANDIDATE_BRANCH -f vs_v012=true
gh workflow run stage-profile.yml --ref CANDIDATE_BRANCH
```

The first command needs P2's target coverage or explicit review of all three target rows; the default small gate only demands the 32² gain. Stage profiles are attribution aids and must not replace uninstrumented paired acceptance results. WSL2 lacks the hardware-counter evidence needed to assert branch-miss or instruction-cache causes here. Use native Linux hardware profiling if later experiments need those distinctions.

## Validation performed for this audit

Read the original four raw summary tables; checked their near-noise and four-worker ranges; reconciled main versus measured codec and historical prototypes; confirmed the scalar-result patch still applies without changing production files; inspected both retained Linux helper symbols and disassemblies; recorded current hashes/provenance limits; reviewed the cached-header, short/general, serial thresholds, output, row-band scheduling, DWT staging and target-gate paths. No candidate was compiled or timed on Linux in this audit, and no new end-to-end parity or regression result is claimed. The actionable output is the prioritized implementation/measurement plan above, strengthened by newly preserved Linux machine-code evidence.
