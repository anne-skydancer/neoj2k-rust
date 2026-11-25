<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Matched three-way API latency — 2026-11-25

This run measures committed Rust source, not the rejected isolated audit prototypes.
Rust: `28d96e9f4fd86f003c97f5bc0d79794ac756caf8`; C++:
`81407c2ef7cfa146905678f09a6c11295e52b15c` (codec version 0.1.3).
OpenJPEG is the vendored 2.5.4 fork pinned at `66b1656192fc621a4b26aa86d05b6a00514a7cee`,
with no extra oracle patches. It is not an unmodified upstream distribution benchmark.

## Environment and protocol

AMD Ryzen 9 9950X3D, Linux under WSL2, eight guest logical CPUs visible; serial execution
pinned to guest CPU 0. Rust 1.100.0, GCC/G++ 16.2.1. All libraries are source-built;
Release/O3 and x86-64-v3/AVX2, no floating-point contraction/fast-math. Rust uses fat LTO
and one codegen unit; C++ keeps the project's no-IPO policy. Only one original Rust crate
and one copy of each native codec are linked. Default Rust features; no stage clocks.

Each process covers all six codec orders twice (12 rounds); 32 calls of the actor about
to be timed precede every batch. Small decode batches have eight calls, large decodes one,
encodes four. Three fresh processes rotate the starting order and caller output-buffer
allocation. Published values are the median of the three process medians. Raw values and
process ranges are retained rather than declaring near-equal values a universal win.

## Measurement attribution

The `neoj2k-rust` result records come from `rust_decode` and the Rust encoder closure in
`tools/bench_three_way/src/main.rs`, which invoke this crate's public APIs. The `openjpeg`
records come from `opj_decode` and `openjpeg::encode_u8`, which invoke OpenJPEG's API.
The `neo-j2k` records separately measure the C++ codec. Both the three-way README table
and its dedicated Rust/OpenJPEG table use these direct measurements. No Rust latency
is calculated from a C++-versus-OpenJPEG ratio or copied from a C++ viewer/upload result.

## What is timed

All decoders use fresh codec instances on every call, including header parsing, allocation
and teardown. Caller-owned I32 buffers are allocated outside timing; descriptor/adapter
work is included. OpenJPEG allocates its own image planes then copies to the caller output;
Rust and neo-j2k can decode directly into caller output. This is fresh-codec request latency
with warm input/allocators, distinct from the retained-decoder audit workloads.

All encoders consume the same generated interleaved RGB bytes and return owned codestreams.
Creation, conversion, allocation, the final owned copy and teardown are timed. One tile,
RLCP, six resolutions, 64×64 code blocks, MCT, one layer/all passes retained; reversible
5/3 or irreversible 9/7. No positive rate target is set. File I/O and output validation
are outside timing; status checks remain active inside calls.

All twelve workloads passed sample/encoded-byte equality before and after timing in all
three processes (36 three-way case checks). Decode buffers are poisoned before final
validation to detect unwritten samples. The 8-bit synthetic fixtures do not characterize
every photographic, high-precision, truncated, streaming or parallel workload.

## Median latencies (ms)

| Operation and input | neo-j2k C++ | neoj2k-rust | OpenJPEG 2.5.4 oracle | Rust speedup vs OpenJPEG |
|---|---:|---:|---:|---:|
| Decode 32 × 32 RGBA, 5/3 | 0.166 | 0.149 | 0.375 | 2.52× |
| Decode 64 × 64 RGB, 9/7 | 0.0515 | 0.0565 | 0.105 | 1.86× |
| Decode 128 × 128 RGBA, 9/7 | 0.11 | 0.115 | 0.374 | 3.26× |
| Decode 64 × 64 RGB, 9/7 → 16 × 16 | 0.0208 | 0.0264 | 0.0316 | 1.20× |
| Decode 1,024 × 1,024 RGB, 9/7 | 4.27 | 4.01 | 14.1 | 3.52× |
| Decode 4,096 × 4,096 grayscale, 9/7 | 11.8 | 11 | 82.6 | 7.53× |
| Encode 32 × 32 RGB, lossless 5/3 | 0.11 | 0.113 | 0.282 | 2.51× |
| Encode 32 × 32 RGB, lossy 9/7 | 0.114 | 0.11 | 0.322 | 2.93× |
| Encode 64 × 64 RGB, lossless 5/3 | 0.606 | 0.617 | 0.995 | 1.61× |
| Encode 64 × 64 RGB, lossy 9/7 | 0.639 | 0.662 | 1.19 | 1.80× |
| Encode 128 × 128 RGB, lossless 5/3 | 2.31 | 2.35 | 3.41 | 1.45× |
| Encode 128 × 128 RGB, lossy 9/7 | 2.41 | 2.57 | 4.29 | 1.67× |

Rust speedup is **OpenJPEG time ÷ Rust time**, calculated from the unrounded matched medians.
A value above 1× means Rust completed the same request faster.

## Process median ranges (ms)

| Operation and input | neo-j2k C++ | neoj2k-rust | OpenJPEG oracle |
|---|---:|---:|---:|
| Decode 32 × 32 RGBA, 5/3 | 0.1622–0.1699 | 0.1464–0.1493 | 0.3742–0.3769 |
| Decode 64 × 64 RGB, 9/7 | 0.05009–0.05335 | 0.05613–0.05665 | 0.09818–0.1092 |
| Decode 128 × 128 RGBA, 9/7 | 0.107–0.1119 | 0.1135–0.1187 | 0.3731–0.3738 |
| Decode 64 × 64 RGB, 9/7 → 16 × 16 | 0.02069–0.0209 | 0.02617–0.02652 | 0.03121–0.0335 |
| Decode 1,024 × 1,024 RGB, 9/7 | 4.25–4.28 | 4.01–4.021 | 13.92–14.3 |
| Decode 4,096 × 4,096 grayscale, 9/7 | 11.57–12.13 | 10.66–11.18 | 82.25–83.1 |
| Encode 32 × 32 RGB, lossless 5/3 | 0.1092–0.1133 | 0.1096–0.1218 | 0.2721–0.2825 |
| Encode 32 × 32 RGB, lossy 9/7 | 0.1122–0.1152 | 0.1095–0.1107 | 0.3213–0.3282 |
| Encode 64 × 64 RGB, lossless 5/3 | 0.6045–0.6068 | 0.6123–0.6184 | 0.9937–0.9998 |
| Encode 64 × 64 RGB, lossy 9/7 | 0.6373–0.6397 | 0.6606–0.6625 | 1.184–1.194 |
| Encode 128 × 128 RGB, lossless 5/3 | 2.299–2.324 | 2.338–2.349 | 3.408–3.418 |
| Encode 128 × 128 RGB, lossy 9/7 | 2.404–2.457 | 2.518–2.586 | 4.268–4.34 |

## Reproduction and evidence

See [the benchmark instructions](../../../tools/bench_three_way/README.md).
`manifest.json` records source/compiler/CPU/ISA/protocol details plus source, driver,
fixture, native-library and executable hashes. `summary.json` contains all process
medians and ranges; `rep-1.jsonl` through `rep-3.jsonl` retain every round and check.
`compile_commands.json` and build logs show actual native optimization/FP/ISA flags;
`cargo-build.txt`, `production-tree.txt` and `feature-tree.txt` show the original Rust
package, default features and absence of production dependencies/stage instrumentation.
Build trees and binaries remain local and are not part of the documentation artifacts.
