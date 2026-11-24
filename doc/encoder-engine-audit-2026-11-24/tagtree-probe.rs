use std::hint::black_box;
use std::time::Instant;
trait Runner { fn call(&mut self); fn output(&mut self)->Vec<u8>; }
macro_rules! runner {($name:ident,$c:ident)=>{
struct $name<'a> {enc:$c::Encoder, view:$c::ImageView<'a>, params:$c::EncodeParams<'a>, cold:bool}
impl<'a> $name<'a> {fn new(pixels:&'a[u8],side:u32,rev:bool,rates:&[f32],cold:bool)->Self {
let view=$c::ImageView{width:side,height:side,flip_vertical:false,samples:$c::ImageSamples::Interleaved{samples:pixels,components:3,row_stride:side as usize*3}};
let mut params=$c::EncodeParams::default();params.reversible=rev;params.resolutions=Some(6);params.mct=true;params.layer_ratios=rates.to_vec();Self{enc:$c::Encoder::new(),view,params,cold}
}}
impl Runner for $name<'_> {fn call(&mut self){if self.cold {black_box($c::Encoder::new().encode(black_box(&self.view),black_box(&self.params)).unwrap());} else {black_box(self.enc.encode(black_box(&self.view),black_box(&self.params)).unwrap());}} fn output(&mut self)->Vec<u8>{self.enc.encode(&self.view,&self.params).unwrap().to_vec()}}
};}
runner!(Current,current);runner!(Control,control);runner!(Bound,bound);runner!(Single,single);runner!(Combined,combined);
fn median(mut v:Vec<f64>)->f64{v.sort_by(f64::total_cmp);v[v.len()/2]}
fn main(){for (side,rev,rates) in [(32,true,vec![]),(128,true,vec![]),(128,false,vec![]),(128,false,vec![30.,10.,0.]),(128,true,vec![3.,1.5]),(256,false,vec![30.,10.,0.]),(512,false,vec![])] {
let pixels:Vec<u8>=(0..side as usize*side as usize*3).map(|i|{let x=(i%(side as usize*3))/3;let y=i/(side as usize*3);let c=i%3;(3*x+5*y+13*(x^y)+23*c) as u8}).collect();
for cold in [false,true] {if side>=256 &&cold {continue;}
let mut runners:Vec<Box<dyn Runner+'_>>=vec![Box::new(Current::new(&pixels,side,rev,&rates,cold)),Box::new(Control::new(&pixels,side,rev,&rates,cold)),Box::new(Bound::new(&pixels,side,rev,&rates,cold)),Box::new(Single::new(&pixels,side,rev,&rates,cold)),Box::new(Combined::new(&pixels,side,rev,&rates,cold))];
let expected=runners[0].output();for r in &mut runners {assert_eq!(expected,r.output());for _ in 0..3{r.call();}}
let batch=if side<=32 {8}else if side<=128 {2}else{1};let mut times:[Vec<f64>;5]=std::array::from_fn(|_|Vec::new());
for round in 0..20{for k in 0..5{let i=(round+k)%5;let start=Instant::now();for _ in 0..batch{runners[i].call();}times[i].push(start.elapsed().as_secs_f64()*1e6/batch as f64);}}
let med=times.map(median);println!("side={side} rev={rev} rates={rates:?} cold={cold} current={:.3} control={:.3} bound={:.3} single={:.3} combined={:.3}",med[0],med[1],med[2],med[3],med[4]);
}}}
