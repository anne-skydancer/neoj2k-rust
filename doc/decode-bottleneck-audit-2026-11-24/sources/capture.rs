
use codec::{Decoder,DecoderConfig};
use codec::bench::{audit_capture_begin,audit_capture_end,AuditBlock,AuditWorkspace};
use std::io::Write;
fn write_capture(path:&std::path::Path,blocks:&[AuditBlock]) {
 let mut f=std::fs::File::create(path).unwrap();f.write_all(b"NJDA1\0\0\0").unwrap();
 fn u(f:&mut std::fs::File,n:u32){f.write_all(&n.to_le_bytes()).unwrap();}
 u(&mut f,blocks.len() as u32);
 for b in blocks {for n in b.meta {u(&mut f,n);}u(&mut f,b.segments.len() as u32);u(&mut f,b.chunk_lengths.len() as u32);for &(n,p) in &b.segments {u(&mut f,n);u(&mut f,p);}for &n in &b.chunk_lengths {u(&mut f,n);}f.write_all(&b.bytes).unwrap();}
}
fn main(){let out=std::path::PathBuf::from(std::env::args().nth(1).unwrap());std::fs::create_dir_all(&out).unwrap();
 for name in ["sl_32_rgba_53.j2k","sl_64_rgb_97.j2k","sl_128_rgba_97.j2k"] {for reduce in [2,0] {
 let data=std::fs::read(format!("C:/Users/annes/source/repos/neoj2k-rust/corpus/generated/{name}")).unwrap();let mut d=Decoder::new(DecoderConfig{reduce,..Default::default()});
 audit_capture_begin();let image=d.decode(&data,Default::default()).unwrap();assert!(!image.truncated);let blocks=audit_capture_end();
 write_capture(&out.join(format!("{name}.r{reduce}.bin")),&blocks);
 let mut ws=AuditWorkspace::new(blocks.clone(),false);let indices:Vec<_>=(0..blocks.len()).collect();for mode in 0..3 {ws.run(&indices,3,mode);}
 println!("{name} reduce{reduce}: blocks{} active{} bytes{} chunks{}",blocks.len(),blocks.iter().filter(|b| !b.bytes.is_empty()).count(),blocks.iter().map(|b|b.bytes.len()).sum::<usize>(),blocks.iter().map(|b|b.chunk_lengths.len()).sum::<usize>());
 }}
}
