#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
"""Fetch OpenJPEG's test data (github.com/uclouvain/openjpeg-data) at a pinned commit, sparse: the
ISO/IEC 15444-4 class 1 streams and their reference images, and the other codestreams and JP2 files
of input/conformance, which the header tests read (about 65 MB); with --all, the whole of
input/conformance, baseline/conformance and input/nonregression.

The files' notice (input/conformance/COPYRIGHT) allows their use for testing conformance only, and
openjpeg-data states no licence for the rest: they are fetched at test time, never committed or
shipped. The default folder is .testdata/openjpeg-data in the repository (ignored by git); point
NJ2K_TESTDATA (CMake or the environment) at it.

usage: fetch_testdata.py [--dest DIR] [--all]
"""
import argparse
import os
import subprocess
import sys

REPO = "https://github.com/uclouvain/openjpeg-data.git"
COMMIT = "39524bd3a601d90ed8e0177559400d23945f96a9"  # 2024-11-13
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def git(args, cwd=None):
    subprocess.run(["git", "-c", "core.autocrlf=false", "-c", "advice.detachedHead=false"] + args, cwd=cwd,
                   check=True)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dest", default=os.path.join(ROOT, ".testdata", "openjpeg-data"))
    ap.add_argument("--all", action="store_true")
    a = ap.parse_args()
    if a.all:
        paths = ["/input/conformance/", "/baseline/conformance/", "/input/nonregression/"]
    else:
        paths = ["/input/conformance/COPYRIGHT", "/input/conformance/p0_*.j2k", "/input/conformance/p1_*.j2k",
                 "/input/conformance/*.j2c", "/input/conformance/*.jp2",
                 "/baseline/conformance/c1p0_*", "/baseline/conformance/c1p1_*"]
    dest = a.dest
    if not os.path.isdir(os.path.join(dest, ".git")):
        os.makedirs(dest, exist_ok=True)
        git(["init", "-q", dest])
        git(["remote", "add", "origin", REPO], cwd=dest)
        git(["config", "core.sparseCheckout", "true"], cwd=dest)
    git(["sparse-checkout", "set", "--no-cone"] + paths, cwd=dest)
    head = subprocess.run(["git", "rev-parse", "HEAD"], cwd=dest, capture_output=True, text=True).stdout.strip()
    if head != COMMIT:
        git(["fetch", "-q", "--depth", "1", "--filter=blob:none", "origin", COMMIT], cwd=dest)
        git(["checkout", "-q", COMMIT], cwd=dest)
    else:
        git(["checkout", "-q", COMMIT], cwd=dest)
    if not os.path.isfile(os.path.join(dest, "input", "conformance", "p0_01.j2k")):
        print("fetch_testdata: the checkout lacks input/conformance/p0_01.j2k", file=sys.stderr)
        return 1
    print("fetch_testdata: %s at %s (%s)" % (dest, COMMIT[:12], ", ".join(paths)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
