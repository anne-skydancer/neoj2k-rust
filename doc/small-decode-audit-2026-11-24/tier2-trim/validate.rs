use std::path::Path;
macro_rules! snapshot { ($fn:ident,$c:ident) => {
fn $fn(d:&mut $c::Decoder,data:&[u8])->String {
 let image=d.decode(data,Default::default());
 let mut out=match image { Ok(v)=>format!("ok:{}:{:?}",v.truncated,v.components.iter().map(|p|(p.info(),p.samples().to_vec())).collect::<Vec<_>>()), Err(e)=>format!("err:{:?}:{}",e.kind(),e.message()) };
 out.push_str(&format!("|stats:{:?}|diag:{:?}",d.statistics(),d.diagnostics()));out
}};}
snapshot!(a,current);snapshot!(b,trim);
fn main(){
 let root=Path::new("C:/Users/annes/source/repos/neoj2k-rust/corpus/generated");
 let mut files:Vec<_>=std::fs::read_dir(root).unwrap().map(|e|e.unwrap().path()).filter(|p|matches!(p.extension().and_then(|s|s.to_str()),Some("j2k"|"jp2"|"jph"))).collect();files.sort();
 let mut checks=0;
 for reduce in [0,2,5] {
  let mut x=current::Decoder::new(current::DecoderConfig{reduce,retain_bytes:u64::MAX,..Default::default()});
  let mut y=trim::Decoder::new(trim::DecoderConfig{reduce,retain_bytes:u64::MAX,..Default::default()});
  for max_memory in [0,16384,32768,65536,131072,0] {
   x.set_limits(current::Limits{max_memory,..Default::default()});
   y.set_limits(trim::Limits{max_memory,..Default::default()});
   for path in &files {let data=std::fs::read(path).unwrap();
    for cut in [0usize,2,7] {let data=&data[..data.len().saturating_sub(cut)]; assert_eq!(a(&mut x,data),b(&mut y,data),"{:?}, reduce {}, cap {}, cut {}",path,reduce,max_memory,cut);checks+=1;}
   }
  }
 }
 println!("{} exact outcome/statistics/diagnostic comparisons across {} corpus files, 3 reductions, changing memory limits and full/cut inputs passed",checks,files.len());
}
