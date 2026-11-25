<!-- SPDX-License-Identifier: BSD-2-Clause -->

# Rebuilt main: Linux/WSL2 benchmarks, 25 November 2026

Rust main: `b4240d6a33baafaf843cd027c83eee1f3e1fde3d` (merged PR #23; package version 0.1.3).
C++: `81407c2ef7cfa146905678f09a6c11295e52b15c`. The OpenJPEG fork and pin are recorded verbatim
in `manifest.json`. This is a fresh rebuild from clean native Linux checkouts under
`/tmp/neoj2k-main-rebuild-20261125`, using the existing tracked three-way driver.

## Environment and scope

AMD Ryzen 9 9950X3D; Arch Linux in WSL2; Rust 1.100.0; GCC/G++ 16.2.1;
Release/O3 and x86-64-v3/AVX2; strict floating point. Rust uses fat LTO and one codegen
unit, default features and no stage timers. C++ retains no IPO. The production
dependency tree and feature checks in `tools/bench_three_way/run.py` passed.

The twelve fresh-codec requests use one serial thread, pinned to guest CPU
0. Each of three processes covers all six actor orders twice,
with 32 preconditioning calls before each actor batch. Decoder output is caller-owned
I32; OpenJPEG allocates internal planes and copies them out. Encoders return owned
bytes. Parsing, internal allocation, conversion and teardown are included; input
loading, caller buffer allocation and validation are excluded. All 36 three-way
sample/encoded-byte checks passed, before and after their timed workload.

The estimator is the median of three process medians. Observed process ranges are
not confidence intervals. These synthetic 8-bit requests do not establish a universal
codec ranking. The separate Windows U8/Kakadu run is not mixed into this table.

## Latest matched latencies (milliseconds)

| Operation and input | neo-j2k C++ | neoj2k-rust | OpenJPEG 2.5.4 oracle | Rust speedup vs OpenJPEG |
|---|---:|---:|---:|---:|
| Decode 32 × 32 RGBA, 5/3 | 0.1714 | 0.1593 | 0.3825 | 2.40× |
| Decode 64 × 64 RGB, 9/7 | 0.0502 | 0.0575 | 0.1020 | 1.78× |
| Decode 128 × 128 RGBA, 9/7 | 0.1107 | 0.1194 | 0.3840 | 3.22× |
| Decode 64 × 64 RGB, 9/7 → 16 × 16 | 0.0206 | 0.0265 | 0.0303 | 1.14× |
| Decode 1,024 × 1,024 RGB, 9/7 | 4.2961 | 4.0680 | 13.6651 | 3.36× |
| Decode 4,096 × 4,096 grayscale, 9/7 | 11.9264 | 11.2747 | 83.6997 | 7.42× |
| Encode 32 × 32 RGB, lossless 5/3 | 0.1186 | 0.1133 | 0.2907 | 2.56× |
| Encode 32 × 32 RGB, lossy 9/7 | 0.1144 | 0.1158 | 0.3266 | 2.82× |
| Encode 64 × 64 RGB, lossless 5/3 | 0.6131 | 0.6132 | 1.0000 | 1.63× |
| Encode 64 × 64 RGB, lossy 9/7 | 0.6393 | 0.6824 | 1.2012 | 1.76× |
| Encode 128 × 128 RGB, lossless 5/3 | 2.3638 | 2.3659 | 3.4421 | 1.45× |
| Encode 128 × 128 RGB, lossy 9/7 | 2.4027 | 2.5432 | 4.3509 | 1.71× |


Speedup is OpenJPEG time divided by Rust time, using unrounded medians.

## Process median ranges

| Operation and input | C++ process range (ms) | Rust process range (ms) | OpenJPEG process range (ms) |
|---|---:|---:|---:|
| Decode 32 × 32 RGBA, 5/3 | 0.1702–0.1738 | 0.1578–0.1632 | 0.3815–0.3867 |
| Decode 64 × 64 RGB, 9/7 | 0.0499–0.0512 | 0.0569–0.0594 | 0.0933–0.1026 |
| Decode 128 × 128 RGBA, 9/7 | 0.1086–0.1128 | 0.1185–0.1219 | 0.3781–0.3917 |
| Decode 64 × 64 RGB, 9/7 → 16 × 16 | 0.0206–0.0208 | 0.0264–0.0288 | 0.0297–0.0321 |
| Decode 1,024 × 1,024 RGB, 9/7 | 4.2700–4.2969 | 4.0461–4.0747 | 13.6283–13.9175 |
| Decode 4,096 × 4,096 grayscale, 9/7 | 11.7298–12.1419 | 11.0527–11.3586 | 83.1869–83.9515 |
| Encode 32 × 32 RGB, lossless 5/3 | 0.1176–0.1245 | 0.1097–0.1144 | 0.2804–0.2908 |
| Encode 32 × 32 RGB, lossy 9/7 | 0.1134–0.1201 | 0.1102–0.1233 | 0.3261–0.3363 |
| Encode 64 × 64 RGB, lossless 5/3 | 0.6032–0.6246 | 0.6116–0.6443 | 0.9986–1.0152 |
| Encode 64 × 64 RGB, lossy 9/7 | 0.6366–0.6650 | 0.6622–0.6944 | 1.1864–1.3443 |
| Encode 128 × 128 RGB, lossless 5/3 | 2.3063–2.3795 | 2.3626–2.3863 | 3.4063–3.6538 |
| Encode 128 × 128 RGB, lossy 9/7 | 2.3971–2.4328 | 2.5271–2.5850 | 4.3320–4.4760 |


## Before/after regression audit

This separate Linux run compares exact pre-change Rust `3000f7d` with merged main
`b4240d6` in one executable. The existing 23-request versions suite is used, with
retained decoders/caller-owned I32 output and fresh encoders. Large decodes use host
executor widths 1/2/4/8 without restricting the process to one CPU. Sides alternate
every round; the starting side rotates between three processes. Every case's output
comparison passed in all three processes. Raw rounds, source inventory and executable
hash are in `paired-*.txt` and `paired-manifest.json`. Samples/bytes are compared before
timing, while timed calls retain status checks.

The oracle sources are exact Git blobs; only their test-package manifest has a unique
package name for side-by-side linking. This avoids accidental Cargo discovery through
historical audit manifests containing absolute local paths. No codec source was edited.
Positive changes below mean the rebuilt crate took longer. Do not attribute apparent
9/7 or encode gains directly to a reversible 5/3 geometry predicate: layout, order,
host scheduling and measurement variation have not been isolated.

| Request / executor width | Before µs | Rebuilt µs | Time change | Process ratio range |
|---|---:|---:|---:|---|
| large_1024_rgb_97.j2k 1 | 4052.243 | 4088.781 | +0.90% | -0.69% to +1.88% |
| large_1024_rgb_97.j2k 2 | 4088.791 | 4083.431 | -0.13% | -1.42% to -0.13% |
| large_1024_rgb_97.j2k 4 | 4989.277 | 5069.700 | +1.61% | -23.48% to +1.61% |
| large_1024_rgb_97.j2k 8 | 7578.796 | 7695.959 | +1.55% | -1.70% to +1.84% |
| large_4096_g_97.j2k 1 | 12410.978 | 11844.102 | -4.57% | -5.47% to -3.02% |
| large_4096_g_97.j2k 2 | 8567.406 | 8477.827 | -1.05% | -4.37% to -0.14% |
| large_4096_g_97.j2k 4 | 6863.703 | 6232.790 | -9.19% | -10.59% to -1.99% |
| large_4096_g_97.j2k 8 | 7409.512 | 7253.729 | -2.10% | -5.92% to +1.39% |
| sl_32_rgba_53.j2k / reduce 0 / whole | 205.407 | 199.921 | -2.67% | -8.48% to -1.20% |
| sl_32_rgba_53.j2k / reduce 2 / whole | 15.932 | 15.889 | -0.27% | -1.96% to +0.64% |
| sl_32_rgba_53.j2k / reduce 0 / missing EOC | 201.830 | 202.490 | +0.33% | +0.33% to +12.09% |
| sl_64_rgb_97.j2k / reduce 0 / whole | 55.688 | 54.703 | -1.77% | -2.45% to -1.53% |
| sl_64_rgb_97.j2k / reduce 2 / whole | 22.350 | 22.155 | -0.87% | -5.22% to +1.54% |
| sl_64_rgb_97.j2k / reduce 0 / missing EOC | 55.257 | 54.989 | -0.49% | -0.49% to +0.48% |
| sl_128_rgba_97.j2k / reduce 0 / whole | 119.172 | 115.348 | -3.21% | -5.60% to -0.34% |
| sl_128_rgba_97.j2k / reduce 2 / whole | 60.971 | 60.849 | -0.20% | -0.48% to +0.43% |
| sl_128_rgba_97.j2k / reduce 0 / missing EOC | 118.452 | 116.885 | -1.32% | -6.10% to +1.04% |
| encode 32x32 RGB / lossless 5/3 | 144.145 | 145.083 | +0.65% | +0.01% to +5.64% |
| encode 32x32 RGB / lossy 9/7 | 156.094 | 158.079 | +1.27% | -5.48% to +3.70% |
| encode 64x64 RGB / lossless 5/3 | 636.432 | 642.396 | +0.94% | -1.03% to +1.26% |
| encode 64x64 RGB / lossy 9/7 | 685.365 | 682.377 | -0.44% | -0.44% to +0.26% |
| encode 128x128 RGB / lossless 5/3 | 2379.933 | 2369.181 | -0.45% | -1.18% to +0.49% |
| encode 128x128 RGB / lossy 9/7 | 2574.890 | 2556.919 | -0.70% | -0.71% to -0.50% |


## Wide-image Linux check

A separate, serial CPU-0 retained-decoder I32 comparison of
`large_2048x64_rgba_53.j2k` gives **1.8634 ms before and
1.2647 ms rebuilt (-32.13% time change)**.
The two outputs match; three processes each alternate the actors over 21 rounds
of 16 calls. This differs from the Windows fresh-decoder U8 scope; their percentage
gains must not be combined. `wide-paired-*` retains raw rounds and the summary,
including executable/harness hashes. `paired-harness/` holds the 23-request source
and its subsequent wide-only entry point.

## Evidence and reproduction

- `summary.json`, `rep-1.jsonl` through `rep-3.jsonl`: all fresh three-way rounds/checks.
- `manifest.json`: source/compiler/CPU/ISA/protocol, source/library/executable hashes.
- `paired-summary.json`, `paired-manifest.json`, `paired-*.txt`: the independent 23-request audit.
- The existing Linux driver performs the source builds and validations:

```sh
python3 tools/bench_three_way/run.py --cpp-source ../neo-j2k --out /tmp/neoj2k-three-way-rebuilt
```

The package was also rebuilt and verified with `cargo package -p neo-j2k --locked`.
The local `.crate` remains version 0.1.3; no replacement release or crates.io publication
was performed. Earlier Linux source/build measurements remain in
[`../2026-11-25-three-way/`](../2026-11-25-three-way/README.md). Full Windows
results and the fresh U8 regression sweep are in
[`../2026-11-25-main-rebuild-windows/`](../2026-11-25-main-rebuild-windows/README.md).
