# Three-way whole-image benchmark

This benchmark compares neo-j2k C++, the native Rust crate and unmodified upstream
OpenJPEG 2.5.4 through their memory APIs. It uses the current Rust checkout;
`--cpp-source` must point at a matching C++ 0.1.3 checkout. No experimental audit optimization
is enabled by the benchmark.

## Run

Use Linux (including WSL2), an x86-64-v3 CPU, Rust 1.100.0, Python 3, GCC/G++, CMake, Ninja
`taskset` from util-linux, and GNU binutils (`nm`, `objcopy`, `ar`):

```sh
python3 tools/bench_three_way/run.py \
  --cpp-source ../neo-j2k \
  --out /tmp/neoj2k-three-way
```

The runner builds the C++ codec and its correctness oracle from source in Release mode,
then builds one original Rust crate with default features and no stage timers. Rust uses
opt-level 3, fat LTO, one codegen unit and `-C target-cpu=x86-64-v3`. Both native libraries
use GCC/G++ Release optimization and the source project's strict floating-point settings;
neo-j2k retains its no-IPO policy. The timed reference is upstream OpenJPEG tag `v2.5.4`,
commit `6c4a29b00211eb0430fa0e5e890f1ce5c80f409f`, built independently. Its exported symbols
are renamed with `objcopy` so the timed adapter and the patched correctness oracle can coexist.
The library's codec implementation is unchanged. The existing test-only OpenJPEG adapter
is compiled against the upstream headers and linked to this isolated library.
`--openjpeg-source` accepts an existing clean checkout at that exact revision.

## Timed work

All requests are serial, with no executor or worker pool. Every decode creates and destroys
a fresh codec, including header setup, and writes caller-owned I32 output. Output buffers
and file loading are outside timing. Rust and C++ can reconstruct into caller buffers;
OpenJPEG allocates internal sample planes and then copies them to the caller buffer.
Adapter/descriptor overhead is included. These are fresh API request latencies with warm
input and allocators. Seven whole-file decode workloads run with strict mode both off and on.
One generated 1024-square RGB codestream is cut to 32 KiB and decoded at reduce 5 with strict
mode off and Clamp. All three strict decoders must reject that prefix before it is timed.

All encoders take identical interleaved 8-bit RGB input and return an owned codestream.
Creation, pixel conversion, allocation, the final owned output copy and teardown are timed.
Settings are one tile, RLCP, six resolutions, 64×64 code blocks, MCT, one layer with every pass
kept, and either reversible 5/3 or irreversible 9/7. The default comments agree across the
three libraries. Strictness is a decoder option. These encoder APIs expose no strict mode,
so each encode workload is measured once.

Each of three processes uses 12 rounds, covering all six actor orders twice. Before each
timed actor batch, 32 calls of that actor precondition execution history. Buffers rotate
between actors across processes; the actor-order start also rotates. All processes use the
same fixed executable and are pinned to the first allowed logical CPU. Small decode batches
contain eight calls, encode batches four, and large decode batches one.

Decoded samples and encoded bytes are checked across all three libraries before and after
every workload. Post-timing output buffers are poisoned before verification. Status checks
remain active on timed calls. The table reports the median of the three process medians;
`summary.json` also retains each process value and range.

## Evidence

The output directory contains raw JSONL timing/check records, `summary.json`, `manifest.json`,
compiler and feature/dependency logs, CMake compile commands and source/library/executable
hashes. Binary build trees are local artifacts. To update a published table, retain the raw
results and manifest and label the machine, source revisions, ISA, request lifecycle and
units. Results from different harnesses or machines must not be multiplied into three-way ratios.
