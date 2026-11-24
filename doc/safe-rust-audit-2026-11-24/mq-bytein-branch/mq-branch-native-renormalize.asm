
/home/dev/safe-crate-audit/target-mq-branch-native-v3/release/build/neo-j2k/1229b097d389baa0/out/neo_j2k-1229b097d389baa0:     file format elf64-x86-64


Disassembly of section .text:

00000000000e6fb0 <neo_j2k::t1::mq::renormalize_with_bytes>:
   e6fb0:	39 f1                	cmp    %esi,%ecx
   e6fb2:	0f 86 cd 00 00 00    	jbe    e7085 <neo_j2k::t1::mq::renormalize_with_bytes+0xd5>
   e6fb8:	55                   	push   %rbp
   e6fb9:	41 56                	push   %r14
   e6fbb:	53                   	push   %rbx
   e6fbc:	4c 8b 02             	mov    (%rdx),%r8
   e6fbf:	4c 8b 52 08          	mov    0x8(%rdx),%r10
   e6fc3:	44 8b 4a 10          	mov    0x10(%rdx),%r9d
   e6fc7:	eb 22                	jmp    e6feb <neo_j2k::t1::mq::renormalize_with_bytes+0x3b>
   e6fc9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   e6fd0:	41 81 c3 00 ff 00 00 	add    $0xff00,%r11d
   e6fd7:	b8 08 00 00 00       	mov    $0x8,%eax
   e6fdc:	44 89 df             	mov    %r11d,%edi
   e6fdf:	29 f1                	sub    %esi,%ecx
   e6fe1:	89 c6                	mov    %eax,%esi
   e6fe3:	39 c1                	cmp    %eax,%ecx
   e6fe5:	0f 86 9e 00 00 00    	jbe    e7089 <neo_j2k::t1::mq::renormalize_with_bytes+0xd9>
   e6feb:	c4 62 49 f7 df       	shlx   %esi,%edi,%r11d
   e6ff0:	4d 85 d2             	test   %r10,%r10
   e6ff3:	74 4b                	je     e7040 <neo_j2k::t1::mq::renormalize_with_bytes+0x90>
   e6ff5:	4d 8d 70 01          	lea    0x1(%r8),%r14
   e6ff9:	49 8d 5a ff          	lea    -0x1(%r10),%rbx
   e6ffd:	41 0f b6 28          	movzbl (%r8),%ebp
   e7001:	41 81 f9 ff 00 00 00 	cmp    $0xff,%r9d
   e7008:	75 56                	jne    e7060 <neo_j2k::t1::mq::renormalize_with_bytes+0xb0>
   e700a:	40 80 fd 8f          	cmp    $0x8f,%bpl
   e700e:	77 c0                	ja     e6fd0 <neo_j2k::t1::mq::renormalize_with_bytes+0x20>
   e7010:	4c 89 32             	mov    %r14,(%rdx)
   e7013:	48 89 5a 08          	mov    %rbx,0x8(%rdx)
   e7017:	89 6a 10             	mov    %ebp,0x10(%rdx)
   e701a:	89 ef                	mov    %ebp,%edi
   e701c:	c1 e7 09             	shl    $0x9,%edi
   e701f:	44 01 df             	add    %r11d,%edi
   e7022:	b8 07 00 00 00       	mov    $0x7,%eax
   e7027:	41 89 e9             	mov    %ebp,%r9d
   e702a:	49 89 da             	mov    %rbx,%r10
   e702d:	4d 89 f0             	mov    %r14,%r8
   e7030:	eb ad                	jmp    e6fdf <neo_j2k::t1::mq::renormalize_with_bytes+0x2f>
   e7032:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e7039:	1f 84 00 00 00 00 00 
   e7040:	41 81 f9 ff 00 00 00 	cmp    $0xff,%r9d
   e7047:	74 87                	je     e6fd0 <neo_j2k::t1::mq::renormalize_with_bytes+0x20>
   e7049:	bd ff 00 00 00       	mov    $0xff,%ebp
   e704e:	31 db                	xor    %ebx,%ebx
   e7050:	eb 11                	jmp    e7063 <neo_j2k::t1::mq::renormalize_with_bytes+0xb3>
   e7052:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   e7059:	1f 84 00 00 00 00 00 
   e7060:	4d 89 f0             	mov    %r14,%r8
   e7063:	4c 89 02             	mov    %r8,(%rdx)
   e7066:	48 89 5a 08          	mov    %rbx,0x8(%rdx)
   e706a:	89 6a 10             	mov    %ebp,0x10(%rdx)
   e706d:	89 ef                	mov    %ebp,%edi
   e706f:	c1 e7 08             	shl    $0x8,%edi
   e7072:	44 01 df             	add    %r11d,%edi
   e7075:	b8 08 00 00 00       	mov    $0x8,%eax
   e707a:	41 89 e9             	mov    %ebp,%r9d
   e707d:	49 89 da             	mov    %rbx,%r10
   e7080:	e9 5a ff ff ff       	jmp    e6fdf <neo_j2k::t1::mq::renormalize_with_bytes+0x2f>
   e7085:	89 f0                	mov    %esi,%eax
   e7087:	eb 04                	jmp    e708d <neo_j2k::t1::mq::renormalize_with_bytes+0xdd>
   e7089:	5b                   	pop    %rbx
   e708a:	41 5e                	pop    %r14
   e708c:	5d                   	pop    %rbp
   e708d:	c4 e2 71 f7 d7       	shlx   %ecx,%edi,%edx
   e7092:	29 c8                	sub    %ecx,%eax
   e7094:	48 c1 e0 20          	shl    $0x20,%rax
   e7098:	48 09 d0             	or     %rdx,%rax
   e709b:	c3                   	ret
