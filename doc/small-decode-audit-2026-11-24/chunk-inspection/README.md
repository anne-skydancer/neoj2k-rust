# Chunk inspection

The instrumented scratch codec counts incoming Tier-1 chunks after their iterator is consumed normally. Process-wide atomic counters and local Cells are instrumentation only. Every original decode succeeds; no latency claim is made from this build.

Current source: Rust `07ba441`. Apply `instrumentation.patch` only to an isolated copy of the codec; expose its library under the `codec` dependency alias and use `probe.rs` as a temporary executable. It was run with Rust 1.100.0, release fat LTO/CGU1, x86-64-v3 on Windows. Adjust explicit fixture paths to reproduce elsewhere.

There are no zero-length chunks in these active blocks. Filtering zero-length chunks cannot help these fixtures. The larger fixtures genuinely have multiple nonempty chunks in nearly every active block, reflecting their multilayer inputs. This does not quantify concatenation cost or establish that it explains the C++ gap.
