// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
use std::time::{Instant,Duration};
macro_rules! module{($m:ident,$k:ident)=>{mod $m{use $k as neo_j2k; include!("suite.rs");}};}
module!(base,neo_j2k_old);module!(candidate,neo_j2k_new);module!(control,neo_j2k_control);
fn median(mut x:Vec<f64>)->f64{x.sort_by(f64::total_cmp);x[x.len()/2]}
fn timings(mut calls:[&mut dyn FnMut();3],batch:u32)->[f64;6]{
 let mut times:[Vec<f64>;3]=std::array::from_fn(|_|Vec::new());let mut gains=Vec::new();let mut controls=Vec::new();let mut new_base=Vec::new();
 let orders=[[0,1,2],[2,1,0],[1,0,2],[2,0,1],[0,2,1],[1,2,0]];
 for n in 0..24{let mut t=[0.;3];for k in orders[n%6]{let begin=Instant::now();for _ in 0..batch{calls[k]();}t[k]=begin.elapsed().as_secs_f64()*1e6/f64::from(batch);times[k].push(t[k]);}gains.push((t[1]/t[2]-1.)*100.);new_base.push((t[1]/t[0]-1.)*100.);controls.push((t[2]/t[0]-1.)*100.);}
 [median(std::mem::take(&mut times[0])),median(std::mem::take(&mut times[1])),median(std::mem::take(&mut times[2])),median(gains),median(new_base),median(controls)]
}
fn decode(name:&str,reduce:u32,cut:bool,width:usize){
 let mut bytes=std::fs::read(format!("{}/../corpus/generated/{name}",env!("CARGO_MANIFEST_DIR"))).unwrap();if cut{bytes.truncate(bytes.len()-2);}
 let expected_cpp=if name.starts_with('p') {
 let mut codec=neo_j2k_oracle::Codec::new(neo_j2k_oracle::Format::J2k,reduce,0).unwrap();
 let (status,samples)=codec.decode_i32(&bytes);assert_eq!(status,0,"precision C++ decode");Some(samples)
 }else{None};
 let pool=neo_j2k_oracle::BenchmarkPool::new(width.try_into().unwrap()).unwrap();
 base::decode(&bytes,reduce,&pool,|a,ao|candidate::decode(&bytes,reduce,&pool,|b,bo|control::decode(&bytes,reduce,&pool,|c,co|{
  assert_eq!(ao,bo);assert_eq!(ao,co);if let Some(cpp)=&expected_cpp{assert_eq!(&ao,cpp,"C++ precision samples");}let t=timings([a,b,c],if width==1&&!name.contains("large"){32}else{2});println!("decode|{name}|r{reduce}|cut{cut}|w{width}|{:.4}|{:.4}|{:.4}|{:+.3}%|{:+.3}%|{:+.3}%|same",t[0],t[1],t[2],t[3],t[4],t[5]);
 })));
}

fn packed(name:&str,reduce:u32,channels:usize){
 let bytes=std::fs::read(format!("{}/../corpus/generated/{name}",env!("CARGO_MANIFEST_DIR"))).unwrap();
 base::packed(&bytes,reduce,channels,|a,ao|candidate::packed(&bytes,reduce,channels,|b,bo|control::packed(&bytes,reduce,channels,|c,co|{assert_eq!(ao,bo);assert_eq!(ao,co);let t=timings([a,b,c],16);println!("packed|{name}|r{reduce}|channels{channels}|{:.4}|{:.4}|{:.4}|{:+.3}%|{:+.3}%|{:+.3}%|same",t[0],t[1],t[2],t[3],t[4],t[5]);})));
}

fn encode_one(side:usize,reversible:bool){let pixels=base::pixels(side);
 base::encode(&pixels,side as u32,reversible,|a,ao|candidate::encode(&pixels,side as u32,reversible,|b,bo|control::encode(&pixels,side as u32,reversible,|c,co|{
 assert_eq!(ao,bo);assert_eq!(ao,co);let t=timings([a,b,c],32);println!("encode|{side}|reversible{reversible}|{:.4}|{:.4}|{:.4}|{:+.3}%|{:+.3}%|{:+.3}%|same",t[0],t[1],t[2],t[3],t[4],t[5]);})));
}
fn main(){
 println!("DIAGNOSTIC: third slot is old-safe INLINE scatter, NOT unchanged original control; percent columns new/old_safe,new/reference,old_safe/reference");
 if std::env::var_os("SAFE_AUDIT_PACKED_REDUCED").is_some(){
  for channels in 1..=4{packed("sl_128_rgba_97.j2k",2,channels);}return;
 }

 if std::env::var_os("SAFE_AUDIT_TAIL_PRECISION").is_some(){
  for source in ["p31_size_17x37_rgb_97.j2k","p31_size_2x2_rgba_97.j2k","p31_size_512x16_rgb_97_r5.j2k"]{
   for reduce in [0,1]{decode(source,reduce,false,1);}
  }return;
 }

 if std::env::var_os("SAFE_AUDIT_PRECISION").is_some(){
  for precision in [24,31]{for source in ["sl_64_rgb_97.j2k","sl_128_rgba_97.j2k","large_1024_rgb_97.j2k"]{
   let name=format!("p{precision}_{source}");decode(&name,0,false,1);
  }}return;
 }

 if std::env::var_os("SAFE_AUDIT_RESIDUAL").is_some(){
  decode("sl_64_rgb_97.j2k",0,false,1);decode("sl_128_rgba_97.j2k",0,false,1);
  decode("sl_128_rgba_97.j2k",2,false,1);
  packed("sl_128_rgba_97.j2k",0,3);packed("sl_128_rgba_97.j2k",0,4);
  encode_one(32,true);encode_one(32,false);return;
 }

 println!("kind|fixture|request|reference_us|new_us|old_safe_us|new_vs_old_safe|new_vs_reference|old_safe_vs_reference|equality");
 for name in ["large_1024_rgb_97.j2k","large_4096_g_97.j2k"]{for w in [1,2,4,8]{decode(name,0,false,w);}}
 for name in ["sl_32_rgba_53.j2k","sl_64_rgb_97.j2k","sl_128_rgba_97.j2k"]{for (r,cut) in [(0,false),(2,false),(0,true)]{decode(name,r,cut,1);}}
 for side in [32,64,128]{let pixels=base::pixels(side);for reversible in [true,false]{
  base::encode(&pixels,side as u32,reversible,|a,ao|candidate::encode(&pixels,side as u32,reversible,|b,bo|control::encode(&pixels,side as u32,reversible,|c,co|{
   assert_eq!(ao,bo);assert_eq!(ao,co);let t=timings([a,b,c],16);println!("encode|{side}|reversible{reversible}|{:.4}|{:.4}|{:.4}|{:+.3}%|{:+.3}%|{:+.3}%|same",t[0],t[1],t[2],t[3],t[4],t[5]);
  })));
 }}
 for reduce in [0,2]{for channels in 1..=4{packed("sl_128_rgba_97.j2k",reduce,channels);}}
 packed("large_1024_rgb_97.j2k",0,3);

}
