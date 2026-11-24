3 repetitions of each suite in this job (rows read in each: large decode 8, small decode 9, small encode 6); Rust and the C++ oracle are timed in one process, alternately.

Change is Rust's time against C++'s, minus is faster: the median of the repetitions' own changes (not the percent of the two median columns), with the lowest and the highest of them as the spread. Times are medians of the suites' own rounds (ms for the large decodes, us for the rest). Rust faster: Yes when the median change is below zero and the spread does not straddle zero, No when it is above zero and does not straddle it, Within noise when the spread straddles zero or the median is under 1.0 %.

C++ side: the neo-j2k oracle (v0.1.3, RelWithDebInfo) built with GCC 16, SIMD AVX2. GCC 16 is not the compiler the C++ performance numbers of the baseline were measured with (MSVC).

| Suite | Request | Workers | Unit | Rust | C++ | Change (Rust against C++) | Spread of the repetitions | Rust faster |
|---|---|---:|---|---:|---:|---:|---|:---:|
| large decode | large_1024_rgb_97.j2k | 1 | ms | 4.160 | 4.140 | +0.5 % | -0.8 to +0.8 % | Within noise |
| large decode | large_1024_rgb_97.j2k | 2 | ms | 3.222 | 3.339 | -3.1 % | -4.2 to -0.7 % | Yes |
| large decode | large_1024_rgb_97.j2k | 4 | ms | 2.595 | 2.515 | +2.8 % | +2.0 to +5.4 % | No |
| large decode | large_1024_rgb_97.j2k | 8 | ms | 2.505 | 2.547 | -2.0 % | -2.6 to -1.5 % | Yes |
| large decode | large_4096_g_97.j2k | 1 | ms | 12.055 | 12.705 | -4.9 % | -7.2 to -4.6 % | Yes |
| large decode | large_4096_g_97.j2k | 2 | ms | 7.676 | 8.158 | -5.5 % | -6.2 to -4.3 % | Yes |
| large decode | large_4096_g_97.j2k | 4 | ms | 5.027 | 5.395 | -7.1 % | -7.6 to -5.2 % | Yes |
| large decode | large_4096_g_97.j2k | 8 | ms | 3.953 | 4.201 | -8.2 % | -9.4 to -1.2 % | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 0 / whole | - | us | 203.91 | 216.17 | -5.7 % | -9.7 to -5.3 % | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 2 / whole | - | us | 16.78 | 16.26 | +3.4 % | +3.0 to +3.5 % | No |
| small decode | sl_32_rgba_53.j2k / reduce 0 / missing EOC | - | us | 188.77 | 215.17 | -12.3 % | -14.3 to -3.7 % | Yes |
| small decode | sl_64_rgb_97.j2k / reduce 0 / whole | - | us | 58.45 | 60.34 | -2.4 % | -4.5 to -2.2 % | Yes |
| small decode | sl_64_rgb_97.j2k / reduce 2 / whole | - | us | 22.50 | 20.79 | +8.7 % | +7.6 to +8.8 % | No |
| small decode | sl_64_rgb_97.j2k / reduce 0 / missing EOC | - | us | 58.02 | 60.07 | -3.6 % | -3.7 to -3.2 % | Yes |
| small decode | sl_128_rgba_97.j2k / reduce 0 / whole | - | us | 124.50 | 129.63 | -4.4 % | -5.9 to +3.8 % | Within noise |
| small decode | sl_128_rgba_97.j2k / reduce 2 / whole | - | us | 65.09 | 64.61 | +0.7 % | -2.7 to +8.0 % | Within noise |
| small decode | sl_128_rgba_97.j2k / reduce 0 / missing EOC | - | us | 131.59 | 127.21 | +3.4 % | -2.4 to +5.8 % | Within noise |
| small encode | encode 32x32 RGB / lossless 5/3 | - | us | 143.31 | 145.12 | -1.5 % | -1.6 to +0.3 % | Within noise |
| small encode | encode 32x32 RGB / lossy 9/7 | - | us | 152.21 | 156.61 | -4.3 % | -4.8 to -1.6 % | Yes |
| small encode | encode 64x64 RGB / lossless 5/3 | - | us | 634.98 | 651.32 | -2.4 % | -2.7 to -2.2 % | Yes |
| small encode | encode 64x64 RGB / lossy 9/7 | - | us | 685.23 | 696.77 | -2.1 % | -3.1 to -1.5 % | Yes |
| small encode | encode 128x128 RGB / lossless 5/3 | - | us | 2366.02 | 2461.25 | -3.9 % | -4.0 to -2.3 % | Yes |
| small encode | encode 128x128 RGB / lossy 9/7 | - | us | 2570.96 | 2626.85 | -2.2 % | -2.3 to -1.5 % | Yes |

Rust faster in 15 rows, within noise in 5, slower in 3, of 23

Rows where Rust is at least as fast as C++ (faster + within noise): 20 of 23
