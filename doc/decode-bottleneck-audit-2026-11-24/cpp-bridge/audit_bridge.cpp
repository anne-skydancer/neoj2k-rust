// Scratch-only prepared actual-block benchmark bridge. No production source changes.
#include "audit_bridge.h"
#include <algorithm>
#include <memory>
#include <vector>
#include "t1.h"
namespace d=nj2k::detail;
static_assert(sizeof(AuditRequest)==88);
struct PreparedBlock {
 std::vector<uint8_t> bytes;
 std::vector<d::Chunk> chunks;
 std::vector<d::Segment> segments;
 std::vector<int32_t> output,raw;
 d::T1Block b{};
 uint32_t stride=0,offset=0; bool reversible=false;float step=0;
};
struct Corpus {
 d::Allocator alloc=d::Allocator::system();
 std::unique_ptr<d::T1> t1=std::make_unique<d::T1>();
 d::Messages quiet;
 std::vector<PreparedBlock> blocks;
 ~Corpus(){t1->release(alloc);}
};
template<uint32_t Mode,bool Indexed>
int run_prepared(Corpus& h,const size_t* indices,size_t nindices,size_t repetitions){
 for(size_t rep=0;rep<repetitions;++rep){for(size_t i=0;i<nindices;++i){auto& b=h.blocks[Indexed?indices[i]:i];
  const bool empty=b.b.nchunks==0;
  if constexpr(Mode!=2){if(empty){if(!d::t1_empty_cblk_ok(b.b,h.quiet))return 0;}else if(!d::t1_decode_cblk(*h.t1,b.b,h.quiet))return 0;}
  if constexpr(Mode!=1)d::t1_store(empty?nullptr:(Mode==2?b.raw.data():h.t1->data),b.b.w,b.b.h,b.b.roishift,b.reversible,b.step,b.output.data()+b.offset,b.stride);
 }}return 1;
}
extern "C" {
void* nj2k_audit_prepare(const AuditRequest* req,size_t count) {
 try {
  if(!req || count==0 || count>100000) return nullptr;
  auto h=std::make_unique<Corpus>();if(!h->t1->ready(h->alloc))return nullptr;
  h->blocks.reserve(count);size_t max_bytes=0;
  for(size_t i=0;i<count;++i){const auto& r=req[i];
   if(!r.w||!r.h||r.w>1024||r.h>1024||uint64_t(r.w)*r.h>4096||r.orient>3||r.cblksty>63||r.nchunks>4096||r.nsegs>4096||r.out_stride<r.w)return nullptr;
   if((r.nchunks&&!r.chunk_len)||(r.nsegs&&(!r.seg_len||!r.seg_passes)))return nullptr;
   uint64_t total=0;for(size_t j=0;j<r.nchunks;++j)total+=r.chunk_len[j];
   const uint64_t length=uint64_t(r.out_offset)+uint64_t(r.h-1)*r.out_stride+r.w+3;
   if(total>(1u<<26)||(total&&!r.bytes)||length>(1u<<27))return nullptr;
   h->blocks.emplace_back();auto& b=h->blocks.back();
   if(total)b.bytes.assign(r.bytes,r.bytes+static_cast<size_t>(total));
   b.chunks.reserve(r.nchunks);size_t at=0;
   for(size_t j=0;j<r.nchunks;++j){b.chunks.push_back({b.bytes.empty()?nullptr:b.bytes.data()+at,r.chunk_len[j]});at+=r.chunk_len[j];}
   b.segments.resize(r.nsegs);
   for(size_t j=0;j<r.nsegs;++j){auto& s=b.segments[j];s.len=r.seg_len[j];s.numpasses=s.real_num_passes=r.seg_passes[j];}
   b.output.assign(static_cast<size_t>(length),0x5a5a5a5a);
   b.raw.resize(size_t(r.w)*r.h);
   b.b={r.w,r.h,r.numbps,r.orient,r.roishift,r.cblksty,b.chunks.data(),static_cast<uint32_t>(b.chunks.size()),b.segments.data(),static_cast<uint32_t>(b.segments.size())};
   b.stride=r.out_stride;b.offset=r.out_offset;b.reversible=r.reversible!=0;b.step=r.stepsize;
   max_bytes=std::max(max_bytes,static_cast<size_t>(total));
  }
  if(!h->t1->reserve(h->alloc,max_bytes))return nullptr;
  for(auto& b:h->blocks){
   const bool empty=b.b.nchunks==0;
   if(empty){if(!d::t1_empty_cblk_ok(b.b,h->quiet))return nullptr;}
   else {if(!d::t1_decode_cblk(*h->t1,b.b,h->quiet))return nullptr;std::copy_n(h->t1->data,b.raw.size(),b.raw.data());}
   d::t1_store(empty?nullptr:h->t1->data,b.b.w,b.b.h,b.b.roishift,b.reversible,b.step,b.output.data()+b.offset,b.stride);
  }
  return h.release();
 }catch(...){return nullptr;}
}
void nj2k_audit_free(void* ptr){delete static_cast<Corpus*>(ptr);}
int nj2k_audit_run(void* ptr,const size_t* indices,size_t nindices,size_t repetitions,uint32_t mode){
 if(!ptr||mode>2)return 0;auto& h=*static_cast<Corpus*>(ptr);
 if(!indices)nindices=h.blocks.size();
 for(size_t i=0;i<nindices;++i){size_t k=indices?indices[i]:i;if(k>=h.blocks.size()||(mode==2&&h.blocks[k].b.roishift!=0))return 0;}
 if(indices){switch(mode){case 0:return run_prepared<0,true>(h,indices,nindices,repetitions);case 1:return run_prepared<1,true>(h,indices,nindices,repetitions);default:return run_prepared<2,true>(h,indices,nindices,repetitions);}}
 switch(mode){case 0:return run_prepared<0,false>(h,nullptr,nindices,repetitions);case 1:return run_prepared<1,false>(h,nullptr,nindices,repetitions);default:return run_prepared<2,false>(h,nullptr,nindices,repetitions);}

}
size_t nj2k_audit_copy_output(void* ptr,size_t index,int32_t* out,size_t capacity){if(!ptr)return 0;auto& h=*static_cast<Corpus*>(ptr);if(index>=h.blocks.size())return 0;auto& b=h.blocks[index];if(!out||capacity<b.output.size())return 0;std::copy(b.output.begin(),b.output.end(),out);return b.output.size();}
size_t nj2k_audit_output_len(void* ptr,size_t index){if(!ptr)return 0;auto& h=*static_cast<Corpus*>(ptr);return index<h.blocks.size()?h.blocks[index].output.size():0;}
}
