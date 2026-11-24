
use codec::bench::{AuditBlock,AuditWorkspace};
use std::ffi::c_void;
use std::hint::black_box;
use std::time::Instant;
use std::alloc::{GlobalAlloc,Layout,System};
use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering::Relaxed};
static COUNT:AtomicBool=AtomicBool::new(false);static ALLOCS:AtomicUsize=AtomicUsize::new(0);
struct Counter;
unsafe impl GlobalAlloc for Counter { unsafe fn alloc(&self,l:Layout)->*mut u8{if COUNT.load(Relaxed){ALLOCS.fetch_add(1,Relaxed);}unsafe{System.alloc(l)}} unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{if COUNT.load(Relaxed){ALLOCS.fetch_add(1,Relaxed);}unsafe{System.alloc_zeroed(l)}} unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{if COUNT.load(Relaxed){ALLOCS.fetch_add(1,Relaxed);}unsafe{System.realloc(p,l,n)}} unsafe fn dealloc(&self,p:*mut u8,l:Layout){unsafe{System.dealloc(p,l)}} }
#[global_allocator]static A:Counter=Counter;
#[repr(C)]struct Request{w:u32,h:u32,numbps:u32,orient:u32,roi:u32,style:u32,lens:*const u32,passes:*const u32,nsegs:usize,bytes:*const u8,chunks:*const u32,nchunks:usize,rev:u32,step:f32,stride:u32,offset:u32}
const _:()=assert!(std::mem::size_of::<Request>()==88);
unsafe extern "C" {fn nj2k_audit_prepare(r:*const Request,n:usize)->*mut c_void;fn nj2k_audit_free(h:*mut c_void);fn nj2k_audit_run(h:*mut c_void,indices:*const usize,n:usize,reps:usize,mode:u32)->i32;fn nj2k_audit_output_len(h:*mut c_void,index:usize)->usize;fn nj2k_audit_copy_output(h:*mut c_void,index:usize,out:*mut i32,cap:usize)->usize;}
struct Cpp(*mut c_void);
impl Drop for Cpp{fn drop(&mut self){unsafe{nj2k_audit_free(self.0)}}}
impl Cpp{fn new(bs:&[AuditBlock],contiguous:bool)->Self{
let lens:Vec<Vec<u32>>=bs.iter().map(|b|b.segments.iter().map(|s|s.0).collect()).collect();let passes:Vec<Vec<u32>>=bs.iter().map(|b|b.segments.iter().map(|s|s.1).collect()).collect();let chunks:Vec<Vec<u32>>=bs.iter().map(|b|if contiguous{if b.bytes.is_empty(){vec![]}else{vec![b.bytes.len() as u32]}}else{b.chunk_lengths.clone()}).collect();
let reqs:Vec<_>=bs.iter().enumerate().map(|(i,b)|{let[w,h,numbps,orient,roi,style,rev,step,stride,offset]=b.meta;Request{w,h,numbps,orient,roi,style,lens:lens[i].as_ptr(),passes:passes[i].as_ptr(),nsegs:lens[i].len(),bytes:b.bytes.as_ptr(),chunks:chunks[i].as_ptr(),nchunks:chunks[i].len(),rev,step:f32::from_bits(step),stride,offset}}).collect();let p=unsafe{nj2k_audit_prepare(reqs.as_ptr(),reqs.len())};assert!(!p.is_null());Self(p)}
fn run(&mut self,is:&[usize],reps:usize,mode:u32){assert_eq!(unsafe{nj2k_audit_run(self.0,is.as_ptr(),is.len(),reps,mode)},1);}
fn output(&self,i:usize)->Vec<i32>{let n=unsafe{nj2k_audit_output_len(self.0,i)};let mut v=vec![0;n];assert_eq!(unsafe{nj2k_audit_copy_output(self.0,i,v.as_mut_ptr(),v.len())},n);v}}
fn load(path:&std::path::Path)->Vec<AuditBlock>{let data=std::fs::read(path).unwrap();assert_eq!(&data[..8],b"NJDA1\0\0\0");let mut at=8;fn u(data:&[u8],at:&mut usize)->u32{let n=u32::from_le_bytes(data[*at..*at+4].try_into().unwrap());*at+=4;n}let count=u(&data,&mut at);let mut out=vec![];for _ in 0..count{let meta=std::array::from_fn(|_|u(&data,&mut at));let ns=u(&data,&mut at);let nc=u(&data,&mut at);let segments=(0..ns).map(|_|(u(&data,&mut at),u(&data,&mut at))).collect();let chunk_lengths:Vec<_>=(0..nc).map(|_|u(&data,&mut at)).collect();let n=chunk_lengths.iter().map(|&x|x as usize).sum::<usize>();let bytes=data[at..at+n].to_vec();at+=n;out.push(AuditBlock{meta,segments,chunk_lengths,bytes});}assert_eq!(at,data.len());out}
fn convert(bs:&[AuditBlock])->Vec<control::bench::AuditBlock>{bs.iter().map(|b|control::bench::AuditBlock{meta:b.meta,segments:b.segments.clone(),chunk_lengths:b.chunk_lengths.clone(),bytes:b.bytes.clone()}).collect()}
fn med(mut xs:Vec<f64>)->f64{xs.sort_by(f64::total_cmp);xs[xs.len()/2]}
fn main(){let args:Vec<_>=std::env::args().collect();
if args.get(1).map(String::as_str)==Some("--spin") {
 let root=std::path::Path::new(&args[2]);let name=&args[3];let group=&args[4];let algorithm=&args[5];let seconds:f64=args[6].parse().unwrap();
 let bs=load(&root.join(format!("{name}.r2.bin")));
 let indices:Vec<usize>=bs.iter().enumerate().filter(|(_,b)| match group.as_str(){"all"=>true,"8x8"=>b.meta[0]==8&&b.meta[1]==8&&!b.bytes.is_empty(),"16x16"=>b.meta[0]==16&&b.meta[1]==16&&!b.bytes.is_empty(),_=>panic!("group")}).map(|(i,_)|i).collect();assert!(!indices.is_empty());
 let mut rust=AuditWorkspace::new(bs.clone(),false);let mut cpp=Cpp::new(&bs,false);let mut ctrl=control::bench::AuditWorkspace::new(convert(&bs),false);
 rust.run(&indices,16,0);cpp.run(&indices,16,0);ctrl.run(&indices,16,0);
 for &i in &indices {assert_eq!(rust.output(i),cpp.output(i));assert_eq!(rust.output(i),ctrl.output(i));}
 println!("SPIN_READY pid{} {algorithm} {name} {group} blocks{} seconds{seconds}",std::process::id(),indices.len());
 let start=Instant::now();let mut repeats=0usize;
 match algorithm.as_str(){"rust"=>while start.elapsed().as_secs_f64()<seconds {rust.run(black_box(&indices),1000,0);repeats+=1000;},"cpp"=>while start.elapsed().as_secs_f64()<seconds {cpp.run(black_box(&indices),1000,0);repeats+=1000;},"control"=>while start.elapsed().as_secs_f64()<seconds {ctrl.run(black_box(&indices),1000,0);repeats+=1000;},_=>panic!("algorithm")}
 println!("SPIN_DONE repeats{repeats} seconds{:.3}",start.elapsed().as_secs_f64());black_box(rust.output(indices[0]));return;
}
let root=std::path::Path::new(&args[1]);let rounds=args.get(2).map(|s|s.parse().unwrap()).unwrap_or(24usize);let reps=args.get(3).map(|s|s.parse().unwrap()).unwrap_or(128usize);
println!("rounds{rounds} repeats{reps}; Rust modes/full noalloc; C++prepared decode+store; source/control exactbyteparity");
for name in ["sl_32_rgba_53.j2k","sl_64_rgb_97.j2k","sl_128_rgba_97.j2k"]{let bs=load(&root.join(format!("{name}.r2.bin")));let mut groups:Vec<(String,Vec<usize>)>=vec![("all".into(),(0..bs.len()).collect())];
for side in [1,2,4,8,16]{let is:Vec<_>=bs.iter().enumerate().filter(|(_,b)|b.meta[0]==side&&b.meta[1]==side&&!b.bytes.is_empty()).map(|(i,_)|i).collect();if !is.is_empty(){groups.push((format!("{side}x{side}"),is));}}
for contiguous in [false,true]{let chunk_label=if contiguous{"joined"}else{"original"};let mut rust=AuditWorkspace::new(bs.clone(),contiguous);let mut ctrl=control::bench::AuditWorkspace::new(convert(&bs),contiguous);let mut cpp=Cpp::new(&bs,contiguous);
let all:Vec<_>=(0..bs.len()).collect();for mode in 0..3{rust.run(&all,4,mode);ctrl.run(&all,4,mode);cpp.run(&all,4,mode);for i in 0..bs.len(){assert_eq!(rust.output(i),cpp.output(i),"{name}block{i}mode{mode}");assert_eq!(rust.output(i),ctrl.output(i));}}
COUNT.store(true,Relaxed);ALLOCS.store(0,Relaxed);rust.run(&all,2,0);let allocations=ALLOCS.load(Relaxed);COUNT.store(false,Relaxed);assert_eq!(allocations,0);println!("ALLOC {name} {chunk_label} {allocations}");
for (group,indices) in &groups{for mode in 0..3 {let mut rv=vec![];let mut cv=vec![];let mut vv=vec![];
for round in 0..rounds{let orders=[[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];for alg in orders[round%6]{let start=Instant::now();match alg{0=>rust.run(black_box(indices),reps,mode),1=>cpp.run(black_box(indices),reps,mode),_=>ctrl.run(black_box(indices),reps,mode)};let spent=start.elapsed().as_secs_f64()*1e6/reps as f64;match alg{0=>rv.push(spent),1=>cv.push(spent),_=>vv.push(spent)}}}
let r=med(rv);let c=med(cv);let v=med(vv);println!("RESULT {name} {chunk_label} {group} mode{mode} blocks{} Rust {r:.4} Cpp {c:.4} Control {v:.4} change {:+.2}% control {:+.2}%",indices.len(),100.0*(r/c-1.0),100.0*(v/r-1.0));black_box(rust.output(indices[0]));
}}
for i in 0..bs.len(){assert_eq!(rust.output(i),cpp.output(i));assert_eq!(rust.output(i),ctrl.output(i));}
}}
}
