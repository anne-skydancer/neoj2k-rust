#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
"""The helpers of .github/workflows/perf.yml (doc/bench.md, "Measuring in CI"). Standard library only.

usage: perf_summary.py plan                              the workflow's matrices, from the environment
       perf_summary.py machine                           the runner's CPU model and logical CPU count, one line
       perf_summary.py table --dir DIR [--kind cpp|openjpeg|versions] [--compiler T] [--oracle T]
                                       [--like-for-like yes|no] [--cpus N]
                                                         one Markdown table from DIR/rep-*.txt: with --kind cpp
                                                         (the default) the output of the repetitions of the three
                                                         suites of neo-j2k/tests/small_image_gates.rs; with
                                                         --kind openjpeg the three suites of
                                                         neo-j2k/tests/vs_openjpeg.rs (Rust against OpenJPEG);
                                                         with --kind versions the program of
                                                         tools/rust-versions (the crate at tag v0.1.2 against
                                                         this checkout's); exit 1 (and a message in place of the
                                                         table) unless every repetition is complete
       perf_summary.py pair-verdict --file F --side decode|encode
                                                         the step gate of a pair, from the driver's table
       perf_summary.py selftest

The suites time Rust and the C++ oracle in one process and print, per row, both medians and a ratio column; the
table reads the two medians and prints percent changes only (Rust's time against C++'s, minus is faster).

The two other kinds print no ratio at all: per row the first side's median, the second's and `same` or `DIFFERENT` (the
two outputs compared). Their tables are measurements, not gates: a percent change, a Yes/No column and the outputs'
identity per row, with the API each side used stated above the table.
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
    # a dispatch may name one pair of its own (any two refs): the gate then runs that pair only, and the Rust-versus-C++
    # jobs only when asked for
    base = (env.get("PAIR_BASE_IN") or "").strip()
    head = (env.get("PAIR_HEAD_IN") or "").strip()
    if bool(base) != bool(head):
        raise SystemExit("perf_summary: pair_base and pair_head go together")
    custom = bool(base)
    if custom:
        side = (env.get("PAIR_SIDE_IN") or "").strip() or "decode"
        if side not in ("decode", "encode", "image"):
            raise SystemExit("perf_summary: pair_side is decode, encode or image, not %r" % side)
        name = (env.get("PAIR_NAME_IN") or "").strip() or "adhoc-" + side
        if not re.fullmatch(r"[A-Za-z0-9._-]+", name):
            raise SystemExit("perf_summary: pair_name may hold letters, digits, '.', '_' and '-' only, not %r" % name)
        pairs = [(name, base, head, side)]
    gate = [{"name": name, "base": base, "head": head, "side": side, "os": os_name, "cpu": cpu,
             "py": "python" if os_name.startswith("windows") else "python3"}
            for name, base, head, side in pairs for os_name in GATE_OS for cpu in CPU_LEVELS]
    base_cells = []
    for os_name, compiler in (("ubuntu-26.04", "GCC 16"), ("windows-2025", "MSVC")):
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
            base_cells.append({"os": os_name, "cpu": cpu, "rustflags": RUSTFLAGS[cpu], "preset": preset,
                               "compiler": compiler, "oracle": oracle, "like": like,
                               "py": "python" if not linux else "python3"})
    # the two measurements of Rust against others: a pull request smoke-tests both, a dispatch runs the ones it asks
    # for (and then only those, unless it also names a pair of its own or asks for the C++ jobs with vs_cpp)
    pull_request = env.get("EVENT_NAME") == "pull_request"
    vs_openjpeg = pull_request or flag(env, "VS_OPENJPEG_IN")
    vs_v012 = pull_request or flag(env, "VS_V012_IN")
    only_those = (vs_openjpeg or vs_v012) and not pull_request and not custom
    if only_those:
        gate = []
    cells = list(base_cells)
    if (custom or only_those) and not flag(env, "VS_CPP_IN"):
        cells = []
    # Rust against OpenJPEG: the oracle's OpenJPEG is always built with AVX2 (oracle/CMakeLists.txt), so a cell whose
    # Rust build is for the baseline x86-64 is not like for like, on either system
    opj_cells = [dict(c, like="yes" if c["cpu"] != "baseline" else "no") for c in base_cells] if vs_openjpeg else []
    ver_cells = [{k: c[k] for k in ("os", "cpu", "rustflags", "py")} for c in base_cells] if vs_v012 else []
    return {"quick": "true" if quick else "false", "rounds": str(rounds), "reps": "1" if quick else "3",
            "bench_args": QUICK_BENCH_ARGS if quick else "",
            "gate": json.dumps({"include": gate}), "gate_count": str(len(gate)),
            "cells": json.dumps({"include": cells}), "cells_count": str(len(cells)),
            "opj_cells": json.dumps({"include": opj_cells}), "opj_count": str(len(opj_cells)),
            "ver_cells": json.dumps({"include": ver_cells}), "ver_count": str(len(ver_cells))}


def flag(env, name):
    """A boolean input of the dispatch: true only when it is the text `true`."""
    return (env.get(name) or "").strip().lower() == "true"


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
        identical = [run[index][4] for run in runs if len(run[index]) > 4]
        out.append({"key": key, "unit": unit, "rust": statistics.median(rust), "cpp": statistics.median(cpp),
                    "change": median, "low": low, "high": high, "verdict": verdict(median, low, high),
                    "over": bool(cpus) and key[0] == "large decode" and key[2] > cpus,
                    "identical": all(identical) if identical else None})
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


# The output of the two other kinds: per row the first side's median, the second's and `same` or `DIFFERENT`.
PAIR_TEST_OK = re.compile(r"^(?:test|harness) result: ok\. 3 passed;", re.M)
PAIR_LARGE = re.compile(r"^(\S+\.j2k)\s+(\d+)\s+(%s)\s+(%s)\s+(same|DIFFERENT)$" % (NUM, NUM))
PAIR_SMALL = re.compile(r"^(.+?)\s+(%s)\s+(%s)\s+(same|DIFFERENT)$" % (NUM, NUM))


def parse_pair_run(text):
    """[(key, unit, first, second, identical)] from one run of the other two kinds, in the order printed."""
    rows = []
    for raw in text.splitlines():
        line = PREFIX.sub("", raw.strip())
        m = PAIR_LARGE.match(line)
        if m:
            rows.append((("large decode", m.group(1), int(m.group(2))), "ms", float(m.group(3)), float(m.group(4)),
                         m.group(5) == "same"))
            continue
        m = PAIR_SMALL.match(line)
        if not m:
            continue
        label = m.group(1).strip()
        if " / reduce " in label:
            suite = "small decode"
        elif label.startswith("encode "):
            suite = "small encode"
        else:
            continue
        rows.append(((suite, label, 0), "us", float(m.group(2)), float(m.group(3)), m.group(4) == "same"))
    return rows


def check_complete_pair(text, number):
    """The rows of repetition `number` of the other two kinds, or Incomplete: the run finished (libtest's result line
    or the program's), every suite has all of its rows, every time is a positive number."""
    if not PAIR_TEST_OK.search(text):
        raise Incomplete("repetition %d has no result line 'ok. 3 passed' (a test failed, panicked or did not "
                         "finish)" % number)
    rows = parse_pair_run(text)
    counts_ = {suite: sum(1 for r in rows if r[0][0] == suite) for suite in EXPECTED_ROWS}
    if counts_ != EXPECTED_ROWS:
        raise Incomplete("repetition %d has rows %s, expected %s" % (
            number, ", ".join("%s %d" % kv for kv in counts_.items()), ", ".join("%s %d" % kv for kv in EXPECTED_ROWS.items())))
    for key, _unit, first, second, _same in rows:
        if not (first > 0.0 and second > 0.0):
            raise Incomplete("repetition %d has a time of 0 in %r" % (number, key))
    return rows


def read_reps(folder, check=None):
    """The repetitions of a folder: rep-*.txt in name order, each checked complete and parsed."""
    names = sorted(n for n in os.listdir(folder) if re.fullmatch(r"rep-\d+\.txt", n))
    if not names:
        raise Incomplete("no rep-N.txt in " + folder)
    runs = []
    for number, name in enumerate(names, 1):
        with open(os.path.join(folder, name), encoding="utf-8", errors="replace") as f:
            runs.append((check or check_complete)(f.read(), number))
    return runs


# What differs between the kinds of table: the two sides' names, the heading, and the API each side used per suite.
KINDS = {
    "openjpeg": {
        "title": "Rust against OpenJPEG",
        "first": "Rust", "second": "OpenJPEG", "change": "Change (Rust against OpenJPEG)", "yes": "Rust faster",
        "apis": [
            ("large decode", "one Decoder kept across calls, decode_into_i32 into caller-owned planes; threads are the "
                             "host executor of the Rust API (the test harness's persistent pool; the width counts the "
                             "caller)",
             "opj_decode: a codec and a memory stream made and destroyed in every call, the sample planes allocated by "
             "OpenJPEG in every call and not copied out; width 1 has no threads, width N asks "
             "opj_codec_set_threads(N), a pool of N workers made in every call"),
            ("small decode", "one Decoder kept across calls, decode_into_i32 into caller-owned planes, no allocation "
                             "per call",
             "opj_decode: a codec and a memory stream made and destroyed in every call, the sample planes allocated by "
             "OpenJPEG in every call and not copied out, no threads"),
            ("small encode", "Encoder::new().encode on the borrowed interleaved pixels, one encoder per call",
             "opj_image_create, the pixels copied into its int32 planes, opj_encode into a growing memory buffer, one "
             "codec per call, one layer, no threads"),
        ],
    },
    "versions": {
        "title": "Rust 0.1.3 against Rust 0.1.2",
        "first": "0.1.3", "second": "0.1.2", "change": "Change (0.1.3 against 0.1.2)", "yes": "0.1.3 faster",
        "apis": [
            ("large decode", "both versions: one Decoder kept across calls, decode_into_i32 into caller-owned planes; "
                             "threads are the harness's own scheduler (scoped threads, width - 1 started per call, the "
                             "width counts the caller), the same for both",
             "the same code, built against the crate at tag v0.1.2"),
            ("small decode", "both versions: one Decoder kept across calls, decode_into_i32 into caller-owned planes",
             "the same code, built against the crate at tag v0.1.2"),
            ("small encode", "both versions: Encoder::new().encode on the borrowed interleaved pixels, one encoder "
                             "per call",
             "the same code, built against the crate at tag v0.1.2"),
        ],
    },
}


def pair_table(rows, kind, like_for_like=True, cpus=None):
    """The one table of the other two kinds in Markdown, and the lines under it: how many rows have the first side
    faster, within noise and slower (rows with more workers than CPUs left out), and how many rows have the two
    outputs different."""
    k = KINDS[kind]
    rows = sorted(rows, key=lambda r: SUITE_ORDER[r["key"][0]])
    head = ["Suite", "Request", "Workers", "Unit", k["first"], k["second"], k["change"], "Spread of the repetitions",
            k["yes"], "Output identical"]
    lines = ["| " + " | ".join(head) + " |", "|---|---|---:|---|---:|---:|---:|---|:---:|:---:|"]
    for r in rows:
        suite, request, workers = r["key"]
        lines.append("| %s | %s | %s | %s | %s | %s | %s | %s to %s | %s | %s |" % (
            suite, request, workers if workers else "-", "ms" if r["unit"] == "ms" else "us",
            number(r["rust"], r["unit"]), number(r["cpp"], r["unit"]), pct(r["change"]),
            pct(r["low"]).replace(" %", ""), pct(r["high"]),
            r["verdict"] + (" (oversubscribed)" if r["over"] else ""),
            "Yes" if r["identical"] else "No"))
    faster, noise, slower = counts(rows)
    lines.append("")
    over = [r for r in rows if r["over"]]
    if over:
        lines.append("Not counted (more workers than the runner's %s logical CPUs; the width includes the caller): %s" % (
            cpus, "; ".join("%s, width %d" % (r["key"][1], r["key"][2]) for r in over)))
        lines.append("")
    different = [r for r in rows if not r["identical"]]
    lines.append("%s faster in %d rows, within noise in %d, slower in %d, of %d%s" % (
        k["first"], faster, noise, slower, faster + noise + slower,
        "" if like_for_like else " (counts shown for reading only: this cell is not like for like)"))
    lines.append("")
    if different:
        lines.append("Outputs that are NOT identical (%d of %d rows): %s" % (
            len(different), len(rows), "; ".join("%s%s" % (r["key"][1], ", width %d" % r["key"][2] if r["key"][2] else "")
                                                for r in different)))
    else:
        lines.append("Outputs identical in all %d rows (decoded samples, encoded bytes)." % len(rows))
    return "\n".join(lines)


def pair_command(args):
    kind = args.kind
    k = KINDS[kind]
    try:
        runs = read_reps(args.dir, check_complete_pair)
    except Incomplete as e:
        print("**The result of %s is incomplete: %s.** No table is shown; see the job log and the artifact." % (
            k["title"], e))
        return 1
    cpus = args.cpus if args.cpus is not None else logical_cpus()
    rows = aggregate(runs, cpus)
    like = args.like_for_like != "no"
    notes = ["%d repetitions of each suite in this job (rows read in each: %s); the two sides are timed in one process, "
             "alternately. This is a measurement, with no pass or fail gate." % (
                 len(runs), ", ".join("%s %d" % kv for kv in EXPECTED_ROWS.items())),
             "%s is the first side's time against the second's, minus is faster: the median of the repetitions' own "
             "changes (not the percent of the two median columns), with the lowest and the highest of them as the "
             "spread. Times are medians of the suites' own rounds (ms for the large decodes, us for the rest). %s: Yes "
             "when the median change is below zero and the spread does not straddle zero, No when it is above zero and "
             "does not straddle it, Within noise when the spread straddles zero or the median is under %.1f %%. Output "
             "identical: Yes when the two sides' outputs (the decoded samples, the encoded bytes) matched in every "
             "repetition." % (k["change"], k["yes"], NOISE_BAND)]
    if kind == "openjpeg":
        if args.compiler:
            notes.append("OpenJPEG side: the oracle's OpenJPEG 2.5.4 (oracle/PIN), built with %s, always with AVX2 "
                         "(oracle/CMakeLists.txt); the Rust side is neo-j2k at this checkout." % args.compiler)
        if not like:
            notes.append("This cell is not like for like: the Rust build is for the baseline x86-64 and OpenJPEG is "
                         "built with AVX2; the table is shown, its counts are for reading only.")
    api = ["| Suite | %s uses | %s uses |" % (k["first"], k["second"]), "|---|---|---|"]
    api += ["| %s | %s | %s |" % row for row in k["apis"]]
    print("\n\n".join(notes) + "\n")
    print("The API each side used:\n")
    print("\n".join(api) + "\n")
    print(pair_table(rows, kind, like, cpus))
    return 0


def table_command(args):
    if getattr(args, "kind", "cpp") != "cpp":
        return pair_command(args)
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
            why = "GCC 16 is not the compiler the C++ performance numbers of the baseline were measured with (MSVC)."
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
        if len(cells) != 8 or not re.match(r"^(decode|encode|image|rated)/", cells[0]):
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
    other = {"decode": "encode", "encode": "decode", "image": "decode, encode and rated"}[side]
    return [
        "Step gate (%s sets): %s (%d of %d sets at least %.1f %% faster, control within +-%.1f %%, outputs identical)" % (
            side, "Yes" if passed == len(mine) else "No", passed, len(mine), MIN_GAIN, CONTROL_BAND),
        "",
        "Control outside +-%.1f %% in %d of %d sets (a noisy runner for those rows)" % (CONTROL_BAND, strayed, len(rows)),
        "",
        "The %s sets are not judged: this step does not change that code (or, for the rated sets, is not meant to "
        "speed it up), so their rows show layout, noise and regressions." % other,
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
        assert table_command(argparse.Namespace(dir=tmp, compiler="GCC 16", oracle="scalar", like_for_like="yes",
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
    rows = ("| image/rgb32_lossy | 0.88 | -6.62 % | -9.00 to -5.00 | -0.11 % | -0.50 to +0.40 | Yes | Yes |\n"
            "| rated/rgb128_lossy_cap | 13.79 | +0.27 % | -1.00 to +1.00 | -0.01 % | -0.50 to +0.40 | Yes | No |\n")
    img = pair_verdict(PAIR_GOLDEN + rows, "image")
    assert img[0].startswith("Step gate (image sets): Yes (1 of 1 sets"), img[0]
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
    # a pair of the dispatch's own: that pair only, no Rust-versus-C++ cell unless asked for
    own = {"EVENT_NAME": "workflow_dispatch", "ROUNDS_IN": "12", "PAIR_BASE_IN": "aaa", "PAIR_HEAD_IN": "bbb",
           "PAIR_SIDE_IN": "decode", "PAIR_NAME_IN": "mine"}
    gate = json.loads(plan(own)["gate"])["include"]
    assert len(gate) == 4 and {(g["name"], g["base"], g["head"], g["side"]) for g in gate} == {("mine", "aaa", "bbb", "decode")}
    assert plan(own)["cells_count"] == "0" and plan(dict(own, VS_CPP_IN="true"))["cells_count"] == "4"
    assert full["cells_count"] == "4"
    for bad in ({"PAIR_HEAD_IN": ""}, {"PAIR_SIDE_IN": "both"}, {"PAIR_NAME_IN": "a b"}):
        try:
            plan(dict(own, **bad))
        except SystemExit:
            pass
        else:
            raise AssertionError("a bad pair was accepted: %r" % bad)
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
    # Rust against OpenJPEG and the crate's two versions: the output has the two medians and `same` per row
    def pair_text(source, ok="test result: ok. 3 passed; 0 failed", differ=None, drop=None):
        out = ["Rust: one Decoder kept across calls.", "Request    Rust us   OpenJPEG us Output"]
        for key, _unit, first, second in source:
            if drop and key[1].startswith(drop):
                continue
            mark = "DIFFERENT" if differ and key[1].startswith(differ) else "same"
            if key[0] == "large decode":
                out.append("%-26s %5d %9.3f %9.3f %s" % (key[1], key[2], first, second, mark))
            else:
                out.append("%-48s %9.2f %9.2f %s" % (key[1], first, second, mark))
        return "\n".join(out) + "\n\n" + ok + "\n"

    base = parse_run(GOLDEN)
    text = pair_text(base)
    parsed = parse_pair_run(text)
    assert len(parsed) == 23 and parsed[0] == (("large decode", "large_1024_rgb_97.j2k", 1), "ms", 13.195, 21.519, True)
    assert len(check_complete_pair(text, 1)) == 23
    assert len(check_complete_pair(pair_text(base, ok="harness result: ok. 3 passed; 0 failed"), 1)) == 23
    for bad, why in ((pair_text(base, ok="test result: FAILED. 2 passed"), "no result line"),
                     (pair_text(base, drop="encode 32x32 RGB / lossless"), "small encode 5"),
                     (pair_text(base, drop="large_4096"), "large decode 4"),
                     (pair_text(base).replace("319.39", "0.00"), "time of 0"),
                     ("", "no result line")):
        try:
            check_complete_pair(bad, 1)
        except Incomplete as e:
            assert why in str(e), (why, str(e))
        else:
            raise AssertionError("an incomplete pair repetition was accepted: " + why)
    with tempfile.TemporaryDirectory() as tmp:
        for number_, differ in ((1, None), (2, None), (3, "encode 64x64 RGB / lossy")):
            with open(os.path.join(tmp, "rep-%d.txt" % number_), "w", encoding="utf-8") as f:
                f.write(pair_text(base, differ=differ))
        runs = read_reps(tmp, check_complete_pair)
        agg = aggregate(runs, 4)
        assert [r["identical"] for r in agg].count(False) == 1 and [r["over"] for r in agg].count(True) == 2
        for kind, first, second in (("openjpeg", "Rust", "OpenJPEG"), ("versions", "0.1.3", "0.1.2")):
            out = pair_table(agg, kind, True, 4)
            assert "| Suite | Request | Workers | Unit | %s | %s | Change (%s against %s) |" % (
                first, second, first, second) in out, out
            assert "| %s faster | Output identical |" % first in out and "ratio" not in out.lower(), out
            assert "| small encode | encode 64x64 RGB / lossy 9/7 | - | us | 1477.15 | 1339.23 | +10.3 % | +10.3 to +10.3 % " \
                   "| No | No |" in out, out
            assert "Outputs that are NOT identical (1 of 23 rows): encode 64x64 RGB / lossy 9/7" in out, out
            assert "Not counted (more workers than the runner's 4 logical CPUs" in out
            assert "%s faster in 8 rows, within noise in 2, slower in 11, of 21" % first in out, out
            ns = argparse.Namespace(dir=tmp, kind=kind, compiler="GCC 16", oracle=None, like_for_like="no", cpus=4)
            assert table_command(ns) == 0
        assert "not like for like" in pair_table(agg, "openjpeg", False, 4)
        with open(os.path.join(tmp, "rep-3.txt"), "w", encoding="utf-8") as f:
            f.write(pair_text(base, ok="boom"))
        assert table_command(argparse.Namespace(dir=tmp, kind="versions", compiler=None, oracle=None,
                                                like_for_like="yes", cpus=4)) == 1
    with tempfile.TemporaryDirectory() as tmp:
        with open(os.path.join(tmp, "rep-1.txt"), "w", encoding="utf-8") as f:
            f.write(pair_text(base))
        out = pair_table(aggregate(read_reps(tmp, check_complete_pair), 4), "versions", True, 4)
        assert "Outputs identical in all 23 rows" in out, out
    # the plan of the two measurements of Rust against others: a pull request smoke-tests both, a dispatch runs the
    # ones it names (only those, unless it also has a pair of its own or vs_cpp), the default dispatch runs neither
    pr = plan({"EVENT_NAME": "pull_request"})
    assert (pr["gate_count"], pr["cells_count"], pr["opj_count"], pr["ver_count"]) == ("4", "4", "4", "4"), pr
    assert (full["gate_count"], full["cells_count"], full["opj_count"], full["ver_count"]) == ("16", "4", "0", "0")
    both = {"EVENT_NAME": "workflow_dispatch", "ROUNDS_IN": "24", "VS_OPENJPEG_IN": "true", "VS_V012_IN": "true"}
    only = plan(both)
    assert (only["gate_count"], only["cells_count"], only["opj_count"], only["ver_count"]) == ("0", "0", "4", "4"), only
    assert json.loads(only["gate"]) == {"include": []}
    assert plan(dict(both, VS_V012_IN=""))["ver_count"] == "0" and plan(dict(both, VS_OPENJPEG_IN="false"))["opj_count"] == "0"
    assert plan(dict(both, VS_CPP_IN="true"))["cells_count"] == "4" and plan(dict(both, VS_CPP_IN="true"))["gate_count"] == "0"
    mixed = plan(dict(own, VS_V012_IN="true"))
    assert (mixed["gate_count"], mixed["cells_count"], mixed["opj_count"], mixed["ver_count"]) == ("4", "0", "0", "4"), mixed
    opj_cells = json.loads(only["opj_cells"])["include"]
    assert [(c["os"], c["cpu"], c["like"]) for c in opj_cells] == [
        ("ubuntu-26.04", "baseline", "no"), ("ubuntu-26.04", "x86-64-v3", "yes"),
        ("windows-2025", "baseline", "no"), ("windows-2025", "x86-64-v3", "yes")]
    assert all(c["preset"] and c["compiler"] and c["rustflags"] is not None for c in opj_cells)
    ver_cells = json.loads(only["ver_cells"])["include"]
    assert [sorted(c) for c in ver_cells] == [["cpu", "os", "py", "rustflags"]] * 4
    assert [c["rustflags"] for c in ver_cells] == ["", "-C target-cpu=x86-64-v3"] * 2
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
    t.add_argument("--kind", choices=["cpp", "openjpeg", "versions"], default="cpp")
    t.add_argument("--compiler")
    t.add_argument("--oracle")
    t.add_argument("--like-for-like", choices=["yes", "no"], default="yes")
    t.add_argument("--cpus", type=int, help="the runner's logical CPUs (default: this process's)")
    v = sub.add_parser("pair-verdict")
    v.add_argument("--file", required=True)
    v.add_argument("--side", required=True, choices=["decode", "encode", "image"])
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
