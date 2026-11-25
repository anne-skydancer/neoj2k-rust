from pathlib import Path
import ctypes, hashlib, json, os, platform, statistics, subprocess, time

root=Path('C:/Users/annes/source/repos/neoj2k-rust')
work=Path(__file__).parent
out=root/'target-main-rebuild-results'
out.mkdir(exist_ok=True)
fresh=root/'target-main-rebuild-three-way/release/neo-j2k-three-way.exe'
paired=work/'main-regression/target/release/main-regression.exe'
kernel=ctypes.WinDLL('kernel32',use_last_error=True)
kernel.GetCurrentProcess.restype=ctypes.c_void_p
kernel.GetProcessAffinityMask.argtypes=[ctypes.c_void_p,ctypes.POINTER(ctypes.c_size_t),ctypes.POINTER(ctypes.c_size_t)]
kernel.SetProcessAffinityMask.argtypes=[ctypes.c_void_p,ctypes.c_size_t]
allowed,system=ctypes.c_size_t(),ctypes.c_size_t()
assert kernel.GetProcessAffinityMask(kernel.GetCurrentProcess(),ctypes.byref(allowed),ctypes.byref(system))
pin=allowed.value & -allowed.value
manifest={'date_utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),'platform':platform.platform(),
 'baseline_commit':'3000f7d127222734e18de9cd69d05d475e01cb6e',
 'main_commit':subprocess.check_output(['git','-C',str(root),'rev-parse','HEAD'],text=True).strip(),
 'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'allowed_cpu_mask':allowed.value,'fresh_cpu_mask':pin,
 'exe_sha256':{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [paired,fresh]},
 'scope':{'paired':'23 retained-decoder/ fresh-encoder requests from tools/rust-versions, baseline vs merged main, alternating, process start order rotated; 3 processes; default allowed CPU mask for host executor widths 1/2/4/8',
 'fresh':'12 fresh single-thread I32 decode/owned encode requests, C++/Rust/OpenJPEG, 3 processes, all six actor orders per process; original bench_three_way executable on Windows MSVC, pinned to first allowed CPU'},
 'baseline_source_inventory':json.loads((work/'main-regression/baseline-reference/oracle-source.json').read_text()),
 'main_source_sha256':{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in (root/'neo-j2k/src').rglob('*.rs')}}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2))

def run(exe,args,path,pinned,env):
    with path.open('w',encoding='utf-8') as log:
        process=subprocess.Popen([str(exe),*map(str,args)],cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
        if pinned:
            assert kernel.SetProcessAffinityMask(int(process._handle),pin)
        code=process.wait()
    if code:raise RuntimeError(f'{exe.name} failed ({code}); see {path}')

env=dict(os.environ,NJ2K_THREE_WAY_CORPUS=str(root/'corpus/generated'))
for rep in range(3):
    start=time.monotonic()
    file=out/f'paired-{rep+1}.txt'
    run(paired,[rep],file,False,env)
    text=file.read_text()
    if 'DIFFERENT' in text or 'harness result: ok' not in text:raise RuntimeError('paired validation failed')
    print(f'Paired 23-request process {rep+1}/3 complete ({time.monotonic()-start:.1f}s)',flush=True)

records=[]
for rep in range(3):
    start=time.monotonic()
    file=out/f'fresh-{rep+1}.jsonl'
    run(fresh,[rep],file,True,env)
    rows=[json.loads(s) for s in file.read_text().splitlines()]
    assert sum(r['type']=='check' and r['equal'] for r in rows)==12
    records.extend(rows)
    print(f'Fresh 12-request three-way process {rep+1}/3 complete ({time.monotonic()-start:.1f}s)',flush=True)
summary={}
for r in records:
    if r['type']=='result':summary.setdefault(r['case'],{}).setdefault(r['codec'],[]).append(r['us'])
for codecs in summary.values():
    for codec,values in list(codecs.items()):codecs[codec]={'median_us':statistics.median(values),'process_us':values}
(out/'fresh-summary.json').write_text(json.dumps(summary,indent=2))
print('Benchmark runs completed; raw rounds and source provenance retained.',flush=True)
