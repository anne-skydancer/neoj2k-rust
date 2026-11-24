<!-- SPDX-License-Identifier: BSD-2-Clause -->
<!-- Copyright (c) 2026, Anne Skydancer (neo-j2k) -->
# Targeted decode bottleneck evidence and reproduction

Reference source: Rust07ba441, C++50f74d2 (productionC++00f273d). All instrumentation and prototypes were built in `TEMP/codex-decode-bottleneck-audit`, with maintained source untouched. Windows Ryzen9 9950X3D,8 reported available logical CPUs; rustc/cargo1.100.0; MSVC19.51.36260; CMake4.4.4. Kernel/public-small timings pinCPU7; large-public timings allow all8CPUs and use the same persistent BenchmarkPool. No competing audit builds ran in the recorded timing windows. Primary pre-window other-process load was1.74%; no continuous unrelated-load trace was collected.

## Source and experiment layout

- `packed-return.patch` is the **clean one-file candidate**. It changes only MQ renormalization argument/result transport and caller a-shift; same arithmetic, same cold outline. It does not contain capture hooks, a new MQ decision rule, size dispatch or a service. The source comment about value/register transport should also be clarified during review; the preserved patch is the tested algorithmic prototype.
- `capture-prepared-runner.patch` is measurement infrastructure for a scratch codec only. Full `Scratch::decode` remains unchanged. A tile-walk hook captures real metadata, while a prepared runner calls Scratch directly without that hook. Decode-without-store is a separate copied body; store-only uses prepared raw coefficients and original empty-block clearing, ROI0 only.
- `sources/matched-probe.rs`, its manifest/buildscript, and `capture.rs` preserve the exact prepared C++/Rust harness. `sources/packed-probe.rs` adds the candidate, all24timing orders and exactoutput checks. Paths are explicit to this workspace and must be adjusted elsewhere.
- `cpp-bridge/` contains the prepared native bridge, C ABI header, CMakeproject and user-mode sampler. Requests own their copied bytes/chunks/segments/output; the single reusable T1 scratch is ready/reserved outside timing. Runtime mode/index policy is template-selected once per batch. `run` has no vector/output allocation. Mode0 is decode+coefficientstore;1is decodeonly;2is prepared raw-coefficientstore (ROI0). Empty blocks use the actual C++ empty-block validation/null-store path.
- `sources/public-*` are clean public-path probes, with `bench-internals` **disabled**. The smallCpp gate is the repository's original test with only a cratealias. The pairedsmall source retains the old printed label `short_change`; it means packedcandidate/current here, not the earlier short-stripe optimization.

## Captures

`captures/*.bin` preserves full and reduce-2 calls for the three existing fixtures. `manifest.json` records every block's metadata, real segment/pass pairs, original chunk lengths and SHA256. Empty blocks are included. The benchmark currently measures the reduce-2 captures; full captures are preserved for reproducibility/follow-up.

Binary format, little-endian u32 words:

1. Eight-byte magic `NJDA1` plus three zero bytes; blockcount.
2. Per block:10words `[width,height,numbps,orientation,ROI,style,reversible,step_f32_bits,stride,offset]`; segmentcount; chunkcount.
3. Each segment's length/passcount pair; each original chunk length; exactly sum(chunklengths) encoded bytes.

No evenly split synthetic contributions are substituted. The `joined` counterfactual changes only the prepared chunk list to one nonempty chunk; C++ still performs its normal gather and Rust can take its existing borrowed single-chunk path. Destination offset/stride and poisoned tail remain identical. Per-block prepared outputs are intentionally separate retained buffers on both sides, so cache/caller context differs from shared full-component output in the ordinary decoder.

## Rebuild

Use isolated copies of `neo-j2k`, expanding inherited workspace package fields unchanged, keeping the library name `neo_j2k`, and assigning package names/path aliases shown in the manifests:

- `codec`: originalsource plus capture/prepared patch, package `neo-j2k-bottleneck`.
- `control`: unchanged source copy of the same measurement codec, package `neo-j2k-bottleneck-control`.
- `packed`: measurementcodec plus packed-return patch, package `neo-j2k-bottleneck-packed`.
- Publiccandidate: **originalsource plus only packed-return.patch**, package `neo-j2k-packed-public`.
- Publiccontrol: originalsource, renamed only.

Do not apply measurement infrastructure to maintained production source. Adjust hard-coded source paths in the preserved CMake/manifest/probe files on another machine.

Build native bridge with `cmake -S cpp-bridge -B BUILD/build-v3 -G "Visual Studio 18 2026" -A x64 -DNJ2K_SIMD=AVX2`, then `cmake --build BUILD/build-v3 --config RelWithDebInfo --target nj2k_audit_bridge --parallel`. Repeat a separate directory with `NJ2K_SIMD=SCALAR` for baseline. The bridge links `neoj2k_static`, uses /fp:precise /Ob2 and noIPO, following the repository codec configuration.

Set `AUDIT_CPP_BUILD` to the applicable native build directory. The preserved `build.rs` links `RelWithDebInfo/nj2k_audit_bridge.lib` and `codec/src/RelWithDebInfo/neoj2k_static.lib`. Set a separate `CARGO_TARGET_DIR`. Build the probe release with fatLTO/CGU1, defaultfeatures plus `bench-internals` for measurement-only entries. For v3 set `RUSTFLAGS='-C target-cpu=x86-64-v3'`; remove it for baseline. Pair SCALAR with baseline and AVX2 with v3.

The capture executable is `capture.rs` as `src/bin/capture.rs`. Its argument is the captureoutput directory. Existing preserved captures can be used directly.

Primary timing command shape:

```powershell
[Diagnostics.Process]::GetCurrentProcess().ProcessorAffinity=128
& PATH/decode-bottleneck-probe.exe PATH/captures 24 128 > matched-v3-1.txt
```

Repeat three processes at each CPUlevel, and repeat `decode-bottleneck-packed-probe.exe PATH/captures` three times. The latter fixes24rounds×128corpus repetitions and all24algorithm orders. Warm preparation and byte/padding parity checks precede timing; Rustfull mode asserts zero warmed allocations/reallocations. Original modes compare all6orders of Rust/C++/unchangedcontrol. Indexgroups are allblocks and each active square blocksize. Do not infer exact stage durations by subtracting independently compiled mode medians.

## Results and controls

- `matched-{baseline,v3}-{1,2,3}.txt` are originalcurrent replay; `matched-summary.json` derives medians/ranges from the within-repetition ratios.
- `packed-{baseline,v3}-{1,2,3}.txt` compare the scalar candidate; `packed-summary.json` uses the same within-repetition method.
- `public-results/` contains three repetitions of pairedsmall baseline/v3, directCppsmallv3, and pairedlarge baseline/v3 plus its derived `summary.json`. Smallpaired uses32alternatingrounds×128calls; originalCppsmall uses21alternatingrounds×16calls. Large uses18rounds×3calls, all6orders, current/candidate/unchangedcontrol, persistent workers1/2/4/8. Exact sample/status checks remain enabled.

The ratio median is not generally the ratio of the two independent time medians. Contexts/executables may have different absolute times due to caller/LTO/code placement. Do not multiply ratios from different executables, or turn candidate/control into a universal adjustment of publicC++results. Largecontrols can move severalpercent; fullraw rows are retained rather than hiding them.

## Sampling and final-code evidence

WPR could not enable the system performance policy (0xc5585011). The alternate sampler only creates and observes its own benchmarkprocess primarythread with Suspend/GetThreadContext/Resume, using DbgHelp/PDB resolution. It perturbs execution and collects instructionpointer residency, not callstacks or hardwareevents. `traces/{rust,cpp}-128-all-v3.csv` and `profile-summary.json` preserve its approximately6700samples per20-second run.

Build optional `audit_sampler` in the CMakeproject. The target executable supports:

```text
--spin CAPTURES sl_128_rgba_97.j2k all rust 20
--spin CAPTURES sl_128_rgba_97.j2k all cpp 20
```

Group can be8x8/16x16; algorithm can becontrol. It prepares/validates once, prints SPIN_READY, then repeatedly runs only that algorithm. Invoke sampler as `audit_sampler output.csv target.exe [targetargs...]`; it pins targetCPU7 and itselfCPU0. Profiling results are not timing results.

`traces/ABI-map.md`, private PDBsymbol excerpts and the original/packed finaldisassemblies establish the helper ranges, 12-byte indirect transport versus scalarRAXresult, and savedregister counts. The nearest exported objdump label is misleading; use the PDBprivate range. Frozenreference EXE/PDB SHA256s are recorded, but binaries/PDBs/fullsymbol dumps remain in TEMP to avoid bloating the report. Rebuilding can change addresses and layout.

## Validation and limits

`selected-native-unit.txt`:66 native selected tests using `--cfg miri`, not a Miri interpreter run. `cpp-native-differential.txt`:21 nativeMQ/Tier-1 oracle tests passed,3 longnightly ignored. Prepared and public probes additionally check exact outputs/status/padding. No new fullfuzz campaign, actualMiri run or Linux profiling was performed.

The actualbottleneck and narrow remedy are demonstrated for these Windowsbuilds. Capturedkernel parity does not establish every publicAPI row at parity, and no finite testmatrix guarantees zero speedregression. The main report retains remaining reducedAPI deficits and noisylarge controls. Sizecategories/service design were deferred follow-up ideas, not implementations or unmeasured substitutes for this audit.
