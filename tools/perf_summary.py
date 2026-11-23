#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
"""The helpers of .github/workflows/perf.yml (doc/bench.md, "Measuring in CI"). Standard library only.

usage: perf_summary.py plan                              the workflow's matrices, from the environment
       perf_summary.py machine                           the runner's CPU model and logical CPU count, one line
       perf_summary.py table --dir DIR [--compiler T] [--oracle T] [--like-for-like yes|no] [--cpus N]
                                                         one Markdown table from DIR/rep-*.txt, the output of
                                                         the repetitions of the three suites of
                                                         neo-j2k/tests/small_image_gates.rs; exit 1 (and a
                                                         message in place of the table) unless every repetition
                                                         is complete
       perf_summary.py pair-verdict --file F --side decode|encode
                                                         the step gate of a pair, from the driver's table
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

# The four pairs of the gate (name, base, head, side): the refs are resolved by the job, which fails when one does not
# exist; the side is the half of the benchmark's sets the step changes (decode/* or encode/*).
PAIRS = [
    ("stage3-decoder", "f07feb7", "33768eb", "decode"),  # main after PR #8 (the decoder's Tier-1 steps)
    ("stage4-encoder", "33768eb", "9052823", "encode"),  # main after PR #7 (the encoder's Tier-1 steps)
    ("e3-only", "06f1b39^", "06f1b39", "encode"),  # the refinement pass as a list of (context, bit) pairs
    ("sigma-chi-only", "213ff55", "0466458", "decode"),  # the sign stays out of the significance flag word until read
]
GATE_OS = ["ubuntu-latest", "windows-2025"]
CPU_LEVELS = ["baseline", "x86-64-v3"]
RUSTFLAGS = {"baseline": "", "x86-64-v3": "-C target-cpu=x86-64-v3"}
QUICK_PAIR = "e3-only"
QUICK_BENCH_ARGS = "--min-mcycles 2 --units 1"
MIN_GAIN = 3.0  # percent faster, the driver's gate (tools/bench_tier1.py)
CONTROL_BAND = 1.0  # percent
NOISE_BAND = 1.0  # percent: a change below this is within noise whatever the spread
# the rows of one complete repetition of the three suites: the large decodes (2 images x widths 1, 2, 4, 8), the small
# decodes (3 images x 3 requests) and the small encodes (3 sizes x 2 modes)
EXPECTED_ROWS = {"large decode": 8, "small decode": 9, "small encode": 6}
SUITE_ORDER = {"large decode": 0, "small decode": 1, "small encode": 2}  # decodes, then encodes, as the baseline tables

NUM = r"\d+(?:\.\d+)?"
# libtest prints "test NAME ... " before the first line a test writes when the output is not captured
PREFIX = re.compile(r"^test \S+ \.\.\. ")
LARGE = re.compile(r"^(\S+\.j2k)\s+(\d+)\s+(%s)\s+(%s)\s+%s\s+%s\s+%s$" % (NUM, NUM, NUM, NUM, NUM))
SMALL = re.compile(r"^(.+?)\s+(%s)\s+(%s)\s+%s$" % (NUM, NUM, NUM))
TEST_OK = re.compile(r"^test result: ok\. 3 passed;", re.M)


class Incomplete(Exception):
    """A repetition that is not the whole of the three suites."""


def plan(env):
    """The workflow's settings from the environment: quick mode (a pull request, or the dispatch input), the rounds,
    the repetitions, the extra bench arguments and the two matrices (as `include` lists)."""
    quick = env.get("EVENT_NAME") == "pull_request" or (env.get("QUICK_IN") or "").strip().lower() == "true"
    text = (env.get("ROUNDS_IN") or "").strip() or "24"
    if quick:
        rounds = 3
    else:
        if not (text.isascii() and text.isdigit()):
            raise SystemExit("perf_summary: rounds must be digits only, not %r" % text)
        rounds = int(text)
        if rounds % 6 != 0 or not 6 <= rounds <= 120:
            raise SystemExit("perf_summary: rounds must be a multiple of 6 from 6 to 120 (each of the six orders "
                             "the builds run in is taken equally often), not %d" % rounds)
    pairs = [p for p in PAIRS if p[0] == QUICK_PAIR] if quick else PAIRS
    gate = [{"name": name, "base": base, "head": head, "side": side, "os": os_name, "cpu": cpu,
             "py": "python" if os_name.startswith("windows") else "python3"}
            for name, base, head, side in pairs for os_name in GATE_OS for cpu in CPU_LEVELS]
    cells = []
    for os_name, compiler in (("ubuntu-26.04", "GCC 15"), ("windows-2025", "MSVC")):
        for cpu in CPU_LEVELS:
            linux = os_name.startswith("ubuntu")
            if linux:
                # the scalar preset is the C++ library built for the baseline x86-64, the default one for x86-64-v3
                preset = "linux-gcc-scalar" if cpu == "baseline" else "linux-gcc"
                oracle = "scalar" if cpu == "baseline" else "AVX2"
            else:
                preset = "windows-msvc"
                oracle = "AVX2"  # there is no scalar preset on Windows
            # the Windows baseline cell times a baseline-CPU Rust build against an AVX2 oracle
            like = "no" if (not linux and cpu == "baseline") else "yes"
            cells.append({"os": os_name, "cpu": cpu, "rustflags": RUSTFLAGS[cpu], "preset": preset,
                          "compiler": compiler, "oracle": oracle, "like": like,
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


def check_complete(text, number):
    """The rows of repetition `number`, or Incomplete: the three tests passed (libtest's result line) and every suite
    has all of its rows, every time of Rust and of C++ a positive number."""
    if not TEST_OK.search(text):
        raise Incomplete("repetition %d has no line 'test result: ok. 3 passed' (a test failed, panicked or did not "
                         "finish)" % number)
    rows = parse_run(text)
    counts = {suite: sum(1 for r in rows if r[0][0] == suite) for suite in EXPECTED_ROWS}
    if counts != EXPECTED_ROWS:
        raise Incomplete("repetition %d has rows %s, expected %s" % (
            number, ", ".join("%s %d" % kv for kv in counts.items()), ", ".join("%s %d" % kv for kv in EXPECTED_ROWS.items())))
    for key, _unit, rust, cpp in rows:
        if not (rust > 0.0 and cpp > 0.0):
            raise Incomplete("repetition %d has a time of 0 in %r" % (number, key))
    return rows


def change(rust, cpp):
    """Rust's time against C++'s in percent: minus is faster."""
    return (rust / cpp - 1.0) * 100.0


def pct(value):
    """A signed percent with one decimal; a change that rounds to zero has no minus sign."""
    value = 0.0 if abs(value) < 0.05 else value
    return "%+.1f %%" % value


def verdict(median, low, high):
    """"Yes" (Rust faster), "No" (slower) or "Within noise": the median change is below zero and the repetitions'
    changes do not straddle zero, or above zero and not straddling; a median under NOISE_BAND percent, or a spread that
    straddles zero, is within noise."""
    if abs(median) < NOISE_BAND or low <= 0.0 <= high:
        return "Within noise"
    return "Yes" if median < 0.0 else "No"


def aggregate(runs, cpus=None):
    """Rows from the repetitions (each the result of parse_run): the median of Rust's and of C++'s times, the median
    of the repetitions' changes with the lowest and the highest, the verdict, and whether the row is oversubscribed (a
    large decode with more workers than `cpus` logical CPUs). Refuses a repetition with no rows or with other rows."""
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
        low, high = min(changes), max(changes)
        out.append({"key": key, "unit": unit, "rust": statistics.median(rust), "cpp": statistics.median(cpp),
                    "change": median, "low": low, "high": high, "verdict": verdict(median, low, high),
                    "over": bool(cpus) and key[0] == "large decode" and key[2] > cpus})
    return out


def number(value, unit):
    return ("%.3f" if unit == "ms" else "%.2f") % value


def counts(rows):
    """(faster, within noise, slower) over the rows that are not oversubscribed."""
    kept = [r["verdict"] for r in rows if not r["over"]]
    return kept.count("Yes"), kept.count("Within noise"), kept.count("No")


def table(rows, like_for_like=True, oracle=None, cpus=None):
    """The one table in Markdown, and the lines under it: how many rows have Rust faster, within noise and slower, the
    rows left out because they run more workers than the runner has CPUs, and, unless the C++ oracle is not the
    like-for-like one, how many have Rust at least as fast."""
    rows = sorted(rows, key=lambda r: SUITE_ORDER[r["key"][0]])  # stable: the order printed within a suite
    head = ["Suite", "Request", "Workers", "Unit", "Rust", "C++", "Change (Rust against C++)", "Spread of the repetitions",
            "Rust faster"]
    lines = ["| " + " | ".join(head) + " |", "|---|---|---:|---|---:|---:|---:|---|:---:|"]
    for r in rows:
        suite, request, workers = r["key"]
        lines.append("| %s | %s | %s | %s | %s | %s | %s | %s to %s | %s |" % (
            suite, request, workers if workers else "-", "ms" if r["unit"] == "ms" else "us",
            number(r["rust"], r["unit"]), number(r["cpp"], r["unit"]), pct(r["change"]),
            pct(r["low"]).replace(" %", ""), pct(r["high"]),
            r["verdict"] + (" (oversubscribed)" if r["over"] else "")))
    faster, noise, slower = counts(rows)
    total = faster + noise + slower
    lines.append("")
    over = [r for r in rows if r["over"]]
    if over:
        lines.append("Not counted (more workers than the runner's %s logical CPUs; the width includes the caller): %s" % (
            cpus, "; ".join("%s, width %d" % (r["key"][1], r["key"][2]) for r in over)))
        lines.append("")
    if like_for_like:
        lines.append("Rust faster in %d rows, within noise in %d, slower in %d, of %d" % (faster, noise, slower, total))
        lines.append("")
        lines.append("Rows where Rust is at least as fast as C++ (faster + within noise): %d of %d" % (faster + noise, total))
    else:
        lines.append("Rows where Rust is at least as fast as C++: not like for like (C++ oracle %s)" % (oracle or "AVX2"))
    return "\n".join(lines)


def read_reps(folder):
    """The repetitions of a folder: rep-*.txt in name order, each checked complete and parsed."""
    names = sorted(n for n in os.listdir(folder) if re.fullmatch(r"rep-\d+\.txt", n))
    if not names:
        raise Incomplete("no rep-N.txt in " + folder)
    runs = []
    for number, name in enumerate(names, 1):
        with open(os.path.join(folder, name), encoding="utf-8", errors="replace") as f:
            runs.append(check_complete(f.read(), number))
    return runs


def table_command(args):
    try:
        runs = read_reps(args.dir)
    except Incomplete as e:
        print("**The Rust-versus-C++ result is incomplete: %s.** No table is shown; see the job log and the artifact." % e)
        return 1
    cpus = args.cpus if args.cpus is not None else logical_cpus()
    rows = aggregate(runs, cpus)
    like = args.like_for_like != "no"
    notes = ["%d repetitions of each suite in this job (rows read in each: %s); Rust and the C++ oracle are timed in one "
             "process, alternately." % (len(runs), ", ".join("%s %d" % kv for kv in EXPECTED_ROWS.items())),
             "Change is Rust's time against C++'s, minus is faster: the median of the repetitions' own changes (not the "
             "percent of the two median columns), with the lowest and the highest of them as the spread. Times are medians "
             "of the suites' own rounds (ms for the large decodes, us for the rest). Rust faster: Yes when the median "
             "change is below zero and the spread does not straddle zero, No when it is above zero and does not "
             "straddle it, Within noise when the spread straddles zero or the median is under %.1f %%." % NOISE_BAND]
    if args.compiler:
        if args.compiler.startswith("MSVC"):
            why = "MSVC is the compiler the C++ performance numbers of the baseline were measured with."
        else:
            why = "GCC 15 is not the compiler the C++ performance numbers of the baseline were measured with (MSVC)."
        notes.append("C++ side: the neo-j2k oracle (v0.1.3, RelWithDebInfo) built with %s%s. %s" % (
            args.compiler, ", SIMD %s" % args.oracle if args.oracle else "", why))
    if not like:
        notes.append("This cell is not like for like: the Rust build is for the baseline x86-64 and the C++ oracle is "
                     "AVX2 (Windows has no scalar oracle preset); the table is shown, its counts are not.")
    print("\n\n".join(notes) + "\n")
    print(table(rows, like, args.oracle, cpus))
    return 0


def parse_pair_table(text):
    """[{set, head, control, identical, gate}] from the driver's table text (one per set), percent figures as floats."""
    rows = []
    for line in text.splitlines():
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) != 8 or not re.match(r"^(decode|encode)/", cells[0]):
            continue
        head = re.match(r"^([+-]\d+\.\d+) %$", cells[2])
        control = re.match(r"^([+-]\d+\.\d+) %$", cells[4])
        if not head or not control or cells[6] not in ("Yes", "No") or cells[7] not in ("Yes", "No"):
            raise SystemExit("perf_summary: cannot read the driver's row %r" % line)
        rows.append({"set": cells[0], "head": float(head.group(1)), "control": float(control.group(1)),
                     "identical": cells[6] == "Yes", "gate": cells[7] == "Yes"})
    return rows


def pair_verdict(text, side):
    """The step gate of a pair: only the sets of the side the step changes (the driver's gate counts all eight, which a
    decoder step can never pass with its encode sets, nor the reverse), and how many control rows strayed."""
    rows = parse_pair_table(text)
    mine = [r for r in rows if r["set"].startswith(side + "/")]
    if not mine:
        raise SystemExit("perf_summary: the driver's table has no %s sets" % side)
    passed = sum(1 for r in mine if r["gate"])
    strayed = sum(1 for r in rows if abs(r["control"]) > CONTROL_BAND)
    other = "encode" if side == "decode" else "decode"
    return [
        "Step gate (%s sets): %s (%d of %d sets at least %.1f %% faster, control within +-%.1f %%, outputs identical)" % (
            side, "Yes" if passed == len(mine) else "No", passed, len(mine), MIN_GAIN, CONTROL_BAND),
        "",
        "Control outside +-%.1f %% in %d of %d sets (a noisy runner for those rows)" % (CONTROL_BAND, strayed, len(rows)),
        "",
        "The %s sets are not judged: this step does not change that code, so their rows show layout and noise." % other,
    ]


GOLDEN = """
running 3 tests
test large_image_executor_latency ... Both codecs use caller-owned typed I32 planes and the same persistent pool. Width includes the caller.
Fixture                    Width   Rust ms    C++ ms Rust/C++   Rust ×   C++ ×
large_1024_rgb_97.j2k          1    13.195    21.519    0.613     1.00     1.00
large_1024_rgb_97.j2k          2     7.867    12.478    0.630     1.68     1.72
large_1024_rgb_97.j2k          4     7.275    11.670    0.623     1.81     1.84
large_1024_rgb_97.j2k          8     7.556    11.722    0.645     1.75     1.84
large_4096_g_97.j2k            1    69.627   190.896    0.365     1.00     1.00
large_4096_g_97.j2k            2    37.371    96.662    0.387     1.86     1.97
large_4096_g_97.j2k            4    30.370    70.726    0.429     2.29     2.70
large_4096_g_97.j2k            8    30.841    72.335    0.426     2.26     2.64
ok
test small_encoder_median_latency ... Both sides create an encoder per call. The C++ shim copies input and encoded output.
Request                                            Rust µs    C++ µs Rust/C++
encode 32x32 RGB / lossless 5/3                     319.39    313.72    1.018
encode 32x32 RGB / lossy 9/7                        377.79    341.81    1.105
encode 64x64 RGB / lossless 5/3                    1310.51   1260.18    1.040
encode 64x64 RGB / lossy 9/7                       1477.15   1339.23    1.103
encode 128x128 RGB / lossless 5/3                  4888.19   4768.50    1.025
encode 128x128 RGB / lossy 9/7                     5435.08   5087.48    1.068
ok
test small_image_median_latency ... Decode uses caller-owned native-endian planar I32 on both sides.
The C++ shim builds plane descriptors per call.
Request                                            Rust µs    C++ µs Rust/C++
sl_32_rgba_53.j2k / reduce 0 / whole                450.20    423.31    1.064
sl_32_rgba_53.j2k / reduce 2 / whole                 40.23     39.00    1.031
sl_32_rgba_53.j2k / reduce 0 / missing EOC          448.36    423.11    1.060
sl_64_rgb_97.j2k / reduce 0 / whole                 160.54    159.31    1.008
sl_64_rgb_97.j2k / reduce 2 / whole                  55.70     50.40    1.105
sl_64_rgb_97.j2k / reduce 0 / missing EOC           158.40    158.81    0.997
sl_128_rgba_97.j2k / reduce 0 / whole               389.44    499.62    0.779
sl_128_rgba_97.j2k / reduce 2 / whole               172.14    148.79    1.157
sl_128_rgba_97.j2k / reduce 0 / missing EOC         394.58    498.49    0.792
ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 24.89s
"""

PAIR_GOLDEN = """neo-j2k Tier-1 benchmark: base 1a5055e, head 06f1b39, control = byte copy of the base
CPU x; pinned, CPU 3, priority raised; baseline build; 3 rounds of the three builds, interleaved

| Set | Base (Mcycles) | Head vs base | Head, rounds min to max | Control vs base | Control, rounds min to max | Output identical | Gate |
|---|---:|---:|---:|---:|---:|:---:|:---:|
| decode/1024_lossless | 784.92 | +1.47 % | -5.37 to +2.08 | +0.27 % | -4.42 to +0.87 | Yes | No |
| decode/1024_lossy | 139.21 | +1.82 % | -6.89 to +3.31 | +1.40 % | -9.93 to +6.43 | Yes | No |
| decode/256_lossless | 41.54 | +0.61 % | -2.84 to +2.45 | +21.12 % | +1.99 to +80.93 | Yes | No |
| decode/256_lossy | 7.13 | +2.39 % | +1.02 to +17.04 | +2.99 % | +1.89 to +59.15 | Yes | No |
| decode/512_lossless | 193.73 | +0.59 % | -1.78 to +1.61 | +4.10 % | -0.02 to +8.22 | Yes | No |
| decode/512_lossy | 34.97 | +2.56 % | +0.92 to +14.43 | +1.30 % | +0.25 to +1.82 | Yes | No |
| encode/lossless | 216.81 | -4.40 % | -18.00 to +2.30 | -0.15 % | -19.49 to +1.93 | Yes | Yes |
| encode/lossy | 158.39 | +1.39 % | +0.48 to +11.92 | +0.92 % | -0.42 to +0.98 | Yes | No |

Gate (head at least 3.0 % faster, control within +-1.0 %, outputs identical): No (1 of 8 sets).
"""


def selftest():
    rows = parse_run(GOLDEN)
    assert len(rows) == 23 and rows[0] == (("large decode", "large_1024_rgb_97.j2k", 1), "ms", 13.195, 21.519), rows[0]
    assert rows[8][0] == ("small encode", "encode 32x32 RGB / lossless 5/3", 0), rows[8]
    assert rows[14][0] == ("small decode", "sl_32_rgba_53.j2k / reduce 0 / whole", 0) and rows[14][1] == "us", rows[14]
    assert len(check_complete(GOLDEN, 1)) == 23
    assert abs(change(96.0, 100.0) + 4.0) < 1e-9
    assert pct(-0.01) == "+0.0 %" and pct(-5.14) == "-5.1 %" and pct(21.2) == "+21.2 %"
    # the verdict: three states
    assert verdict(-5.0, -6.0, -4.0) == "Yes" and verdict(5.0, 4.0, 6.0) == "No"
    assert verdict(-5.0, -6.0, 1.0) == "Within noise" and verdict(0.9, 0.5, 1.2) == "Within noise"
    assert verdict(-0.9, -1.2, -0.5) == "Within noise" and verdict(1.0, 1.0, 1.0) == "No"
    # a complete run: three identical repetitions, four CPUs (the two width-8 rows are left out of the counts)
    agg = aggregate([rows, rows, rows], 4)
    assert [r["over"] for r in agg].count(True) == 2 and all(r["key"][2] == 8 for r in agg if r["over"])
    assert counts(agg) == (8, 2, 11), counts(agg)
    out = table(agg, True, "scalar", 4)
    lines = out.splitlines()
    assert lines[-1] == "Rows where Rust is at least as fast as C++ (faster + within noise): 10 of 21", lines[-1]
    assert "Rust faster in 8 rows, within noise in 2, slower in 11, of 21" in out
    assert "Not counted (more workers than the runner's 4 logical CPUs" in out and "large_1024_rgb_97.j2k, width 8" in out
    assert "| Yes (oversubscribed) |" in out and "| Within noise |" in out and "| No |" in out
    assert "| small encode | encode 32x32 RGB / lossless 5/3 | - | us | 319.39 | 313.72 | +1.8 % | +1.8 to +1.8 % | No |" in out
    assert "ratio" not in out.lower() and "Rust/C++" not in out
    order = [l.split("|")[1].strip() for l in lines if l.startswith("| ") and not l.startswith("| Suite")]
    assert order == sorted(order, key=lambda s: SUITE_ORDER[s]), order
    # not like for like: the table, no counts
    unlike = table(agg, False, "AVX2", 4)
    assert "Rows where Rust is at least as fast as C++: not like for like (C++ oracle AVX2)" in unlike
    assert "Rust faster in" not in unlike
    # spread: three repetitions that straddle zero are within noise whatever the median
    reps = [[(key, unit, rust * scale, cpp) for key, unit, rust, cpp in rows] for scale in (0.5, 1.0, 1.9)]
    assert aggregate(reps)[0]["verdict"] == "Within noise" and aggregate(reps)[0]["over"] is False
    # an incomplete repetition is an error and never a table
    for bad, why in (
            (GOLDEN.replace("test result: ok. 3 passed", "test result: FAILED. 2 passed"), "no line"),
            (GOLDEN.split("test result")[0] + "thread 'x' panicked at t.rs:1:1:\nboom\n", "no line"),
            ("\n".join(l for l in GOLDEN.split("\n") if not l.startswith("encode 32x32 RGB / lossless")),
             "small encode 5"),
            (GOLDEN.replace("313.72", "inf"), "small encode 5"),
            (GOLDEN.replace("1.018", "NaN"), "small encode 5"),
            (GOLDEN.replace("319.39", "0.00"), "time of 0"),
            (GOLDEN[:GOLDEN.index("test small_encoder")] + "test result: ok. 3 passed;", "small encode 0"),
            ("", "no line")):
        try:
            check_complete(bad, 1)
        except Incomplete as e:
            assert why in str(e), (why, str(e))
        else:
            raise AssertionError("an incomplete repetition was accepted: " + why)
    # the folder reader: rep-N.txt in order and nothing else; a partial repetition fails the whole
    with tempfile.TemporaryDirectory() as tmp:
        for name, body in (("rep-1.txt", GOLDEN), ("rep-2.txt", GOLDEN), ("notes.txt", "x")):
            with open(os.path.join(tmp, name), "w", encoding="utf-8") as f:
                f.write(body)
        assert len(read_reps(tmp)) == 2
        with open(os.path.join(tmp, "rep-3.txt"), "w", encoding="utf-8") as f:
            f.write(GOLDEN.split("test small_encoder")[0])
        try:
            read_reps(tmp)
        except Incomplete as e:
            assert "repetition 3" in str(e)
        else:
            raise AssertionError("a partial repetition rendered")
        ns = argparse.Namespace(dir=tmp, compiler="MSVC", oracle="AVX2", like_for_like="no", cpus=4)
        assert table_command(ns) == 1  # prints the message, exit 1
    with tempfile.TemporaryDirectory() as tmp:
        with open(os.path.join(tmp, "rep-1.txt"), "w", encoding="utf-8") as f:
            f.write(GOLDEN)
        assert table_command(argparse.Namespace(dir=tmp, compiler="GCC 15", oracle="scalar", like_for_like="yes",
                                                cpus=4)) == 0
    # the pair's step gate: one side only, the control count over all eight
    enc = pair_verdict(PAIR_GOLDEN, "encode")
    assert enc[0] == ("Step gate (encode sets): No (1 of 2 sets at least 3.0 % faster, control within +-1.0 %, "
                      "outputs identical)"), enc[0]
    assert enc[2] == "Control outside +-1.0 % in 5 of 8 sets (a noisy runner for those rows)", enc[2]
    dec = pair_verdict(PAIR_GOLDEN, "decode")
    assert dec[0].startswith("Step gate (decode sets): No (0 of 6 sets"), dec[0]
    allyes = PAIR_GOLDEN.replace("| Yes | No |", "| Yes | Yes |")
    assert pair_verdict(allyes, "decode")[0].startswith("Step gate (decode sets): Yes (6 of 6 sets")
    try:
        pair_verdict("no table", "decode")
    except SystemExit:
        pass
    else:
        raise AssertionError("an empty table was judged")
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
    assert {g["name"]: g["side"] for g in gate} == {"stage3-decoder": "decode", "stage4-encoder": "encode",
                                                    "e3-only": "encode", "sigma-chi-only": "decode"}
    assert [(c["os"], c["cpu"], c["oracle"], c["like"]) for c in cells] == [
        ("ubuntu-26.04", "baseline", "scalar", "yes"), ("ubuntu-26.04", "x86-64-v3", "AVX2", "yes"),
        ("windows-2025", "baseline", "AVX2", "no"), ("windows-2025", "x86-64-v3", "AVX2", "yes")]
    assert plan({"EVENT_NAME": "workflow_dispatch", "QUICK_IN": "true", "ROUNDS_IN": "24"})["rounds"] == "3"
    assert plan({"EVENT_NAME": "workflow_dispatch", "QUICK_IN": "", "ROUNDS_IN": "12"})["rounds"] == "12"
    for bad in ("x", "0", "5", "7", "126", "", "-6", "٣٦", "6.0", "1e1"):
        if bad == "":
            continue  # empty means the default
        try:
            plan({"EVENT_NAME": "workflow_dispatch", "ROUNDS_IN": bad})
        except SystemExit:
            pass
        else:
            raise AssertionError("rounds %r accepted" % bad)
    for good in ("6", "120", "24"):
        assert plan({"EVENT_NAME": "workflow_dispatch", "ROUNDS_IN": good})["rounds"] == good
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
    t.add_argument("--like-for-like", choices=["yes", "no"], default="yes")
    t.add_argument("--cpus", type=int, help="the runner's logical CPUs (default: this process's)")
    v = sub.add_parser("pair-verdict")
    v.add_argument("--file", required=True)
    v.add_argument("--side", required=True, choices=["decode", "encode"])
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
    if args.command == "pair-verdict":
        with open(args.file, encoding="utf-8", errors="replace") as f:
            print("\n".join(pair_verdict(f.read(), args.side)))
        return 0
    return table_command(args)


if __name__ == "__main__":
    sys.exit(main())
