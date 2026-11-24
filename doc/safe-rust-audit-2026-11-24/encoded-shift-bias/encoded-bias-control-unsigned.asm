
/home/dev/safe-crate-audit/target-encoded-bias-paired-v3/release/neo-j2k-versions:     file format elf64-x86-64


Disassembly of section .text:

0000000000179f00 <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>>:
  179f00:	48 39 ce             	cmp    %rcx,%rsi
  179f03:	48 0f 42 ce          	cmovb  %rsi,%rcx
  179f07:	4c 39 c9             	cmp    %r9,%rcx
  179f0a:	49 0f 43 c9          	cmovae %r9,%rcx
  179f0e:	48 85 c9             	test   %rcx,%rcx
  179f11:	0f 84 a9 02 00 00    	je     17a1c0 <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0x2c0>
  179f17:	53                   	push   %rbx
  179f18:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
  179f1d:	8b 06                	mov    (%rsi),%eax
  179f1f:	44 8b 4e 04          	mov    0x4(%rsi),%r9d
  179f23:	8b 76 08             	mov    0x8(%rsi),%esi
  179f26:	41 29 c1             	sub    %eax,%r9d
  179f29:	c4 c1 02 2a c1       	vcvtsi2ss %r9d,%xmm15,%xmm0
  179f2e:	41 89 f1             	mov    %esi,%r9d
  179f31:	41 29 c1             	sub    %eax,%r9d
  179f34:	c4 c1 02 2a c9       	vcvtsi2ss %r9d,%xmm15,%xmm1
  179f39:	05 00 00 c0 74       	add    $0x74c00000,%eax
  179f3e:	48 83 f9 08          	cmp    $0x8,%rcx
  179f42:	73 08                	jae    179f4c <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0x4c>
  179f44:	45 31 c9             	xor    %r9d,%r9d
  179f47:	e9 55 01 00 00       	jmp    17a0a1 <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0x1a1>
  179f4c:	49 b9 f8 ff ff ff ff 	movabs $0x1ffffffffffffff8,%r9
  179f53:	ff ff 1f 
  179f56:	49 21 c9             	and    %rcx,%r9
  179f59:	c4 e2 7d 18 d0       	vbroadcastss %xmm0,%ymm2
  179f5e:	c4 e2 7d 18 d9       	vbroadcastss %xmm1,%ymm3
  179f63:	c5 f9 6e e0          	vmovd  %eax,%xmm4
  179f67:	c4 e2 7d 58 e4       	vpbroadcastd %xmm4,%ymm4
  179f6c:	c5 f9 6e ee          	vmovd  %esi,%xmm5
  179f70:	c4 e2 7d 58 ed       	vpbroadcastd %xmm5,%ymm5
  179f75:	4c 8d 14 8d 00 00 00 	lea    0x0(,%rcx,4),%r10
  179f7c:	00 
  179f7d:	49 83 e2 e0          	and    $0xffffffffffffffe0,%r10
  179f81:	45 31 db             	xor    %r11d,%r11d
  179f84:	c4 e2 7d 18 3d 0f f0 	vbroadcastss -0x160ff1(%rip),%ymm7        # 18f9c <GCC_except_table1176+0x25a8>
  179f8b:	e9 ff 
  179f8d:	c4 62 7d 18 05 b2 ef 	vbroadcastss -0x16104e(%rip),%ymm8        # 18f48 <GCC_except_table1176+0x2554>
  179f94:	e9 ff 
  179f96:	c4 62 7d 18 0d 65 f1 	vbroadcastss -0x160e9b(%rip),%ymm9        # 19104 <GCC_except_table1176+0x2710>
  179f9d:	e9 ff 
  179f9f:	c4 62 7d 18 15 38 f0 	vbroadcastss -0x160fc8(%rip),%ymm10        # 18fe0 <GCC_except_table1176+0x25ec>
  179fa6:	e9 ff 
  179fa8:	c4 62 7d 18 1d 1f f0 	vbroadcastss -0x160fe1(%rip),%ymm11        # 18fd0 <GCC_except_table1176+0x25dc>
  179faf:	e9 ff 
  179fb1:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  179fb8:	0f 1f 84 00 00 00 00 
  179fbf:	00 
  179fc0:	c4 21 7c 10 24 1f    	vmovups (%rdi,%r11,1),%ymm12
  179fc6:	c4 01 7c 10 2c 18    	vmovups (%r8,%r11,1),%ymm13
  179fcc:	c4 e2 7d 18 35 6b f0 	vbroadcastss -0x160f95(%rip),%ymm6        # 19040 <GCC_except_table1176+0x264c>
  179fd3:	e9 ff 
  179fd5:	c5 14 59 f6          	vmulps %ymm6,%ymm13,%ymm14
  179fd9:	c4 41 1c 58 f6       	vaddps %ymm14,%ymm12,%ymm14
  179fde:	c5 0c 5f f2          	vmaxps %ymm2,%ymm14,%ymm14
  179fe2:	c5 0c 5d f3          	vminps %ymm3,%ymm14,%ymm14
  179fe6:	c5 0c 58 ff          	vaddps %ymm7,%ymm14,%ymm15
  179fea:	c4 c1 3c 5c f7       	vsubps %ymm15,%ymm8,%ymm6
  179fef:	c5 8c 58 f6          	vaddps %ymm6,%ymm14,%ymm6
  179ff3:	c4 c1 0d 72 f7 08    	vpslld $0x8,%ymm15,%ymm14
  179ff9:	c5 0d fe f4          	vpaddd %ymm4,%ymm14,%ymm14
  179ffd:	c5 8d fe f6          	vpaddd %ymm6,%ymm14,%ymm6
  17a001:	c4 e2 4d 3b f5       	vpminud %ymm5,%ymm6,%ymm6
  17a006:	c4 a1 7e 7f 34 1f    	vmovdqu %ymm6,(%rdi,%r11,1)
  17a00c:	c4 a1 7c 10 34 1a    	vmovups (%rdx,%r11,1),%ymm6
  17a012:	c5 34 59 f6          	vmulps %ymm6,%ymm9,%ymm14
  17a016:	c4 41 1c 58 f6       	vaddps %ymm14,%ymm12,%ymm14
  17a01b:	c4 41 14 59 ea       	vmulps %ymm10,%ymm13,%ymm13
  17a020:	c4 41 0c 58 ed       	vaddps %ymm13,%ymm14,%ymm13
  17a025:	c5 14 5f ea          	vmaxps %ymm2,%ymm13,%ymm13
  17a029:	c5 14 5d eb          	vminps %ymm3,%ymm13,%ymm13
  17a02d:	c5 14 58 f7          	vaddps %ymm7,%ymm13,%ymm14
  17a031:	c4 41 3c 5c fe       	vsubps %ymm14,%ymm8,%ymm15
  17a036:	c4 41 14 58 ef       	vaddps %ymm15,%ymm13,%ymm13
  17a03b:	c4 c1 0d 72 f6 08    	vpslld $0x8,%ymm14,%ymm14
  17a041:	c5 0d fe f4          	vpaddd %ymm4,%ymm14,%ymm14
  17a045:	c4 41 0d fe ed       	vpaddd %ymm13,%ymm14,%ymm13
  17a04a:	c4 62 15 3b ed       	vpminud %ymm5,%ymm13,%ymm13
  17a04f:	c4 21 7e 7f 2c 1a    	vmovdqu %ymm13,(%rdx,%r11,1)
  17a055:	c5 a4 59 f6          	vmulps %ymm6,%ymm11,%ymm6
  17a059:	c5 9c 58 f6          	vaddps %ymm6,%ymm12,%ymm6
  17a05d:	c5 cc 5f f2          	vmaxps %ymm2,%ymm6,%ymm6
  17a061:	c5 cc 5d f3          	vminps %ymm3,%ymm6,%ymm6
  17a065:	c5 4c 58 e7          	vaddps %ymm7,%ymm6,%ymm12
  17a069:	c4 41 3c 5c ec       	vsubps %ymm12,%ymm8,%ymm13
  17a06e:	c5 94 58 f6          	vaddps %ymm6,%ymm13,%ymm6
  17a072:	c4 c1 1d 72 f4 08    	vpslld $0x8,%ymm12,%ymm12
  17a078:	c5 1d fe e4          	vpaddd %ymm4,%ymm12,%ymm12
  17a07c:	c5 9d fe f6          	vpaddd %ymm6,%ymm12,%ymm6
  17a080:	c4 e2 4d 3b f5       	vpminud %ymm5,%ymm6,%ymm6
  17a085:	c4 81 7e 7f 34 18    	vmovdqu %ymm6,(%r8,%r11,1)
  17a08b:	49 83 c3 20          	add    $0x20,%r11
  17a08f:	4d 39 da             	cmp    %r11,%r10
  17a092:	0f 85 28 ff ff ff    	jne    179fc0 <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0xc0>
  17a098:	4c 39 c9             	cmp    %r9,%rcx
  17a09b:	0f 84 1e 01 00 00    	je     17a1bf <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0x2bf>
  17a0a1:	c5 fa 10 15 97 ef e9 	vmovss -0x161069(%rip),%xmm2        # 19040 <GCC_except_table1176+0x264c>
  17a0a8:	ff 
  17a0a9:	c5 fa 10 1d eb ee e9 	vmovss -0x161115(%rip),%xmm3        # 18f9c <GCC_except_table1176+0x25a8>
  17a0b0:	ff 
  17a0b1:	c5 fa 10 25 8f ee e9 	vmovss -0x161171(%rip),%xmm4        # 18f48 <GCC_except_table1176+0x2554>
  17a0b8:	ff 
  17a0b9:	c5 fa 10 2d 43 f0 e9 	vmovss -0x160fbd(%rip),%xmm5        # 19104 <GCC_except_table1176+0x2710>
  17a0c0:	ff 
  17a0c1:	c5 fa 10 35 17 ef e9 	vmovss -0x1610e9(%rip),%xmm6        # 18fe0 <GCC_except_table1176+0x25ec>
  17a0c8:	ff 
  17a0c9:	c5 fa 10 3d ff ee e9 	vmovss -0x161101(%rip),%xmm7        # 18fd0 <GCC_except_table1176+0x25dc>
  17a0d0:	ff 
  17a0d1:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  17a0d8:	0f 1f 84 00 00 00 00 
  17a0df:	00 
  17a0e0:	c4 21 7a 10 04 8f    	vmovss (%rdi,%r9,4),%xmm8
  17a0e6:	c4 21 7a 10 0c 8a    	vmovss (%rdx,%r9,4),%xmm9
  17a0ec:	c4 01 7a 10 14 88    	vmovss (%r8,%r9,4),%xmm10
  17a0f2:	c5 2a 59 da          	vmulss %xmm2,%xmm10,%xmm11
  17a0f6:	c4 41 3a 58 db       	vaddss %xmm11,%xmm8,%xmm11
  17a0fb:	c5 22 5f d8          	vmaxss %xmm0,%xmm11,%xmm11
  17a0ff:	c5 22 5d d9          	vminss %xmm1,%xmm11,%xmm11
  17a103:	c5 22 58 e3          	vaddss %xmm3,%xmm11,%xmm12
  17a107:	c4 41 5a 5c ec       	vsubss %xmm12,%xmm4,%xmm13
  17a10c:	c4 41 22 58 dd       	vaddss %xmm13,%xmm11,%xmm11
  17a111:	c4 41 79 7e e2       	vmovd  %xmm12,%r10d
  17a116:	41 c1 e2 08          	shl    $0x8,%r10d
  17a11a:	c4 41 79 7e db       	vmovd  %xmm11,%r11d
  17a11f:	41 01 c2             	add    %eax,%r10d
  17a122:	45 01 da             	add    %r11d,%r10d
  17a125:	41 39 f2             	cmp    %esi,%r10d
  17a128:	44 0f 43 d6          	cmovae %esi,%r10d
  17a12c:	c5 32 59 dd          	vmulss %xmm5,%xmm9,%xmm11
  17a130:	c4 41 3a 58 db       	vaddss %xmm11,%xmm8,%xmm11
  17a135:	c5 2a 59 d6          	vmulss %xmm6,%xmm10,%xmm10
  17a139:	c4 41 22 58 d2       	vaddss %xmm10,%xmm11,%xmm10
  17a13e:	c5 2a 5f d0          	vmaxss %xmm0,%xmm10,%xmm10
  17a142:	c5 2a 5d d1          	vminss %xmm1,%xmm10,%xmm10
  17a146:	c5 2a 58 db          	vaddss %xmm3,%xmm10,%xmm11
  17a14a:	c4 41 5a 5c e3       	vsubss %xmm11,%xmm4,%xmm12
  17a14f:	c4 41 2a 58 d4       	vaddss %xmm12,%xmm10,%xmm10
  17a154:	c4 41 79 7e db       	vmovd  %xmm11,%r11d
  17a159:	41 c1 e3 08          	shl    $0x8,%r11d
  17a15d:	c5 79 7e d3          	vmovd  %xmm10,%ebx
  17a161:	41 01 c3             	add    %eax,%r11d
  17a164:	41 01 db             	add    %ebx,%r11d
  17a167:	41 39 f3             	cmp    %esi,%r11d
  17a16a:	44 0f 43 de          	cmovae %esi,%r11d
  17a16e:	46 89 14 8f          	mov    %r10d,(%rdi,%r9,4)
  17a172:	c5 32 59 cf          	vmulss %xmm7,%xmm9,%xmm9
  17a176:	c4 41 3a 58 c1       	vaddss %xmm9,%xmm8,%xmm8
  17a17b:	c5 3a 5f c0          	vmaxss %xmm0,%xmm8,%xmm8
  17a17f:	c5 3a 5d c1          	vminss %xmm1,%xmm8,%xmm8
  17a183:	c5 3a 58 cb          	vaddss %xmm3,%xmm8,%xmm9
  17a187:	c4 41 5a 5c d1       	vsubss %xmm9,%xmm4,%xmm10
  17a18c:	c4 41 3a 58 c2       	vaddss %xmm10,%xmm8,%xmm8
  17a191:	c4 41 79 7e ca       	vmovd  %xmm9,%r10d
  17a196:	41 c1 e2 08          	shl    $0x8,%r10d
  17a19a:	c5 79 7e c3          	vmovd  %xmm8,%ebx
  17a19e:	41 01 c2             	add    %eax,%r10d
  17a1a1:	41 01 da             	add    %ebx,%r10d
  17a1a4:	41 39 f2             	cmp    %esi,%r10d
  17a1a7:	44 0f 43 d6          	cmovae %esi,%r10d
  17a1ab:	46 89 1c 8a          	mov    %r11d,(%rdx,%r9,4)
  17a1af:	47 89 14 88          	mov    %r10d,(%r8,%r9,4)
  17a1b3:	49 ff c1             	inc    %r9
  17a1b6:	4c 39 c9             	cmp    %r9,%rcx
  17a1b9:	0f 85 21 ff ff ff    	jne    17a0e0 <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0x1e0>
  17a1bf:	5b                   	pop    %rbx
  17a1c0:	c5 f8 77             	vzeroupper
  17a1c3:	c3                   	ret
