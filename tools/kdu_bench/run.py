#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
"""Validate and time the exact Firestorm 81788 Kakadu byte-output decode path."""
import argparse
import hashlib
import json
import platform
from pathlib import Path
import shutil
import statistics
import struct
import subprocess
import tempfile
import time

from probe import PE

ROOT = Path(__file__).resolve().parents[2]
VIEWER_SHA256 = 'c3a3b5483b9778aa64b2cb48f7e61e5aafa1c2550a04b14a9740e97c4a2bfb6e'
DEFAULT_CASES = ['size_1x1_g', 'size_1x1_rgb', 'size_1x64_g', 'size_3x5_g',
                 'size_4x4_rgb_r3', 'size_7x9_rgb_r1', 'size_16x16_rgb_r5',
                 'sl_32_rgba_53', 'layers_4_53', 'large_2048x64_rgba_53']


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def child(command):
    run = subprocess.run([str(part) for part in command], capture_output=True,
                         text=True, timeout=60, creationflags=getattr(subprocess, 'CREATE_NO_WINDOW', 0))
    if run.returncode:
        raise RuntimeError(f'{command[0]} exited {run.returncode}: {run.stderr}')
    return json.loads(run.stdout)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--viewer', required=True, type=Path)
    parser.add_argument('--dependencies', required=True, type=Path)
    parser.add_argument('--probe', type=Path, default=ROOT/'target-kdu-bench/Release/kdu_decode_probe.exe')
    parser.add_argument('--reference', type=Path, default=ROOT/'target-kdu-bench/rust/release/kdu-pixel-reference.exe')
    parser.add_argument('--out', type=Path, default=ROOT/'target-kdu-bench/results')
    parser.add_argument('--iterations', type=int, default=8,
                        help='Calls per round; 0 validates without timing')
    parser.add_argument('--cases', nargs='+', default=DEFAULT_CASES,
                        help='Corpus fixture stems; mismatched outputs are refused')
    args = parser.parse_args()
    if not 0 <= args.iterations <= 100000:
        parser.error('--iterations must be in 0..100000')
    pe = PE(args.viewer)
    if hashlib.sha256(pe.data).hexdigest() != VIEWER_SHA256:
        parser.error('Viewer differs from the exact build this ABI adapter supports')
    args.out.mkdir(parents=True, exist_ok=True)
    manifest = {'date_utc': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
                'platform': platform.platform(), 'viewer': pe.inspect(),
                'decode_exe_sha256': digest(args.probe), 'reference_exe_sha256': digest(args.reference),
                'dependencies_sha256': {},
                'protocol': {'single_thread': True, 'warmup': 16, 'rounds': 9,
                             'iterations_per_round': args.iterations, 'fresh_codec': True,
                             'output': 'interleaved unsigned 8-bit, top-down',
                             'timed': 'create, header/geometry checks, tile setup, decode, byte conversion, teardown',
                             'excluded': 'process/image startup, file IO, input/output allocation, validation',
                             'comparison': 'Kakadu timing only; Rust executable supplies reference pixels'},
                'tools_sha256': {str(p.relative_to(ROOT)): digest(p)
                                 for p in (ROOT/'tools/kdu_bench').rglob('*')
                                 if p.is_file() and p.suffix in ('.py', '.cpp', '.toml', '.lock', '.rs', '.txt')
                                 and '__pycache__' not in p.parts},
                'rust_source_sha256': {str(p.relative_to(ROOT)): digest(p)
                                       for p in (ROOT/'neo-j2k/src').rglob('*.rs')}}
    results = []
    with tempfile.TemporaryDirectory(prefix='kdu-decode-bench-') as directory:
        folder = Path(directory)
        for dependency in args.dependencies.glob('*.dll'):
            if dependency.name.lower() == 'kdu-embedded.dll':
                continue
            shutil.copy2(dependency, folder/dependency.name)
            manifest['dependencies_sha256'][dependency.name] = digest(dependency)
        image = bytearray(pe.data)
        struct.pack_into('<H', image, pe.pe+22, pe.u16(pe.pe+22) | 0x2000)
        struct.pack_into('<I', image, pe.opt+16, 0)
        private = folder/'kdu-embedded.dll'
        private.write_bytes(image)
        manifest['prepared_image_sha256'] = digest(private)
        for case in args.cases:
            if Path(case).name != case or '/' in case or '\\' in case:
                parser.error('Cases must be fixture stems')
            fixture = ROOT/'corpus/generated'/f'{case}.j2k'
            expected = args.out/f'{case}.reference.raw'
            output = args.out/f'{case}.kdu.raw'
            ref = child([args.reference.resolve(), fixture, expected.resolve()])
            result = child([args.probe.resolve(), private, fixture, expected.resolve(), output.resolve(), args.iterations])
            if any(result[key] != ref[key] for key in ('width', 'height', 'components')):
                raise RuntimeError(f'{case}: geometry mismatch')
            if expected.read_bytes() != output.read_bytes():
                raise RuntimeError(f'{case}: pixel mismatch')
            result.update(case=case, fixture_sha256=digest(fixture), pixels_sha256=digest(output))
            if result['round_us']:
                result['median_us'] = statistics.median(result['round_us'])
            results.append(result)
            (args.out/'results.json').write_text(json.dumps(results, indent=2)+'\n')
            print(f"{case}: pixels match" + (f", {result['median_us']:.3f} us" if 'median_us' in result else ''), flush=True)
    (args.out/'manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')
    print(f'Validated {len(results)} cases; evidence: {args.out.resolve()}')


if __name__ == '__main__':
    main()
