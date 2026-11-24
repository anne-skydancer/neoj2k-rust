
/home/dev/safe-crate-audit/target-allocation-order-v3/release/neo-j2k-versions:     file format elf64-x86-64


Disassembly of section .text:

00000000000b3c30 <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_>:
   b3c30:	cmp    %rcx,%rsi
   b3c33:	cmovb  %rsi,%rcx
   b3c37:	cmp    %r9,%rcx
   b3c3a:	cmovae %r9,%rcx
   b3c3e:	test   %rcx,%rcx
   b3c41:	je     b3ef0 <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0x2c0>
   b3c47:	push   %rbx
   b3c48:	mov    0x10(%rsp),%rsi
   b3c4d:	mov    (%rsi),%eax
   b3c4f:	mov    0x4(%rsi),%r9d
   b3c53:	mov    0x8(%rsi),%esi
   b3c56:	sub    %eax,%r9d
   b3c59:	vcvtsi2ss %r9d,%xmm15,%xmm0
   b3c5e:	mov    %esi,%r9d
   b3c61:	sub    %eax,%r9d
   b3c64:	vcvtsi2ss %r9d,%xmm15,%xmm1
   b3c69:	add    $0x74c00000,%eax
   b3c6e:	cmp    $0x8,%rcx
   b3c72:	jae    b3c7c <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0x4c>
   b3c74:	xor    %r9d,%r9d
   b3c77:	jmp    b3dd1 <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0x1a1>
   b3c7c:	movabs $0x1ffffffffffffff8,%r9
   b3c86:	and    %rcx,%r9
   b3c89:	vbroadcastss %xmm0,%ymm2
   b3c8e:	vbroadcastss %xmm1,%ymm3
   b3c93:	vmovd  %eax,%xmm4
   b3c97:	vpbroadcastd %xmm4,%ymm4
   b3c9c:	vmovd  %esi,%xmm5
   b3ca0:	vpbroadcastd %xmm5,%ymm5
   b3ca5:	lea    0x0(,%rcx,4),%r10
   b3cad:	and    $0xffffffffffffffe0,%r10
   b3cb1:	xor    %r11d,%r11d
   b3cb4:	vbroadcastss -0x9bcf1(%rip),%ymm7        # 17fcc <GCC_except_table1137+0x16c4>
   b3cbd:	vbroadcastss -0x9bd42(%rip),%ymm8        # 17f84 <GCC_except_table1137+0x167c>
   b3cc6:	vbroadcastss -0x9bba7(%rip),%ymm9        # 18128 <GCC_except_table1137+0x1820>
   b3ccf:	vbroadcastss -0x9bcc0(%rip),%ymm10        # 18018 <GCC_except_table1137+0x1710>
   b3cd8:	vbroadcastss -0x9bcd9(%rip),%ymm11        # 18008 <GCC_except_table1137+0x1700>
   b3ce1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   b3cf0:	vmovups (%rdi,%r11,1),%ymm12
   b3cf6:	vmovups (%r8,%r11,1),%ymm13
   b3cfc:	vbroadcastss -0x9bc91(%rip),%ymm6        # 18074 <GCC_except_table1137+0x176c>
   b3d05:	vmulps %ymm6,%ymm13,%ymm14
   b3d09:	vaddps %ymm14,%ymm12,%ymm14
   b3d0e:	vmaxps %ymm2,%ymm14,%ymm14
   b3d12:	vminps %ymm3,%ymm14,%ymm14
   b3d16:	vaddps %ymm7,%ymm14,%ymm15
   b3d1a:	vsubps %ymm15,%ymm8,%ymm6
   b3d1f:	vaddps %ymm6,%ymm14,%ymm6
   b3d23:	vpslld $0x8,%ymm15,%ymm14
   b3d29:	vpaddd %ymm4,%ymm14,%ymm14
   b3d2d:	vpaddd %ymm6,%ymm14,%ymm6
   b3d31:	vpminud %ymm5,%ymm6,%ymm6
   b3d36:	vmovdqu %ymm6,(%rdi,%r11,1)
   b3d3c:	vmovups (%rdx,%r11,1),%ymm6
   b3d42:	vmulps %ymm6,%ymm9,%ymm14
   b3d46:	vaddps %ymm14,%ymm12,%ymm14
   b3d4b:	vmulps %ymm10,%ymm13,%ymm13
   b3d50:	vaddps %ymm13,%ymm14,%ymm13
   b3d55:	vmaxps %ymm2,%ymm13,%ymm13
   b3d59:	vminps %ymm3,%ymm13,%ymm13
   b3d5d:	vaddps %ymm7,%ymm13,%ymm14
   b3d61:	vsubps %ymm14,%ymm8,%ymm15
   b3d66:	vaddps %ymm15,%ymm13,%ymm13
   b3d6b:	vpslld $0x8,%ymm14,%ymm14
   b3d71:	vpaddd %ymm4,%ymm14,%ymm14
   b3d75:	vpaddd %ymm13,%ymm14,%ymm13
   b3d7a:	vpminud %ymm5,%ymm13,%ymm13
   b3d7f:	vmovdqu %ymm13,(%rdx,%r11,1)
   b3d85:	vmulps %ymm6,%ymm11,%ymm6
   b3d89:	vaddps %ymm6,%ymm12,%ymm6
   b3d8d:	vmaxps %ymm2,%ymm6,%ymm6
   b3d91:	vminps %ymm3,%ymm6,%ymm6
   b3d95:	vaddps %ymm7,%ymm6,%ymm12
   b3d99:	vsubps %ymm12,%ymm8,%ymm13
   b3d9e:	vaddps %ymm6,%ymm13,%ymm6
   b3da2:	vpslld $0x8,%ymm12,%ymm12
   b3da8:	vpaddd %ymm4,%ymm12,%ymm12
   b3dac:	vpaddd %ymm6,%ymm12,%ymm6
   b3db0:	vpminud %ymm5,%ymm6,%ymm6
   b3db5:	vmovdqu %ymm6,(%r8,%r11,1)
   b3dbb:	add    $0x20,%r11
   b3dbf:	cmp    %r11,%r10
   b3dc2:	jne    b3cf0 <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0xc0>
   b3dc8:	cmp    %r9,%rcx
   b3dcb:	je     b3eef <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0x2bf>
   b3dd1:	vmovss -0x9bd65(%rip),%xmm2        # 18074 <GCC_except_table1137+0x176c>
   b3dd9:	vmovss -0x9be15(%rip),%xmm3        # 17fcc <GCC_except_table1137+0x16c4>
   b3de1:	vmovss -0x9be65(%rip),%xmm4        # 17f84 <GCC_except_table1137+0x167c>
   b3de9:	vmovss -0x9bcc9(%rip),%xmm5        # 18128 <GCC_except_table1137+0x1820>
   b3df1:	vmovss -0x9bde1(%rip),%xmm6        # 18018 <GCC_except_table1137+0x1710>
   b3df9:	vmovss -0x9bdf9(%rip),%xmm7        # 18008 <GCC_except_table1137+0x1700>
   b3e01:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   b3e10:	vmovss (%rdi,%r9,4),%xmm8
   b3e16:	vmovss (%rdx,%r9,4),%xmm9
   b3e1c:	vmovss (%r8,%r9,4),%xmm10
   b3e22:	vmulss %xmm2,%xmm10,%xmm11
   b3e26:	vaddss %xmm11,%xmm8,%xmm11
   b3e2b:	vmaxss %xmm0,%xmm11,%xmm11
   b3e2f:	vminss %xmm1,%xmm11,%xmm11
   b3e33:	vaddss %xmm3,%xmm11,%xmm12
   b3e37:	vsubss %xmm12,%xmm4,%xmm13
   b3e3c:	vaddss %xmm13,%xmm11,%xmm11
   b3e41:	vmovd  %xmm12,%r10d
   b3e46:	shl    $0x8,%r10d
   b3e4a:	vmovd  %xmm11,%r11d
   b3e4f:	add    %eax,%r10d
   b3e52:	add    %r11d,%r10d
   b3e55:	cmp    %esi,%r10d
   b3e58:	cmovae %esi,%r10d
   b3e5c:	vmulss %xmm5,%xmm9,%xmm11
   b3e60:	vaddss %xmm11,%xmm8,%xmm11
   b3e65:	vmulss %xmm6,%xmm10,%xmm10
   b3e69:	vaddss %xmm10,%xmm11,%xmm10
   b3e6e:	vmaxss %xmm0,%xmm10,%xmm10
   b3e72:	vminss %xmm1,%xmm10,%xmm10
   b3e76:	vaddss %xmm3,%xmm10,%xmm11
   b3e7a:	vsubss %xmm11,%xmm4,%xmm12
   b3e7f:	vaddss %xmm12,%xmm10,%xmm10
   b3e84:	vmovd  %xmm11,%r11d
   b3e89:	shl    $0x8,%r11d
   b3e8d:	vmovd  %xmm10,%ebx
   b3e91:	add    %eax,%r11d
   b3e94:	add    %ebx,%r11d
   b3e97:	cmp    %esi,%r11d
   b3e9a:	cmovae %esi,%r11d
   b3e9e:	mov    %r10d,(%rdi,%r9,4)
   b3ea2:	vmulss %xmm7,%xmm9,%xmm9
   b3ea6:	vaddss %xmm9,%xmm8,%xmm8
   b3eab:	vmaxss %xmm0,%xmm8,%xmm8
   b3eaf:	vminss %xmm1,%xmm8,%xmm8
   b3eb3:	vaddss %xmm3,%xmm8,%xmm9
   b3eb7:	vsubss %xmm9,%xmm4,%xmm10
   b3ebc:	vaddss %xmm10,%xmm8,%xmm8
   b3ec1:	vmovd  %xmm9,%r10d
   b3ec6:	shl    $0x8,%r10d
   b3eca:	vmovd  %xmm8,%ebx
   b3ece:	add    %eax,%r10d
   b3ed1:	add    %ebx,%r10d
   b3ed4:	cmp    %esi,%r10d
   b3ed7:	cmovae %esi,%r10d
   b3edb:	mov    %r11d,(%rdx,%r9,4)
   b3edf:	mov    %r10d,(%r8,%r9,4)
   b3ee3:	inc    %r9
   b3ee6:	cmp    %r9,%rcx
   b3ee9:	jne    b3e10 <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0x1e0>
   b3eef:	pop    %rbx
   b3ef0:	vzeroupper
   b3ef3:	ret
