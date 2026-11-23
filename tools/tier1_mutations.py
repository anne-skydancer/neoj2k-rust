#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
"""Mutation check of the Tier-1 tests: every mutation of tools/mutations/*.json must be caught.

A mutation is one change of the code (a comparison, a shift, a merge left out) that makes it wrong. The tests that
name it must then fail; one that passes means the tests cannot see that kind of mistake, and the check fails.

    python -I tools/tier1_mutations.py [--list] [--only TEXT] [--shard I/N] [--profile NAME] [--keep] [FILE.json ...]

With no file it reads every tools/mutations/*.json (tools/mutations/README.md has the format and the rules), so a stage
adds its mutations in a file of its own without touching the workflow; with no list at all there is nothing to check
and it exits 0.

The mutations of all the lists, sorted by name, are numbered from 0; `--shard I/N` (I from 1 to N) runs those whose
number modulo N is I - 1, so that N jobs split the list between them. One scratch copy of the tracked files is made,
each mutation is applied to it in place and reverted after its tests, and the build folder is shared: only the changed
crate is built again, the C++ oracle once. The checkout itself is never touched. Before the first mutation every test
filter runs once without any mutation and must pass, so that a test that fails for another reason cannot count as a
catch.

Tests run as `cargo test --profile NAME --locked` (default profile `mutation`, in the workspace Cargo.toml), in the CPU
level the environment's RUSTFLAGS set (the baseline when none). A mutation that does not compile, or whose pattern does
not match, is an error of the list, not a caught mutation. A survivor is possibly an equivalent mutant (a change that
does not alter what the code computes); it is printed with its change and still fails the run, and the list is
corrected by dropping or replacing it.

Exit 0 when every mutation was caught, 1 when one survived, 2 on an error of the list or of the run. `--list` only
checks the patterns (no build); `--only` runs the mutations whose name contains the text. The result is printed as a
markdown table and appended to $GITHUB_STEP_SUMMARY when it is set.
"""

import argparse
import glob
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def load(files):
    mutations = []
    for path in files:
        with open(path, encoding="utf-8") as f:
            for m in json.load(f):
                for key in ("name", "file", "old", "new", "tests"):
                    if key not in m:
                        raise SystemExit("%s: a mutation lacks %r: %s" % (path, key, m))
                if not m["tests"]:
                    raise SystemExit("%s: %s names no test" % (path, m["name"]))
                m["list"] = os.path.basename(path)
                mutations.append(m)
    names = [m["name"] for m in mutations]
    dup = sorted({n for n in names if names.count(n) > 1})
    if dup:
        raise SystemExit("duplicate mutation names: %s" % ", ".join(dup))
    return sorted(mutations, key=lambda m: m["name"])


def read(path):
    with open(path, encoding="utf-8", newline="") as f:
        return f.read()


def write(path, text):
    with open(path, "w", encoding="utf-8", newline="") as f:
        f.write(text)


def mutated(text, m):
    """The text with the mutation applied (the patterns are written with line feeds), or None when the pattern's
    count is not the expected one."""
    crlf = "\r\n" in text
    old, new = (m["old"], m["new"]) if not crlf else (m["old"].replace("\n", "\r\n"), m["new"].replace("\n", "\r\n"))
    if text.count(old) != m.get("count", 1):
        return None
    return text.replace(old, new)


def copy_tree(dest):
    files = subprocess.run(["git", "ls-files", "-z"], cwd=ROOT, capture_output=True, check=True).stdout
    for rel in files.decode("utf-8").split("\0"):
        if not rel:
            continue
        src = os.path.join(ROOT, rel)
        if not os.path.isfile(src):
            continue
        out = os.path.join(dest, rel)
        os.makedirs(os.path.dirname(out), exist_ok=True)
        shutil.copy2(src, out)


def cargo_test(tree, m, test, env, profile):
    cmd = ["cargo", "test", "--profile", profile, "--locked", "-p", m.get("package", "neo-j2k")]
    cmd += m.get("target", "--lib").split()
    cmd += [test]
    r = subprocess.run(cmd, cwd=tree, env=env, capture_output=True, text=True, errors="replace")
    return r.returncode, r.stdout + r.stderr


def failed_tests(out):
    return re.findall(r"^test (\S+) \.\.\. FAILED$", out, re.M)


def passed_tests(out):
    return re.findall(r"^test (\S+) \.\.\. ok$", out, re.M)


def does_not_build(out):
    return "could not compile" in out or "error[" in out or "error: failed to run custom build" in out


def judge(m, env, tree, profile):
    """(caught, killer, detail): caught is True, False (survived) or None (an error of the list or of the run)."""
    killer = ""
    for test in m["tests"]:
        code, out = cargo_test(tree, m, test, env, profile)
        if does_not_build(out):
            return None, "", "does not compile"
        failed = failed_tests(out)
        if code == 0:
            if not passed_tests(out):
                return None, "", "the filter %r matches no test" % test
            return False, "", "%s passes" % test
        if not failed:
            return None, "", "cargo test failed without a failed test (%s)" % test
        killer = killer or failed[0]
    return True, killer, ""


def table(rows):
    lines = ["| Mutation | Killed | Killed by | Seconds |", "|---|---|---|---|"]
    for name, killed, by, seconds in rows:
        lines.append("| %s | %s | %s | %d |" % (name, killed, by.replace("|", "\\|"), round(seconds)))
    return "\n".join(lines)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("files", nargs="*")
    ap.add_argument("--list", action="store_true", help="check the patterns only; build and run nothing")
    ap.add_argument("--only", help="run the mutations whose name contains this text")
    ap.add_argument("--shard", help="I/N: run the mutations whose number modulo N is I - 1")
    ap.add_argument("--profile", default="mutation", help="the cargo profile the tests run in (default mutation)")
    ap.add_argument("--keep", action="store_true", help="keep the scratch copy")
    args = ap.parse_args()
    files = args.files or sorted(glob.glob(os.path.join(ROOT, "tools", "mutations", "*.json")))
    if not files:
        print("tier1_mutations: no mutation lists: nothing to check")
        return 0
    mutations = load(files)
    if args.shard:
        m_ = re.fullmatch(r"(\d+)/(\d+)", args.shard)
        if not m_ or not 1 <= int(m_.group(1)) <= int(m_.group(2)):
            raise SystemExit("--shard wants I/N with I from 1 to N, got %r" % args.shard)
        index, count = int(m_.group(1)), int(m_.group(2))
        mutations = [m for i, m in enumerate(mutations) if i % count == index - 1]
    mutations = [m for m in mutations if not args.only or args.only in m["name"]]
    bad = []
    for m in mutations:
        path = os.path.join(ROOT, m["file"])
        if not os.path.isfile(path) or mutated(read(path), m) is None:
            bad.append(m["name"])
            print("PATTERN  %s (%s): %r does not occur %d time(s) in %s" % (
                m["name"], m["list"], m["old"], m.get("count", 1), m["file"]))
    if bad:
        return 2
    if args.list:
        print("tier1_mutations: %d mutations, every pattern matches" % len(mutations))
        return 0
    if not mutations:
        print("tier1_mutations: no mutation in this shard: nothing to check")
        return 0

    rows = []
    survivors = []
    errors = []
    tree = tempfile.mkdtemp(prefix="nj2k-mut-")
    try:
        copy_tree(tree)
        env = dict(os.environ)
        env.setdefault("CARGO_TARGET_DIR", os.path.join(tree, "target"))

        # the tests without any mutation: they must pass, or a failure under a mutation proves nothing
        seen = set()
        unmutated_failures = []
        for m in mutations:
            for test in m["tests"]:
                key = (m.get("package", "neo-j2k"), m.get("target", "--lib"), test)
                if key in seen:
                    continue
                seen.add(key)
                code, out = cargo_test(tree, m, test, env, args.profile)
                if code != 0 or not passed_tests(out):
                    # keep going: report every failing test in one run, not one a run
                    unmutated_failures.append(test)
                    print("tier1_mutations: the unmutated tests of %r do not pass (profile %s):" % (test, args.profile))
                    print("\n".join(out.splitlines()[-40:]))


        if unmutated_failures:
            print("tier1_mutations: %d of %d named tests fail without any mutation (profile %s): %s"
                  % (len(unmutated_failures), len(seen), args.profile, ", ".join(unmutated_failures)))
            return 2

        for m in mutations:
            path = os.path.join(tree, m["file"])
            original = read(path)
            write(path, mutated(original, m))
            start = time.monotonic()
            try:
                caught, killer, detail = judge(m, env, tree, args.profile)
            finally:
                write(path, original)
            seconds = time.monotonic() - start
            if caught is True:
                rows.append((m["name"], "Yes", killer, seconds))
                print("caught    %s (%s, %d s)" % (m["name"], killer, round(seconds)), flush=True)
            elif caught is False:
                rows.append((m["name"], "No", "survived: " + detail, seconds))
                survivors.append(m)
                print("SURVIVED  %s (%s)" % (m["name"], detail), flush=True)
            else:
                rows.append((m["name"], "No", "error: " + detail, seconds))
                errors.append(m)
                print("ERROR     %s (%s)" % (m["name"], detail), flush=True)
    finally:
        if args.keep:
            print("tier1_mutations: scratch copy kept at", tree)
        else:
            shutil.rmtree(tree, ignore_errors=True)

    caught_count = sum(1 for r in rows if r[1] == "Yes")
    summary = "%s\n\n%d of %d mutations caught\n" % (table(rows), caught_count, len(rows))
    print()
    print(summary)
    for m in survivors:
        print("A survivor, possibly an equivalent mutant (%s, %s):" % (m["list"], m["name"]))
        print("  in %s replace\n%s\n  with\n%s" % (m["file"], m["old"], m["new"]))
    path = os.environ.get("GITHUB_STEP_SUMMARY")
    if path:
        with open(path, "a", encoding="utf-8") as f:
            f.write("### Tier-1 mutations\n\n" + summary + "\n")
            for m in survivors:
                f.write("Survivor, possibly an equivalent mutant: `%s` (%s)\n\n" % (m["name"], m["list"]))
    return 2 if errors else 1 if survivors else 0


if __name__ == "__main__":
    sys.exit(main())
