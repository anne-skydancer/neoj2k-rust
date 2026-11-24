
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
fn convert_p(bs:&[AuditBlock])->Vec<packed::bench::AuditBlock>{bs.iter().map(|b|packed::bench::AuditBlock{meta:b.meta,segments:b.segments.clone(),chunk_lengths:b.chunk_lengths.clone(),bytes:b.bytes.clone()}).collect()}
fn main(){let args:Vec<_>=std::env::args().collect();let root=std::path::Path::new(&args[1]);let rounds=24usize;let repeats=128usize;
let mut orders=vec![];for a in 0..4{for b in 0..4{for c in 0..4{for d in 0..4{if a!=b&&a!=c&&a!=d&&b!=c&&b!=d&&c!=d{orders.push([a,b,c,d]);}}}}}assert_eq!(orders.len(),24);
for name in ["sl_32_rgba_53.j2k","sl_64_rgb_97.j2k","sl_128_rgba_97.j2k"]{let bs=load(&root.join(format!("{name}.r2.bin")));let mut groups=vec![("all".to_string(),(0..bs.len()).collect::<Vec<_>>())];for side in [1,2,4,8,16]{let is:Vec<_>=bs.iter().enumerate().filter(|(_,b)|b.meta[0]==side&&b.meta[1]==side&&!b.bytes.is_empty()).map(|(i,_)|i).collect();if!is.is_empty(){groups.push((format!("{side}x{side}"),is));}}
for contiguous in [false,true]{let chunks=if contiguous{"joined"}else{"original"};let mut rust=AuditWorkspace::new(bs.clone(),contiguous);let mut ctrl=control::bench::AuditWorkspace::new(convert(&bs),contiguous);let mut candidate=packed::bench::AuditWorkspace::new(convert_p(&bs),contiguous);let mut cpp=Cpp::new(&bs,contiguous);let all:Vec<_>=(0..bs.len()).collect();
for mode in 0..3{rust.run(&all,4,mode);ctrl.run(&all,4,mode);candidate.run(&all,4,mode);cpp.run(&all,4,mode);for i in 0..bs.len(){assert_eq!(rust.output(i),cpp.output(i));assert_eq!(rust.output(i),ctrl.output(i));assert_eq!(rust.output(i),candidate.output(i));}}
COUNT.store(true,Relaxed);ALLOCS.store(0,Relaxed);candidate.run(&all,2,0);let allocs=ALLOCS.load(Relaxed);COUNT.store(false,Relaxed);assert_eq!(allocs,0);println!("ALLOC_PACKED {name} {chunks} {allocs}");
for (group,indices) in&groups{for mode in [0u32,1]{let mut times:[Vec<f64>;4]=std::array::from_fn(|_|vec![]);
for round in 0..rounds{for algorithm in orders[round]{let started=Instant::now();match algorithm{0=>rust.run(black_box(indices),repeats,mode),1=>cpp.run(black_box(indices),repeats,mode),2=>ctrl.run(black_box(indices),repeats,mode),_=>candidate.run(black_box(indices),repeats,mode)}times[algorithm].push(started.elapsed().as_secs_f64()*1e6/repeats as f64);}}
let[r,c,v,p]=times.map(med);println!("PACKED {name} {chunks} {group} mode{mode} Rust {r:.4} Cpp {c:.4} Control {v:.4} Packed {p:.4} packed_change {:+.2}% packed_vs_cpp {:+.2}% control {:+.2}%",100.0*(p/r-1.0),100.0*(p/c-1.0),100.0*(v/r-1.0));}}
for i in 0..bs.len(){assert_eq!(rust.output(i),candidate.output(i));assert_eq!(rust.output(i),cpp.output(i));}
}}
}
