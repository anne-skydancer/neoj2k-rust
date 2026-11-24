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
runner!(New,current);runner!(Control,control);runner!(Short,short);runner!(Transform,transform);runner!(Geometry,geometry);
fn median(mut v:Vec<f64>)->f64 {v.sort_by(f64::total_cmp);v[v.len()/2]}
fn time(r:&mut dyn Run,n:usize)->f64 {let s=Instant::now();for _ in 0..n {r.call();}s.elapsed().as_secs_f64()*1e6/n as f64}
fn main(){for name in ["sl_32_rgba_53.j2k","sl_64_rgb_97.j2k","sl_128_rgba_97.j2k"] {let data=std::fs::read(format!("C:/Users/annes/source/repos/neoj2k-rust/corpus/generated/{name}")).unwrap();for reduce in [2,0] {
let mut runners:Vec<(&str,Box<dyn Run>)>=vec![("current",Box::new(New::new(&data,reduce))),("control",Box::new(Control::new(&data,reduce))),("short",Box::new(Short::new(&data,reduce))),("transform",Box::new(Transform::new(&data,reduce))),("geometry",Box::new(Geometry::new(&data,reduce)))];
for (_,r) in &mut runners{for _ in 0..32{r.call();}}
for (_,r) in &runners{assert_eq!(runners[0].1.bytes(),r.bytes());}
let mut results=vec![vec![];runners.len()];for round in 0..35{for step in 0..runners.len(){let i=(round+step)%runners.len();results[i].push(time(&mut *runners[i].1,64));}}
let values:Vec<_>=results.into_iter().map(median).collect();for ((label,_),&v) in runners.iter().zip(&values){println!("{name} reduce {reduce} {label}: {v:.3} us change {:+.2}%",100.0*(v/values[0]-1.0));}
}}}
