#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
"""The Rust crate neo-j2k is native Rust (owner's rule; README.md), checked at the repository root:

1. its normal and build dependency graph (dev-dependencies left out) holds no crate that builds or
   binds C or C++: none has a build script or a `links` key, none is a -sys crate or a C/C++ build
   or binding helper (cc, cmake, bindgen, cxx, ...); and the C++ oracle crate (neo-j2k-oracle) is
   not in it. The crate has no runtime dependency today, so the graph is the crate alone;
2. the crate itself has no build script and no `links` key;
3. the package `cargo package` would publish holds Rust sources, its manifests, readme and licence
   files only: no C or C++ source or header, no build script, no binary;
4. the crate's sources hold no FFI (extern "C" or any other ABI, #[link], #[no_mangle],
   #[export_name]), no `unsafe` outside src/simd/, no `static mut` and no `thread_local!`.

usage: check_rust_native.py [--workspace DIR] [--selftest]
"""
import argparse
import json
import os
import re
import subprocess
import sys

CRATE = "neo-j2k"
ORACLE = "neo-j2k-oracle"
C_HELPERS = {"cc", "cmake", "bindgen", "cxx", "cxx-build", "cxxbridge-macro", "pkg-config", "vcpkg", "autocfg-c",
             "libc", "windows-sys", "winapi", "nasm-rs", "cc-version"}
C_SUFFIXES = (".c", ".cc", ".cpp", ".cxx", ".h", ".hh", ".hpp", ".hxx", ".inl", ".s", ".asm", ".m", ".mm",
              ".a", ".lib", ".o", ".obj", ".so", ".dll", ".dylib")
PACKAGE_ALLOWED = re.compile(r"^(Cargo\.toml|Cargo\.toml\.orig|Cargo\.lock|\.cargo_vcs_info\.json|README\.md|"
                             r"LICENSE[A-Za-z.]*|NOTICE\.md|src/.*\.rs)$")
SOURCE_RULES = [
    (re.compile(r'\bextern\s+"[A-Za-z0-9_-]+"'), "an extern ABI (FFI)"),
    (re.compile(r"\bunsafe\s+extern\b"), "an extern block (FFI)"),
    (re.compile(r"#\s*\[\s*(unsafe\s*\(\s*)?(link|no_mangle|export_name|link_name)\b"), "a link attribute (FFI)"),
    (re.compile(r"\bstatic\s+mut\b"), "a static mut"),
    (re.compile(r"\bthread_local!"), "a thread_local!"),
]
UNSAFE = re.compile(r"\bunsafe\b")


def cargo(workspace, *args):
    env = dict(os.environ, NJ2K_ORACLE_NO_BUILD="1")
    r = subprocess.run(["cargo", *args], cwd=workspace, env=env, capture_output=True, text=True)
    if r.returncode != 0:
        sys.stderr.write(r.stderr)
        raise SystemExit("check_rust_native: cargo %s failed" % " ".join(args))
    return r.stdout


def graph_problems(meta):
    """Problems in the crate's normal and build dependency closure, and the closure's size."""
    packages = {p["id"]: p for p in meta["packages"]}
    nodes = {n["id"]: n for n in meta["resolve"]["nodes"]}
    roots = [p["id"] for p in meta["packages"] if p["name"] == CRATE]
    if len(roots) != 1:
        return ["the workspace holds %d packages named %s" % (len(roots), CRATE)], 0
    problems, seen, todo = [], set(), list(roots)
    while todo:
        pid = todo.pop()
        if pid in seen:
            continue
        seen.add(pid)
        for dep in nodes[pid]["deps"]:
            kinds = {k["kind"] for k in dep["dep_kinds"]}
            if kinds - {"dev"}:  # normal (None) or build
                todo.append(dep["pkg"])
    for pid in sorted(seen):
        p = packages[pid]
        name = p["name"]
        where = "%s %s" % (name, p["version"])
        if name == ORACLE:
            problems.append("the C++ oracle crate is in %s's normal or build graph" % CRATE)
        if name in C_HELPERS or name.endswith("-sys"):
            problems.append("%s builds or binds C or C++" % where)
        if p.get("links"):
            problems.append("%s links a native library (links = %s)" % (where, p["links"]))
        if any("custom-build" in t["kind"] for t in p["targets"]):
            problems.append("%s has a build script" % where)
    return problems, len(seen)


def package_problems(files):
    problems = []
    for f in files:
        f = f.strip().replace("\\", "/")
        if not f:
            continue
        if f.lower().endswith(C_SUFFIXES) or f == "build.rs" or not PACKAGE_ALLOWED.match(f):
            problems.append("the package holds %s" % f)
    return problems


def source_problems(text, rel):
    """FFI, unsafe outside src/simd/, static mut, thread_local! in one source (comments left out)."""
    problems = []
    in_simd = rel.replace("\\", "/").startswith("src/simd/")
    for no, line in enumerate(text.splitlines(), 1):
        code = line.split("//", 1)[0]
        for rule, what in SOURCE_RULES:
            if rule.search(code):
                problems.append("%s:%d: %s" % (rel, no, what))
        if not in_simd and UNSAFE.search(code):
            problems.append("%s:%d: unsafe outside src/simd/" % (rel, no))
    return problems


def selftest():
    meta = {
        "packages": [
            {"id": "a", "name": CRATE, "version": "0.1.0", "links": None, "targets": [{"kind": ["lib"]}]},
            {"id": "b", "name": "cc", "version": "1.0.0", "links": None, "targets": [{"kind": ["lib"]}]},
            {"id": "c", "name": ORACLE, "version": "0.1.0", "links": "x", "targets": [{"kind": ["custom-build"]}]},
            {"id": "d", "name": "zstd-sys", "version": "2.0.0", "links": "zstd", "targets": [{"kind": ["lib"]}]},
        ],
        "resolve": {"nodes": [
            {"id": "a", "deps": [{"pkg": "b", "dep_kinds": [{"kind": "build"}]},
                                 {"pkg": "c", "dep_kinds": [{"kind": "dev"}]},
                                 {"pkg": "d", "dep_kinds": [{"kind": None}]}]},
            {"id": "b", "deps": []}, {"id": "c", "deps": []}, {"id": "d", "deps": []},
        ]},
    }
    got, size = graph_problems(meta)
    assert size == 3, size
    assert any("cc 1.0.0" in p for p in got), got
    assert any("zstd-sys" in p and "links" in p for p in got), got
    assert not any("oracle" in p for p in got), got  # a dev-dependency only
    assert package_problems(["Cargo.toml", "src/lib.rs", "src/a/b.rs", "Cargo.toml.orig"]) == []
    assert package_problems(["src/x.cpp"]) and package_problems(["build.rs"]) and package_problems(["lib/x.h"])
    assert source_problems('unsafe extern "C" { fn f(); }', "src/a.rs")
    assert source_problems("#[no_mangle] pub fn f() {}", "src/a.rs")
    assert source_problems("#[unsafe(no_mangle)] pub fn f() {}", "src/a.rs")
    assert source_problems("static mut X: u8 = 0;", "src/a.rs")
    assert source_problems("let v = unsafe { f() };", "src/a.rs")
    assert not source_problems("let v = unsafe { f() };", "src/simd/x.rs")
    assert not source_problems("// unsafe extern \"C\" in a comment", "src/a.rs")
    assert not source_problems("#![deny(unsafe_code)]", "src/lib.rs")  # the lint's name is no unsafe code
    print("check_rust_native: selftest passed")
    return 0


def main():
    ap = argparse.ArgumentParser()
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    ap.add_argument("--workspace", default=root)
    ap.add_argument("--selftest", action="store_true")
    a = ap.parse_args()
    if a.selftest:
        return selftest()
    meta = json.loads(cargo(a.workspace, "metadata", "--format-version", "1", "--all-features", "--locked"))
    problems, size = graph_problems(meta)
    crate = next(p for p in meta["packages"] if p["name"] == CRATE)
    crate_dir = os.path.dirname(crate["manifest_path"])
    if crate.get("links") or any("custom-build" in t["kind"] for t in crate["targets"]):
        problems.append("%s itself has a build script or a links key" % CRATE)
    files = cargo(a.workspace, "package", "-p", CRATE, "--list", "--allow-dirty", "--locked").splitlines()
    problems += package_problems(files)
    sources = 0
    for dirpath, _, names in os.walk(os.path.join(crate_dir, "src")):
        for n in names:
            if n.endswith(".rs"):
                path = os.path.join(dirpath, n)
                rel = os.path.relpath(path, crate_dir)
                with open(path, encoding="utf-8") as f:
                    text = f.read()
                problems += source_problems(text, rel)
                sources += 1
    for p in problems:
        print("check_rust_native: " + p, file=sys.stderr)
    if problems:
        return 1
    print("check_rust_native: %s is native Rust: %d crate(s) in its normal and build graph, none building or "
          "binding C or C++; a %d-file package of Rust sources; %d sources without FFI and without unsafe outside "
          "src/simd/" % (CRATE, size, len(files), sources))
    return 0


if __name__ == "__main__":
    sys.exit(main())
