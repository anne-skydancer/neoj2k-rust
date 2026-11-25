import sys, json, itertools, statistics, hashlib, subprocess
from pathlib import Path
ROOT=Path('C:/Users/annes/source/repos/neoj2k-rust')
sys.path.insert(0,str(ROOT/'tools/kdu_bench'))
from compare import Worker
OUT=ROOT/'target-kdu-bench'/ (sys.argv[1] if len(sys.argv)>1 else 'wide-rgba-parity')
OUT.mkdir(parents=True,exist_ok=True)
actors={
 'baseline':Path('C:/Users/annes/Documents/Codex/2026-11-25/in/work/worker-baseline.exe'),
 'rust': ROOT/'target-kdu-bench/rust-v3/release/worker.exe',
 'cpp':ROOT/'target-kdu-bench/Release/kdu_compare_native.exe'}
orders=list(itertools.permutations(actors))
results=[]
rounds=[]
cases=[('rgba_2048x64_53',ROOT/'corpus/generated/large_2048x64_rgba_53.j2k',ROOT/'target-kdu-bench/comparison-fast/rectangle.raw',2048,64,4)]
cases += [(f'rgb_{n}_53',ROOT/f'target-kdu-bench/comparison-fast/fixtures/rgb_{n}_53.j2k',ROOT/f'target-kdu-bench/comparison-fast/fixtures/rgb_{n}_53.raw',n,n,3) for n in [32,128,512,1024]]
if len(sys.argv)>2: cases=[case for case in cases if case[0] in sys.argv[2:]]
for case,source,expected,w,h,c in cases:
 medians={n:[] for n in actors}
 for trial in range(3):
  workers={}
  try:
   names=list(actors)
   for name in names[trial:]+names[:trial]:
    output=OUT/f'{case}.{trial}.{name}.raw'
    command=[actors[name],source,expected,output,w,h,c]
    if name=='cpp':command=[actors[name],'cpp','unused',source,expected,output,w,h,c]
    workers[name]=Worker(command,1,OUT/f'{case}.{trial}.{name}.stderr')
    assert output.read_bytes()==expected.read_bytes()
   warm=32 if w*h*c<=65536 else 4
   batches={n:min(2048,max(1,round(20000/x.batch(1,warm)))) for n,x in workers.items()}
   values={n:[] for n in actors}
   for r in range(6):
    for name in orders[(r+trial)%6]:
     us=workers[name].batch(batches[name],warm)
     values[name].append(us)
     rounds.append(dict(case=case,trial=trial,round=r,actor=name,batch=batches[name],us=us))
   for name in actors:medians[name].append(statistics.median(values[name]))
   print(case,trial, {n:round(medians[n][-1],3) for n in actors},flush=True)
  finally:
   for worker in workers.values():worker.close()
 result=dict(case=case,trial_medians_us=medians,median_us={n:statistics.median(v) for n,v in medians.items()})
 results.append(result)
 (OUT/'summary.json').write_text(json.dumps(results,indent=2))
 (OUT/'rounds.json').write_text(json.dumps(rounds,indent=2))
manifest=dict(binary_sha256={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in actors.values()},diff=subprocess.check_output(['git','-C',str(ROOT),'diff'],text=True),protocol='3 fresh process trials, 6 actor orders, one actor at a time on logical CPU 0, same Worker protocol as compare.py; byte and guard validation before/after every batch')
(OUT/'manifest.json').write_text(json.dumps(manifest,indent=2))
