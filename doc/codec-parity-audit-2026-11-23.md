<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Encode and decode parity audit

## Decision and priorities

The remaining parity problem is several different problems, not a missing wholesale port of the C++ Tier-1 redesign. The Rust encoder already has constant-lane passes, the refinement pair list, cleanup candidate masks and borrowed MQ contexts. Its decoder has the select decision, aggregate runs and the significance/sign dependency split, but **still copies its complete MQ state for every pass and keeps runtime lane/width indexing**. Those are concrete differences from the C++ implementation.

The first implementation work should be:

1. Split the decoder's MQ registers from its borrowed context array; prove the 160-byte pass copies disappear.
2. Give the common decoder stripes constant lanes and checked, structured row/flag views; retain the generic path.
3. Add an encoder path that omits rate-distortion bookkeeping when the resolved request retains every pass.
4. Replace the encoder's generic gather loop with full-stripe kernels, and remove its baseline per-coefficient `rintf` call using the existing rounding infrastructure with matching semantics.
5. Remove avoidable parallel metadata allocations and serial table construction, then measure scheduling thresholds and task size.
6. Address output-buffer traffic and reduced-image setup using stage measurements. Merely changing the small benchmark to typed I32 does **not** establish parity: the fresh typed experiment still has approximately 14–19% reduced 9/7 deficits on this machine.

These are an implementation roadmap, not promises of particular percentage gains. There is enough evidence to start focused changes; there is not enough evidence to guarantee that any one change will close every row. No production code was changed by this audit.

## Scope, revisions and evidence

Audited the maintained Rust `neo-j2k/src/` encoder and decoder, their benchmark/oracle interfaces, the adjacent C++ `src/` implementation, and build/SIMD choices. Archive code is excluded. Rust started at `f5f9e92`; the checkout moved through `bb7a372` to `87e02f9` during the review, committing audit documentation/evidence without production codec changes. C++ was `50f74d2`. No applicable `AGENTS.md` was found in the repositories in the initial inspection.

Two revision comparisons matter:

- Rust `git diff d5c164e 87e02f9` contains workflow, benchmark-summary, test and documentation changes, with no production codec changes.
- C++ `git diff 00f273d 50f74d2` contains benchmark/build/test/documentation changes, with no production `src/` or `include/` changes. The oracle's pinned C++ production implementation remains representative of the adjacent current twin.

Evidence labels used below:

- **Measured:** actual whole-codec timings or allocation counts, with the stated machine and method.
- **Assembly:** a specific generated-code property observed with rustc 1.100.0, Windows MSVC target, release optimization.
- **Source:** a definite operation or dependency in the implementation, with an optimization whose runtime value remains to be measured.
- **Hypothesis:** a profiling target; not a demonstrated bottleneck or a promised fix.

Fresh library assembly was emitted at baseline and `-C target-cpu=x86-64-v3`, with workspace release fat LTO and one code-generation unit, default codec features plus `bench-internals`. This is optimized **library emission**, not a claim that every instruction survives final downstream executable LTO. Static instruction and panic-call counts are not dynamic instruction counts. [The extraction script and outputs](codec-parity-audit-2026-11-23/inspect-assembly.py) make the observations reproducible without checking in 20 MiB of compiler assembly.

The earlier [Miri/correctness audit](audit-2026-11-23.md) is separate. Miri splitting and lazy assertion formatting are already implemented and are not outstanding speed findings here. This report does not certify correctness or the absence of undefined behaviour.

## What currently misses parity

### Full CI evidence

[Performance run 45242934789](https://github.com/anne-skydancer/neoj2k-rust/actions/runs/45242934789), Rust `d5c164e` against pinned C++ `00f273d`, has three repetitions per cell. Its raw artifacts were reprocessed using the corrected current summary script. [Full tables](audit-2026-11-23/performance.md) retain the spreads and absolute times.

| Comparable cell | Rust faster | Within noise | Rust slower | Eligible rows |
|---|---:|---:|---:|---:|
| Linux GCC 15, baseline / scalar C++ | 8 | 1 | 12 | 21 |
| Linux GCC 15, x86-64-v3 / AVX2 C++ | 4 | 1 | 16 | 21 |
| Windows MSVC, x86-64-v3 / AVX2 C++ | 10 | 5 | 6 | 21 |

On Linux v3, 1,024² RGB decode is +2.0% at one worker and +5.2% at two. The 64² RGB and 128² RGBA reduce-2 rows are +13.3% and +13.9%. The six cold encoder rows are +3.0% to +8.0%. Large gray decoding is faster. Windows v3's encoder rows are faster by about 6–13%, while six small 9/7 decode rows are slower by about 6–16%. The encoder's result depends on platform/compiler; there is no universal whole-codec slowdown.

The Windows baseline cell uses an AVX2 C++ oracle against baseline Rust and is not a like-for-like parity cell. Eight-worker rows on four-logical-CPU hosted runners are oversubscribed and excluded from eligible counts. Earlier pre-port tables in `doc/bench.md` and one-repetition quick smoke runs must not replace the full current-code measurements.

### Fresh local typed-versus-byte experiment

The audit also built current Rust and C++ locally with matched AVX2 CPU levels. Machine: Ryzen 9 9950X3D, Windows, eight reported logical CPUs; rustc 1.100.0, workspace fat LTO/CGU1, C++ MSVC AVX2 `RelWithDebInfo`. The benchmark shell and children were restricted to logical CPU 7. Before the window, other-process load measured over two seconds was 3.79%; audit compilation was idle during timing. There were three process repetitions, 21 alternating Rust/C++ rounds of 16 calls per row. Byte/typed order was BT, TB, BT. Every existing exact-output check passed.

The typed probe copies the current small-image harness and replaces its Rust `PlanarI32` helper with typed buffers and `decode_into_i32`. It constructs eight stack descriptors per call; the byte helper has prebuilt descriptors. It uses a separate binary. Consequently the difference between byte and typed percentages includes descriptor work, binary layout and drift, and is **not a causal measurement of the copy alone**.

| Request | Byte Rust vs C++ | Typed Rust vs C++ |
|---|---:|---:|
| 32² RGBA 5/3, full | -15.23% | -14.63% |
| 32² RGBA 5/3, reduce 2 | +4.06% | +3.53% |
| 32² RGBA 5/3, missing EOC | -19.06% | -19.23% |
| 64² RGB 9/7, full | +9.59% | +6.96% |
| 64² RGB 9/7, reduce 2 | +15.92% | +14.38% |
| 64² RGB 9/7, missing EOC | +8.70% | +6.33% |
| 128² RGBA 9/7, full | -3.77% | -8.29% |
| 128² RGBA 9/7, reduce 2 | +19.59% | +18.62% |
| 128² RGBA 9/7, missing EOC | +0.78% | -7.39% |

Values are medians of the repetitions' percentage changes. The last byte and typed rows have spreads crossing zero and should not be treated as reliable wins. [Raw timing logs](codec-parity-audit-2026-11-23/local-results/byte-1.txt), [typed probe](codec-parity-audit-2026-11-23/typed-probe.rs) and [manifest](codec-parity-audit-2026-11-23/typed-probe-Cargo.toml) are preserved. All three repetitions are in that evidence directory.

The same local window ran the cold encoder comparison three times. The 32² lossless row was +2.30%; 32² lossy was +1.21% with a spread crossing zero; the 64² and 128² rows were about 0.86–2.64% faster. This does not invalidate the Linux deficits. It reinforces the need to validate encoder changes on Linux, rather than infer Linux parity from a Windows-only result.

## Findings and implementation actions

Priorities below rank work toward the requested parity objective. They do not denote security severity. Effort is relative implementation/review effort, not an estimate of elapsed time.

| ID | Priority | Concrete change | Affected work | Evidence | Effort |
|---|---|---|---|---|---|
| D1 | First | Borrow MQ contexts separately from decoder registers | All MQ decode passes, especially small blocks | Source + assembly | Medium |
| D2 | First | Constant lanes/common-width decoder stripe kernels | Tier-1 significance and cleanup | Source + assembly | Medium/high |
| E1 | First | Skip distortion and slope work when all passes are retained | Default/full-pass lossless and lossy encode | Source | Medium |
| E2 | First | Unroll encoder gather; use matching native rounding; fuse preparation | Encoder coefficient preparation | Source + assembly | Medium |
| E3 | Next | Reduce encoder hot-loop checks and separate edge paths | Tier-1 encoder significance/cleanup | Source + assembly | Medium/high |
| P1 | Next | Retain owning parallel metadata and reduce serial slice-table work | Multiworker decode | Source + measured allocations | Medium/high |
| O1 | Next | Avoid repeated output-capacity zeroing and redundant plane clearing | Encodes and owned/byte decode | Source | Medium |
| D3 | Next | Reduce fixed setup for small/reduced decodes with validated reuse | Small reduce-2 decode | Measured gap; source hypothesis | Medium/high |
| E4 | Small change | Build block-job descriptors only for parallel encode | Serial/cold encoder | Source | Low |
| P2 | Profile first | Coarsen encoder batches; tune decode work and thresholds | Large multithreaded workloads | Source hypothesis | Medium |
| W1 | Profile first | Target DWT/MCT vectorization and fusion where stage shares justify it | Transform-bound workloads | Source + existing SIMD coverage | Medium/high |

### D1. Decoder pass locals still copy the complete context array

**Rust:** `neo-j2k/src/t1/passes.rs:117`, `:175`, `:220`; `neo-j2k/src/t1/mq.rs:95`, `:162`. **C++:** `src/t1.cpp:131`, `:475`, `:673`, `:793`; `src/t1_mq.h:160` and `:178`.

Each Rust pass starts `let mut local = *state` and ends `*state = local`. The 160-byte decoder includes its 32 context words. Its comment says registers become locals, but a variable-indexed context array remains embedded in the same local. This is not merely aesthetic: baseline significance, refinement and cleanup each emit two `memcpy` calls; significance's prologue explicitly passes size 160. V3 replaces the calls with full-width YMM copies, still copying the entire object. Significance reserves a 552-byte stack frame in both emissions. This evidence establishes copies and substantial stack traffic; it does not establish how much wall time they cost.

**Change:** use the encoder's existing `Encoder::lend`/`Run` pattern (`encoder/tier1/mq.rs:145`) for decoding: borrow `&mut [u32;32]`, carry `a`, `c`, `bits`, `current` and the input slice separately, and return only kept registers. The cold byte-in routine should take only its small register payload. Context reset between passes must act on the borrowed array. Keep the public API unchanged.

**Why before another MQ arithmetic rewrite:** the select decision and aggregate-run algorithms already match the C++ redesign. Removing an observed state-management cost is a narrower experiment than changing that decision again.

**Acceptance:** no full 160-byte copies in final benchmark disassembly, no new context-array bounds checks, identical single-decision registers and byte consumption, all context-reset/raw/segment tests passing, then improvement on real Tier-1 dumps and relevant whole-codec rows. Test empty/truncated segments and RAW-to-MQ transitions, not only ordinary images. A smaller stack frame is useful evidence but is not by itself a performance pass.

### D2. Common decoder lanes and widths remain runtime quantities

**Rust:** `t1/passes.rs:58`, `:75`, `:111`, `:170`, `:214`; dispatch `t1.rs:244`–`:293`. **C++:** `t1.cpp:466`–`:501`, `:512`, `:665`–`:710`, `:783`–`:910`.

The C++ full-stripe significance/cleanup paths invoke lane 0/1/2/3 explicitly and have 64×64 entry points with constant strides. Rust's decoder loops over `lane in 0..rows`, computes lane-dependent shifts and sample offsets, and retains runtime `vertical_causal`. Its existing `SINGLE_STRIPE` specialization addresses short blocks but not the common four-row interior. Baseline emitted significance has 11 static bounds-panic call sites and cleanup 20; v3 retains the same counts. These counts include cold error paths and do not mean every decision executes 11 or 20 checks, but demonstrate the compiler has not discharged all index proofs.

**Change:** first separate complete four-row stripes from the final partial stripe and call small `const LANE` helpers for interior significance and cleanup. Hoist vertical-causal handling to lane-zero/stripe-boundary work. Next add one measured common 64-wide, non-VSC specialization; keep arbitrary dimensions, VSC, RAW and short-stripe fallbacks. Structure data as checked disjoint row slices and flags as checked neighboring stripes, with the dimensional proof outside the hot column loop. Fixed-size chunk/array views expose more facts than indexing the entire dynamic plane repeatedly.

Do not replace every safe access with `get_unchecked`, nor generate every combination of width/style/transform. Large monomorphized code can trade arithmetic savings for instruction-cache pressure. D1 and D2 should be separate commits and measurements.

**Acceptance:** exact samples across all existing T1 differential tests, generic-vs-specialized draws covering widths 1–64 and decoder's wider accepted blocks, partial heights, ROI and all block styles; inspect hot-loop branches/shifts/checks; then interleaved Tier-1 and whole-decode timing. The decoder dump benchmark includes coefficient stores, unlike the C++ kernel-only bench, so compare Rust variants with identical harnesses before attributing absolute C++ kernel differences.

### E1. All-pass encoding pays for distortion values nobody consumes

**Rust:** `encoder.rs:648`–`:678`, `:737`–`:790`; `encoder/tier1.rs:85`, `:188`, `:264`, `:339`, `:520`–`:546`; `encoder/tier2.rs:66`–`:101`, `:147`–`:219`. **C++:** `enc_t1.cpp:148`, `:459`–`:480`, `:581`–`:593`; `enc_t2.cpp:533` onward.

The default encoder cases used in the six timing rows retain all code-block passes: empty layer ratios resolve to zero rate targets. Both the reversible and irreversible transform cases qualify; “all passes” is not synonymous with the reversible wavelet. Nevertheless, every significant/refined sample contributes distortion-table work, every pass computes a weighted distortion, and `allocate_rates` first scans all pass slopes and divides distortion by rate. When a layer's threshold is negative, `Workspace::layer` selects all remaining passes without reading any distortion value. When **every resolved target is zero**, none of the slope extrema or pass distortion values are needed for output selection.

This waste also exists in C++; it is an opportunity for Rust to meet or beat the twin, not proof of a port mistake.

**Change in two independently measurable steps:** (1) skip `min_max_slope` when all effective rates are zero; (2) select a `const COMPUTE_DISTORTION: bool` Tier-1 path once per request, eliminating distortion-table loads, accumulation and weighting in the false variant. Keep MQ symbols, pass lengths/rates, termination and flush unchanged. Derive eligibility **after** `setup_rates` and `update_rates`, since the byte cap can turn an apparent all-pass request into a rate-limited one. Conservatively require every effective rate to equal zero; preserve existing handling of nonfinite inputs through the normal path. Keep the bench/oracle code-block API's default metadata-complete path.

**Risk:** prematurely switching based on `params.reversible` or an empty input ratio vector would be wrong for capped or mixed-layer requests. Existing oracle tests compare pass distortion, so the opt-in production fast path must not silently change those bench contracts.

**Acceptance:** byte-for-byte equality to C++ and the original path for both transforms, MCT settings, zero/multiple layers and zero-valued blocks; mixed positive/zero ratios and caps force the normal path; positive-rate distortion metadata remains exact. Measure all six Linux encoder rows and whole-image full-pass encodes; measure positive-rate encodes for regressions from additional specialization/code size. No gain percentage is claimed without that experiment.

### E2. Encoder gathering is scalar, dynamically nested, and baseline rounding calls the runtime

**Rust:** `encoder.rs:357`–`:388`, `:963`–`:991`; `encoder/tier1.rs:475`–`:483`; `simd/mod.rs:20`. **C++:** `encode.cpp:582`–`:654`; `enc_t1.cpp:508` onward.

Rust's gather visits stripe → column → runtime row, recomputes `row * stride + column`, checks the source index and advances a fallible destination iterator for each sample. The C++ full stripe explicitly loads four rows per column. The emitted Rust gather retains the runtime inner loop and source/destination checks at baseline and v3. More significantly, **baseline lossy gathering calls `rintf` per coefficient** for `round_ties_even`; v3 emits hardware rounding and removes that call. Both builds remain scalar in this gather. Division by the band step is shared with C++ and cannot simply become reciprocal multiplication under exact-output requirements.

**Change:** split full four-row stripes and the 1–3-row tail; slice four validated source rows once, zip their equal-width parts and write into destination `[i32;4]` chunks. Dispatch reversible/irreversible once outside the pixel loop. For baseline SIMD-enabled x86_64, investigate the existing `simd::round_i32` conversion helper so gather has the same native rounding route as decode finishing. Preserve the scalar fallback and establish the floating-point-environment contract; the intrinsic observes the rounding environment whereas `round_ties_even` explicitly requests ties-to-even. Match C++ overflow/NaN behaviour (`i32::MIN`) and division/multiply order exactly.

Then consider an internal `encode_from_plane` preparation entry that gathers directly into the Tier-1 sign-magnitude workspace while reducing the maximum magnitude. This can remove the intermediate signed zigzag buffer plus the second sign-magnitude pass. Keep the benchmark's signed-input entry as an adapter to a common coding core. C++ also converts the gathered data to sign-magnitude; fusion here is an opportunity rather than a necessary transcription.

**Acceptance:** compare gathered fixed-point values, sign-magnitude values and maximum to the original path, including `i32::MIN`/conversion boundaries and partial stripes. Preserve exact encoded bytes and pass rates for all oracle cases. Check emitted gather for the disappearance of the per-sample `rintf` call, then measure both CPU levels; do not infer a v3 gain from a baseline-only rounding improvement.

### E3. Encoder Tier-1 is already specialized, but checks and edge-path duplication remain

**Rust:** `encoder/tier1.rs:136`–`:247`, `:250`–`:318`, `:323`–`:447`; `encoder/tier1/mq.rs:220`–`:264`. **C++:** `enc_t1.cpp:185`–`:245`, `:267`–`:333`, `:337`–`:453`; `enc_mq.h:118` onward.

Do not reimplement E3 pair-list refinement or cleanup masks as though missing: they are present. The fresh emitted code still has substantial fallback/check machinery: baseline significance has 1,911 static instruction lines and 17 bounds-panic call sites; cleanup 3,702 and 18. V3 cleanup has 3,660 lines and 21 such sites. There are 14 statically duplicated byte-out helper call sites in significance and 33 in cleanup. Static counts are a code-shape warning, **not proof of instruction-cache misses**, and include partial-stripe copies of the logic.

**Change:** separate rare partial-stripe handling from the complete-stripe hot functions, rather than force-inline every lane through one large body. Expose validated flag-stripe dimensions to the compiler as in D2. Audit the 19-slot MQ context indexing separately: variable context values from lookup tables/pair bytes can retain a bound check even though logically below 19. Options include a compact internal validated context representation, constant-context entry points for AGG/UNI, or a padded power-of-two slot table with an explicit bounded index, as the decoder already uses. Preserve all actual context values and test every table-produced context. Do not apply masking indiscriminately to sample or flag indices to conceal invalid accesses.

**Acceptance:** use final executable disassembly and a profiler to confirm hot checks or front-end pressure fall. Exact MQ register/flush, codeword and distortion tests remain mandatory, as does benchmark control. A smaller function that spills more or inhibits inlining can be slower. The Windows encoder already wins many rows, so Linux-focused gains must preserve those wins.

### P1. Parallel decoding rebuilds heap metadata and serial borrowing tables

**Rust:** `decode.rs:623`–`:645`; `decode/blocks.rs:197`–`:285`; `dwt.rs:370`–`:401`, `:506`–`:582`, `:675`–`:697`; `executor.rs:48`. **C++:** `tile.cpp:218`, `:631`; `dwt.cpp:1733`–`:1906`, `:1988`.

The staged path makes a fresh reconstruction-plane vector and row-cost vector. Wavelet levels make row/stripe slice tables and walk them serially before dispatch. The 9/7 stripe table stores `height * stripe_count` fat pointers, and the interleave path constructs a row map and cycle representation. C++ computes stripe offsets directly and uses retained cycle scratch. The earlier allocation probe, still valid for unchanged production code, observed **14 allocation calls after warm-up** on typed 1,024² RGB decoding with reported concurrency 2/4/8, against zero at concurrency 1. Its inline executor excludes pool allocation and does not measure parallel throughput.

**Change order:** retain owning metadata (`Row` estimates and integer cycle/index maps) in reusable scratch; use bounded storage for component descriptors if practical; reduce or fuse serial table population with existing horizontal work. Retain only ownership-free indices/geometry across calls, not borrowed mutable slices with fabricated lifetimes. Rebuild temporary borrows in a scoped owner whose disjointness is clear. Consider a single allocated table rather than one vector per stripe for the generic path. The cost of building borrow tables, not merely the allocator call, must be measured.

`Jobs::run` initializes 64 mutex slots for every batch. Measure this cost and right-size active storage or expose a more direct safe task representation where caller partitioning permits it. Mutexes here protect one-time transfer of disjoint jobs; removing them without replacing that ownership guarantee is not an optimization plan.

**Acceptance:** repeat the allocation probe, count total metadata bytes and dispatches, and add stage timestamps around table construction/worker execution. Compare 1/2/4/8 on hardware with at least that many available CPUs and persistent pools. Maintain serial fallback on allocation failure and deterministic diagnostic/error ordering. Zero allocations is a useful secondary gate, not proof of faster decode.

### O1. Repeated zeroing and output copies add traffic outside the arithmetic kernels

**Rust:** `encoder.rs:346`–`:354`, `:482`–`:503`, `:850`; `decode.rs:1064`–`:1074`; `decode/output.rs:197`–`:231`; `decoder.rs:580`–`:624`. **C++:** `encode.cpp:817`–`:833`, `:1106`; `decode.cpp:1069`–`:1071`, `:1284` onward.

Encoder output is cleared to header length and then resized to estimated tile capacity with zero on every request. The estimate is approximately `1.4 * input sample count + overhead`, even for a much smaller final codestream. C++ obtains writable arena storage without an equivalent complete output zero fill. Rust's owned coefficient plane is also filled before a block walk whose direct-output path deliberately skips the clear because blocks write samples or zero missing blocks.

**Change:** for repeated encoding, keep an initialized backing buffer at its high-water length with a separate valid codestream length/cursor, and return only the valid prefix. Write headers/packets through slices/cursors and keep unused bytes private. This can avoid repeated warm zeroing in safe Rust; do not form `&mut [u8]` over uninitialized storage. Avoid a second full-sized buffer that saves a clear by adding a copy. Cold zeroing requires a different output-writer design and should be measured separately.

Remove owned decode clearing only after proving every relevant coefficient is written on every path, including retained-buffer reuse after failure. For caller I32 planes, prefer the existing typed API; a byte slice cannot simply be cast to `&mut [i32]` without alignment/aliasing obligations. For U8/U16/interleaved callers, investigate a fused final conversion/store from reconstruction, preserving overlapping output layout semantics and write order.

**Acceptance:** poison retained buffers, alternate sizes/components/reduce settings, decode empty/truncated/missing blocks and odd-origin/subsampled inputs, and compare exact output. For encode, alternate small/large results and errors, verify no stale bytes escape `encode` or `encode_to`. Measure bytes cleared/copied and full latency; output traffic is particularly unlikely to explain the entire reduce-2 deficit by itself, as the local typed experiment demonstrates.

### D3. Reduced decode retains a fixed header/tile/Tier-2 setup floor

**Rust:** `decoder.rs:727`–`:803`; `tile.rs:130`–`:288`; `t2.rs:112`–`:166`; `t2/progression.rs:142`–`:213`; `t2/packet.rs:61`–`:247`. **C++:** `decode.cpp` header/tile walk, `tile.cpp` component/precinct setup, `t2.cpp` packet traversal.

The strongly reduced outputs are small, but each request reparses the main header, configures resolutions/bands/precincts and packet traversal, and reads packet headers to locate the retained contributions. Rust retains vector capacity but recomputes geometry and resets mutable trees/segments. This is a definite setup cost; no stage profile in this audit establishes which of those operations dominates the remaining gap. C++ also performs setup, so the mere presence of setup is not a port defect.

**Action:** instrument header, tile setup, packet walk, Tier-1, DWT, finish and copy separately before changing parsing. Use the typed experiment's remaining reduce-2 gap as the target. If setup is material, cache immutable geometry only under a complete validated coding/size/reduce key, while resetting packet/tag/segment state for each codestream. Alternatively introduce an explicitly prepared decode/session path for callers repeatedly decoding the same validated structure; do not silently assume different inputs share headers because dimensions match.

Do **not** skip arbitrary high-resolution packet headers or marker validation just because the caller requests reduction. Packet progression, packed headers, length accounting, later layers and exact error behaviour can depend on them. Existing LRCP/RLCP duplicate tracking is already elided where safe (`prepare_seen`); proposing that optimization again would be stale.

**Acceptance:** record stage shares for the failing rows, then demonstrate a decrease in the measured setup stage and whole-call latency. Alternate streams with COC/QCC/POC/PPT/PPM/tile overrides and different reductions on the same decoder; maintain all resource-limit and error/retry tests. This finding is a profiling prerequisite with a concrete cache boundary, not a confirmed recommendation to add a broad cache immediately.

### E4. Serial encoding constructs descriptors used only by the parallel path

**Rust:** `encoder.rs:341`, `:357`–`:389`, `:446`–`:480`. **C++:** `encode.cpp:727`–`:778`.

`prepare` unconditionally calls `reset_block_jobs`, rebuilding one descriptor per block with division/modulo, offsets and T1 parameters. Serial `encode_blocks` then walks the bands and recomputes the same metadata, ignoring `block_jobs`. C++ uses its job list even for serial processing; Rust currently pays both paths.

**Change:** build descriptors only when parallel Tier-1 will consume them, or have serial and parallel encoding share one descriptor traversal. The first is smaller; the second avoids divergent geometry logic but changes the serial code shape. Repeated shape caching can be considered later if profiling warrants it.

**Acceptance:** identical bytes at resolutions 1–13, partial blocks and all component counts; cold/warm serial allocation and latency measurements. This is a small, direct reduction of duplicated work, unlikely by itself to close every encoder deficit.

### P2. Both encoders synchronize after every width-sized block batch

**Rust:** `encoder.rs:392`–`:443`; `decode/blocks.rs:197`–`:285`; `dwt.rs:56`, `:209`. **C++:** `encode.cpp:779`–`:808`; `tile.cpp:631`; `dwt.cpp:1815` onward.

Rust and C++ encode at most one code-block per worker in each batch, wait, collect errors, then launch the next batch. This makes a large image incur many joins and lets a hard block hold up all other workers at every boundary. Rust additionally transfers parts through its mutex slots. Decoder scheduling uses weighted row bands, more bands for wide RGB pools, and wavelet thresholds; those policies are plausible but not established optimal on the available hardware.

**Change:** after measuring dispatch count and per-batch imbalance, let each borrowed worker own a disjoint contiguous run of blocks/descriptors and encode several before the next join. If content imbalance is large, partition by estimated work into more parts than workers, while preserving disjoint output ownership and deterministic first-error selection. Keep per-worker scratch reuse. Avoid a shared mutable queue or new thread pool unless simpler partitions fail measured requirements. This can improve both codecs; it is not a Rust-specific missing C++ algorithm.

For decode, separately sweep two versus four bands per worker and the wavelet parallel area threshold on nonoversubscribed machines. Do not assume width 8 should always use more parts, or infer a hardware crossover from a four-vCPU CI result.

**Acceptance:** count joins and idle time, test a mix of textured/sparse components and large gray/RGB, use a valid persistent executor, and verify first-error diagnostics, omission/duplicate protection where supported and sample/byte identity. Report latency and scaling, not simply worker utilization.

### W1. Transform optimization should preserve already-good kernels and exact arithmetic

**Rust:** `encoder/wavelet.rs:148`–`:163`, `:184`–`:260`, `:293`–`:345`; `dwt/lifting97.rs:115`–`:207`; `mct.rs:53`–`:93`; `simd/dwt.rs`, `simd/mct.rs`. **C++:** `enc_dwt.cpp:160`–`:288`, `:346` onward; `dwt.cpp:1723` onward; `mct.h`.

The encoder already batches eight adjacent columns for both 5/3 and 9/7 and exposes fixed-size lane arrays, while decoder row-wise 9/7 and AVX2 finishing are already implemented. Float bit-pattern conversions in the Rust buffers are bit reinterpretations, not automatically expensive numeric conversions. The current large gray wins argue against replacing the transform subsystem wholesale.

**Action:** first measure transform shares for the specific losing workloads. If forward DWT dominates, check final Linux and Windows code for packed arithmetic in full column batches and avoid checked index reconstruction inside each lifting step. Separate the scalar tail and keep complete groups in array/slice kernels. For input MCT, test a fused U8 deinterleave/DC-shift/colour-transform kernel to reduce full-plane passes; for decode, extend final format conversion only where O1's API constraints permit it. These fusions are independent experiments, not a reason to add a framework of kernel traits.

Preserve the exact four lifting steps, boundary doubling, scaling and multiply/add order. Do not enable fast-math, reassociate to FMA, replace division by a reciprocal, or substitute superficially equivalent 5/3 rounding without byte/sample parity proof. Existing SIMD sometimes intentionally follows a different intermediate formula to match the C++ build; generic algebraic cleanup can break that contract.

**Acceptance:** edge sizes/odd origins and all exact DWT/MCT differential cases, emitted packed instructions where expected, then measured transform and whole-codec improvement at both CPU levels. SSE2-only decode finishing may merit a baseline experiment if profiling identifies it; v3 regressions cannot be solved by adding a baseline-only kernel.

## Implementation and validation plan

### Stage 0: freeze the comparison

Record both commits, rustc/LLVM, C++ compiler/options, features, CPU level, actual available CPUs and executor. Keep baseline/scalar and v3/AVX2 comparisons separate. Preserve the byte-layout suite and add the typed suite as a separate API contract; do not replace a failing row with a faster API without accounting for callers. Keep cold encoder results and add reusable-encoder comparisons on both sides when available. The C++ shim's input/output copies and descriptor construction are real harness costs; stage/kernel claims must exclude or match them.

Use a dedicated quiet runner for implementation gates. Hosted CI is useful for confirming compiler/platform behaviour but cannot give a universal cycle budget. Do not run competing compiler/profiler jobs during a timing window. Count batches and timed work, and record output digests. Profiles should separate stages before attempting a broad design change.

### Stage 1: narrow kernel changes

Implement D1, then D2 as separate changes; E1 and E2 as separate changes. Each should be reviewable with one output-preserving mechanism. Preserve unchanged-code control binaries and interleave runs. Use existing Tier-1 mutation checks/sweeps where the changes touch arithmetic or traversal; add only targeted cases for a genuinely new path. Include positive-rate encodes even when optimizing the all-pass path.

### Stage 2: remove fixed and memory costs

Implement E4 and safe warm-buffer reuse, then P1's owning-metadata reuse. Instrument D3 before a geometry cache. Review allocation/limit accounting and failure recovery alongside latency. Removing the C++ accounting ledger merely because it is complex would alter intentional error/limit parity; optimize its execution only with the relevant tests preserved.

### Stage 3: scheduling and remaining transform work

Use stage shares and nonoversubscribed scaling results to choose P2/W1 work. More SIMD is not automatically the next step: serial borrowing-table construction and repeated joins can dominate the multiworker tail. Treat PGO or architecture-specific scheduling as later experiments after the identified structural costs are addressed. Maintain baseline functionality and avoid making deployment CPUs depend on an unrecorded `target-cpu=native` build.

### Parity acceptance gate

The user objective is achieved only when the supported workload matrix meets parity, rather than when an average hides a regression:

- Exact decoded samples and encoded bytes for the supported twin contract; no lost diagnostics/status/limit behaviour.
- Like-for-like Linux baseline and v3 plus Windows v3 results for each existing small decode/encode request and large decode width supported by available hardware. Windows baseline requires a scalar C++ build before it is eligible.
- Separate cold and reused lifecycles; byte and typed layouts remain identifiable. Add representative large encodes and rate-limited/multilayer encodes so a default full-pass optimization is not overgeneralized.
- For a narrowly optimized kernel, use the existing paired benchmark/control method and repeat near-threshold results. For whole-codec parity, predeclare a noise band (the current summary uses 1%) and inspect confidence/spread: a negative average is insufficient when a required row remains clearly slower.
- Do not count oversubscribed width-8 rows as eight-CPU parity. Repeat on a machine with eight available execution contexts if width 8 is a requirement.

## Limits and files produced

This audit measured API comparisons and allocation behaviour, inspected optimized Rust assembly and compared source algorithms. It did not obtain a Linux hardware profile, modify or benchmark a proposed optimized codec, or independently prove that all suggested changes combine constructively. The expected mechanism and target workload are specified for each finding; throughput benefits remain experiments until implemented and measured.

Fresh evidence is under [codec-parity-audit-2026-11-23](codec-parity-audit-2026-11-23/): assembly summaries and extraction script; typed probe source/manifest; three raw repetitions each for byte decode, typed decode and cold encode. Earlier full CI tables and allocation reproduction are linked above. Scratch target directories stayed outside the repository. No production files, dependencies or git commits were changed by this specialist audit.
