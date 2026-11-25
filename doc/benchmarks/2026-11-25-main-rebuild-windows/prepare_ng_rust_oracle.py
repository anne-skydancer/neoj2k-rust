# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)

"""Materialize the exact pinned Rust sources under a unique test-only package name.

Direct Git dependencies scan historical audit manifests, some of which contain
absolute paths. An explicit verified source copy avoids that discovery ambiguity.
No codec source is rewritten. The independent upstream checkout remains clean.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tomllib


def materialize(source, destination, revision):
    source, destination = source.resolve(), destination.resolve()
    actual = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
    if actual != revision:
        raise ValueError(f'oracle revision mismatch: {actual} != {revision}')
    dirty = subprocess.check_output(['git', '-C', str(source), 'status', '--porcelain', '--untracked-files=no'], text=True)
    if dirty.strip():
        raise ValueError('oracle checkout has modified tracked files')
    workspace = tomllib.loads((source/'Cargo.toml').read_text(encoding='utf-8'))['workspace']['package']
    original = tomllib.loads((source/'neo-j2k/Cargo.toml').read_text(encoding='utf-8'))
    if original.get('dependencies') or original['package'].get('build') or original['package'].get('links'):
        raise ValueError('oracle materialization requires the dependency-free baseline')
    files = subprocess.check_output(['git', '-C', str(source), 'ls-files', 'neo-j2k/src'], text=True).splitlines()
    if not files:
        raise ValueError('empty oracle source inventory')
    destination.mkdir(parents=True, exist_ok=True)
    hashes = {}
    for name in files:
        # Read Git blobs so checkout newline settings cannot change the oracle material.
        data = subprocess.check_output(['git', '-C', str(source), 'show', revision+':'+name])
        relative = Path(name).relative_to('neo-j2k')
        target = destination/relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
        hashes[str(relative).replace('\\', '/')] = hashlib.sha256(data).hexdigest()
    expected = {str(Path(n).relative_to('neo-j2k')).replace('\\', '/') for n in files}
    present = {str(p.relative_to(destination)).replace('\\', '/') for p in (destination/'src').rglob('*') if p.is_file()}
    if expected != present:
        raise ValueError('stale/unexpected oracle source files; use a fresh oracle destination')
    manifest = '# SPDX-License-Identifier: BSD-2-Clause\n[package]\nname = "neo-j2k-reference"\npublish = false\n'
    for key in ['version', 'edition', 'rust-version', 'license']:
        manifest += key+' = '+json.dumps(workspace[key])+'\n'
    manifest += '\n[lib]\nname = "neo_j2k_reference"\n\n[features]\n'
    for key, value in original['features'].items():
        manifest += key+' = '+json.dumps(value)+'\n'
    (destination/'Cargo.toml').write_text(manifest, encoding='utf-8')
    (destination/'oracle-source.json').write_text(json.dumps({'revision': revision, 'source_sha256': hashes}, indent=2)+'\n')
    print(f'Pinned Rust oracle: {len(hashes)} unchanged source blobs at {revision}')


if __name__ == '__main__':
    root = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, default=root/'.oracles/neoj2k-rust')
    args = parser.parse_args()
    pin = tomllib.loads((root/'experimental.toml').read_text())['oracle']['revision']
    materialize(args.source, root/'.oracles/rust-reference', pin)
