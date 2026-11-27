# Strict-mode and KDU benchmark rerun

Measured 2026-11-27 using Rust source `0cb2f3f07a06aae7fa73a2cbabb738df05923a64`
and C++ source `0535cc9b9c68909c6244dca95db3ec9123f20d06`. Runtime decoder sources
include the partial-download correction. The Rust changes for this run extend only
the test adapter and benchmark drivers to select strictness explicitly.

## Three-way measurements

The timed reference is a clean checkout of upstream OpenJPEG 2.5.4 at
`6c4a29b00211eb0430fa0e5e890f1ce5c80f409f`. Its global symbols are renamed so the
test executable can also link the patched correctness oracle. The test-only
OpenJPEG adapter is rebuilt against the upstream headers and links the isolated
upstream library. Both implementations coexist in the executable, and the timed
adapter calls the upstream-prefixed entry points. The codec source is unchanged.

Linux/WSL2, AMD Ryzen 9 9950X3D, one logical CPU, one thread. Three processes each
run twelve balanced actor-order rounds with 32 warm-up calls per actor batch.
The report uses medians of process medians. `linux/summary.json` retains the
three process values and ranges for every actor and workload. The manifest records
compilers, CPU affinity, source/library/executable hashes and generated fixture hashes.

Seven whole images run in both decoder modes. A generated 1024-square RGB image
is cut to 32768 bytes and decoded at reduce 5, Clamp, in non-strict mode. Strict
rejection is checked independently in every process. Encode has no strictness flag.
All 21 cases compare every sample or encoded byte before and after timing, with
decode buffers poisoned before final verification. Raw per-round records are in
`linux/rep-*.jsonl`.

Reproduce from this Rust checkout with an available matching C++ checkout.

```sh
python3 tools/bench_three_way/run.py --cpp-source ../neo-j2k --out /tmp/strict-mode-bench
```

## KDU measurements

Windows, same CPU, one pinned logical CPU, three fresh-process trials covering all
six actor orders. Each batch is calibrated to approximately 20 ms. All output
bytes and guards are checked before and after every batch. Each fixture's summary
retains its three trial medians. `kdu/rounds.json` retains all 270 timings.
The prebuilt KDU configuration is Firestorm's recovered fast path. The binary
and dependency hashes are recorded in `kdu/manifest.json`. Proprietary binaries
and private textures are kept outside the repository.

```powershell
python tools/kdu_bench/compare.py --viewer <verified-Firestorm-81788.exe> --dependencies <matching-DLL-folder> --cpp-source <neo-j2k-source> --build <rebuilt-actors> --out <result-folder> --trials 3
```

## Results

The README performance section is reproduced below. Units, request lifecycle and
toolchain differ between the Linux I32 comparison and Windows U8 comparison.

## Performance

Measured on **2026-11-27**, after the partial-download correction, on an AMD Ryzen 9
9950X3D with one thread pinned to one logical CPU. The three-way tables use Linux/WSL2
and AVX2 builds. Rust is `0cb2f3f`, C++ is `0535cc9`, both version 0.1.3. The timed
OpenJPEG reference is **unmodified upstream 2.5.4**, commit `6c4a29b`, built independently
of the patched correctness oracle. Inputs are deterministic synthetic images.

Each timing cell gives **milliseconds / megapixels per second**. Lower latency and
higher throughput are faster. Decode throughput counts output pixels after reduction.
Encode throughput counts input pixels. Speedup is OpenJPEG time divided by the
candidate time. Values use the median of three process medians.

### Decode with strict mode off

| Input | OpenJPEG | neo-j2k C++ | neoj2k-rust | C++ speedup | Rust speedup |
|---|---:|---:|---:|---:|---:|
| 32 × 32 RGBA, 5/3 | 0.3843 / 2.7 | 0.1720 / 6.0 | 0.1694 / 6.0 | 2.23× | 2.27× |
| 64 × 64 RGB, 9/7 | 0.0918 / 44.6 | 0.0536 / 76.4 | 0.0577 / 71.0 | 1.71× | 1.59× |
| 128 × 128 RGBA, 9/7 | 0.3617 / 45.3 | 0.1114 / 147.0 | 0.1204 / 136.1 | 3.25× | 3.00× |
| 64 × 64 RGB → 16 × 16, 9/7 | 0.0289 / 8.8 | 0.0207 / 12.4 | 0.0267 / 9.6 | 1.40× | 1.08× |
| 1,024 × 1,024 RGB, 9/7 | 13.2922 / 78.9 | 4.3805 / 239.4 | 4.2669 / 245.7 | 3.03× | 3.12× |
| 4,096 × 4,096 grayscale, 9/7 | 81.3905 / 206.1 | 12.9250 / 1,298.0 | 12.0861 / 1,388.1 | 6.30× | 6.73× |
| 2,048 × 64 RGBA, 5/3 | 1.7817 / 73.6 | 1.1832 / 110.8 | 1.3153 / 99.7 | 1.51× | 1.35× |
| 1,024² RGB, 32 KiB prefix → 32², 9/7 | 0.1538 / 6.7 | 0.1173 / 8.7 | 0.1148 / 8.9 | 1.31× | 1.34× |

### Decode with strict mode on

| Input | OpenJPEG | neo-j2k C++ | neoj2k-rust | C++ speedup | Rust speedup |
|---|---:|---:|---:|---:|---:|
| 32 × 32 RGBA, 5/3 | 0.3803 / 2.7 | 0.1690 / 6.1 | 0.1681 / 6.1 | 2.25× | 2.26× |
| 64 × 64 RGB, 9/7 | 0.0936 / 43.8 | 0.0545 / 75.2 | 0.0563 / 72.7 | 1.72× | 1.66× |
| 128 × 128 RGBA, 9/7 | 0.2970 / 55.2 | 0.1098 / 149.3 | 0.1167 / 140.4 | 2.71× | 2.54× |
| 64 × 64 RGB → 16 × 16, 9/7 | 0.0283 / 9.0 | 0.0207 / 12.4 | 0.0265 / 9.7 | 1.37× | 1.07× |
| 1,024 × 1,024 RGB, 9/7 | 13.2544 / 79.1 | 4.2937 / 244.2 | 4.0631 / 258.1 | 3.09× | 3.26× |
| 4,096 × 4,096 grayscale, 9/7 | 80.9836 / 207.2 | 13.0672 / 1,283.9 | 12.2120 / 1,373.8 | 6.20× | 6.63× |
| 2,048 × 64 RGBA, 5/3 | 1.7653 / 74.2 | 1.2031 / 108.9 | 1.3103 / 100.0 | 1.47× | 1.35× |

The 32 KiB prefix uses reduce 5 and Clamp. All three non-strict decoders returned
identical samples. All three strict decoders rejected that prefix, so strict mode
has no successful partial-image throughput row.

### Encode

**Strict mode is a decoder setting.** These three encoder APIs have no strict-on or
strict-off option, so the encode table has one measurement per configuration.
All three produced identical codestream bytes. Settings are one tile, RLCP, six
resolutions, 64 × 64 codeblocks, MCT, and one layer retaining every coding pass.

| Input | OpenJPEG | neo-j2k C++ | neoj2k-rust | C++ speedup | Rust speedup |
|---|---:|---:|---:|---:|---:|
| 32 × 32 RGB, 5/3 | 0.2801 / 3.7 | 0.1251 / 8.2 | 0.1110 / 9.2 | 2.24× | 2.52× |
| 32 × 32 RGB, 9/7 | 0.3209 / 3.2 | 0.1129 / 9.1 | 0.1159 / 8.8 | 2.84× | 2.77× |
| 64 × 64 RGB, 5/3 | 0.9892 / 4.1 | 0.6278 / 6.5 | 0.6241 / 6.6 | 1.58× | 1.59× |
| 64 × 64 RGB, 9/7 | 1.1716 / 3.5 | 0.6468 / 6.3 | 0.6809 / 6.0 | 1.81× | 1.72× |
| 128 × 128 RGB, 5/3 | 3.4779 / 4.7 | 2.4384 / 6.7 | 2.4172 / 6.8 | 1.43× | 1.44× |
| 128 × 128 RGB, 9/7 | 4.3060 / 3.8 | 2.5334 / 6.5 | 2.6090 / 6.3 | 1.70× | 1.65× |

Rust 1.100.0 uses release optimization, fat LTO and one codegen unit. C++ and
OpenJPEG use GCC/G++ 16.2.1 Release builds with strict floating-point settings.
Each decode creates a fresh codec and writes caller-owned I32 planes. OpenJPEG
allocates internal planes and copies them out. Encode timings include codec
creation, pixel conversion, allocation, owned output and teardown. File I/O and
caller decode-buffer allocation are excluded. Every workload's samples or encoded
bytes are compared before and after timing.

### neo-j2k, neoj2k-rust and KDU

Separate **Windows** measurements use the same current C++ and Rust sources,
caller-owned interleaved U8 output and fresh serial decoders. C++ uses MSVC Release
AVX2, Rust uses release x86-64-v3/fat LTO, and KDU is the recovered Kakadu 8.6.1
`set_fast` path from Firestorm 81788. The adapter supports whole-file lossless
decoding. Every output byte and both output guards matched in every timed batch.
Cells again give **milliseconds / megapixels per second**. Speedup compares KDU time
with each neo-j2k implementation's time.

| Lossless 5/3 input | KDU 8.6.1 | neo-j2k C++ | neoj2k-rust | C++ speedup | Rust speedup |
|---|---:|---:|---:|---:|---:|
| 32 × 32 RGB | 0.3434 / 3.0 | 0.1294 / 7.9 | 0.1274 / 8.0 | 2.65× | 2.69× |
| 128 × 128 RGB | 3.6867 / 4.4 | 2.9245 / 5.6 | 2.6944 / 6.1 | 1.26× | 1.37× |
| 512 × 512 RGB | 47.4046 / 5.5 | 34.1487 / 7.7 | 33.7239 / 7.8 | 1.39× | 1.41× |
| 1,024 × 1,024 RGB | 185.1226 / 5.7 | 128.0617 / 8.2 | 127.5871 / 8.2 | 1.45× | 1.45× |
| 2,048 × 64 RGBA | 2.1553 / 60.8 | 1.8688 / 70.1 | 1.7509 / 74.9 | 1.15× | 1.23× |


The [benchmark report](README.md) contains
source/build provenance, raw measurements, process variation and reproduction
commands. The [three-way driver](../../../tools/bench_three_way/README.md) and
[KDU driver](../../../tools/kdu_bench/README.md) describe the timed work. Historical figures
remain in the dated reports under `doc/benchmarks/`.
