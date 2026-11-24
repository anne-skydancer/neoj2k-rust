# Segment-slot retention experiment

Current source: Rust 07ba441. Production source unchanged. Patch applied only to an external scratch copy.

The candidate retains initialized `CodeBlock.segs` slots instead of clearing their length on reset. `init_segment` still resets every active slot and charges the original modeled segment-growth schedule at indices 0, 4, 8, 16, … independently of retained length. This relies on sequential segment initialization and must be reviewed before production use.

Paired probe: original crate, renamed unchanged-source copy, renamed candidate; identical features; Rust 1.100.0 release fat LTO, CGU1, x86-64-v3. Windows Ryzen 9 9950X3D (8 reported logical CPUs), process pinned to CPU 7. Other audit builds/timing idle. No continuous system-load trace. Three repetitions, 36 cyclic-order rounds × 128 calls per fixture, 32 warmups, exact byte equality before timing. This renamed-source control measures package/code-placement effects, not a byte-identical executable noise control.

Results do not establish an optimization benefit on the reduced fixtures: trim against unchanged renamed control medians are +0.06%, -0.08%, -0.24% for 32²/64²/128² at reduce 2, respectively; every range crosses zero. Full 64² includes a large positive outlier. Do not present the sub-1% trim-versus-current differences as causal gains.

Validation executed `cargo run --release --manifest-path $env:TEMP/codex-small-tier2-trim/probe/Cargo.toml --bin validate` with the same v3 flag and scratch target. Result: 3510 exact outcome/statistics/diagnostic comparisons across 65 corpus files, 3 reductions, changing memory limits and full/cut inputs passed. Every decoded component's information and samples are included in the comparison. Limits cycle unlimited, 16/32/64/128 KiB, then unlimited; reductions 0/2/5 and input cuts of 0/2/7 bytes. This is native current-versus-candidate equivalence, not Miri or a new C++ oracle run.

Reproduce by copying codec source into separate standalone packages `neo-j2k-control` (unchanged) and `neo-j2k-trim` (patch applied), retaining the original features and library name. The preserved manifest shows the path aliases and release profile; adjust its explicit paths on another machine. `paired-probe.rs` is `src/main.rs`, and `validate.rs` is `src/bin/validate.rs`.
