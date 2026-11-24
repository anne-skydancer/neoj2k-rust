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


#[path="/home/dev/safe-crate-audit/parent-mct/typed-level.rs"] pub mod kernel;
}mod current {
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


#[path="/home/dev/safe-crate-audit/parent-mct/f64.rs"] pub mod kernel;
}



use std::{hint::black_box,time::Instant};
fn main(){let ops:[fn(&mut[i32],i32,i32,i32,bool)->usize;4]=[reference::level_shift,current::kernel::level_shift,safe::kernel::level_shift,control::level_shift];
for(label,finish) in [("signed31",(0,-1073741824,1073741823)),("unsigned31",(1073741824,0,2147483647)),("signed24",(0,-8388608,8388607)),("unsigned24",(8388608,0,16777215)),("unsigned8",(128,0,255))]{for width in [32,128,1024]{for magnitude in [100.0f32,8_000_000.0,900_000_000.0]{
let(shift,lo,hi)=black_box(finish);let src:Vec<i32>=(0..width).map(|i|((i as f32*1.251-64.25)*magnitude).to_bits().cast_signed()).collect();let mut expected=src.clone();ops[0](&mut expected,shift,lo,hi,false);for op in ops{let mut out=src.clone();op(&mut out,shift,lo,hi,false);assert_eq!(out,expected);}
let mut all=[Vec::new(),Vec::new(),Vec::new(),Vec::new()];let mut dst=src.clone();for r in 0..24{for k in 0..4{let v=(r+k)%4;let begin=Instant::now();for _ in 0..3000{dst.copy_from_slice(black_box(&src));black_box(ops[v](black_box(&mut dst),shift,lo,hi,false));}all[v].push(begin.elapsed().as_secs_f64()*1e9/3000.0);}}
let ns=all.each_mut().map(|x|{x.sort_by(f64::total_cmp);x[x.len()/2]});println!("{label}|w{width}|mag{magnitude}|ref{:.2}|current{:.2}|flat{:.2}|control{:.2}|flat/current{:+.2}%",ns[0],ns[1],ns[2],ns[3],100.0*(ns[2]/ns[1]-1.0));
}}}}
