	.att_syntax
	.file	"mq_byte_select_minimal.7facded1d22404dd-cgu.0"
	.section	.text.unlikely._RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal7current22renormalize_with_bytes,"ax",@progbits
	.globl	_RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal7current22renormalize_with_bytes
	.prefalign	4, .Lfunc_end0, nop
	.type	_RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal7current22renormalize_with_bytes,@function
_RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal7current22renormalize_with_bytes:
	.cfi_startproc
	cmpl	%esi, %ecx
	jbe	.LBB0_1
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %rbp, -16
	movq	(%rdx), %r9
	movq	8(%rdx), %r8
	movl	16(%rdx), %r11d
	jmp	.LBB0_4
	.p2align	4
.LBB0_9:
	addl	$65280, %ebp
	movl	$8, %eax
	movl	$255, %r10d
	movl	%ebp, %edi
.LBB0_10:
	subl	%esi, %ecx
	movl	%r10d, %r11d
	movl	%eax, %esi
	cmpl	%eax, %ecx
	jbe	.LBB0_11
.LBB0_4:
	testq	%r8, %r8
	je	.LBB0_5
	leaq	1(%r9), %rbx
	leaq	-1(%r8), %r14
	movzbl	(%r9), %r10d
	jmp	.LBB0_7
	.p2align	4
.LBB0_5:
	movl	$255, %r10d
	movq	%r9, %rbx
	xorl	%r14d, %r14d
.LBB0_7:
	shlxl	%esi, %edi, %ebp
	cmpl	$255, %r11d
	sete	%al
	cmpl	$144, %r10d
	setae	%dil
	testb	%al, %dil
	jne	.LBB0_9
	xorl	%eax, %eax
	cmpl	$255, %r11d
	movq	%rbx, (%rdx)
	movq	%r14, 8(%rdx)
	movl	%r10d, 16(%rdx)
	setne	%al
	addl	$7, %eax
	movb	$16, %dil
	subb	%al, %dil
	shlxl	%edi, %r10d, %edi
	addl	%ebp, %edi
	movq	%r14, %r8
	movq	%rbx, %r9
	jmp	.LBB0_10
.LBB0_1:
	.cfi_def_cfa_offset 8
	.cfi_restore %rbx
	.cfi_restore %rbp
	.cfi_restore %r14
	movl	%esi, %eax
	jmp	.LBB0_2
.LBB0_11:
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %rbp, -16
	.cfi_offset %r14, -24
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	.cfi_restore %rbx
	.cfi_restore %r14
	.cfi_restore %rbp
.LBB0_2:
	shlxl	%ecx, %edi, %edx
	subl	%ecx, %eax
	shlq	$32, %rax
	orq	%rdx, %rax
	retq
.Lfunc_end0:
	.size	_RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal7current22renormalize_with_bytes, .Lfunc_end0-_RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal7current22renormalize_with_bytes
	.cfi_endproc

	.section	.text.unlikely._RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal8selected22renormalize_with_bytes,"ax",@progbits
	.globl	_RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal8selected22renormalize_with_bytes
	.prefalign	4, .Lfunc_end1, nop
	.type	_RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal8selected22renormalize_with_bytes,@function
_RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal8selected22renormalize_with_bytes:
	.cfi_startproc
	cmpl	%esi, %ecx
	jbe	.LBB1_1
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %rbp, -16
	movq	(%rdx), %r9
	movq	8(%rdx), %r8
	movl	16(%rdx), %r11d
	jmp	.LBB1_4
	.p2align	4
.LBB1_9:
	addl	$65280, %ebp
	movl	$8, %eax
	movl	$255, %r10d
	movl	%ebp, %edi
.LBB1_10:
	subl	%esi, %ecx
	movl	%r10d, %r11d
	movl	%eax, %esi
	cmpl	%eax, %ecx
	jbe	.LBB1_11
.LBB1_4:
	testq	%r8, %r8
	je	.LBB1_5
	leaq	1(%r9), %rbx
	leaq	-1(%r8), %r14
	movzbl	(%r9), %r10d
	jmp	.LBB1_7
	.p2align	4
.LBB1_5:
	movl	$255, %r10d
	movq	%r9, %rbx
	xorl	%r14d, %r14d
.LBB1_7:
	shlxl	%esi, %edi, %ebp
	cmpl	$255, %r11d
	sete	%al
	cmpl	$144, %r10d
	setae	%dil
	testb	%al, %dil
	jne	.LBB1_9
	xorl	%edi, %edi
	cmpl	$255, %r11d
	sete	%dil
	movq	%rbx, (%rdx)
	movq	%r14, 8(%rdx)
	movl	%r10d, 16(%rdx)
	movl	$8, %eax
	subl	%edi, %eax
	orb	$8, %dil
	shlxl	%edi, %r10d, %edi
	addl	%ebp, %edi
	movq	%r14, %r8
	movq	%rbx, %r9
	jmp	.LBB1_10
.LBB1_1:
	.cfi_def_cfa_offset 8
	.cfi_restore %rbx
	.cfi_restore %rbp
	.cfi_restore %r14
	movl	%esi, %eax
	jmp	.LBB1_2
.LBB1_11:
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %rbp, -16
	.cfi_offset %r14, -24
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	.cfi_restore %rbx
	.cfi_restore %r14
	.cfi_restore %rbp
.LBB1_2:
	shlxl	%ecx, %edi, %edx
	subl	%ecx, %eax
	shlq	$32, %rax
	orq	%rdx, %rax
	retq
.Lfunc_end1:
	.size	_RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal8selected22renormalize_with_bytes, .Lfunc_end1-_RNvNtCsaXBYq9Tq1IL_22mq_byte_select_minimal8selected22renormalize_with_bytes
	.cfi_endproc

	.ident	"rustc version 1.100.0 (0ef1bc386 2026-11-09)"
	.section	".note.GNU-stack","",@progbits
