# Embedded Kakadu byte-output harness

The harness decodes through the Kakadu engine embedded in the supplied Firestorm
viewer, without installing or launching the viewer. **Ten lossless fixtures have
passed byte-for-byte comparison against the native Rust decoder**, including
2048x64 RGBA. It also measures repeated, fresh-codec in-memory decode requests.

These are external tools. The Rust codec remains unchanged, with zero dependencies
and unsafe code forbidden. No proprietary or viewer binaries are included.

## Supported binary and input

This adapter is specific to the EXE in
`Phoenix-Firestorm-Releasex64_AVX2-7-2-5-81788_Setup.exe`:

| Property | Verified result |
| --- | --- |
| Embedded Kakadu | `v8.6.1` |
| Original EXE SHA-256 | `c3a3b5483b9778aa64b2cb48f7e61e5aafa1c2550a04b14a9740e97c4a2bfb6e` |
| Exports / Kakadu-named exports | 488 / 485 |
| Patched private image SHA-256 | `dcd2a3726a3ad382681d55c14eec0120fbc5ba4c93279440cc7089f3ec4927da` |
| Viewer entry point invoked | No |
| CPU dispatch on the tested machine | MMX level 7 (AVX2), BMI2 level 2 |

The Python driver checks the original digest; the native adapter independently
checks the private image digest **before loading it**. Another viewer build needs
new address/layout verification. Version numbers alone are insufficient.

The first adapter supports bare J2K, one tile, 1..4 unsigned 8-bit components with
equal geometry, and dimensions up to 8192. It retains top-down raster order.
It does not implement reduced-resolution, subsampled, high-precision, JP2 or
multi-tile requests. The default suite uses reversible 5/3 fixtures. Other fixtures
are accepted only if their decoded bytes match exactly; no lossy tolerance is used.

## Build and run on Windows x64

Use an archive reader such as 7-Zip to extract `Firestorm-Releasex64.exe` and all
root-level DLLs from the same installer into a directory. Do not run the NSIS
installer. The supplied installer was inspected and extracted this way.

Use 64-bit Windows Python, CMake, MSVC C++ and the repository's Rust toolchain:

```powershell
cmake -S tools/kdu_bench -B target-kdu-bench -A x64
cmake --build target-kdu-bench --config Release
cargo build --release --manifest-path tools/kdu_bench/reference/Cargo.toml --target-dir target-kdu-bench/rust
python tools/kdu_bench/run.py --viewer C:/extracted/Firestorm-Releasex64.exe --dependencies C:/extracted --out target-kdu-bench/results
```

The reference tool depends only on the local Rust crate and forbids unsafe code.
`--iterations 0` performs validation only. The default is eight requests per round.
`--cases size_3x5_g sl_32_rgba_53` selects corpus fixture stems.
`--probe` and `--reference` override executable paths.

## What executes

The driver copies the EXE and dependencies into a temporary directory. On that
private EXE copy only, it sets the PE DLL flag and zeroes the entry-point RVA.
Windows resolves imports, and dependency initializers/TLS callbacks may execute.
The viewer entry point remains disabled. The native child explicitly invokes a
verified subset of Kakadu's original CRT initializers, then uses the viewer's
memory-source vtable and tile-output routines. [ABI.md](ABI.md) records the evidence.
No application-wide viewer startup or full CRT initializer array is invoked.

The original viewer was not a supported decoding DLL. This is an experimental,
exact-build ABI adapter, validated on the stated inputs; it is not a general
Kakadu SDK replacement. Each case runs in a hidden child with a timeout. Native
faults and C++ exceptions fail the run. The child exits without viewer shutdown;
every successful decode explicitly destroys its engines, tile, sample allocator
and codestream first.

## Timing and evidence

Input loading, output allocation, image loading/initialization and process startup
are outside timing. Each request creates a fresh codestream, reads/checks its
header and geometry, opens its tile, constructs the original tile engine,
decodes, converts to interleaved bytes and tears down the decoder. Threads are not
created by the adapter and Kakadu receives no thread environment. The process is
not pinned to a CPU.

There are 16 warm-up requests and nine timed rounds. Output is poisoned, compared
with Rust and checked for guard overwrites before and after timing. The report
contains every round and its median. The output directory retains reference and
Kakadu pixel buffers, `results.json` and `manifest.json`, including executable,
dependency, source, fixture and pixel hashes.

The `run.py` results are **Kakadu-only timings**. Rust supplies correctness
reference pixels there; it is not timed by that smaller driver. The existing three-way Linux I32-output benchmark uses a
different output type, platform and driver. Do not compare these timings directly
with that README table. The matched Windows comparison below supplies a separate three-actor measurement
with the same byte-output contract and a controlled workload order.

## Smaller probes

```powershell
python tools/kdu_bench/probe.py --viewer C:/extracted/Firestorm-Releasex64.exe --json version.json
python tools/kdu_bench/runtime.py --viewer C:/extracted/Firestorm-Releasex64.exe --dependencies C:/extracted --probe target-kdu-bench/Release/kdu_runtime_probe.exe --json runtime.json
```

The version-only probe verifies an import-free LEA/RET accessor and calls it in a
child using `DONT_RESOLVE_DLL_REFERENCES`; no imports, TLS callbacks or entry point
are initialized. `--inspect-only` reads the PE without executing code and works
on other platforms. The runtime probe resolves imports and queries version and
processor count, but performs no image decoding. Its `benchmark_available: false`
field describes that smaller probe, not `run.py`.

## Matched comparison with C++ and Rust

`compare.py` times Kakadu, neo-j2k C++ and neo-j2k Rust on the same Windows machine.
C++ is built from the supplied source checkout; Rust uses the current local crate.
The production crate is unchanged. Kakadu explicitly uses `set_fast`, as the
viewer's MODE_FAST path does.

```powershell
cmake -S tools/kdu_bench -B target-kdu-bench -A x64 -DNEOJ2K_CPP_SOURCE=C:/source/neo-j2k
cmake --build target-kdu-bench --config Release --target kdu_compare_native kdu_decode_probe
$env:RUSTFLAGS = '-C target-cpu=x86-64-v3'
cargo build --release --manifest-path tools/kdu_bench/reference/Cargo.toml --target-dir target-kdu-bench/rust-v3
python tools/kdu_bench/compare.py --viewer C:/extracted/Firestorm-Releasex64.exe --dependencies C:/extracted --cpp-source C:/source/neo-j2k --out target-kdu-bench/comparison-fast
```

This comparison uses fresh decoders, caller-owned top-down interleaved U8 output,
and includes header/geometry checks, reconstruction, conversion and full decoder
teardown. Both neo-j2k implementations use their native U8 output APIs. File IO,
process and library startup, dispatch initialization, output allocation and pixel
validation are outside timing.

Three long-lived worker processes are pinned to the same first allowed logical
CPU; only one receives work at a time. C++ and Kakadu use the same host executable
in separate actor processes, and Rust uses a safe native Rust worker. Three fresh
process trials rotate process creation order. Each trial visits all six actor
orders. Each batch has 32 warm-up requests for outputs up to 64 KiB and four for
larger outputs, with the timed batch calibrated per actor to about 20 ms (1..2048
requests). Every batch poisons and checks all output pixels and both guards before
and after timing. The result is the median of three trial medians, with all round
values retained. Process isolation means this does not eliminate all binary or
address-layout sensitivity; the per-trial values expose measured variation.

The four square fixtures are deterministic synthetic RGB images generated by the
safe Rust encoder, with reversible 5/3, MCT and six resolutions. Expected output
comes directly from the original uncompressed bytes, independently of decoding.
The 2048x64 RGBA fixture comes from the existing corpus, checked against the safe
Rust planar reference. These are lossless tests, not a 9/7 or general image survey.
Every actor must match every expected byte; a mismatch aborts the comparison.

`summary.json`, `rounds.json` and `manifest.json` retain per-trial and per-round
measurements, source/compiler/build metadata, CPU/affinity, and input, output,
executable, library and dependency hashes. The fixture generator and worker
executables must be built as above. C++ uses MSVC Release AVX2 with its no-IPO
policy; Rust uses release, x86-64-v3, fat LTO and one codegen unit. Kakadu is the
precompiled 8.6.1 viewer engine; its compiler flags are not controlled here.

Results describe the recovered viewer tile-output path, not the fastest possible
Kakadu SDK application, stripe API or multithreaded configuration. Do not combine
these Windows U8 timings with the separate Linux I32 benchmark table.
