#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
"""Matched Windows byte-output decode comparison: Kakadu, neo-j2k C++, Rust."""
import argparse
import ctypes
import itertools
import json
import os
from pathlib import Path
import platform
import queue
import shutil
import statistics
import struct
import subprocess
import tempfile
import threading
import time
import winreg

from probe import PE
from run import ROOT, VIEWER_SHA256, digest


class Worker:
    def __init__(self, command, mask, log):
        self.log = open(log, 'w')
        self.process = subprocess.Popen([str(x) for x in command], stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, stderr=self.log, text=True,
                                        creationflags=subprocess.CREATE_NO_WINDOW)
        kernel = ctypes.WinDLL('kernel32', use_last_error=True)
        kernel.SetProcessAffinityMask.argtypes = [ctypes.c_void_p, ctypes.c_size_t]
        if not kernel.SetProcessAffinityMask(int(self.process._handle), mask):
            self.process.kill()
            raise ctypes.WinError(ctypes.get_last_error())
        self.lines = queue.Queue()
        threading.Thread(target=self.read, daemon=True).start()
        try:
            if not self.receive().get('ready'):
                raise RuntimeError('Worker did not become ready')
        except Exception:
            self.process.kill()
            self.process.wait(timeout=5)
            self.log.close()
            raise

    def read(self):
        for line in self.process.stdout:
            self.lines.put(line)
        self.lines.put(None)

    def receive(self):
        line = self.lines.get(timeout=60)
        if line is None:
            raise RuntimeError(f'Worker exited; see {self.log.name}')
        return json.loads(line)

    def batch(self, count, warm):
        self.process.stdin.write(f'{count} {warm}\n')
        self.process.stdin.flush()
        result = self.receive()
        if not result.get('validated') or result['us'] <= 0:
            raise RuntimeError('Invalid batch result')
        return result['us']

    def close(self):
        try:
            self.process.stdin.close()
            self.process.wait(timeout=5)
        except Exception:
            self.process.kill()
            self.process.wait(timeout=5)
        self.log.close()


def git(directory, *args):
    return subprocess.check_output(['git', '-C', str(directory), *args], text=True).strip()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--viewer', required=True, type=Path)
    parser.add_argument('--dependencies', required=True, type=Path)
    parser.add_argument('--cpp-source', required=True, type=Path)
    parser.add_argument('--build', type=Path, default=ROOT/'target-kdu-bench')
    parser.add_argument('--out', type=Path, default=ROOT/'target-kdu-bench/comparison')
    parser.add_argument('--trials', type=int, default=3)
    args = parser.parse_args()
    if os.name != 'nt' or not 1 <= args.trials <= 9:
        parser.error('Requires Windows and 1..9 trials')
    args.out = args.out.resolve()
    args.out.mkdir(parents=True, exist_ok=True)
    build = args.build.resolve()
    native = build/'Release/kdu_compare_native.exe'
    rust = build/'rust-v3/release/worker.exe'
    generator = build/'rust-v3/release/fixtures.exe'
    fixtures = args.out/'fixtures'
    subprocess.run([str(generator), str(fixtures)], check=True)
    pe = PE(args.viewer)
    if digest(args.viewer) != VIEWER_SHA256:
        parser.error('Unrecognized viewer binary')
    kernel = ctypes.WinDLL('kernel32', use_last_error=True)
    kernel.GetCurrentProcess.restype = ctypes.c_void_p
    kernel.GetProcessAffinityMask.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_size_t), ctypes.POINTER(ctypes.c_size_t)]
    allowed, system = ctypes.c_size_t(), ctypes.c_size_t()
    if not kernel.GetProcessAffinityMask(kernel.GetCurrentProcess(), ctypes.byref(allowed), ctypes.byref(system)):
        raise ctypes.WinError(ctypes.get_last_error())
    mask = allowed.value & -allowed.value
    cases = [(f'rgb_{side}_53', fixtures/f'rgb_{side}_53.j2k', fixtures/f'rgb_{side}_53.raw', side, side, 3)
             for side in [32, 128, 512, 1024]]
    # Include a wide rectangular RGBA image from the existing verified corpus.
    rectangle = ROOT/'corpus/generated/large_2048x64_rgba_53.j2k'
    reference = args.out/'rectangle.raw'
    subprocess.run([str(build/'rust-v3/release/kdu-pixel-reference.exe'), str(rectangle), str(reference)], check=True, capture_output=True)
    cases.append(('rgba_2048x64_53', rectangle, reference, 2048, 64, 4))
    names = ('kdu', 'cpp', 'rust')
    orders = list(itertools.permutations(names))
    with winreg.OpenKey(winreg.HKEY_LOCAL_MACHINE, r'HARDWARE\DESCRIPTION\System\CentralProcessor\0') as key:
        cpu_name = winreg.QueryValueEx(key, 'ProcessorNameString')[0].strip()
    manifest = {'date_utc': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
                'platform': platform.platform(), 'cpu': cpu_name, 'processor': platform.processor(), 'logical_processors_visible': os.cpu_count(), 'affinity_mask': mask,
                'rust_commit': git(ROOT, 'rev-parse', 'HEAD'), 'cpp_commit': git(args.cpp_source, 'rev-parse', 'HEAD'),
                'rust_status': git(ROOT, 'status', '--short'), 'cpp_status': git(args.cpp_source, 'status', '--short'),
                'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
                'viewer': pe.inspect(), 'binary_sha256': {str(p): digest(p) for p in [native, rust, generator, build/'cpp/src/Release/neoj2k_static.lib']},
                'source_sha256': {str(p): digest(p) for directory in [ROOT/'neo-j2k/src', args.cpp_source/'src', args.cpp_source/'include', ROOT/'tools/kdu_bench']
                                  for p in directory.rglob('*') if p.is_file() and p.suffix in ('.rs', '.cpp', '.h', '.py', '.toml', '.lock', '.txt')},
                'cmake_cache': (build/'CMakeCache.txt').read_text(),
                'protocol': {'trials': args.trials, 'rounds_per_trial': 6, 'orders': orders,
                             'fresh_codec': True, 'single_thread': True, 'kakadu_mode': 'set_fast (viewer MODE_FAST)', 'output': 'caller-owned interleaved U8 top-down',
                             'warm_calls': '32 for <=64KiB output, otherwise 4',
                             'batch': 'per actor calibrated to approximately 20ms; 1..2048 calls',
                             'validation': 'every byte and output guards before/after each batch',
                             'timing': 'create, header/geometry checks, decode, conversion, full decoder teardown',
                             'excluded': 'process startup, library/dispatch init, file IO, output allocation, pixel validation',
                             'build': 'C++ MSVC Release AVX2 no IPO; Rust release x86-64-v3 fat LTO; Kakadu prebuilt Firestorm 81788'}}
    summary, raw = [], []
    with tempfile.TemporaryDirectory(prefix='kdu-matched-') as directory:
        folder = Path(directory)
        manifest['dependencies_sha256'] = {}
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
        for case, source, expected, width, height, channels in cases:
            medians = {name: [] for name in names}
            warm = 32 if width*height*channels <= 65536 else 4
            for trial in range(args.trials):
                workers = {}
                try:
                    for name in names[trial:] + names[:trial]:
                        output = args.out/f'{case}.{trial}.{name}.raw'
                        command = ([rust, source, expected, output, width, height, channels] if name == 'rust' else
                                   [native, name, private, source, expected, output, width, height, channels])
                        workers[name] = Worker(command, mask, args.out/f'{case}.{trial}.{name}.stderr')
                        if output.read_bytes() != expected.read_bytes():
                            raise RuntimeError('Initial output mismatch')
                    batches = {name: min(2048, max(1, round(20000/workers[name].batch(1, warm)))) for name in names}
                    values = {name: [] for name in names}
                    for round_index in range(6):
                        order = orders[(round_index+trial) % 6]
                        for name in order:
                            us = workers[name].batch(batches[name], warm)
                            values[name].append(us)
                            raw.append({'case': case, 'trial': trial, 'round': round_index, 'order': order,
                                        'actor': name, 'batch': batches[name], 'warm': warm, 'us': us})
                    for name in names:
                        medians[name].append(statistics.median(values[name]))
                finally:
                    for worker in workers.values():
                        worker.close()
                print(f'{case}, trial {trial+1}/{args.trials}: all pixels match', flush=True)
                (args.out/'rounds.json').write_text(json.dumps(raw, indent=2)+'\n')
            result = {'case': case, 'width': width, 'height': height, 'components': channels,
                      'input_bytes': source.stat().st_size, 'fixture_sha256': digest(source), 'pixels_sha256': digest(expected),
                      'trial_medians_us': medians, 'median_us': {n: statistics.median(medians[n]) for n in names}}
            result['speedup_over_kdu'] = {n: result['median_us']['kdu']/result['median_us'][n] for n in ('cpp', 'rust')}
            summary.append(result)
            (args.out/'summary.json').write_text(json.dumps(summary, indent=2)+'\n')
            print(json.dumps(result['median_us']), flush=True)
    (args.out/'manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')
    print(f'Completed comparison: {args.out}')


if __name__ == '__main__':
    main()
