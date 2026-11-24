<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Independent review of the remaining safe-Rust precision gap

Read-only review of the frozen `neoj2k-rust-safe-simd` candidate. No codec source, dependency, CI file, or existing audit was changed by this reviewer. The source still forbids unsafe and uses no normal/build dependencies. The existing final Linux/Windows matrix remains the performance-acceptance evidence; the new diagnostics below do not supersede it or establish a retained fix.

The prior stage checkpoints predate typed finishing and scatter outlining. They were insufficient to attribute the current roughly 2% precision-31 gap, so the existing original/candidate/unchanged-control stage harness was rebuilt against the frozen candidate in a separate Linux target. Source hashes and normalized Windows/Linux correspondence are recorded in [manifest-and-stages.json](manifest-and-stages.json). The raw DWT hashes differ because of line endings; normalized source matches.

## Current stage accounting

One pinned Linux x86-64-v3 process, 24 balanced-order rounds, batch 128 for small fixtures and 2 for large. Values below are accumulated stage time divided by call count: **means, not paired percentage medians**. All three Rust sample outputs matched. The stage feature adds clocks/atomics and can alter caller code generation, so it is an attribution diagnostic, not uninstrumented acceptance timing.

| Fixture / stage | Original µs | Safe µs | Unchanged control µs |
|---|---:|---:|---:|
| p31 large 1,024² / Tier-1 | 2,638.668 | 2,607.119 | 2,592.978 |
| p31 large 1,024² / wavelet | 1,219.101 | 1,224.826 | 1,230.152 |
| p31 large 1,024² / colour | 249.920 | 305.679 | 248.804 |
| p31 128² RGBA / Tier-1 | 60.521 | 61.610 | 60.249 |
| p31 128² RGBA / wavelet | 41.886 | 41.864 | 41.730 |
| p31 128² RGBA / colour | 3.805 | 4.712 | 5.827 |
| p31 128² RGBA / unary finish | 0.827 | 1.188 | 0.834 |

Large precision-31 colour finishing retains about 56 µs excess against both original/control in this instrumented context. Against an approximately 4 ms uninstrumented call, that is the scale of a 1.4% opportunity, not a measured decomposition of the separate 2.09% acceptance result. At 128², colour/unary excess against original totals about 1.27 µs; the large unchanged-control colour variation cautions against treating a single instrumented process as a precise causal budget.

These observations support concentrating the next bounded investigation on finishing rather than another MQ or general DWT variant. They do not explain every remaining cycle. [Raw stage rows](stages-current-v3-1.txt), [build log](stage-build-v3.txt), source/executable hashes and the harness sources are preserved here.

The current stage executable's unsigned MCT helper is 708 bytes / 148 normalized instructions and matches the frozen broad-matrix helper instruction-for-instruction after relocation normalization; see [comparison](mct-stage-vs-broad-code.json) and [disassembly](mct-unsigned-stage.asm.txt). Its main loop is eight-wide, has no vector register spills, and repeats one coefficient broadcast. The [unary helper](unary-unsigned-stage.asm.txt) already interleaves four YMM groups / 32 samples; its stack saves serve later cleanup, not a spill/reload in each hot iteration. This rules out casually proposing more unary unrolling or assuming the remaining helpers still scalarize all conversions.

## Relative-plane-padding diagnostic

The final MCT machine loop stores red before loading the original U plane. At 1,024² each typed plane occupies 4 MiB; at 128² each occupies 64 KiB. Contiguous planes therefore have the same low 12 address bits. This suggests a **possible**, unproven store/load alias-pressure mechanism. The earlier allocation-order test changes the whole buffer's address but leaves relative plane spacing fixed, so it cannot isolate that mechanism.

A scratch-only default-feature harness inserted 0, 32 or 64 bytes between caller-owned plane slices while retaining exact lengths, data, API and codec code. No production layout changed. Nine pinned processes rotated gap order over three repetitions. Printed addresses were checked: all 54 original/candidate/control plane layouts have the requested relative spacing. Eighteen precision rows matched samples across all three Rust variants and C++; C++ status was zero and Rust decode Results succeeded. This scratch timing harness does not separately assert the returned Rust truncation flag; existing full-matrix checks remain status acceptance evidence.

| Fixture | Gap between planes | Safe/original median | Unchanged control/original median |
|---|---:|---:|---:|
| p31 1,024² | 0 B | +0.486% | +0.255% |
| p31 1,024² | 32 B | +0.383% | +0.071% |
| p31 1,024² | 64 B | +0.198% | −0.093% |
| p31 128² | 0 B | +1.946% | +1.412% |
| p31 128² | 32 B | +0.445% | −1.241% |
| p31 128² | 64 B | +1.072% | +0.849% |

Each cell is a median of the processes' paired changes, not a ratio of independent time medians. [All raw logs, ranges and layout-verification counts](padding-aggregate.json) are preserved. These results **do not establish a candidate-specific padding win**: control movement accounts for much of the visual change, particularly at 128². The large-image gap is already smaller in this new executable even without padding, consistent with the previously observed context sensitivity. Neither effect dismisses the frozen broad-matrix gap or proves 4 KiB aliasing. No hardware counters were available to identify a memory-disambiguation stall.

The experiment is diagnostic only. Inserting padding into production storage, weakening alignment guarantees, or declaring performance preserved from these rows is not justified.

## One bounded source experiment: no material unsigned-target kernel win

The one additional source experiment undertaken was **green-first completion in the current typed wide MCT loop**, at `neo-j2k/src/simd/mct.rs:270`: keep exactly the current green expression `(y - u*0.34413) - v*0.71414`, red/blue expressions, typed rounding, clamps and per-lane arithmetic association, but finish/store green before red and blue. Green depends on all Y/U/V inputs, whereas the current red-first schedule allows the compiler to sink U's load past the red store. This was a test of load/live-range scheduling, not another rounding algorithm or a claim that aliasing was proved. Earlier array/sequential trials used older unstable or f64 kernels; they did not provide a clean current-typed comparison.

A standalone scratch driver, with unsafe forbidden and no dependencies, compiled current/green-first/identical-current-control together at Rust 1.100.0 v3 release fat LTO/one codegen unit. The current/control typed helpers merged to the same address. The principal assembly gate required all three loads before the first store, unchanged vector width/three-operation rounding, and no additional hot spills or calls. Repeated broadcasts were inspected as a regression risk. LLVM can reorder independent source stores, so source order alone was not evidence.

Green-first passed those principal checks: all three inputs loaded before its first store, still eight-wide without hot spills/calls, with the same three-operation rounder. Its helper shrank from 708 to 692 bytes. Two coefficient broadcasts remained inside every vector iteration, versus one in current code: greater simultaneous input liveness displaced a loop-invariant coefficient. The coordinating reviewer requested measurement rather than rejecting it from that instruction count alone. See [source diff](green-first.patch), [gate record](green-first-gate.json), [green helper](green-unsigned.asm.txt), and [merged current/control helper](current-control-unsigned.asm.txt).

Each of three pinned processes passed 715,914 exact current/green/control value comparisons over all 31 precisions, both signs, random float bits and widths 0/1/7/8/9/15/16/17/25/33/128/1,024. The bounded kernel comparison reused the **same backing buffer for every variant**, restored inputs outside timing, and used contiguous 1,024-value plane streams with optional 32/64-byte gaps. There were 128 frames per timed batch and 30 balanced-order rounds; this is a warmed synthetic kernel measurement, not the public decoder.

| Finish | Relative plane gap | Green-first/current median | Identical current control/current median |
|---|---:|---:|---:|
| unsigned p31 | 0 B | −0.292% | +0.132% |
| unsigned p31 | 32 B | +1.263% | +0.023% |
| unsigned p31 | 64 B | +0.806% | −0.373% |
| signed p31 | 0 B | −1.088% | +0.241% |
| signed p31 | 32 B | +0.575% | +0.104% |
| signed p31 | 64 B | +0.552% | −0.159% |

The original failing public precision fixtures are unsigned. Their contiguous-kernel lead was below the 1% floor, and padded cases became slower. Precision-24 contiguous cases improved about 0.7–0.8%, while padded cases regressed about 0.8–1.5%. There is no robust general improvement or a material unsigned-p31 lead supporting another full codec matrix. No public codec timing, Windows port or production change followed. [All kernel rows and process ranges](green-kernel-aggregate.json), raw logs and the standalone driver are preserved; this diagnostic does not establish interpreted Miri/C++ validation of the scratch permutation.

No second source change is justified by this review's evidence. In particular, it does not recommend repeating the rejected encoded-bias, tail-removal, fused-DWT, T1-width/store, or BYTEIN trials. Finishing remains the measured opportunity, but a further safe implementation that closes it has not yet been demonstrated. Any future alternative needs a new mechanism, exact all-precision/tail checks and public default-feature acceptance; removing a coefficient broadcast or changing store order alone must not be promoted to a fix.

## Reproduction

The preserved harness manifests and sources are `harness-stage-*` and `harness-independent-padding-*` in this directory. Original/candidate/control paths point into `/home/dev/safe-crate-audit/`; the checked candidate source is unchanged. The stage target was `/home/dev/safe-crate-audit/target-independent-stage-v3`, padding target `target-independent-padding-v3`, with Rust 1.100.0 release fat LTO / one codegen unit and `RUSTFLAGS=-C target-cpu=x86-64-v3`. The inherited Windows `RUSTC_WRAPPER` had to be cleared and the Linux Rust compiler selected explicitly; this corrected an initial setup failure before the successful build.

Stage run: `taskset -c 7 target-independent-stage-v3/release/neo-j2k-versions`. Padding runs set `INDEPENDENT_PLANE_GAP_SAMPLES` to 0, 8 or 16 and invoke the corresponding default-feature executable on CPU 7. Padding process order was 0/8/16, 16/0/8, 8/16/0. Each fixture uses 24 interleaved rounds, batch 32 small / 2 large. Full commands/outputs are in the build/raw logs; [manifest](manifest-and-stages.json) and [padding aggregate](padding-aggregate.json) record current executable hashes. No commit or push was made.
