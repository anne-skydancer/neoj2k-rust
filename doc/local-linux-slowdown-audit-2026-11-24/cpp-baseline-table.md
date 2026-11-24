3 repetitions of each suite in this job (rows read in each: large decode 8, small decode 9, small encode 6); Rust and the C++ oracle are timed in one process, alternately.

Change is Rust's time against C++'s, minus is faster: the median of the repetitions' own changes (not the percent of the two median columns), with the lowest and the highest of them as the spread. Times are medians of the suites' own rounds (ms for the large decodes, us for the rest). Rust faster: Yes when the median change is below zero and the spread does not straddle zero, No when it is above zero and does not straddle it, Within noise when the spread straddles zero or the median is under 1.0 %.

C++ side: the neo-j2k oracle (v0.1.3, RelWithDebInfo) built with GCC 16, SIMD scalar. GCC 16 is not the compiler the C++ performance numbers of the baseline were measured with (MSVC).

| Suite | Request | Workers | Unit | Rust | C++ | Change (Rust against C++) | Spread of the repetitions | Rust faster |
|---|---|---:|---|---:|---:|---:|---|:---:|
| large decode | large_1024_rgb_97.j2k | 1 | ms | 6.784 | 10.740 | -36.8 % | -37.3 to -35.3 % | Yes |
| large decode | large_1024_rgb_97.j2k | 2 | ms | 4.794 | 6.946 | -30.5 % | -31.1 to -29.8 % | Yes |
| large decode | large_1024_rgb_97.j2k | 4 | ms | 3.316 | 4.724 | -29.8 % | -31.9 to -29.0 % | Yes |
| large decode | large_1024_rgb_97.j2k | 8 | ms | 3.013 | 3.871 | -22.4 % | -24.1 to -22.0 % | Yes |
| large decode | large_4096_g_97.j2k | 1 | ms | 22.979 | 43.034 | -46.7 % | -48.2 to -46.6 % | Yes |
| large decode | large_4096_g_97.j2k | 2 | ms | 13.217 | 22.844 | -43.0 % | -43.5 to -41.9 % | Yes |
| large decode | large_4096_g_97.j2k | 4 | ms | 7.828 | 13.205 | -40.7 % | -43.1 to -40.2 % | Yes |
| large decode | large_4096_g_97.j2k | 8 | ms | 5.150 | 8.163 | -36.6 % | -36.9 to -35.7 % | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 0 / whole | - | us | 212.95 | 224.36 | -5.2 % | -9.0 to -4.2 % | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 2 / whole | - | us | 16.83 | 16.28 | +2.9 % | +0.5 to +3.4 % | No |
| small decode | sl_32_rgba_53.j2k / reduce 0 / missing EOC | - | us | 213.89 | 224.93 | -4.9 % | -14.2 to -3.9 % | Yes |
| small decode | sl_64_rgb_97.j2k / reduce 0 / whole | - | us | 73.70 | 74.21 | -0.6 % | -1.5 to -0.3 % | Within noise |
| small decode | sl_64_rgb_97.j2k / reduce 2 / whole | - | us | 23.40 | 22.50 | +6.3 % | +3.2 to +9.7 % | No |
| small decode | sl_64_rgb_97.j2k / reduce 0 / missing EOC | - | us | 73.38 | 73.85 | -0.6 % | -2.1 to +0.7 % | Within noise |
| small decode | sl_128_rgba_97.j2k / reduce 0 / whole | - | us | 187.94 | 247.11 | -24.1 % | -24.3 to -23.1 % | Yes |
| small decode | sl_128_rgba_97.j2k / reduce 2 / whole | - | us | 71.63 | 68.42 | +5.9 % | +1.8 to +6.1 % | No |
| small decode | sl_128_rgba_97.j2k / reduce 0 / missing EOC | - | us | 182.45 | 245.16 | -25.6 % | -26.2 to -23.9 % | Yes |
| small encode | encode 32x32 RGB / lossless 5/3 | - | us | 145.13 | 149.72 | -2.9 % | -5.7 to -0.9 % | Yes |
| small encode | encode 32x32 RGB / lossy 9/7 | - | us | 156.64 | 163.45 | -4.2 % | -4.2 to +0.1 % | Within noise |
| small encode | encode 64x64 RGB / lossless 5/3 | - | us | 636.97 | 667.77 | -4.6 % | -4.8 to -4.3 % | Yes |
| small encode | encode 64x64 RGB / lossy 9/7 | - | us | 686.75 | 710.35 | -3.3 % | -4.8 to -3.2 % | Yes |
| small encode | encode 128x128 RGB / lossless 5/3 | - | us | 2409.23 | 2503.27 | -4.2 % | -4.7 to -3.7 % | Yes |
| small encode | encode 128x128 RGB / lossy 9/7 | - | us | 2587.59 | 2678.17 | -3.3 % | -3.7 to -3.3 % | Yes |

Rust faster in 17 rows, within noise in 3, slower in 3, of 23

Rows where Rust is at least as fast as C++ (faster + within noise): 20 of 23
