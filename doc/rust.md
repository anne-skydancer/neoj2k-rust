<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Native Rust port

The `neo-j2k` crate implements JPEG 2000 Part 1 for the same codec scope as neo-j2k's C++
library. The [OpenJPEG departure list](departures.md) documents the shared codestream behavior.
The [workspace README](../README.md) covers the public API, build, and validation commands.

## Decode and encode

The reusable decoder caches a validated main header and resolves component settings for each tile.
Tier-2 gathers code-block contributions from the packets. Tier-1 reconstructs the coefficients before
the inverse wavelet, colour transform, and sample conversion. The serial 9/7 path lifts each row
horizontally just before the vertical pass reads it. A single eligible tile can reconstruct
directly into the caller's I32 planes. Other layouts use the same reconstruction with reusable planar
storage. The reader API buffers bytes as the parser requests them and supports resuming after header
inspection through `DecodeStream`. It discards unused JP2 box payloads in fixed-size chunks and
reports header offsets in the original stream.

The reusable encoder accepts interleaved or planar U8 samples. It implements the forward colour
and wavelet transforms, Tier-1 coding, layer allocation, and Tier-2 packet assembly. The writer API
delivers header, tile, and end-marker bytes in order. The mutable parameter methods expose the layer
ratio and codestream size cap updates made during encoder setup.

## Scheduling and memory

An optional host executor runs disjoint code-block row bands, wavelet rows and stripes, and output
rows for large tiles. Each call joins before the next stage uses its results. Serial replay after an
optional scratch allocation fails preserves the normal pixel and status behavior. Diagnostics keep
their order and can drop individual messages when the allocator cannot store them. The public
budget statistics use the same logical allocation accounting as C++. The retained-work setting
controls the capacity of reusable native buffers separately.

Scalar kernels are available on all supported targets. On x86-64, a build with AVX2 can use vector
wavelet, colour, and sample conversion kernels. Rust callers and their executor threads use their
target's standard floating-point environment. The production crate has no external runtime
dependencies. The C++ oracle is linked into test tools and benchmarks.

## Tier-1 tests and the benchmark

The decoder's and the encoder's Tier-1 are tested against the C++ block by block and decision by decision (README,
"Tier-1 against the C++"), and timed by `neo-j2k-bench` ([bench.md](bench.md)). The benchmark reaches the two
code-block entry points through the off-by-default feature `bench-internals`; the crate's `unsafe` stays in `simd`.
