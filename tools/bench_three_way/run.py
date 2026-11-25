#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
"""Linux three-way README benchmark; source-built libraries, one fixed executable."""
import argparse,hashlib,json,os,platform,statistics,subprocess,time
from pathlib import Path

ap=argparse.ArgumentParser(description=__doc__)
ap.add_argument('--cpp-source',type=Path,required=True)
ap.add_argument('--out',type=Path,required=True)
ap.add_argument('--repetitions',type=int,default=3)
args=ap.parse_args()
assert args.repetitions>=3
root=Path(__file__).resolve().parents[2]
cpp=args.cpp_source.resolve();out=args.out.resolve();out.mkdir(parents=True,exist_ok=True)
project=root/'tools/bench_three_way'
env=dict(os.environ);env.pop('CARGO_ENCODED_RUSTFLAGS',None)
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
    assert sum(r['type']=='result'for r in rows)==36
    assert sum(r['type']=='check'and r['equal']for r in rows)==12
    records.extend(rows)
    print(f'Repetition {rep+1}/{args.repetitions}:12 exact three-way cases in {time.monotonic()-t:.1f}s.',flush=True)
summary={}
for r in records:
    if r['type']=='result':summary.setdefault(r['case'],{}).setdefault(r['codec'],[]).append(r['us'])
for codecs in summary.values():
    for codec,values in list(codecs.items()):codecs[codec]={'median_us':statistics.median(values),'process_us':values,'range_us':[min(values),max(values)]}
(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
manifest={'date_utc':time.strftime('%Y-%m-%d',time.gmtime()),'rust_commit':git_revision(root),'cpp_commit':git_revision(cpp),'openjpeg_pin':(cpp/'oracle/PIN').read_text(),'rust_compiler':output(['rustc','+1.100.0','-Vv']),'c_compiler':output(['cc','--version']),'cpp_compiler':output(['c++','--version']),'system':platform.platform(),'lscpu':json.loads(output(['lscpu','-J'])),'cpu_affinity':cpu,'protocol':{'rounds':12,'prewarm_per_actor_batch':32,'repetitions':args.repetitions,'single_thread':True,'fresh_codecs':True,'actor_order':'all6 permutations twice per process, shifted by repetition; output buffer allocation rotated'},'exe_sha256':exe_hash,'source_sha256':{str(f.relative_to(root)):digest(f)for f in sorted((root/'neo-j2k/src').rglob('*.rs'))},'driver_sha256':{str(f.relative_to(project)):digest(f)for f in[project/'Cargo.toml',project/'Cargo.lock',project/'src/main.rs',project/'run.py']},'fixture_sha256':{f.name:digest(f)for f in(root/'corpus/generated').glob('*.j2k')},'compile_commands_sha256':digest(out/'cpp-release/compile_commands.json'),'library_sha256':{str(f.relative_to(out/'cpp-release')):digest(f)for f in(out/'cpp-release').rglob('*.a')},'elapsed_s':time.monotonic()-start}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('Saved summary.json, raw rounds and source/build manifest.',flush=True)
