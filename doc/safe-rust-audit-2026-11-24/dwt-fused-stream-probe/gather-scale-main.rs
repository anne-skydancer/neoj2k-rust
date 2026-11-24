mod dwt {pub const ALPHA: f32 = f32::from_bits(0xbfcb_0673);
pub const BETA: f32 = f32::from_bits(0xbd59_01ae);
pub const DELTA: f32 = f32::from_bits(0x3ee3_1355);
pub const GAMMA: f32 = f32::from_bits(0x3f62_0676);
pub const K: f32 = f32::from_bits(0x3f9d_7658);
pub const TWO_INV_K: f32 = f32::from_bits(0x3fd0_1800);}
#[path="/home/dev/safe-crate-audit/base/neo-j2k/src/simd/dwt.rs"] mod reference;
mod safe {#[path="/home/dev/safe-crate-audit/candidate/neo-j2k/src/simd/lanes.rs"] pub mod lanes; #[path="/home/dev/safe-crate-audit/candidate/neo-j2k/src/simd/dwt.rs"] pub mod kernel;}
mod barrier {#[path="/home/dev/safe-crate-audit/candidate/neo-j2k/src/simd/lanes.rs"] pub mod lanes; #[path="/home/dev/safe-crate-audit/transpose-probe/gather-then-scale.rs"] pub mod kernel;}

#[path="/home/dev/safe-crate-audit/base/neo-j2k/src/simd/dwt.rs"] mod control;
use std::time::Instant;
type Kernel=fn(&mut[i32],usize,usize,&mut[[i32;8]]);
fn timed(call:Kernel,input:&[i32],width:usize,parity:usize)->f64{
 let mut values=input.to_vec();let mut scratch=vec![[0;8];width];
 let begin=Instant::now();for _ in 0..10000 {values.copy_from_slice(input);call(std::hint::black_box(&mut values),width,parity,&mut scratch);std::hint::black_box(&values);}
 begin.elapsed().as_secs_f64()*1e6/10000.
}
fn main(){for width in [32,64,128,256,1024]{for parity in [0,1]{
 let input:Vec<_>=(0..8*width).map(|i|(((i%31)as f32)-15.).to_bits().cast_signed()).collect();
 let kernels:[Kernel;4]=[reference::inverse_97_rows,safe::kernel::inverse_97_rows,barrier::kernel::inverse_97_rows,control::inverse_97_rows];
 let outputs:Vec<_>=kernels.iter().map(|&call|{let mut data=input.clone();let mut scratch=vec![[0;8];width];call(&mut data,width,parity,&mut scratch);data}).collect();assert_eq!(outputs[0],outputs[1]);assert_eq!(outputs[0],outputs[2]);
 let mut times:[Vec<f64>;4]=std::array::from_fn(|_|Vec::new());let orders=[[0,1,2,3],[3,2,1,0],[1,3,0,2],[2,0,3,1],[0,2,1,3],[3,1,2,0]];
 for round in 0..12{for k in orders[round%6]{times[k].push(timed(kernels[k],&input,width,parity));}}
 let med=times.map(|mut v|{v.sort_by(f64::total_cmp);v[v.len()/2]});println!("width{width}|parity{parity}|reference{:.4}|safe{:.4}|barrier{:.4}|control{:.4}|same",med[0],med[1],med[2],med[3]);
}}}
