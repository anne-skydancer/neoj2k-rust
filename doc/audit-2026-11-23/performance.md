<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->
# Audit performance evidence
Source: https://github.com/anne-skydancer/neoj2k-rust/actions/runs/45242934789. Regenerated with HEAD perf_summary.py; three repetitions in each cell.

## ubuntu-26.04-baseline

Intel(R) Xeon(R) Platinum 8370C CPU @ 2.80GHz, 4 logical CPUs
3 repetitions of each suite in this job (rows read in each: large decode 8, small decode 9, small encode 6); Rust and the C++ oracle are timed in one process, alternately.

Change is Rust's time against C++'s, minus is faster: the median of the repetitions' own changes (not the percent of the two median columns), with the lowest and the highest of them as the spread. Times are medians of the suites' own rounds (ms for the large decodes, us for the rest). Rust faster: Yes when the median change is below zero and the spread does not straddle zero, No when it is above zero and does not straddle it, Within noise when the spread straddles zero or the median is under 1.0 %.

C++ side: the neo-j2k oracle (v0.1.3, RelWithDebInfo) built with GCC 15, SIMD scalar. GCC 15 is not the compiler the C++ performance numbers of the baseline were measured with (MSVC).

| Suite | Request | Workers | Unit | Rust | C++ | Change (Rust against C++) | Spread of the repetitions | Rust faster |
|---|---|---:|---|---:|---:|---:|---|:---:|
| large decode | large_1024_rgb_97.j2k | 1 | ms | 16.524 | 27.345 | -39.3 % | -39.6 to -38.8 % | Yes |
| large decode | large_1024_rgb_97.j2k | 2 | ms | 10.204 | 15.657 | -34.8 % | -34.8 to -33.4 % | Yes |
| large decode | large_1024_rgb_97.j2k | 4 | ms | 9.097 | 15.203 | -40.1 % | -40.2 to -39.6 % | Yes |
| large decode | large_1024_rgb_97.j2k | 8 | ms | 9.773 | 15.540 | -36.7 % | -37.1 to -36.1 % | Yes (oversubscribed) |
| large decode | large_4096_g_97.j2k | 1 | ms | 60.471 | 112.927 | -46.5 % | -46.9 to -46.5 % | Yes |
| large decode | large_4096_g_97.j2k | 2 | ms | 34.103 | 60.906 | -44.1 % | -44.4 to -43.2 % | Yes |
| large decode | large_4096_g_97.j2k | 4 | ms | 33.087 | 59.166 | -44.1 % | -44.5 to -44.0 % | Yes |
| large decode | large_4096_g_97.j2k | 8 | ms | 35.050 | 62.082 | -43.5 % | -44.0 to -42.8 % | Yes (oversubscribed) |
| small decode | sl_32_rgba_53.j2k / reduce 0 / whole | - | us | 486.12 | 436.85 | +11.3 % | +11.0 to +11.4 % | No |
| small decode | sl_32_rgba_53.j2k / reduce 2 / whole | - | us | 39.24 | 37.50 | +4.6 % | +4.6 to +4.6 % | No |
| small decode | sl_32_rgba_53.j2k / reduce 0 / missing EOC | - | us | 485.21 | 436.59 | +11.1 % | +11.0 to +11.2 % | No |
| small decode | sl_64_rgb_97.j2k / reduce 0 / whole | - | us | 196.92 | 192.70 | +2.2 % | -0.7 to +3.7 % | Within noise |
| small decode | sl_64_rgb_97.j2k / reduce 2 / whole | - | us | 58.26 | 53.67 | +8.4 % | +7.9 to +9.7 % | No |
| small decode | sl_64_rgb_97.j2k / reduce 0 / missing EOC | - | us | 194.09 | 192.13 | +1.2 % | +0.3 to +2.7 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 0 / whole | - | us | 448.88 | 580.70 | -22.7 % | -22.7 to -22.2 % | Yes |
| small decode | sl_128_rgba_97.j2k / reduce 2 / whole | - | us | 214.03 | 188.01 | +13.8 % | +12.6 to +14.2 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 0 / missing EOC | - | us | 448.07 | 579.38 | -22.7 % | -23.4 to -22.0 % | Yes |
| small encode | encode 32x32 RGB / lossless 5/3 | - | us | 353.89 | 315.51 | +12.2 % | +12.0 to +12.9 % | No |
| small encode | encode 32x32 RGB / lossy 9/7 | - | us | 404.18 | 347.05 | +16.5 % | +16.2 to +17.0 % | No |
| small encode | encode 64x64 RGB / lossless 5/3 | - | us | 1327.31 | 1210.02 | +9.7 % | +9.7 to +9.9 % | No |
| small encode | encode 64x64 RGB / lossy 9/7 | - | us | 1490.83 | 1316.83 | +13.2 % | +13.2 to +13.2 % | No |
| small encode | encode 128x128 RGB / lossless 5/3 | - | us | 4776.39 | 4406.10 | +8.2 % | +8.2 to +8.5 % | No |
| small encode | encode 128x128 RGB / lossy 9/7 | - | us | 5344.63 | 4855.57 | +10.0 % | +9.9 to +10.1 % | No |

Not counted (more workers than the runner's 4 logical CPUs; the width includes the caller): large_1024_rgb_97.j2k, width 8; large_4096_g_97.j2k, width 8

Rust faster in 8 rows, within noise in 1, slower in 12, of 21

Rows where Rust is at least as fast as C++ (faster + within noise): 9 of 21

## ubuntu-26.04-x86-64-v3

AMD EPYC 9V45 96-Core Processor, 4 logical CPUs
3 repetitions of each suite in this job (rows read in each: large decode 8, small decode 9, small encode 6); Rust and the C++ oracle are timed in one process, alternately.

Change is Rust's time against C++'s, minus is faster: the median of the repetitions' own changes (not the percent of the two median columns), with the lowest and the highest of them as the spread. Times are medians of the suites' own rounds (ms for the large decodes, us for the rest). Rust faster: Yes when the median change is below zero and the spread does not straddle zero, No when it is above zero and does not straddle it, Within noise when the spread straddles zero or the median is under 1.0 %.

C++ side: the neo-j2k oracle (v0.1.3, RelWithDebInfo) built with GCC 15, SIMD AVX2. GCC 15 is not the compiler the C++ performance numbers of the baseline were measured with (MSVC).

| Suite | Request | Workers | Unit | Rust | C++ | Change (Rust against C++) | Spread of the repetitions | Rust faster |
|---|---|---:|---|---:|---:|---:|---|:---:|
| large decode | large_1024_rgb_97.j2k | 1 | ms | 5.927 | 5.813 | +2.0 % | +0.7 to +2.2 % | No |
| large decode | large_1024_rgb_97.j2k | 2 | ms | 4.070 | 3.846 | +5.2 % | +4.9 to +6.1 % | No |
| large decode | large_1024_rgb_97.j2k | 4 | ms | 3.161 | 3.212 | -1.8 % | -2.2 to -1.6 % | Yes |
| large decode | large_1024_rgb_97.j2k | 8 | ms | 3.396 | 3.075 | +10.4 % | +9.2 to +10.5 % | No (oversubscribed) |
| large decode | large_4096_g_97.j2k | 1 | ms | 21.944 | 23.060 | -5.0 % | -5.2 to -3.6 % | Yes |
| large decode | large_4096_g_97.j2k | 2 | ms | 16.344 | 17.118 | -4.0 % | -6.9 to -3.8 % | Yes |
| large decode | large_4096_g_97.j2k | 4 | ms | 12.671 | 12.909 | -1.8 % | -2.0 to -1.0 % | Yes |
| large decode | large_4096_g_97.j2k | 8 | ms | 13.062 | 13.325 | -1.9 % | -2.0 to -0.2 % | Yes (oversubscribed) |
| small decode | sl_32_rgba_53.j2k / reduce 0 / whole | - | us | 277.99 | 268.53 | +3.3 % | +0.4 to +4.5 % | No |
| small decode | sl_32_rgba_53.j2k / reduce 2 / whole | - | us | 22.76 | 21.87 | +4.1 % | +3.9 to +5.2 % | No |
| small decode | sl_32_rgba_53.j2k / reduce 0 / missing EOC | - | us | 269.89 | 270.24 | +1.8 % | -0.1 to +5.3 % | Within noise |
| small decode | sl_64_rgb_97.j2k / reduce 0 / whole | - | us | 77.18 | 74.40 | +3.5 % | +3.1 to +3.8 % | No |
| small decode | sl_64_rgb_97.j2k / reduce 2 / whole | - | us | 31.00 | 27.44 | +13.3 % | +13.0 to +13.5 % | No |
| small decode | sl_64_rgb_97.j2k / reduce 0 / missing EOC | - | us | 76.34 | 73.95 | +3.3 % | +2.9 to +3.4 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 0 / whole | - | us | 165.33 | 158.59 | +4.2 % | +0.4 to +8.7 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 2 / whole | - | us | 85.29 | 75.02 | +13.9 % | +13.7 to +14.1 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 0 / missing EOC | - | us | 162.56 | 154.41 | +3.5 % | +0.8 to +5.3 % | No |
| small encode | encode 32x32 RGB / lossless 5/3 | - | us | 187.36 | 176.79 | +4.6 % | +4.5 to +6.4 % | No |
| small encode | encode 32x32 RGB / lossy 9/7 | - | us | 208.80 | 193.02 | +8.0 % | +6.7 to +11.4 % | No |
| small encode | encode 64x64 RGB / lossless 5/3 | - | us | 843.44 | 812.66 | +3.0 % | +2.7 to +3.8 % | No |
| small encode | encode 64x64 RGB / lossy 9/7 | - | us | 910.31 | 871.02 | +4.6 % | +4.4 to +5.2 % | No |
| small encode | encode 128x128 RGB / lossless 5/3 | - | us | 3194.09 | 3056.39 | +4.5 % | +3.8 to +4.8 % | No |
| small encode | encode 128x128 RGB / lossy 9/7 | - | us | 3426.71 | 3265.45 | +5.1 % | +4.6 to +5.2 % | No |

Not counted (more workers than the runner's 4 logical CPUs; the width includes the caller): large_1024_rgb_97.j2k, width 8; large_4096_g_97.j2k, width 8

Rust faster in 4 rows, within noise in 1, slower in 16, of 21

Rows where Rust is at least as fast as C++ (faster + within noise): 5 of 21

## windows-2025-baseline

AMD EPYC 9V74 80-Core Processor, 4 logical CPUs
3 repetitions of each suite in this job (rows read in each: large decode 8, small decode 9, small encode 6); Rust and the C++ oracle are timed in one process, alternately.

Change is Rust's time against C++'s, minus is faster: the median of the repetitions' own changes (not the percent of the two median columns), with the lowest and the highest of them as the spread. Times are medians of the suites' own rounds (ms for the large decodes, us for the rest). Rust faster: Yes when the median change is below zero and the spread does not straddle zero, No when it is above zero and does not straddle it, Within noise when the spread straddles zero or the median is under 1.0 %.

C++ side: the neo-j2k oracle (v0.1.3, RelWithDebInfo) built with MSVC, SIMD AVX2. MSVC is the compiler the C++ performance numbers of the baseline were measured with.

This cell is not like for like: the Rust build is for the baseline x86-64 and the C++ oracle is AVX2 (Windows has no scalar oracle preset); the table is shown, its counts are not.

| Suite | Request | Workers | Unit | Rust | C++ | Change (Rust against C++) | Spread of the repetitions | Rust faster |
|---|---|---:|---|---:|---:|---:|---|:---:|
| large decode | large_1024_rgb_97.j2k | 1 | ms | 11.489 | 7.245 | +58.9 % | +58.6 to +64.1 % | No |
| large decode | large_1024_rgb_97.j2k | 2 | ms | 7.649 | 5.254 | +44.0 % | +42.9 to +45.6 % | No |
| large decode | large_1024_rgb_97.j2k | 4 | ms | 6.680 | 4.524 | +46.6 % | +44.9 to +48.9 % | No |
| large decode | large_1024_rgb_97.j2k | 8 | ms | 7.150 | 4.730 | +51.8 % | +47.6 to +53.0 % | No (oversubscribed) |
| large decode | large_4096_g_97.j2k | 1 | ms | 49.222 | 29.010 | +69.7 % | +64.2 to +79.3 % | No |
| large decode | large_4096_g_97.j2k | 2 | ms | 27.909 | 18.988 | +47.0 % | +40.9 to +67.2 % | No |
| large decode | large_4096_g_97.j2k | 4 | ms | 25.104 | 16.310 | +53.9 % | +50.8 to +70.6 % | No |
| large decode | large_4096_g_97.j2k | 8 | ms | 26.339 | 17.047 | +54.5 % | +51.1 to +62.8 % | No (oversubscribed) |
| small decode | sl_32_rgba_53.j2k / reduce 0 / whole | - | us | 307.81 | 354.96 | -14.3 % | -17.9 to -13.3 % | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 2 / whole | - | us | 32.89 | 28.99 | +11.3 % | +10.6 to +13.5 % | No |
| small decode | sl_32_rgba_53.j2k / reduce 0 / missing EOC | - | us | 280.98 | 353.65 | -20.1 % | -20.5 to -18.8 % | Yes |
| small decode | sl_64_rgb_97.j2k / reduce 0 / whole | - | us | 128.52 | 95.18 | +34.2 % | +34.2 to +35.2 % | No |
| small decode | sl_64_rgb_97.j2k / reduce 2 / whole | - | us | 45.39 | 36.89 | +24.2 % | +22.7 to +24.9 % | No |
| small decode | sl_64_rgb_97.j2k / reduce 0 / missing EOC | - | us | 128.16 | 95.94 | +34.6 % | +33.6 to +36.2 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 0 / whole | - | us | 309.04 | 201.47 | +53.4 % | +51.6 to +56.3 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 2 / whole | - | us | 126.50 | 98.78 | +28.8 % | +28.1 to +30.0 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 0 / missing EOC | - | us | 311.01 | 200.19 | +55.1 % | +54.5 to +55.4 % | No |
| small encode | encode 32x32 RGB / lossless 5/3 | - | us | 235.70 | 243.59 | -3.6 % | -9.3 to +1.4 % | Within noise |
| small encode | encode 32x32 RGB / lossy 9/7 | - | us | 246.17 | 268.06 | -5.9 % | -10.5 to -0.1 % | Yes |
| small encode | encode 64x64 RGB / lossless 5/3 | - | us | 1081.88 | 1154.81 | -4.5 % | -6.3 to -4.2 % | Yes |
| small encode | encode 64x64 RGB / lossy 9/7 | - | us | 1176.08 | 1186.50 | -0.6 % | -2.1 to +1.0 % | Within noise |
| small encode | encode 128x128 RGB / lossless 5/3 | - | us | 4078.43 | 4162.97 | -2.6 % | -2.8 to -2.0 % | Yes |
| small encode | encode 128x128 RGB / lossy 9/7 | - | us | 4350.59 | 4325.91 | +0.0 % | +0.0 to +0.6 % | Within noise |

Not counted (more workers than the runner's 4 logical CPUs; the width includes the caller): large_1024_rgb_97.j2k, width 8; large_4096_g_97.j2k, width 8

Rows where Rust is at least as fast as C++: not like for like (C++ oracle AVX2)

## windows-2025-x86-64-v3

AMD EPYC 7763 64-Core Processor, 4 logical CPUs
3 repetitions of each suite in this job (rows read in each: large decode 8, small decode 9, small encode 6); Rust and the C++ oracle are timed in one process, alternately.

Change is Rust's time against C++'s, minus is faster: the median of the repetitions' own changes (not the percent of the two median columns), with the lowest and the highest of them as the spread. Times are medians of the suites' own rounds (ms for the large decodes, us for the rest). Rust faster: Yes when the median change is below zero and the spread does not straddle zero, No when it is above zero and does not straddle it, Within noise when the spread straddles zero or the median is under 1.0 %.

C++ side: the neo-j2k oracle (v0.1.3, RelWithDebInfo) built with MSVC, SIMD AVX2. MSVC is the compiler the C++ performance numbers of the baseline were measured with.

| Suite | Request | Workers | Unit | Rust | C++ | Change (Rust against C++) | Spread of the repetitions | Rust faster |
|---|---|---:|---|---:|---:|---:|---|:---:|
| large decode | large_1024_rgb_97.j2k | 1 | ms | 9.211 | 9.050 | +0.5 % | -1.2 to +2.1 % | Within noise |
| large decode | large_1024_rgb_97.j2k | 2 | ms | 5.984 | 6.024 | +2.4 % | -0.7 to +3.3 % | Within noise |
| large decode | large_1024_rgb_97.j2k | 4 | ms | 5.246 | 5.155 | +0.2 % | -1.6 to +1.8 % | Within noise |
| large decode | large_1024_rgb_97.j2k | 8 | ms | 5.460 | 5.180 | +5.4 % | +4.5 to +6.1 % | No (oversubscribed) |
| large decode | large_4096_g_97.j2k | 1 | ms | 42.992 | 43.919 | -2.1 % | -2.9 to -0.8 % | Yes |
| large decode | large_4096_g_97.j2k | 2 | ms | 26.039 | 27.949 | -2.6 % | -7.0 to -2.6 % | Yes |
| large decode | large_4096_g_97.j2k | 4 | ms | 21.162 | 21.057 | +0.5 % | -0.1 to +1.0 % | Within noise |
| large decode | large_4096_g_97.j2k | 8 | ms | 21.491 | 21.846 | -0.4 % | -1.6 to +0.0 % | Within noise (oversubscribed) |
| small decode | sl_32_rgba_53.j2k / reduce 0 / whole | - | us | 425.47 | 464.61 | -8.5 % | -8.6 to -8.1 % | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 2 / whole | - | us | 38.86 | 39.22 | -1.8 % | -3.2 to +8.9 % | Within noise |
| small decode | sl_32_rgba_53.j2k / reduce 0 / missing EOC | - | us | 425.47 | 464.86 | -8.4 % | -9.4 to -8.4 % | Yes |
| small decode | sl_64_rgb_97.j2k / reduce 0 / whole | - | us | 139.99 | 127.23 | +10.1 % | +6.5 to +11.1 % | No |
| small decode | sl_64_rgb_97.j2k / reduce 2 / whole | - | us | 54.62 | 48.09 | +11.9 % | +10.4 to +17.2 % | No |
| small decode | sl_64_rgb_97.j2k / reduce 0 / missing EOC | - | us | 135.05 | 126.98 | +6.4 % | +4.9 to +12.5 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 0 / whole | - | us | 315.36 | 285.71 | +8.3 % | +6.0 to +10.7 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 2 / whole | - | us | 164.87 | 142.19 | +16.2 % | +16.0 to +16.4 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 0 / missing EOC | - | us | 310.18 | 284.76 | +8.9 % | +1.2 to +11.0 % | No |
| small encode | encode 32x32 RGB / lossless 5/3 | - | us | 337.04 | 385.93 | -12.7 % | -14.4 to -7.4 % | Yes |
| small encode | encode 32x32 RGB / lossy 9/7 | - | us | 352.49 | 387.51 | -9.5 % | -11.9 to -8.1 % | Yes |
| small encode | encode 64x64 RGB / lossless 5/3 | - | us | 1294.01 | 1428.24 | -9.6 % | -9.7 to -9.2 % | Yes |
| small encode | encode 64x64 RGB / lossy 9/7 | - | us | 1351.54 | 1439.26 | -6.0 % | -6.3 to -5.0 % | Yes |
| small encode | encode 128x128 RGB / lossless 5/3 | - | us | 4691.47 | 5187.82 | -9.6 % | -10.0 to -9.6 % | Yes |
| small encode | encode 128x128 RGB / lossy 9/7 | - | us | 4950.26 | 5408.72 | -8.4 % | -8.7 to -8.0 % | Yes |

Not counted (more workers than the runner's 4 logical CPUs; the width includes the caller): large_1024_rgb_97.j2k, width 8; large_4096_g_97.j2k, width 8

Rust faster in 10 rows, within noise in 5, slower in 6, of 21

Rows where Rust is at least as fast as C++ (faster + within noise): 15 of 21
