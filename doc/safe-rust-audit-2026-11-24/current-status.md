# Current safe Rust candidate: performance work remains

The production crate conversion is implemented in the uncommitted `safe-simd` worktree. `#![forbid(unsafe_code)]` has no SIMD exception, and the normal dependency closure is the crate itself. Strict performance preservation is **not achieved**: controlled high-precision requests retain measurable losses. The local PR preparation is described in [the PR draft](../safe-rust-pr.md); no push or remote PR creation is part of this checkpoint.

## Implementation

- Safe arrays/slices replace architecture pointers and unsafe blocks in DWT, colour finishing, T1 stores and output packing. SIMD remains meaningful: final v3 code contains packed eight-lane arithmetic; baseline safe kernels also vectorize.
- Dense low/high DWT scratch and flat lifting loops preserve the existing eight-row scratch budget and edge-operation order. Outlining only scatter improves the measured row kernel; gather stays inline.
- Fast finishing uses ordered float clamps and nearest-even mantissa biasing; the Aurora reference at `1ea0749c798daf81ccb059de74557b862c9e2340` informed the clamp-before-bias pattern. Uniform wide AVX2 finishing uses flat per-pixel loops with three separate mutable inputs, three-operation rounding and signed/unsigned caps; mixed/baseline wide finishing retains f64. Bit-cast constructors avoid demonstrated out-of-line array mapping calls.
- Long two-channel packing rows use the measured flatten/copy hybrid; other packing paths keep their better zipped loops. T1 store experiments were reverted. Quantization retains the validated four-coefficient fast path and cold general fallback.

Source and executable snapshots: [Windows](final-candidate-checkpoint/windows-manifest.json), [Linux](linux-broad-matrix/manifest.json). The initial audit report and early experiments are historical checkpoints, not current acceptance evidence.

## Verification

Latest optimized library checks pass 108 tests with seven ignored in v3, and 102 with seven ignored in Linux baseline. They include independent rounding/quantization, odd/parity/padded/nonfinite DWT, all-precision colour/unary finishing, packing boundaries and existing C++ differential tests. Focused actual Miri passes cover the current [wide colour/unary](windows-typed-combo-paired/miri-colour-and-unary.json) and [outlined dense DWT](windows-scatter-paired/miri-dense-dwt.json) snapshots. Clippy and the native policy check pass; the policy inspects 56 source files and finds one normal crate and no production unsafe/FFI. Final Windows full-workspace native verification passed: [v3](final-candidate-checkpoint/windows-workspace-v3.txt) 215 tests and [baseline](final-candidate-checkpoint/windows-workspace-baseline.txt) 209 tests, 18 ignored in each cell. The [std-only native library check](final-candidate-checkpoint/windows-std-only.txt) passed 73 tests (4 ignored), and [std-only Clippy](final-candidate-checkpoint/windows-std-only-clippy.txt) passed with warnings denied. Final policy self-test/checker, formatting and diff checks passed. Current verification is complete.

The two-platform broad matrix comprises **36 processes / 528 rows**: three processes for each baseline/v3 and full/precision/tail mode. All compare Rust outputs/status; 144 precision/tail rows additionally match C++ samples/status. The other 384 normal/packed/encoder rows compare the three Rust versions. Native differential tests are separate. Default features are used without stage clocks.

## Measured limits

Three-process medians versus main `d13082f`; control is an unchanged renamed copy of that main.

| Platform / request | Candidate | Unchanged control |
|---|---:|---:|
| Linux baseline, large RGB one worker | -21.48% | -0.23% |
| Windows baseline, large RGB one worker | -21.36% | -1.46% |
| Linux v3, normal large RGB one worker | -2.44% | +0.03% |
| Linux v3, large precision31 RGB | **+2.09%** | +0.04% |
| Windows v3, large precision31 RGB | **+1.49%** | +0.24% |
| Linux v3, precision31 128 RGBA | **+2.27%** | -0.32% |
| Windows v3, precision31 128 RGBA | **+3.19%** | +0.48% |
| Windows v3, precision24 128 RGBA | **+5.70%** | +2.13% |

Full ranges and all requests: [Linux](linux-broad-matrix/README.md), [Windows](final-windows-matrix/README.md). Gray controls move materially, so gray results cannot establish preserved performance. Small baseline and encoder residuals also appear in the complete tables. Three repeats do not prove absence of regressions.

The [runtime allocation-order diagnostic](allocation-order-diagnostic/README.md) keeps one executable and all six constructor permutations. It exposes context sensitivity, but small precision31 remains slower across all six order medians. It does not dismiss the quiet broad-matrix residual.

## Rejected follow-ups and next decision

[Encoded rounding bias](encoded-shift-bias/README.md) passes over two million independent float/tie checks and improves a standalone kernel, yet regresses paired codec requests, including after proving the multiple-of-eight trip count. [Natural casts](flat-natural-cast-probe/README.md) still scalarize saturation. [BYTEIN branching](mq-bytein-branch/README.md) is supported by a 98–99.5% common-path histogram and passes MQ model/C++ tests, but worsens whole-codec timings. None was copied into the frozen candidate.

[Fresh independent stage accounting](../remaining-gap-independent/README.md) on the current typed-plus-scatter snapshot identifies finishing: large precision31 colour305.68us versus original249.92/control248.80, while DWT1224.83 versus1219.10/1230.15. At128 precision31, colour adds0.907us and unary finishing0.361us; DWT is effectively equal. Stage clocks alter code generation and are attribution evidence, not acceptance timings. Archived pre-typed stage profiles must not be treated as current. The next action should follow this attribution and retain zero dependencies, the stable compiler and exact outputs. Performance remains open, not an accepted tradeoff.


The final [independent review](../remaining-gap-independent/README.md) also completed the plane-padding and parent-authorized G-first diagnostics. Padding controls move, preventing a causal alias claim. G-first forces all inputs before the first store and passes exact checks, but adds coefficient rebroadcasts and yields no robust unsigned precision-31 kernel gain; no codec trial or port followed. No experiment is currently active. The frozen candidate and its unresolved performance gaps remain the reviewable result.
