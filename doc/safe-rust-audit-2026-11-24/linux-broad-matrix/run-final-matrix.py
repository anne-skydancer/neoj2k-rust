from pathlib import Path
import subprocess,os,time,json,hashlib
r=Path('/home/dev/safe-crate-audit')
records=[]
for cell in ['baseline','v3']:
 exe=r/f'target-{cell}/release/neo-j2k-versions'
 for n in range(1,4):
  for mode,key in [('full',None),('precision','SAFE_AUDIT_PRECISION')]:
   path=r/f'out/scatter-typed-final-{cell}-{mode}-{n}.txt'
   print(f'START {cell} {mode} {n}',flush=True);begin=time.monotonic()
   env=dict(os.environ)
   for k in ['SAFE_AUDIT_PRECISION','SAFE_AUDIT_TAIL_PRECISION','SAFE_AUDIT_RESIDUAL','SAFE_AUDIT_PACKED_REDUCED']:env.pop(k,None)
   if key:env[key]='1'
   args=[str(exe)] if key is None else ['taskset','-c','7',str(exe)]
   with path.open('w') as out: result=subprocess.run(args,cwd=r,env=env,stdout=out,stderr=subprocess.STDOUT,timeout=120)
   elapsed=time.monotonic()-begin;lines=path.read_text().splitlines();count=sum(line.startswith(('decode|','encode|','packed|'))for line in lines)
   assert result.returncode==0,(cell,mode,n,result.returncode)
   assert count==(32 if mode=='full' else 6),(cell,mode,n,count)
   records.append(dict(cell=cell,mode=mode,run=n,host_seconds=elapsed,rows=count,exe_sha256=hashlib.sha256(exe.read_bytes()).hexdigest()))
   (r/'out/scatter-typed-final-summary.json').write_text(json.dumps(records,indent=2))
   print(f'DONE {cell} {mode} {n}: {count} rows exact, {elapsed:.2f}s',flush=True)
