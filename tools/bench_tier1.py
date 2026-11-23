#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
"""The Tier-1 benchmark's driver (doc/bench.md).

Builds `neo-j2k-bench` from a base ref, from the head (this working tree, or a ref) and copies the base's
executable as the same-source control; runs the three alternately and interleaved (every round takes them in
another order, all six orders in turn), each pinned to one logical CPU; and writes one table: the head's and the
control's change of the cycles of one pass over each set against the base's, as percent (minus is faster), and the
gate: the head at least 3.0 % faster, the control within +-1 %, the outputs identical.

usage: bench_tier1.py [--base-ref REF] [--head-ref REF] [--rounds N] [--pin CPU] [--cpu baseline|x86-64-v3]
                      [--cpp-source DIR] [--workdir DIR] [--min-mcycles N] [--units N] [--strict] [--keep]
       bench_tier1.py --base-exe A --head-exe B [...]      (executables built beforehand)
       bench_tier1.py --selftest
"""
import argparse
import itertools
import os
import platform
import re
import shutil
import statistics
import subprocess
import sys
import tempfile

BENCH = "neo-j2k-bench"
RESULT = re.compile(r"^RESULT set=(\S+) blocks=(\d+) inner=(\d+) per_pass=(\d+) digest=([0-9a-f]{16})$")
MIN_GAIN = 3.0  # percent faster, the gate
CONTROL_BAND = 1.0  # percent, the gate


def parse_results(text):
    """{set: (per_pass cycles, digest)} from one run's output; the header line and the rest are skipped."""
    out = {}
    for line in text.splitlines():
        m = RESULT.match(line.strip())
        if m:
            out[m.group(1)] = (int(m.group(4)), m.group(5))
    return out


def change(new, old):
    """The change of `new` against `old` in percent: minus is faster."""
    return round((new / old - 1.0) * 100.0, 9)


def orders(rounds, names):
    """The order the builds run in, round by round: every permutation in turn, so no build keeps a position."""
    perms = list(itertools.permutations(names))
    return [perms[r % len(perms)] for r in range(rounds)]


def summarize(samples, base="base", head="head", control="control"):
    """Rows from {build: [{set: (cycles, digest)}, ...]} (one dict per round, the rounds in step).

    A row per set: the base's median cycles, the median over the rounds of each round's head / control against that
    round's base (percent), whether the three builds' outputs agree, and the gate."""
    rows = []
    for name in sorted(samples[base][0]):
        per_round = range(len(samples[base]))
        head_changes = [change(samples[head][r][name][0], samples[base][r][name][0]) for r in per_round]
        control_changes = [change(samples[control][r][name][0], samples[base][r][name][0]) for r in per_round]
        digests = {samples[b][r][name][1] for b in (base, head, control) for r in per_round}
        identical = len(digests) == 1
        h, c = statistics.median(head_changes), statistics.median(control_changes)
        rows.append({
            "set": name,
            "base_mcycles": statistics.median(samples[base][r][name][0] for r in per_round) / 1e6,
            "head": h,
            "control": c,
            "identical": identical,
            "gate": h <= -MIN_GAIN and abs(c) <= CONTROL_BAND and identical,
        })
    return rows


def pct(value):
    """A signed percent with one decimal; a change that rounds to zero has no minus sign."""
    value = 0.0 if abs(value) < 0.05 else value
    return "%+.1f %%" % value


def cpu_name():
    """The processor's name where the system gives it, else Python's."""
    try:
        if os.name == "nt":
            out = subprocess.run(["powershell", "-NoProfile", "-Command",
                                  "(Get-CimInstance Win32_Processor | Select-Object -First 1).Name"],
                                 capture_output=True, text=True, timeout=20).stdout.strip()
        else:
            with open("/proc/cpuinfo", encoding="utf-8") as f:
                out = next((l.split(":", 1)[1].strip() for l in f if l.startswith("model name")), "")
        if out:
            return out
    except (OSError, subprocess.SubprocessError):
        pass
    return platform.processor() or platform.machine()


def yes_no(flag):
    return "Yes" if flag else "No"


def table(rows):
    """The one table, in Markdown: percent changes (minus is faster) and Yes/No columns, no ratios."""
    head = ["Set", "Base (Mcycles)", "Head vs base", "Control vs base", "Output identical", "Gate"]
    lines = ["| " + " | ".join(head) + " |", "|---|---:|---:|---:|:---:|:---:|"]
    for r in rows:
        lines.append("| %s | %.2f | %s | %s | %s | %s |" % (
            r["set"], r["base_mcycles"], pct(r["head"]), pct(r["control"]), yes_no(r["identical"]), yes_no(r["gate"])))
    passed = sum(1 for r in rows if r["gate"])
    lines.append("")
    lines.append("Gate (head at least %.1f %% faster, control within +-%.1f %%, outputs identical): %s (%d of %d sets)."
                 % (MIN_GAIN, CONTROL_BAND, yes_no(passed == len(rows)), passed, len(rows)))
    return "\n".join(lines)


def run(cmd, **kw):
    r = subprocess.run(cmd, capture_output=True, text=True, **kw)
    if r.returncode != 0:
        sys.stderr.write(r.stdout + r.stderr)
        raise SystemExit("bench_tier1: %s failed (%d)" % (" ".join(map(str, cmd)), r.returncode))
    return r.stdout


def git(root, *args):
    return run(["git", *args], cwd=root).strip()


def build(root, target_dir, rustflags):
    """`cargo build --release --locked -p neo-j2k-bench` in `root`; the executable's path."""
    env = dict(os.environ, CARGO_TARGET_DIR=target_dir, CARGO_INCREMENTAL="0")
    if rustflags:
        env["RUSTFLAGS"] = rustflags
    else:
        env.pop("RUSTFLAGS", None)
    run(["cargo", "build", "--release", "--locked", "-p", BENCH], cwd=root, env=env)
    exe = os.path.join(target_dir, "release", BENCH + (".exe" if os.name == "nt" else ""))
    if not os.path.isfile(exe):
        raise SystemExit("bench_tier1: no executable at " + exe)
    return exe


def worktree(root, ref, path):
    git(root, "worktree", "add", "--detach", path, ref)
    return path


def overlay_harness(head_root, base_root):
    """The head's measuring code in the base's tree, so that only the codec differs between the builds."""
    src_head = os.path.join(head_root, BENCH, "src")
    src_base = os.path.join(base_root, BENCH, "src")
    if not os.path.isdir(src_base):
        raise SystemExit("bench_tier1: the base ref has no %s crate: take a base that contains it" % BENCH)
    shutil.copytree(src_head, src_base, dirs_exist_ok=True)


def measure_rounds(exes, rounds, args, dumps):
    """Runs the builds alternately; the rounds after the first (a warm-up) as {build: [{set: ...}, ...]}."""
    samples = {name: [] for name in exes}
    for r, order in enumerate(orders(rounds + 1, list(exes))):
        for name in order:
            cmd = [exes[name], "--pin", str(args.pin), "--min-mcycles", str(args.min_mcycles),
                   "--units", str(args.units), "--dumps", dumps]
            result = parse_results(run(cmd))
            if r > 0:
                samples[name].append(result)
        print("bench_tier1: round %d of %d done (%s)" % (r, rounds, " ".join(order)), file=sys.stderr)
    return samples


def environment(args, refs):
    cpu = cpu_name()
    return ["neo-j2k Tier-1 benchmark: base %s, head %s, control = copy of the base" % refs,
            "CPU %s; pinned to logical CPU %s; %s build; %d rounds of the three builds, interleaved" % (
                cpu, args.pin, args.cpu, args.rounds),
            "each set: median of %d timed units of at least %s Mcycles, cycles of one pass; change = head / base "
            "per round, median over the rounds; minus is faster" % (args.units, args.min_mcycles), ""]


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    ap.add_argument("--base-ref", default="origin/main", help="the baseline (default origin/main)")
    ap.add_argument("--head-ref", help="build this ref as the head instead of the working tree")
    ap.add_argument("--base-exe", help="a prebuilt base executable (with --head-exe)")
    ap.add_argument("--head-exe", help="a prebuilt head executable (with --base-exe)")
    ap.add_argument("--rounds", type=int, default=21, help="rounds of the three builds (default 21, after a warm-up)")
    ap.add_argument("--pin", type=int, default=2, help="the logical CPU (default 2)")
    ap.add_argument("--cpu", choices=["baseline", "x86-64-v3"], default="baseline")
    ap.add_argument("--cpp-source", default=os.environ.get("NJ2K_CPP_SOURCE"),
                    help="the C++ neo-j2k checkout, for the six decoder dumps (default NJ2K_CPP_SOURCE)")
    ap.add_argument("--workdir", help="the folder for the builds (default: a temporary one)")
    ap.add_argument("--min-mcycles", type=float, default=30.0)
    ap.add_argument("--units", type=int, default=3)
    ap.add_argument("--strict", action="store_true", help="exit 1 when the gate is No")
    ap.add_argument("--keep", action="store_true", help="keep the work folder")
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    if not args.cpp_source:
        raise SystemExit("bench_tier1: set NJ2K_CPP_SOURCE or pass --cpp-source (the six decoder dumps are read from it)")
    dumps = os.path.join(args.cpp_source, "tests", "vectors", "t1_bench")
    if not os.path.isdir(dumps):
        raise SystemExit("bench_tier1: no " + dumps)
    work = args.workdir or tempfile.mkdtemp(prefix="nj2k-bench-")
    os.makedirs(work, exist_ok=True)
    made = []
    try:
        if args.base_exe or args.head_exe:
            if not (args.base_exe and args.head_exe):
                raise SystemExit("bench_tier1: --base-exe and --head-exe go together")
            base_exe, head_exe, refs = args.base_exe, args.head_exe, (args.base_exe, args.head_exe)
        else:
            flags = "-C target-cpu=x86-64-v3" if args.cpu == "x86-64-v3" else ""
            base_root = worktree(root, args.base_ref, os.path.join(work, "base"))
            made.append(base_root)
            if args.head_ref:
                head_root = worktree(root, args.head_ref, os.path.join(work, "head"))
                made.append(head_root)
            else:
                head_root = root
            overlay_harness(head_root, base_root)
            print("bench_tier1: building the base", file=sys.stderr)
            base_exe = build(base_root, os.path.join(work, "target-base"), flags)
            print("bench_tier1: building the head", file=sys.stderr)
            head_exe = build(head_root, os.path.join(work, "target-head"), flags)
            refs = (git(root, "rev-parse", "--short", args.base_ref),
                    git(head_root, "rev-parse", "--short", "HEAD") + ("" if args.head_ref else " + working tree"))
        control_exe = os.path.join(work, "control-" + os.path.basename(base_exe))
        shutil.copyfile(base_exe, control_exe)  # byte for byte the base's executable
        shutil.copymode(base_exe, control_exe)
        exes = {"base": base_exe, "head": head_exe, "control": control_exe}
        samples = measure_rounds(exes, args.rounds, args, dumps)
        rows = summarize(samples)
        print("\n".join(environment(args, refs)))
        print(table(rows))
        return 1 if args.strict and not all(r["gate"] for r in rows) else 0
    finally:
        for path in made:
            subprocess.run(["git", "worktree", "remove", "--force", path], cwd=root, capture_output=True)
        if not args.keep and not args.workdir:
            shutil.rmtree(work, ignore_errors=True)


def selftest():
    text = ("neo-j2k-bench 0.1.3: clock TSC cycles, pin 2\n"
            "RESULT set=decode/a blocks=3 inner=1 per_pass=1000 digest=00000000000000aa\n"
            "RESULT set=encode/b blocks=4 inner=2 per_pass=2000 digest=00000000000000bb\nnoise\n")
    got = parse_results(text)
    assert got == {"decode/a": (1000, "00000000000000aa"), "encode/b": (2000, "00000000000000bb")}, got
    assert abs(change(96.0, 100.0) + 4.0) < 1e-9 and abs(change(101.0, 100.0) - 1.0) < 1e-9
    names = ["base", "head", "control"]
    seen = {o for o in orders(6, names)}
    assert len(seen) == 6 and all(sorted(o) == sorted(names) for o in seen)
    assert orders(7, names)[6] == orders(7, names)[0]

    def samples(head, control, digest_head="aa"):
        mk = lambda cycles, digest: [{"decode/a": (cycles, digest * 8)} for _ in range(5)]
        return {"base": mk(1000, "aa"), "head": mk(head, digest_head), "control": mk(control, "aa")}

    row = summarize(samples(950, 1005))[0]
    assert abs(row["head"] + 5.0) < 1e-9 and abs(row["control"] - 0.5) < 1e-9 and row["gate"] and row["identical"]
    assert not summarize(samples(980, 1005))[0]["gate"]  # only 2 % faster
    assert not summarize(samples(950, 1020))[0]["gate"]  # the control moved 2 %
    assert not summarize(samples(950, 1005, "bb"))[0]["gate"]  # the output changed
    assert summarize(samples(970, 1000))[0]["gate"]  # exactly 3.0 % passes
    # one noisy round does not move the median
    noisy = samples(950, 1000)
    noisy["head"][2] = {"decode/a": (2000, "aa" * 8)}
    assert summarize(noisy)[0]["gate"]
    out = table(summarize(samples(950, 1005)))
    assert pct(-0.01) == "+0.0 %" and pct(-3.04) == "-3.0 %" and pct(0.5) == "+0.5 %"
    assert "-5.0 %" in out and "+0.5 %" in out and "| Yes |" in out and "Gate" in out and "ratio" not in out.lower()
    print("bench_tier1: selftest passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
