#![forbid(unsafe_code)]
pub(crate) fn store(coefficients:&[i32],output:&mut[i32],width:usize,stride:usize,reversible:bool,half_step:f32)->bool {
 if width<8 {return false;}
 if reversible {
  for (row,input) in coefficients.chunks_exact(width).enumerate() {
   let start=row*stride;
   for (&source,target) in input.iter().zip(&mut output[start..start+width]) {*target=source/2;}
  }
 } else {
  for (row,input) in coefficients.chunks_exact(width).enumerate() {
   let start=row*stride;
   for (&source,target) in input.iter().zip(&mut output[start..start+width]) {*target=((source as f32)*half_step).to_bits().cast_signed();}
  }
 }
 true
}
