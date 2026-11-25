<!-- SPDX-License-Identifier: BSD-2-Clause -->

# neoj2k-rust merged-main rebuild and regression check

25 November 2026. Rebuilt and packaged merged main `b4240d6a33baafaf843cd027c83eee1f3e1fde3d` after PR #23. The before-change source is `3000f7d127222734e18de9cd69d05d475e01cb6e`. All timings below are Windows x86-64-v3 on the same local AMD Ryzen 9 9950X3D machine; they are not directly comparable to the earlier Linux/WSL2 tables.

## Findings

- The wide 2048x64 RGBA case retains its large improvement: 2171.256 to 1641.088 us, 24.42% less decode time in the fresh U8 paired run.
- The clearest observed loss is fresh 32x32 RGB U8: 119.067 to 124.113 us, +4.24% (about 5.05 us).
- Other square fresh U8 rows range from -0.10% to +0.79%.
- In the separate 23-request retained-decoder/fresh-encoder paired suite, the largest positive change of median times is +1.62% for 64x64 RGB 9/7 at reduce 2: 21.218 to 21.562 us. The six encode rows range from -1.38% to +1.04%.
- Large 9/7 results vary with execution order, host thread creation and memory/cache effects. Do not attribute their apparent gains to a reversible 5/3 selection change. Raw rounds and all process ratios are retained; three processes do not establish confidence bounds.

All 69 paired-suite output comparisons matched. All 36 fresh three-way case checks matched decoded samples or encoded bytes. Every U8 worker also passed its byte and guard checks around timed batches.

## Fresh U8: old versus rebuilt Rust

Three fresh-process trials, six actor orders each, same pinned logical CPU and fresh decoders. Original Rust worker SHA-256 is the same archived baseline binary used in the earlier benchmark. Times are medians of three trial medians, in microseconds. Positive changes are slower.

| Fixture | Before Rust | Rebuilt Rust | Time change |
| --- | ---: | ---: | ---: |
| rgba_2048x64_53 | 2171.256 | 1641.088 | -24.42% |
| rgb_32_53 | 119.067 | 124.113 | +4.24% |
| rgb_128_53 | 2502.688 | 2505.406 | +0.11% |
| rgb_512_53 | 32635.000 | 32603.450 | -0.10% |
| rgb_1024_53 | 117321.150 | 118242.900 | +0.79% |

## Broader paired suite: identical lifecycle on each side

The 23-request `tools/rust-versions` suite was copied to an external measurement workspace and pointed at the precise before/after sources. Decode retains one warmed decoder and writes caller-owned I32 planes; encode creates a new encoder for each call. The host scheduler creates scoped helpers for large rows at widths 2/4/8. Both sides share the same executable and alternate each round; the starting order rotates between three processes. No affinity restriction was imposed on this suite so host executor widths could use the allowed CPU set. All values below are microseconds, including the large rows. Three-process ratio ranges are observations, not confidence intervals.

| Request / executor width | Before | Rebuilt | Time change | Process ratio change range |
| --- | ---: | ---: | ---: | --- |
| large_1024_rgb_97.j2k 1 | 3752.300 | 3673.550 | -2.10% | -4.65% to -0.57% |
| large_1024_rgb_97.j2k 2 | 3124.200 | 3071.250 | -1.69% | -1.77% to -0.82% |
| large_1024_rgb_97.j2k 4 | 3603.100 | 3613.650 | +0.29% | -2.02% to +4.38% |
| large_1024_rgb_97.j2k 8 | 5850.700 | 5806.150 | -0.76% | -0.96% to -0.30% |
| large_4096_g_97.j2k 1 | 10383.350 | 10140.150 | -2.34% | -6.39% to +1.80% |
| large_4096_g_97.j2k 2 | 7041.450 | 6515.400 | -7.47% | -10.76% to -0.26% |
| large_4096_g_97.j2k 4 | 5593.850 | 5090.850 | -8.99% | -10.98% to +1.67% |
| large_4096_g_97.j2k 8 | 6182.450 | 5872.000 | -5.02% | -5.02% to +7.22% |
| sl_32_rgba_53.j2k / reduce 0 / whole | 195.262 | 197.518 | +1.16% | -0.42% to +1.24% |
| sl_32_rgba_53.j2k / reduce 2 / whole | 16.187 | 16.037 | -0.93% | -0.97% to -0.27% |
| sl_32_rgba_53.j2k / reduce 0 / missing EOC | 197.718 | 199.393 | +0.85% | -0.82% to +1.47% |
| sl_64_rgb_97.j2k / reduce 0 / whole | 50.293 | 50.831 | +1.07% | -1.45% to +1.76% |
| sl_64_rgb_97.j2k / reduce 2 / whole | 21.218 | 21.562 | +1.62% | +0.67% to +1.98% |
| sl_64_rgb_97.j2k / reduce 0 / missing EOC | 50.693 | 51.056 | +0.72% | -0.84% to +1.80% |
| sl_128_rgba_97.j2k / reduce 0 / whole | 111.181 | 111.412 | +0.21% | -1.23% to +4.18% |
| sl_128_rgba_97.j2k / reduce 2 / whole | 55.668 | 55.581 | -0.16% | -1.44% to +0.69% |
| sl_128_rgba_97.j2k / reduce 0 / missing EOC | 108.987 | 105.962 | -2.78% | -4.44% to +3.48% |
| encode 32x32 RGB / lossless 5/3 | 141.537 | 140.956 | -0.41% | -0.41% to +0.62% |
| encode 32x32 RGB / lossy 9/7 | 159.831 | 161.500 | +1.04% | -0.85% to +4.42% |
| encode 64x64 RGB / lossless 5/3 | 654.287 | 645.262 | -1.38% | -3.75% to +0.68% |
| encode 64x64 RGB / lossy 9/7 | 678.237 | 678.231 | -0.00% | -2.90% to +0.50% |
| encode 128x128 RGB / lossless 5/3 | 2390.606 | 2373.981 | -0.70% | -0.70% to +0.26% |
| encode 128x128 RGB / lossy 9/7 | 2597.637 | 2566.681 | -1.19% | -1.19% to -0.81% |

The before-change package was materialized from verified Git blobs under a unique test-package name. This avoids Cargo Git dependency discovery accidentally selecting a sibling checkout through absolute paths in archived audit manifests. Codec sources were not edited. The source inventory and harness hashes are in the manifest. Each paired case compares samples/bytes obtained before timing; timed calls retain status checks. These are not per-batch byte/guard checks like the U8 workers.

## Fresh C++ / Rust / OpenJPEG comparison

The tracked `tools/bench_three_way` executable was rebuilt for Windows with the local C++ source and its pinned OpenJPEG fork. All three actors use fresh serial codecs; decode writes caller-owned I32 output and encode returns owned bytes. File loading and caller output allocation are excluded. OpenJPEG creates internal image planes; Rust/C++ can decode directly to caller planes. Twelve rounds cover all six actor orders twice, with 32 preconditioning calls per actor batch. Three processes are pinned to the first allowed logical CPU. Each case checks samples/bytes before and after timing. Microseconds; median of process medians.

| Request | C++ | Rust | OpenJPEG fork | OpenJPEG/Rust |
| --- | ---: | ---: | ---: | ---: |
| decode/sl_32_rgba_53.j2k/reduce0 | 265.788 | 156.856 | 377.975 | 2.41x |
| decode/sl_64_rgb_97.j2k/reduce0 | 57.644 | 60.275 | 88.231 | 1.46x |
| decode/sl_128_rgba_97.j2k/reduce0 | 114.162 | 116.750 | 323.262 | 2.77x |
| decode/sl_64_rgb_97.j2k/reduce2 | 23.381 | 31.113 | 40.794 | 1.31x |
| decode/large_1024_rgb_97.j2k/reduce0 | 4273.100 | 3897.700 | 15939.250 | 4.09x |
| decode/large_4096_g_97.j2k/reduce0 | 10089.100 | 9164.750 | 82358.100 | 8.99x |
| encode/32x32/53 | 128.188 | 112.987 | 268.825 | 2.38x |
| encode/32x32/97 | 131.225 | 112.825 | 307.425 | 2.72x |
| encode/64x64/53 | 684.600 | 625.688 | 1034.975 | 1.65x |
| encode/64x64/97 | 747.938 | 679.200 | 1179.688 | 1.74x |
| encode/128x128/53 | 2645.250 | 2409.450 | 3522.363 | 1.46x |
| encode/128x128/97 | 2860.200 | 2620.838 | 4226.025 | 1.61x |

This supplies current Windows OpenJPEG measurements. It does not establish a before/after OpenJPEG change or replace the older Linux table. Rust/C++ differences in this table are API/lifecycle/build comparisons, not isolated language costs.

## Fresh Kakadu / C++ / Rust U8 comparison

The original matched Windows harness was rerun independently. Do not divide values from this table by values in another table. Microseconds; all five fixtures passed all actors' output/guard checks.

| Fixture | Kakadu | C++ | Rust |
| --- | ---: | ---: | ---: |
| rgb_32_53 | 325.672 | 125.236 | 123.675 |
| rgb_128_53 | 3339.875 | 2638.750 | 2481.181 |
| rgb_512_53 | 44416.450 | 32343.750 | 31958.100 |
| rgb_1024_53 | 165337.450 | 115433.500 | 114730.050 |
| rgba_2048x64_53 | 1950.385 | 1705.975 | 1600.783 |

Kakadu is the recovered Firestorm 8.6.1 tile-output path with set_fast, not a claim about every Kakadu SDK configuration.

## Package and provenance

- Packaged artifact: the local `neo-j2k-0.1.3.crate` at the measured main revision (not stored in this documentation directory).
- SHA-256: `b8ad70437adb3ee082f473519fff7f0b5d4ff07e7708804775f165ceb890e0f2`.
- `cargo package -p neo-j2k --locked` built and verified the package. Its 56 packaged source files were compared byte for byte with the benchmarked workspace sources.
- This is a local main rebuild at the existing 0.1.3 package version. No replacement release or crates.io publication was performed.
- C++ source: `81407c2ef7cfa146905678f09a6c11295e52b15c`; MSVC Release AVX2, strict FP, no IPO. Rust 1.100.0, release, x86-64-v3, fat LTO, one codegen unit, default features, no stage timers.
- [Raw measurements and source/build manifest](manifest.json), [23-request summary](paired-summary.json), [fresh three-way summary](fresh-summary.json), and the raw process logs are retained beside this report. The archived drivers contain the exact machine-specific paths used for this run.
