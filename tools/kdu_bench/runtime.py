#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
"""Isolated runtime feasibility probe; this does not measure image decoding."""
import argparse
import json
from pathlib import Path
import shutil
import struct
import subprocess
import tempfile

from probe import PE


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--viewer', required=True, type=Path)
    parser.add_argument('--dependencies', required=True, type=Path,
                        help='Directory containing root DLLs extracted from the same installer')
    parser.add_argument('--probe', required=True, type=Path,
                        help='Built kdu_runtime_probe.exe')
    parser.add_argument('--json', type=Path)
    args = parser.parse_args()
    pe = PE(args.viewer)
    result = pe.inspect()
    with tempfile.TemporaryDirectory(prefix='kdu-runtime-probe-') as directory:
        folder = Path(directory)
        for dependency in args.dependencies.glob('*.dll'):
            shutil.copy2(dependency, folder / dependency.name)
        image = bytearray(pe.data)
        struct.pack_into('<H', image, pe.pe + 22, pe.u16(pe.pe + 22) | 0x2000)
        struct.pack_into('<I', image, pe.opt + 16, 0)
        target = folder / 'kdu-embedded.dll'
        target.write_bytes(image)
        run = subprocess.run([str(args.probe.resolve()), str(target)],
                             capture_output=True, text=True, timeout=20,
                             creationflags=getattr(subprocess, 'CREATE_NO_WINDOW', 0))
        if run.returncode:
            raise RuntimeError(f'Runtime child failed ({run.returncode}): {run.stderr}')
        result.update(runtime=json.loads(run.stdout), private_copy_only=True,
                      imports_resolved=True, tls_callbacks_may_run=True,
                      decode_validated=False, benchmark_available=False)
    encoded = json.dumps(result, indent=2)
    if args.json:
        args.json.write_text(encoded + '\n')
    print(encoded)


if __name__ == '__main__':
    main()
