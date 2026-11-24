
/home/dev/safe-crate-audit/target-v3/release/neo-j2k-versions:     file format elf64-x86-64


Disassembly of section .text:

00000000000b10c0 <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_>:
   b10c0:	cmp    %rcx,%rsi
   b10c3:	cmovb  %rsi,%rcx
   b10c7:	cmp    %r9,%rcx
   b10ca:	cmovae %r9,%rcx
   b10ce:	test   %rcx,%rcx
   b10d1:	je     b1380 <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0x2c0>
   b10d7:	push   %rbx
   b10d8:	mov    0x10(%rsp),%rsi
   b10dd:	mov    (%rsi),%eax
   b10df:	mov    0x4(%rsi),%r9d
   b10e3:	mov    0x8(%rsi),%esi
   b10e6:	sub    %eax,%r9d
   b10e9:	vcvtsi2ss %r9d,%xmm15,%xmm0
   b10ee:	mov    %esi,%r9d
   b10f1:	sub    %eax,%r9d
   b10f4:	vcvtsi2ss %r9d,%xmm15,%xmm1
   b10f9:	add    $0x74c00000,%eax
   b10fe:	cmp    $0x8,%rcx
   b1102:	jae    b110c <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0x4c>
   b1104:	xor    %r9d,%r9d
   b1107:	jmp    b1261 <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0x1a1>
   b110c:	movabs $0x1ffffffffffffff8,%r9
   b1116:	and    %rcx,%r9
   b1119:	vbroadcastss %xmm0,%ymm2
   b111e:	vbroadcastss %xmm1,%ymm3
   b1123:	vmovd  %eax,%xmm4
   b1127:	vpbroadcastd %xmm4,%ymm4
   b112c:	vmovd  %esi,%xmm5
   b1130:	vpbroadcastd %xmm5,%ymm5
   b1135:	lea    0x0(,%rcx,4),%r10
   b113d:	and    $0xffffffffffffffe0,%r10
   b1141:	xor    %r11d,%r11d
   b1144:	vbroadcastss -0x98279(%rip),%ymm7        # 18ed4 <GCC_except_table1172+0x2594>
   b114d:	vbroadcastss -0x982d2(%rip),%ymm8        # 18e84 <GCC_except_table1172+0x2544>
   b1156:	vbroadcastss -0x98123(%rip),%ymm9        # 1903c <GCC_except_table1172+0x26fc>
   b115f:	vbroadcastss -0x98248(%rip),%ymm10        # 18f20 <GCC_except_table1172+0x25e0>
   b1168:	vbroadcastss -0x98261(%rip),%ymm11        # 18f10 <GCC_except_table1172+0x25d0>
   b1171:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   b1180:	vmovups (%rdi,%r11,1),%ymm12
   b1186:	vmovups (%r8,%r11,1),%ymm13
   b118c:	vbroadcastss -0x98219(%rip),%ymm6        # 18f7c <GCC_except_table1172+0x263c>
   b1195:	vmulps %ymm6,%ymm13,%ymm14
   b1199:	vaddps %ymm14,%ymm12,%ymm14
   b119e:	vmaxps %ymm2,%ymm14,%ymm14
   b11a2:	vminps %ymm3,%ymm14,%ymm14
   b11a6:	vaddps %ymm7,%ymm14,%ymm15
   b11aa:	vsubps %ymm15,%ymm8,%ymm6
   b11af:	vaddps %ymm6,%ymm14,%ymm6
   b11b3:	vpslld $0x8,%ymm15,%ymm14
   b11b9:	vpaddd %ymm4,%ymm14,%ymm14
   b11bd:	vpaddd %ymm6,%ymm14,%ymm6
   b11c1:	vpminud %ymm5,%ymm6,%ymm6
   b11c6:	vmovdqu %ymm6,(%rdi,%r11,1)
   b11cc:	vmovups (%rdx,%r11,1),%ymm6
   b11d2:	vmulps %ymm6,%ymm9,%ymm14
   b11d6:	vaddps %ymm14,%ymm12,%ymm14
   b11db:	vmulps %ymm10,%ymm13,%ymm13
   b11e0:	vaddps %ymm13,%ymm14,%ymm13
   b11e5:	vmaxps %ymm2,%ymm13,%ymm13
   b11e9:	vminps %ymm3,%ymm13,%ymm13
   b11ed:	vaddps %ymm7,%ymm13,%ymm14
   b11f1:	vsubps %ymm14,%ymm8,%ymm15
   b11f6:	vaddps %ymm15,%ymm13,%ymm13
   b11fb:	vpslld $0x8,%ymm14,%ymm14
   b1201:	vpaddd %ymm4,%ymm14,%ymm14
   b1205:	vpaddd %ymm13,%ymm14,%ymm13
   b120a:	vpminud %ymm5,%ymm13,%ymm13
   b120f:	vmovdqu %ymm13,(%rdx,%r11,1)
   b1215:	vmulps %ymm6,%ymm11,%ymm6
   b1219:	vaddps %ymm6,%ymm12,%ymm6
   b121d:	vmaxps %ymm2,%ymm6,%ymm6
   b1221:	vminps %ymm3,%ymm6,%ymm6
   b1225:	vaddps %ymm7,%ymm6,%ymm12
   b1229:	vsubps %ymm12,%ymm8,%ymm13
   b122e:	vaddps %ymm6,%ymm13,%ymm6
   b1232:	vpslld $0x8,%ymm12,%ymm12
   b1238:	vpaddd %ymm4,%ymm12,%ymm12
   b123c:	vpaddd %ymm6,%ymm12,%ymm6
   b1240:	vpminud %ymm5,%ymm6,%ymm6
   b1245:	vmovdqu %ymm6,(%r8,%r11,1)
   b124b:	add    $0x20,%r11
   b124f:	cmp    %r11,%r10
   b1252:	jne    b1180 <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0xc0>
   b1258:	cmp    %r9,%rcx
   b125b:	je     b137f <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0x2bf>
   b1261:	vmovss -0x982ed(%rip),%xmm2        # 18f7c <GCC_except_table1172+0x263c>
   b1269:	vmovss -0x9839d(%rip),%xmm3        # 18ed4 <GCC_except_table1172+0x2594>
   b1271:	vmovss -0x983f5(%rip),%xmm4        # 18e84 <GCC_except_table1172+0x2544>
   b1279:	vmovss -0x98245(%rip),%xmm5        # 1903c <GCC_except_table1172+0x26fc>
   b1281:	vmovss -0x98369(%rip),%xmm6        # 18f20 <GCC_except_table1172+0x25e0>
   b1289:	vmovss -0x98381(%rip),%xmm7        # 18f10 <GCC_except_table1172+0x25d0>
   b1291:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   b12a0:	vmovss (%rdi,%r9,4),%xmm8
   b12a6:	vmovss (%rdx,%r9,4),%xmm9
   b12ac:	vmovss (%r8,%r9,4),%xmm10
   b12b2:	vmulss %xmm2,%xmm10,%xmm11
   b12b6:	vaddss %xmm11,%xmm8,%xmm11
   b12bb:	vmaxss %xmm0,%xmm11,%xmm11
   b12bf:	vminss %xmm1,%xmm11,%xmm11
   b12c3:	vaddss %xmm3,%xmm11,%xmm12
   b12c7:	vsubss %xmm12,%xmm4,%xmm13
   b12cc:	vaddss %xmm13,%xmm11,%xmm11
   b12d1:	vmovd  %xmm12,%r10d
   b12d6:	shl    $0x8,%r10d
   b12da:	vmovd  %xmm11,%r11d
   b12df:	add    %eax,%r10d
   b12e2:	add    %r11d,%r10d
   b12e5:	cmp    %esi,%r10d
   b12e8:	cmovae %esi,%r10d
   b12ec:	vmulss %xmm5,%xmm9,%xmm11
   b12f0:	vaddss %xmm11,%xmm8,%xmm11
   b12f5:	vmulss %xmm6,%xmm10,%xmm10
   b12f9:	vaddss %xmm10,%xmm11,%xmm10
   b12fe:	vmaxss %xmm0,%xmm10,%xmm10
   b1302:	vminss %xmm1,%xmm10,%xmm10
   b1306:	vaddss %xmm3,%xmm10,%xmm11
   b130a:	vsubss %xmm11,%xmm4,%xmm12
   b130f:	vaddss %xmm12,%xmm10,%xmm10
   b1314:	vmovd  %xmm11,%r11d
   b1319:	shl    $0x8,%r11d
   b131d:	vmovd  %xmm10,%ebx
   b1321:	add    %eax,%r11d
   b1324:	add    %ebx,%r11d
   b1327:	cmp    %esi,%r11d
   b132a:	cmovae %esi,%r11d
   b132e:	mov    %r10d,(%rdi,%r9,4)
   b1332:	vmulss %xmm7,%xmm9,%xmm9
   b1336:	vaddss %xmm9,%xmm8,%xmm8
   b133b:	vmaxss %xmm0,%xmm8,%xmm8
   b133f:	vminss %xmm1,%xmm8,%xmm8
   b1343:	vaddss %xmm3,%xmm8,%xmm9
   b1347:	vsubss %xmm9,%xmm4,%xmm10
   b134c:	vaddss %xmm10,%xmm8,%xmm8
   b1351:	vmovd  %xmm9,%r10d
   b1356:	shl    $0x8,%r10d
   b135a:	vmovd  %xmm8,%ebx
   b135e:	add    %eax,%r10d
   b1361:	add    %ebx,%r10d
   b1364:	cmp    %esi,%r10d
   b1367:	cmovae %esi,%r10d
   b136b:	mov    %r11d,(%rdx,%r9,4)
   b136f:	mov    %r10d,(%r8,%r9,4)
   b1373:	inc    %r9
   b1376:	cmp    %r9,%rcx
   b1379:	jne    b12a0 <_RINvNtNtCscmlNaaSWqWF_7neo_j2k4simd3mct29inverse_and_finish_typed_rowsKb0_EB6_+0x1e0>
   b137f:	pop    %rbx
   b1380:	vzeroupper
   b1383:	ret
