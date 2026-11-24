# Specialist progress log

## Current verified state

**Safety conversion implemented; strict performance preservation remains unresolved.** The frozen `safe-simd` worktree uses typed three-operation wide colour/unary finishing, dense DWT with scatter-only outlining on both platforms, the retained original safe T1 store, and the long-row two-channel packing hybrid. Production unsafe is forbidden without exceptions. The normal closure is one crate; the policy check finds no production FFI/unsafe across 56 source files. No dependency was added. The retained changes and audit evidence are recorded on the local safe-simd branch; see [the PR draft](../safe-rust-pr.md).

Current verification is complete:

- Windows full-workspace native suites: **209 tests passed in baseline, 215 in v3; 18 ignored per cell**.
- Windows std-only native library: **73 passed, 4 ignored**; std-only Clippy with warnings denied passed.
- Current focused actual Miri colour/unary and DWT checks, Clippy, policy self-test/checker, formatting and diff checks passed. The [current status](current-status.md) links the exact evidence and validation scope.
- Both platform broad matrices are complete: **36 processes / 528 row checks**, three repeats of 32-row full, 6-row precision and 6-row tail modes in baseline/v3. All compare Rust output/status; 144 precision/tail rows also compare C++ samples/status.

Normal large RGB improves about 21.5% in baseline and 2.44% in Linux v3. Quiet high-precision gaps remain: Linux v3 large precision 31 **+2.09%** (unchanged control +0.04%); Windows **+1.49%** (+0.24%). At 128 RGBA precision 31: Linux +2.27% (−0.32%), Windows +3.19% (+0.48%); Windows precision 24 +5.70% (+2.13%). These are not accepted tradeoffs or a no-regression result.

Fresh independent stages identify colour/unary finishing as the remaining excess; DWT is essentially equal in those precision cases. Encoded-bias, MQ BYTEIN and isolated G-first scheduling variants were rejected after their bounded checks. None changed the frozen production candidate.

Read the concise [current status and verification](current-status.md), complete [Linux matrix](linux-broad-matrix/README.md), [Windows matrix](final-windows-matrix/README.md), [allocation-order diagnostic](allocation-order-diagnostic/README.md), and [final independent attribution review](../remaining-gap-independent/README.md). Source/executable hashes are preserved with those artifacts.

## Current work state

**No experiment or verification process is active.** The independent plane-relative padding diagnostic produced exact samples and C++ checks, but its controls also moved, so it established no causal fix or shipping allocation change. The G-first trial passed exact checks and its compiler gate, but shared-buffer kernel gains were weak/inconsistent; it was rejected without a codec trial or port. All current verification is complete; the performance requirement remains unmet. No commit or push has been made.

## History — superseded checkpoints

Entries below describe their named intermediate snapshots and experiments. Statements such as “not yet ported” or “matrix outstanding” below are historical; the current state is above.

This records work in progress, not final acceptance evidence. Production changes remain in the safe-simd worktree. The constraints are zero added dependencies, unsafe forbidden, and measured performance preservation.

Latest verified Windows snapshot uses the typed three-operation converter for wide uniform colour and single-plane finishing. The full native/C++ library suite passed108 tests (seven ignored), both integrated colour/unary tests passed actual fresh-target Miri, and test-target Clippy passed. The original coefficient store remains. Scatter-only DWT outlining is still an experiment, not yet ported to Windows.

Current Windows direct paired checkpoint: large precision24/31 are about-0.06/+0.07% versus original; precision31 at128 square is+1.89%. Remaining examples are precision24 at128 square+4.92% and packed-three-channel+4.31%. These named checkpoint medians are not a completed final platform matrix or a general no-regression claim.

Current candidate: the wide uniform AVX2 path now uses a separate helper with three mutable scalar slices and a per-pixel loop. It retains the improved kernel timing without a compiler fence or opaque index. Linux's 107 native/C++ tests passed; Windows release all-precision checks, test-target Clippy, and an actual Miri integrated colour/tail test passed. Windows alternating executable precision measurements are promising on large images but retain regressions. Performance preservation and the final complete platform matrix remain unfinished.

The later coefficient-store width-only hybrid was tested and then **restored to the previous implementation on Windows**: its tight-row microbenchmark gain did not survive large-image decoder measurements. That hybrid is experimental evidence only. The coefficient-store test now adds native widths31/33 and focuses Miri on8/31/32/33 while retaining all native large-row cases.

The stride-gated coefficient-store trial was also rolled back after direct same-process codec checks failed to establish an overall benefit. Both store experiments remain archived; the existing coefficient-store implementation is retained. Its expanded native/Miri boundary tests remain.

Latest retained rounding candidate: the clean per-pixel wide colour loop now computes the low residue with three floating-point operations instead of four. It passed2,085,184 independent float/tie checks, the full107-test Windows native/C++ library suite, actual fresh-target Miri, and Clippy. Direct Windows large precision24/31 comparisons improved0.6–0.9% against four-operation rounding in all three runs; small cases were mixed. The full performance goal is still unfinished.

Signed/unsigned integer-cap specialization passed the same2,085,184 checks and improved the wide colour kernel by about8% further (372→341ns/1024); its combined colour/unary integrated validation is now recorded above and in windows-typed-combo-paired/. The fused wavelet experiments have not produced a retained improvement. Scatter-only outlining is the next isolated DWT assessment.

## 2026-11-24 — latest validation

The flattened dense DWT improvement is ported to Windows. Four focused Miri tests passed: finite DWT with parity/padding, nonfinite DWT finishing, wide colour finishing, and output packing boundaries. The complete final Linux/Windows performance matrix is outstanding. Whole-decoder precision-31 measurements still show roughly 5–7% regressions in named cases; these remain unresolved.

## Shared RGB finishing parameters

A same-process comparison measured approximately 760 ns for the proposed shared-parameter version versus 465 ns for the current safe kernel at 1,024 pixels. The unchanged 8-bit branch also became slower. No production change was made. Subsequent identical-source controls showed this microbenchmark structure is unreliable for attributing the difference to the proposed source change.

## Separate general row helper

Moving the complete general finishing row into a private helper measured approximately 756 ns versus 467 ns for the current safe kernel. Again, the unchanged 8-bit path slowed. No production change was made; the identical-source control below prevents treating this as a reliable source-level rejection.

## Identical-source control exposes compiler variation

Two modules containing exactly the same current source produced different machine functions: approximately 210 versus 628 ns for the 8-bit path and 466 versus 754 ns for wide finishing at 1,024 pixels. The original intrinsic reference and its unchanged control merged to one address and measured approximately 269 ns for wide finishing.

The specialist is inspecting generated code and correcting the comparison structure before relying on further microbenchmark differences. Public whole-decoder precision cases remain the acceptance evidence. No MCT alternative from these experiments has been applied to production.

- Confirmed compiler/inliner mechanism: identical copies of the safe MCT source differed because one retained three out-of-line array `try_map` calls per eight-pixel group in bit reinterpretation. Replacing only the two eight-lane bitcast helpers with explicit constructors removed those calls and made identical eight-bit clones agree within about0.1% (206ns/1024), versus an earlier210/628ns discrepancy. Linux candidate only; public-codec timing and native validation remain the acceptance checks. Stabilized variant reruns are now executing.

- The constructor candidate passed106 native release/C++ unit tests (seven ignored). Actual public precision/tail rows remain exactly equal to all three Rust variants and the C++ oracle, with successful/nontruncated status. It does not close the wide gap: large precision24/31 +3.93/+5.65%, controls+0.21/+0.12%. The minimal helper fix is now ported to Windows for code-generation robustness; no full performance acceptance is claimed. Next bounded experiment removes iterator machinery from wide f32 rounding to test a stable all-lane implementation.

- After the Windows helper port, eight focused native tests passed (wide finishing, rounding, quantization, dense DWT, packing), and library Clippy passed with warnings denied. These checks used the existing native target with `--cfg miri -C target-cpu=x86-64-v3`; they are native executions, not a new interpreter run. Actual earlier Miri results remain separately recorded.

- Explicit scalar f32 wide-rounding constructors passed roughly2million float/tie patterns for all precisions and signs, but measured~795ns/1024 RGB versus the stabilized current f64~462ns. Restricting this to uniform RGB bounds stayed~792ns; mixed and eight-bit fallbacks remained within about1%. Both alternatives remain unported. The next check inspects their vector instruction shape before trying any further form.

- Whole-row range preflight (parent probe) passed exact output checks for signed31, unsigned31, mixed finishes and8bit. At1024 small-valued pixels it reduced the current safe kernel from467ns to298ns, still above original273ns. On larger values the scan added about10% (513ns versus467ns). It remains experimental and unported; declared precision also scales dequantized values, so the small-input result does not close the observed high-precision codec gap. Four-way driver/source/raw timing saved in parent-mct-bounded-probes/.

- The f32 loop is vectorized, but outer loop widening produces a much larger interleaved shuffle body. A group boundary reduced uniform wide finishing from~792 to~402ns/1024 (current f64~463). A compiler-only fence emits no hardware fence and reproduces this gain without opaque indices; the same boundary on f64 was slower and is not used there. A Linux-only AVX2/uniform candidate is now being tested against the actual codec and independent wide/tail tests; mixed/baseline finishing stays on f64.

- The independent integrated colour/tail test passed on Windows in optimized normal configuration across all31 precisions, with the existing MSVC oracle linked. An initial linking attempt failed because the skip-link flag was set; after correcting it, the build and test passed. This Windows snapshot retains f64 wide finishing. Linux candidate correctness includes the new uniform f32 path (107 native/C++ tests passed).

- Precision24's simpler signed-bias converter was exact over its whole normalized range in roughly1.5million pattern/tie checks, but measured~400ns/1024RGB, effectively the same as the uniform wide candidate. It is not added. The Linux uniform candidate passed107 full native/C++ tests, including the independent integrated uniform/mixed/tail test. Its first public six-row run remains exact but still has precision31 largeRGB+4.46% (original control+0.66%). A separate diagnostic public harness now compares new-safe and old-safe directly in one warmed process; its third slot is explicitly NOT an unchanged original control.

- Parent round-float/bit-extraction experiment uses explicit eight-element constructors and no saturating float-to-int cast. Its assembly contains vector rounding and variable shifts with no hot helper calls, but repeated bound loads/register spills. Exact driver checks passed; about843ns/1024 versus current464ns. Checked-shift refinement reduced this to about800ns, still much slower. Neither was applied. Sources/drivers/raw results are archived in parent-mct-bounded-probes/.

- A controlled per-pixel version reduced the bit-extraction experiment to about600ns/1024, still slower than the current f64 kernel (464ns) and bias candidate (403ns); it remains unported. This confirms loop structure affected earlier results without making that conversion competitive.

- The cleaner bias candidate passed the Windows integrated colour/tail test under actual nightly Miri (source hash, flags, exit status, log saved as miri-clean-wide-integrated.json/txt). Its optimized normal-configuration tests cover all31 precisions. Windows test-target Clippy passed with warnings denied.

- Three alternating Windows executable pairs compared the preserved f64 and cleaner candidate, each against original/unchanged control internally. Large precision24 residual: median+4.03% f64 versus+2.02% clean; large precision31:+4.14% versus+1.90%. Small128 precision24 remains+7.08% clean, and some controls move substantially. This is not a same-process old-safe/new-safe comparison or final acceptance. See windows-live-alternating-summary.md and named raw logs.

- The width-only coefficient-store hybrid passed native scalar/C++ comparisons, actual focused Miri, and Clippy. It improved tight-row store kernels, but alternating Windows decoder runs widened large precision24/31 residuals from about1.8–1.9% to3.0–3.1%. It was therefore not accepted. A realistic stride probe exposed the missing distinction: width64, height64, stride1024 reversible stores measured about622ns hybrid versus206ns current (original642ns), despite wins at stride128. All sample checks passed. A broader stride/alignment grid is pending; no shape-dependent rule is accepted yet.

- Dense97 outlining passed107 native/C++ tests and reduced the caller17.7KB→643bytes plus a14.3KB helper. Three paired residual/precision processes did not establish a benefit; precision64 cases were typically about1–2% slower, and unchanged encoder code also shifted, showing layout sensitivity. The Linux-only attribute was reverted and never ported to Windows. Next bounded DWT check separates raw gather from contiguous scaling, preserving the same scratch and floating multiply order.

- Realistic-stride DWT controls agree:32-wide rows atstride128/1024 show about13–22% safe-kernel overhead, greater than tight-row tests suggested. Separate gather/scaling was exact but20–30% slower and is not used. A fused even-origin lifting prototype reduced scratch passes and passed142 independent odd/even/parity/stride/random finite-bit cases, but its first micro was50–70% slower despite vector instructions;214 stack references motivate controlled lowering checks before judging it. No fused DWT code is in the worktree candidate.

- Typed signed/unsigned caps passed all 108 native/C++ tests. Three Linux public precision processes improved large p31 by 2.04–2.44% versus the otherwise-identical three-operation safe control; small rows still regress, so performance acceptance remains open.
- The same typed converter improves wide one-plane finishing from about 108 ns to 86–87 ns per 1024 values in a controlled microbenchmark. Combined colour+unary validation is now running; this is a kernel result, not a whole-decoder claim.

- Combined typed colour+unary passes 108 native/C++ tests and Clippy, plus three public precision/tail processes with exact outputs/status. Its large-image gain is inconclusive (p31 −0.61..+0.39% versus three-op control), and small64 still regresses. The kernel gain did not establish a whole-decoder win; Windows assessment is next.

- Outlining only DWT gather/scatter (unchanged lifting math/scratch) improves realistic-stride horizontal micro about9–11%, reaching original performance at widths64/128. Native/C++108 pass. Public paired assessment is prepared and awaiting the coordinated Windows timing window; no acceptance yet.

- DWT I/O outline public repeats improve largep31 about0.9–1.4% and normal64 about0.3–1.7% versus unchanged safe control, but encoder32 timings worsen. Three corresponding encoder hot functions have identical normalized assembly, supporting placement sensitivity. Whole-performance acceptance remains open; Windows DWT has not changed.

- Separating DWT call boundaries identifies scatter outlining as the gain: gather-only is slower, scatter-only improves about11–12% kernel and beats outlining both. Public paired build for this single-attribute candidate is ready; native/reference check is running before timing.

- Safe natural float casts were rechecked in the corrected flat three-reference loop, both clamp-before-round and round-before-clamp. Assembly still scalarizes each saturating conversion into lane extracts/branches; static bounds do not remove it on Rust1.100. No production change or unnecessary timing followed. Baseline/v3 final public harness builds are now prepared for the next matrix.

- Windows direct scatter-only assessment supports retaining it for the broad matrix: normal64 about−1.25%, packed3−1.79%, smallp31−2.0–2.2% against prior safe code, with some remaining mixed rows. Both platforms now evaluate typed wide finishing plus outlined scatter. Full baseline/v3 original/control matrix is next; the performance requirement is not yet declared met.

- Broad matrix is now active with typed three-operation colour/unary finishing, scatter-only DWT outlining, and the original coefficient store. Windows native108+7ignored, focused colour/unary Miri2 and DWT Miri2, test-target Clippy and native-policy/selftests passed. Final source/executable hashes are frozen in final-candidate-checkpoint/. Linux baseline full run1 completed32 exact rows in28.98host seconds; remaining repeat processes are pending. Performance acceptance is not yet established.

- Broad Linux matrix complete:18 processes/264 row checks across baseline/v3 full32, precision6 and tail6 modes, all exact/status;72 precision/tail rows also match C++. Normal8bitlargeRGB improves21.48% baseline and2.44%v3. Quiet-control v3 largep31 remains+2.09% versus original(control+.04%), so strict performance preservation is still unresolved. Full source/executable hashes and per-row ranges are in linux-broad-matrix/.

- Derived rounding-bias encoding removes one integer add/vector per high-precision conversion.2,115,584 independent float/tie checks and108 native/C++ tests pass; controlled colour kernel improves3–5%. Direct public safe-control assessment is prepared; no Windows source change or performance acceptance yet.

- Encoded-bias and its proven multiple-of8 lowering refinement were rejected after direct public comparison: despite fewer instructions/compact clean vector loops, largep31 worsens~0.3–2.0% versus prior safe control. No Windows port. Linux restored to the typed+scatter broad-matrix snapshot; quiet~2% original-performance gap remains.

- One-executable allocation-order diagnostic passed72 exact rows over all6 construction permutations/two passes. Candidate MCT instructions are identical to the broad matrix. Largep31 gap varies with context, but smallp31 remains positive across all order medians; this does not establish preserved performance or dismiss the quiet residual. Full addresses, order checks, source/exe hashes and tables are archived.

- Actual BYTEIN profiling finds98.2–99.5% non-FF events. A tiny constant-shift branch passes all7 MQ model/C++ tests and emits SHL8/9, but direct public largep31 worsens2.1–2.6%; rejected without main-source mutation. Histogram, assembly and exact-output results are archived in mq-bytein-branch/.
