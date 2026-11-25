# Wide RGBA decode parity, 25 November 2026

The 2048x64 lossless RGBA case reaches parity with the C++ neo-j2k in repeated matched Windows measurements. The change selects the existing row-wise vertical 5/3 reconstruction for wide, short planes rather than the strided column path.

## Paired baseline comparison

Three fresh-process trials, every actor order, same logical CPU, fresh decoders and caller-owned interleaved U8 output. Every output byte and both guards are checked before and after each timed batch. Actors are the original Rust worker, the changed Rust worker and the unchanged C++ worker. All pixels and guards passed.

| Actor | Median us per decode |
| --- | ---: |
| Original Rust | 2163.794 |
| Changed Rust | 1609.235 |
| C++ neo-j2k | 1706.767 |

Changed Rust uses 25.63% less decode time than original Rust and 5.71% less than C++ in this run. Each of the three target trial medians beats its corresponding C++ trial median.

The original worker SHA-256 matches the worker recorded in the pre-change `target-kdu-bench/comparison-fast/manifest.json`: `b87a22f86d99388f194a33b1f99a7bd37ecf9df62d49902bc6751c82d6e858e4`. Baseline source commit: `3000f7d127222734e18de9cd69d05d475e01cb6e`.

## Independent final three-way run

The original `tools/kdu_bench/compare.py` was run with the final Rust binary, C++ and embedded Kakadu set_fast. All five fixtures passed all three actors' byte and guard checks. Values are microseconds, median of three trial medians.

| Fixture | Kakadu | C++ | Rust |
| --- | ---: | ---: | ---: |
| rgb_32_53 | 352.481 | 134.203 | 129.758 |
| rgb_128_53 | 3516.658 | 2828.450 | 2664.094 |
| rgb_512_53 | 46028.700 | 33227.100 | 33572.600 |
| rgb_1024_53 | 173459.650 | 120121.900 | 118072.650 |
| rgba_2048x64_53 | 2070.711 | 1826.123 | 1776.632 |

Absolute times vary between process runs; compare actors within the same run. These measurements cover the recovered viewer path, Windows, single-threaded lossless 5/3 U8 output, not general Kakadu SDK performance.

## Cause and change

The previous reversible row-wise gate required an area of at least 512x512 samples. A 2048x64 component is only half that area, despite its 8192-byte coefficient row stride. Its column batches repeatedly cross that stride. Use row-wise lifts also when width is at least 256 and height is at most width/4, retaining the existing width and nontrivial-height checks. This is a geometry rule, independent of fixture name and component count. The arithmetic and output API are unchanged.

One stage-timer diagnostic measured wavelet time falling from approximately 939 us to 417 us, with total time falling from 2108 us to 1613 us. Stage instrumentation and these separate diagnostic calls are excluded from the matched timing claims above. Tier-1 and allocation/plane preparation are the next largest costs.

## Small-image tradeoff

The exploratory paired sweep found the 32x32 RGB case about 6.4% slower (119.24 to 126.86 us); 128/512/1024 square medians differed by less than 1%. A repeat of the final source measured 32x32 at 118.67 to 124.30 us (+4.7%). The final binary's paired run measured it at 125.40 to 126.26 us (+0.68%). Small-image preservation is therefore not claimed. The small square cases do not take the new row-wise path. Process/binary layout and machine variation remain possible contributors; they have not been isolated.

## Validation

- `cargo test --release --workspace --locked` with `RUSTFLAGS=-C target-cpu=x86-64-v3` and the local C++ oracle: passed, including generated parity, damaged/truncated inputs, executor, memory limits and reference replay. Explicitly ignored nightly/performance tests remained ignored; optional external conformance tests follow their dataset-availability checks.
- `cargo test --release -p neo-j2k --lib dwt:: --locked` at baseline x86-64: 9 passed.
- The corresponding x86-64-v3 wavelet run: 12 passed.
- New regression test compares the chosen path to the independent column reference for boundary widths/aspects, odd dimensions, all origin parities, wrapping i32 data and untouched padding.
- `cargo clippy -p neo-j2k --all-targets --locked -- -D warnings`: passed at x86-64-v3.
- Formatting, `git diff --check`, and `python -I tools/check_rust_native.py`: passed.

## Evidence and reproduction

Raw per-trial/per-round measurements, hashes and build/source metadata are in `benchmarks/wide-rgba-parity-2026-11-25/`. `matched-*` is the final three-way run; `paired-*` is the final binary against original Rust and C++; `exploratory-*` records the first five-case paired run (an equivalent multiplication form of the aspect predicate). `paired-driver.py` is the exact machine-specific diagnostic driver; its original baseline executable was retained outside the repository. No proprietary binaries are included.

The final matched manifest and paired manifest record the same changed worker SHA-256. These measurements use the pre-existing local `tools/kdu_bench` harness, which was untracked before this optimization and is not included in this change. The commands below require that external harness, the matching viewer binary and dependencies, and the C++ source checkout:

```powershell
$env:RUSTFLAGS = '-C target-cpu=x86-64-v3'
cargo build --release --manifest-path tools/kdu_bench/reference/Cargo.toml --target-dir target-kdu-bench/rust-v3
python tools/kdu_bench/compare.py --viewer <original-exe> --dependencies <matching-root-dll-directory> --cpp-source <cpp-checkout> --out target-kdu-bench/wide-rgba-matched-rerun
```

The production change is confined to wavelet path selection; the accompanying regression test and benchmark evidence document its correctness and measured limits.
