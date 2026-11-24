#pragma once
#include <stddef.h>
#include <stdint.h>
#ifdef _WIN32
#define AUDIT_EXPORT __declspec(dllexport)
#else
#define AUDIT_EXPORT
#endif
#ifdef __cplusplus
extern "C" {
#endif
typedef struct AuditRequest {
 uint32_t w,h,numbps,orient,roishift,cblksty;
 const uint32_t *seg_len,*seg_passes;
 size_t nsegs;
 const uint8_t *bytes;
 const uint32_t *chunk_len;
 size_t nchunks;
 uint32_t reversible;
 float stepsize;
 uint32_t out_stride,out_offset;
} AuditRequest;
AUDIT_EXPORT void* nj2k_audit_prepare(const AuditRequest*,size_t count);
AUDIT_EXPORT void nj2k_audit_free(void*);
// mode0: decode+store;1:decode;2:store prepared raw coefficients (ROI0 only).
// indices==NULL selects all blocks; otherwise selected indices, in the supplied order.
AUDIT_EXPORT int nj2k_audit_run(void*,const size_t* indices,size_t nindices,size_t repetitions,uint32_t mode);
AUDIT_EXPORT size_t nj2k_audit_copy_output(void*,size_t index,int32_t* out,size_t capacity);
AUDIT_EXPORT size_t nj2k_audit_output_len(void*,size_t index);
#ifdef __cplusplus
}
#endif
