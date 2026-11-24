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
| large decode | large_1024_rgb_97.j2k | 1 | ms | 6.847 | 7.717 | -11.1 % | -11.8 to -10.2 % | Yes | Yes |
| large decode | large_1024_rgb_97.j2k | 2 | ms | 5.437 | 5.897 | -7.7 % | -8.2 to -7.0 % | Yes | Yes |
| large decode | large_1024_rgb_97.j2k | 4 | ms | 5.402 | 5.707 | -5.4 % | -5.6 to -4.3 % | Yes | Yes |
| large decode | large_1024_rgb_97.j2k | 8 | ms | 8.007 | 8.072 | -0.7 % | -0.8 to -0.4 % | Within noise | Yes |
| large decode | large_4096_g_97.j2k | 1 | ms | 22.772 | 23.685 | -3.9 % | -4.6 to -1.7 % | Yes | Yes |
| large decode | large_4096_g_97.j2k | 2 | ms | 13.513 | 14.058 | -2.5 % | -5.1 to -1.3 % | Yes | Yes |
| large decode | large_4096_g_97.j2k | 4 | ms | 9.278 | 9.496 | -3.2 % | -3.6 to -2.1 % | Yes | Yes |
| large decode | large_4096_g_97.j2k | 8 | ms | 8.411 | 8.799 | -3.1 % | -5.0 to -3.1 % | Yes | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 0 / whole | - | us | 203.45 | 330.40 | -38.8 % | -39.0 to -35.0 % | Yes | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 2 / whole | - | us | 16.69 | 15.26 | +9.6 % | +7.9 to +13.9 % | No | Yes |
| small decode | sl_32_rgba_53.j2k / reduce 0 / missing EOC | - | us | 205.82 | 329.56 | -37.5 % | -38.4 to -36.1 % | Yes | Yes |
| small decode | sl_64_rgb_97.j2k / reduce 0 / whole | - | us | 74.17 | 81.77 | -8.3 % | -10.0 to -5.3 % | Yes | Yes |
| small decode | sl_64_rgb_97.j2k / reduce 2 / whole | - | us | 23.18 | 22.31 | +3.9 % | -0.3 to +4.6 % | Within noise | Yes |
| small decode | sl_64_rgb_97.j2k / reduce 0 / missing EOC | - | us | 72.72 | 80.24 | -8.7 % | -10.2 to -7.6 % | Yes | Yes |
| small decode | sl_128_rgba_97.j2k / reduce 0 / whole | - | us | 187.34 | 241.80 | -21.8 % | -27.1 to -21.8 % | Yes | Yes |
| small decode | sl_128_rgba_97.j2k / reduce 2 / whole | - | us | 72.18 | 106.12 | -30.5 % | -32.0 to -27.3 % | Yes | Yes |
| small decode | sl_128_rgba_97.j2k / reduce 0 / missing EOC | - | us | 185.68 | 240.62 | -22.8 % | -24.6 to -20.5 % | Yes | Yes |
| small encode | encode 32x32 RGB / lossless 5/3 | - | us | 145.26 | 329.30 | -55.9 % | -57.9 to -55.2 % | Yes | Yes |
| small encode | encode 32x32 RGB / lossy 9/7 | - | us | 157.97 | 396.51 | -60.0 % | -60.2 to -59.3 % | Yes | Yes |
| small encode | encode 64x64 RGB / lossless 5/3 | - | us | 638.28 | 1230.59 | -48.2 % | -48.5 to -47.6 % | Yes | Yes |
| small encode | encode 64x64 RGB / lossy 9/7 | - | us | 690.89 | 1490.57 | -53.7 % | -54.1 to -53.5 % | Yes | Yes |
| small encode | encode 128x128 RGB / lossless 5/3 | - | us | 2418.82 | 4368.49 | -44.6 % | -44.7 to -44.5 % | Yes | Yes |
| small encode | encode 128x128 RGB / lossy 9/7 | - | us | 2607.33 | 5470.03 | -52.5 % | -52.6 to -52.3 % | Yes | Yes |

0.1.3 faster in 20 rows, within noise in 2, slower in 1, of 23

Outputs identical in all 23 rows (decoded samples, encoded bytes).
