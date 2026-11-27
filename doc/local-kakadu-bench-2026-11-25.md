# Local Kakadu / neo-j2k decode benchmark — 2026-11-25

All three actors decode identical lossless codestreams into caller-owned, top-down interleaved 8-bit output. Every pixel and both output guards matched before and after every timed batch. The results below are from the completed fast-mode comparison; no Linux benchmark values are included.

| Image | Kakadu 8.6.1 (ms) | neo-j2k C++ 0.1.3 (ms) | neo-j2k Rust 0.1.3 (ms) | C++ speedup vs KDU | Rust speedup vs KDU |
| --- | ---: | ---: | ---: | ---: | ---: |
| 32×32 RGB | 0.329 | 0.130 | 0.121 | 2.53× | 2.71× |
| 128×128 RGB | 3.441 | 2.722 | 2.528 | 1.26× | 1.36× |
| 512×512 RGB | 46.917 | 33.714 | 33.469 | 1.39× | 1.40× |
| 1024×1024 RGB | 168.781 | 118.297 | 117.963 | 1.43× | 1.43× |
| 2048×64 RGBA | 1.994 | 1.722 | 2.227 | 1.16× | 0.90× |

Lower latency is better. Speedup is Kakadu time divided by neo-j2k time; values below 1 mean neo-j2k is slower. C++ wins on all five tested inputs. Rust wins on the square RGB inputs; the 2048×64 RGBA case takes about 11.7% longer than Kakadu.

## Machine and builds

- Windows x64; AMD Ryzen 9 9950X3D 16-Core Processor; 8 logical processors visible to Windows. Every worker was pinned to affinity mask `0x1`.
- C++ source commit `81407c2ef7cfa146905678f09a6c11295e52b15c`; clean source tree. Built from source with MSVC 19.51.36260, Release AVX2, strict floating point, no IPO.
- Rust source commit `3000f7d127222734e18de9cd69d05d475e01cb6e`; benchmark additions uncommitted, production codec unchanged. Built from source with Rust 1.100.0, release optimization, `-C target-cpu=x86-64-v3`, fat LTO and one codegen unit. The codec still forbids unsafe and has no dependencies.
- Kakadu is the prebuilt 8.6.1 core in Firestorm 7.2.5.81788 AVX2. The adapter calls its original tile engine and byte conversion, explicitly selects `set_fast` (the viewer MODE_FAST setting), and initializes the verified Kakadu startup subset. The compiler settings used to build Kakadu are not controlled by this benchmark.
- The installer and viewer were neither installed nor launched. Only a private DLL-flagged, entry-point-disabled image was loaded. The original binary is SHA-256 pinned.

## Work and verification

Three long-lived actor processes execute serially on the same logical CPU. C++ and Kakadu share one host executable, running in separate actor processes; Rust runs its native safe worker. Each request creates a fresh decoder and includes header/geometry validation, tile setup, decode, byte conversion and complete decoder teardown. Input/output allocation, file IO, process and library startup, dispatch initialization and pixel comparison are excluded.

Each of three fresh process trials covers all six actor orders, with creation order and starting actor order rotated between trials. Each batch has 32 warm-up calls for outputs up to 64 KiB and four for larger outputs. A preliminary per-actor calibration selects 1..2048 timed calls targeting approximately 20 ms. The table is the median of three trial medians; all 270 batch timings are retained.

The 32², 128², 512² and 1024² fixtures are deterministic synthetic RGB, reversible 5/3, MCT, six resolutions, encoded once by the Rust encoder. Expected pixels are the original input bytes, rather than a decode assumed to be correct. The wide RGBA fixture is the existing lossless corpus file, with a reference produced by the Rust planar decoder and then independently matched by both other actors. No tolerance is used.

## Per-trial medians

| Image / actor | Trial 1 (ms) | Trial 2 (ms) | Trial 3 (ms) |
| --- | ---: | ---: | ---: |
| 32×32 Kakadu | 0.329 | 0.308 | 0.332 |
| 32×32 C++ | 0.129 | 0.130 | 0.137 |
| 32×32 Rust | 0.121 | 0.120 | 0.122 |
| 128×128 Kakadu | 3.413 | 3.485 | 3.441 |
| 128×128 C++ | 2.722 | 2.757 | 2.722 |
| 128×128 Rust | 2.514 | 2.599 | 2.528 |
| 512×512 Kakadu | 47.069 | 45.691 | 46.917 |
| 512×512 C++ | 33.714 | 35.319 | 33.450 |
| 512×512 Rust | 33.469 | 34.206 | 33.118 |
| 1024×1024 Kakadu | 170.290 | 168.569 | 168.781 |
| 1024×1024 C++ | 122.432 | 118.297 | 117.648 |
| 1024×1024 Rust | 119.467 | 117.963 | 117.665 |
| 2048×64 Kakadu | 1.946 | 2.091 | 1.994 |
| 2048×64 C++ | 1.711 | 1.817 | 1.722 |
| 2048×64 Rust | 2.135 | 2.278 | 2.227 |

## Scope and evidence

This measures the recovered Firestorm tile-output path, not the fastest possible Kakadu SDK, stripe-decompression, retained-decoder or multithreaded configuration. These synthetic lossless results do not establish performance for lossy 9/7, high precision, subsampling, multiple tiles or arbitrary texture corpora. Separate worker executables and address layouts can still affect performance; the raw per-trial values expose the observed variation. Do not combine these Windows U8 numbers with the Linux I32-output README benchmark.

[Harness and reproduction commands](../tools/kdu_bench/README.md#matched-comparison-with-c-and-rust), [summary](benchmarks/kakadu-windows-2026-11-25/summary.json), [all rounds](benchmarks/kakadu-windows-2026-11-25/rounds.json), [manifest and hashes](benchmarks/kakadu-windows-2026-11-25/manifest.json). Generated codestreams and per-actor output files remain under `target-kdu-bench/comparison-fast/`; regenerate them with the harness. No proprietary binaries or pixel fixtures were copied into this report.
