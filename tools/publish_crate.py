#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
"""Publish the verified crate from successful main CI, preserving source provenance."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tarfile


def gh(*args):
    return subprocess.check_output(['gh', *args], text=True)


def main():
    folder = Path(sys.argv[1])
    repo = os.environ['GITHUB_REPOSITORY']
    revision = os.environ['GITHUB_SHA']
    current = gh('api', f'repos/{repo}/git/ref/heads/main', '--jq', '.object.sha').strip()
    if current != revision:
        print('A newer main revision will publish its own crate')
        return
    crates = list(folder.glob('neo-j2k-*.crate'))
    assert len(crates) == 1, 'Expected exactly one verified crate'
    crate = crates[0]
    with tarfile.open(crate) as archive:
        vcs = archive.extractfile(f'{crate.stem}/.cargo_vcs_info.json')
        assert vcs is not None, 'Missing crate source provenance'
        assert json.load(vcs)['git']['sha1'] == revision, 'Crate source revision mismatch'
    tag = 'v' + crate.stem.removeprefix('neo-j2k-')
    releases = json.loads(gh('release', 'list', '--repo', repo, '--limit', '100', '--json', 'tagName'))
    if tag not in {r['tagName'] for r in releases}:
        gh('release', 'create', tag, '--repo', repo, '--target', revision, '--title', f'neo-j2k {tag}',
           '--notes', f'Crate built and tested by GitHub Actions from {revision}.')
    release = json.loads(gh('release', 'view', tag, '--repo', repo, '--json', 'body,assets'))
    backup = folder / 'previous'
    backup.mkdir()
    (backup / 'release.json').write_text(json.dumps(release, indent=2))
    previous = [a for a in release['assets'] if a['name'] == crate.name]
    assert len(previous) <= 1
    body = release['body'] or ''
    digest = hashlib.sha256(crate.read_bytes()).hexdigest()
    if previous:
        gh('release', 'download', tag, '--repo', repo, '--pattern', crate.name, '--dir', str(backup))
        old_digest = hashlib.sha256((backup / crate.name).read_bytes()).hexdigest()
        assert previous[0]['digest'] == 'sha256:' + old_digest
        body = body.replace(old_digest, digest)
    body += f'\n\nCrate built and verified from `{revision}` by https://github.com/{repo}/actions/runs/{os.environ["GITHUB_RUN_ID"]}. SHA256 `{digest}`.\n'
    notes = folder / 'release-notes.md'
    notes.write_text(body)
    gh('release', 'upload', tag, str(crate), '--repo', repo, '--clobber')
    verified = folder / 'verified'
    verified.mkdir()
    gh('release', 'download', tag, '--repo', repo, '--pattern', crate.name, '--dir', str(verified))
    assert (verified / crate.name).read_bytes() == crate.read_bytes()
    gh('release', 'edit', tag, '--repo', repo, '--notes-file', str(notes))
    print(f'Published {crate.name}, SHA256 {digest}')


if __name__ == '__main__':
    main()
