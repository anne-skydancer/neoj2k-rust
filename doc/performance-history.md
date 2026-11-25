<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Historical performance tables

Archived from the README on 2026-11-25. These tables retain their original dates, revisions,
inputs and harness differences. They describe earlier snapshots and must not be combined into
three-way ratios. Current matched measurements are in the [README](../README.md#performance).
Paths in the archived text are relative to the repository root.

## Performance

"OpenJPEG" below means the pinned OpenJPEG 2.5.4 oracle. There are two direct, paired benchmark suites:
the C++ repository measures OpenJPEG against neo-j2k C++, and this repository measures native Rust against
the same neo-j2k C++ implementation. The fixtures and output APIs differ between suites, so the tables keep
the comparisons separate instead of multiplying them into an invented Rust/OpenJPEG result. In every table
the change is the time of the named build against its stated baseline (minus is faster); every comparison
produces identical decoded samples or encoded bytes. No pass/fail gate is stated for these numbers, so the
tables have no gate column.

### neo-j2k C++ versus OpenJPEG 2.5.4

Whole-image decode and encode, measured by the C++ repository's suites. The baseline is OpenJPEG.

| Operation | Suite | Threads | OpenJPEG | neo-j2k C++ | Change vs OpenJPEG |
|---|---|---:|---:|---:|---:|
| Decode 64² whole | C++ viewer suite | 1 | 373 µs | 316 µs | -15.0 % |
| Decode 1,024² whole | C++ viewer suite | 1 | 37.176 ms | 26.158 ms | -30.0 % |
| Decode 4,096² one-tile 9/7 | C++ large suite | 8 | 136 ms | 46 ms | -66.0 % |
| Encode 1,024² RGB lossy | C++ upload suite | 1 | 240.4 ms | 183.7 ms | -24.0 % |
| Encode 1,024² RGB lossless | C++ upload suite | 1 | 253.9 ms | 178.8 ms | -30.0 % |

- neo-j2k C++ is faster than OpenJPEG on every listed decode, from 1.18× to 2.96×.
- One-thread C++ encoding is 24-30% faster than OpenJPEG at 1,024² in the upload benchmark, measured after the
  C++ encoder's MQ-coder redesign (the C++ README's encode rows; later encoder steps take a few percent more off).

The complete C++/OpenJPEG tables are in the
[neo-j2k C++ README](https://github.com/anne-skydancer/neo-j2k#performance).

### Native Rust versus neo-j2k C++

Direct paired measurements; the baseline is neo-j2k C++.

**These rows were measured against earlier neo-j2k C++ code, before the C++ 0.1.3 encoder and decoder redesign, and do not describe 0.1.3.** Against C++ 0.1.3 (2026-11-23) the Rust twin decodes large images about level or faster on the baseline CPU level, is slower on `x86-64-v3` 1,024² decodes (+4.5 % to +19.4 %), and is 30 % to 73 % slower on small encodes; the numbers are in `doc/bench.md`, "Baseline against neo-j2k C++ 0.1.3". The port of the missing Tier-1 steps is under way and these tables are re-measured after it.

| Operation | Suite | Threads | neo-j2k C++ | neo-j2k Rust | Change vs C++ |
|---|---|---:|---:|---:|---:|
| Decode 1,024² RGB 9/7 | direct I32 AVX2 suite | 1 | 4.780 ms | 4.725 ms | -1.1 % |
| Decode 1,024² RGB 9/7 | direct I32 AVX2 suite | 8 | 1.413 ms | 1.372 ms | -2.9 % |
| Decode 4,096² grayscale 9/7 | direct I32 AVX2 suite | 8 | 3.158 ms | 3.017 ms | -4.4 % |
| Encode 32²-128² RGB | paired AVX2 suite | 1 | baseline | within -2.1 % to +1.3 % | Rust is within about 2% of C++ |

- Native Rust is slightly faster than C++ on the listed large AVX2 decodes, by 1-4% while preserving the same
  multi-core scaling model.
- Rust small-image encoding is approximately equal to C++. Native Rust encoding scales by 4.27-5.47× lossless and
  4.36-5.38× lossy at eight workers on 256²-1,024² inputs. A direct three-way large-encode ratio is not
  claimed because the OpenJPEG/C++ and Rust scaling suites use different inputs and harnesses.

The small-image release tests compare whole, reduced, and partial decodes against C++ and check
that a warmed Rust decoder performs no heap calls on the selected fixtures. The opt-in timing
tests use matching inputs and settings, and compare lossless and lossy encodes byte for byte:

```sh
cargo test --release -p neo-j2k --test small_image_gates -- --ignored --nocapture --test-threads=1
```

### Large-image decode scaling

The large-image benchmark compares direct caller-owned I32 planes using the same persistent
executor at widths 1, 2, 4 and 8. Use a C++ oracle configured with `-DNJ2K_SIMD=SCALAR` for
scalar timing and the AVX2 oracle for an `x86-64-v3` Rust build.

```sh
cargo test --release -p neo-j2k --test small_image_gates -- --ignored --exact large_image_executor_latency --nocapture
```

The same 21 November 2026 system produced the following large-image scaling results. Both codecs
wrote caller-owned I32 planes through the same persistent pool; worker count includes the calling
thread. Each latency is the median of nine alternating rounds with two decodes per round.

Rust against C++ (the change is Rust's time against C++'s):

| Build | Fixture | Workers | Rust ms | C++ ms | Change vs C++ |
|---|---|---:|---:|---:|---:|
| Scalar | 1024² RGB 9/7 | 1 | 7.694 | 8.459 | -9.0 % |
| Scalar | 1024² RGB 9/7 | 2 | 4.452 | 4.887 | -8.9 % |
| Scalar | 1024² RGB 9/7 | 4 | 2.720 | 2.940 | -7.5 % |
| Scalar | 1024² RGB 9/7 | 8 | 1.815 | 1.881 | -3.5 % |
| Scalar | 4096² grayscale 9/7 | 1 | 23.240 | 28.279 | -17.8 % |
| Scalar | 4096² grayscale 9/7 | 2 | 12.301 | 14.983 | -17.9 % |
| Scalar | 4096² grayscale 9/7 | 4 | 7.229 | 9.064 | -20.2 % |
| Scalar | 4096² grayscale 9/7 | 8 | 4.516 | 5.298 | -14.8 % |
| AVX2 | 1024² RGB 9/7 | 1 | 4.725 | 4.780 | -1.1 % |
| AVX2 | 1024² RGB 9/7 | 2 | 2.999 | 3.041 | -1.4 % |
| AVX2 | 1024² RGB 9/7 | 4 | 1.865 | 1.984 | -6.0 % |
| AVX2 | 1024² RGB 9/7 | 8 | 1.372 | 1.413 | -2.9 % |
| AVX2 | 4096² grayscale 9/7 | 1 | 11.025 | 11.960 | -7.8 % |
| AVX2 | 4096² grayscale 9/7 | 2 | 6.520 | 6.992 | -6.7 % |
| AVX2 | 4096² grayscale 9/7 | 4 | 4.138 | 4.840 | -14.5 % |
| AVX2 | 4096² grayscale 9/7 | 8 | 3.017 | 3.158 | -4.4 % |

Scaling of each codec against its own one-worker time (speed-up factors, as measured):

| Build | Fixture | Workers | Rust speedup | C++ speedup |
|---|---|---:|---:|---:|
| Scalar | 1024² RGB 9/7 | 1 | 1.00× | 1.00× |
| Scalar | 1024² RGB 9/7 | 2 | 1.73× | 1.73× |
| Scalar | 1024² RGB 9/7 | 4 | 2.83× | 2.88× |
| Scalar | 1024² RGB 9/7 | 8 | 4.24× | 4.50× |
| Scalar | 4096² grayscale 9/7 | 1 | 1.00× | 1.00× |
| Scalar | 4096² grayscale 9/7 | 2 | 1.89× | 1.89× |
| Scalar | 4096² grayscale 9/7 | 4 | 3.21× | 3.12× |
| Scalar | 4096² grayscale 9/7 | 8 | 5.15× | 5.34× |
| AVX2 | 1024² RGB 9/7 | 1 | 1.00× | 1.00× |
| AVX2 | 1024² RGB 9/7 | 2 | 1.58× | 1.57× |
| AVX2 | 1024² RGB 9/7 | 4 | 2.53× | 2.41× |
| AVX2 | 1024² RGB 9/7 | 8 | 3.44× | 3.38× |
| AVX2 | 4096² grayscale 9/7 | 1 | 1.00× | 1.00× |
| AVX2 | 4096² grayscale 9/7 | 2 | 1.69× | 1.71× |
| AVX2 | 4096² grayscale 9/7 | 4 | 2.66× | 2.47× |
| AVX2 | 4096² grayscale 9/7 | 8 | 3.65× | 3.79× |

### Encode scaling

The native encoder uses the same borrowed-executor model. It divides component wavelets and the
flattened Tier-1 code-block sequence into bounded worker-local batches while retaining codestream
order and byte-for-byte serial output. On 22 November 2026, the release benchmark on the same system
measured these median end-to-end encode latencies:

| Fixture | Workers 1 | Workers 2 | Workers 4 | Workers 8 | 8-worker speedup |
|---|---:|---:|---:|---:|---:|
| 256² RGB lossless 5/3 | 15.818 ms | 9.463 ms | 5.669 ms | 3.706 ms | 4.27× |
| 256² RGB lossy 9/7 | 19.992 ms | 11.147 ms | 6.602 ms | 4.586 ms | 4.36× |
| 512² RGB lossless 5/3 | 59.154 ms | 32.906 ms | 18.070 ms | 10.895 ms | 5.43× |
| 512² RGB lossy 9/7 | 78.967 ms | 42.944 ms | 23.293 ms | 14.771 ms | 5.35× |
| 1,024² RGB lossless 5/3 | 222.519 ms | 124.848 ms | 67.604 ms | 40.648 ms | 5.47× |
| 1,024² RGB lossy 9/7 | 319.312 ms | 180.977 ms | 95.287 ms | 59.399 ms | 5.38× |

Every worker count emits exactly the serial codestream. Tests cover widths 2, 4, 8, 16, and 64,
and enforce that 128² stays serial while 256² uses the supplied executor. Reproduce the table with:

```sh
cargo test --release -p neo-j2k --test small_image_gates -- --ignored --exact large_encoder_executor_latency --nocapture --test-threads=1
```

An `x86-64-v3` assessment produced no consistent encoder improvement: eight-worker speedups ranged
from 4.15-5.61× lossless and 4.28-5.51× lossy, but absolute times moved within noise in both
directions. The encoder therefore retains the profitable SIMD-friendly eight-column 5/3 wavelet
layout and compile-time AVX2 target support without an encoder-only intrinsic path. Tier-1's MQ coder
is sequential within each code-block, so independent code-block parallelism supplies the material gain.

### Small images and the parallel crossover

The decoder deliberately does not query the host executor below 2^18 component samples: the C++
measurements found small decodes about 2% slower when dispatched across threads. This keeps the
measured small-image path serial while applying parallel Tier-1, wavelet, colour/output, and clearing
work to images large enough to benefit. The `decode_executor` tests enforce both sides of that
crossover and compare scheduled output with serial output at job widths 2, 4, 8, 16, and 64.

On 21 November 2026, paired runs on a Ryzen 9 9950X3D under Windows gave these changes in median
Rust latency against C++ for warmed, full-resolution, whole-image caller-owned I32 decodes.
Each reported latency is the median of 21 alternating Rust/C++ rounds with 16 decodes per
round. Rust 1.100.0 and MSVC 19.51 release builds used the same scalar or x86-64-v3/AVX2
instruction-set floor.

| Fixture | Scalar | AVX2 |
|---|---:|---:|
| 32 × 32 RGBA, 5/3 | +2.8 % | +0.9 % |
| 64 × 64 RGB, 9/7 | -11.5 % | +5.0 % |
| 128 × 128 RGBA, 9/7 | -13.8 % | 0.0 % |

Across those fixtures at full resolution, reduce 2, and with a missing EOC marker, the change in
caller-owned decode time ranged from -21.7 % to +8.2 % in the scalar build and -3.8 % to +11.1 % with
AVX2. The change in encoder time over 32², 64², and 128² RGB inputs using both 5/3 and 9/7 ranged from -0.2 % to +5.8 % scalar
and -2.1 % to +1.3 % with AVX2. A negative change means Rust was faster. This is a small-image latency
assessment, not a throughput claim; the C++ measurement includes its test shim constructing plane
descriptors, and encoder calls include the shim's input/output copies. Reproduce it with the ignored
`small_image_gates` timing tests above on an otherwise quiet system.

The warmed allocation test covers both output APIs on seven fixtures, including packed headers,
16-bit samples and 4:2:0 subsampling.
