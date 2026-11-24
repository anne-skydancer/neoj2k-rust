use std::hint::black_box;
use std::time::Instant;
use neo_j2k_oracle::BenchmarkPool;
struct Pool(BenchmarkPool);
macro_rules! host {($c:ident)=>{impl $c::Executor for Pool{fn concurrency(&self)->usize{self.0.concurrency()as usize}fn parallel_for(&self,n:usize,f:&(dyn Fn(usize)+Sync)){self.0.parallel_for(n as u32,&|i|f(i as usize));}}};}
host!(current);host!(packed);host!(control);
trait Run{fn call(&mut self);fn output(&self)->&[i32];}
macro_rules! runner {($n:ident,$c:ident)=>{
struct $n<'a>{d:$c::Decoder,data:&'a[u8],samples:Vec<i32>,counts:Vec<usize>,pool:Option<&'a Pool>}
impl<'a>$n<'a>{fn new(data:&'a[u8],pool:Option<&'a Pool>,reduce:u32)->Self{let mut d=$c::Decoder::new($c::DecoderConfig{reduce,retain_bytes:256<<20,..Default::default()});let image=d.decode(data,Default::default()).unwrap();let counts:Vec<_>=image.components.iter().map(|c|c.samples().len()).collect();let samples=vec![0;counts.iter().sum()];Self{d,data,samples,counts,pool}}}
impl Run for $n<'_>{fn call(&mut self){let mut planes:[$c::I32Plane<'_>;8]=std::array::from_fn(|_|$c::I32Plane{component:0,samples:&mut[]});let mut rest=self.samples.as_mut_slice();for(i,&n)in self.counts.iter().enumerate(){let(a,b)=rest.split_at_mut(n);planes[i]=$c::I32Plane{component:i as u32,samples:a};rest=b;}let p=$c::DecodeParams{executor:self.pool.map(|h|h as &dyn $c::Executor),..Default::default()};let cut=self.d.decode_into_i32(black_box(self.data),p,&mut planes[..self.counts.len()]).unwrap();assert!(!cut);black_box(&self.samples);}fn output(&self)->&[i32]{&self.samples}}
};}
runner!(Current,current);runner!(Packed,packed);runner!(Control,control);
fn med(mut x:Vec<f64>)->f64{x.sort_by(f64::total_cmp);x[x.len()/2]}
fn time(x:&mut dyn Run,n:usize)->f64{let t=Instant::now();for _ in 0..n{x.call();}t.elapsed().as_secs_f64()*1e3/n as f64}
fn main(){for name in["large_1024_rgb_97.j2k","large_4096_g_97.j2k"]{let data=std::fs::read(format!("C:/Users/annes/source/repos/neoj2k-rust/corpus/generated/{name}")).unwrap();for width in[1,2,4,8]{let host=if width>1{Some(Pool(BenchmarkPool::new(width).unwrap()))}else{None};let mut a=Current::new(&data,host.as_ref(),0);let mut b=Packed::new(&data,host.as_ref(),0);let mut c=Control::new(&data,host.as_ref(),0);for _ in 0..3{a.call();b.call();c.call();}assert_eq!(a.output(),b.output());assert_eq!(a.output(),c.output());let mut times:[Vec<f64>;3]=std::array::from_fn(|_|vec![]);let orders=[[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];for round in 0..18{for i in orders[round%6]{let v=match i{0=>time(&mut a,3),1=>time(&mut b,3),_=>time(&mut c,3)};times[i].push(v);}}let[a,b,c]=times.map(med);println!("{name} workers{width} current{a:.4}ms packed{b:.4}ms control{c:.4}ms packed_change{:+.2}% control_change{:+.2}%",100.0*(b/a-1.0),100.0*(c/a-1.0));}}}
