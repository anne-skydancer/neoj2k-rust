3 repetitions of each suite in this job (rows read in each: large decode 8, small decode 9, small encode 6); the two sides are timed in one process, alternately. This is a measurement, with no pass or fail gate.

Change (0.1.3 against 0.1.2) is the first side's time against the second's, minus is faster: the median of the repetitions' own changes (not the percent of the two median columns), with the lowest and the highest of them as the spread. Times are medians of the suites' own rounds (ms for the large decodes, us for the rest). 0.1.3 faster: Yes when the median change is below zero and the spread does not straddle zero, No when it is above zero and does not straddle it, Within noise when the spread straddles zero or the median is under 1.0 %. Output identical: Yes when the two sides' outputs (the decoded samples, the encoded bytes) matched in every repetition.

The API each side used:

| Suite | 0.1.3 uses | 0.1.2 uses |
|---|---|---|
| large decode | both versions: one Decoder kept across calls, decode_into_i32 into caller-owned planes; threads are the harness's own scheduler (scoped threads, width - 1 started per call, the width counts the caller), the same for both | the same code, built against the crate at tag v0.1.2 |
| small decode | both versions: one Decoder kept across calls, decode_into_i32 into caller-owned planes | the same code, built against the crate at tag v0.1.2 |
| small encode | both versions: Encoder::new().encode on the borrowed interleaved pixels, one encoder per call | the same code, built against the crate at tag v0.1.2 |

| Suite | Request | Workers | Unit | 0.1.3 | 0.1.2 | Change (0.1.3 against 0.1.2) | Spread of the repetitions | 0.1.3 faster | Output identical |
|---|---|---:|---|---:|---:|---:|---|:---:|:---:|
| large decode | large_1024_rgb_97.j2k | 1 | ms | 4.143 | 5.161 | -19.0 % | -20.0 to -18.6 % | Yes | Yes |
| large decode | large_1024_rgb_97.j2k | 2 | ms | 4.005 | 4.539 | -11.6 % | -13.1 to -11.5 % | Yes | Yes |
| large decode | large_1024_rgb_97.j2k | 4 | ms | 4.921 | 5.138 | -5.5 % | -6.4 to -4.2 % | Yes | Yes |
| large decode | large_1024_rgb_97.j2k | 8 | ms | 7.537 | 7.653 | -1.3 % | -2.3 to -0.3 % | Yes | Yes |
| large decode | large_4096_g_97.j2k | 1 | ms | 11.908 | 12.810 | -7.8 % | -10.9 to -6.5 % | Yes | Yes |
| large decode | large_4096_g_97.j2k | 2 | ms | 7.768 | 8.577 | -9.4 % | -9.9 to -4.4 % | Yes | Yes |
| large decode | large_4096_g_97.j2k | 4 | ms | 6.029 | 6.615 | -7.6 % | -12.4 to -2.2 % | Yes | Yes |
| large decode | large_4096_g_97.j2k | 8 | ms | 7.056 | 7.361 | -3.3 % | -5.5 to -2.4 % | Yes | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 0 / whole | - | us | 199.58 | 326.74 | -38.9 % | -41.5 to -38.0 % | Yes | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 2 / whole | - | us | 16.41 | 15.19 | +8.2 % | +7.3 to +8.3 % | No | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 0 / missing EOC | - | us | 191.24 | 328.91 | -41.8 % | -41.9 to -41.4 % | Yes | Yes |
| small decode | sl_64_rgb_97.j2k / reduce 0 / whole | - | us | 58.60 | 65.02 | -10.5 % | -11.9 to -8.6 % | Yes | Yes |
| small decode | sl_64_rgb_97.j2k / reduce 2 / whole | - | us | 22.71 | 21.11 | +7.6 % | -0.8 to +8.2 % | Within noise | Yes |
| small decode | sl_64_rgb_97.j2k / reduce 0 / missing EOC | - | us | 58.73 | 66.42 | -11.4 % | -12.5 to -10.6 % | Yes | Yes |
| small decode | sl_128_rgba_97.j2k / reduce 0 / whole | - | us | 118.33 | 182.50 | -35.2 % | -35.3 to -29.7 % | Yes | Yes |
| small decode | sl_128_rgba_97.j2k / reduce 2 / whole | - | us | 66.62 | 100.70 | -33.8 % | -34.7 to -26.3 % | Yes | Yes |
| small decode | sl_128_rgba_97.j2k / reduce 0 / missing EOC | - | us | 123.92 | 181.84 | -32.1 % | -32.6 to -29.1 % | Yes | Yes |
| small encode | encode 32x32 RGB / lossless 5/3 | - | us | 143.34 | 312.95 | -54.3 % | -54.7 to -53.7 % | Yes | Yes |
| small encode | encode 32x32 RGB / lossy 9/7 | - | us | 153.55 | 364.43 | -57.9 % | -58.6 to -56.4 % | Yes | Yes |
| small encode | encode 64x64 RGB / lossless 5/3 | - | us | 627.93 | 1174.24 | -46.7 % | -46.7 to -46.3 % | Yes | Yes |
| small encode | encode 64x64 RGB / lossy 9/7 | - | us | 676.62 | 1376.52 | -50.8 % | -51.5 to -50.5 % | Yes | Yes |
| small encode | encode 128x128 RGB / lossless 5/3 | - | us | 2364.61 | 4217.34 | -43.9 % | -44.3 to -43.6 % | Yes | Yes |
| small encode | encode 128x128 RGB / lossy 9/7 | - | us | 2553.05 | 5093.53 | -49.8 % | -50.3 to -49.8 % | Yes | Yes |

0.1.3 faster in 21 rows, within noise in 1, slower in 1, of 23

Outputs identical in all 23 rows (decoded samples, encoded bytes).
