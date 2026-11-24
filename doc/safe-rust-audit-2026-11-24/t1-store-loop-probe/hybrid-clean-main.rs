#![allow(dead_code)]
#[path="/home/dev/safe-crate-audit/base/neo-j2k/src/simd/t1.rs"] mod reference;
#[path="/home/dev/safe-crate-audit/base/neo-j2k/src/simd/t1.rs"] mod control;
mod current {
#[path="/home/dev/safe-crate-audit/candidate/neo-j2k/src/simd/lanes.rs"] pub mod lanes;
#[path="/home/dev/safe-crate-audit/candidate/neo-j2k/src/simd/t1.rs"] pub mod kernel;
}
mod clone_current {
#[path="/home/dev/safe-crate-audit/candidate/neo-j2k/src/simd/lanes.rs"] pub mod lanes;
#[path="/home/dev/safe-crate-audit/candidate/neo-j2k/src/simd/t1.rs"] pub mod kernel;
}
mod flat {
#[path="/home/dev/safe-crate-audit/candidate/neo-j2k/src/simd/lanes.rs"] pub mod lanes;
#[path="/home/dev/safe-crate-audit/t1-store-probe/hybrid-clean.rs"] pub mod kernel;
}
use std::{hint::black_box,time::Instant};
fn main(){
 let ops:[fn(&[i32],&mut[i32],usize,usize,bool,f32)->bool;5]=[reference::store,current::kernel::store,flat::kernel::store,clone_current::kernel::store,control::store];
 for width in [8,16,31,32,64,65] {for height in [1,32] {for reversible in [false,true] {
  let stride=width+3;let prefix=5;let mut state=0x81a3_4567u32;
  let input:Vec<i32>=(0..width*height).map(|_|{state^=state<<13;state^=state>>17;state^=state<<5;state.cast_signed()}).collect();
  let pristine=vec![0x1357_2468;prefix+stride*height+16];
  for half_step in [0.37f32,0.0,-0.0,32_000_000.0] {
   let mut expected=pristine.clone();ops[0](&input,&mut expected[prefix..],width,stride,reversible,half_step);
   for op in ops {let mut actual=pristine.clone();assert!(op(&input,&mut actual[prefix..],width,stride,reversible,half_step));assert_eq!(actual,expected);}
  }
  let mut target=pristine;let mut samples:[Vec<f64>;5]=core::array::from_fn(|_|Vec::new());
  for round in 0..15 {for index in 0..5 {let which=(round+index)%5;let start=Instant::now();
   for _ in 0..1000 {black_box(ops[which](black_box(&input),black_box(&mut target[prefix..]),black_box(width),black_box(stride),black_box(reversible),black_box(0.37)));}
   samples[which].push(start.elapsed().as_secs_f64()*1e9/1000.0);
  }}
  let med=|x:&mut Vec<f64>|{x.sort_by(f64::total_cmp);x[x.len()/2]};let ns=samples.each_mut().map(med);
  println!("w{width}|h{height}|rev{reversible}|ref{:.2}|current{:.2}|hybrid{:.2}|clone{:.2}|control{:.2}|hybrid/current{:+.2}%",ns[0],ns[1],ns[2],ns[3],ns[4],100.0*(ns[2]/ns[1]-1.0));
 }}}
}
