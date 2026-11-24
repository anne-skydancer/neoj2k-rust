# Boundary and larger-reduced follow-up evidence

All variants are isolated copies of Rust `07ba441`. No production edits, staging or commits. Same Windows Ryzen 9950X3D /8 reported logical CPUs, rustc 1.100.0, release fatLTO/CGU1, default features; V3 sets `-C target-cpu=x86-64-v3`, baseline does not. Timing processes were pinned to CPU 7, and audit builds were idle. One runtime-boundary baseline repetition overlapped an independently reported histogram build; that repetition was discarded and rerun before preserving the final raw file. No continuous unrelated-load trace was collected.

Each probe includes the original current crate, a renamed unchanged-source copy (`control`), and the previous short-only candidate. Controls assess package/code-placement effects, not a byte-identical executable noise control. All variants warm their own reusable decoder and byte-output buffers32 times, verify exact output bytes, then rotate through all timing positions cyclically. Every raw file contains all full/reduced fixture results; the report does not cherry-pick reduced gains while hiding full-row regressions.

## Variant and patch mapping

| Set / label | Patch | Change |
|---|---|---|
| boundary / boundary | boundary-store.patch | Runtime top/bottom-real predicates; no initialization trim |
| boundary / init | boundary-init.patch | Above plus omit outer guard initialization |
| boundary / refine | refine-view.patch | Refinement takes only its own exact-width Cell row |
| boundary / combined | boundary-refine.patch | Runtime boundary/init plus refinement view |
| const-boundary / boundary | boundary-const.patch | Const first/interior/last/single dispatch outside each stripe |
| const-boundary / init | boundary-const-init.patch | Const boundary plus initialization trim |
| const-boundary / sides | boundary-sides.patch | Runtime suppression of padding side/diagonal stores |
| const-boundary / combined | boundary-const-sides.patch | Const/init plus side stores |
| mq-transform / transform | mq-transform.patch | Const MQ type selected once per block by irreversible transform |
| mq-small / geometry | mq-small-block.patch | FAST only if irreversible and width/height ≤ 8 |
| width / w8,w16,both | width-8.patch,width-16.patch,width-8-16.patch | Const pass width 8,16 or both; generic fallback |

The boundary and const-boundary sets use35 cyclic rounds × 64 calls; MQ-transform uses 36×64, MQ-small35×64; width 36×64. Three executable repetitions per set/CPU. MQ-small also includes the broader transform candidate as a comparator. Original current and renamed control are distinct package identities compiled into one executable.

Files `*-probe.rs` and corresponding `*-probe-Cargo.toml` preserve exact paired harnesses and dependency aliases. Fixture and dependency paths are explicit to this workspace; adjust elsewhere. To rebuild, copy original codec source to standalone packages with the names in each manifest, expand workspace-inherited manifest fields unchanged, and apply each named patch to its copy. Keep the library name `neo_j2k`. Follow the release profile and quiet pinned-window instructions in the parent evidence README. Do not apply these rejected/experimental patches to maintained source as a bundle.

Raw `boundary-{baseline,v3}-*.txt`, `const-boundary-{baseline,v3}-*.txt`, `mq-transform-{baseline,v3}-*.txt`, `mq-small-{baseline,v3}-*.txt` and `width-{baseline,v3}-*.txt` contain all medians. `summary.json` derives per-repetition candidate/current and candidate/control percentages from those printed microseconds, reporting median/min/max. The extraction uses the printed three-decimal timings, so rounding may differ slightly from the raw two-decimal percent column. Source/control changes between separately linked probe sets can move the same candidate's result; do not mix their absolute timings or imply a universal corrected score.

Validation outputs `validation-boundary-const-sides.txt`, `validation-mq-small-block.txt`, `validation-width-8-16.txt` each record 8 native current C++ Tier-1 differential tests passed,2 long nightly sweeps ignored. These representative combined variants cover the changed flag invariant, generic RAW/VSC/partial handling and MQ path. Every timing variant separately passed fixture byte equality. Native tests do not replace actual Miri, broad fuzzing or production performance gates. No final executable hardware counters were collected, and source code size/branch explanations remain mechanisms to investigate, not measured microarchitectural attribution.

The larger-fixture objective is still open. The most aggressive boundary and restricted MQ variants regress; segment retention has no clear gain after control; bounded width specializations are small/mixed. The report's prepared real-block oracle specification states the concrete next diagnostic/implementation work instead of declaring parity achieved.
