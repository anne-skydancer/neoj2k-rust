# Three-way whole-image benchmark

This unpublished benchmark compares neo-j2k C++, the native Rust crate and the pinned
OpenJPEG 2.5.4 oracle through their memory APIs. It uses the current Rust checkout;
`--cpp-source` must point at a matching C++ 0.1.3 checkout. No experimental audit optimization
is enabled by the benchmark.

## Run

Use Linux (including WSL2), an x86-64-v3 CPU, Rust 1.100.0, Python 3, GCC/G++, CMake, Ninja
and `taskset` from util-linux:

```sh
python3 tools/bench_three_way/run.py \
  --cpp-source ../neo-j2k \
  --out /tmp/neoj2k-three-way
```

The runner builds the C++ codec and its vendored OpenJPEG from source in Release mode,
then builds one original Rust crate with default features and no stage timers. Rust uses
opt-level 3, fat LTO, one codegen unit and `-C target-cpu=x86-64-v3`. Both native libraries
use GCC/G++ Release optimization and the source project's strict floating-point settings;
neo-j2k retains its no-IPO policy. The OpenJPEG fork and commit come from `oracle/PIN` in
the supplied C++ checkout. This is not a claim about an unmodified upstream OpenJPEG binary.

## Timed work

All requests are serial, with no executor or worker pool. Every decode creates and destroys
a fresh codec, including header setup, and writes caller-owned I32 output. Output buffers
and file loading are outside timing. Rust and C++ can reconstruct into caller buffers;
OpenJPEG allocates internal sample planes and then copies them to the caller buffer.
Adapter/descriptor overhead is included. These are API request latencies with warm input
and allocators, rather than retained-decoder latencies.

All encoders take identical interleaved 8-bit RGB input and return an owned codestream.
Creation, pixel conversion, allocation, the final owned output copy and teardown are timed.
Settings are one tile, RLCP, six resolutions, 64×64 code blocks, MCT, one layer with every pass
kept, and either reversible 5/3 or irreversible 9/7. The default comments agree across the
three libraries.

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
