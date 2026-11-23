#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
"""Summarize one rustc --emit=asm file; counts are static, never dynamic costs.

Usage: python inspect-assembly.py path/to/neo_j2k-HASH.s
Build from the repository root, with a separate CARGO_TARGET_DIR:
  cargo rustc --locked --release -p neo-j2k --features bench-internals --lib -- --emit=asm
Repeat with RUSTFLAGS='-C target-cpu=x86-64-v3'. The custom Cargo directory layout
places assembly below release/build/neo-j2k/HASH/out/. This inspects the optimized
library emission, not the final linked executable after downstream LTO.
"""
import collections
import pathlib
import re
import sys

KEYS = [
    "passesNtB3_6Passes10refinementKb0",
    "passesNtB3_6Passes12significanceKb0",
    "passesNtB3_6Passes7cleanupKb0",
    "tier1NtB4_6Passes12significance",
    "tier1NtB4_6Passes6refine",
    "tier1NtB4_6Passes7cleanup",
    "gather_samples",
]
lines = pathlib.Path(sys.argv[1]).read_text().splitlines()
for index, line in enumerate(lines):
    if not (line.startswith("_R") and line.endswith(":")):
        continue
    if not any(key in line for key in KEYS):
        continue
    end = next((j for j in range(index + 1, len(lines)) if lines[j].strip() == ".seh_endproc"), len(lines))
    instructions = [x.strip() for x in lines[index:end] if re.match(r"^\t[a-z][a-z0-9]*\s", x)]
    print(line)
    print("assembly lines", index + 1, end + 1)
    print("static instruction lines", len(instructions), "stack-reference lines", sum("(%rsp)" in x for x in instructions))
    print("calls", dict(collections.Counter(x.split("\t")[-1] for x in instructions if x.startswith("callq"))))
    if "significanceKb0" in line:
        print("prologue:\n" + "\n".join(instructions[:24]))
    if "gather_samples" in line:
        print("round/conversion instructions:\n" + "\n".join(x for x in instructions if any(k in x for k in ("round", "cvt", "rintf", "divss"))))
