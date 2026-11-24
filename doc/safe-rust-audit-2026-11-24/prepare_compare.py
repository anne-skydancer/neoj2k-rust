#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
"""Prepare the safe-Rust three-way public-API comparison outside the repository."""
import argparse
import io
from pathlib import Path
import shutil
import subprocess
import tarfile

ap=argparse.ArgumentParser(description=__doc__)
ap.add_argument('--reference-repo',type=Path,required=True)
ap.add_argument('--candidate-repo',type=Path,required=True)
ap.add_argument('--destination',type=Path,required=True)
ap.add_argument('--reference',default='d13082f')
a=ap.parse_args()
dest=a.destination.resolve()
if dest.exists():
    raise SystemExit(f'{dest} already exists; choose a fresh destination')
dest.mkdir(parents=True)
crates=['neo-j2k','neo-j2k-bench','neo-j2k-oracle','neo-j2k-testkit']
fixtures=['large_1024_rgb_97.j2k','large_4096_g_97.j2k','sl_32_rgba_53.j2k','sl_64_rgb_97.j2k','sl_128_rgba_97.j2k']
paths=['Cargo.toml','Cargo.lock','rust-toolchain.toml','README.md',*crates,*[f'corpus/generated/{name}' for name in fixtures]]
archive=subprocess.check_output(['git','-C',str(a.reference_repo.resolve()),'archive',a.reference,*paths])
base=dest/'base';base.mkdir()
with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
    tar.extractall(base,filter='data')
shutil.copytree(base,dest/'control')
for folder,name in [('base','neo-j2k-reference'),('control','neo-j2k-control')]:
    manifest=dest/folder/'neo-j2k/Cargo.toml'
    manifest.write_text(manifest.read_text().replace('name = "neo-j2k"',f'name = "{name}"',1))
shutil.copytree(base/'corpus',dest/'corpus')
resource=Path(__file__).resolve().parent/'harness'
shutil.copytree(resource,dest/'harness')
manifest=dest/'harness/Cargo.toml'
s=manifest.read_text().replace('../candidate/neo-j2k-oracle',(a.candidate_repo.resolve()/'neo-j2k-oracle').as_posix()).replace('../candidate/neo-j2k',(a.candidate_repo.resolve()/'neo-j2k').as_posix())
manifest.write_text(s)
print(dest/'harness/Cargo.toml')
print('Set NJ2K_ORACLE_DIR to a compatible built oracle, then build --release (default features).')
print('Use distinct CARGO_TARGET_DIR for baseline and RUSTFLAGS="-C target-cpu=x86-64-v3".')
print('SAFE_AUDIT_RESIDUAL=1 selects the seven focused residual rows; unset runs all 32 rows.')
