
/home/dev/safe-crate-audit/target-encoded-bias-paired-v3/release/neo-j2k-versions:     file format elf64-x86-64


Disassembly of section .text:

00000000000b1810 <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>>:
   b1810:	48 39 ce             	cmp    %rcx,%rsi
   b1813:	48 0f 42 ce          	cmovb  %rsi,%rcx
   b1817:	4c 39 c9             	cmp    %r9,%rcx
   b181a:	49 0f 43 c9          	cmovae %r9,%rcx
   b181e:	48 85 c9             	test   %rcx,%rcx
   b1821:	0f 84 7f 02 00 00    	je     b1aa6 <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0x296>
   b1827:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   b182c:	8b 30                	mov    (%rax),%esi
   b182e:	44 8b 48 04          	mov    0x4(%rax),%r9d
   b1832:	41 29 f1             	sub    %esi,%r9d
   b1835:	c4 c1 02 2a c1       	vcvtsi2ss %r9d,%xmm15,%xmm0
   b183a:	8b 40 08             	mov    0x8(%rax),%eax
   b183d:	41 89 c1             	mov    %eax,%r9d
   b1840:	41 29 f1             	sub    %esi,%r9d
   b1843:	c4 c1 02 2a c9       	vcvtsi2ss %r9d,%xmm15,%xmm1
   b1848:	81 c6 00 00 c0 b4    	add    $0xb4c00000,%esi
   b184e:	c1 ee 0a             	shr    $0xa,%esi
   b1851:	81 ce 00 00 00 50    	or     $0x50000000,%esi
   b1857:	c5 f9 6e d6          	vmovd  %esi,%xmm2
   b185b:	c5 ea 58 1d 6d 78 f6 	vaddss -0x98793(%rip),%xmm2,%xmm3        # 190d0 <GCC_except_table1176+0x26dc>
   b1862:	ff 
   b1863:	48 83 f9 08          	cmp    $0x8,%rcx
   b1867:	73 07                	jae    b1870 <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0x60>
   b1869:	31 f6                	xor    %esi,%esi
   b186b:	e9 3f 01 00 00       	jmp    b19af <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0x19f>
   b1870:	48 be f8 ff ff ff ff 	movabs $0x1ffffffffffffff8,%rsi
   b1877:	ff ff 1f 
   b187a:	48 21 ce             	and    %rcx,%rsi
   b187d:	c4 e2 7d 18 e0       	vbroadcastss %xmm0,%ymm4
   b1882:	c4 e2 7d 18 e9       	vbroadcastss %xmm1,%ymm5
   b1887:	c4 e2 7d 18 f2       	vbroadcastss %xmm2,%ymm6
   b188c:	c4 e2 7d 18 fb       	vbroadcastss %xmm3,%ymm7
   b1891:	c5 79 6e c0          	vmovd  %eax,%xmm8
   b1895:	c4 42 7d 58 c0       	vpbroadcastd %xmm8,%ymm8
   b189a:	4c 8d 0c 8d 00 00 00 	lea    0x0(,%rcx,4),%r9
   b18a1:	00 
   b18a2:	49 83 e1 e0          	and    $0xffffffffffffffe0,%r9
   b18a6:	45 31 d2             	xor    %r10d,%r10d
   b18a9:	c4 62 7d 18 25 1e 77 	vbroadcastss -0x988e2(%rip),%ymm12        # 18fd0 <GCC_except_table1176+0x25dc>
   b18b0:	f6 ff 
   b18b2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   b18b9:	1f 84 00 00 00 00 00 
   b18c0:	c4 21 7c 10 2c 17    	vmovups (%rdi,%r10,1),%ymm13
   b18c6:	c4 21 7c 10 34 12    	vmovups (%rdx,%r10,1),%ymm14
   b18cc:	c4 01 7c 10 3c 10    	vmovups (%r8,%r10,1),%ymm15
   b18d2:	c4 62 7d 18 0d 65 77 	vbroadcastss -0x9889b(%rip),%ymm9        # 19040 <GCC_except_table1176+0x264c>
   b18d9:	f6 ff 
   b18db:	c4 41 04 59 c9       	vmulps %ymm9,%ymm15,%ymm9
   b18e0:	c4 41 14 58 c9       	vaddps %ymm9,%ymm13,%ymm9
   b18e5:	c5 34 5f cc          	vmaxps %ymm4,%ymm9,%ymm9
   b18e9:	c5 34 5d cd          	vminps %ymm5,%ymm9,%ymm9
   b18ed:	c5 34 58 d6          	vaddps %ymm6,%ymm9,%ymm10
   b18f1:	c4 41 44 5c da       	vsubps %ymm10,%ymm7,%ymm11
   b18f6:	c4 41 34 58 cb       	vaddps %ymm11,%ymm9,%ymm9
   b18fb:	c4 c1 2d 72 f2 0a    	vpslld $0xa,%ymm10,%ymm10
   b1901:	c4 41 2d fe c9       	vpaddd %ymm9,%ymm10,%ymm9
   b1906:	c4 42 35 3b c8       	vpminud %ymm8,%ymm9,%ymm9
   b190b:	c4 21 7e 7f 0c 17    	vmovdqu %ymm9,(%rdi,%r10,1)
   b1911:	c4 62 7d 18 0d ea 77 	vbroadcastss -0x98816(%rip),%ymm9        # 19104 <GCC_except_table1176+0x2710>
   b1918:	f6 ff 
   b191a:	c4 41 0c 59 c9       	vmulps %ymm9,%ymm14,%ymm9
   b191f:	c4 41 14 58 c9       	vaddps %ymm9,%ymm13,%ymm9
   b1924:	c4 62 7d 18 15 b3 76 	vbroadcastss -0x9894d(%rip),%ymm10        # 18fe0 <GCC_except_table1176+0x25ec>
   b192b:	f6 ff 
   b192d:	c4 41 04 59 d2       	vmulps %ymm10,%ymm15,%ymm10
   b1932:	c4 41 34 58 ca       	vaddps %ymm10,%ymm9,%ymm9
   b1937:	c5 34 5f cc          	vmaxps %ymm4,%ymm9,%ymm9
   b193b:	c5 34 5d cd          	vminps %ymm5,%ymm9,%ymm9
   b193f:	c5 34 58 d6          	vaddps %ymm6,%ymm9,%ymm10
   b1943:	c4 41 44 5c da       	vsubps %ymm10,%ymm7,%ymm11
   b1948:	c4 41 34 58 cb       	vaddps %ymm11,%ymm9,%ymm9
   b194d:	c4 c1 2d 72 f2 0a    	vpslld $0xa,%ymm10,%ymm10
   b1953:	c4 41 2d fe c9       	vpaddd %ymm9,%ymm10,%ymm9
   b1958:	c4 42 35 3b c8       	vpminud %ymm8,%ymm9,%ymm9
   b195d:	c4 21 7e 7f 0c 12    	vmovdqu %ymm9,(%rdx,%r10,1)
   b1963:	c4 41 0c 59 cc       	vmulps %ymm12,%ymm14,%ymm9
   b1968:	c4 41 14 58 c9       	vaddps %ymm9,%ymm13,%ymm9
   b196d:	c5 34 5f cc          	vmaxps %ymm4,%ymm9,%ymm9
   b1971:	c5 34 5d cd          	vminps %ymm5,%ymm9,%ymm9
   b1975:	c5 34 58 d6          	vaddps %ymm6,%ymm9,%ymm10
   b1979:	c4 41 44 5c da       	vsubps %ymm10,%ymm7,%ymm11
   b197e:	c4 41 34 58 cb       	vaddps %ymm11,%ymm9,%ymm9
   b1983:	c4 c1 2d 72 f2 0a    	vpslld $0xa,%ymm10,%ymm10
   b1989:	c4 41 2d fe c9       	vpaddd %ymm9,%ymm10,%ymm9
   b198e:	c4 42 35 3b c8       	vpminud %ymm8,%ymm9,%ymm9
   b1993:	c4 01 7e 7f 0c 10    	vmovdqu %ymm9,(%r8,%r10,1)
   b1999:	49 83 c2 20          	add    $0x20,%r10
   b199d:	4d 39 d1             	cmp    %r10,%r9
   b19a0:	0f 85 1a ff ff ff    	jne    b18c0 <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0xb0>
   b19a6:	48 39 f1             	cmp    %rsi,%rcx
   b19a9:	0f 84 f7 00 00 00    	je     b1aa6 <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0x296>
   b19af:	c5 fa 10 25 89 76 f6 	vmovss -0x98977(%rip),%xmm4        # 19040 <GCC_except_table1176+0x264c>
   b19b6:	ff 
   b19b7:	c5 fa 10 2d 45 77 f6 	vmovss -0x988bb(%rip),%xmm5        # 19104 <GCC_except_table1176+0x2710>
   b19be:	ff 
   b19bf:	c5 fa 10 35 19 76 f6 	vmovss -0x989e7(%rip),%xmm6        # 18fe0 <GCC_except_table1176+0x25ec>
   b19c6:	ff 
   b19c7:	c5 fa 10 3d 01 76 f6 	vmovss -0x989ff(%rip),%xmm7        # 18fd0 <GCC_except_table1176+0x25dc>
   b19ce:	ff 
   b19cf:	90                   	nop
   b19d0:	c5 7a 10 04 b7       	vmovss (%rdi,%rsi,4),%xmm8
   b19d5:	c4 41 7a 10 0c b0    	vmovss (%r8,%rsi,4),%xmm9
   b19db:	c5 32 59 d4          	vmulss %xmm4,%xmm9,%xmm10
   b19df:	c4 41 3a 58 d2       	vaddss %xmm10,%xmm8,%xmm10
   b19e4:	c5 2a 5f d0          	vmaxss %xmm0,%xmm10,%xmm10
   b19e8:	c5 2a 5d d1          	vminss %xmm1,%xmm10,%xmm10
   b19ec:	c5 2a 58 da          	vaddss %xmm2,%xmm10,%xmm11
   b19f0:	c4 41 62 5c e3       	vsubss %xmm11,%xmm3,%xmm12
   b19f5:	c4 41 2a 58 d4       	vaddss %xmm12,%xmm10,%xmm10
   b19fa:	c4 41 79 7e d9       	vmovd  %xmm11,%r9d
   b19ff:	41 c1 e1 0a          	shl    $0xa,%r9d
   b1a03:	c4 41 79 7e d2       	vmovd  %xmm10,%r10d
   b1a08:	45 01 ca             	add    %r9d,%r10d
   b1a0b:	41 39 c2             	cmp    %eax,%r10d
   b1a0e:	44 0f 43 d0          	cmovae %eax,%r10d
   b1a12:	c5 7a 10 14 b2       	vmovss (%rdx,%rsi,4),%xmm10
   b1a17:	44 89 14 b7          	mov    %r10d,(%rdi,%rsi,4)
   b1a1b:	c5 2a 59 dd          	vmulss %xmm5,%xmm10,%xmm11
   b1a1f:	c4 41 3a 58 db       	vaddss %xmm11,%xmm8,%xmm11
   b1a24:	c5 32 59 ce          	vmulss %xmm6,%xmm9,%xmm9
   b1a28:	c4 41 22 58 c9       	vaddss %xmm9,%xmm11,%xmm9
   b1a2d:	c5 32 5f c8          	vmaxss %xmm0,%xmm9,%xmm9
   b1a31:	c5 32 5d c9          	vminss %xmm1,%xmm9,%xmm9
   b1a35:	c5 32 58 da          	vaddss %xmm2,%xmm9,%xmm11
   b1a39:	c4 41 62 5c e3       	vsubss %xmm11,%xmm3,%xmm12
   b1a3e:	c4 41 32 58 cc       	vaddss %xmm12,%xmm9,%xmm9
   b1a43:	c4 41 79 7e d9       	vmovd  %xmm11,%r9d
   b1a48:	41 c1 e1 0a          	shl    $0xa,%r9d
   b1a4c:	c4 41 79 7e ca       	vmovd  %xmm9,%r10d
   b1a51:	45 01 ca             	add    %r9d,%r10d
   b1a54:	41 39 c2             	cmp    %eax,%r10d
   b1a57:	44 0f 43 d0          	cmovae %eax,%r10d
   b1a5b:	c5 2a 59 cf          	vmulss %xmm7,%xmm10,%xmm9
   b1a5f:	c4 41 3a 58 c1       	vaddss %xmm9,%xmm8,%xmm8
   b1a64:	c5 3a 5f c0          	vmaxss %xmm0,%xmm8,%xmm8
   b1a68:	c5 3a 5d c1          	vminss %xmm1,%xmm8,%xmm8
   b1a6c:	c5 3a 58 ca          	vaddss %xmm2,%xmm8,%xmm9
   b1a70:	c4 41 62 5c d1       	vsubss %xmm9,%xmm3,%xmm10
   b1a75:	c4 41 3a 58 c2       	vaddss %xmm10,%xmm8,%xmm8
   b1a7a:	c4 41 79 7e c9       	vmovd  %xmm9,%r9d
   b1a7f:	41 c1 e1 0a          	shl    $0xa,%r9d
   b1a83:	c4 41 79 7e c3       	vmovd  %xmm8,%r11d
   b1a88:	45 01 cb             	add    %r9d,%r11d
   b1a8b:	41 39 c3             	cmp    %eax,%r11d
   b1a8e:	44 0f 43 d8          	cmovae %eax,%r11d
   b1a92:	44 89 14 b2          	mov    %r10d,(%rdx,%rsi,4)
   b1a96:	45 89 1c b0          	mov    %r11d,(%r8,%rsi,4)
   b1a9a:	48 ff c6             	inc    %rsi
   b1a9d:	48 39 f1             	cmp    %rsi,%rcx
   b1aa0:	0f 85 2a ff ff ff    	jne    b19d0 <neo_j2k::simd::mct::inverse_and_finish_typed_rows::<false>+0x1c0>
   b1aa6:	c5 f8 77             	vzeroupper
   b1aa9:	c3                   	ret
