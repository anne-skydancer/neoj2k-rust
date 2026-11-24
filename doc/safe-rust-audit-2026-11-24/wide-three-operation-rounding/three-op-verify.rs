#![allow(dead_code)]
mod safe {
#[path="/home/dev/safe-crate-audit/candidate/neo-j2k/src/simd/lanes.rs"] pub mod lanes;
#[path="/home/dev/safe-crate-audit/parent-mct/three-op.rs"] pub mod kernel;
}
fn expected(v:f32,(shift,lo,hi):(i32,i32,i32))->i32 {
 let n=if v.is_nan(){i32::MIN}else{v.round_ties_even()as i32};
 n.clamp(lo-shift,hi-shift)+shift
}
fn main(){
 let mut state=0x7318_4521u32;let mut count=0usize;
 let specials=[0,0x80000000,1,0x80000001,0x3f000000,0xbf000000,0x3fc00000,0xbfc00000,0x4affffff,0xcaffffff,0x4b000000,0xcb000000,0x4effffff,0xceffffff,0x4f000000,0xcf000000,0x7f800000,0xff800000,0x7fc01234,0xffa01234,0x7f7fffff,0xff7fffff];
 for precision in 1..=31 {for signed in [false,true] {
  let half=1i32<<(precision-1);let f=if signed{(0,-half,half-1)}else{(half,0,(half-1)*2+1)};
  for block in 0..4096 {
   let values=core::array::from_fn(|lane|{state^=state<<13;state^=state>>17;state^=state<<5;let i=block*8+lane;f32::from_bits(if i<specials.len(){specials[i]}else{state})});
   assert_eq!(safe::kernel::verify_wide(values,f),values.map(|v|expected(v,f)),"p{precision} signed{signed} block{block}");count+=8;
  }
  for anchor in [0.0f32,256.0,65536.0,4194304.0,8388608.0,1073741824.0] {for sign in [-1.0f32,1.0] {for delta in [-128.5f32,-128.0,-127.5,-0.5,0.0,0.5,127.5,128.0,128.5] {
   let center=(anchor+delta)*sign;let bits=center.to_bits();
   let values=core::array::from_fn(|lane|f32::from_bits(bits.wrapping_add(lane as u32).wrapping_sub(4)));
   assert_eq!(safe::kernel::verify_wide(values,f),values.map(|v|expected(v,f)),"tie p{precision} signed{signed} {center}");count+=8;
  }}}
 }}println!("PASS {count} independent float-pattern/tie checks");
}
