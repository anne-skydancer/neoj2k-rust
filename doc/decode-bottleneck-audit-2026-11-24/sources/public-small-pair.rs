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
runner!(New,current); runner!(Old,packed);
fn median(mut v:Vec<f64>)->f64 {v.sort_by(f64::total_cmp);v[v.len()/2]}
fn time(r:&mut dyn Run,n:usize)->f64 {let s=Instant::now();for _ in 0..n {r.call();}s.elapsed().as_secs_f64()*1e6/n as f64}
fn main(){for name in ["sl_32_rgba_53.j2k","sl_64_rgb_97.j2k","sl_128_rgba_97.j2k"] {let data=std::fs::read(format!("C:/Users/annes/source/repos/neoj2k-rust/corpus/generated/{name}")).unwrap();for reduce in [2,0] {let mut a=New::new(&data,reduce);let mut b=Old::new(&data,reduce);for _ in 0..32 {a.call();b.call();}assert_eq!(a.bytes(),b.bytes());let mut av=vec![];let mut bv=vec![];for round in 0..32 {if round%2==0 {av.push(time(&mut a,128));bv.push(time(&mut b,128));}else{bv.push(time(&mut b,128));av.push(time(&mut a,128));}}let x=median(av);let y=median(bv);println!("{name} reduce {reduce}: current {x:.3} us packed {y:.3} us short_change {:+.2}%",100.0*(y/x-1.0));}}}
