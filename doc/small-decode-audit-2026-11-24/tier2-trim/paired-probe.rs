use std::time::Instant;
use std::hint::black_box;
trait Run { fn call(&mut self); fn bytes(&self)->&[u8]; }
macro_rules! runner { ($name:ident,$c:ident)=>{
struct $name<'a> { d:$c::Decoder, data:&'a[u8], bytes:Vec<u8>, planes:Vec<$c::OutputPlane> }
impl<'a> $name<'a> { fn new(data:&'a[u8], reduce:u32)->Self {
let mut d=$c::Decoder::new($c::DecoderConfig{reduce,..Default::default()});
let img=d.decode(data,Default::default()).unwrap(); let mut offset=0;
let planes=img.components.iter().enumerate().map(|(i,p)|{let n=p.samples().len()*4;let v=$c::OutputPlane{component:i as u32,offset,row_stride:p.info().width as isize*4,sample_stride:4};offset+=n;v}).collect();
Self{d,data,bytes:vec![0;offset],planes} } }
impl Run for $name<'_> { fn call(&mut self){let t=self.d.decode_into(black_box(self.data),Default::default(),$c::SampleType::I32,&mut[$c::DecodeTarget{bytes:&mut self.bytes,planes:&self.planes}]).unwrap();assert!(!t);black_box(&self.bytes);} fn bytes(&self)->&[u8]{&self.bytes} }
}; }
runner!(New,current); runner!(Control,control); runner!(Trim,trim);
fn median(mut v:Vec<f64>)->f64 {v.sort_by(f64::total_cmp);v[v.len()/2]}
fn time(r:&mut dyn Run,n:usize)->f64 {let s=Instant::now();for _ in 0..n {r.call();}s.elapsed().as_secs_f64()*1e6/n as f64}
fn main() { for name in ["sl_32_rgba_53.j2k","sl_64_rgb_97.j2k","sl_128_rgba_97.j2k"] {
 let data=std::fs::read(format!("C:/Users/annes/source/repos/neoj2k-rust/corpus/generated/{name}")).unwrap();
 for reduce in [2,0] {
  let mut a=New::new(&data,reduce);let mut b=Control::new(&data,reduce);let mut c=Trim::new(&data,reduce);
  for _ in 0..32 {a.call();b.call();c.call();}
  assert_eq!(a.bytes(),b.bytes());assert_eq!(a.bytes(),c.bytes());
  let mut times:[Vec<f64>;3]=std::array::from_fn(|_|vec![]);
  for round in 0..36 { for k in 0..3 { let i=(round+k)%3; let v=match i {0=>time(&mut a,128),1=>time(&mut b,128),_=>time(&mut c,128)};times[i].push(v); } }
  let [a,b,c]=times.map(median);
  println!("{name} reduce {reduce}: current {a:.3} us control {b:.3} us trim {c:.3} us control_change {:+.2}% trim_change {:+.2}%",100.0*(b/a-1.0),100.0*(c/a-1.0));
 }
}}
