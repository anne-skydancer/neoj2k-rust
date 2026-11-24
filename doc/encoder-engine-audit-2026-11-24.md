<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Encoder engine audit

Audited 2026-11-24: native Rust `07ba441`, C++ twin checkout `50f74d2`. This review covers the current encoder, not the pre-optimization code from the earlier parity audit. Production source, dependencies and CI were not modified, staged or committed. Scratch variants were used for experiments.

## Main findings

The current encoder already incorporates the two substantial recent changes: full-pass requests skip Tier-1 distortion measurement, and sample gathering uses four-row kernels plus native sample rounding. The current published small-encoder comparison is at least level with C++ in its like-for-like cells. There is no basis to describe the whole encoder as uniformly slower.

The strongest new implementation lead is **encoder tag-tree path construction**. Even a one-leaf tree currently constructs and returns a 65-entry index path. Fresh optimized library assembly contains repeated 520-byte clears/copies and a 1,544-byte Tier-2 writing stack frame. A small one-node shortcut avoids that work and improved selected small/rated local requests. It needs an independent final-build confirmation because unchanged-source controls also move some rows by 1–2%.

No new sample-corruption, invalid-access or output-lifecycle bug was established in the inspected encoder. The current and combined tag-tree candidate passed the selected native oracle tests; this is targeted evidence, not a blanket correctness certification. Significant scheduling and buffer-traffic opportunities remain, but their throughput effects have not been demonstrated by this audit.

| Priority | Finding | Status | Concrete disposition |
|---|---|---|---|
| P2 | One-leaf tag trees pay for a general 65-entry path | Source + assembly + paired timings + native validation | Prototype a small one-node shortcut, then confirm rated and full-image gates |
| P2 | Parallel Tier-1 joins after each width-sized batch | Source-confirmed, shared with C++; no new parallel throughput result | Test a bounded coarser work partition with persistent pools and exact first-error ordering |
| P3 | Reused output is resized/zeroed to an estimated full tile capacity | Source-confirmed; latency share unmeasured | Measure warm bytes cleared; use a valid-prefix design only if material |
| P3 | Generic path depth is much larger than validated encoder geometry requires | Source + assembly; mixed timing results | Derive a domain bound if retained, separately from one-node shortcut |
| Deferred | Slope-scan and serial descriptor skips | Already measured and deliberately dropped in PR 15 | Do not relabel them as missing fixes or repeat them as high-confidence gains |

Priorities denote engineering work, not security severity. No P1 correctness issue was proved.

## Scope and measurement boundaries

Inspected `encoder.rs`, forward wavelet/colour/sample conversion, `encoder/tier1.rs`, encoder MQ, packet generation/rate allocation/tag trees, stream output, scheduler ownership transfer, current encoder tests, and corresponding C++ `encode.cpp`, `enc_dwt.cpp`, `enc_t1.cpp`, `enc_mq.h`, `enc_t2.cpp`. No applicable `AGENTS.md` was found in repository or checked ancestry. Archive code is excluded.

Local timing: Windows, Ryzen 9 9950X3D, eight reported logical CPUs, Rust 1.100.0, workspace-equivalent release fat LTO and one codegen unit. Baseline and `-C target-cpu=x86-64-v3` were built separately. The dedicated benchmark process was pinned to CPU 7. Decoder audit builds/profiles were paused during encoder timing. Three executable repetitions per CPU level used 20 cyclic-order rounds; each implementation occupied every order position equally. Inputs/settings were prepared outside timing; both warm and cold encoder lifecycles were measured. Every fixture's exact encoded bytes matched before timing.

The benchmark links current, a renamed unchanged-source copy, bounded-depth candidate, one-node candidate and combined candidate in one executable. It controls for some code-placement effects, not all of them. No continuous load trace or byte-identical executable control was collected for every repetition. Some unchanged-source rows differ by 1–2%; small gains and adverse rows must remain visible. No Linux throughput experiment or new matched C++ timing campaign was performed here. [Raw data, patches, manifests and reproduction](encoder-engine-audit-2026-11-24/README.md) are preserved.

## E1 — one-node tag-tree fast path is a concrete, narrow lead

**Rust:** `neo-j2k/src/encoder/tier2/coding.rs:90`, `:121–178`; callers in `encoder/tier2.rs:255–325`. **C++:** `src/enc_t2.cpp:139–190` and `:237–310`.

`TagTree::path` constructs `[usize; usize::BITS + 1]`, computes leaf coordinates by division/modulo, walks successively ceil-halved levels, and returns the complete array plus depth by value. `set_value` calls it before checking whether any ancestor value needs changing; `write` calls it to code a path. Small encoder subbands commonly contain one code-block, so the tree has one node and its only legal leaf is zero. The root is already the leaf: no coordinate calculation, path traversal or path array is needed.

The C++ set-value loop walks parent pointers and exits directly on a root. Its writer uses an uninitialized fixed stack of node pointers and does not fill/copy a full Rust-style 65-entry path. This is a definite port/design difference. It does not imply every encoder call is dominated by it.

Fresh baseline optimized library emission shows `Workspace::write` with a 1,544-byte stack reservation, several explicit `memset` and `memcpy` call sites whose length is 520 bytes, and variable divisions. [Assembly summary](encoder-engine-audit-2026-11-24/assembly-baseline-summary.txt) records the observed code. These are static sites in library emission before final executable LTO, not dynamic counts or a claim that all survive downstream optimization.

### The tested shortcut

The [one-node prototype](encoder-engine-audit-2026-11-24/single.patch) checks the node slice once in `set_value` and `write`. For a singleton, set-value applies the same minimum to the root; write directly updates its `low`, `value` and `known` state and emits the same bits. Other trees retain the original path. This uses safe borrowing and adds no dependency, global cache or unsafe access.

A production change should share the root-bit-writing logic with the general traversal where that keeps the code clear, rather than maintain two subtly different tag-tree algorithms. Keep the one-node optimization separate from changing the generic path bound so each can be reviewed and measured independently.

### Paired results for the one-node candidate

Percent change below is time against the original current crate; negative is faster. The adjacent column compares the same candidate against the renamed unchanged-source control. These are medians of per-repetition changes, not quotients of independently aggregated times. Complete ranges and adverse rows are in the JSON/raw evidence.

| Request | Lifecycle | Baseline vs current / control | v3 vs current / control |
|---|---|---:|---:|
| 32² RGB 5/3, all passes | Warm | −1.25% / −0.87% | −2.73% / −3.07% |
| 32² RGB 5/3, all passes | Cold | −1.52% / −1.71% | −0.98% / −1.13% |
| 128² RGB 5/3, all passes | Warm | +0.03% / −0.35% | −0.72% / −0.76% |
| 128² RGB 9/7, all passes | Warm | −0.17% / −0.07% | −1.13% / +0.65% |
| 128² RGB 9/7, ratios 30/10/0 | Warm | −2.22% / −0.90% | −1.65% / −3.82% |
| 128² RGB 9/7, ratios 30/10/0 | Cold | −2.16% / −1.00% | −1.55% / −3.14% |
| 128² RGB 5/3, ratios 3/1.5 | Warm | −2.41% / −1.77% | −1.70% / −3.31% |
| 128² RGB 5/3, ratios 3/1.5 | Cold | −2.48% / −1.70% | −2.20% / −3.67% |
| 256² RGB 9/7, ratios 30/10/0 | Warm | −0.71% / **+0.74%** | **+0.40%** / −1.09% |
| 512² RGB 9/7, all passes | Warm | −0.18% / −0.07% | −0.54% / +0.27% |

Rated 128² requests are a plausible target because rate allocation repeatedly attempts packet writing. Their one-node candidate/control spreads remain below zero, but the unchanged control itself is outside a 1% band in several rated cases. That makes this a reproducible local lead, not a certified universal gain. The 256² row also shows why the improvement must not be generalized to every tree size or code layout. Full-image and cold/warm results must remain separate.

**Action:** implement the one-node shortcut as a small independent change, preserve the generic traversal, then repeat on Linux and Windows with final linked code, interleaved controls and varied content. Require exact encoded bytes, pass metadata where applicable, stream offers/status, settings updates and reuse after errors. Include single-node-to-multi-node transitions, all-pass and rated/mixed/capped layers, not just the six current small all-pass rows. Do not claim zero regression from this finite Windows probe.

## E2 — bound the general tree to its actual domain only with evidence

The encoder validates a maximum image side of 4,096 (`encoder.rs:22`, `:605–613`) and emits 64×64 code-blocks. A resolution-one image can have at most 64 leaves along either axis; successive ceil-halvings reach the root in seven node levels. Higher resolutions cannot increase that bound. The 65-slot path is sized for an arbitrary `usize` grid that this encoder cannot produce.

A separate [depth-seven prototype](encoder-engine-audit-2026-11-24/bound.patch) changes only the path array. It removes much of the stack path clearing/copying, but its timings are mixed. On v3 it improves rated 128² requests against control by roughly 3.4–4.0%, while 512² full lossy becomes about 0.56% slower against current and 1.35% against control. Baseline effects differ. The combined candidate is not consistently better than the singleton shortcut alone.

**Action:** prefer the singleton change first. If the generic bound is pursued, derive it from `MAX_SIDE`/`BLOCK_SIDE`, document the validated geometry, and assert/test the coupling when limits change. Do not ship an unexplained hard-coded seven or treat smaller stack storage as automatic throughput improvement. An alternative is a compact index/parent representation with no returned large array, but that is a separate measured design experiment, not a reason to build a general tree framework.

The standalone domain check covered 144 representative dimension pairs, including the maximum 64×64 leaf grid, and 60,516 leaf paths. Every checked path fit seven entries and indexed valid nodes. This is not an exhaustive test of all geometries or a full 4096² public-API oracle comparison.

## E3 — parallel scheduling has a concrete synchronization cost, not a measured Rust-only bug

**Rust:** `encoder.rs:397–453`, `:456–489`, `executor.rs:48–75`. **C++:** `src/encode.cpp:779–808`.

The current parallel encoder submits width-sized batches: each worker codes one code-block, the call joins, errors are collected, then the next batch starts. The shared `Jobs::run` initializes up to 64 mutex slots for each call and transfers each mutable part once. C++ uses the same one-block-per-worker batch structure, although it does not need Rust's ownership-transfer mutex slots. This is an opportunity shared with the twin, not a missing C++ algorithm.

For a sufficiently large block count, the number of Tier-1 joins is `ceil(block_count / executor_width)`, plus the separate component-wavelet batch. This is a source formula, not an experimentally sampled throughput result. No new multiworker speedup is claimed by this audit.

**Actionable experiment:** give each borrowed worker a consecutive run of blocks/descriptors and reuse its existing `tier1`/`zigzag` scratch while coding that run. Submit one or a small number of coarser batches. Keep parts disjoint; collect errors in original block-range order after joining so the earliest failing block remains the reported one. A single long run per worker can imbalance luma/chroma, sparse/textured bands and edge blocks; evaluate bounded coarsening factors before a dynamic queue. Do not add another codec-owned thread pool or thread-local scratch.

Use the existing persistent `BenchmarkPool`, not the testkit `Pool`: the latter deliberately starts scoped threads on every batch and would exaggerate the benefit of fewer joins. Measure 1/2/4/8 workers on hardware with those CPUs available; count dispatches and idle/working time; compare rate-limited, full-pass, RGB/RGBA/gray, sparse and textured content. Verify output bytes and first-error behavior. Executor scheduling changes can alter performance without touching MQ, but the gain remains unmeasured here.

## E4 — output traffic and preparation: concrete costs with bounded claims

**Rust:** `encoder.rs:352–359`, `:492–515`, `:864`; sample input/MCT at `:547–570` and `encoder/wavelet.rs:148–163`. **C++:** `src/encode.cpp:817–833`, `:1106`.

Every request clears the logical output, writes the header, then resizes to estimated tile capacity with zero. The estimate is approximately `1.4 × input sample count + overhead`, even if the final rate-limited codestream is much smaller. Warm vector capacity avoids heap allocation but does not avoid writing those zero bytes again. Input deinterleave/DC shift and MCT also make separate full-plane passes. These operations exist; their present latency share was not measured by this audit.

A targeted warm-output experiment can keep initialized backing storage at its high-water length and maintain a separate valid prefix/cursor. Packet code writes into already-initialized slices and the public result exposes only the valid codestream. This can avoid repeated warm zeroing in safe Rust. Preserve cumulative-prefix behavior of `StreamWriter` and header/tile/end write failures; do not add an equivalent full-sized copy or expose stale bytes. Cold allocation/initialization is a separate problem. Do not form slices over uninitialized `Vec` capacity.

A fused input deinterleave/shift/MCT kernel is another bounded possibility if input preparation is material, preserving exact float operation order and 8-bit planar/interleaved/flip semantics. Do not replace strict arithmetic with fast-math, FMA reassociation or reciprocal division merely to reduce operation count.

**Priority:** measure cleared/copied bytes and stage time before redesigning buffers or fusing transforms. The current small-image encoder is already competitive, and these source costs do not prove a present bottleneck.

## Previously investigated changes that should not be rediscovered as bugs

[PR 15](https://github.com/anne-skydancer/neoj2k-rust/pull/15) records that skipping the all-zero-rate slope scan and building block-job descriptors only for parallel Tier-1 were measured and deliberately dropped as within about 1%. They remain on `rust-enc-speed-all4`. Current `allocate_rates` does still scan slopes and serial `prepare` does still create unused parallel descriptors; that is not proof of an overlooked regression. Reconsider them only if a new workload/profile exposes a material cost, not because a previous audit listed them.

Already present: full-pass distortion skipping after effective-rate resolution; four-row sample gathering/native rounding; borrowed MQ contexts/registers; constant-lane passes; cleanup candidate masks; refinement pair list; warm buffer retention; deterministic error selection; checked input extent arithmetic. Recommendations to port these again are stale.

## Correctness, API and Rust-quality review

The inspected maintained encoder remains safe Rust; intrinsic unsafety is confined to the existing SIMD module. The borrow-based worker model transfers disjoint mutable outputs once and waits for completion. Mutex slots have an ownership purpose; they are not removable solely because C++ uses pointers. Separate normal/all-pass Tier-1 specializations have a documented output-selection invariant and oracle coverage; they are not needless abstractions by themselves.

Input dimensions/component count/sample count are bounded before sample allocation, and row extents use checked multiplication/addition. Public input supports only unsigned eight-bit samples, so internal wrap-aware wavelet arithmetic and quantization bounds must be assessed in that domain. No valid-input panic was reproduced in this review. Native tests cover malformed settings, caps, unusual float ratios, short inputs/strides, comments/options, state reuse and exact codestream comparisons.

The stream writer consumes accumulated encoder prefixes with a `written` cursor. Header, tile-without-EOC, and final-full-prefix calls therefore do not duplicate headers or bytes; the helper's cumulative-prefix contract is intentional and the stream traces test it. Simplifying it to write each supplied slice in full would break output and failure-phase behavior. Parameter updates before later coding/writer errors likewise match documented behavior and oracle tests.

The memory arena ledger complexity from the decoder is not part of this encoder's storage scheme. Encoder buffer retention is deliberate for repeated calls; replacing vectors with a generic arena or kernel trait framework is not supported by the measured findings. The clearest excessive generality found is the tag-tree path sized for arbitrary machine-word grids despite a much smaller validated domain.

Rust explicitly requires the target's standard floating-point control settings, including executor threads (`README.md:195–199`). C++ installs a default-environment guard. That difference is a documented integration precondition, not a newly discovered encoder bug. No claim of parity under arbitrary host MXCSR/FTZ/DAZ settings is made.

`max_codestream_size` is implemented through effective compression ratios and header/layer adjustments, following C++; it should not be inferred to be a universal hard output-buffer bound independent of headers/settings. No cap-semantics change is recommended without the existing update/error compatibility tests.

## Validation and acceptance

Current native encoder selection: **22 passed, 3 long nightly sweeps ignored**. The combined tag-tree scratch candidate also passed **22**, ignored the same **3**, then passed **8** stream/update/executor integration tests. The exact-output timing probe validates all five variants on each measured fixture. The separate domain test checks representative path bounds. Logs and exact commands are in the evidence directory.

These checks were native Windows release tests against the current MSVC oracle; no actual Miri run, full fuzz campaign, new Linux performance run or exhaustive failure-allocation campaign was performed. Passing the combined candidate supports its semantics, not every possible future formulation of a singleton shortcut or depth bound.

Suggested implementation order:

1. Review a small one-node tag-tree path independently. Confirm removal of path clearing/copying in final executable code and rerun representative all-pass/rated/capped cold/warm cases on Linux and Windows. Preserve every adverse row and layout control.
2. If parallel latency remains a target, evaluate coarser borrowed runs with persistent pools, measured dispatch/imbalance and exact output/error gates.
3. Profile output/input traffic before a retained-prefix or fusion change. Defer previously rejected micro-trims unless a profile changes their priority.

A change is acceptable only with exact bytes/status/diagnostics/update behavior and no material regression in supported encode or decode workloads. Encoder-private code changes should not affect decoder algorithms, but final binary layout and shared helper changes can still move decoder timing; preserve the full/reduced/missing-EOC and large-image decode regression gates. This audit provides a specific tested speed lead and a code-quality assessment; it does not assert that a universal regression-free patch has already been established.
