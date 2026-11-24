<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Current small-image decode audit

Follow-up (2026-11-24): the [targeted bottleneck audit](decode-bottleneck-audit-2026-11-24.md) has now performed the matched real-block experiment. It identifies an outlined MQ helper interface cost and a measured scalar-result remedy; whole-API parity is still not universal. Historical measurements and the original investigation below are preserved.

Audited 2026-11-24: Rust `07ba441` against adjacent C++ `50f74d2` (the production codec remains the pinned C++ `00f273d`). No production files were changed. Scratch variants and native comparisons were used to test mechanisms, not merely inspect source.

## Conclusion

The remaining deficit is concentrated in **reduced images**, not small images generally. The best supported regression cause is the removal of the decoder's short-block specialization in `eaea150`: the current general flag-row machinery is beneficial for larger blocks but pays avoidable work for blocks at most four samples high.

A scratch restoration that **keeps the current MQ register split, unrolled full stripes and Row/Cell layout** made the 32² RGBA reduce-2 request 6.0–7.6% faster at x86-64-v3 and 6.8–7.6% faster at baseline. In a separate, direct same-process Rust-versus-C++ comparison, that variant decoded this request 3.1–3.9% faster than C++, with identical samples. This is a demonstrated local route to parity for the smallest failing request, not a shipped fix or proof of Linux parity.

The 64² and 128² reduce-2 requests still need work. The expanded investigation below measured general boundary/store/view variants, bounded width kernels, conditional MQ paths and segment retention; none establishes parity for both larger fixtures. The short variant is nearly neutral on the former and has a small adverse result on the latter at v3. A broader rollback or an unconditional branchy MQ decoder is inappropriate: a tested MQ shortcut improved some reduced cases but made the full 32² lossless request approximately **71% slower**.

The complete 0.1.2-to-current slowdown is not fully apportioned by this audit. The experiments identify one specific source of it and a tested remedy; they do not establish that specialization removal explains every microsecond of the reported 15–20% version regression.

## The current evidence

### Separate parity deficit from version regression

[Current C++ comparison, run 45297083962](https://github.com/anne-skydancer/neoj2k-rust/actions/runs/45297083962), Rust `a061fbc`, has the same production codec as audited HEAD. The like-for-like remaining small failures are predominantly reduce 2: Linux baseline +7.0%, +14.2%, +13.7% for the 32², 64² and 128² fixtures; Linux v3 +2.6%, +3.1%, −1.7%; Windows v3 +0.6% (within noise), +2.3%, −5.6%. Full and missing-EOC small decodes generally improve or meet parity. Windows baseline Rust versus AVX2 C++ is not a fair baseline cell.

[Version comparison, run 45296325934](https://github.com/anne-skydancer/neoj2k-rust/actions/runs/45296325934), separately reports current Rust versus 0.1.2: 32² reduce-2 is 14.8–19.7% slower in all four cells, while full small decodes are much faster. These are different comparisons and must not be conflated. Different hosted CPU models explain why a gap may differ between cells; they do not remove a same-process measured deficit.

Fresh uninstrumented local runs also confirm the current reduced gap. Ryzen 9 9950X3D, Windows, eight reported logical CPUs, rustc 1.100.0, Rust v3/fat-LTO/one codegen unit, MSVC AVX2 C++ `RelWithDebInfo`, process affinity CPU 7; three repetitions of the existing 21 alternating rounds × 16 calls, exact sample checks retained:

| Reduce-2 request | Current Rust vs C++, median | Repetition range |
|---|---:|---:|
| 32² RGBA 5/3 | +5.81% | +5.00 to +6.03% |
| 64² RGB 9/7 | +5.81% | +5.61 to +6.93% |
| 128² RGBA 9/7 | +6.11% | +5.36 to +6.41% |

Whole-image versions of those requests were faster locally. These results are preserved in the [evidence directory](small-decode-audit-2026-11-24/README.md).

### Current stage profile excludes several tempting explanations

A fresh current-code stage profile was built and run locally. An earlier CI stage-profile run (`45288907317`, commit `6086372`) predates the final Tier-1 rewrite and must not be presented as a current causal profile.

The second fresh instrumented repetition gives:

| Reduce-2 request, byte API | Total µs | Tier-1 µs / share | Tile setup µs | Packet walk µs | Header µs | Output µs |
|---|---:|---:|---:|---:|---:|---:|
| 32² RGBA | 18.86 | 12.71 / 67.4% | 1.67 | 2.14 | 0.06 | 0.08 |
| 64² RGB | 23.26 | 15.58 / 67.0% | 1.27 | 3.87 | 0.04 | 0.11 |
| 128² RGBA | 59.68 | 48.00 / 80.4% | 1.67 | 4.19 | 0.05 | 0.29 |

Tier-1 includes block traversal and coefficient storage, not MQ decisions alone (`decode.rs:1070–1083`). Timers perturb these short calls: one scope measured 46 ns and a call has roughly 14 scopes. The first profile's 32² byte total was noisy (~23 µs), so the table is an illustrative second repetition, not an uncertainty-free decomposition. The paired optimization timings below have timers disabled.

The typed profile has nearly the same absolute Tier-1 cost, with a smaller output-copy total. It allocates its temporary target-descriptor vector per call, so total typed/byte differences are not a copy-only causal experiment. Nevertheless, output copy is far too small in these profiles to explain the remaining deficit by itself.

`decoder.rs:739–753` already reuses a validated header when its prefix and limits match. The warmed fixtures therefore do **not** reparse the complete header every call. A new general header cache is not a justified remedy. Small calls have no executor, so the older finding of 14 warm **parallel** allocations does not explain this serial regression either.

## S1 — restore a bounded short-block path in the current implementation

**Priority: first. Evidence: history, actual block histogram, isolated historical comparison, current-code A/B, C++ differential checks, direct C++ timing.**

Rust locations: `neo-j2k/src/t1.rs:157`, `:200–208`, `:222–278`; `t1/passes.rs:83–117`, `:135–163`, `:282–332`, `:424–453`. C++ comparison: `src/t1.cpp:295–320` (flag-neighbor updates), `:960–1000` (general guard-row initialization), and generic significance/cleanup paths. C++ itself uses general guard rows: this recommendation is an opportunity for Rust to outperform it, not an assertion that an identical C++ short specialization is missing.

Commit `eaea150` explicitly removed `SINGLE_STRIPE` to reduce code instantiations. The preceding `05e7fb3` already had the borrowed MQ state and unrolled full lanes. The current code always takes the general flag layout, builds three rows of left/middle/right cell views for each pass/stripe, initializes guard rows, and records upward/downward significance into guards even when no real neighboring stripe exists.

For height ≤ 4, those neighboring stripes cannot subsequently be decoded. Only the real flag row, its side padding and partial-lane padding matter. Repeated generic stripe setup is also a larger fraction of the work for tiny blocks.

A scratch histogram at the nonempty block's `decode_passes` entry confirms the workload, rather than inferring it from image size alone:

| Reduce-2 request | Active blocks | Height ≤ 4 | Active dimensions | Coding passes |
|---|---:|---:|---|---:|
| 32² RGBA | 39 | 39 | 15 × 1×1; 12 × 2×2; 12 × 4×4 | 624 |
| 64² RGB | 30 | 21 | 12 × 2×2; 9 × 4×4; 9 × 8×8 | 372 |
| 128² RGBA | 38 | 16 | 16 × 4×4; 12 × 8×8; 10 × 16×16 | 459 |

Empty blocks are not counted here. All fixtures are origin-zero single tiles, six resolutions, 64×64 nominal code-blocks, coding style zero, MCT enabled. Reduction exposes tiny low-resolution subband blocks, rather than merely making the output copy smaller.

### What the experiments prove

First, one executable compared current code, coherent Tier-1 files from `05e7fb3`, and that same old code with the **one-line** change `height <= 4` → `false`. Disabling the old short path increased 32² reduce-2 latency 4.6–5.1% in the three-way experiment. Current code was much faster on larger/full blocks. This identifies a real short-path tradeoff, not grounds to restore the whole old decoder. Different combined executables change binary layout: a later four-way build showed a smaller current-versus-old difference, while disabling the old short path still consistently hurt the target. Do not equate these percentages with the complete version regression.

Second, a candidate restored specialization **inside the current implementation**. The [patch](small-decode-audit-2026-11-24/short-path.patch) adds one `SINGLE` boolean through existing pass helpers, dispatches once by block height, initializes only the real flag row in that path, suppresses dead guard updates, and makes the one-stripe iteration explicit. It retains current full-row kernels, Cell views, MQ arithmetic and the generic fallback. This is a small family of related short-path changes; the audit does not separately apportion the gain among initialization, stores and compiler simplification.

Paired current/candidate timings, same executable, warmed byte API, exact output equality, 32 alternating rounds × 128 calls, three process repetitions, pinned CPU 7:

| Request | Candidate vs current, v3 median | v3 range | Baseline median |
|---|---:|---:|---:|
| 32² reduce 2 | **−6.61%** | −7.61 to −6.03% | **−7.25%** |
| 32² full | +0.13% | −0.53 to +0.23% | −2.62% |
| 64² reduce 2 | +0.20% | −0.57 to +0.58% | −1.31% |
| 64² full | −0.12% | −0.20 to +1.01% | −1.10% |
| 128² reduce 2 | **+1.06%** | +0.76 to +2.05% | −0.27% |
| 128² full | +1.23% | −0.28 to +1.28% | −0.63% |

No unchanged-code layout control was run for these scratch packages; package/code placement can move small differences. The v3 128² reduced adverse result is a real acceptance concern. Its cause was not isolated; it may include different code placement or changed short-path execution. It must not be omitted or dismissed as proved noise.

Finally, the existing small-image gate was copied with only its crate alias redirected to the candidate. It directly alternates the candidate and C++ in one process and preserves all original sample/status checks:

| Reduce-2 request | Candidate Rust vs C++, median | Range | Interpretation |
|---|---:|---:|---|
| 32² RGBA | **−3.74%** | −3.91 to −3.06% | Local target reached |
| 64² RGB | +4.11% | +4.00 to +4.62% | Still slower |
| 128² RGBA | +4.23% | +2.63 to +6.00% | Still slower |

All full and missing-EOC rows were faster in this direct candidate comparison. This is direct parity evidence for the 32² request; it was **not** calculated by multiplying measurements from different runs. It does not establish a cross-platform guarantee or isolate how much of the other rows' differences comes from code layout.

### Action and correctness gate

Land the short-block idea as a narrowly reviewed implementation experiment, retaining the generic path's shape as far as practical. Measure initialization/store-only and one-stripe-loop changes separately if the 128² regression persists. Avoid reinstating all old `SINGLE × RAW × FULL` structures wholesale or proliferating width/style combinations.

The scratch candidate passed **66 native tests selected by `--cfg miri`**; this is not interpreter/Miri verification. It also passed **8 current native C++ Tier-1 differential tests**, including tiny blocks in every orientation, generated blocks, committed vectors, bench dumps, overlong segments, RAW cleanup handling and ROI stores; two long nightly sweeps remained ignored. Existing sample comparisons passed every probe case.

Before shipping, run the normal release differential/mutation checks, actual Miri checks, and same-process parity rows on Linux baseline/v3 and Windows v3. Explicitly cover height 1–4, width extremes, VSC/RAW/RESET/SEGSYM, partial stripes, ROI, truncated segments and retained-buffer reuse. Guard words can remain stale only if no path reads them as a real row; every generic invocation must still initialize its own guards. Require no material regression in the existing full and 128² reduced rows.

## S2 — boundary trims: exact candidates, invariants and measured disposition

**Expanded investigation: current source, both CPU levels, unchanged-source package control. The earlier suggestion to try general boundary trimming is superseded by the following experiments.**

The short-only result does not mean every dead boundary store is worth removing. Concrete trims were implemented in scratch copies, keeping MQ arithmetic and current Row/Cell sample kernels. Three repetitions of each experiment timed full and reduced fixtures in one executable, cyclically rotating every variant through every position. Runtime/refinement and const-boundary experiments used 35 rounds × 64 calls; each had a renamed **unchanged-source** codec alongside original current and the original short candidate. This is a control for package/code-placement differences, not a byte-identical executable control or a complete correction for layout.

### Applicable boundary work and proof obligations

| Work | Exact trim | Required invariant |
|---|---|---|
| Outer top/bottom flag rows | At first stripe, suppress upward three-word updates; at the last complete stripe, suppress downward updates | Those words are never the current row of a real stripe; preserve every update between two real stripes |
| Guard initialization | Stop clearing/filling outer flag rows once every access to them is eliminated | Keep all storage initialized, real rows and side padding reset, and partial-lane PI mask intact; no guard value may leak into context selection |
| Left/right padding stores | Omit side and diagonal writes at column 0/last column when the destination is padding | `sign` reads CHI bits from padding, while those dead writes set SIGMA; preserve padding CHI initialization and every real-neighbor update |
| Refinement view construction | Build only the current exact-width Cell row in refinement | Refinement uses `here.mid` only; retain all sample, flag and dimensional checks actually needed for its own row |
| First/interior/last specialization | Determine boundary policy once per stripe, rather than testing it for each significant sample | Do not clone every style/width/lane combination or assume a removed branch saves more than extra code costs |

Rust references: `t1/passes.rs:24–51`, `:83–117`, `:135–163`, `:304–332`; C++ `t1.cpp:295–320` and `:960–1000` performs analogous general flag updates/initialization. Guard trimming is not removal of a missing C++ algorithm.

The prototype runtime implementation reuses the existing upper-VSC field for `VSC || first_stripe` and adds a lower-real-row predicate. A second version removes outer initialization. A const version dispatches among first/interior/last/single helpers once per stripe, compiling out dead updates. Side-only and combined versions skip padding stores. The isolated refinement version replaces a complete Stripe construction with `Cell::from_mut(&mut flags[first..first+width]).as_slice_of_cells()`.

All borrowing remains safe, with no pointer casts or fabricated lifetimes. For VSC, only upward propagation into a **real** previous stripe is suppressed by its existing rule; lower propagation remains normal. RAW uses the same flag invariants. A partial stripe must keep padding lanes marked PI, and cannot perform lane-three lower propagation when lane three is absent. Do not trim coefficient/sample zeroing: uncoded samples still require zero values. Side padding cannot simply retain old real-column values after a width change.

### Measured results: fewer stores did not yield parity

The following are medians against original current, not a claim that the unchanged-source control perfectly removes code-placement effects. Raw full/reduced times, ranges and candidate/control percentages are in [boundary evidence](small-decode-audit-2026-11-24/boundary-work/README.md) and `summary.json` there.

| Runtime/view experiment | 64² reduced baseline / v3 | 128² reduced baseline / v3 | Disposition |
|---|---:|---:|---|
| Runtime top/bottom store predicates | −0.10% / −0.90% | −0.05% / −0.78% | No decisive larger-fixture gain; full 32 v3 regresses ~7% |
| Above plus omit outer guard initialization | −1.07% / −1.35% | −0.37% / −0.06% | Small/near-control effect; full 32 v3 regresses ~8–9% |
| Refinement current-row view only | −0.11% / −1.68% | +0.05% / −0.80% | v3 64 range crosses zero (−3.75 to +2.14%); no proved bottleneck |
| Boundary/init plus refinement view | +0.33% / −0.91% | +0.20% / +0.71% | No consistent win |

In this set the unchanged-source control's reduced medians ranged from +0.25 to +0.88% depending fixture/CPU, with individual rows sometimes outside 1%. Sub-percent candidate/current numbers should not be promoted to causal gains. The direct refinement experiment particularly limits the earlier speculation: a source-level full Stripe constructor is present, but removing its unused views did **not** demonstrate a substantial view-reconstruction bottleneck. This does not prove every view/check has zero cost.

| Const/side experiment | 64² reduced baseline / v3 | 128² reduced baseline / v3 | Disposition |
|---|---:|---:|---|
| Const first/interior/last boundary kernels | +7.92% / +10.79% | +11.04% / +13.70% | Reject this implementation |
| Const boundary plus guard-init trim | +7.43% / +9.99% | +10.67% / +12.54% | Reject |
| Runtime side-padding store trim | +6.09% / +6.29% | +8.41% / +9.86% | Reject |
| Const boundary/init plus side trim | +9.07% / +11.43% | +12.49% / +15.08% | Reject |

The corresponding control reduced medians stay within approximately 0.4% in this set. These large regressions exceed that control. Const dispatch duplicates the full/partial coding bodies and changes code shape; runtime trimming adds predicates and live state. Those are concrete mechanisms to inspect, but no branch-miss or instruction-cache counters were collected, so this audit does not claim to have isolated the microarchitectural cause.

**Action:** retain the narrowly measured short-block path as S1's experiment. Do not land general runtime boundary tests, clone first/last kernels across the whole decoder, or remove side stores solely because they are semantically dead. If another boundary implementation is attempted, specify a smaller hot-code representation or one-time preprocessing that changes neither per-decision branches nor full-kernel footprint, and require a measured win over both current and unchanged-source control. No such general implementation was established here.

The combined const-boundary/side candidate passed 8 native C++ Tier-1 differential tests, with 2 nightly tests ignored. The timing probes also verified byte equality for all full/reduced fixture variants. This supports the tested flag invariant, not production readiness, interpreted UB verification or a performance recommendation. Patches are intentionally preserved as rejected experiments.

## S3 — do not globally replace select-based MQ with a predictable fast branch

**Priority: conditional research only. Evidence: controlled negative/positive A/B.**

Rust `t1/mq.rs:214–254` computes the next packed state and renormalization amount even when an MPS leaves a normalized interval and changes no context. An [experimental shortcut](small-decode-audit-2026-11-24/fast-mps.patch) returns early when `a - qe >= 0x8000 && c_hi >= qe`, subtracting the interval/code amount and preserving context/source/bit count.

In a current-code scratch variant it improved 32² reduced by 1.5–2.4%, 64² reduced by 3.0–4.1%, and 128² reduced by 2.0–2.6%. But full 32² lossless became 71.1–71.6% slower. Output equality held. The audit did not collect branch-miss/front-end counters, so branch unpredictability is a plausible mechanism, not a measured microarchitectural diagnosis; changed code shape may contribute too.

**Reject this as a global fix.** Only consider a geometry/workload-selected variant if it improves the entire supported matrix and the selection overhead is measured. Keep existing select arithmetic for the workloads where it already wins. This experiment is strong evidence against solving the smallest gap by undoing the broad Tier-1 gains.

### Follow-up: selecting MQ by transform or block geometry is not sufficient

A second prototype selects the MQ implementation **once per code-block** using a const-generic decoder type: current selects for reversible data, early-MPS shortcut for irreversible data. There is no per-symbol mode flag. A further variant enables it only for irreversible blocks of width/height ≤ 8, keeping the existing decoder for larger or reversible blocks. These are transform/geometry rules, not fixture-name matching.

| MQ selection, reduced fixture | Baseline median vs current / control | v3 median vs current / control |
|---|---:|---:|
| Irreversible FAST,64² | −2.63% / −3.54% | −4.51% / −4.98% |
| Irreversible FAST,128² | +2.19% / +4.40% | +1.64% / +3.34% |
| Irreversible ≤ 8×8 FAST,64² | −4.00% / −3.66% | +0.75% / +1.06% |
| Irreversible ≤ 8×8 FAST,128² | +5.36% / +5.97% | +7.71% / +9.65% |

A second executable containing both FAST variants also shows the broad transform rule helping 64² while hurting128². The exact magnitudes vary with code placement. Lossless avoids the original 71% disaster, but can still move by a few percent due to changed dispatch/code shape. The geometry restriction can regress even when the target's decoded blocks all qualify: one-time eligibility checks and different generated/layout context are not automatically free.

**Disposition:** irreversible FAST is a real positive lead for 64², not a universal9/7 solution. The tested transform-only and ≤ 8×8 rules fail the 128² target and are not recommended for global dispatch. Further selection requires actual captured-block/counter evidence, rather than inferring predictable arithmetic decisions from transform choice or dimensions. The restricted MQ prototype passed 8 native C++ differential tests; its samples match all timed fixtures. No new direct C++ parity claim is made from these modest/negative results.

## S4 — larger reduced fixtures: concrete setup trims investigated

The remaining larger requests are not simply a scaled version of the all-short32² case. Their active block mixes are21/30 and16/38 height ≤ 4, with 8×8 and16×16 blocks accounting for the remaining arithmetic work. Their current Tier-1 shares are about 67% and 80%; packet/setup work is secondary but large enough to investigate specific waste.

### Segment-slot retention: valid tested cleanup, not a measured parity fix

Rust `tile.rs:43` clears a code-block's segment vector on reset. Its first `t2/packet.rs:382` initialization then resizes to four default Segment slots, before initializing the active slot again. C++ `t2.cpp:663–674` resets the selected segment instead. A scratch [retention patch and validation](small-decode-audit-2026-11-24/tier2-trim/README.md) kept initialized slots and reset only active segments, while preserving the modeled growth/budget charges at indices 0,4,8,16,… independently of retained length.

The probe compared original, unchanged renamed source and candidate in one process, three repetitions of 36 cyclic rounds × 128 calls. Candidate/control reduced medians were **+0.06%,−0.08%,−0.24%** for 32²/64²/128², with every range crossing zero. One full 64² repetition had a large adverse outlier. Sub-1% improvements against original current largely followed the unchanged control and do not establish a bottleneck.

The candidate passed 3,510 exact sample/information/status/statistics/diagnostic comparisons across 65 generated files, reductions0/2/5, full/cut inputs and changing unlimited/16–128 KiB limits. This is useful validation of storage/accounting separation, not proof of a performance win. **Deprioritize it as a parity fix.** Never use retained vector length to bypass the original per-request allocation/limit accounting.

### Nonempty multilayer chunks: filtering is not an applicable trim

The [chunk inspection](small-decode-audit-2026-11-24/chunk-inspection/README.md) found:

| Reduced request | Active blocks | Incoming chunks | Empty chunks | Multi-chunk blocks |
|---|---:|---:|---:|---:|
| 32² |39|39|0|0|
| 64² |30|80|0|29|
| 128² |38|86|0|37|

Rust `t1.rs:184–196` joins contributions when at least two chunks arrive; nearly every active block in the larger multilayer fixtures genuinely needs more than one chunk. Filtering empty chunks would do no useful work here. C++ `t1.cpp:1075–1110` also copies contributions into retained contiguous Tier-1 storage, including the single-chunk case. The observation identifies a useful matched-benchmark axis, not a proved Rust-only copying regression.

A bounded follow-up, if matched block timing attributes cost to joining, is to compare one reserve for the total contribution length against repeated warm `try_reserve` checks, or an already-prepared contiguous contribution path. Preserve original segment boundaries, error handling and real chunk lengths. Do not add a byte-source chunk iterator merely to avoid a small copy without measuring its per-byte boundary-check cost.

## S5 — bounded width specialization and what is still needed for 128² parity

The final kernel experiment specialized only width 8, width 16, or both at the block entry, keeping MQ and current stripe/full/partial helpers. Width became a const parameter of the three pass methods, exposing constant sample/flag strides and loop bounds. The generic path remained for all other widths. This does not instantiate every width/style combination.

| Reduced request / variant | Baseline candidate/control median | v3 candidate/control median |
|---|---:|---:|
| 64²,width 8 |−0.36%|−1.00%|
| 64²,width 16 |+3.43%|−0.15%|
| 64²,width 8+16 |−1.06%|−0.81%|
| 128²,width 8 |+0.17%|−0.96%|
| 128²,width 16 |+1.50%|−0.71%|
| 128²,width 8+16 |−0.41%|−1.27%|

The v3 combined128² range against control was −1.33 to −0.91%, a small lead, while baseline did not show a comparable clear gain. Some unchanged-source control rows moved by 1–2%, and full-row results vary; complete times/ranges are preserved. This is insufficient to close the measured5–6% local gap or justify proliferating specializations. The combined-width candidate passed 8 native C++ differential tests, 2 nightly ignored. Keep it only as a small v3 tuning lead requiring repeated final-code/profile evidence, not the main solution.

### Ranked disposition after the expanded experiments

| Priority | Action | Evidence / decision |
|---|---|---|
|1|Validate and refine the S1 short-only path across target platforms |Clear32² gain and direct local C++ parity; preserve other rows |
|2|Build the matched **actual reduced-block** benchmark below for 64²/128² |Needed to identify a specific remaining kernel gap rather than accumulate speculative trims |
|3|Investigate the 64² MQ lead only with real-block state/counter evidence |Broad9/7 FAST helps64², but tested selection rules regress128² |
|4|Repeat the small v3 width 8+16 lead with final executable and layout controls |About1%128², not a parity-closing result |
|Defer|Segment retention and refinement-view cleanup |No robust target gain after control; possible cleanup only |
|Reject as implemented|Runtime/general const boundary and side-padding trimming |Measured larger-fixture/full-row regressions |

### Concrete next implementation experiment for the unresolved larger gap

1. **Capture the actual inputs once, outside timing.** At `Scratch::decode`/`decode_passes`, save all active reduced-fixture block bytes and stratify the ≤ 4,8×8 and16×16 groups, real chunk lengths, segment lengths/pass counts, numbps, orientation, ROI, coding style, transform/step and destination stride/offset. Retain the actual multilayer contribution shapes. The current Rust `bench::DecodeBlock::split(parts)` divides bytes evenly and cannot reproduce these real boundaries: add explicit chunk lengths/ranges in the scratch benchmark block instead. The C++ oracle request already accepts exact `chunks`, stride/offset and step/transform metadata. Synthetic entropy or64×64 dumps are not interchangeable with these blocks.
2. **Add a prepared oracle measurement entry, not a new production API.** Existing `neo-j2k-oracle/src/tier1.rs:122` allocates segment arrays/output on each `T1Decoder::decode`; `shim_t1.cpp:179–222` also constructs chunk/segment vectors. Timing those wrappers as-is would distort tiny-block conclusions. Prepare them once, retain both scratch buffers and output, and time repeated C++ `t1_decode_cblk` **plus `t1_store`**, matching Rust `DecodeWorkspace::decode`'s store. Add a benchmark-only Rust destination-stride/offset option or use equivalent buffers so sample layout is identical. Keep allocations, hashing and wrapper setup outside the region; amortize or match FFI overhead over a batch.
3. **First decide whether the deficit is in Tier-1 proper or its surrounding work.** Run both single contiguous bytes and original chunk shapes, with exact output equality. Compare initialization, segment setup/joining, significance, refinement, cleanup and coefficient-store work as separate sufficiently long batches. Per-decision clocks are too intrusive; aggregate counters and repeated blocks first.
4. **Then make one patch to the measured operation.** If segment setup dominates, test retained preparation/reserve changes; if pass bodies dominate, inspect final executable assembly for actual8/16 strides, live flag-row pointers, spills, context loads and byte-in calls. Collect branch/front-end counters before attributing MQ regressions to unpredictability or const-boundary regressions to instruction-cache pressure. A compact real-row/window representation that reduces live slice metadata is a specific candidate only if those spills/checks are observed. If store dominates, compare matched scalar/vector tails and caller strides. Do not globally replace MQ or clone more edge kernels from source appearance alone.
5. **Close the loop against C++.** The proposed patch must improve the captured real-block set with an unchanged-code control, then the same-process public64²/128² reduced rows, while preserving32²/full/missing-EOC results, limits and diagnostics. Repeat Linux baseline/v3 and Windows v3; a Windows 1% kernel effect is not Linux parity.

This is an actionable benchmark/patch specification for a still-unresolved target, not an assertion that128² parity was achieved. The experiments rule out the tested broad trims as solutions; they do not prove that all boundary or setup work is irrelevant.

## Acceptance and limits

A parity patch should be accepted by per-row results, not an average: preserve exact samples/status/diagnostics and resource semantics; compare the same API and lifecycle at matching SIMD levels; include full, reduce-2 and missing-EOC requests, both CPU levels and the compiler/platform cells that still lose. Repeat near-noise results and retain an unchanged-code/layout control. Do not compare absolute times from different CI machines, and do not treat a lower instruction/allocator count as an automatic latency win.

This audit identifies a tested local remedy for the smallest current parity failure and explains why earlier whole-codec optimizations traded away its short-block advantage. It does not yet prove that S1 alone meets every Linux cell, that an untested boundary implementation improves performance, or that the entire 0.1.2 regression is explained. Remaining 64²/128² parity and the candidate's v3 128² adverse paired result are explicit follow-up requirements.

No production source, dependency or CI file was modified, staged or committed by this audit. Scratch code lives outside the repository; only this report and evidence were added. See [reproduction notes, patches and raw logs](small-decode-audit-2026-11-24/README.md).
