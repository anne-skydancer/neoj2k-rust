# Forbid unsafe code and replace intrinsic kernels with safe Rust

Target repository: `anne-skydancer/neoj2k-rust`

Base: `main` (`d13082f`)

Head: `safe-simd`

Suggested initial PR state: **draft**, because performance preservation remains unresolved.

## Change

The production `neo-j2k` crate previously allowed unsafe architecture intrinsics inside its SIMD module. This change replaces those implementations with safe Rust arrays and slices and enforces `#![forbid(unsafe_code)]` without a module exception. Normal/build dependencies remain empty; no unsafe implementation is moved into a dependency.

The retained implementation includes safe lane operations, exact sample rounding and grouped encoder quantization, dense horizontal 9/7 lifting with scatter-only outlining, typed wide colour/unary finishing, safe coefficient stores and output packing. Existing compile-time target selection and floating-point operation order are preserved. The native-source policy now rejects unsafe throughout the production crate.

Add independent rounding, quantization, colour, unary, DWT parity/padding/nonfinite, packing and coefficient-store boundary coverage. Include the complete local audit trail: source hashes, drivers, assembly, timing logs, Miri logs, final platform matrices and rejected experiments. Archived experiment sources under `doc/` are evidence, not compiled production implementations. Test-only oracle and benchmark infrastructure remains outside the production library's unsafe prohibition.

## Validation

- Windows full-workspace release tests: baseline **209 passed / 18 ignored**, x86-64-v3 **215 passed / 18 ignored**.
- Linux native library checks: baseline **102 passed / 7 ignored**, x86-64-v3 **108 passed / 7 ignored**.
- Windows std-only release library: **73 passed / 4 ignored**; std-only Clippy with warnings denied passed. Default-feature Clippy also passed.
- Focused actual Miri checks passed for current wide colour/unary finishing and dense DWT; additional focused results and their exact source snapshots are archived.
- Both-platform comparison matrix: **36 processes / 528 output and status checks**; **144 precision/tail checks additionally compare C++ samples/status**. The remaining 384 rows compare original/candidate/unchanged-control Rust implementations. Native C++ differential tests are separate.
- Native-policy self-test/checker, production-source formatting and diff checks passed. Archived raw tool logs retain their emitted whitespace. The policy finds one production crate, 56 source files and no production unsafe/FFI. All 56 production files match the frozen tested source manifest.

Full evidence and reproduction scope: [current status](safe-rust-audit-2026-11-24/current-status.md), [Linux matrix](safe-rust-audit-2026-11-24/linux-broad-matrix/README.md), [Windows matrix](safe-rust-audit-2026-11-24/final-windows-matrix/README.md), [independent remaining-gap review](remaining-gap-independent/README.md).

## Performance limits requiring review

Safety and correctness validation pass; **strict performance preservation is not achieved**. Normal large RGB improves about 21.5% on baseline builds and 2.44% on Linux v3, but regressions remain in both 24-bit and 31-bit requests. Selected v3 medians versus the original follow; the control is an unchanged renamed original, not the prior safe implementation.

| Request | Candidate | Unchanged control |
|---|---:|---:|
| Linux, 24-bit 1024² RGB | +1.08% | +0.19% |
| Windows, 24-bit 1024² RGB | +1.92% | +0.55% |
| Windows, 24-bit 128² RGBA | +5.70% | +2.13% |
| Linux, 31-bit 1024² RGB | +2.09% | +0.04% |
| Windows, 31-bit 1024² RGB | +1.49% | +0.24% |
| Linux, 31-bit 128² RGBA | +2.27% | -0.32% |
| Windows, 31-bit 128² RGBA | +3.19% | +0.48% |

Smaller residuals also occur in ordinary small-image decoding, encoding and reduced packed output. Gray and several other controls move materially, so those timings are inconclusive rather than cleared. Three repeats and context-sensitive diagnostics do not prove absence of regression. No precision restriction, dependency addition or accepted performance tradeoff is implied by this PR.

Fresh instrumented stage accounting points to colour/unary finishing as the remaining opportunity. Allocation-order, relative-plane-padding and green-first scheduling diagnostics did not establish a production fix. Rejected rounding, MQ, coefficient-store and wavelet variants are retained only as historical evidence. This PR does not change the CI workflow or claim to resolve the remaining performance work.
