#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
"""Linux three-way README benchmark; source-built libraries, one fixed executable."""
import argparse,hashlib,json,os,platform,statistics,subprocess,time
from pathlib import Path

ap=argparse.ArgumentParser(description=__doc__)
ap.add_argument('--cpp-source',type=Path,required=True)
ap.add_argument('--out',type=Path,required=True)
ap.add_argument('--repetitions',type=int,default=3)
ap.add_argument('--openjpeg-source', type=Path, help='Clean upstream OpenJPEG 2.5.4 checkout')
args=ap.parse_args()
assert args.repetitions>=3
root=Path(__file__).resolve().parents[2]
cpp=args.cpp_source.resolve();out=args.out.resolve();out.mkdir(parents=True,exist_ok=True)
project=root/'tools/bench_three_way'
env=dict(os.environ);env.pop('CARGO_ENCODED_RUSTFLAGS',None)
env['NJ2K_THREE_WAY_GENERATED'] = str(out/'generated-fixtures')
env.update(RUSTC_WRAPPER='',RUSTFLAGS='-C target-cpu=x86-64-v3',CARGO_TARGET_DIR=str(out/'rust-target'),NJ2K_ORACLE_DIR=str(out/'cpp-release'),NJ2K_ORACLE_CONFIG='Release',NJ2K_ORACLE_NO_BUILD='0',NJ2K_THREE_WAY_CORPUS=str(root/'corpus/generated'))

def run(cmd,log,cwd=root):
    with (out/log).open('w')as f:subprocess.run([str(x)for x in cmd],cwd=cwd,env=env,stdout=f,stderr=subprocess.STDOUT,check=True)
def output(cmd,cwd=root):return subprocess.check_output(cmd,cwd=cwd,text=True).strip()
def git_revision(folder):return output(['git','rev-parse','HEAD'],folder)
def digest(file):return hashlib.sha256(file.read_bytes()).hexdigest()
start=time.monotonic()
run(['cmake','-S',cpp,'-B',out/'cpp-release','-G','Ninja','-DCMAKE_BUILD_TYPE=Release','-DCMAKE_C_FLAGS=-march=x86-64-v3','-DCMAKE_CXX_FLAGS=-march=x86-64-v3','-DCMAKE_EXPORT_COMPILE_COMMANDS=ON','-DNJ2K_SIMD=AVX2','-DNJ2K_BUILD_TESTS=OFF','-DNJ2K_BUILD_TOOLS=OFF','-DNJ2K_BUILD_BENCH=OFF','-DNJ2K_BUILD_SHARED=OFF','-DNJ2K_BUILD_RUST_SHIM=ON','-DNJ2K_RUST_SHIM_SOURCE='+str(root/'neo-j2k-oracle/shim'),'-DNJ2K_INSTALL=OFF'],'cmake-configure.txt')
run(['cmake','--build',out/'cpp-release','--target','nj2k_rust_shim','-j','8'],'cmake-build.txt')
print('C++ neo-j2k and OpenJPEG Release builds passed.',flush=True)

# Keep the correctness oracle intact. Only the timed OpenJPEG adapter links this
# independent upstream library, whose symbols are isolated from the patched oracle.
upstream = args.openjpeg_source.resolve() if args.openjpeg_source else out/'openjpeg-source'
if not args.openjpeg_source:
    run(['git', '-c', 'advice.detachedHead=false', 'clone', '--quiet', '--depth', '1', '--branch', 'v2.5.4',
         'https://github.com/uclouvain/openjpeg.git', upstream], 'upstream-clone.txt')
upstream_revision = git_revision(upstream)
assert upstream_revision == '6c4a29b00211eb0430fa0e5e890f1ce5c80f409f'
assert not output(['git', 'status', '--porcelain'], upstream)
upstream_build = out/'openjpeg-build'
run(['cmake', '-S', upstream, '-B', upstream_build, '-G', 'Ninja', '-DCMAKE_BUILD_TYPE=Release',
     '-DBUILD_SHARED_LIBS=OFF', '-DBUILD_CODEC=OFF', '-DBUILD_TESTING=OFF',
     '-DCMAKE_C_FLAGS=-march=x86-64-v3 -ffp-contract=off -fno-fast-math'], 'upstream-configure.txt')
run(['cmake', '--build', upstream_build, '--target', 'openjp2', '--parallel', '8'], 'upstream-build.txt')
original_library = upstream_build/'bin/libopenjp2.a'
symbols = sorted({line.split()[-1] for line in output(['nm', '-g', '--defined-only', str(original_library)]).splitlines()
                  if len(line.split()) >= 3 and line.split()[-1].isidentifier()})
assert 'opj_decode' in symbols and 'opj_encode' in symbols
rename_map = out/'upstream-symbols.txt'
rename_map.write_text(''.join(f'{name} nj2k_upstream_{name}\n' for name in symbols))
rename_header = out/'upstream-symbols.h'
rename_header.write_text(''.join(f'#define {name} nj2k_upstream_{name}\n' for name in symbols))
upstream_library = out/'libopenjp2_upstream.a'
run(['objcopy', '--redefine-syms='+str(rename_map), original_library, upstream_library], 'upstream-isolate.txt')
adapter = out/'shim_opj.cpp.o'
run(['c++', '-std=c++17', '-O3', '-march=x86-64-v3', '-ffp-contract=off', '-fno-fast-math',
     '-include', rename_header, '-I'+str(upstream/'src/lib/openjp2'),
     '-I'+str(upstream_build/'src/lib/openjp2'), '-c', root/'neo-j2k-oracle/shim/shim_opj.cpp',
     '-o', adapter], 'upstream-adapter.txt')
shim = out/'cpp-release/rust-shim/libnj2k_rust_shim.a'
assert 'shim_opj.cpp.o' in output(['ar', 't', str(shim)]).splitlines()
run(['ar', 'r', shim, adapter], 'upstream-link.txt')
link_manifest = out/'cpp-release/nj2k-rust-shim-Release.txt'
manifest_text = link_manifest.read_text()
upstream_link = 'lib '+str(upstream_library)+'\n'
if upstream_link not in manifest_text:
    link_manifest.write_text(manifest_text + upstream_link)
print('Timed OpenJPEG adapter links unmodified upstream 2.5.4 with isolated symbols.', flush=True)
run(['cargo','+1.100.0','generate-lockfile','--offline'],'cargo-lock.txt',project)
run(['cargo','+1.100.0','build','--release','--locked','--offline','-vv'],'cargo-build.txt',project)
run(['cargo','+1.100.0','tree','--locked','--offline','-p','neo-j2k','-e','normal'],'production-tree.txt',project)
run(['cargo','+1.100.0','tree','--locked','--offline','-e','features','-i','neo-j2k'],'feature-tree.txt',project)
assert len((out/'production-tree.txt').read_text().strip().splitlines())==1
assert 'bench-internals'not in(out/'feature-tree.txt').read_text()
assert 'target-cpu=x86-64-v3'in(out/'cargo-build.txt').read_text()
exe=out/'rust-target/release/neo-j2k-three-way';exe_hash=digest(exe)
print('One original Rust package built; default features, no stage timers.',flush=True)
cpu=min(os.sched_getaffinity(0))
records=[]
for rep in range(args.repetitions):
    assert digest(exe)==exe_hash
    t=time.monotonic();name=f'rep-{rep+1}.jsonl'
    run(['taskset','-c',str(cpu),exe,str(rep)],name,project)
    rows=[json.loads(line)for line in(out/name).read_text().splitlines()]
    assert sum(r['type']=='result'for r in rows)==63
    assert sum(r['type']=='check'and r['equal']for r in rows)==21
    records.extend(rows)
    print(f'Repetition {rep+1}/{args.repetitions}:21 exact three-way cases in {time.monotonic()-t:.1f}s.',flush=True)
summary={}
for r in records:
    if r['type']=='result':summary.setdefault(r['case'],{}).setdefault(r['codec'],[]).append(r['us'])
for codecs in summary.values():
    for codec,values in list(codecs.items()):codecs[codec]={'median_us':statistics.median(values),'process_us':values,'range_us':[min(values),max(values)]}
(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
manifest={'date_utc':time.strftime('%Y-%m-%d',time.gmtime()),'rust_commit':git_revision(root),'cpp_commit':git_revision(cpp),'openjpeg_pin':(cpp/'oracle/PIN').read_text(),'rust_compiler':output(['rustc','+1.100.0','-Vv']),'c_compiler':output(['cc','--version']),'cpp_compiler':output(['c++','--version']),'system':platform.platform(),'lscpu':json.loads(output(['lscpu','-J'])),'cpu_affinity':cpu,'protocol':{'rounds':12,'prewarm_per_actor_batch':32,'repetitions':args.repetitions,'single_thread':True,'fresh_codecs':True,'actor_order':'all6 permutations twice per process, shifted by repetition; output buffer allocation rotated'},'exe_sha256':exe_hash,'source_sha256':{str(f.relative_to(root)):digest(f)for f in sorted((root/'neo-j2k/src').rglob('*.rs'))},'driver_sha256':{str(f.relative_to(project)):digest(f)for f in[project/'Cargo.toml',project/'Cargo.lock',project/'src/main.rs',project/'run.py']},'fixture_sha256':{f.name:digest(f)for f in(root/'corpus/generated').glob('*.j2k')},'compile_commands_sha256':digest(out/'cpp-release/compile_commands.json'),'library_sha256':{str(f.relative_to(out/'cpp-release')):digest(f)for f in(out/'cpp-release').rglob('*.a')},'elapsed_s':time.monotonic()-start}
manifest['timed_openjpeg'] = {'repository': 'https://github.com/uclouvain/openjpeg',
    'commit': upstream_revision, 'version': '2.5.4', 'library_sha256': digest(upstream_library),
    'adapter_sha256': digest(adapter), 'symbol_map_sha256': digest(rename_map),
    'strictness': 'explicit per decode; encode has no strictness setting'}
manifest['generated_fixture_sha256'] = {p.name: digest(p) for p in (out/'generated-fixtures').glob('*.j2k')}
manifest['test_adapter_source_sha256'] = {str(p.relative_to(root)): digest(p) for p in [
    root/'neo-j2k-oracle/shim/shim_opj.cpp', root/'neo-j2k-oracle/shim/opj_shim.h',
    root/'neo-j2k-oracle/src/openjpeg.rs', root/'neo-j2k-oracle/src/ffi.rs']}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('Saved summary.json, raw rounds and source/build manifest.',flush=True)
