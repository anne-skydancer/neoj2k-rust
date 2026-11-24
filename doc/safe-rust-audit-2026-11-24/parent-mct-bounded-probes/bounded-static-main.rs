#![allow(dead_code)]
#[path="/home/dev/safe-crate-audit/base/neo-j2k/src/simd/mct.rs"] mod reference;
#[path="/home/dev/safe-crate-audit/base/neo-j2k/src/simd/mct.rs"] mod control;
mod safe {
#[path="/home/dev/safe-crate-audit/candidate/neo-j2k/src/simd/lanes.rs"] pub mod lanes;
/// Round to nearest even in the default floating-point environment.
/// NaN and an unrepresentable result produce i32::MIN.
#[inline]
#[allow(clippy::cast_possible_truncation, reason = "rounded and range checked")]
pub(crate) fn round_i32(value: f32) -> i32 {
    if (-4_194_304.0..=4_194_304.0).contains(&value) {
        return (value + 12_582_912.0).to_bits().wrapping_sub(0x4b40_0000).cast_signed();
    }
    round_general(value)
}

#[cold]
#[inline(never)]
#[allow(clippy::cast_possible_truncation, reason = "rounded and range checked")]
fn round_general(value: f32) -> i32 {
    // Values with magnitude at least 2^23 have no fractional bits. In the
    // remaining outer interval a signed 2^23 bias removes the fraction.
    let rounded = if value.abs() < 8_388_608.0 {
        let bias = f32::from_bits(0x4b00_0000 | (value.to_bits() & 0x8000_0000));
        (value + bias) - bias
    } else {
        value
    };
    if (-2_147_483_648.0..2_147_483_648.0).contains(&rounded) {
        rounded as i32
    } else {
        i32::MIN
    }
}


#[path="/home/dev/safe-crate-audit/parent-mct/bounded-static.rs"] pub mod kernel;
}


use std::{hint::black_box,time::Instant};
fn main(){
let ops:[fn([&mut[i32];3],bool,[(i32,i32,i32);3])->usize;3]=[reference::inverse_and_finish,safe::kernel::inverse_and_finish,control::inverse_and_finish];
for width in [32,128,1024]{for magnitude in [100.0f32,8_000_000.0,900_000_000.0]{
let src:[Vec<i32>;3]=core::array::from_fn(|c|(0..width).map(|i| ((i as f32*1.251-64.25+(c as f32*5.0))*magnitude).to_bits().cast_signed()).collect());
let finishes=black_box([(0,-1073741824,1073741823);3]);let mut expected=src.clone();let[a,b,c]=&mut expected;ops[0]([a,b,c],false,finishes);
for op in ops {let mut out=src.clone();let[a,b,c]=&mut out;op([a,b,c],false,finishes);assert_eq!(out,expected);}
let mut all=[Vec::new(),Vec::new(),Vec::new()];let mut dst=src.clone();
for round in 0..12 {for k in 0..3{let v=(round+k)%3;let begin=Instant::now();for _ in 0..10000{for(d,s) in dst.iter_mut().zip(&src){d.copy_from_slice(black_box(s));}let[a,b,c]=&mut dst;black_box(ops[v](black_box([a,b,c]),false,finishes));}all[v].push(begin.elapsed().as_secs_f64()*1e9/10000.0);}}
let median=|x:&mut Vec<f64>|{x.sort_by(f64::total_cmp);x[x.len()/2]};let ns=all.each_mut().map(median);println!("w{width} mag{magnitude} ref{:.2} safe{:.2} ctrl{:.2} %+{:.2}",ns[0],ns[1],ns[2],100.0*(ns[1]/ns[0]-1.0));
}}
}
