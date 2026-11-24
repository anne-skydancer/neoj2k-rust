<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->

# Local Linux benchmarks: Rust against C++, and Rust 0.1.3 against 0.1.2 (2026-11-24)

The Rust codec at `main` 40ee9b4 (the decoder's Tier-1 rewrite, the encoder's distortion skip and four-row gather, the short-block path) timed on the owner's own machine under Linux, in two comparisons: against the C++ neo-j2k 0.1.3 oracle, and against the Rust crate at tag v0.1.2. This is a local, steady second opinion beside the CI tables of [bench.md](bench.md). It is not comparable with them: the CI runners have other CPUs (and change between runs), and CI builds the C++ with a March snapshot of GCC 16.

## Machine and method

| | |
|---|---|
| CPU | AMD Ryzen 9 9950X3D 16-Core Processor, 8 logical CPUs (Zen 5, 3D V-Cache); the WSL2 virtual machine was given 8 of its logical CPUs |
| System | Arch Linux in WSL2 (kernel 6.18), the Windows host idle at about 2 % CPU during the runs |
| Compiler | g++ (GCC) 16.2.1 20260810 for the C++ oracle (`CC=gcc`, `CXX=g++`); rustc 1.100.0, release profile with fat LTO and one codegen unit |
| CPU levels | baseline: x86-64 Rust build, the C++ oracle built scalar (`linux-gcc-scalar`); x86-64-v3: `-C target-cpu=x86-64-v3`, the oracle with AVX2 (`linux-gcc`). Both comparisons are like for like |
| Suites | the same ones as the CI jobs of `.github/workflows/perf.yml` (`small_image_gates` for C++, the `rust-versions` program for 0.1.2): 23 rows, Rust and the other side timed in one process, alternately, 3 repetitions, outputs compared and identical on every row |
| Counted rows | width 8 is counted: the machine has 8 logical CPUs (no pinning; the WSL2 virtual machine had the whole machine to itself) |
| Statistic | the median of the repetitions' own percent changes, as in the CI tables; minus is faster |

The C++ oracle is the commit 00f273d (neo-j2k v0.1.3). The crate at v0.1.2 is the commit d81e4ec. Raw output and the full tables, with absolute times and the spread of each row, are kept in `~/bench-out/` of the WSL (`20261124-154205-cpp-baseline`, `...-154240-cpp-v3`, `...-154307-versions-baseline`, `...-154341-versions-v3`). A pass of all four took under two minutes.

Marks in the tables: `=` within noise, `!` Rust (or 0.1.3) slower, no mark: faster than the noise.

## Findings

1. **Against C++ 0.1.3, Rust is at least level on 20 of 23 rows at both CPU levels.** Rust is faster on 17 rows at baseline and 15 at x86-64-v3, within noise on 3 and 5, slower on 3 and 3.
2. **The only rows still slower than C++ are the three small reduce-2 decodes** (a 32², 64² and 128² picture decoded at a quarter of its size), plus one large row:
   - baseline: 32² +2.9 %, 64² +6.3 %, 128² +5.9 %;
   - x86-64-v3: 32² +3.4 %, 64² +8.7 %, 128² +0.7 % (noise), and 1,024² RGB at 4 workers +2.8 %.
3. **The earlier large-decode gaps are gone.** The 1,024² RGB decode at 8 workers, which was +10.4 % at x86-64-v3 in the first CI run, is -2.0 % here; at baseline Rust is 22 % to 47 % faster on every large row.
4. **Every small encode is level or faster than C++** (-1.5 % to -4.6 %, two of them within noise).
5. **Rust 0.1.3 against 0.1.2:** faster on 20 of 23 rows at baseline and 21 at x86-64-v3 (table 2); encodes 44 % to 60 % faster, small whole and missing-EOC decodes 8 % to 42 % faster, large decodes 1 % to 19 % faster. 0.1.3 is slower on the two smallest pictures decoded at reduce 2: 32² +9.6 % (baseline) and +8.2 % (v3), 64² +3.9 % and +7.6 % (both within noise). The 128² reduce-2 decode is 30 % to 34 % faster. In the CI run before the short-block path, the 32² reduce-2 row was +14.8 % to +19.7 % in all four cells; that it is now +8 % to +10 % here fits the short-block path helping, but the machines differ and this is not proof.
6. **The remaining deficit is concentrated in one place:** small decodes at reduce 2, mostly code-blocks of 1×1 to 16×16 samples. The decode-bottleneck audit of the same day ties it to how the arithmetic decoder's byte-input helper hands its registers back (a scalar-return change measured -6 % to -8 % on the captured blocks, Windows); these local Linux numbers are consistent with that being the main thing left, and the change still has to be checked on Linux and Windows in CI.

## Table 1: Rust against neo-j2k C++ 0.1.3 (percent change, minus = Rust faster)

Rust faster / within noise / slower, of 23 rows: baseline 17 / 3 / 3, x86-64-v3 15 / 5 / 3.

| Row | Workers | Baseline | x86-64-v3 |
|---|---|---:|---:|
| large decode 1,024² RGB 9/7 | w1 | -36.8 | +0.5 = |
| large decode 1,024² RGB 9/7 | w2 | -30.5 | -3.1 |
| large decode 1,024² RGB 9/7 | w4 | -29.8 | +2.8 ! |
| large decode 1,024² RGB 9/7 | w8 | -22.4 | -2.0 |
| large decode 4,096² gray 9/7 | w1 | -46.7 | -4.9 |
| large decode 4,096² gray 9/7 | w2 | -43.0 | -5.5 |
| large decode 4,096² gray 9/7 | w4 | -40.7 | -7.1 |
| large decode 4,096² gray 9/7 | w8 | -36.6 | -8.2 |
| small decode 32² RGBA 5/3, whole | - | -5.2 | -5.7 |
| small decode 32² RGBA 5/3, reduce 2 | - | +2.9 ! | +3.4 ! |
| small decode 32² RGBA 5/3, missing EOC | - | -4.9 | -12.3 |
| small decode 64² RGB 9/7, whole | - | -0.6 = | -2.4 |
| small decode 64² RGB 9/7, reduce 2 | - | +6.3 ! | +8.7 ! |
| small decode 64² RGB 9/7, missing EOC | - | -0.6 = | -3.6 |
| small decode 128² RGBA 9/7, whole | - | -24.1 | -4.4 = |
| small decode 128² RGBA 9/7, reduce 2 | - | +5.9 ! | +0.7 = |
| small decode 128² RGBA 9/7, missing EOC | - | -25.6 | +3.4 = |
| small encode 32² RGB, lossless 5/3 | - | -2.9 | -1.5 = |
| small encode 32² RGB, lossy 9/7 | - | -4.2 = | -4.3 |
| small encode 64² RGB, lossless 5/3 | - | -4.6 | -2.4 |
| small encode 64² RGB, lossy 9/7 | - | -3.3 | -2.1 |
| small encode 128² RGB, lossless 5/3 | - | -4.2 | -3.9 |
| small encode 128² RGB, lossy 9/7 | - | -3.3 | -2.2 |

## Table 2: Rust 0.1.3 (main 40ee9b4) against Rust 0.1.2 (tag v0.1.2)

Both versions through the public API they share (`decode_into_i32`, `Encoder::encode`), one process, alternately, with the harness's own scheduler for the large decodes (scoped threads, started on every call, so the large-decode figures include that start-up on both sides). Output identical on every row. 0.1.3 faster / within noise / slower, of 23 rows: baseline 20 / 2 / 1, x86-64-v3 21 / 1 / 1.

| Row | Workers | Baseline | x86-64-v3 |
|---|---|---:|---:|
| large decode 1,024² RGB 9/7 | w1 | -11.1 | -19.0 |
| large decode 1,024² RGB 9/7 | w2 | -7.7 | -11.6 |
| large decode 1,024² RGB 9/7 | w4 | -5.4 | -5.5 |
| large decode 1,024² RGB 9/7 | w8 | -0.7 = | -1.3 |
| large decode 4,096² gray 9/7 | w1 | -3.9 | -7.8 |
| large decode 4,096² gray 9/7 | w2 | -2.5 | -9.4 |
| large decode 4,096² gray 9/7 | w4 | -3.2 | -7.6 |
| large decode 4,096² gray 9/7 | w8 | -3.1 | -3.3 |
| small decode 32² RGBA 5/3, whole | - | -38.8 | -38.9 |
| small decode 32² RGBA 5/3, reduce 2 | - | +9.6 ! | +8.2 ! |
| small decode 32² RGBA 5/3, missing EOC | - | -37.5 | -41.8 |
| small decode 64² RGB 9/7, whole | - | -8.3 | -10.5 |
| small decode 64² RGB 9/7, reduce 2 | - | +3.9 = | +7.6 = |
| small decode 64² RGB 9/7, missing EOC | - | -8.7 | -11.4 |
| small decode 128² RGBA 9/7, whole | - | -21.8 | -35.2 |
| small decode 128² RGBA 9/7, reduce 2 | - | -30.5 | -33.8 |
| small decode 128² RGBA 9/7, missing EOC | - | -22.8 | -32.1 |
| small encode 32² RGB, lossless 5/3 | - | -55.9 | -54.3 |
| small encode 32² RGB, lossy 9/7 | - | -60.0 | -57.9 |
| small encode 64² RGB, lossless 5/3 | - | -48.2 | -46.7 |
| small encode 64² RGB, lossy 9/7 | - | -53.7 | -50.8 |
| small encode 128² RGB, lossless 5/3 | - | -44.6 | -43.9 |
| small encode 128² RGB, lossy 9/7 | - | -52.5 | -49.8 |


## Second run: after the arithmetic decoder's scalar-return change (PR 21), 2026-11-24 17:16 to 17:19

The same four runs, the same script, the same machine and the same compiler, on `main` 449f4b9 (the first run was 40ee9b4; the codec differs by PR 21 only: the cold byte-input helper of the arithmetic decoder returns its registers as two scalars in one 64-bit value instead of a struct in memory). The machine's CPU load, sampled by the Windows host, was 2 % to 13 % before the runs and 2 % to 6 % after, so this run was a little less quiet than the first (about 2 %). Raw output is in `~/bench-out/20261124-171646-cpp-baseline`, `...-171723-cpp-v3`, `...-171754-versions-baseline`, `...-171837-versions-v3` of the WSL.

### What changed

1. **Against C++ 0.1.3, Rust is at least level on 22 of 23 rows at baseline and on all 23 at x86-64-v3.** Rust is faster on 19 rows at baseline and 17 at x86-64-v3, within noise on 3 and 6, slower on 1 and 0 (first run: 17 / 3 / 3 and 15 / 5 / 3).
2. **The three small reduce-2 decodes**, which were the only slower rows, moved as follows (percent change against C++, minus is faster, first run → second run):

| Row | Baseline | x86-64-v3 |
|---|---|---|
| small decode 32² RGBA 5/3, reduce 2 | +2.9 ! → -0.2 = | +3.4 ! → -2.9 |
| small decode 64² RGB 9/7, reduce 2 | +6.3 ! → +4.3 ! | +8.7 ! → -0.7 = |
| small decode 128² RGBA 9/7, reduce 2 | +5.9 ! → -0.5 = | +0.7 = → -4.2 = |

   The one that is still slower against C++ is the 64² reduce-2 decode at baseline (+4.3 %). The 1,024² RGB decode at 4 workers at x86-64-v3, which was +2.8 % in the first run, is +0.9 % (within noise) here, so that gap was not stable.
3. **Against Rust 0.1.2,** the reduce-2 decodes (0.1.3 against 0.1.2, first run → second run):

| Row | Baseline | x86-64-v3 |
|---|---|---|
| small decode 32² RGBA 5/3, reduce 2 | +9.6 ! → +6.3 ! | +8.2 ! → +4.4 ! |
| small decode 64² RGB 9/7, reduce 2 | +3.9 = → +1.4 ! | +7.6 = → +2.8 ! |
| small decode 128² RGBA 9/7, reduce 2 | -30.5 → -26.7 | -33.8 → -33.5 |

   The 32² reduce-2 decode is still slower than 0.1.2 (+6.3 % baseline, +4.4 % x86-64-v3, down from +9.6 % and +8.2 %), and the 64² one is now marked slower at both levels (+1.4 % and +2.8 %, within noise in the first run at both). Encodes, small full decodes and the 128² reduce-2 decode remain much faster than 0.1.2. 0.1.3 faster / within noise / slower, of 23 rows: baseline 21 / 0 / 2, x86-64-v3 19 / 2 / 2.
4. **What is left:** the 32² reduce-2 regression against 0.1.2, and the 64² reduce-2 decode at baseline against C++. The audit's next step is a stage profile of the current source (a CI run was started on this commit); the two earlier suspects (four-worker scheduling, the arithmetic decoder's interface) are no longer the first candidates.

### Table 3: Rust against neo-j2k C++ 0.1.3, second run (percent change, minus = Rust faster)

| Row | Workers | Baseline | x86-64-v3 |
|---|---|---:|---:|
| large decode 1,024² RGB 9/7 | w1 | -37.1 | -1.0 |
| large decode 1,024² RGB 9/7 | w2 | -30.2 | -5.8 |
| large decode 1,024² RGB 9/7 | w4 | -31.5 | +0.9 = |
| large decode 1,024² RGB 9/7 | w8 | -22.9 | -1.9 |
| large decode 4,096² gray 9/7 | w1 | -48.2 | -5.5 |
| large decode 4,096² gray 9/7 | w2 | -41.0 | -4.1 |
| large decode 4,096² gray 9/7 | w4 | -41.2 | -9.5 |
| large decode 4,096² gray 9/7 | w8 | -31.1 | -2.4 |
| small decode 32² RGBA 5/3, whole | - | -8.5 | -8.4 |
| small decode 32² RGBA 5/3, reduce 2 | - | -0.2 = | -2.9 |
| small decode 32² RGBA 5/3, missing EOC | - | -14.4 | -15.6 |
| small decode 64² RGB 9/7, whole | - | -4.8 | -8.8 |
| small decode 64² RGB 9/7, reduce 2 | - | +4.3 ! | -0.7 = |
| small decode 64² RGB 9/7, missing EOC | - | -5.8 | -9.7 |
| small decode 128² RGBA 9/7, whole | - | -24.3 | -1.0 = |
| small decode 128² RGBA 9/7, reduce 2 | - | -0.5 = | -4.2 = |
| small decode 128² RGBA 9/7, missing EOC | - | -26.1 | -2.3 |
| small encode 32² RGB, lossless 5/3 | - | -1.6 | -0.3 = |
| small encode 32² RGB, lossy 9/7 | - | -2.4 = | +0.6 = |
| small encode 64² RGB, lossless 5/3 | - | -4.8 | -2.5 |
| small encode 64² RGB, lossy 9/7 | - | -3.8 | -1.4 |
| small encode 128² RGB, lossless 5/3 | - | -4.2 | -3.5 |
| small encode 128² RGB, lossy 9/7 | - | -3.1 | -2.0 |

### Table 4: Rust 0.1.3 (main 449f4b9) against Rust 0.1.2, second run

| Row | Workers | Baseline | x86-64-v3 |
|---|---|---:|---:|
| large decode 1,024² RGB 9/7 | w1 | -12.6 | -18.5 |
| large decode 1,024² RGB 9/7 | w2 | -8.8 | -12.3 |
| large decode 1,024² RGB 9/7 | w4 | -5.4 | -4.6 |
| large decode 1,024² RGB 9/7 | w8 | -1.6 | -1.2 = |
| large decode 4,096² gray 9/7 | w1 | -3.1 | -6.6 |
| large decode 4,096² gray 9/7 | w2 | -2.9 | -8.1 |
| large decode 4,096² gray 9/7 | w4 | -4.1 | -6.6 |
| large decode 4,096² gray 9/7 | w8 | -4.9 | +17.4 = |
| small decode 32² RGBA 5/3, whole | - | -37.5 | -39.8 |
| small decode 32² RGBA 5/3, reduce 2 | - | +6.3 ! | +4.4 ! |
| small decode 32² RGBA 5/3, missing EOC | - | -37.5 | -40.5 |
| small decode 64² RGB 9/7, whole | - | -10.2 | -11.1 |
| small decode 64² RGB 9/7, reduce 2 | - | +1.4 ! | +2.8 ! |
| small decode 64² RGB 9/7, missing EOC | - | -12.7 | -8.8 |
| small decode 128² RGBA 9/7, whole | - | -24.0 | -33.7 |
| small decode 128² RGBA 9/7, reduce 2 | - | -26.7 | -33.5 |
| small decode 128² RGBA 9/7, missing EOC | - | -22.8 | -29.0 |
| small encode 32² RGB, lossless 5/3 | - | -55.7 | -55.8 |
| small encode 32² RGB, lossy 9/7 | - | -59.2 | -57.6 |
| small encode 64² RGB, lossless 5/3 | - | -47.3 | -48.7 |
| small encode 64² RGB, lossy 9/7 | - | -54.0 | -53.0 |
| small encode 128² RGB, lossless 5/3 | - | -44.8 | -45.6 |
| small encode 128² RGB, lossy 9/7 | - | -52.8 | -51.8 |

## Limits

- Three repetitions per row, one machine, one session. The spread of every row is in the raw tables; rows marked `=` are inside it.
- WSL2 is a virtual machine: CPU-bound times are close to native but not identical, and the hardware performance counters are not available, so nothing here says why a row is slower.
- The C++ is built with GCC 16.2.1 here and a GCC 16 snapshot in CI; the Rust is the same compiler in both.
- Nothing in this report was produced on the Windows host. The Windows numbers are in bench.md (the CI tables) and in the small-decode and decode-bottleneck audits.
- No OpenJPEG row was run locally; the OpenJPEG comparison is in bench.md (CI).
