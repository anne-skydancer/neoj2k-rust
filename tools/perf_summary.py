#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
"""The helpers of .github/workflows/perf.yml (doc/bench.md, "Measuring in CI"). Standard library only.

usage: perf_summary.py plan                              the workflow's matrices, from the environment
       perf_summary.py machine                           the runner's CPU model and logical CPU count, one line
       perf_summary.py table --dir DIR [--compiler T] [--oracle T]
                                                         one Markdown table from DIR/rep-*.txt, the output of
                                                         the repetitions of the three suites of
                                                         neo-j2k/tests/small_image_gates.rs
       perf_summary.py selftest

The suites time Rust and the C++ oracle in one process and print, per row, both medians and a ratio column; the
table reads the two medians and prints percent changes only (Rust's time against C++'s, minus is faster).
"""
import argparse
import json
import os
import platform
import re
import statistics
import subprocess
import sys
import tempfile

# The four pairs of the gate (name, base, head): the refs are resolved by the job, which fails when one does not exist.
PAIRS = [
    ("stage3-decoder", "f07feb7", "33768eb"),  # main after PR #8 (the decoder's Tier-1 steps)
    ("stage4-encoder", "33768eb", "9052823"),  # main after PR #7 (the encoder's Tier-1 steps)
    ("e3-only", "06f1b39^", "06f1b39"),  # the refinement pass as a list of (context, bit) pairs
    ("sigma-chi-only", "213ff55", "0466458"),  # the sign stays out of the significance flag word until read
]
GATE_OS = ["ubuntu-latest", "windows-2025"]
CPU_LEVELS = ["baseline", "x86-64-v3"]
RUSTFLAGS = {"baseline": "", "x86-64-v3": "-C target-cpu=x86-64-v3"}
QUICK_PAIR = "e3-only"
QUICK_BENCH_ARGS = "--min-mcycles 2 --units 1"

NUM = r"\d+(?:\.\d+)?"
# libtest prints "test NAME ... " before the first line a test writes when the output is not captured
PREFIX = re.compile(r"^test \S+ \.\.\. ")
LARGE = re.compile(r"^(\S+\.j2k)\s+(\d+)\s+(%s)\s+(%s)\s+%s\s+%s\s+%s$" % (NUM, NUM, NUM, NUM, NUM))
SMALL = re.compile(r"^(.+?)\s+(%s)\s+(%s)\s+%s$" % (NUM, NUM, NUM))


def plan(env):
    """The workflow's settings from the environment: quick mode (a pull request, or the dispatch input), the rounds,
    the repetitions, the extra bench arguments and the two matrices (as `include` lists)."""
    quick = env.get("EVENT_NAME") == "pull_request" or (env.get("QUICK_IN") or "").strip().lower() == "true"
    text = (env.get("ROUNDS_IN") or "").strip() or "24"
    try:
        rounds = int(text)
    except ValueError:
        raise SystemExit("perf_summary: rounds must be a whole number, not %r" % text)
    if rounds < 1:
        raise SystemExit("perf_summary: rounds must be at least 1")
    if quick:
        rounds = 3
    pairs = [p for p in PAIRS if p[0] == QUICK_PAIR] if quick else PAIRS
    gate = [{"name": name, "base": base, "head": head, "os": os_name, "cpu": cpu,
             "py": "python" if os_name.startswith("windows") else "python3"}
            for name, base, head in pairs for os_name in GATE_OS for cpu in CPU_LEVELS]
    cells = []
    for os_name, compiler in (("ubuntu-26.04", "GCC 15"), ("windows-2025", "MSVC")):
        for cpu in CPU_LEVELS:
            linux = os_name.startswith("ubuntu")
            if linux:
                # the scalar preset is the C++ library built for the baseline x86-64, the default one for x86-64-v3
                preset = "linux-gcc-scalar" if cpu == "baseline" else "linux-gcc"
                oracle = "SCALAR" if cpu == "baseline" else "AVX2"
            else:
                preset = "windows-msvc"
                oracle = "AVX2 (the preset's default; there is no scalar preset on Windows)"
            cells.append({"os": os_name, "cpu": cpu, "rustflags": RUSTFLAGS[cpu], "preset": preset,
                          "compiler": compiler, "oracle": oracle,
                          "py": "python" if not linux else "python3"})
    return {"quick": "true" if quick else "false", "rounds": str(rounds), "reps": "1" if quick else "3",
            "bench_args": QUICK_BENCH_ARGS if quick else "",
            "gate": json.dumps({"include": gate}), "cells": json.dumps({"include": cells})}


def cpu_name():
    """The processor's name where the system gives it, else Python's."""
    try:
        if os.name == "nt":
            out = subprocess.run(["powershell", "-NoProfile", "-Command",
                                  "(Get-CimInstance Win32_Processor | Select-Object -First 1).Name"],
                                 capture_output=True, text=True, timeout=30).stdout.strip()
        else:
            with open("/proc/cpuinfo", encoding="utf-8") as f:
                out = next((l.split(":", 1)[1].strip() for l in f if l.startswith("model name")), "")
        if out:
            return out
    except (OSError, subprocess.SubprocessError):
        pass
    return platform.processor() or platform.machine()


def logical_cpus():
    """The logical CPUs this process may run on (nproc)."""
    try:
        return len(os.sched_getaffinity(0))
    except (AttributeError, OSError):
        return os.cpu_count() or 0


def parse_run(text):
    """[(key, unit, rust, cpp)] from one run's output, in the order printed. A key is (suite, request, workers); the
    unit is "ms" for the large decodes and "us" for the rest. Lines that are not rows are skipped."""
    rows = []
    for raw in text.splitlines():
        line = PREFIX.sub("", raw.strip())
        m = LARGE.match(line)
        if m:
            rows.append((("large decode", m.group(1), int(m.group(2))), "ms", float(m.group(3)), float(m.group(4))))
            continue
        m = SMALL.match(line)
        if not m:
            continue
        label = m.group(1).strip()
        if " / reduce " in label:
            suite = "small decode"
        elif label.startswith("encode "):
            suite = "small encode"
        else:
            continue
        rows.append(((suite, label, 0), "us", float(m.group(2)), float(m.group(3))))
    return rows


def change(rust, cpp):
    """Rust's time against C++'s in percent: minus is faster."""
    return (rust / cpp - 1.0) * 100.0


def pct(value):
    """A signed percent with one decimal; a change that rounds to zero has no minus sign."""
    value = 0.0 if abs(value) < 0.05 else value
    return "%+.1f %%" % value


def aggregate(runs):
    """Rows from the repetitions (each the result of parse_run): the median of Rust's and of C++'s times, the median
    of the repetitions' changes with the lowest and the highest, and whether Rust is at least as fast (the median change,
    rounded to one decimal as shown, is zero or less). Refuses a repetition with no rows or with other rows."""
    if not runs:
        raise SystemExit("perf_summary: no repetition to read")
    keys = [row[0] for row in runs[0]]
    for number, run in enumerate(runs, 1):
        if not run:
            raise SystemExit("perf_summary: repetition %d has no result rows" % number)
        if [row[0] for row in run] != keys:
            raise SystemExit("perf_summary: repetition %d has other rows than the first" % number)
    out = []
    for index, key in enumerate(keys):
        unit = runs[0][index][1]
        rust = [run[index][2] for run in runs]
        cpp = [run[index][3] for run in runs]
        if min(cpp) <= 0.0:
            raise SystemExit("perf_summary: a C++ time of 0 in %r" % (key,))
        changes = [change(r, c) for r, c in zip(rust, cpp)]
        median = statistics.median(changes)
        shown = 0.0 if abs(median) < 0.05 else round(median, 1)
        out.append({"key": key, "unit": unit, "rust": statistics.median(rust), "cpp": statistics.median(cpp),
                    "change": median, "low": min(changes), "high": max(changes), "faster": shown <= 0.0})
    return out


def yes_no(flag):
    return "Yes" if flag else "No"


def number(value, unit):
    return ("%.3f" if unit == "ms" else "%.2f") % value


def table(rows):
    """The one table in Markdown, and its last line: how many rows have Rust at least as fast as C++."""
    head = ["Suite", "Request", "Workers", "Unit", "Rust", "C++", "Change (Rust against C++)", "Spread of the repetitions",
            "Rust faster"]
    lines = ["| " + " | ".join(head) + " |", "|---|---|---:|---|---:|---:|---:|---|:---:|"]
    for r in rows:
        suite, request, workers = r["key"]
        lines.append("| %s | %s | %s | %s | %s | %s | %s | %s to %s | %s |" % (
            suite, request, workers if workers else "-", "ms" if r["unit"] == "ms" else "us",
            number(r["rust"], r["unit"]), number(r["cpp"], r["unit"]), pct(r["change"]),
            pct(r["low"]).replace(" %", ""), pct(r["high"]), yes_no(r["faster"])))
    passed = sum(1 for r in rows if r["faster"])
    lines.append("")
    lines.append("Rows where Rust is at least as fast as C++: %d of %d" % (passed, len(rows)))
    return "\n".join(lines)


def read_reps(folder):
    """The repetitions of a folder: rep-*.txt in name order, each parsed."""
    names = sorted(n for n in os.listdir(folder) if re.fullmatch(r"rep-\d+\.txt", n))
    if not names:
        raise SystemExit("perf_summary: no rep-N.txt in " + folder)
    runs = []
    for name in names:
        with open(os.path.join(folder, name), encoding="utf-8", errors="replace") as f:
            runs.append(parse_run(f.read()))
    return runs


def table_command(args):
    runs = read_reps(args.dir)
    rows = aggregate(runs)
    notes = ["%d repetitions of each suite in this job; Rust and the C++ oracle are timed in one process, alternately; "
             "the figure of a row is the median of the repetitions, the spread their lowest and highest change." % len(runs),
             "Change is Rust's time against C++'s: minus is faster. Times are medians of the suites' own rounds "
             "(ms for the large decodes, us for the rest)."]
    if args.compiler:
        notes.append("C++ side: the neo-j2k oracle (v0.1.3, RelWithDebInfo) built with %s%s. On Linux it is GCC 15, on "
                     "Windows MSVC, the compiler the C++ performance numbers were measured with." % (
                         args.compiler, ", SIMD %s" % args.oracle if args.oracle else ""))
    print("\n\n".join(notes) + "\n")
    print(table(rows))
    return 0


def selftest():
    large = ("test large_image_executor_latency ... Both codecs use caller-owned typed I32 planes and the same persistent "
             "pool. Width includes the caller.\n"
             "Fixture                    Width   Rust ms    C++ ms Rust/C++   Rust ×   C++ ×\n"
             "large_1024_rgb_97.j2k          1     7.491     7.858    0.953     1.00     1.00\n"
             "large_1024_rgb_97.j2k          2     4.452     4.887    0.911     1.68     1.61\n")
    small = ("Decode uses caller-owned native-endian planar I32 on both sides.\n"
             "Request                                          Rust µs    C++ µs Rust/C++\n"
             "sl_32_rgba_53.j2k / reduce 0 / whole              345.32    284.95    1.212\n"
             "sl_64_rgb_97.j2k / reduce 2 / missing EOC          74.47     90.83    0.820\n")
    enc = ("test small_encoder_median_latency ... Both sides create an encoder per call. The C++ shim copies input.\n"
           "Request                                          Rust µs    C++ µs Rust/C++\n"
           "encode 32x32 RGB / lossless 5/3                   265.09    174.08    1.523\n"
           "encode 64x64 RGB / lossy 9/7                     1080.98    742.16    1.457\n"
           "test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.34s\n")
    text = large + small + enc
    rows = parse_run(text)
    assert [r[0] for r in rows] == [
        ("large decode", "large_1024_rgb_97.j2k", 1), ("large decode", "large_1024_rgb_97.j2k", 2),
        ("small decode", "sl_32_rgba_53.j2k / reduce 0 / whole", 0),
        ("small decode", "sl_64_rgb_97.j2k / reduce 2 / missing EOC", 0),
        ("small encode", "encode 32x32 RGB / lossless 5/3", 0), ("small encode", "encode 64x64 RGB / lossy 9/7", 0)], rows
    assert rows[0][1:] == ("ms", 7.491, 7.858) and rows[2][1:] == ("us", 345.32, 284.95), rows
    assert abs(change(96.0, 100.0) + 4.0) < 1e-9
    assert pct(-0.01) == "+0.0 %" and pct(-5.14) == "-5.1 %" and pct(21.2) == "+21.2 %"
    # three repetitions: medians, the median of the changes, the spread, the Yes/No column
    reps = []
    for rust_scale in (0.9, 1.0, 1.1):
        reps.append([(key, unit, rust * rust_scale, cpp) for key, unit, rust, cpp in rows])
    agg = aggregate(reps)
    first = agg[0]
    assert abs(first["rust"] - 7.491) < 1e-9 and abs(first["cpp"] - 7.858) < 1e-9
    assert abs(first["change"] - change(7.491, 7.858)) < 1e-9 and first["faster"]
    assert abs(first["low"] - change(7.491 * 0.9, 7.858)) < 1e-9 and abs(first["high"] - change(7.491 * 1.1, 7.858)) < 1e-9
    assert [r["faster"] for r in agg] == [True, True, False, True, False, False]
    out = table(agg)
    assert out.splitlines()[-1] == "Rows where Rust is at least as fast as C++: 3 of 6", out.splitlines()[-1]
    assert "| Yes |" in out and "| No |" in out and "ratio" not in out.lower() and "Rust/C++" not in out
    assert "| large decode | large_1024_rgb_97.j2k | 1 | ms | 7.491 | 7.858 |" in out
    assert "| small encode | encode 32x32 RGB / lossless 5/3 | - | us | 265.09 | 174.08 |" in out
    # equal times are at least as fast; a change that rounds to zero from above is not a loss
    assert aggregate([[(("s", "a", 0), "us", 100.0, 100.0)]])[0]["faster"]
    assert aggregate([[(("s", "a", 0), "us", 100.04, 100.0)]])[0]["faster"]
    assert not aggregate([[(("s", "a", 0), "us", 100.1, 100.0)]])[0]["faster"]
    # an empty repetition, other rows, or no repetition is an error and never a table
    for bad in ([], [[]], [rows, rows[:2]], [rows, []]):
        try:
            aggregate(bad)
        except SystemExit:
            pass
        else:
            raise AssertionError("a bad run was summarised")
    # the folder reader takes rep-N.txt in order and nothing else
    with tempfile.TemporaryDirectory() as tmp:
        for name, body in (("rep-1.txt", text), ("rep-2.txt", text), ("notes.txt", "x")):
            with open(os.path.join(tmp, name), "w", encoding="utf-8") as f:
                f.write(body)
        assert len(read_reps(tmp)) == 2 and len(aggregate(read_reps(tmp))) == 6
    # the plan: a pull request is quick (one pair, three rounds, one repetition); a dispatch is the whole matrix
    quick = plan({"EVENT_NAME": "pull_request", "QUICK_IN": "", "ROUNDS_IN": ""})
    gate = json.loads(quick["gate"])["include"]
    assert quick["quick"] == "true" and quick["rounds"] == "3" and quick["reps"] == "1" and len(gate) == 4
    assert {g["name"] for g in gate} == {QUICK_PAIR} and quick["bench_args"] == QUICK_BENCH_ARGS
    full = plan({"EVENT_NAME": "workflow_dispatch", "QUICK_IN": "false", "ROUNDS_IN": "24"})
    gate = json.loads(full["gate"])["include"]
    cells = json.loads(full["cells"])["include"]
    assert full["quick"] == "false" and full["rounds"] == "24" and full["reps"] == "3" and full["bench_args"] == ""
    assert len(gate) == 16 and len(cells) == 4 and len({(g["name"], g["os"], g["cpu"]) for g in gate}) == 16
    assert all(g["py"] == ("python" if g["os"] == "windows-2025" else "python3") for g in gate)
    assert plan({"EVENT_NAME": "workflow_dispatch", "QUICK_IN": "true", "ROUNDS_IN": "24"})["rounds"] == "3"
    assert plan({"EVENT_NAME": "workflow_dispatch", "QUICK_IN": "", "ROUNDS_IN": "12"})["rounds"] == "12"
    for bad in ("x", "0"):
        try:
            plan({"EVENT_NAME": "workflow_dispatch", "ROUNDS_IN": bad})
        except SystemExit:
            pass
        else:
            raise AssertionError("rounds %r accepted" % bad)
    print("perf_summary: selftest passed")
    return 0


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="command", required=True)
    sub.add_parser("plan")
    sub.add_parser("machine")
    sub.add_parser("selftest")
    t = sub.add_parser("table")
    t.add_argument("--dir", required=True)
    t.add_argument("--compiler")
    t.add_argument("--oracle")
    args = ap.parse_args()
    if args.command == "selftest":
        return selftest()
    if args.command == "machine":
        print("%s, %d logical CPUs" % (cpu_name(), logical_cpus()))
        return 0
    if args.command == "plan":
        result = plan(os.environ)
        path = os.environ.get("GITHUB_OUTPUT")
        text = "".join("%s=%s\n" % (k, v) for k, v in result.items())
        if path:
            with open(path, "a", encoding="utf-8") as f:
                f.write(text)
        print(text, end="")
        return 0
    return table_command(args)


if __name__ == "__main__":
    sys.exit(main())
