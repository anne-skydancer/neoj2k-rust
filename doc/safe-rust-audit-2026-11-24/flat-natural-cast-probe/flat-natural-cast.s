	.att_syntax
	.file	"flat_natural_cast_main.7effe08e31002ef5-cgu.0"
	.section	.text._RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtB4_5array5GuardINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEECsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end0, nop
	.type	_RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtB4_5array5GuardINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEECsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtB4_5array5GuardINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEECsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	testq	%rsi, %rsi
	je	.LBB0_6
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	%rsi, %rbx
	movq	%rdi, %r14
	addq	$8, %r14
	movq	_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip), %r15
	jmp	.LBB0_2
	.p2align	4
.LBB0_4:
	addq	$24, %r14
	decq	%rbx
	je	.LBB0_5
.LBB0_2:
	movq	-8(%r14), %rsi
	testq	%rsi, %rsi
	je	.LBB0_4
	movq	(%r14), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*%r15
	jmp	.LBB0_4
.LBB0_5:
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	.cfi_restore %rbx
	.cfi_restore %r14
	.cfi_restore %r15
.LBB0_6:
	retq
.Lfunc_end0:
	.size	_RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtB4_5array5GuardINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEECsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end0-_RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtB4_5array5GuardINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RINvNtCs8uuN1nrGffB_4core5array11try_from_fnINtNtNtB4_3ops9try_trait17NeverShortCircuitINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEKj3_NCINvMBJ_BG_10wrap_mut_1jNCINvXsk_B2_B1n_NtB2_14SpecArrayClone5cloneKB1Y_E0E0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end1, nop
	.type	_RINvNtCs8uuN1nrGffB_4core5array11try_from_fnINtNtNtB4_3ops9try_trait17NeverShortCircuitINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEKj3_NCINvMBJ_BG_10wrap_mut_1jNCINvXsk_B2_B1n_NtB2_14SpecArrayClone5cloneKB1Y_E0E0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RINvNtCs8uuN1nrGffB_4core5array11try_from_fnINtNtNtB4_3ops9try_trait17NeverShortCircuitINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEKj3_NCINvMBJ_BG_10wrap_mut_1jNCINvXsk_B2_B1n_NtB2_14SpecArrayClone5cloneKB1Y_E0E0ECsaU0XrY2Q1Mf_22flat_natural_cast_main:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$88, %rsp
	.cfi_def_cfa_offset 144
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rsi, %r14
	movq	%rdi, %rbx
	movq	16(%rsi), %rbp
	movl	$4, %r12d
	movl	$4, %r13d
	testq	%rbp, %rbp
	je	.LBB1_7
	movq	8(%r14), %rax
	movq	%rax, 8(%rsp)
	leaq	(,%rbp,4), %r15
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	%r15, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB1_2
	movq	%rax, %r13
	movq	%rax, %rdi
	movq	8(%rsp), %rsi
	movq	%r15, %rdx
	callq	*memcpy@GOTPCREL(%rip)
.LBB1_7:
	movq	%rbp, 16(%rsp)
	movq	%r13, 24(%rsp)
	movq	%rbp, 32(%rsp)
	movq	40(%r14), %rbp
	testq	%rbp, %rbp
	je	.LBB1_11
	movq	32(%r14), %r13
	leaq	(,%rbp,4), %r15
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	%r15, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB1_9
	movq	%rax, %r12
	movq	%rax, %rdi
	movq	%r13, %rsi
	movq	%r15, %rdx
	callq	*memcpy@GOTPCREL(%rip)
.LBB1_11:
	movq	%rbp, 40(%rsp)
	movq	%r12, 48(%rsp)
	movq	%rbp, 56(%rsp)
	movq	64(%r14), %r13
	testq	%r13, %r13
	je	.LBB1_12
	movq	56(%r14), %r14
	leaq	(,%r13,4), %r15
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	%r15, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB1_14
	movq	%rax, %r12
	movq	%rax, %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	callq	*memcpy@GOTPCREL(%rip)
	jmp	.LBB1_16
.LBB1_12:
	movl	$4, %r12d
.LBB1_16:
	movq	%r13, 64(%rsp)
	movq	%r12, 72(%rsp)
	movq	%r13, 80(%rsp)
	vmovups	16(%rsp), %ymm0
	vmovups	%ymm0, (%rbx)
	movq	%r13, 64(%rbx)
	vmovups	48(%rsp), %xmm0
	vmovups	%xmm0, 32(%rbx)
	movq	64(%rsp), %rax
	movq	%rax, 48(%rbx)
	movq	72(%rsp), %rax
	movq	%rax, 56(%rbx)
	addq	$88, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.LBB1_2:
	.cfi_def_cfa_offset 144
	xorl	%ebx, %ebx
	jmp	.LBB1_3
.LBB1_9:
	movl	$1, %ebx
	jmp	.LBB1_3
.LBB1_14:
	movl	$2, %ebx
.LBB1_3:
.Ltmp0:
	movl	$4, %edi
	movq	%r15, %rsi
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.Ltmp1:
	ud2
.LBB1_5:
.Ltmp2:
	movq	%rax, %r14
	leaq	16(%rsp), %rdi
	movq	%rbx, %rsi
	callq	_RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtB4_5array5GuardINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEECsaU0XrY2Q1Mf_22flat_natural_cast_main
	movq	%r14, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end1:
	.size	_RINvNtCs8uuN1nrGffB_4core5array11try_from_fnINtNtNtB4_3ops9try_trait17NeverShortCircuitINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEKj3_NCINvMBJ_BG_10wrap_mut_1jNCINvXsk_B2_B1n_NtB2_14SpecArrayClone5cloneKB1Y_E0E0ECsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end1-_RINvNtCs8uuN1nrGffB_4core5array11try_from_fnINtNtNtB4_3ops9try_trait17NeverShortCircuitINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEKj3_NCINvMBJ_BG_10wrap_mut_1jNCINvXsk_B2_B1n_NtB2_14SpecArrayClone5cloneKB1Y_E0E0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc
	.section	.gcc_except_table._RINvNtCs8uuN1nrGffB_4core5array11try_from_fnINtNtNtB4_3ops9try_trait17NeverShortCircuitINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEKj3_NCINvMBJ_BG_10wrap_mut_1jNCINvXsk_B2_B1n_NtB2_14SpecArrayClone5cloneKB1Y_E0E0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,"a",@progbits
	.p2align	2, 0x0
GCC_except_table1:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Lfunc_begin0-.Lfunc_begin0
	.uleb128 .Ltmp0-.Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.uleb128 .Lfunc_end1-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	.text.unlikely._RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_BM_ECsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.type	_RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_BM_ECsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_BM_ECsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	leaq	24(%rsp), %rax
	movq	%rdi, (%rax)
	leaq	32(%rsp), %rcx
	movq	%rsi, (%rcx)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.20(%rip), %rdx
	movq	%rdx, 8(%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.2(%rip), %rdx
	xorl	%edi, %edi
	movq	%rax, %rsi
	movq	%rdx, %r8
	xorl	%r9d, %r9d
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking19assert_failed_inner@GOTPCREL(%rip)
.Lfunc_end2:
	.size	_RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_BM_ECsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end2-_RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_BM_ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.hidden	_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.globl	_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.prefalign	4, .Lfunc_end3, nop
	.type	_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	%ecx, %r8d
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, (%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.3(%rip), %rsi
	movq	%rsp, %rdi
	callq	*_RNvNtCsh0g7Hz38UNf_3std2rt19lang_start_internal@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end3:
	.size	_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end3-_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI4_0:
	.long	0x3fb374bc
.LCPI4_1:
	.long	0xce800000
.LCPI4_2:
	.long	0x4e800000
.LCPI4_3:
	.long	0x4effffff
.LCPI4_4:
	.long	0xbeb031cf
.LCPI4_5:
	.long	0xbf36d1e1
.LCPI4_6:
	.long	0x3fe2d0e5
	.section	.text._RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb0_EB6_,"ax",@progbits
	.prefalign	4, .Lfunc_end4, nop
	.type	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb0_EB6_,@function
_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb0_EB6_:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$72, %rsp
	.cfi_def_cfa_offset 128
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	cmpq	%rcx, %rsi
	cmovbq	%rsi, %rcx
	cmpq	%r9, %rcx
	cmovaeq	%r9, %rcx
	testq	%rcx, %rcx
	je	.LBB4_129
	movq	128(%rsp), %rsi
	movl	(%rsi), %r15d
	movl	4(%rsi), %eax
	movl	8(%rsi), %r14d
	subl	%r15d, %eax
	subl	%r15d, %r14d
	cmpl	%r14d, %eax
	jg	.LBB4_130
	cmpq	$8, %rcx
	jae	.LBB4_4
	xorl	%r10d, %r10d
	jmp	.LBB4_103
.LBB4_4:
	movabsq	$2305843009213693944, %r10
	andq	%rcx, %r10
	vmovd	%eax, %xmm0
	vpbroadcastd	%xmm0, %ymm0
	movl	%r14d, 12(%rsp)
	vmovd	%r14d, %xmm1
	vpbroadcastd	%xmm1, %ymm1
	movl	%r15d, 8(%rsp)
	vmovd	%r15d, %xmm2
	vpbroadcastd	%xmm2, %ymm2
	movq	%rcx, 24(%rsp)
	leaq	(,%rcx,4), %rcx
	andq	$-32, %rcx
	movq	%rcx, 32(%rsp)
	xorl	%ebx, %ebx
	vbroadcastss	.LCPI4_0(%rip), %ymm3
	vbroadcastss	.LCPI4_1(%rip), %ymm4
	vbroadcastss	.LCPI4_2(%rip), %ymm5
	vmovss	.LCPI4_3(%rip), %xmm6
	vbroadcastss	.LCPI4_4(%rip), %ymm7
	vbroadcastss	.LCPI4_5(%rip), %ymm8
	vbroadcastss	.LCPI4_6(%rip), %ymm9
	jmp	.LBB4_5
	.p2align	4
.LBB4_101:
	vmovd	%r12d, %xmm10
	vpinsrd	$1, %r15d, %xmm10, %xmm10
	vpinsrd	$2, %r13d, %xmm10, %xmm10
	vpinsrd	$3, %r11d, %xmm10, %xmm10
	vmovd	%ecx, %xmm11
	vpinsrd	$1, %r14d, %xmm11, %xmm11
	vpinsrd	$2, %ebp, %xmm11, %xmm11
	vpinsrd	$3, %esi, %xmm11, %xmm11
	vinserti128	$1, %xmm10, %ymm11, %ymm10
	vpcmpgtd	%ymm10, %ymm0, %ymm11
	vpminsd	%ymm1, %ymm10, %ymm10
	vblendvps	%ymm11, %ymm0, %ymm10, %ymm10
	vpaddd	%ymm2, %ymm10, %ymm10
	vmovdqu	%ymm10, (%r8,%rbx)
	addq	$32, %rbx
	cmpq	%rbx, 32(%rsp)
	je	.LBB4_102
.LBB4_5:
	vmovups	(%rdi,%rbx), %ymm10
	vmovups	(%r8,%rbx), %ymm11
	vmulps	%ymm3, %ymm11, %ymm12
	vaddps	%ymm12, %ymm10, %ymm12
	vmaxps	%ymm4, %ymm12, %ymm12
	vminps	%ymm5, %ymm12, %ymm12
	vroundps	$4, %ymm12, %ymm12
	vextractf128	$1, %ymm12, %xmm13
	vmovshdup	%xmm13, %xmm14
	vucomiss	%xmm6, %xmm14
	movl	$2147483647, 4(%rsp)
	movl	$2147483647, %ecx
	jbe	.LBB4_6
	vucomiss	%xmm14, %xmm14
	movl	$0, %r15d
	jnp	.LBB4_8
.LBB4_9:
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	jbe	.LBB4_10
.LBB4_11:
	vucomiss	%xmm13, %xmm13
	movl	$0, %r12d
	jnp	.LBB4_12
.LBB4_13:
	vshufpd	$1, %xmm13, %xmm13, %xmm14
	vucomiss	%xmm6, %xmm14
	movl	$2147483647, %ecx
	jbe	.LBB4_14
.LBB4_15:
	vucomiss	%xmm14, %xmm14
	movl	$0, %r13d
	jnp	.LBB4_16
.LBB4_17:
	vshufps	$255, %xmm13, %xmm13, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	jbe	.LBB4_18
.LBB4_19:
	vucomiss	%xmm13, %xmm13
	movl	$0, %r11d
	jnp	.LBB4_20
.LBB4_21:
	vmovshdup	%xmm12, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	jbe	.LBB4_22
.LBB4_23:
	vucomiss	%xmm13, %xmm13
	movl	$0, %r14d
	jnp	.LBB4_24
.LBB4_25:
	vucomiss	%xmm6, %xmm12
	movl	$2147483647, %r9d
	jbe	.LBB4_26
.LBB4_27:
	vucomiss	%xmm12, %xmm12
	movl	$0, %ecx
	jnp	.LBB4_28
.LBB4_29:
	vshufpd	$1, %xmm12, %xmm12, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %r9d
	jbe	.LBB4_30
.LBB4_31:
	vucomiss	%xmm13, %xmm13
	movl	$0, %ebp
	jnp	.LBB4_32
.LBB4_33:
	vshufps	$255, %xmm12, %xmm12, %xmm12
	vucomiss	%xmm6, %xmm12
	movl	$2147483647, %esi
	jbe	.LBB4_34
.LBB4_35:
	vucomiss	%xmm12, %xmm12
	movl	$0, %r9d
	jp	.LBB4_37
.LBB4_36:
	movl	%esi, %r9d
.LBB4_37:
	vmovd	%r12d, %xmm12
	vpinsrd	$1, %r15d, %xmm12, %xmm12
	vpinsrd	$2, %r13d, %xmm12, %xmm12
	vpinsrd	$3, %r11d, %xmm12, %xmm12
	vmovd	%ecx, %xmm13
	vpinsrd	$1, %r14d, %xmm13, %xmm13
	vpinsrd	$2, %ebp, %xmm13, %xmm13
	vpinsrd	$3, %r9d, %xmm13, %xmm13
	vinserti128	$1, %xmm12, %ymm13, %ymm12
	vpcmpgtd	%ymm12, %ymm0, %ymm13
	vpminsd	%ymm1, %ymm12, %ymm12
	vblendvps	%ymm13, %ymm0, %ymm12, %ymm13
	vmovups	(%rdx,%rbx), %ymm12
	vpaddd	%ymm2, %ymm13, %ymm13
	vmovdqu	%ymm13, (%rdi,%rbx)
	vmulps	%ymm7, %ymm12, %ymm13
	vaddps	%ymm13, %ymm10, %ymm13
	vmulps	%ymm8, %ymm11, %ymm11
	vaddps	%ymm11, %ymm13, %ymm11
	vmaxps	%ymm4, %ymm11, %ymm11
	vminps	%ymm5, %ymm11, %ymm11
	vroundps	$4, %ymm11, %ymm11
	vextractf128	$1, %ymm11, %xmm13
	vmovshdup	%xmm13, %xmm14
	vucomiss	%xmm6, %xmm14
	movl	$2147483647, %ecx
	jbe	.LBB4_38
	vucomiss	%xmm14, %xmm14
	movl	$0, %r15d
	jnp	.LBB4_40
.LBB4_41:
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	jbe	.LBB4_42
.LBB4_43:
	vucomiss	%xmm13, %xmm13
	movl	$0, %r12d
	jnp	.LBB4_44
.LBB4_45:
	vshufpd	$1, %xmm13, %xmm13, %xmm14
	vucomiss	%xmm6, %xmm14
	movl	$2147483647, %ecx
	jbe	.LBB4_46
.LBB4_47:
	vucomiss	%xmm14, %xmm14
	movl	$0, %r13d
	jnp	.LBB4_48
.LBB4_49:
	vshufps	$255, %xmm13, %xmm13, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	jbe	.LBB4_50
.LBB4_51:
	vucomiss	%xmm13, %xmm13
	movl	$0, %r11d
	jnp	.LBB4_52
.LBB4_53:
	vmovshdup	%xmm11, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	jbe	.LBB4_54
.LBB4_55:
	vucomiss	%xmm13, %xmm13
	movl	$0, %r14d
	jnp	.LBB4_56
.LBB4_57:
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %esi
	jbe	.LBB4_58
.LBB4_59:
	vucomiss	%xmm11, %xmm11
	movl	$0, %ecx
	jnp	.LBB4_60
.LBB4_61:
	vshufpd	$1, %xmm11, %xmm11, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %esi
	jbe	.LBB4_62
.LBB4_63:
	vucomiss	%xmm13, %xmm13
	movl	$0, %ebp
	jnp	.LBB4_64
.LBB4_65:
	vshufps	$255, %xmm11, %xmm11, %xmm11
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %esi
	jbe	.LBB4_66
.LBB4_67:
	vucomiss	%xmm11, %xmm11
	movl	$0, %r9d
	jp	.LBB4_69
.LBB4_68:
	movl	%esi, %r9d
.LBB4_69:
	vmovd	%r12d, %xmm11
	vpinsrd	$1, %r15d, %xmm11, %xmm11
	vpinsrd	$2, %r13d, %xmm11, %xmm11
	vpinsrd	$3, %r11d, %xmm11, %xmm11
	vmovd	%ecx, %xmm13
	vpinsrd	$1, %r14d, %xmm13, %xmm13
	vpinsrd	$2, %ebp, %xmm13, %xmm13
	vpinsrd	$3, %r9d, %xmm13, %xmm13
	vinserti128	$1, %xmm11, %ymm13, %ymm11
	vpcmpgtd	%ymm11, %ymm0, %ymm13
	vpminsd	%ymm1, %ymm11, %ymm11
	vblendvps	%ymm13, %ymm0, %ymm11, %ymm11
	vpaddd	%ymm2, %ymm11, %ymm11
	vmovdqu	%ymm11, (%rdx,%rbx)
	vmulps	%ymm9, %ymm12, %ymm11
	vaddps	%ymm11, %ymm10, %ymm10
	vmaxps	%ymm4, %ymm10, %ymm10
	vminps	%ymm5, %ymm10, %ymm10
	vroundps	$4, %ymm10, %ymm10
	vextractf128	$1, %ymm10, %xmm11
	vmovshdup	%xmm11, %xmm12
	vucomiss	%xmm6, %xmm12
	movl	$2147483647, %ecx
	jbe	.LBB4_70
	vucomiss	%xmm12, %xmm12
	movl	$0, %r15d
	jnp	.LBB4_72
.LBB4_73:
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %ecx
	jbe	.LBB4_74
.LBB4_75:
	vucomiss	%xmm11, %xmm11
	movl	$0, %r12d
	jnp	.LBB4_76
.LBB4_77:
	vshufpd	$1, %xmm11, %xmm11, %xmm12
	vucomiss	%xmm6, %xmm12
	movl	$2147483647, %ecx
	jbe	.LBB4_78
.LBB4_79:
	vucomiss	%xmm12, %xmm12
	movl	$0, %r13d
	jnp	.LBB4_80
.LBB4_81:
	vshufps	$255, %xmm11, %xmm11, %xmm11
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %ecx
	jbe	.LBB4_82
.LBB4_83:
	vucomiss	%xmm11, %xmm11
	movl	$0, %r11d
	jnp	.LBB4_84
.LBB4_85:
	vmovshdup	%xmm10, %xmm11
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %ecx
	jbe	.LBB4_86
.LBB4_87:
	vucomiss	%xmm11, %xmm11
	movl	$0, %r14d
	jnp	.LBB4_88
.LBB4_89:
	vucomiss	%xmm6, %xmm10
	movl	$2147483647, %esi
	jbe	.LBB4_90
.LBB4_91:
	vucomiss	%xmm10, %xmm10
	movl	$0, %ecx
	jnp	.LBB4_92
.LBB4_93:
	vshufpd	$1, %xmm10, %xmm10, %xmm11
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %esi
	jbe	.LBB4_94
.LBB4_95:
	vucomiss	%xmm11, %xmm11
	movl	$0, %ebp
	jnp	.LBB4_96
.LBB4_97:
	vshufps	$255, %xmm10, %xmm10, %xmm10
	vucomiss	%xmm6, %xmm10
	jbe	.LBB4_98
.LBB4_99:
	vucomiss	%xmm10, %xmm10
	movl	$0, %esi
	jp	.LBB4_101
	jmp	.LBB4_100
	.p2align	4
.LBB4_6:
	vcvttss2si	%xmm14, %ecx
	vucomiss	%xmm14, %xmm14
	movl	$0, %r15d
	jp	.LBB4_9
.LBB4_8:
	movl	%ecx, %r15d
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	ja	.LBB4_11
.LBB4_10:
	vcvttss2si	%xmm13, %ecx
	vucomiss	%xmm13, %xmm13
	movl	$0, %r12d
	jp	.LBB4_13
.LBB4_12:
	movl	%ecx, %r12d
	vshufpd	$1, %xmm13, %xmm13, %xmm14
	vucomiss	%xmm6, %xmm14
	movl	$2147483647, %ecx
	ja	.LBB4_15
.LBB4_14:
	vcvttss2si	%xmm14, %ecx
	vucomiss	%xmm14, %xmm14
	movl	$0, %r13d
	jp	.LBB4_17
.LBB4_16:
	movl	%ecx, %r13d
	vshufps	$255, %xmm13, %xmm13, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	ja	.LBB4_19
.LBB4_18:
	vcvttss2si	%xmm13, %ecx
	vucomiss	%xmm13, %xmm13
	movl	$0, %r11d
	jp	.LBB4_21
.LBB4_20:
	movl	%ecx, %r11d
	vmovshdup	%xmm12, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	ja	.LBB4_23
.LBB4_22:
	vcvttss2si	%xmm13, %ecx
	vucomiss	%xmm13, %xmm13
	movl	$0, %r14d
	jp	.LBB4_25
.LBB4_24:
	movl	%ecx, %r14d
	vucomiss	%xmm6, %xmm12
	movl	$2147483647, %r9d
	ja	.LBB4_27
.LBB4_26:
	vcvttss2si	%xmm12, %r9d
	vucomiss	%xmm12, %xmm12
	movl	$0, %ecx
	jp	.LBB4_29
.LBB4_28:
	movl	%r9d, %ecx
	vshufpd	$1, %xmm12, %xmm12, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %r9d
	ja	.LBB4_31
.LBB4_30:
	vcvttss2si	%xmm13, %r9d
	vucomiss	%xmm13, %xmm13
	movl	$0, %ebp
	jp	.LBB4_33
.LBB4_32:
	movl	%r9d, %ebp
	vshufps	$255, %xmm12, %xmm12, %xmm12
	vucomiss	%xmm6, %xmm12
	movl	$2147483647, %esi
	ja	.LBB4_35
.LBB4_34:
	vcvttss2si	%xmm12, %esi
	vucomiss	%xmm12, %xmm12
	movl	$0, %r9d
	jnp	.LBB4_36
	jmp	.LBB4_37
	.p2align	4
.LBB4_38:
	vcvttss2si	%xmm14, %ecx
	vucomiss	%xmm14, %xmm14
	movl	$0, %r15d
	jp	.LBB4_41
.LBB4_40:
	movl	%ecx, %r15d
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	ja	.LBB4_43
.LBB4_42:
	vcvttss2si	%xmm13, %ecx
	vucomiss	%xmm13, %xmm13
	movl	$0, %r12d
	jp	.LBB4_45
.LBB4_44:
	movl	%ecx, %r12d
	vshufpd	$1, %xmm13, %xmm13, %xmm14
	vucomiss	%xmm6, %xmm14
	movl	$2147483647, %ecx
	ja	.LBB4_47
.LBB4_46:
	vcvttss2si	%xmm14, %ecx
	vucomiss	%xmm14, %xmm14
	movl	$0, %r13d
	jp	.LBB4_49
.LBB4_48:
	movl	%ecx, %r13d
	vshufps	$255, %xmm13, %xmm13, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	ja	.LBB4_51
.LBB4_50:
	vcvttss2si	%xmm13, %ecx
	vucomiss	%xmm13, %xmm13
	movl	$0, %r11d
	jp	.LBB4_53
.LBB4_52:
	movl	%ecx, %r11d
	vmovshdup	%xmm11, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %ecx
	ja	.LBB4_55
.LBB4_54:
	vcvttss2si	%xmm13, %ecx
	vucomiss	%xmm13, %xmm13
	movl	$0, %r14d
	jp	.LBB4_57
.LBB4_56:
	movl	%ecx, %r14d
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %esi
	ja	.LBB4_59
.LBB4_58:
	vcvttss2si	%xmm11, %esi
	vucomiss	%xmm11, %xmm11
	movl	$0, %ecx
	jp	.LBB4_61
.LBB4_60:
	movl	%esi, %ecx
	vshufpd	$1, %xmm11, %xmm11, %xmm13
	vucomiss	%xmm6, %xmm13
	movl	$2147483647, %esi
	ja	.LBB4_63
.LBB4_62:
	vcvttss2si	%xmm13, %esi
	vucomiss	%xmm13, %xmm13
	movl	$0, %ebp
	jp	.LBB4_65
.LBB4_64:
	movl	%esi, %ebp
	vshufps	$255, %xmm11, %xmm11, %xmm11
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %esi
	ja	.LBB4_67
.LBB4_66:
	vcvttss2si	%xmm11, %esi
	vucomiss	%xmm11, %xmm11
	movl	$0, %r9d
	jnp	.LBB4_68
	jmp	.LBB4_69
	.p2align	4
.LBB4_70:
	vcvttss2si	%xmm12, %ecx
	vucomiss	%xmm12, %xmm12
	movl	$0, %r15d
	jp	.LBB4_73
.LBB4_72:
	movl	%ecx, %r15d
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %ecx
	ja	.LBB4_75
.LBB4_74:
	vcvttss2si	%xmm11, %ecx
	vucomiss	%xmm11, %xmm11
	movl	$0, %r12d
	jp	.LBB4_77
.LBB4_76:
	movl	%ecx, %r12d
	vshufpd	$1, %xmm11, %xmm11, %xmm12
	vucomiss	%xmm6, %xmm12
	movl	$2147483647, %ecx
	ja	.LBB4_79
.LBB4_78:
	vcvttss2si	%xmm12, %ecx
	vucomiss	%xmm12, %xmm12
	movl	$0, %r13d
	jp	.LBB4_81
.LBB4_80:
	movl	%ecx, %r13d
	vshufps	$255, %xmm11, %xmm11, %xmm11
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %ecx
	ja	.LBB4_83
.LBB4_82:
	vcvttss2si	%xmm11, %ecx
	vucomiss	%xmm11, %xmm11
	movl	$0, %r11d
	jp	.LBB4_85
.LBB4_84:
	movl	%ecx, %r11d
	vmovshdup	%xmm10, %xmm11
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %ecx
	ja	.LBB4_87
.LBB4_86:
	vcvttss2si	%xmm11, %ecx
	vucomiss	%xmm11, %xmm11
	movl	$0, %r14d
	jp	.LBB4_89
.LBB4_88:
	movl	%ecx, %r14d
	vucomiss	%xmm6, %xmm10
	movl	$2147483647, %esi
	ja	.LBB4_91
.LBB4_90:
	vcvttss2si	%xmm10, %esi
	vucomiss	%xmm10, %xmm10
	movl	$0, %ecx
	jp	.LBB4_93
.LBB4_92:
	movl	%esi, %ecx
	vshufpd	$1, %xmm10, %xmm10, %xmm11
	vucomiss	%xmm6, %xmm11
	movl	$2147483647, %esi
	ja	.LBB4_95
.LBB4_94:
	vcvttss2si	%xmm11, %esi
	vucomiss	%xmm11, %xmm11
	movl	$0, %ebp
	jp	.LBB4_97
.LBB4_96:
	movl	%esi, %ebp
	vshufps	$255, %xmm10, %xmm10, %xmm10
	vucomiss	%xmm6, %xmm10
	ja	.LBB4_99
.LBB4_98:
	vcvttss2si	%xmm10, %esi
	movl	%esi, 4(%rsp)
	vucomiss	%xmm10, %xmm10
	movl	$0, %esi
	jp	.LBB4_101
.LBB4_100:
	movl	4(%rsp), %esi
	jmp	.LBB4_101
.LBB4_102:
	movq	24(%rsp), %rcx
	cmpq	%r10, %rcx
	movl	12(%rsp), %r14d
	movl	8(%rsp), %r15d
	jne	.LBB4_103
.LBB4_129:
	addq	$72, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.LBB4_103:
	.cfi_def_cfa_offset 128
	vmovss	.LCPI4_0(%rip), %xmm0
	vmovss	.LCPI4_1(%rip), %xmm1
	vmovss	.LCPI4_2(%rip), %xmm2
	vmovss	.LCPI4_3(%rip), %xmm3
	vmovss	.LCPI4_4(%rip), %xmm4
	vmovss	.LCPI4_5(%rip), %xmm5
	vmovss	.LCPI4_6(%rip), %xmm6
	jmp	.LBB4_104
	.p2align	4
.LBB4_128:
	addl	%r15d, %esi
	movl	%esi, (%r8,%r10,4)
	incq	%r10
	cmpq	%r10, %rcx
	je	.LBB4_129
.LBB4_104:
	vmovss	(%rdi,%r10,4), %xmm7
	vmovss	(%r8,%r10,4), %xmm8
	vmulss	%xmm0, %xmm8, %xmm9
	vaddss	%xmm7, %xmm9, %xmm9
	vmaxss	%xmm1, %xmm9, %xmm9
	vminss	%xmm2, %xmm9, %xmm9
	vroundss	$4, %xmm9, %xmm9, %xmm9
	vucomiss	%xmm3, %xmm9
	movl	$2147483647, %ebx
	movl	$2147483647, %r9d
	jbe	.LBB4_105
	xorl	%r11d, %r11d
	vucomiss	%xmm9, %xmm9
	movl	$0, %ebp
	jnp	.LBB4_107
.LBB4_108:
	movl	%ebp, %r9d
	cmpl	%r14d, %ebp
	jge	.LBB4_109
.LBB4_110:
	movl	%eax, %esi
	cmpl	%eax, %ebp
	jl	.LBB4_112
.LBB4_111:
	movl	%r9d, %esi
.LBB4_112:
	vmovss	(%rdx,%r10,4), %xmm9
	addl	%r15d, %esi
	movl	%esi, (%rdi,%r10,4)
	vmulss	%xmm4, %xmm9, %xmm10
	vaddss	%xmm7, %xmm10, %xmm10
	vmulss	%xmm5, %xmm8, %xmm8
	vaddss	%xmm8, %xmm10, %xmm8
	vmaxss	%xmm1, %xmm8, %xmm8
	vminss	%xmm2, %xmm8, %xmm8
	vroundss	$4, %xmm8, %xmm8, %xmm8
	vucomiss	%xmm3, %xmm8
	movl	$2147483647, %esi
	jbe	.LBB4_113
	vucomiss	%xmm8, %xmm8
	movl	$0, %ebp
	jnp	.LBB4_115
.LBB4_116:
	movl	%ebp, %r9d
	cmpl	%r14d, %ebp
	jge	.LBB4_117
.LBB4_118:
	movl	%eax, %esi
	cmpl	%eax, %ebp
	jl	.LBB4_120
.LBB4_119:
	movl	%r9d, %esi
.LBB4_120:
	addl	%r15d, %esi
	movl	%esi, (%rdx,%r10,4)
	vmulss	%xmm6, %xmm9, %xmm8
	vaddss	%xmm7, %xmm8, %xmm7
	vmaxss	%xmm1, %xmm7, %xmm7
	vminss	%xmm2, %xmm7, %xmm7
	vroundss	$4, %xmm7, %xmm7, %xmm7
	vucomiss	%xmm3, %xmm7
	jbe	.LBB4_121
	vucomiss	%xmm7, %xmm7
	jnp	.LBB4_123
.LBB4_124:
	movl	%r11d, %r9d
	cmpl	%r14d, %r11d
	jge	.LBB4_125
.LBB4_126:
	movl	%eax, %esi
	cmpl	%eax, %r11d
	jl	.LBB4_128
	jmp	.LBB4_127
	.p2align	4
.LBB4_105:
	vcvttss2si	%xmm9, %r9d
	xorl	%r11d, %r11d
	vucomiss	%xmm9, %xmm9
	movl	$0, %ebp
	jp	.LBB4_108
.LBB4_107:
	movl	%r9d, %ebp
	movl	%ebp, %r9d
	cmpl	%r14d, %ebp
	jl	.LBB4_110
.LBB4_109:
	movl	%r14d, %r9d
	movl	%eax, %esi
	cmpl	%eax, %ebp
	jge	.LBB4_111
	jmp	.LBB4_112
	.p2align	4
.LBB4_113:
	vcvttss2si	%xmm8, %esi
	vucomiss	%xmm8, %xmm8
	movl	$0, %ebp
	jp	.LBB4_116
.LBB4_115:
	movl	%esi, %ebp
	movl	%ebp, %r9d
	cmpl	%r14d, %ebp
	jl	.LBB4_118
.LBB4_117:
	movl	%r14d, %r9d
	movl	%eax, %esi
	cmpl	%eax, %ebp
	jge	.LBB4_119
	jmp	.LBB4_120
	.p2align	4
.LBB4_121:
	vcvttss2si	%xmm7, %ebx
	vucomiss	%xmm7, %xmm7
	jp	.LBB4_124
.LBB4_123:
	movl	%ebx, %r11d
	movl	%r11d, %r9d
	cmpl	%r14d, %r11d
	jl	.LBB4_126
.LBB4_125:
	movl	%r14d, %r9d
	movl	%eax, %esi
	cmpl	%eax, %r11d
	jl	.LBB4_128
.LBB4_127:
	movl	%r9d, %esi
	jmp	.LBB4_128
.LBB4_130:
	movl	%eax, 16(%rsp)
	movl	%r14d, 20(%rsp)
	leaq	16(%rsp), %rax
	movq	%rax, 40(%rsp)
	leaq	_RNvXsQ_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_5Debug3fmt(%rip), %rax
	movq	%rax, 48(%rsp)
	leaq	20(%rsp), %rcx
	movq	%rcx, 56(%rsp)
	movq	%rax, 64(%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.22(%rip), %rdi
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.5(%rip), %rdx
	leaq	40(%rsp), %rsi
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.Lfunc_end4:
	.size	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb0_EB6_, .Lfunc_end4-_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb0_EB6_
	.cfi_endproc

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI5_0:
	.long	0x3fb374bc
.LCPI5_1:
	.long	0xce800000
.LCPI5_2:
	.long	0x4e800000
.LCPI5_3:
	.long	0x4effffff
.LCPI5_4:
	.long	0xbeb031cf
.LCPI5_5:
	.long	0xbf36d1e1
.LCPI5_6:
	.long	0x3fe2d0e5
	.section	.text._RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb1_EB6_,"ax",@progbits
	.prefalign	4, .Lfunc_end5, nop
	.type	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb1_EB6_,@function
_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb1_EB6_:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$72, %rsp
	.cfi_def_cfa_offset 128
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	cmpq	%rcx, %rsi
	cmovbq	%rsi, %rcx
	cmpq	%r9, %rcx
	cmovaeq	%r9, %rcx
	testq	%rcx, %rcx
	je	.LBB5_129
	movl	136(%rsp), %r14d
	movl	128(%rsp), %esi
	cmpl	%r14d, %esi
	jg	.LBB5_130
	cmpq	$8, %rcx
	jae	.LBB5_4
	xorl	%r9d, %r9d
	jmp	.LBB5_103
.LBB5_4:
	movabsq	$2305843009213693944, %r9
	andq	%rcx, %r9
	vmovd	%esi, %xmm0
	vpbroadcastd	%xmm0, %ymm0
	vmovd	%r14d, %xmm1
	vpbroadcastd	%xmm1, %ymm1
	movq	%rcx, 24(%rsp)
	leaq	(,%rcx,4), %rax
	andq	$-32, %rax
	movq	%rax, 32(%rsp)
	xorl	%r11d, %r11d
	vbroadcastss	.LCPI5_0(%rip), %ymm2
	vbroadcastss	.LCPI5_1(%rip), %ymm3
	vbroadcastss	.LCPI5_2(%rip), %ymm4
	vmovss	.LCPI5_3(%rip), %xmm5
	vbroadcastss	.LCPI5_4(%rip), %ymm6
	vbroadcastss	.LCPI5_5(%rip), %ymm7
	vbroadcastss	.LCPI5_6(%rip), %ymm8
	jmp	.LBB5_5
	.p2align	4
.LBB5_101:
	vmovd	%r15d, %xmm9
	vpinsrd	$1, %r14d, %xmm9, %xmm9
	vpinsrd	$2, %r12d, %xmm9, %xmm9
	vpinsrd	$3, %r13d, %xmm9, %xmm9
	vmovd	%ebp, %xmm10
	vpinsrd	$1, %r10d, %xmm10, %xmm10
	vpinsrd	$2, %ecx, %xmm10, %xmm10
	vpinsrd	$3, %eax, %xmm10, %xmm10
	vinserti128	$1, %xmm9, %ymm10, %ymm9
	vpcmpgtd	%ymm9, %ymm0, %ymm10
	vpminsd	%ymm1, %ymm9, %ymm9
	vblendvps	%ymm10, %ymm0, %ymm9, %ymm9
	vmovups	%ymm9, (%r8,%r11)
	addq	$32, %r11
	cmpq	%r11, 32(%rsp)
	je	.LBB5_102
.LBB5_5:
	vmovups	(%rdi,%r11), %ymm9
	vmovups	(%r8,%r11), %ymm11
	vmulps	%ymm2, %ymm11, %ymm10
	vaddps	%ymm10, %ymm9, %ymm10
	vmaxps	%ymm3, %ymm10, %ymm10
	vminps	%ymm4, %ymm10, %ymm10
	vroundps	$4, %ymm10, %ymm10
	vextractf128	$1, %ymm10, %xmm12
	vmovshdup	%xmm12, %xmm13
	vucomiss	%xmm5, %xmm13
	movl	$2147483647, 12(%rsp)
	movl	$2147483647, %eax
	jbe	.LBB5_6
	vucomiss	%xmm13, %xmm13
	movl	$0, %r14d
	jnp	.LBB5_8
.LBB5_9:
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	jbe	.LBB5_10
.LBB5_11:
	vucomiss	%xmm12, %xmm12
	movl	$0, %r15d
	jnp	.LBB5_12
.LBB5_13:
	vshufpd	$1, %xmm12, %xmm12, %xmm13
	vucomiss	%xmm5, %xmm13
	movl	$2147483647, %eax
	jbe	.LBB5_14
.LBB5_15:
	vucomiss	%xmm13, %xmm13
	movl	$0, %r12d
	jnp	.LBB5_16
.LBB5_17:
	vshufps	$255, %xmm12, %xmm12, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	jbe	.LBB5_18
.LBB5_19:
	vucomiss	%xmm12, %xmm12
	movl	$0, %r13d
	jnp	.LBB5_20
.LBB5_21:
	vmovshdup	%xmm10, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	jbe	.LBB5_22
.LBB5_23:
	vucomiss	%xmm12, %xmm12
	movl	$0, %r10d
	jnp	.LBB5_24
.LBB5_25:
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %eax
	jbe	.LBB5_26
.LBB5_27:
	vucomiss	%xmm10, %xmm10
	movl	$0, %ebp
	jnp	.LBB5_28
.LBB5_29:
	vshufpd	$1, %xmm10, %xmm10, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	jbe	.LBB5_30
.LBB5_31:
	vucomiss	%xmm12, %xmm12
	movl	$0, %ecx
	jnp	.LBB5_32
.LBB5_33:
	vshufps	$255, %xmm10, %xmm10, %xmm10
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %ebx
	jbe	.LBB5_34
.LBB5_35:
	vucomiss	%xmm10, %xmm10
	movl	$0, %eax
	jp	.LBB5_37
.LBB5_36:
	movl	%ebx, %eax
.LBB5_37:
	vmovd	%r15d, %xmm10
	vpinsrd	$1, %r14d, %xmm10, %xmm10
	vpinsrd	$2, %r12d, %xmm10, %xmm10
	vpinsrd	$3, %r13d, %xmm10, %xmm10
	vmovd	%ebp, %xmm12
	vpinsrd	$1, %r10d, %xmm12, %xmm12
	vpinsrd	$2, %ecx, %xmm12, %xmm12
	vpinsrd	$3, %eax, %xmm12, %xmm12
	vinserti128	$1, %xmm10, %ymm12, %ymm12
	vmovups	(%rdx,%r11), %ymm10
	vpcmpgtd	%ymm12, %ymm0, %ymm13
	vpminsd	%ymm1, %ymm12, %ymm12
	vblendvps	%ymm13, %ymm0, %ymm12, %ymm12
	vmovups	%ymm12, (%rdi,%r11)
	vmulps	%ymm6, %ymm10, %ymm12
	vaddps	%ymm12, %ymm9, %ymm12
	vmulps	%ymm7, %ymm11, %ymm11
	vaddps	%ymm11, %ymm12, %ymm11
	vmaxps	%ymm3, %ymm11, %ymm11
	vminps	%ymm4, %ymm11, %ymm11
	vroundps	$4, %ymm11, %ymm11
	vextractf128	$1, %ymm11, %xmm12
	vmovshdup	%xmm12, %xmm13
	vucomiss	%xmm5, %xmm13
	movl	$2147483647, %eax
	jbe	.LBB5_38
	vucomiss	%xmm13, %xmm13
	movl	$0, %r14d
	jnp	.LBB5_40
.LBB5_41:
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	jbe	.LBB5_42
.LBB5_43:
	vucomiss	%xmm12, %xmm12
	movl	$0, %r15d
	jnp	.LBB5_44
.LBB5_45:
	vshufpd	$1, %xmm12, %xmm12, %xmm13
	vucomiss	%xmm5, %xmm13
	movl	$2147483647, %eax
	jbe	.LBB5_46
.LBB5_47:
	vucomiss	%xmm13, %xmm13
	movl	$0, %r12d
	jnp	.LBB5_48
.LBB5_49:
	vshufps	$255, %xmm12, %xmm12, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	jbe	.LBB5_50
.LBB5_51:
	vucomiss	%xmm12, %xmm12
	movl	$0, %r13d
	jnp	.LBB5_52
.LBB5_53:
	vmovshdup	%xmm11, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	jbe	.LBB5_54
.LBB5_55:
	vucomiss	%xmm12, %xmm12
	movl	$0, %r10d
	jnp	.LBB5_56
.LBB5_57:
	vucomiss	%xmm5, %xmm11
	movl	$2147483647, %eax
	jbe	.LBB5_58
.LBB5_59:
	vucomiss	%xmm11, %xmm11
	movl	$0, %ebp
	jnp	.LBB5_60
.LBB5_61:
	vshufpd	$1, %xmm11, %xmm11, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	jbe	.LBB5_62
.LBB5_63:
	vucomiss	%xmm12, %xmm12
	movl	$0, %ecx
	jnp	.LBB5_64
.LBB5_65:
	vshufps	$255, %xmm11, %xmm11, %xmm11
	vucomiss	%xmm5, %xmm11
	movl	$2147483647, %ebx
	jbe	.LBB5_66
.LBB5_67:
	vucomiss	%xmm11, %xmm11
	movl	$0, %eax
	jp	.LBB5_69
.LBB5_68:
	movl	%ebx, %eax
.LBB5_69:
	vmovd	%r15d, %xmm11
	vpinsrd	$1, %r14d, %xmm11, %xmm11
	vpinsrd	$2, %r12d, %xmm11, %xmm11
	vpinsrd	$3, %r13d, %xmm11, %xmm11
	vmovd	%ebp, %xmm12
	vpinsrd	$1, %r10d, %xmm12, %xmm12
	vpinsrd	$2, %ecx, %xmm12, %xmm12
	vpinsrd	$3, %eax, %xmm12, %xmm12
	vinserti128	$1, %xmm11, %ymm12, %ymm11
	vpcmpgtd	%ymm11, %ymm0, %ymm12
	vpminsd	%ymm1, %ymm11, %ymm11
	vblendvps	%ymm12, %ymm0, %ymm11, %ymm11
	vmovups	%ymm11, (%rdx,%r11)
	vmulps	%ymm8, %ymm10, %ymm10
	vaddps	%ymm10, %ymm9, %ymm9
	vmaxps	%ymm3, %ymm9, %ymm9
	vminps	%ymm4, %ymm9, %ymm9
	vroundps	$4, %ymm9, %ymm9
	vextractf128	$1, %ymm9, %xmm10
	vmovshdup	%xmm10, %xmm11
	vucomiss	%xmm5, %xmm11
	movl	$2147483647, %eax
	jbe	.LBB5_70
	vucomiss	%xmm11, %xmm11
	movl	$0, %r14d
	jnp	.LBB5_72
.LBB5_73:
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %eax
	jbe	.LBB5_74
.LBB5_75:
	vucomiss	%xmm10, %xmm10
	movl	$0, %r15d
	jnp	.LBB5_76
.LBB5_77:
	vshufpd	$1, %xmm10, %xmm10, %xmm11
	vucomiss	%xmm5, %xmm11
	movl	$2147483647, %eax
	jbe	.LBB5_78
.LBB5_79:
	vucomiss	%xmm11, %xmm11
	movl	$0, %r12d
	jnp	.LBB5_80
.LBB5_81:
	vshufps	$255, %xmm10, %xmm10, %xmm10
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %eax
	jbe	.LBB5_82
.LBB5_83:
	vucomiss	%xmm10, %xmm10
	movl	$0, %r13d
	jnp	.LBB5_84
.LBB5_85:
	vmovshdup	%xmm9, %xmm10
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %eax
	jbe	.LBB5_86
.LBB5_87:
	vucomiss	%xmm10, %xmm10
	movl	$0, %r10d
	jnp	.LBB5_88
.LBB5_89:
	vucomiss	%xmm5, %xmm9
	movl	$2147483647, %eax
	jbe	.LBB5_90
.LBB5_91:
	vucomiss	%xmm9, %xmm9
	movl	$0, %ebp
	jnp	.LBB5_92
.LBB5_93:
	vshufpd	$1, %xmm9, %xmm9, %xmm10
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %eax
	jbe	.LBB5_94
.LBB5_95:
	vucomiss	%xmm10, %xmm10
	movl	$0, %ecx
	jnp	.LBB5_96
.LBB5_97:
	vshufps	$255, %xmm9, %xmm9, %xmm9
	vucomiss	%xmm5, %xmm9
	jbe	.LBB5_98
.LBB5_99:
	vucomiss	%xmm9, %xmm9
	movl	$0, %eax
	jp	.LBB5_101
	jmp	.LBB5_100
	.p2align	4
.LBB5_6:
	vcvttss2si	%xmm13, %eax
	vucomiss	%xmm13, %xmm13
	movl	$0, %r14d
	jp	.LBB5_9
.LBB5_8:
	movl	%eax, %r14d
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	ja	.LBB5_11
.LBB5_10:
	vcvttss2si	%xmm12, %eax
	vucomiss	%xmm12, %xmm12
	movl	$0, %r15d
	jp	.LBB5_13
.LBB5_12:
	movl	%eax, %r15d
	vshufpd	$1, %xmm12, %xmm12, %xmm13
	vucomiss	%xmm5, %xmm13
	movl	$2147483647, %eax
	ja	.LBB5_15
.LBB5_14:
	vcvttss2si	%xmm13, %eax
	vucomiss	%xmm13, %xmm13
	movl	$0, %r12d
	jp	.LBB5_17
.LBB5_16:
	movl	%eax, %r12d
	vshufps	$255, %xmm12, %xmm12, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	ja	.LBB5_19
.LBB5_18:
	vcvttss2si	%xmm12, %eax
	vucomiss	%xmm12, %xmm12
	movl	$0, %r13d
	jp	.LBB5_21
.LBB5_20:
	movl	%eax, %r13d
	vmovshdup	%xmm10, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	ja	.LBB5_23
.LBB5_22:
	vcvttss2si	%xmm12, %eax
	vucomiss	%xmm12, %xmm12
	movl	$0, %r10d
	jp	.LBB5_25
.LBB5_24:
	movl	%eax, %r10d
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %eax
	ja	.LBB5_27
.LBB5_26:
	vcvttss2si	%xmm10, %eax
	vucomiss	%xmm10, %xmm10
	movl	$0, %ebp
	jp	.LBB5_29
.LBB5_28:
	movl	%eax, %ebp
	vshufpd	$1, %xmm10, %xmm10, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	ja	.LBB5_31
.LBB5_30:
	vcvttss2si	%xmm12, %eax
	vucomiss	%xmm12, %xmm12
	movl	$0, %ecx
	jp	.LBB5_33
.LBB5_32:
	movl	%eax, %ecx
	vshufps	$255, %xmm10, %xmm10, %xmm10
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %ebx
	ja	.LBB5_35
.LBB5_34:
	vcvttss2si	%xmm10, %ebx
	vucomiss	%xmm10, %xmm10
	movl	$0, %eax
	jnp	.LBB5_36
	jmp	.LBB5_37
	.p2align	4
.LBB5_38:
	vcvttss2si	%xmm13, %eax
	vucomiss	%xmm13, %xmm13
	movl	$0, %r14d
	jp	.LBB5_41
.LBB5_40:
	movl	%eax, %r14d
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	ja	.LBB5_43
.LBB5_42:
	vcvttss2si	%xmm12, %eax
	vucomiss	%xmm12, %xmm12
	movl	$0, %r15d
	jp	.LBB5_45
.LBB5_44:
	movl	%eax, %r15d
	vshufpd	$1, %xmm12, %xmm12, %xmm13
	vucomiss	%xmm5, %xmm13
	movl	$2147483647, %eax
	ja	.LBB5_47
.LBB5_46:
	vcvttss2si	%xmm13, %eax
	vucomiss	%xmm13, %xmm13
	movl	$0, %r12d
	jp	.LBB5_49
.LBB5_48:
	movl	%eax, %r12d
	vshufps	$255, %xmm12, %xmm12, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	ja	.LBB5_51
.LBB5_50:
	vcvttss2si	%xmm12, %eax
	vucomiss	%xmm12, %xmm12
	movl	$0, %r13d
	jp	.LBB5_53
.LBB5_52:
	movl	%eax, %r13d
	vmovshdup	%xmm11, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	ja	.LBB5_55
.LBB5_54:
	vcvttss2si	%xmm12, %eax
	vucomiss	%xmm12, %xmm12
	movl	$0, %r10d
	jp	.LBB5_57
.LBB5_56:
	movl	%eax, %r10d
	vucomiss	%xmm5, %xmm11
	movl	$2147483647, %eax
	ja	.LBB5_59
.LBB5_58:
	vcvttss2si	%xmm11, %eax
	vucomiss	%xmm11, %xmm11
	movl	$0, %ebp
	jp	.LBB5_61
.LBB5_60:
	movl	%eax, %ebp
	vshufpd	$1, %xmm11, %xmm11, %xmm12
	vucomiss	%xmm5, %xmm12
	movl	$2147483647, %eax
	ja	.LBB5_63
.LBB5_62:
	vcvttss2si	%xmm12, %eax
	vucomiss	%xmm12, %xmm12
	movl	$0, %ecx
	jp	.LBB5_65
.LBB5_64:
	movl	%eax, %ecx
	vshufps	$255, %xmm11, %xmm11, %xmm11
	vucomiss	%xmm5, %xmm11
	movl	$2147483647, %ebx
	ja	.LBB5_67
.LBB5_66:
	vcvttss2si	%xmm11, %ebx
	vucomiss	%xmm11, %xmm11
	movl	$0, %eax
	jnp	.LBB5_68
	jmp	.LBB5_69
	.p2align	4
.LBB5_70:
	vcvttss2si	%xmm11, %eax
	vucomiss	%xmm11, %xmm11
	movl	$0, %r14d
	jp	.LBB5_73
.LBB5_72:
	movl	%eax, %r14d
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %eax
	ja	.LBB5_75
.LBB5_74:
	vcvttss2si	%xmm10, %eax
	vucomiss	%xmm10, %xmm10
	movl	$0, %r15d
	jp	.LBB5_77
.LBB5_76:
	movl	%eax, %r15d
	vshufpd	$1, %xmm10, %xmm10, %xmm11
	vucomiss	%xmm5, %xmm11
	movl	$2147483647, %eax
	ja	.LBB5_79
.LBB5_78:
	vcvttss2si	%xmm11, %eax
	vucomiss	%xmm11, %xmm11
	movl	$0, %r12d
	jp	.LBB5_81
.LBB5_80:
	movl	%eax, %r12d
	vshufps	$255, %xmm10, %xmm10, %xmm10
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %eax
	ja	.LBB5_83
.LBB5_82:
	vcvttss2si	%xmm10, %eax
	vucomiss	%xmm10, %xmm10
	movl	$0, %r13d
	jp	.LBB5_85
.LBB5_84:
	movl	%eax, %r13d
	vmovshdup	%xmm9, %xmm10
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %eax
	ja	.LBB5_87
.LBB5_86:
	vcvttss2si	%xmm10, %eax
	vucomiss	%xmm10, %xmm10
	movl	$0, %r10d
	jp	.LBB5_89
.LBB5_88:
	movl	%eax, %r10d
	vucomiss	%xmm5, %xmm9
	movl	$2147483647, %eax
	ja	.LBB5_91
.LBB5_90:
	vcvttss2si	%xmm9, %eax
	vucomiss	%xmm9, %xmm9
	movl	$0, %ebp
	jp	.LBB5_93
.LBB5_92:
	movl	%eax, %ebp
	vshufpd	$1, %xmm9, %xmm9, %xmm10
	vucomiss	%xmm5, %xmm10
	movl	$2147483647, %eax
	ja	.LBB5_95
.LBB5_94:
	vcvttss2si	%xmm10, %eax
	vucomiss	%xmm10, %xmm10
	movl	$0, %ecx
	jp	.LBB5_97
.LBB5_96:
	movl	%eax, %ecx
	vshufps	$255, %xmm9, %xmm9, %xmm9
	vucomiss	%xmm5, %xmm9
	ja	.LBB5_99
.LBB5_98:
	vcvttss2si	%xmm9, %eax
	movl	%eax, 12(%rsp)
	vucomiss	%xmm9, %xmm9
	movl	$0, %eax
	jp	.LBB5_101
.LBB5_100:
	movl	12(%rsp), %eax
	jmp	.LBB5_101
.LBB5_102:
	movq	24(%rsp), %rcx
	cmpq	%r9, %rcx
	movl	136(%rsp), %r14d
	jne	.LBB5_103
.LBB5_129:
	addq	$72, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.LBB5_103:
	.cfi_def_cfa_offset 128
	vmovss	.LCPI5_0(%rip), %xmm0
	vmovss	.LCPI5_1(%rip), %xmm1
	vmovss	.LCPI5_2(%rip), %xmm2
	vmovss	.LCPI5_3(%rip), %xmm3
	vmovss	.LCPI5_4(%rip), %xmm4
	vmovss	.LCPI5_5(%rip), %xmm5
	vmovss	.LCPI5_6(%rip), %xmm6
	jmp	.LBB5_104
	.p2align	4
.LBB5_128:
	movl	%r11d, (%r8,%r9,4)
	incq	%r9
	cmpq	%r9, %rcx
	je	.LBB5_129
.LBB5_104:
	vmovss	(%rdi,%r9,4), %xmm7
	vmovss	(%r8,%r9,4), %xmm8
	vmulss	%xmm0, %xmm8, %xmm9
	vaddss	%xmm7, %xmm9, %xmm9
	vmaxss	%xmm1, %xmm9, %xmm9
	vminss	%xmm2, %xmm9, %xmm9
	vroundss	$4, %xmm9, %xmm9, %xmm9
	vucomiss	%xmm3, %xmm9
	movl	$2147483647, %r11d
	movl	$2147483647, %eax
	jbe	.LBB5_105
	xorl	%r10d, %r10d
	vucomiss	%xmm9, %xmm9
	movl	$0, %ebp
	jnp	.LBB5_107
.LBB5_108:
	movl	%ebp, %eax
	cmpl	%r14d, %ebp
	jge	.LBB5_109
.LBB5_110:
	movl	%esi, %ebx
	cmpl	%esi, %ebp
	jl	.LBB5_112
.LBB5_111:
	movl	%eax, %ebx
.LBB5_112:
	vmovss	(%rdx,%r9,4), %xmm9
	movl	%ebx, (%rdi,%r9,4)
	vmulss	%xmm4, %xmm9, %xmm10
	vaddss	%xmm7, %xmm10, %xmm10
	vmulss	%xmm5, %xmm8, %xmm8
	vaddss	%xmm8, %xmm10, %xmm8
	vmaxss	%xmm1, %xmm8, %xmm8
	vminss	%xmm2, %xmm8, %xmm8
	vroundss	$4, %xmm8, %xmm8, %xmm8
	vucomiss	%xmm3, %xmm8
	movl	$2147483647, %eax
	jbe	.LBB5_113
	vucomiss	%xmm8, %xmm8
	movl	$0, %ebp
	jnp	.LBB5_115
.LBB5_116:
	movl	%ebp, %eax
	cmpl	%r14d, %ebp
	jge	.LBB5_117
.LBB5_118:
	movl	%esi, %ebx
	cmpl	%esi, %ebp
	jl	.LBB5_120
.LBB5_119:
	movl	%eax, %ebx
.LBB5_120:
	movl	%ebx, (%rdx,%r9,4)
	vmulss	%xmm6, %xmm9, %xmm8
	vaddss	%xmm7, %xmm8, %xmm7
	vmaxss	%xmm1, %xmm7, %xmm7
	vminss	%xmm2, %xmm7, %xmm7
	vroundss	$4, %xmm7, %xmm7, %xmm7
	vucomiss	%xmm3, %xmm7
	jbe	.LBB5_121
	vucomiss	%xmm7, %xmm7
	jnp	.LBB5_123
.LBB5_124:
	movl	%r10d, %eax
	cmpl	%r14d, %r10d
	jge	.LBB5_125
.LBB5_126:
	movl	%esi, %r11d
	cmpl	%esi, %r10d
	jl	.LBB5_128
	jmp	.LBB5_127
	.p2align	4
.LBB5_105:
	vcvttss2si	%xmm9, %eax
	xorl	%r10d, %r10d
	vucomiss	%xmm9, %xmm9
	movl	$0, %ebp
	jp	.LBB5_108
.LBB5_107:
	movl	%eax, %ebp
	movl	%ebp, %eax
	cmpl	%r14d, %ebp
	jl	.LBB5_110
.LBB5_109:
	movl	%r14d, %eax
	movl	%esi, %ebx
	cmpl	%esi, %ebp
	jge	.LBB5_111
	jmp	.LBB5_112
	.p2align	4
.LBB5_113:
	vcvttss2si	%xmm8, %eax
	vucomiss	%xmm8, %xmm8
	movl	$0, %ebp
	jp	.LBB5_116
.LBB5_115:
	movl	%eax, %ebp
	movl	%ebp, %eax
	cmpl	%r14d, %ebp
	jl	.LBB5_118
.LBB5_117:
	movl	%r14d, %eax
	movl	%esi, %ebx
	cmpl	%esi, %ebp
	jge	.LBB5_119
	jmp	.LBB5_120
	.p2align	4
.LBB5_121:
	vcvttss2si	%xmm7, %r11d
	vucomiss	%xmm7, %xmm7
	jp	.LBB5_124
.LBB5_123:
	movl	%r11d, %r10d
	movl	%r10d, %eax
	cmpl	%r14d, %r10d
	jl	.LBB5_126
.LBB5_125:
	movl	%r14d, %eax
	movl	%esi, %r11d
	cmpl	%esi, %r10d
	jl	.LBB5_128
.LBB5_127:
	movl	%eax, %r11d
	jmp	.LBB5_128
.LBB5_130:
	movl	%esi, 16(%rsp)
	movl	%r14d, 20(%rsp)
	leaq	16(%rsp), %rax
	movq	%rax, 40(%rsp)
	leaq	_RNvXsQ_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_5Debug3fmt(%rip), %rax
	movq	%rax, 48(%rsp)
	leaq	20(%rsp), %rcx
	movq	%rcx, 56(%rsp)
	movq	%rax, 64(%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.22(%rip), %rdi
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.5(%rip), %rdx
	leaq	40(%rsp), %rsi
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.Lfunc_end5:
	.size	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb1_EB6_, .Lfunc_end5-_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb1_EB6_
	.cfi_endproc

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI6_0:
	.long	0x3fb374bc
.LCPI6_1:
	.long	0x4f400000
.LCPI6_2:
	.long	0x4f40c000
.LCPI6_3:
	.long	0xbeb031cf
.LCPI6_4:
	.long	0xbf36d1e1
.LCPI6_5:
	.long	0x3fe2d0e5
	.section	.text._RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb0_EB6_,"ax",@progbits
	.prefalign	4, .Lfunc_end6, nop
	.type	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb0_EB6_,@function
_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb0_EB6_:
	.cfi_startproc
	cmpq	%rcx, %rsi
	cmovbq	%rsi, %rcx
	cmpq	%r9, %rcx
	cmovaeq	%r9, %rcx
	testq	%rcx, %rcx
	je	.LBB6_9
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	movq	16(%rsp), %rsi
	movl	(%rsi), %eax
	movl	4(%rsi), %r9d
	movl	8(%rsi), %esi
	subl	%eax, %r9d
	vcvtsi2ss	%r9d, %xmm15, %xmm0
	movl	%esi, %r9d
	subl	%eax, %r9d
	vcvtsi2ss	%r9d, %xmm15, %xmm1
	addl	$1958739968, %eax
	cmpq	$8, %rcx
	jae	.LBB6_3
	xorl	%r9d, %r9d
	jmp	.LBB6_6
.LBB6_3:
	movabsq	$2305843009213693944, %r9
	andq	%rcx, %r9
	vbroadcastss	%xmm0, %ymm2
	vbroadcastss	%xmm1, %ymm3
	vmovd	%eax, %xmm4
	vpbroadcastd	%xmm4, %ymm4
	vmovd	%esi, %xmm5
	vpbroadcastd	%xmm5, %ymm5
	leaq	(,%rcx,4), %r10
	andq	$-32, %r10
	xorl	%r11d, %r11d
	vbroadcastss	.LCPI6_1(%rip), %ymm7
	vbroadcastss	.LCPI6_2(%rip), %ymm8
	vbroadcastss	.LCPI6_3(%rip), %ymm9
	vbroadcastss	.LCPI6_4(%rip), %ymm10
	vbroadcastss	.LCPI6_5(%rip), %ymm11
	.p2align	4
.LBB6_4:
	vmovups	(%rdi,%r11), %ymm12
	vmovups	(%r8,%r11), %ymm13
	vbroadcastss	.LCPI6_0(%rip), %ymm6
	vmulps	%ymm6, %ymm13, %ymm14
	vaddps	%ymm14, %ymm12, %ymm14
	vmaxps	%ymm2, %ymm14, %ymm14
	vminps	%ymm3, %ymm14, %ymm14
	vaddps	%ymm7, %ymm14, %ymm15
	vsubps	%ymm15, %ymm8, %ymm6
	vaddps	%ymm6, %ymm14, %ymm6
	vpslld	$8, %ymm15, %ymm14
	vpaddd	%ymm4, %ymm14, %ymm14
	vpaddd	%ymm6, %ymm14, %ymm6
	vpminud	%ymm5, %ymm6, %ymm6
	vmovdqu	%ymm6, (%rdi,%r11)
	vmovups	(%rdx,%r11), %ymm6
	vmulps	%ymm6, %ymm9, %ymm14
	vaddps	%ymm14, %ymm12, %ymm14
	vmulps	%ymm10, %ymm13, %ymm13
	vaddps	%ymm13, %ymm14, %ymm13
	vmaxps	%ymm2, %ymm13, %ymm13
	vminps	%ymm3, %ymm13, %ymm13
	vaddps	%ymm7, %ymm13, %ymm14
	vsubps	%ymm14, %ymm8, %ymm15
	vaddps	%ymm15, %ymm13, %ymm13
	vpslld	$8, %ymm14, %ymm14
	vpaddd	%ymm4, %ymm14, %ymm14
	vpaddd	%ymm13, %ymm14, %ymm13
	vpminud	%ymm5, %ymm13, %ymm13
	vmovdqu	%ymm13, (%rdx,%r11)
	vmulps	%ymm6, %ymm11, %ymm6
	vaddps	%ymm6, %ymm12, %ymm6
	vmaxps	%ymm2, %ymm6, %ymm6
	vminps	%ymm3, %ymm6, %ymm6
	vaddps	%ymm7, %ymm6, %ymm12
	vsubps	%ymm12, %ymm8, %ymm13
	vaddps	%ymm6, %ymm13, %ymm6
	vpslld	$8, %ymm12, %ymm12
	vpaddd	%ymm4, %ymm12, %ymm12
	vpaddd	%ymm6, %ymm12, %ymm6
	vpminud	%ymm5, %ymm6, %ymm6
	vmovdqu	%ymm6, (%r8,%r11)
	addq	$32, %r11
	cmpq	%r11, %r10
	jne	.LBB6_4
	cmpq	%r9, %rcx
	je	.LBB6_8
.LBB6_6:
	vmovss	.LCPI6_0(%rip), %xmm2
	vmovss	.LCPI6_1(%rip), %xmm3
	vmovss	.LCPI6_2(%rip), %xmm4
	vmovss	.LCPI6_3(%rip), %xmm5
	vmovss	.LCPI6_4(%rip), %xmm6
	vmovss	.LCPI6_5(%rip), %xmm7
	.p2align	4
.LBB6_7:
	vmovss	(%rdi,%r9,4), %xmm8
	vmovss	(%rdx,%r9,4), %xmm9
	vmovss	(%r8,%r9,4), %xmm10
	vmulss	%xmm2, %xmm10, %xmm11
	vaddss	%xmm11, %xmm8, %xmm11
	vmaxss	%xmm0, %xmm11, %xmm11
	vminss	%xmm1, %xmm11, %xmm11
	vaddss	%xmm3, %xmm11, %xmm12
	vsubss	%xmm12, %xmm4, %xmm13
	vaddss	%xmm13, %xmm11, %xmm11
	vmovd	%xmm12, %r10d
	shll	$8, %r10d
	vmovd	%xmm11, %r11d
	addl	%eax, %r10d
	addl	%r11d, %r10d
	cmpl	%esi, %r10d
	cmovael	%esi, %r10d
	vmulss	%xmm5, %xmm9, %xmm11
	vaddss	%xmm11, %xmm8, %xmm11
	vmulss	%xmm6, %xmm10, %xmm10
	vaddss	%xmm10, %xmm11, %xmm10
	vmaxss	%xmm0, %xmm10, %xmm10
	vminss	%xmm1, %xmm10, %xmm10
	vaddss	%xmm3, %xmm10, %xmm11
	vsubss	%xmm11, %xmm4, %xmm12
	vaddss	%xmm12, %xmm10, %xmm10
	vmovd	%xmm11, %r11d
	shll	$8, %r11d
	vmovd	%xmm10, %ebx
	addl	%eax, %r11d
	addl	%ebx, %r11d
	cmpl	%esi, %r11d
	cmovael	%esi, %r11d
	movl	%r10d, (%rdi,%r9,4)
	vmulss	%xmm7, %xmm9, %xmm9
	vaddss	%xmm9, %xmm8, %xmm8
	vmaxss	%xmm0, %xmm8, %xmm8
	vminss	%xmm1, %xmm8, %xmm8
	vaddss	%xmm3, %xmm8, %xmm9
	vsubss	%xmm9, %xmm4, %xmm10
	vaddss	%xmm10, %xmm8, %xmm8
	vmovd	%xmm9, %r10d
	shll	$8, %r10d
	vmovd	%xmm8, %ebx
	addl	%eax, %r10d
	addl	%ebx, %r10d
	cmpl	%esi, %r10d
	cmovael	%esi, %r10d
	movl	%r11d, (%rdx,%r9,4)
	movl	%r10d, (%r8,%r9,4)
	incq	%r9
	cmpq	%r9, %rcx
	jne	.LBB6_7
.LBB6_8:
	popq	%rbx
	.cfi_def_cfa_offset 8
	.cfi_restore %rbx
.LBB6_9:
	vzeroupper
	retq
.Lfunc_end6:
	.size	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb0_EB6_, .Lfunc_end6-_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb0_EB6_
	.cfi_endproc

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI7_0:
	.long	0x3fb374bc
.LCPI7_1:
	.long	0x4f400000
.LCPI7_2:
	.long	0x4f40c000
.LCPI7_3:
	.long	1958739968
.LCPI7_4:
	.long	0xbeb031cf
.LCPI7_5:
	.long	0xbf36d1e1
.LCPI7_6:
	.long	0x3fe2d0e5
	.section	.text._RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb1_EB6_,"ax",@progbits
	.prefalign	4, .Lfunc_end7, nop
	.type	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb1_EB6_,@function
_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb1_EB6_:
	.cfi_startproc
	cmpq	%rcx, %rsi
	cmovbq	%rsi, %rcx
	cmpq	%r9, %rcx
	cmovaeq	%r9, %rcx
	testq	%rcx, %rcx
	je	.LBB7_8
	vcvtsi2ssl	8(%rsp), %xmm15, %xmm0
	movl	16(%rsp), %eax
	vcvtsi2ss	%eax, %xmm15, %xmm1
	cmpq	$8, %rcx
	jae	.LBB7_3
	xorl	%esi, %esi
	jmp	.LBB7_6
.LBB7_3:
	movabsq	$2305843009213693944, %rsi
	andq	%rcx, %rsi
	vbroadcastss	%xmm0, %ymm2
	vbroadcastss	%xmm1, %ymm3
	vmovd	%eax, %xmm4
	vpbroadcastd	%xmm4, %ymm4
	leaq	(,%rcx,4), %r9
	andq	$-32, %r9
	xorl	%r10d, %r10d
	vbroadcastss	.LCPI7_1(%rip), %ymm6
	vbroadcastss	.LCPI7_2(%rip), %ymm7
	vpbroadcastd	.LCPI7_3(%rip), %ymm8
	vbroadcastss	.LCPI7_4(%rip), %ymm9
	vbroadcastss	.LCPI7_5(%rip), %ymm10
	vbroadcastss	.LCPI7_6(%rip), %ymm11
	.p2align	4
.LBB7_4:
	vmovups	(%rdi,%r10), %ymm12
	vmovups	(%r8,%r10), %ymm13
	vbroadcastss	.LCPI7_0(%rip), %ymm5
	vmulps	%ymm5, %ymm13, %ymm14
	vaddps	%ymm14, %ymm12, %ymm14
	vmaxps	%ymm2, %ymm14, %ymm14
	vminps	%ymm3, %ymm14, %ymm14
	vaddps	%ymm6, %ymm14, %ymm15
	vsubps	%ymm15, %ymm7, %ymm5
	vaddps	%ymm5, %ymm14, %ymm5
	vpslld	$8, %ymm15, %ymm14
	vpaddd	%ymm8, %ymm14, %ymm14
	vpaddd	%ymm5, %ymm14, %ymm5
	vpminsd	%ymm4, %ymm5, %ymm5
	vmovdqu	%ymm5, (%rdi,%r10)
	vmovups	(%rdx,%r10), %ymm5
	vmulps	%ymm5, %ymm9, %ymm14
	vaddps	%ymm14, %ymm12, %ymm14
	vmulps	%ymm10, %ymm13, %ymm13
	vaddps	%ymm13, %ymm14, %ymm13
	vmaxps	%ymm2, %ymm13, %ymm13
	vminps	%ymm3, %ymm13, %ymm13
	vaddps	%ymm6, %ymm13, %ymm14
	vsubps	%ymm14, %ymm7, %ymm15
	vaddps	%ymm15, %ymm13, %ymm13
	vpslld	$8, %ymm14, %ymm14
	vpaddd	%ymm8, %ymm14, %ymm14
	vpaddd	%ymm13, %ymm14, %ymm13
	vpminsd	%ymm4, %ymm13, %ymm13
	vmovdqu	%ymm13, (%rdx,%r10)
	vmulps	%ymm5, %ymm11, %ymm5
	vaddps	%ymm5, %ymm12, %ymm5
	vmaxps	%ymm2, %ymm5, %ymm5
	vminps	%ymm3, %ymm5, %ymm5
	vaddps	%ymm6, %ymm5, %ymm12
	vsubps	%ymm12, %ymm7, %ymm13
	vaddps	%ymm5, %ymm13, %ymm5
	vpslld	$8, %ymm12, %ymm12
	vpaddd	%ymm8, %ymm12, %ymm12
	vpaddd	%ymm5, %ymm12, %ymm5
	vpminsd	%ymm4, %ymm5, %ymm5
	vmovdqu	%ymm5, (%r8,%r10)
	addq	$32, %r10
	cmpq	%r10, %r9
	jne	.LBB7_4
	cmpq	%rsi, %rcx
	je	.LBB7_8
.LBB7_6:
	vmovss	.LCPI7_0(%rip), %xmm2
	vmovss	.LCPI7_1(%rip), %xmm3
	vmovss	.LCPI7_2(%rip), %xmm4
	vmovss	.LCPI7_4(%rip), %xmm5
	vmovss	.LCPI7_5(%rip), %xmm6
	vmovss	.LCPI7_6(%rip), %xmm7
	.p2align	4
.LBB7_7:
	vmovss	(%rdi,%rsi,4), %xmm8
	vmovss	(%r8,%rsi,4), %xmm9
	vmulss	%xmm2, %xmm9, %xmm10
	vaddss	%xmm10, %xmm8, %xmm10
	vmaxss	%xmm0, %xmm10, %xmm10
	vminss	%xmm1, %xmm10, %xmm10
	vaddss	%xmm3, %xmm10, %xmm11
	vsubss	%xmm11, %xmm4, %xmm12
	vaddss	%xmm12, %xmm10, %xmm10
	vmovd	%xmm11, %r9d
	shll	$8, %r9d
	vmovd	%xmm10, %r10d
	addl	%r10d, %r9d
	addl	$1958739968, %r9d
	cmpl	%eax, %r9d
	cmovgel	%eax, %r9d
	vmovss	(%rdx,%rsi,4), %xmm10
	movl	%r9d, (%rdi,%rsi,4)
	vmulss	%xmm5, %xmm10, %xmm11
	vaddss	%xmm11, %xmm8, %xmm11
	vmulss	%xmm6, %xmm9, %xmm9
	vaddss	%xmm9, %xmm11, %xmm9
	vmaxss	%xmm0, %xmm9, %xmm9
	vminss	%xmm1, %xmm9, %xmm9
	vaddss	%xmm3, %xmm9, %xmm11
	vsubss	%xmm11, %xmm4, %xmm12
	vaddss	%xmm12, %xmm9, %xmm9
	vmovd	%xmm11, %r9d
	shll	$8, %r9d
	vmovd	%xmm9, %r10d
	addl	%r10d, %r9d
	addl	$1958739968, %r9d
	cmpl	%eax, %r9d
	cmovgel	%eax, %r9d
	vmulss	%xmm7, %xmm10, %xmm9
	vaddss	%xmm9, %xmm8, %xmm8
	vmaxss	%xmm0, %xmm8, %xmm8
	vminss	%xmm1, %xmm8, %xmm8
	vaddss	%xmm3, %xmm8, %xmm9
	vsubss	%xmm9, %xmm4, %xmm10
	vaddss	%xmm10, %xmm8, %xmm8
	vmovd	%xmm9, %r10d
	shll	$8, %r10d
	vmovd	%xmm8, %r11d
	addl	%r11d, %r10d
	addl	$1958739968, %r10d
	cmpl	%eax, %r10d
	cmovgel	%eax, %r10d
	movl	%r9d, (%rdx,%rsi,4)
	movl	%r10d, (%r8,%rsi,4)
	incq	%rsi
	cmpq	%rsi, %rcx
	jne	.LBB7_7
.LBB7_8:
	vzeroupper
	retq
.Lfunc_end7:
	.size	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb1_EB6_, .Lfunc_end7-_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb1_EB6_
	.cfi_endproc

	.section	.text._RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end8, nop
	.type	_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	*%rdi
	#APP
	#NO_APP
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end8:
	.size	_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end8-_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end9, nop
	.type	_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECsaU0XrY2Q1Mf_22flat_natural_cast_main:
.Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception1
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r13
	.cfi_def_cfa_offset 32
	pushq	%r12
	.cfi_def_cfa_offset 40
	pushq	%rbx
	.cfi_def_cfa_offset 48
	subq	$4096, %rsp
	.cfi_def_cfa_offset 4144
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r13, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	%rsi, %rcx
	shrq	%rcx
	movq	%rsi, %rax
	subq	%rcx, %rax
	cmpq	$1000000, %rsi
	movl	$1000000, %ebx
	cmovbq	%rsi, %rbx
	cmpq	%rbx, %rax
	cmovaq	%rax, %rbx
	cmpq	$513, %rbx
	jb	.LBB9_1
	leaq	(,%rbx,8), %r15
	shrq	$61, %rax
	setne	%al
	movabsq	$9223372036854775800, %rcx
	cmpq	%rcx, %r15
	seta	%cl
	orb	%al, %cl
	je	.LBB9_7
	xorl	%r12d, %r12d
.LBB9_8:
	movq	%r12, %rdi
	movq	%r15, %rsi
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.LBB9_1:
	movq	%rsp, %r14
	movl	$512, %ecx
	jmp	.LBB9_2
.LBB9_7:
	movq	%rsi, %r14
	movq	%rdi, %r13
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$8, %r12d
	movl	$8, %esi
	movq	%r15, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	movq	%r13, %rdi
	movq	%r14, %rsi
	movq	%rax, %r14
	movq	%rbx, %rcx
	testq	%rax, %rax
	je	.LBB9_8
.LBB9_2:
	xorl	%r8d, %r8d
	cmpq	$65, %rsi
	setb	%r8b
.Ltmp3:
	movq	%r14, %rdx
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
.Ltmp4:
	cmpq	$512, %rbx
	jbe	.LBB9_4
	shlq	$3, %rbx
	movl	$8, %edx
	movq	%r14, %rdi
	movq	%rbx, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB9_4:
	addq	$4096, %rsp
	.cfi_def_cfa_offset 48
	popq	%rbx
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.LBB9_10:
	.cfi_def_cfa_offset 4144
.Ltmp5:
	movq	%rax, %r15
	cmpq	$512, %rbx
	jbe	.LBB9_12
	shlq	$3, %rbx
	movl	$8, %edx
	movq	%r14, %rdi
	movq	%rbx, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB9_12:
	movq	%r15, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end9:
	.size	_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end9-_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc
	.section	.gcc_except_table._RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECsaU0XrY2Q1Mf_22flat_natural_cast_main,"a",@progbits
	.p2align	2, 0x0
GCC_except_table9:
.Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end1-.Lcst_begin1
.Lcst_begin1:
	.uleb128 .Lfunc_begin1-.Lfunc_begin1
	.uleb128 .Ltmp3-.Lfunc_begin1
	.byte	0
	.byte	0
	.uleb128 .Ltmp3-.Lfunc_begin1
	.uleb128 .Ltmp4-.Ltmp3
	.uleb128 .Ltmp5-.Lfunc_begin1
	.byte	0
	.uleb128 .Ltmp4-.Lfunc_begin1
	.uleb128 .Lfunc_end9-.Ltmp4
	.byte	0
	.byte	0
.Lcst_end1:
	.p2align	2, 0x0

	.section	.text._RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end10, nop
	.type	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	pushq	%rax
	.cfi_def_cfa_offset 64
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r14
	movq	%rsi, %rbx
	cmpq	$8, %rcx
	jb	.LBB10_2
	shrq	$3, %rcx
	movq	%rcx, %r15
	shlq	$5, %r15
	leaq	(%rdi,%r15), %rsi
	imulq	$56, %rcx, %r12
	leaq	(%rdi,%r12), %rdx
	movq	%rcx, %r13
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	movq	%rax, %rbp
	leaq	(%rbx,%r15), %rsi
	leaq	(%rbx,%r12), %rdx
	movq	%rbx, %rdi
	movq	%r13, %rcx
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	movq	%rax, %rbx
	addq	%r14, %r15
	addq	%r14, %r12
	movq	%r14, %rdi
	movq	%r15, %rsi
	movq	%r12, %rdx
	movq	%r13, %rcx
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	movq	%rbp, %rdi
	movq	%rax, %r14
.LBB10_2:
	movq	(%rdi), %rax
	movq	(%rbx), %rcx
	movq	%rax, %rdx
	sarq	$63, %rdx
	shrq	%rdx
	xorq	%rax, %rdx
	movq	%rcx, %rax
	sarq	$63, %rax
	shrq	%rax
	xorq	%rcx, %rax
	cmpq	%rax, %rdx
	setl	%cl
	movq	(%r14), %rsi
	movq	%rsi, %r8
	sarq	$63, %r8
	shrq	%r8
	xorq	%rsi, %r8
	cmpq	%r8, %rdx
	setl	%dl
	xorb	%cl, %dl
	cmpq	%r8, %rax
	setl	%al
	xorb	%cl, %al
	cmovneq	%r14, %rbx
	testb	%dl, %dl
	cmovneq	%rdi, %rbx
	movq	%rbx, %rax
	addq	$8, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end10:
	.size	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end10-_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end11, nop
	.type	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	pushq	%rax
	.cfi_def_cfa_offset 64
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r8
	vpxor	%xmm0, %xmm0, %xmm0
	vmovdqu	(%rdi), %xmm1
	vmovdqu	16(%rdi), %xmm2
	vpcmpgtq	%xmm1, %xmm0, %xmm3
	vpsrlq	$1, %xmm3, %xmm3
	vpxor	%xmm1, %xmm3, %xmm1
	vpshufd	$238, %xmm1, %xmm3
	vpcmpgtq	%xmm3, %xmm1, %xmm1
	vmovq	%xmm1, %rax
	vpcmpgtq	%xmm2, %xmm0, %xmm1
	vpsrlq	$1, %xmm1, %xmm1
	vpxor	%xmm2, %xmm1, %xmm1
	vpshufd	$238, %xmm1, %xmm2
	vpcmpgtq	%xmm2, %xmm1, %xmm1
	movl	%eax, %ecx
	andl	$1, %ecx
	notb	%al
	movzbl	%al, %eax
	andl	$1, %eax
	leaq	(%rdi,%rax,8), %r10
	vmovd	%xmm1, %ebx
	movl	$2, %r9d
	subl	%ebx, %r9d
	andl	$1, %ebx
	xorl	$3, %ebx
	movq	(%rdi,%r9,8), %r11
	movq	(%rdi,%rcx,8), %rdx
	movq	%rdx, (%rsp)
	movq	%r11, %r15
	sarq	$63, %r15
	shrq	%r15
	xorq	%r11, %r15
	movq	%rdx, %r12
	sarq	$63, %r12
	shrq	%r12
	xorq	%rdx, %r12
	movq	(%rdi,%rbx,8), %r14
	movq	(%rdi,%rax,8), %r13
	movq	%r14, %rbp
	sarq	$63, %rbp
	shrq	%rbp
	xorq	%r14, %rbp
	movq	%r13, %r14
	sarq	$63, %r14
	shrq	%r14
	xorq	%r13, %r14
	cmpq	%r12, %r15
	movl	%r9d, %r13d
	cmovll	%eax, %r13d
	cmpq	%r14, %rbp
	leaq	(%rdi,%rbx,8), %r14
	cmovgel	%eax, %r9d
	cmovll	%ebx, %r13d
	cmovlq	%r10, %r14
	movl	$2, %edx
	cmpq	%r12, %r15
	leaq	(%rdi,%r13,8), %r10
	movq	(%rdi,%r13,8), %rbx
	cmovll	%ecx, %r9d
	leaq	(%rdi,%r9,8), %rcx
	movq	(%rdi,%r9,8), %r9
	movq	(%rsp), %rax
	cmovlq	%r11, %rax
	movq	%rax, (%rsp)
	movq	%rbx, %r11
	sarq	$63, %r11
	shrq	%r11
	xorq	%rbx, %r11
	movq	%r9, %rbx
	sarq	$63, %rbx
	shrq	%rbx
	xorq	%r9, %rbx
	cmpq	%rbx, %r11
	movq	%rax, (%r8)
	movq	%rcx, %r9
	cmovlq	%r10, %r9
	movq	(%r9), %r9
	movq	%r9, 8(%r8)
	cmovlq	%rcx, %r10
	movq	(%r10), %rcx
	movq	%rcx, 16(%r8)
	movq	(%r14), %r9
	movq	%r9, 24(%r8)
	vmovdqu	32(%rdi), %xmm1
	vmovdqu	48(%rdi), %xmm2
	vpcmpgtq	%xmm1, %xmm0, %xmm3
	vpsrlq	$1, %xmm3, %xmm3
	vpxor	%xmm1, %xmm3, %xmm1
	vpshufd	$238, %xmm1, %xmm3
	vpcmpgtq	%xmm3, %xmm1, %xmm1
	vmovq	%xmm1, %r10
	vpcmpgtq	%xmm2, %xmm0, %xmm0
	vpsrlq	$1, %xmm0, %xmm0
	vpxor	%xmm2, %xmm0, %xmm0
	vpshufd	$238, %xmm0, %xmm1
	vpcmpgtq	%xmm1, %xmm0, %xmm0
	movl	%r10d, %ecx
	andl	$1, %ecx
	notb	%r10b
	movzbl	%r10b, %ebx
	andl	$1, %ebx
	vmovd	%xmm0, %r14d
	subl	%r14d, %edx
	andl	$1, %r14d
	xorl	$3, %r14d
	movq	32(%rdi,%rdx,8), %r11
	movq	32(%rdi,%rcx,8), %r10
	movq	%r11, %r15
	sarq	$63, %r15
	shrq	%r15
	xorq	%r11, %r15
	movq	32(%rdi,%r14,8), %r12
	movq	32(%rdi,%rbx,8), %r13
	movq	%r12, %rbp
	sarq	$63, %rbp
	shrq	%rbp
	xorq	%r12, %rbp
	movq	%r13, %r12
	sarq	$63, %r12
	shrq	%r12
	xorq	%r13, %r12
	movq	%r10, %r13
	sarq	$63, %r13
	shrq	%r13
	xorq	%r10, %r13
	cmpq	%r13, %r15
	movl	%edx, %eax
	cmovll	%ebx, %eax
	cmpq	%r12, %rbp
	cmovgel	%ebx, %edx
	leaq	32(%rdi,%rbx,8), %r12
	leaq	32(%rdi,%r14,8), %rbx
	cmovll	%r14d, %eax
	cmovlq	%r12, %rbx
	cmpq	%r13, %r15
	leaq	32(%rdi,%rax,8), %r14
	movq	32(%rdi,%rax,8), %r15
	cmovll	%ecx, %edx
	leaq	32(%rdi,%rdx,8), %rcx
	movq	32(%rdi,%rdx,8), %rdx
	cmovlq	%r11, %r10
	leaq	24(%r8), %rax
	movq	%r15, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%r15, %rdi
	movq	%rdx, %r11
	sarq	$63, %r11
	shrq	%r11
	xorq	%rdx, %r11
	cmpq	%r11, %rdi
	movq	%r10, 32(%r8)
	movq	%rcx, %rdx
	cmovlq	%r14, %rdx
	movq	(%rdx), %rdx
	movq	%rdx, 40(%r8)
	cmovlq	%rcx, %r14
	movq	(%r14), %rcx
	movq	%rcx, 48(%r8)
	leaq	56(%r8), %rcx
	movq	(%rbx), %rbx
	movq	%rbx, 56(%r8)
	movq	%r10, %rdx
	sarq	$63, %rdx
	shrq	%rdx
	xorq	%r10, %rdx
	movq	(%rsp), %rdi
	movq	%rdi, %r14
	sarq	$63, %r14
	shrq	%r14
	xorq	%rdi, %r14
	movq	%rdi, %r15
	xorl	%edi, %edi
	xorl	%r11d, %r11d
	cmpq	%r14, %rdx
	setge	%dil
	setl	%r11b
	cmovgeq	%r15, %r10
	movq	%r10, (%rsi)
	leaq	(%r8,%r11,8), %r14
	addq	$32, %r14
	leaq	(%r8,%rdi,8), %r10
	movq	%rbx, %rdx
	sarq	$63, %rdx
	shrq	%rdx
	xorq	%rbx, %rdx
	movq	%r9, %r15
	sarq	$63, %r15
	shrq	%r15
	xorq	%r9, %r15
	xorl	%r12d, %r12d
	xorl	%r13d, %r13d
	cmpq	%r15, %rdx
	setl	%r12b
	setge	%r13b
	cmovlq	%r9, %rbx
	movq	%rbx, 56(%rsi)
	shll	$3, %r13d
	subq	%r13, %rcx
	shll	$3, %r12d
	subq	%r12, %rax
	movq	32(%r8,%r11,8), %rdx
	movq	(%r8,%rdi,8), %rdi
	movq	%rdx, %r8
	sarq	$63, %r8
	shrq	%r8
	xorq	%rdx, %r8
	movq	%rdi, %rbx
	sarq	$63, %rbx
	shrq	%rbx
	xorq	%rdi, %rbx
	xorl	%r9d, %r9d
	xorl	%r11d, %r11d
	cmpq	%rbx, %r8
	setge	%r9b
	setl	%r11b
	cmovlq	%rdx, %rdi
	movq	%rdi, 8(%rsi)
	leaq	(%r14,%r11,8), %rdi
	leaq	(%r10,%r9,8), %rdx
	movq	(%rcx), %r8
	movq	(%rax), %rbx
	movq	%r8, %r15
	sarq	$63, %r15
	shrq	%r15
	xorq	%r8, %r15
	movq	%rbx, %r12
	sarq	$63, %r12
	shrq	%r12
	xorq	%rbx, %r12
	xorl	%r13d, %r13d
	xorl	%ebp, %ebp
	cmpq	%r12, %r15
	setl	%r13b
	setge	%bpl
	cmovlq	%rbx, %r8
	movq	%r8, 48(%rsi)
	shll	$3, %ebp
	subq	%rbp, %rcx
	shll	$3, %r13d
	subq	%r13, %rax
	movq	(%r14,%r11,8), %r11
	movq	(%r10,%r9,8), %r10
	movq	%r11, %rbx
	sarq	$63, %rbx
	shrq	%rbx
	xorq	%r11, %rbx
	movq	%r10, %r14
	sarq	$63, %r14
	shrq	%r14
	xorq	%r10, %r14
	xorl	%r8d, %r8d
	xorl	%r9d, %r9d
	cmpq	%r14, %rbx
	setge	%r8b
	setl	%r9b
	cmovlq	%r11, %r10
	movq	%r10, 16(%rsi)
	movq	(%rcx), %r10
	movq	(%rax), %r11
	movq	%r10, %rbx
	sarq	$63, %rbx
	shrq	%rbx
	xorq	%r10, %rbx
	movq	%r11, %r14
	sarq	$63, %r14
	shrq	%r14
	xorq	%r11, %r14
	xorl	%r15d, %r15d
	xorl	%r12d, %r12d
	cmpq	%r14, %rbx
	setl	%r15b
	setge	%r12b
	cmovlq	%r11, %r10
	movq	%r10, 40(%rsi)
	shll	$3, %r12d
	subq	%r12, %rcx
	shll	$3, %r15d
	subq	%r15, %rax
	movq	(%rdi,%r9,8), %rbx
	movq	(%rdx,%r8,8), %r14
	movq	%rbx, %r15
	sarq	$63, %r15
	shrq	%r15
	xorq	%rbx, %r15
	movq	%r14, %r12
	sarq	$63, %r12
	shrq	%r12
	xorq	%r14, %r12
	xorl	%r10d, %r10d
	xorl	%r11d, %r11d
	cmpq	%r12, %r15
	setge	%r10b
	setl	%r11b
	cmovlq	%rbx, %r14
	movq	%r14, 24(%rsi)
	movq	(%rcx), %rbx
	movq	(%rax), %r14
	movq	%rbx, %r15
	sarq	$63, %r15
	shrq	%r15
	xorq	%rbx, %r15
	movq	%r14, %r12
	sarq	$63, %r12
	shrq	%r12
	xorq	%r14, %r12
	xorl	%r13d, %r13d
	xorl	%ebp, %ebp
	cmpq	%r12, %r15
	cmovlq	%r14, %rbx
	movq	%rbx, 32(%rsi)
	leaq	(%rdi,%r9,8), %rsi
	leaq	(%rsi,%r11,8), %rsi
	setl	%r13b
	setge	%bpl
	shll	$3, %ebp
	subq	%rbp, %rcx
	shll	$3, %r13d
	subq	%r13, %rax
	addq	$8, %rax
	addq	$8, %rcx
	cmpq	%rcx, %rsi
	leaq	(%rdx,%r8,8), %rcx
	leaq	(%rcx,%r10,8), %rdx
	setne	%cl
	cmpq	%rax, %rdx
	jne	.LBB11_1
	testb	%cl, %cl
	jne	.LBB11_4
.LBB11_3:
	addq	$8, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.LBB11_1:
	.cfi_def_cfa_offset 64
	movb	$1, %cl
	testb	%cl, %cl
	je	.LBB11_3
.LBB11_4:
	callq	*_RNvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort22panic_on_ord_violation@GOTPCREL(%rip)
.Lfunc_end11:
	.size	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end11-_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort25insertion_sort_shift_leftdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end12, nop
	.type	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort25insertion_sort_shift_leftdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort25insertion_sort_shift_leftdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	leaq	(%rdi,%rsi,8), %rax
	leaq	8(%rdi), %rsi
	xorl	%ecx, %ecx
	movq	%rdi, %r9
	jmp	.LBB12_1
.LBB12_6:
	addq	%rdi, %r10
	movq	%r10, %r11
.LBB12_7:
	movq	%rsi, (%r11)
.LBB12_8:
	leaq	8(%rdx), %rsi
	addq	$8, %rcx
	movq	%rdx, %r9
	cmpq	%rax, %rsi
	je	.LBB12_9
.LBB12_1:
	movq	%rsi, %rdx
	movq	(%r9), %r10
	movq	8(%r9), %rsi
	movq	%rsi, %r8
	sarq	$63, %r8
	shrq	%r8
	xorq	%rsi, %r8
	movq	%r10, %r11
	sarq	$63, %r11
	shrq	%r11
	xorq	%r10, %r11
	cmpq	%r11, %r8
	jge	.LBB12_8
	movq	%r10, 8(%r9)
	movq	%rcx, %r10
	movq	%rdi, %r11
	cmpq	%rdi, %r9
	je	.LBB12_7
	.p2align	4
.LBB12_3:
	movq	-8(%rdi,%r10), %r9
	movq	%r9, %r11
	sarq	$63, %r11
	shrq	%r11
	xorq	%r9, %r11
	cmpq	%r11, %r8
	jge	.LBB12_6
	movq	%r9, (%rdi,%r10)
	addq	$-8, %r10
	jne	.LBB12_3
	movq	%rdi, %r11
	jmp	.LBB12_7
.LBB12_9:
	retq
.Lfunc_end12:
	.size	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort25insertion_sort_shift_leftdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end12-_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort25insertion_sort_shift_leftdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end13, nop
	.type	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$728, %rsp
	.cfi_def_cfa_offset 784
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movl	%r8d, 68(%rsp)
	movq	%rdx, %rbx
	movq	%rdi, 16(%rsp)
	movabsq	$4611686018427387903, %rax
	addq	%rsi, %rax
	movq	%rax, %rdx
	orq	%rsi, %rdx
	shrq	$32, %rdx
	je	.LBB13_1
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB13_3
.LBB13_1:
	xorl	%edx, %edx
	divl	%esi
.LBB13_3:
	movq	%rax, 72(%rsp)
	cmpq	$4097, %rsi
	movq	%rcx, 24(%rsp)
	movq	%rsi, 32(%rsp)
	jae	.LBB13_4
	movq	%rsi, %rax
	shrq	%rax
	movq	%rsi, %rdx
	subq	%rax, %rdx
	cmpq	$64, %rdx
	movl	$64, %eax
	cmovbq	%rdx, %rax
	jmp	.LBB13_6
.LBB13_4:
	movq	%rsi, %rdi
	callq	*_RNvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift11sqrt_approx@GOTPCREL(%rip)
	movq	24(%rsp), %rcx
.LBB13_6:
	movq	%rax, (%rsp)
	movq	16(%rsp), %rax
	leaq	32(%rax), %rdx
	movq	%rdx, 104(%rsp)
	leaq	-32(%rax), %rdx
	movq	%rdx, 96(%rsp)
	addq	$-8, %rax
	movq	%rax, 80(%rsp)
	movl	$1, %r14d
	xorl	%ebp, %ebp
	xorl	%r11d, %r11d
	movq	%rbx, 48(%rsp)
	.p2align	4
.LBB13_7:
	movq	32(%rsp), %r10
	movq	16(%rsp), %rax
	leaq	(%rax,%rbp,8), %rax
	movq	%rax, 8(%rsp)
	subq	%rbp, %r10
	jbe	.LBB13_8
	cmpq	(%rsp), %r10
	jae	.LBB13_27
.LBB13_25:
	cmpb	$0, 68(%rsp)
	je	.LBB13_26
	cmpq	$32, %r10
	movl	$32, %eax
	cmovaeq	%rax, %r10
	movq	8(%rsp), %rdi
	movq	%r10, %rsi
	movq	%rbx, %rdx
	xorl	%r8d, %r8d
	xorl	%r9d, %r9d
	movq	%r11, %r15
	movq	%r10, %r12
	vzeroupper
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	movq	%r15, %r11
	movq	24(%rsp), %rcx
	movl	%r12d, %eax
	jmp	.LBB13_47
	.p2align	4
.LBB13_8:
	xorl	%esi, %esi
	movl	$1, %r10d
	cmpq	$2, %r11
	jae	.LBB13_10
	jmp	.LBB13_57
	.p2align	4
.LBB13_27:
	cmpq	$2, %r10
	jae	.LBB13_29
	movq	%r10, %rax
.LBB13_47:
	leaq	1(,%rax,2), %r10
	jmp	.LBB13_48
.LBB13_26:
	movq	(%rsp), %rax
	cmpq	%r10, %rax
	cmovbq	%rax, %r10
	addq	%r10, %r10
.LBB13_48:
	movq	%r14, %rax
	shrq	%rax
	movq	%r10, %rsi
	shrq	%rsi
	leaq	(,%rbp,2), %rdx
	subq	%rax, %rdx
	leaq	(%rsi,%rbp,2), %rax
	movq	72(%rsp), %rsi
	imulq	%rsi, %rdx
	imulq	%rsi, %rax
	xorq	%rdx, %rax
	xorl	%esi, %esi
	lzcntq	%rax, %rsi
	cmpq	$2, %r11
	jb	.LBB13_57
.LBB13_10:
	movq	80(%rsp), %rax
	leaq	(%rax,%rbp,8), %rax
	movq	%rax, 112(%rsp)
	movq	%r10, 40(%rsp)
	movq	%rbp, 120(%rsp)
	movq	%rsi, 88(%rsp)
	jmp	.LBB13_11
.LBB13_62:
	movq	%r14, %rbp
	movq	%r11, %rsi
.LBB13_63:
	subq	%rsi, %rdx
	movq	%rbp, %rdi
	callq	*memcpy@GOTPCREL(%rip)
	movq	24(%rsp), %rcx
	movq	56(%rsp), %r11
	movq	40(%rsp), %r10
	movq	88(%rsp), %rsi
.LBB13_64:
	leaq	1(,%r15,2), %r14
	movq	48(%rsp), %rbx
	movq	120(%rsp), %rbp
	cmpq	$1, %r11
	jbe	.LBB13_56
.LBB13_11:
	cmpb	%sil, 133(%rsp,%r11)
	jb	.LBB13_57
	decq	%r11
	movq	200(%rsp,%r11,8), %rax
	movq	%rax, %r12
	shrq	%r12
	movq	%r14, %r13
	shrq	%r13
	leaq	(%r12,%r13), %r15
	cmpq	%rcx, %r15
	ja	.LBB13_14
	movl	%eax, %edx
	orl	%r14d, %edx
	testb	$1, %dl
	jne	.LBB13_14
	addq	%r15, %r15
	movq	%r15, %r14
	cmpq	$1, %r11
	ja	.LBB13_11
	jmp	.LBB13_56
	.p2align	4
.LBB13_14:
	movq	%rbp, %rdx
	subq	%r15, %rdx
	movq	16(%rsp), %rsi
	leaq	(%rsi,%rdx,8), %rbp
	testb	$1, %al
	movq	%r11, 56(%rsp)
	je	.LBB13_59
	testb	$1, %r14b
	je	.LBB13_16
.LBB13_17:
	testq	%r13, %r13
	movq	88(%rsp), %rsi
	jne	.LBB13_18
	jmp	.LBB13_64
	.p2align	4
.LBB13_59:
	movq	%r12, %rax
	orq	$1, %rax
	xorl	%r8d, %r8d
	lzcntq	%rax, %r8
	addl	%r8d, %r8d
	xorl	$126, %r8d
	movq	%rbp, %rdi
	movq	%r12, %rsi
	movq	%rbx, %rdx
	xorl	%r9d, %r9d
	vzeroupper
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	movq	40(%rsp), %r10
	movq	56(%rsp), %r11
	movq	24(%rsp), %rcx
	testb	$1, %r14b
	jne	.LBB13_17
.LBB13_16:
	leaq	(,%r12,8), %rdi
	addq	%rbp, %rdi
	movq	%r13, %rax
	orq	$1, %rax
	xorl	%r8d, %r8d
	lzcntq	%rax, %r8
	addl	%r8d, %r8d
	xorl	$126, %r8d
	movq	%r13, %rsi
	movq	%rbx, %rdx
	xorl	%r9d, %r9d
	vzeroupper
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	movq	40(%rsp), %r10
	movq	56(%rsp), %r11
	movq	24(%rsp), %rcx
	testq	%r13, %r13
	movq	88(%rsp), %rsi
	je	.LBB13_64
.LBB13_18:
	testq	%r12, %r12
	je	.LBB13_64
	cmpq	%r13, %r12
	movq	%r13, %rbx
	cmovbq	%r12, %rbx
	cmpq	%rbx, %rcx
	jb	.LBB13_64
	leaq	(,%r12,8), %r14
	addq	%rbp, %r14
	cmpq	%r13, %r12
	movq	%rbp, %rsi
	cmovaq	%r14, %rsi
	leaq	(,%rbx,8), %rdx
	movq	48(%rsp), %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	movq	48(%rsp), %r11
	leaq	(%r11,%rbx,8), %rdx
	cmpq	%r13, %r12
	jbe	.LBB13_21
	movq	112(%rsp), %rax
	.p2align	4
.LBB13_61:
	movq	-8(%rdx), %rcx
	movq	-8(%r14), %rsi
	movq	%rcx, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%rcx, %rdi
	movq	%rsi, %r8
	sarq	$63, %r8
	shrq	%r8
	xorq	%rsi, %r8
	xorl	%r9d, %r9d
	xorl	%r10d, %r10d
	cmpq	%r8, %rdi
	setl	%r9b
	setge	%r10b
	cmovlq	%rsi, %rcx
	movq	%rcx, (%rax)
	leaq	(%r14,%r10,8), %r14
	addq	$-8, %r14
	leaq	(%rdx,%r9,8), %rdx
	addq	$-8, %rdx
	cmpq	%rbp, %r14
	je	.LBB13_62
	addq	$-8, %rax
	cmpq	%r11, %rdx
	jne	.LBB13_61
	jmp	.LBB13_62
.LBB13_21:
	movq	%r11, %rsi
	movq	8(%rsp), %r11
	.p2align	4
.LBB13_22:
	movq	(%r14), %rdi
	movq	(%rsi), %r8
	movq	%rdi, %rcx
	sarq	$63, %rcx
	shrq	%rcx
	xorq	%rdi, %rcx
	movq	%r8, %r9
	sarq	$63, %r9
	shrq	%r9
	xorq	%r8, %r9
	xorl	%eax, %eax
	xorl	%r10d, %r10d
	cmpq	%r9, %rcx
	setl	%cl
	setge	%r10b
	cmovlq	%rdi, %r8
	movq	%r8, (%rbp)
	leaq	(%rsi,%r10,8), %rsi
	addq	$8, %rbp
	cmpq	%rdx, %rsi
	je	.LBB13_63
	movb	%cl, %al
	leaq	(%r14,%rax,8), %r14
	cmpq	%r11, %r14
	jne	.LBB13_22
	jmp	.LBB13_63
	.p2align	4
.LBB13_56:
	movl	$1, %r11d
.LBB13_57:
	movq	%r14, 200(%rsp,%r11,8)
	movb	%sil, 134(%rsp,%r11)
	cmpq	%rbp, 32(%rsp)
	jbe	.LBB13_66
	incq	%r11
	movq	%r10, %rax
	shrq	%rax
	addq	%rax, %rbp
	movq	%r10, %r14
	jmp	.LBB13_7
.LBB13_29:
	movq	8(%rsp), %rdx
	movq	(%rdx), %rax
	movq	8(%rdx), %rsi
	movq	%rsi, %r9
	sarq	$63, %r9
	shrq	%r9
	xorq	%rsi, %r9
	movq	%rax, %rdx
	sarq	$63, %rdx
	shrq	%rdx
	xorq	%rax, %rdx
	cmpq	%rdx, %r9
	jge	.LBB13_30
	cmpq	$2, %r10
	jne	.LBB13_34
	cmpq	$2, (%rsp)
	ja	.LBB13_25
	movl	$2, %eax
	movl	$1, %r10d
	jmp	.LBB13_41
.LBB13_30:
	movl	$2, %eax
	cmpq	$2, %r10
	jne	.LBB13_31
	cmpq	$2, (%rsp)
	ja	.LBB13_25
	jmp	.LBB13_47
	.p2align	4
.LBB13_31:
	movq	%rsi, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%rsi, %rdi
	movq	8(%rsp), %rsi
	movq	(%rsi,%rax,8), %rsi
	movq	%rsi, %r8
	sarq	$63, %r8
	shrq	%r8
	xorq	%rsi, %r8
	cmpq	%rdi, %r8
	jl	.LBB13_38
	incq	%rax
	cmpq	%rax, %r10
	jne	.LBB13_31
	jmp	.LBB13_37
.LBB13_34:
	movl	$2, %eax
	.p2align	4
.LBB13_35:
	movq	%rsi, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%rsi, %rdi
	movq	8(%rsp), %rsi
	movq	(%rsi,%rax,8), %rsi
	movq	%rsi, %r8
	sarq	$63, %r8
	shrq	%r8
	xorq	%rsi, %r8
	cmpq	%rdi, %r8
	jge	.LBB13_38
	incq	%rax
	cmpq	%rax, %r10
	jne	.LBB13_35
.LBB13_37:
	movq	%r10, %rax
.LBB13_38:
	cmpq	(%rsp), %rax
	jb	.LBB13_25
	cmpq	%rdx, %r9
	jge	.LBB13_47
	movq	%rax, %r10
	shrq	%r10
	je	.LBB13_47
.LBB13_41:
	cmpq	$8, %r10
	jae	.LBB13_49
	xorl	%edx, %edx
	jmp	.LBB13_52
.LBB13_49:
	movq	%r10, %rdx
	andq	$-8, %rdx
	movq	%rdx, %rsi
	negq	%rsi
	movq	104(%rsp), %rdi
	leaq	(%rdi,%rbp,8), %rdi
	leaq	(,%rax,8), %r8
	leaq	(%r8,%rbp,8), %r8
	addq	96(%rsp), %r8
	xorl	%r9d, %r9d
	.p2align	4
.LBB13_50:
	vpermpd	$27, (%r8,%r9,8), %ymm0
	vpermpd	$27, -32(%r8,%r9,8), %ymm1
	vpermpd	$27, -32(%rdi), %ymm2
	vmovups	%ymm0, -32(%rdi)
	vpermpd	$27, (%rdi), %ymm0
	vmovups	%ymm1, (%rdi)
	vmovups	%ymm2, (%r8,%r9,8)
	vmovups	%ymm0, -32(%r8,%r9,8)
	addq	$-8, %r9
	addq	$64, %rdi
	cmpq	%r9, %rsi
	jne	.LBB13_50
	cmpq	%rdx, %r10
	je	.LBB13_47
.LBB13_52:
	negq	%r10
	movq	%rdx, %rsi
	negq	%rsi
	leaq	(,%rbp,8), %rdi
	shlq	$3, %rdx
	leaq	(%rdx,%rbp,8), %rdx
	addq	16(%rsp), %rdx
	leaq	(%rdi,%rax,8), %rdi
	addq	80(%rsp), %rdi
	.p2align	4
.LBB13_53:
	vmovsd	(%rdx), %xmm0
	movq	(%rdi,%rsi,8), %r8
	movq	%r8, (%rdx)
	vmovsd	%xmm0, (%rdi,%rsi,8)
	decq	%rsi
	addq	$8, %rdx
	cmpq	%rsi, %r10
	jne	.LBB13_53
	jmp	.LBB13_47
.LBB13_66:
	testb	$1, %r14b
	jne	.LBB13_67
	movq	32(%rsp), %rsi
	movq	%rsi, %rax
	orq	$1, %rax
	xorl	%r8d, %r8d
	lzcntq	%rax, %r8
	addl	%r8d, %r8d
	xorl	$126, %r8d
	movq	16(%rsp), %rdi
	movq	%rbx, %rdx
	xorl	%r9d, %r9d
	addq	$728, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	vzeroupper
	jmp	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
.LBB13_67:
	.cfi_def_cfa_offset 784
	addq	$728, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.Lfunc_end13:
	.size	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end13-_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end14, nop
	.type	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main:
.Lfunc_begin2:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception2
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$72, %rsp
	.cfi_def_cfa_offset 128
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%r9, 24(%rsp)
	movq	%rcx, 8(%rsp)
	movq	%rdx, %rbx
	movq	%rsi, %r14
	movq	%rdi, %r12
	cmpq	$33, %rsi
	jae	.LBB14_7
.LBB14_1:
	movq	%r12, %rbp
.LBB14_2:
	cmpq	$2, %r14
	jb	.LBB14_47
	leaq	16(%r14), %rax
	cmpq	%rax, 8(%rsp)
	jb	.LBB14_37
	movq	%r14, %r12
	shrq	%r12
	cmpq	$15, %r14
	movq	%rbp, (%rsp)
	jbe	.LBB14_5
	leaq	(%rbx,%r14,8), %rdx
	movq	%rbp, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	leaq	(,%r12,8), %rdi
	addq	%rbp, %rdi
	leaq	(%rbx,%r12,8), %rsi
	leaq	(%rbx,%r14,8), %rdx
	addq	$64, %rdx
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	movl	$8, %ecx
	movq	%r14, %rdx
	subq	%r12, %rdx
	cmpq	%r12, %rcx
	jb	.LBB14_13
	jmp	.LBB14_18
.LBB14_7:
	movl	%r8d, (%rsp)
.LBB14_8:
	movl	(%rsp), %r15d
	testl	%r15d, %r15d
	je	.LBB14_46
	leaq	96(%r12), %rbp
	movq	%r14, %r13
	.p2align	4
.LBB14_44:
	movq	%r13, %rcx
	shrq	$3, %rcx
	movq	%rcx, %rax
	shlq	$5, %rax
	addq	%r12, %rax
	imulq	$56, %rcx, %rdx
	addq	%r12, %rdx
	cmpq	$64, %r13
	jae	.LBB14_45
	movq	(%r12), %rcx
	movq	(%rax), %rsi
	movq	%rcx, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%rcx, %rdi
	movq	%rsi, %rcx
	sarq	$63, %rcx
	shrq	%rcx
	xorq	%rsi, %rcx
	cmpq	%rcx, %rdi
	setl	%sil
	movq	(%rdx), %r8
	movq	%r8, %r9
	sarq	$63, %r9
	shrq	%r9
	xorq	%r8, %r9
	cmpq	%r9, %rdi
	setl	%dil
	xorb	%sil, %dil
	cmpq	%r9, %rcx
	setl	%cl
	xorb	%sil, %cl
	cmovneq	%rdx, %rax
	testb	%dil, %dil
	cmovneq	%r12, %rax
	jmp	.LBB14_49
	.p2align	4
.LBB14_45:
	movq	%r12, %rdi
	movq	%rax, %rsi
	vzeroupper
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
.LBB14_49:
	decl	%r15d
	movl	%r15d, (%rsp)
	movq	%rax, %r15
	subq	%r12, %r15
	movq	%r15, %rcx
	shrq	$3, %rcx
	movq	%rcx, 16(%rsp)
	vmovq	(%rax), %xmm0
	vmovq	%xmm0, 64(%rsp)
	cmpq	$0, 24(%rsp)
	je	.LBB14_51
	vmovq	%xmm0, %rax
	movq	24(%rsp), %rcx
	movq	(%rcx), %rcx
	movq	%rcx, %rdx
	sarq	$63, %rdx
	shrq	%rdx
	xorq	%rcx, %rdx
	movq	%rax, %rcx
	sarq	$63, %rcx
	shrq	%rcx
	xorq	%rax, %rcx
	cmpq	%rcx, %rdx
	jge	.LBB14_78
.LBB14_51:
	cmpq	%r13, 8(%rsp)
	jb	.LBB14_37
	leaq	(%rbx,%r13,8), %rax
	xorl	%r14d, %r14d
	movq	%r12, %rcx
	movq	16(%rsp), %rdx
	.p2align	4
.LBB14_53:
	movq	%rdx, %rsi
	subq	$3, %rsi
	movl	$0, %edi
	cmovaeq	%rsi, %rdi
	leaq	(%r12,%rdi,8), %rsi
	cmpq	%rsi, %rcx
	jae	.LBB14_56
	movq	(%r12,%r15), %r8
	movq	%r8, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%r8, %rdi
	.p2align	4
.LBB14_55:
	movq	(%rcx), %r8
	movq	%r8, %r9
	sarq	$63, %r9
	shrq	%r9
	xorq	%r8, %r9
	leaq	-8(%rax), %r10
	xorl	%r11d, %r11d
	cmpq	%rdi, %r9
	setl	%r11b
	cmovlq	%rbx, %r10
	movq	%r8, (%r10,%r14,8)
	addq	%r14, %r11
	movq	8(%rcx), %r8
	movq	%r8, %r9
	sarq	$63, %r9
	shrq	%r9
	xorq	%r8, %r9
	leaq	-16(%rax), %r10
	xorl	%r14d, %r14d
	cmpq	%rdi, %r9
	setl	%r14b
	cmovlq	%rbx, %r10
	movq	%r8, (%r10,%r11,8)
	addq	%r11, %r14
	movq	16(%rcx), %r8
	movq	%r8, %r9
	sarq	$63, %r9
	shrq	%r9
	xorq	%r8, %r9
	leaq	-24(%rax), %r10
	xorl	%r11d, %r11d
	cmpq	%rdi, %r9
	setl	%r11b
	cmovlq	%rbx, %r10
	movq	%r8, (%r10,%r14,8)
	addq	%r14, %r11
	movq	24(%rcx), %r8
	movq	%r8, %r9
	sarq	$63, %r9
	shrq	%r9
	xorq	%r8, %r9
	addq	$-32, %rax
	xorl	%r14d, %r14d
	cmpq	%rdi, %r9
	setl	%r14b
	movq	%rax, %r9
	cmovlq	%rbx, %r9
	movq	%r8, (%r9,%r11,8)
	addq	%r11, %r14
	addq	$32, %rcx
	cmpq	%rsi, %rcx
	jb	.LBB14_55
.LBB14_56:
	leaq	(%r12,%rdx,8), %rsi
	cmpq	%rsi, %rcx
	jae	.LBB14_59
	movq	(%r12,%r15), %r8
	movq	%r8, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%r8, %rdi
	.p2align	4
.LBB14_58:
	movq	%r14, %r8
	movq	(%rcx), %r9
	movq	%r9, %r10
	sarq	$63, %r10
	shrq	%r10
	xorq	%r9, %r10
	addq	$-8, %rax
	xorl	%r14d, %r14d
	cmpq	%rdi, %r10
	setl	%r14b
	movq	%rax, %r10
	cmovlq	%rbx, %r10
	movq	%r9, (%r10,%r8,8)
	addq	%r8, %r14
	addq	$8, %rcx
	cmpq	%rsi, %rcx
	jb	.LBB14_58
.LBB14_59:
	cmpq	%r13, %rdx
	je	.LBB14_61
	movq	(%rcx), %rdx
	movq	%rdx, -8(%rax,%r14,8)
	addq	$-8, %rax
	addq	$8, %rcx
	movq	%r13, %rdx
	jmp	.LBB14_53
	.p2align	4
.LBB14_61:
	leaq	(,%r14,8), %rdx
	movq	%r12, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	movq	%r13, %rsi
	subq	%r14, %rsi
	je	.LBB14_75
	cmpq	$4, %rsi
	jae	.LBB14_64
	xorl	%eax, %eax
	jmp	.LBB14_73
	.p2align	4
.LBB14_64:
	cmpq	$16, %rsi
	jae	.LBB14_66
	xorl	%eax, %eax
	jmp	.LBB14_70
.LBB14_66:
	movq	%rsi, %rax
	andq	$-16, %rax
	movq	%rax, %rcx
	negq	%rcx
	leaq	(,%r14,8), %rdx
	addq	%rbp, %rdx
	leaq	-32(%rbx), %rdi
	leaq	(%rdi,%r13,8), %rdi
	xorl	%r8d, %r8d
	.p2align	4
.LBB14_67:
	vpermq	$27, (%rdi,%r8,8), %ymm0
	vpermq	$27, -32(%rdi,%r8,8), %ymm1
	vpermq	$27, -64(%rdi,%r8,8), %ymm2
	vpermq	$27, -96(%rdi,%r8,8), %ymm3
	vmovdqu	%ymm0, -96(%rdx)
	vmovdqu	%ymm1, -64(%rdx)
	vmovdqu	%ymm2, -32(%rdx)
	vmovdqu	%ymm3, (%rdx)
	addq	$-16, %r8
	subq	$-128, %rdx
	cmpq	%r8, %rcx
	jne	.LBB14_67
	cmpq	%rax, %rsi
	je	.LBB14_75
	testb	$12, %sil
	je	.LBB14_73
.LBB14_70:
	movq	%rax, %rcx
	leaq	(%r12,%r14,8), %rdx
	movq	%rsi, %rax
	andq	$-4, %rax
	leaq	(,%rcx,8), %r8
	leaq	-32(%rbx), %rdi
	leaq	(%rdi,%r13,8), %rdi
	subq	%r8, %rdi
	.p2align	4
.LBB14_71:
	vpermq	$27, (%rdi), %ymm0
	vmovdqu	%ymm0, (%rdx,%rcx,8)
	addq	$4, %rcx
	addq	$-32, %rdi
	cmpq	%rcx, %rax
	jne	.LBB14_71
	cmpq	%rax, %rsi
	je	.LBB14_75
.LBB14_73:
	leaq	(%rax,%r14), %rcx
	shlq	$3, %rax
	leaq	-8(%rbx), %rdx
	leaq	(%rdx,%r13,8), %rdx
	subq	%rax, %rdx
	.p2align	4
.LBB14_74:
	movq	(%rdx), %rax
	movq	%rax, (%r12,%rcx,8)
	incq	%rcx
	addq	$-8, %rdx
	cmpq	%rcx, %r13
	jne	.LBB14_74
.LBB14_75:
	testq	%r14, %r14
	je	.LBB14_78
	cmpq	%r14, %r13
	jb	.LBB14_105
	leaq	(%r12,%r14,8), %rdi
	movq	%rbx, %rdx
	movq	8(%rsp), %rcx
	movl	(%rsp), %r15d
	movl	%r15d, %r8d
	leaq	64(%rsp), %r9
	vzeroupper
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	cmpq	$33, %r14
	jb	.LBB14_1
	movq	%r14, %r13
	testl	%r15d, %r15d
	jne	.LBB14_44
	jmp	.LBB14_46
	.p2align	4
.LBB14_78:
	cmpq	%r13, 8(%rsp)
	jb	.LBB14_37
	leaq	(%rbx,%r13,8), %rax
	xorl	%ebp, %ebp
	movq	%r12, %rcx
	movq	16(%rsp), %r14
	.p2align	4
.LBB14_80:
	movq	%r14, %rdx
	subq	$3, %rdx
	movl	$0, %esi
	cmovaeq	%rdx, %rsi
	leaq	(%r12,%rsi,8), %rdx
	cmpq	%rdx, %rcx
	jae	.LBB14_83
	movq	(%r12,%r15), %rdi
	movq	%rdi, %rsi
	sarq	$63, %rsi
	shrq	%rsi
	xorq	%rdi, %rsi
	.p2align	4
.LBB14_82:
	movq	(%rcx), %rdi
	movq	%rdi, %r8
	sarq	$63, %r8
	shrq	%r8
	xorq	%rdi, %r8
	leaq	-8(%rax), %r9
	xorl	%r10d, %r10d
	cmpq	%r8, %rsi
	setge	%r10b
	cmovgeq	%rbx, %r9
	movq	%rdi, (%r9,%rbp,8)
	addq	%rbp, %r10
	movq	8(%rcx), %rdi
	movq	%rdi, %r8
	sarq	$63, %r8
	shrq	%r8
	xorq	%rdi, %r8
	leaq	-16(%rax), %r9
	xorl	%r11d, %r11d
	cmpq	%r8, %rsi
	setge	%r11b
	cmovgeq	%rbx, %r9
	movq	%rdi, (%r9,%r10,8)
	addq	%r10, %r11
	movq	16(%rcx), %rdi
	movq	%rdi, %r8
	sarq	$63, %r8
	shrq	%r8
	xorq	%rdi, %r8
	leaq	-24(%rax), %r9
	xorl	%r10d, %r10d
	cmpq	%r8, %rsi
	setge	%r10b
	cmovgeq	%rbx, %r9
	movq	%rdi, (%r9,%r11,8)
	addq	%r11, %r10
	movq	24(%rcx), %rdi
	movq	%rdi, %r8
	sarq	$63, %r8
	shrq	%r8
	xorq	%rdi, %r8
	addq	$-32, %rax
	xorl	%ebp, %ebp
	cmpq	%r8, %rsi
	setge	%bpl
	movq	%rax, %r8
	cmovgeq	%rbx, %r8
	movq	%rdi, (%r8,%r10,8)
	addq	%r10, %rbp
	addq	$32, %rcx
	cmpq	%rdx, %rcx
	jb	.LBB14_82
.LBB14_83:
	leaq	(%r12,%r14,8), %rdx
	cmpq	%rdx, %rcx
	jae	.LBB14_86
	movq	(%r12,%r15), %rdi
	movq	%rdi, %rsi
	sarq	$63, %rsi
	shrq	%rsi
	xorq	%rdi, %rsi
	.p2align	4
.LBB14_85:
	movq	%rbp, %rdi
	movq	(%rcx), %r8
	movq	%r8, %r9
	sarq	$63, %r9
	shrq	%r9
	xorq	%r8, %r9
	addq	$-8, %rax
	xorl	%ebp, %ebp
	cmpq	%r9, %rsi
	setge	%bpl
	movq	%rax, %r9
	cmovgeq	%rbx, %r9
	movq	%r8, (%r9,%rdi,8)
	addq	%rdi, %rbp
	addq	$8, %rcx
	cmpq	%rdx, %rcx
	jb	.LBB14_85
.LBB14_86:
	cmpq	%r13, %r14
	je	.LBB14_88
	addq	$-8, %rax
	movq	(%rcx), %rdx
	movq	%rdx, (%rbx,%rbp,8)
	incq	%rbp
	addq	$8, %rcx
	movq	%r13, %r14
	jmp	.LBB14_80
	.p2align	4
.LBB14_88:
	leaq	(,%rbp,8), %rdx
	movq	%r12, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	movq	%r13, %r14
	subq	%rbp, %r14
	je	.LBB14_47
	leaq	(%r12,%rbp,8), %r15
	cmpq	$4, %r14
	jae	.LBB14_91
	xorl	%eax, %eax
	jmp	.LBB14_100
.LBB14_91:
	cmpq	$16, %r14
	jae	.LBB14_93
	xorl	%eax, %eax
	jmp	.LBB14_97
.LBB14_93:
	movq	%r14, %rax
	andq	$-16, %rax
	movq	%rax, %rcx
	negq	%rcx
	leaq	(%r12,%rbp,8), %rdx
	addq	$96, %rdx
	leaq	-32(%rbx), %rsi
	leaq	(%rsi,%r13,8), %rsi
	xorl	%edi, %edi
	.p2align	4
.LBB14_94:
	vpermq	$27, (%rsi,%rdi,8), %ymm0
	vpermq	$27, -32(%rsi,%rdi,8), %ymm1
	vpermq	$27, -64(%rsi,%rdi,8), %ymm2
	vpermq	$27, -96(%rsi,%rdi,8), %ymm3
	vmovdqu	%ymm0, -96(%rdx)
	vmovdqu	%ymm1, -64(%rdx)
	vmovdqu	%ymm2, -32(%rdx)
	vmovdqu	%ymm3, (%rdx)
	addq	$-16, %rdi
	subq	$-128, %rdx
	cmpq	%rdi, %rcx
	jne	.LBB14_94
	cmpq	%rax, %r14
	je	.LBB14_102
	testb	$12, %r14b
	je	.LBB14_100
.LBB14_97:
	movq	%rax, %rcx
	movq	%r14, %rax
	andq	$-4, %rax
	leaq	(,%rcx,8), %rdx
	leaq	-32(%rbx), %rsi
	subq	%rdx, %rsi
	leaq	(%rsi,%r13,8), %rdx
	movq	%r15, %rsi
	.p2align	4
.LBB14_98:
	vpermq	$27, (%rdx), %ymm0
	vmovdqu	%ymm0, (%rsi,%rcx,8)
	addq	$4, %rcx
	addq	$-32, %rdx
	cmpq	%rcx, %rax
	jne	.LBB14_98
	cmpq	%rax, %r14
	je	.LBB14_102
.LBB14_100:
	leaq	(%rax,%rbp), %rcx
	shlq	$3, %rax
	leaq	-8(%rbx), %rdx
	subq	%rax, %rdx
	leaq	(%rdx,%r13,8), %rax
	.p2align	4
.LBB14_101:
	movq	(%rax), %rdx
	movq	%rdx, (%r12,%rcx,8)
	incq	%rcx
	addq	$-8, %rax
	cmpq	%rcx, %r13
	jne	.LBB14_101
.LBB14_102:
	cmpq	%rbp, %r13
	jb	.LBB14_104
	movq	%r15, %rbp
	movq	%r15, %r12
	movl	$0, %eax
	movq	%rax, 24(%rsp)
	cmpq	$33, %r14
	jae	.LBB14_8
	jmp	.LBB14_2
.LBB14_46:
	movq	%r12, %rdi
	movq	%r14, %rsi
	movq	%rbx, %rdx
	movq	8(%rsp), %rcx
	movl	$1, %r8d
	vzeroupper
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	jmp	.LBB14_47
.LBB14_5:
	cmpq	$7, %r14
	jbe	.LBB14_11
	vpxor	%xmm0, %xmm0, %xmm0
	vmovdqu	(%rbp), %xmm1
	vmovdqu	16(%rbp), %xmm2
	vpcmpgtq	%xmm1, %xmm0, %xmm3
	vpsrlq	$1, %xmm3, %xmm3
	vpxor	%xmm1, %xmm3, %xmm1
	vpshufd	$238, %xmm1, %xmm3
	vpcmpgtq	%xmm3, %xmm1, %xmm1
	vmovq	%xmm1, %rax
	vpcmpgtq	%xmm2, %xmm0, %xmm1
	vpsrlq	$1, %xmm1, %xmm1
	vpxor	%xmm2, %xmm1, %xmm1
	vpshufd	$238, %xmm1, %xmm2
	vpcmpgtq	%xmm2, %xmm1, %xmm1
	movl	%eax, %esi
	andl	$1, %esi
	notb	%al
	movzbl	%al, %edx
	andl	$1, %edx
	vmovd	%xmm1, %r9d
	movl	$2, %edi
	subl	%r9d, %edi
	andl	$1, %r9d
	xorl	$3, %r9d
	movq	(%rbp,%rdi,8), %r8
	movq	(%rbp,%rsi,8), %r10
	movq	%r10, 16(%rsp)
	movq	%r8, %rax
	sarq	$63, %rax
	shrq	%rax
	xorq	%r8, %rax
	movq	%r10, %rcx
	sarq	$63, %rcx
	shrq	%rcx
	xorq	%r10, %rcx
	movq	(%rbp,%r9,8), %r15
	movq	(%rbp,%rdx,8), %r10
	movq	%r15, %r11
	sarq	$63, %r11
	shrq	%r11
	xorq	%r15, %r11
	movq	%r10, %r13
	sarq	$63, %r13
	shrq	%r13
	xorq	%r10, %r13
	cmpq	%rcx, %rax
	movl	%edi, %ebp
	cmovll	%edx, %ebp
	cmpq	%r13, %r11
	cmovgel	%edx, %edi
	cmovll	%r9d, %ebp
	cmovlq	%r10, %r15
	movq	%r15, 24(%rsp)
	cmpq	%rcx, %rax
	movq	(%rsp), %rax
	movq	(%rax,%rbp,8), %rdx
	cmovll	%esi, %edi
	movq	(%rsp), %rax
	movq	(%rax,%rdi,8), %rax
	movq	16(%rsp), %rcx
	cmovlq	%r8, %rcx
	movq	%rcx, 16(%rsp)
	movq	%rdx, %rdi
	movq	%rdx, %rcx
	sarq	$63, %rcx
	shrq	%rcx
	xorq	%rdx, %rcx
	movq	%rax, %rdx
	sarq	$63, %rdx
	shrq	%rdx
	xorq	%rax, %rdx
	cmpq	%rdx, %rcx
	movq	%rax, %rcx
	cmovlq	%rdi, %rcx
	movq	%rcx, 56(%rsp)
	movl	$2, %esi
	cmovlq	%rax, %rdi
	movq	%rdi, 48(%rsp)
	movq	(%rsp), %rax
	leaq	(%rax,%r12,8), %r8
	movq	(%rsp), %rax
	vmovdqu	(%rax,%r12,8), %xmm1
	movq	(%rsp), %rax
	vmovdqu	16(%rax,%r12,8), %xmm2
	vpcmpgtq	%xmm1, %xmm0, %xmm3
	vpsrlq	$1, %xmm3, %xmm3
	vpxor	%xmm1, %xmm3, %xmm1
	vpshufd	$238, %xmm1, %xmm3
	vpcmpgtq	%xmm3, %xmm1, %xmm1
	vmovq	%xmm1, %rax
	vpcmpgtq	%xmm2, %xmm0, %xmm0
	vpsrlq	$1, %xmm0, %xmm0
	vpxor	%xmm2, %xmm0, %xmm0
	vpshufd	$238, %xmm0, %xmm1
	vpcmpgtq	%xmm1, %xmm0, %xmm0
	movl	%eax, %ecx
	andl	$1, %ecx
	movq	%rcx, 40(%rsp)
	notb	%al
	movzbl	%al, %ebp
	andl	$1, %ebp
	vmovd	%xmm0, %eax
	subl	%eax, %esi
	andl	$1, %eax
	xorl	$3, %eax
	movq	(%r8,%rsi,8), %rdi
	movq	(%r8,%rcx,8), %r15
	movq	%rdi, 32(%rsp)
	movq	%rdi, %r13
	sarq	$63, %r13
	shrq	%r13
	xorq	%rdi, %r13
	movq	%r15, %r9
	sarq	$63, %r9
	shrq	%r9
	xorq	%r15, %r9
	movq	(%r8,%rax,8), %r10
	movq	%r10, %r11
	sarq	$63, %r11
	shrq	%r11
	xorq	%r10, %r11
	movq	(%r8,%rbp,8), %rdx
	movq	%rdx, %rcx
	sarq	$63, %rcx
	shrq	%rcx
	xorq	%rdx, %rcx
	cmpq	%r9, %r13
	movl	%esi, %edi
	cmovll	%ebp, %edi
	cmpq	%rcx, %r11
	cmovgel	%ebp, %esi
	movq	(%rsp), %rbp
	cmovll	%eax, %edi
	cmovlq	%rdx, %r10
	cmpq	%r9, %r13
	cmovll	40(%rsp), %esi
	movq	(%r8,%rdi,8), %rax
	movq	(%r8,%rsi,8), %rcx
	cmovlq	32(%rsp), %r15
	movq	%rax, %rdx
	sarq	$63, %rdx
	shrq	%rdx
	xorq	%rax, %rdx
	movq	%rcx, %rsi
	sarq	$63, %rsi
	shrq	%rsi
	xorq	%rcx, %rsi
	cmpq	%rsi, %rdx
	movq	16(%rsp), %rdx
	movq	%rdx, (%rbx)
	movq	56(%rsp), %rdx
	movq	%rdx, 8(%rbx)
	movq	48(%rsp), %rdx
	movq	%rdx, 16(%rbx)
	movq	24(%rsp), %rdx
	movq	%rdx, 24(%rbx)
	movq	%r15, (%rbx,%r12,8)
	movq	%rcx, %rdx
	cmovlq	%rax, %rdx
	cmovlq	%rcx, %rax
	movq	%rdx, 8(%rbx,%r12,8)
	movq	%rax, 16(%rbx,%r12,8)
	movq	%r10, 24(%rbx,%r12,8)
	movl	$4, %ecx
	movq	%r14, %rdx
	subq	%r12, %rdx
	cmpq	%r12, %rcx
	jb	.LBB14_13
	jmp	.LBB14_18
.LBB14_11:
	movq	(%rbp), %rax
	movq	%rax, (%rbx)
	movq	(%rbp,%r12,8), %rax
	movq	%rax, (%rbx,%r12,8)
	movl	$1, %ecx
	movq	%r14, %rdx
	subq	%r12, %rdx
	cmpq	%r12, %rcx
	jae	.LBB14_18
.LBB14_13:
	leaq	-8(,%rcx,8), %rax
	movq	%rcx, %rsi
	jmp	.LBB14_14
.LBB14_17:
	addq	%rbx, %r9
	movq	%r9, %r10
.LBB14_41:
	movq	%rdi, (%r10)
.LBB14_42:
	incq	%rsi
	addq	$8, %rax
	cmpq	%r12, %rsi
	je	.LBB14_18
.LBB14_14:
	movq	(%rbp,%rsi,8), %rdi
	movq	%rdi, (%rbx,%rsi,8)
	movq	-8(%rbx,%rsi,8), %r9
	movq	%rdi, %r8
	sarq	$63, %r8
	shrq	%r8
	xorq	%rdi, %r8
	movq	%r9, %r10
	sarq	$63, %r10
	shrq	%r10
	xorq	%r9, %r10
	cmpq	%r10, %r8
	jge	.LBB14_42
	movq	%r9, (%rbx,%rsi,8)
	movq	%rax, %r9
	movq	%rbx, %r10
	cmpq	$1, %rsi
	je	.LBB14_41
	.p2align	4
.LBB14_16:
	movq	-8(%rbx,%r9), %r10
	movq	%r10, %r11
	sarq	$63, %r11
	shrq	%r11
	xorq	%r10, %r11
	cmpq	%r11, %r8
	jge	.LBB14_17
	movq	%r10, (%rbx,%r9)
	addq	$-8, %r9
	jne	.LBB14_16
	movq	%rbx, %r10
	jmp	.LBB14_41
.LBB14_18:
	leaq	(%rbx,%r12,8), %rax
	cmpq	%rdx, %rcx
	jae	.LBB14_28
	leaq	(,%r12,8), %rsi
	addq	%rbp, %rsi
	leaq	-8(,%rcx,8), %rdi
	jmp	.LBB14_20
.LBB14_25:
	addq	%rax, %r10
	movq	%r10, %r11
.LBB14_26:
	movq	%r8, (%r11)
.LBB14_27:
	incq	%rcx
	addq	$8, %rdi
	cmpq	%rdx, %rcx
	je	.LBB14_28
.LBB14_20:
	movq	(%rsi,%rcx,8), %r8
	movq	%r8, (%rax,%rcx,8)
	movq	-8(%rax,%rcx,8), %r10
	movq	%r8, %r9
	sarq	$63, %r9
	shrq	%r9
	xorq	%r8, %r9
	movq	%r10, %r11
	sarq	$63, %r11
	shrq	%r11
	xorq	%r10, %r11
	cmpq	%r11, %r9
	jge	.LBB14_27
	movq	%r10, (%rax,%rcx,8)
	movq	%rdi, %r10
	movq	%rax, %r11
	cmpq	$1, %rcx
	je	.LBB14_26
	.p2align	4
.LBB14_22:
	movq	-8(%rax,%r10), %r11
	movq	%r11, %r13
	sarq	$63, %r13
	shrq	%r13
	xorq	%r11, %r13
	cmpq	%r13, %r9
	jge	.LBB14_25
	movq	%r11, (%rax,%r10)
	addq	$-8, %r10
	jne	.LBB14_22
	movq	%rax, %r11
	jmp	.LBB14_26
.LBB14_28:
	leaq	-1(%r14), %rdi
	leaq	(%rbx,%r14,8), %rdx
	addq	$-8, %rdx
	leaq	-8(%rax), %r15
	xorl	%r8d, %r8d
	movq	%rbx, %rsi
	movq	%r12, 8(%rsp)
	movq	(%rsp), %rcx
	.p2align	4
.LBB14_29:
	movq	(%rax), %r9
	movq	(%rsi), %r10
	movq	%r9, %r11
	sarq	$63, %r11
	shrq	%r11
	xorq	%r9, %r11
	movq	%r10, %r13
	sarq	$63, %r13
	shrq	%r13
	xorq	%r10, %r13
	xorl	%ebp, %ebp
	xorl	%r12d, %r12d
	cmpq	%r13, %r11
	cmovgeq	%r10, %r9
	movq	%r9, (%rcx,%r8,8)
	leaq	1(%r8), %r8
	setge	%bpl
	setl	%r12b
	leaq	(%rax,%r12,8), %rax
	leaq	(%rsi,%rbp,8), %rsi
	movq	(%rdx), %r9
	movq	(%r15), %r10
	movq	%r9, %r11
	sarq	$63, %r11
	shrq	%r11
	xorq	%r9, %r11
	movq	%r10, %r12
	sarq	$63, %r12
	shrq	%r12
	xorq	%r10, %r12
	xorl	%r13d, %r13d
	xorl	%ebp, %ebp
	cmpq	%r12, %r11
	movq	8(%rsp), %r12
	setl	%r13b
	cmovlq	%r10, %r9
	setge	%bpl
	movq	%r9, (%rcx,%rdi,8)
	shll	$3, %ebp
	subq	%rbp, %rdx
	shll	$3, %r13d
	subq	%r13, %r15
	decq	%rdi
	cmpq	%r8, %r12
	jne	.LBB14_29
	addq	$8, %r15
	addq	$8, %rdx
	testb	$1, %r14b
	jne	.LBB14_32
	movq	(%rsp), %r13
	jmp	.LBB14_33
.LBB14_32:
	xorl	%edi, %edi
	xorl	%r9d, %r9d
	cmpq	%r15, %rsi
	setae	%dil
	setb	%r9b
	movq	%rax, %r10
	cmovbq	%rsi, %r10
	movq	(%r10), %r10
	movq	(%rsp), %r13
	movq	%r10, (%r13,%r8,8)
	leaq	(%rsi,%r9,8), %rsi
	leaq	(%rax,%rdi,8), %rax
.LBB14_33:
	cmpq	%rdx, %rax
	setne	%al
	cmpq	%r15, %rsi
	jne	.LBB14_34
	testb	%al, %al
	jne	.LBB14_36
.LBB14_47:
	addq	$72, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.LBB14_105:
	.cfi_def_cfa_offset 128
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.21(%rip), %rdi
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.7(%rip), %rdx
	movl	$19, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.LBB14_34:
	movb	$1, %al
	testb	%al, %al
	je	.LBB14_47
.LBB14_36:
.Ltmp6:
	vzeroupper
	callq	*_RNvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort22panic_on_ord_violation@GOTPCREL(%rip)
.Ltmp7:
.LBB14_37:
	ud2
.LBB14_104:
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.8(%rip), %rcx
	movq	%rbp, %rdi
	movq	%r13, %rsi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB14_38:
.Ltmp8:
	movq	%rax, %r12
	shlq	$3, %r14
	movq	%r13, %rdi
	movq	%rbx, %rsi
	movq	%r14, %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	%r12, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end14:
	.size	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end14-_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc
	.section	.gcc_except_table._RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main,"a",@progbits
	.p2align	2, 0x0
GCC_except_table14:
.Lexception2:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end2-.Lcst_begin2
.Lcst_begin2:
	.uleb128 .Lfunc_begin2-.Lfunc_begin2
	.uleb128 .Ltmp6-.Lfunc_begin2
	.byte	0
	.byte	0
	.uleb128 .Ltmp6-.Lfunc_begin2
	.uleb128 .Ltmp7-.Ltmp6
	.uleb128 .Ltmp8-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp7-.Lfunc_begin2
	.uleb128 .Lfunc_end14-.Ltmp7
	.byte	0
	.byte	0
.Lcst_end2:
	.p2align	2, 0x0

	.section	.text._RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0CsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end15, nop
	.type	_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0CsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0CsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsaU0XrY2Q1Mf_22flat_natural_cast_main
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end15:
	.size	_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0CsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end15-_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0CsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end16, nop
	.type	_RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsaU0XrY2Q1Mf_22flat_natural_cast_main
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end16:
	.size	_RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end16-_RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.rodata.cst32,"aM",@progbits,32
	.p2align	5, 0x0
.LCPI17_0:
	.long	0
	.long	3221225472
	.long	1073741823
	.long	0
	.long	3221225472
	.long	1073741823
	.long	0
	.long	3221225472
.LCPI17_1:
	.long	1073741824
	.long	0
	.long	2147483647
	.long	1073741824
	.long	0
	.long	2147483647
	.long	1073741824
	.long	0
.LCPI17_2:
	.long	1073741824
	.long	0
	.long	2147483647
	.long	0
	.long	4286578688
	.long	8388607
	.long	536870912
	.long	0
.LCPI17_3:
	.long	128
	.long	0
	.long	255
	.long	128
	.long	0
	.long	255
	.long	128
	.long	0
.LCPI17_5:
	.long	0
	.long	1
	.long	2
	.long	3
	.long	4
	.long	5
	.long	6
	.long	7
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.LCPI17_4:
	.long	0x42c80000
	.long	0x4af42400
	.zero	4
	.zero	4
.LCPI17_19:
	.long	1127219200
	.long	1160773632
	.long	0
	.long	0
.LCPI17_20:
	.quad	0x4330000000000000
	.quad	0x4530000000000000
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI17_6:
	.long	8
.LCPI17_7:
	.long	16
.LCPI17_8:
	.long	24
.LCPI17_9:
	.long	0x3fa020c5
.LCPI17_10:
	.long	0xc2808000
.LCPI17_11:
	.long	32
.LCPI17_12:
	.long	0x40a00000
.LCPI17_13:
	.long	0x41200000
.LCPI17_14:
	.long	0x3fb374bc
.LCPI17_15:
	.long	0xbeb031cf
.LCPI17_16:
	.long	0xbf36d1e1
.LCPI17_17:
	.long	0x3fe2d0e5
.LCPI17_18:
	.long	0x4f000000
	.section	.rodata.cst8,"aM",@progbits,8
	.p2align	3, 0x0
.LCPI17_21:
	.quad	0x41cdcd6500000000
.LCPI17_22:
	.quad	0x409f400000000000
.LCPI17_23:
	.quad	0xbff0000000000000
.LCPI17_24:
	.quad	0x4059000000000000
	.section	.text._RNvCsaU0XrY2Q1Mf_22flat_natural_cast_main4main,"ax",@progbits
	.hidden	_RNvCsaU0XrY2Q1Mf_22flat_natural_cast_main4main
	.globl	_RNvCsaU0XrY2Q1Mf_22flat_natural_cast_main4main
	.prefalign	4, .Lfunc_end17, nop
	.type	_RNvCsaU0XrY2Q1Mf_22flat_natural_cast_main4main,@function
_RNvCsaU0XrY2Q1Mf_22flat_natural_cast_main4main:
.Lfunc_begin3:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception3
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$1224, %rsp
	.cfi_def_cfa_offset 1280
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	leaq	_RNvNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7control18inverse_and_finish(%rip), %rax
	movq	%rax, 832(%rsp)
	leaq	_RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel18inverse_and_finish(%rip), %rcx
	movq	%rcx, 840(%rsp)
	leaq	_RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel18inverse_and_finish(%rip), %rcx
	movq	%rcx, 848(%rsp)
	movq	$0, 984(%rsp)
	movq	$4, 992(%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.14(%rip), %rcx
	movq	%rcx, 1000(%rsp)
	movq	$8, 1008(%rsp)
	vmovaps	.LCPI17_0(%rip), %ymm0
	vmovups	%ymm0, 1016(%rsp)
	movq	%rax, 856(%rsp)
	movl	$1073741823, 1048(%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.15(%rip), %rax
	movq	%rax, 1056(%rsp)
	movq	$10, 1064(%rsp)
	vmovaps	.LCPI17_1(%rip), %ymm0
	vmovups	%ymm0, 1072(%rsp)
	leaq	1000(%rsp), %rdx
	movl	$2147483647, 1104(%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.16(%rip), %rax
	movq	%rax, 1112(%rsp)
	movq	$5, 1120(%rsp)
	vmovaps	.LCPI17_2(%rip), %ymm0
	vmovups	%ymm0, 1128(%rsp)
	movl	$1073741823, 1160(%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.17(%rip), %rax
	movq	%rax, 1168(%rsp)
	movq	$9, 1176(%rsp)
	vmovapd	.LCPI17_3(%rip), %ymm0
	vmovupd	%ymm0, 1184(%rsp)
	movl	$255, 1216(%rsp)
	leaq	64(%rsp), %r15
	movabsq	$549755813920, %rsi
	vmovq	.LCPI17_4(%rip), %xmm1
	leaq	48(%rsp), %r12
	xorl	%eax, %eax
.LBB17_1:
	leaq	1(%rax), %rcx
	movq	%rcx, 984(%rsp)
	imulq	$56, %rax, %rax
	movq	(%rdx,%rax), %rcx
	testq	%rcx, %rcx
	je	.LBB17_120
	movq	$0, 224(%rsp)
	addq	%rdx, %rax
	movq	8(%rax), %rdx
	movq	%rcx, 864(%rsp)
	movq	%rdx, 872(%rsp)
	movl	48(%rax), %ecx
	movl	%ecx, 976(%rsp)
	vmovups	16(%rax), %ymm0
	vmovups	%ymm0, 944(%rsp)
	movq	$3, 888(%rsp)
	movq	%rsi, 896(%rsp)
	movl	$1024, 904(%rsp)
	xorl	%eax, %eax
.LBB17_3:
	leaq	1(%rax), %rcx
	movq	%rcx, 632(%rsp)
	movq	%rcx, 880(%rsp)
	movl	896(%rsp,%rax,4), %eax
	movl	%eax, 44(%rsp)
	movq	$3, 920(%rsp)
	vmovq	%xmm1, 928(%rsp)
	movl	$1314296740, 936(%rsp)
	xorl	%eax, %eax
.LBB17_4:
	leaq	1(%rax), %rcx
	movl	44(%rsp), %r14d
	movq	%rcx, 640(%rsp)
	movq	%rcx, 912(%rsp)
	vmovss	928(%rsp,%rax,4), %xmm0
	vmovss	%xmm0, 268(%rsp)
	testl	%r14d, %r14d
	movl	$0, %ebx
	cmovgl	%r14d, %ebx
	jle	.LBB17_8
	vmovaps	%xmm0, 192(%rsp)
	shlq	$2, %rbx
	vzeroupper
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	%rbx, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB17_128
	cmpl	$8, %r14d
	vmovss	.LCPI17_9(%rip), %xmm6
	vmovss	.LCPI17_10(%rip), %xmm7
	vbroadcastss	.LCPI17_9(%rip), %ymm8
	vbroadcastss	.LCPI17_10(%rip), %ymm9
	jae	.LBB17_9
	xorl	%ecx, %ecx
	vmovaps	192(%rsp), %xmm10
	jmp	.LBB17_18
	.p2align	4
.LBB17_8:
	movq	$0, 48(%rsp)
	movq	$4, 56(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%xmm0, (%r15)
	movq	$4, 80(%rsp)
	vmovups	%xmm0, 24(%r15)
	movl	$4, %eax
	xorl	%r14d, %r14d
	jmp	.LBB17_46
	.p2align	4
.LBB17_9:
	cmpl	$32, %r14d
	vmovaps	192(%rsp), %xmm10
	jae	.LBB17_11
	xorl	%ecx, %ecx
	jmp	.LBB17_15
.LBB17_11:
	movl	%r14d, %ecx
	andl	$2147483616, %ecx
	vbroadcastss	%xmm10, %ymm0
	movl	%r14d, %edx
	shrl	$5, %edx
	andl	$67108863, %edx
	shlq	$7, %rdx
	xorl	%esi, %esi
	vmovdqa	.LCPI17_5(%rip), %ymm1
	.p2align	4
.LBB17_12:
	vpbroadcastd	.LCPI17_6(%rip), %ymm2
	vpaddd	%ymm2, %ymm1, %ymm2
	vpbroadcastd	.LCPI17_7(%rip), %ymm3
	vpaddd	%ymm3, %ymm1, %ymm3
	vpbroadcastd	.LCPI17_8(%rip), %ymm4
	vpaddd	%ymm4, %ymm1, %ymm4
	vcvtdq2ps	%ymm1, %ymm5
	vcvtdq2ps	%ymm2, %ymm2
	vcvtdq2ps	%ymm3, %ymm3
	vcvtdq2ps	%ymm4, %ymm4
	vmulps	%ymm5, %ymm8, %ymm5
	vmulps	%ymm2, %ymm8, %ymm2
	vmulps	%ymm3, %ymm8, %ymm3
	vmulps	%ymm4, %ymm8, %ymm4
	vaddps	%ymm5, %ymm9, %ymm5
	vaddps	%ymm2, %ymm9, %ymm2
	vaddps	%ymm3, %ymm9, %ymm3
	vaddps	%ymm4, %ymm9, %ymm4
	vmulps	%ymm5, %ymm0, %ymm5
	vmulps	%ymm2, %ymm0, %ymm2
	vmulps	%ymm3, %ymm0, %ymm3
	vmulps	%ymm4, %ymm0, %ymm4
	vmovups	%ymm5, (%rax,%rsi)
	vmovups	%ymm2, 32(%rax,%rsi)
	vmovups	%ymm3, 64(%rax,%rsi)
	vmovups	%ymm4, 96(%rax,%rsi)
	vpbroadcastd	.LCPI17_11(%rip), %ymm2
	vpaddd	%ymm2, %ymm1, %ymm1
	subq	$-128, %rsi
	cmpq	%rsi, %rdx
	jne	.LBB17_12
	cmpl	%r14d, %ecx
	je	.LBB17_19
	testb	$24, %r14b
	je	.LBB17_18
.LBB17_15:
	movq	%rcx, %rdx
	movl	%r14d, %ecx
	andl	$2147483640, %ecx
	vbroadcastss	%xmm10, %ymm0
	vmovd	%edx, %xmm1
	vpbroadcastd	%xmm1, %ymm1
	vpor	.LCPI17_5(%rip), %ymm1, %ymm1
	.p2align	4
.LBB17_16:
	vcvtdq2ps	%ymm1, %ymm2
	vmulps	%ymm2, %ymm8, %ymm2
	vaddps	%ymm2, %ymm9, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vmovups	%ymm2, (%rax,%rdx,4)
	addq	$8, %rdx
	vpbroadcastd	.LCPI17_6(%rip), %ymm2
	vpaddd	%ymm2, %ymm1, %ymm1
	cmpq	%rdx, %rcx
	jne	.LBB17_16
	cmpl	%r14d, %ecx
	je	.LBB17_19
	.p2align	4
.LBB17_18:
	vcvtsi2ss	%ecx, %xmm15, %xmm0
	vmulss	%xmm6, %xmm0, %xmm0
	vaddss	%xmm7, %xmm0, %xmm0
	vmulss	%xmm0, %xmm10, %xmm0
	vmovss	%xmm0, (%rax,%rcx,4)
	incq	%rcx
	cmpq	%rcx, %r14
	jne	.LBB17_18
.LBB17_19:
	movq	%r14, 48(%rsp)
	movq	%rax, 56(%rsp)
	movq	%r14, 64(%rsp)
	vzeroupper
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	%rbx, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB17_126
	cmpl	$8, %r14d
	vmovss	.LCPI17_9(%rip), %xmm6
	vmovss	.LCPI17_10(%rip), %xmm7
	vmovss	.LCPI17_12(%rip), %xmm8
	vbroadcastss	.LCPI17_12(%rip), %ymm11
	jae	.LBB17_22
	xorl	%ecx, %ecx
	vmovaps	192(%rsp), %xmm12
	jmp	.LBB17_31
	.p2align	4
.LBB17_22:
	cmpl	$32, %r14d
	vbroadcastss	.LCPI17_9(%rip), %ymm9
	vbroadcastss	.LCPI17_10(%rip), %ymm10
	vmovaps	192(%rsp), %xmm12
	jae	.LBB17_24
	xorl	%ecx, %ecx
	jmp	.LBB17_28
.LBB17_24:
	movl	%r14d, %ecx
	andl	$2147483616, %ecx
	vbroadcastss	%xmm12, %ymm0
	xorl	%edx, %edx
	vmovdqa	.LCPI17_5(%rip), %ymm1
	.p2align	4
.LBB17_25:
	vpbroadcastd	.LCPI17_6(%rip), %ymm2
	vpaddd	%ymm2, %ymm1, %ymm2
	vpbroadcastd	.LCPI17_7(%rip), %ymm3
	vpaddd	%ymm3, %ymm1, %ymm3
	vpbroadcastd	.LCPI17_8(%rip), %ymm4
	vpaddd	%ymm4, %ymm1, %ymm4
	vcvtdq2ps	%ymm1, %ymm5
	vcvtdq2ps	%ymm2, %ymm2
	vcvtdq2ps	%ymm3, %ymm3
	vcvtdq2ps	%ymm4, %ymm4
	vmulps	%ymm5, %ymm9, %ymm5
	vmulps	%ymm2, %ymm9, %ymm2
	vmulps	%ymm3, %ymm9, %ymm3
	vmulps	%ymm4, %ymm9, %ymm4
	vaddps	%ymm5, %ymm10, %ymm5
	vaddps	%ymm2, %ymm10, %ymm2
	vaddps	%ymm3, %ymm10, %ymm3
	vaddps	%ymm4, %ymm10, %ymm4
	vaddps	%ymm5, %ymm11, %ymm5
	vaddps	%ymm2, %ymm11, %ymm2
	vaddps	%ymm3, %ymm11, %ymm3
	vaddps	%ymm4, %ymm11, %ymm4
	vmulps	%ymm5, %ymm0, %ymm5
	vmulps	%ymm2, %ymm0, %ymm2
	vmulps	%ymm3, %ymm0, %ymm3
	vmulps	%ymm4, %ymm0, %ymm4
	vmovups	%ymm5, (%rax,%rdx,4)
	vmovups	%ymm2, 32(%rax,%rdx,4)
	vmovups	%ymm3, 64(%rax,%rdx,4)
	vmovups	%ymm4, 96(%rax,%rdx,4)
	addq	$32, %rdx
	vpbroadcastd	.LCPI17_11(%rip), %ymm2
	vpaddd	%ymm2, %ymm1, %ymm1
	cmpq	%rdx, %rcx
	jne	.LBB17_25
	cmpl	%r14d, %ecx
	je	.LBB17_32
	testb	$24, %r14b
	je	.LBB17_31
.LBB17_28:
	movq	%rcx, %rdx
	movl	%r14d, %ecx
	andl	$2147483640, %ecx
	vbroadcastss	%xmm12, %ymm0
	vmovd	%edx, %xmm1
	vpbroadcastd	%xmm1, %ymm1
	vpor	.LCPI17_5(%rip), %ymm1, %ymm1
	.p2align	4
.LBB17_29:
	vcvtdq2ps	%ymm1, %ymm2
	vmulps	%ymm2, %ymm9, %ymm2
	vaddps	%ymm2, %ymm10, %ymm2
	vaddps	%ymm2, %ymm11, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vmovups	%ymm2, (%rax,%rdx,4)
	addq	$8, %rdx
	vpbroadcastd	.LCPI17_6(%rip), %ymm2
	vpaddd	%ymm2, %ymm1, %ymm1
	cmpq	%rdx, %rcx
	jne	.LBB17_29
	cmpl	%r14d, %ecx
	je	.LBB17_32
	.p2align	4
.LBB17_31:
	vcvtsi2ss	%ecx, %xmm15, %xmm0
	vmulss	%xmm6, %xmm0, %xmm0
	vaddss	%xmm7, %xmm0, %xmm0
	vaddss	%xmm0, %xmm8, %xmm0
	vmulss	%xmm0, %xmm12, %xmm0
	vmovss	%xmm0, (%rax,%rcx,4)
	incq	%rcx
	cmpq	%rcx, %r14
	jne	.LBB17_31
.LBB17_32:
	movq	%r14, 72(%rsp)
	movq	%rax, 80(%rsp)
	movq	%r14, 88(%rsp)
	vzeroupper
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	%rbx, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB17_125
	cmpl	$8, %r14d
	vmovss	.LCPI17_9(%rip), %xmm6
	vmovss	.LCPI17_10(%rip), %xmm7
	vmovss	.LCPI17_13(%rip), %xmm8
	vbroadcastss	.LCPI17_13(%rip), %ymm11
	jae	.LBB17_35
	xorl	%ecx, %ecx
	vmovaps	192(%rsp), %xmm12
	jmp	.LBB17_44
	.p2align	4
.LBB17_35:
	cmpl	$32, %r14d
	vbroadcastss	.LCPI17_9(%rip), %ymm9
	vbroadcastss	.LCPI17_10(%rip), %ymm10
	vmovaps	192(%rsp), %xmm12
	jae	.LBB17_37
	xorl	%ecx, %ecx
	jmp	.LBB17_41
.LBB17_37:
	movl	%r14d, %ecx
	andl	$2147483616, %ecx
	vbroadcastss	%xmm12, %ymm0
	xorl	%edx, %edx
	vmovdqa	.LCPI17_5(%rip), %ymm1
	.p2align	4
.LBB17_38:
	vpbroadcastd	.LCPI17_6(%rip), %ymm2
	vpaddd	%ymm2, %ymm1, %ymm2
	vpbroadcastd	.LCPI17_7(%rip), %ymm3
	vpaddd	%ymm3, %ymm1, %ymm3
	vpbroadcastd	.LCPI17_8(%rip), %ymm4
	vpaddd	%ymm4, %ymm1, %ymm4
	vcvtdq2ps	%ymm1, %ymm5
	vcvtdq2ps	%ymm2, %ymm2
	vcvtdq2ps	%ymm3, %ymm3
	vcvtdq2ps	%ymm4, %ymm4
	vmulps	%ymm5, %ymm9, %ymm5
	vmulps	%ymm2, %ymm9, %ymm2
	vmulps	%ymm3, %ymm9, %ymm3
	vmulps	%ymm4, %ymm9, %ymm4
	vaddps	%ymm5, %ymm10, %ymm5
	vaddps	%ymm2, %ymm10, %ymm2
	vaddps	%ymm3, %ymm10, %ymm3
	vaddps	%ymm4, %ymm10, %ymm4
	vaddps	%ymm5, %ymm11, %ymm5
	vaddps	%ymm2, %ymm11, %ymm2
	vaddps	%ymm3, %ymm11, %ymm3
	vaddps	%ymm4, %ymm11, %ymm4
	vmulps	%ymm5, %ymm0, %ymm5
	vmulps	%ymm2, %ymm0, %ymm2
	vmulps	%ymm3, %ymm0, %ymm3
	vmulps	%ymm4, %ymm0, %ymm4
	vmovups	%ymm5, (%rax,%rdx,4)
	vmovups	%ymm2, 32(%rax,%rdx,4)
	vmovups	%ymm3, 64(%rax,%rdx,4)
	vmovups	%ymm4, 96(%rax,%rdx,4)
	addq	$32, %rdx
	vpbroadcastd	.LCPI17_11(%rip), %ymm2
	vpaddd	%ymm2, %ymm1, %ymm1
	cmpq	%rdx, %rcx
	jne	.LBB17_38
	cmpl	%r14d, %ecx
	je	.LBB17_45
	testb	$24, %r14b
	je	.LBB17_44
.LBB17_41:
	movq	%rcx, %rdx
	movl	%r14d, %ecx
	andl	$2147483640, %ecx
	vbroadcastss	%xmm12, %ymm0
	vmovd	%edx, %xmm1
	vpbroadcastd	%xmm1, %ymm1
	vpor	.LCPI17_5(%rip), %ymm1, %ymm1
	.p2align	4
.LBB17_42:
	vcvtdq2ps	%ymm1, %ymm2
	vmulps	%ymm2, %ymm9, %ymm2
	vaddps	%ymm2, %ymm10, %ymm2
	vaddps	%ymm2, %ymm11, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vmovups	%ymm2, (%rax,%rdx,4)
	addq	$8, %rdx
	vpbroadcastd	.LCPI17_6(%rip), %ymm2
	vpaddd	%ymm2, %ymm1, %ymm1
	cmpq	%rdx, %rcx
	jne	.LBB17_42
	cmpl	%r14d, %ecx
	je	.LBB17_45
	.p2align	4
.LBB17_44:
	vcvtsi2ss	%ecx, %xmm15, %xmm0
	vmulss	%xmm6, %xmm0, %xmm0
	vaddss	%xmm7, %xmm0, %xmm0
	vaddss	%xmm0, %xmm8, %xmm0
	vmulss	%xmm0, %xmm12, %xmm0
	vmovss	%xmm0, (%rax,%rcx,4)
	incq	%rcx
	cmpq	%rcx, %r14
	jne	.LBB17_44
.LBB17_45:
	movq	%r14, 96(%rsp)
.LBB17_46:
	movq	%rax, 104(%rsp)
	movq	48(%rsp), %rax
	movq	%rax, 416(%rsp)
	movq	56(%rsp), %rax
	vmovups	64(%rsp), %xmm0
	vmovups	%xmm0, 432(%rsp)
	movq	%rax, 424(%rsp)
	movq	%r14, 480(%rsp)
	movq	80(%rsp), %rax
	movq	%rax, 448(%rsp)
	movq	88(%rsp), %rax
	movq	%rax, 456(%rsp)
	movq	96(%rsp), %rax
	movq	%rax, 464(%rsp)
	movq	104(%rsp), %rax
	movq	%rax, 472(%rsp)
	movl	976(%rsp), %eax
	movl	%eax, 400(%rsp)
	vmovups	944(%rsp), %ymm0
	vmovups	%ymm0, 368(%rsp)
	leaq	368(%rsp), %rax
	#APP
	#NO_APP
.Ltmp12:
	movq	%r12, %rdi
	leaq	416(%rsp), %rsi
	vzeroupper
	callq	_RINvNtCs8uuN1nrGffB_4core5array11try_from_fnINtNtNtB4_3ops9try_trait17NeverShortCircuitINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEKj3_NCINvMBJ_BG_10wrap_mut_1jNCINvXsk_B2_B1n_NtB2_14SpecArrayClone5cloneKB1Y_E0E0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
.Ltmp13:
	movq	112(%rsp), %rdx
	vmovdqu	48(%rsp), %ymm0
	vmovups	80(%rsp), %ymm1
	vmovups	%ymm1, 752(%rsp)
	vmovdqu	%ymm0, 720(%rsp)
	movq	%rdx, 784(%rsp)
	movq	728(%rsp), %rax
	movq	%rax, 184(%rsp)
	movq	736(%rsp), %rcx
	movq	752(%rsp), %r15
	movq	760(%rsp), %rsi
	movq	776(%rsp), %r14
	movq	%rcx, 16(%rsp)
	shrq	$3, %rcx
	movq	%rsi, %rax
	movq	%rsi, 32(%rsp)
	shrq	$3, %rax
	movq	%rdx, 24(%rsp)
	shrq	$3, %rdx
	cmpq	%rax, %rcx
	cmovbq	%rcx, %rax
	cmpq	%rdx, %rax
	cmovaeq	%rdx, %rax
	testq	%rax, %rax
	vbroadcastss	.LCPI17_14(%rip), %ymm15
	vbroadcastss	.LCPI17_18(%rip), %ymm3
	je	.LBB17_50
	movl	368(%rsp), %esi
	movl	372(%rsp), %edi
	movl	376(%rsp), %r8d
	movl	380(%rsp), %r9d
	movl	384(%rsp), %r10d
	movl	388(%rsp), %r11d
	movl	392(%rsp), %ecx
	movl	396(%rsp), %ebx
	movl	400(%rsp), %edx
	subl	%esi, %edi
	vmovd	%edi, %xmm0
	vpbroadcastd	%xmm0, %ymm0
	vmovdqu	%ymm0, 192(%rsp)
	vmovd	%r8d, %xmm0
	subl	%esi, %r8d
	vmovd	%r8d, %xmm1
	vpbroadcastd	%xmm1, %ymm1
	vmovdqu	%ymm1, 544(%rsp)
	vmovd	%esi, %xmm2
	movq	184(%rsp), %rsi
	vpbroadcastd	%xmm2, %ymm1
	vmovdqu	%ymm1, 800(%rsp)
	vpbroadcastd	%xmm0, %ymm0
	vmovdqu	%ymm0, 688(%rsp)
	subl	%r9d, %r10d
	vmovd	%r10d, %xmm4
	vpbroadcastd	%xmm4, %ymm4
	vmovd	%r11d, %xmm7
	subl	%r9d, %r11d
	vmovd	%r11d, %xmm5
	vpbroadcastd	%xmm5, %ymm5
	vmovd	%r9d, %xmm6
	vpbroadcastd	%xmm6, %ymm6
	vpbroadcastd	%xmm7, %ymm7
	subl	%ecx, %ebx
	vmovd	%ebx, %xmm8
	vpbroadcastd	%xmm8, %ymm8
	vmovd	%edx, %xmm11
	subl	%ecx, %edx
	vmovd	%edx, %xmm9
	vpbroadcastd	%xmm9, %ymm9
	vmovd	%ecx, %xmm10
	vpbroadcastd	%xmm10, %ymm10
	vpbroadcastd	%xmm11, %ymm11
	xorl	%ecx, %ecx
	.p2align	4
.LBB17_49:
	vmovups	(%rsi,%rcx), %ymm12
	vmovups	(%r15,%rcx), %ymm13
	vmovups	(%r14,%rcx), %ymm14
	vmovaps	%ymm15, %ymm2
	vmulps	%ymm15, %ymm14, %ymm15
	vaddps	%ymm15, %ymm12, %ymm15
	vbroadcastss	.LCPI17_15(%rip), %ymm0
	vmulps	%ymm0, %ymm13, %ymm0
	vaddps	%ymm0, %ymm12, %ymm0
	vbroadcastss	.LCPI17_16(%rip), %ymm1
	vmulps	%ymm1, %ymm14, %ymm14
	vaddps	%ymm0, %ymm14, %ymm0
	vroundps	$8, %ymm15, %ymm14
	vcvttps2dq	%ymm14, %ymm14
	vpmaxsd	192(%rsp), %ymm14, %ymm14
	vpminsd	544(%rsp), %ymm14, %ymm14
	vpaddd	800(%rsp), %ymm14, %ymm14
	vcmpge_oqps	%ymm3, %ymm15, %ymm15
	vpblendvb	%ymm15, 688(%rsp), %ymm14, %ymm14
	vmovaps	%ymm2, %ymm15
	vbroadcastss	.LCPI17_17(%rip), %ymm1
	vmulps	%ymm1, %ymm13, %ymm13
	vmovdqu	%ymm14, (%rsi,%rcx)
	vroundps	$8, %ymm0, %ymm14
	vaddps	%ymm13, %ymm12, %ymm12
	vcvttps2dq	%ymm14, %ymm13
	vpmaxsd	%ymm4, %ymm13, %ymm13
	vpminsd	%ymm5, %ymm13, %ymm13
	vpaddd	%ymm6, %ymm13, %ymm13
	vcmpge_oqps	%ymm3, %ymm0, %ymm0
	vpblendvb	%ymm0, %ymm7, %ymm13, %ymm0
	vmovdqu	%ymm0, (%r15,%rcx)
	vroundps	$8, %ymm12, %ymm0
	vcvttps2dq	%ymm0, %ymm0
	vpmaxsd	%ymm8, %ymm0, %ymm0
	vpminsd	%ymm9, %ymm0, %ymm0
	vpaddd	%ymm0, %ymm10, %ymm0
	vcmpge_oqps	%ymm3, %ymm12, %ymm12
	vpblendvb	%ymm12, %ymm11, %ymm0, %ymm0
	vmovdqu	%ymm0, (%r14,%rcx)
	addq	$32, %rcx
	decq	%rax
	jne	.LBB17_49
.LBB17_50:
	movq	%r15, 240(%rsp)
	movq	%r14, 232(%rsp)
	vmovups	832(%rsp), %ymm0
	leaq	592(%rsp), %rax
	vmovups	%ymm0, (%rax)
	movq	$4, 584(%rsp)
	movq	16(%rsp), %rax
	leaq	(,%rax,4), %rax
	movq	%rax, 800(%rsp)
	movq	32(%rsp), %rax
	leaq	(,%rax,4), %rax
	movq	%rax, 688(%rsp)
	movq	24(%rsp), %rax
	leaq	(,%rax,4), %rax
	movq	%rax, 256(%rsp)
	xorl	%r15d, %r15d
	jmp	.LBB17_52
	.p2align	4
.LBB17_51:
	incq	%r15
	cmpq	$4, %r15
	je	.LBB17_66
.LBB17_52:
	movq	592(%rsp,%r15,8), %rbx
.Ltmp15:
	movq	%r12, %rdi
	leaq	416(%rsp), %rsi
	vzeroupper
	callq	_RINvNtCs8uuN1nrGffB_4core5array11try_from_fnINtNtNtB4_3ops9try_trait17NeverShortCircuitINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEKj3_NCINvMBJ_BG_10wrap_mut_1jNCINvXsk_B2_B1n_NtB2_14SpecArrayClone5cloneKB1Y_E0E0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
.Ltmp16:
	movq	%r12, %rdi
	movq	112(%rsp), %r14
	movq	%r14, 336(%rsp)
	vmovups	48(%rsp), %ymm0
	vmovupd	80(%rsp), %ymm1
	vmovupd	%ymm1, 304(%rsp)
	vmovups	%ymm0, 272(%rsp)
	movq	280(%rsp), %rbp
	movq	288(%rsp), %r12
	movq	304(%rsp), %rcx
	movq	312(%rsp), %r13
	movq	328(%rsp), %rax
	movq	%rbp, 48(%rsp)
	movq	%r12, 56(%rsp)
	movq	%rcx, 192(%rsp)
	movq	%rcx, 64(%rsp)
	movq	%r13, 72(%rsp)
	movq	%rax, 544(%rsp)
	movq	%rax, 80(%rsp)
	movq	%r14, 88(%rsp)
	movl	400(%rsp), %eax
	movl	%eax, 528(%rsp)
	vmovupd	368(%rsp), %ymm0
	vmovupd	%ymm0, 496(%rsp)
.Ltmp18:
	xorl	%esi, %esi
	leaq	496(%rsp), %rdx
	vzeroupper
	callq	*%rbx
.Ltmp19:
	cmpq	16(%rsp), %r12
	movq	bcmp@GOTPCREL(%rip), %rbx
	jne	.LBB17_124
	movq	%rbp, %rdi
	movq	184(%rsp), %rsi
	movq	800(%rsp), %rdx
	callq	*%rbx
	testl	%eax, %eax
	jne	.LBB17_124
	cmpq	32(%rsp), %r13
	jne	.LBB17_124
	movq	192(%rsp), %rdi
	movq	240(%rsp), %rsi
	movq	688(%rsp), %rdx
	callq	*%rbx
	testl	%eax, %eax
	jne	.LBB17_124
	cmpq	24(%rsp), %r14
	jne	.LBB17_124
	movq	544(%rsp), %rdi
	movq	232(%rsp), %rsi
	movq	256(%rsp), %rdx
	callq	*%rbx
	testl	%eax, %eax
	jne	.LBB17_124
	movq	272(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_62
	shlq	$2, %rsi
	movl	$4, %edx
	movq	%rbp, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_62:
	movq	296(%rsp), %rsi
	testq	%rsi, %rsi
	leaq	48(%rsp), %r12
	je	.LBB17_64
	shlq	$2, %rsi
	movl	$4, %edx
	movq	192(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_64:
	movq	320(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_51
	shlq	$2, %rsi
	movl	$4, %edx
	movq	544(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	jmp	.LBB17_51
	.p2align	4
.LBB17_66:
	movq	$0, 272(%rsp)
	movq	$8, 280(%rsp)
	leaq	288(%rsp), %rax
	vxorpd	%xmm0, %xmm0, %xmm0
	vmovupd	%xmm0, (%rax)
	movq	$8, 304(%rsp)
	vmovupd	%xmm0, 24(%rax)
	movq	$8, 328(%rsp)
	vmovupd	%xmm0, 48(%rax)
	movq	$8, 352(%rsp)
	movq	$0, 360(%rsp)
.Ltmp24:
	movq	%r12, %rdi
	leaq	416(%rsp), %rsi
	callq	_RINvNtCs8uuN1nrGffB_4core5array11try_from_fnINtNtNtB4_3ops9try_trait17NeverShortCircuitINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEKj3_NCINvMBJ_BG_10wrap_mut_1jNCINvXsk_B2_B1n_NtB2_14SpecArrayClone5cloneKB1Y_E0E0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
.Ltmp25:
	movq	48(%rsp), %r14
	movq	56(%rsp), %rax
	movq	%rax, 192(%rsp)
	movq	64(%rsp), %r12
	movq	72(%rsp), %rax
	movq	%rax, (%rsp)
	movq	80(%rsp), %rbp
	movq	88(%rsp), %r15
	movq	96(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	104(%rsp), %rax
	movq	%rax, 248(%rsp)
	movq	112(%rsp), %rax
	movq	%rax, 544(%rsp)
	leaq	(,%r12,4), %rax
	movq	%rax, 672(%rsp)
	xorl	%eax, %eax
	movq	%r14, 176(%rsp)
.LBB17_68:
	movq	%rax, 656(%rsp)
	incq	%rax
	movq	%rax, 648(%rsp)
	xorl	%ebx, %ebx
	jmp	.LBB17_70
	.p2align	4
.LBB17_69:
	movq	8(%r14), %rax
	vmovsd	16(%rsp), %xmm0
	vmovsd	%xmm0, (%rax,%rbx,8)
	incq	%rbx
	movq	%rbx, 16(%r14)
	movq	664(%rsp), %rax
	movq	%rax, %rbx
	cmpq	$4, %rax
	movq	176(%rsp), %r14
	je	.LBB17_80
.LBB17_70:
.Ltmp27:
	callq	*_RNvMNtCsh0g7Hz38UNf_3std4timeNtB2_7Instant3now@GOTPCREL(%rip)
.Ltmp28:
	leaq	1(%rbx), %rcx
	movq	%rcx, 664(%rsp)
	addl	656(%rsp), %ebx
	andl	$3, %ebx
	movq	%rbx, 32(%rsp)
	movq	%rax, 496(%rsp)
	movl	%edx, 504(%rsp)
	movq	424(%rsp), %rax
	movq	%rax, 24(%rsp)
	movq	432(%rsp), %rax
	movq	%rax, 800(%rsp)
	movq	448(%rsp), %rax
	movq	%rax, 688(%rsp)
	movq	456(%rsp), %rax
	movq	%rax, 256(%rsp)
	movq	472(%rsp), %rax
	movq	%rax, 408(%rsp)
	movl	$2000, %ecx
	movq	480(%rsp), %rax
	movq	%rax, 680(%rsp)
	leaq	48(%rsp), %r13
	.p2align	4
.LBB17_72:
	movq	24(%rsp), %rax
	movq	%rax, 48(%rsp)
	movq	800(%rsp), %rax
	movq	%rax, 56(%rsp)
	#APP
	#NO_APP
	movq	56(%rsp), %rsi
	cmpq	%rsi, %r12
	jne	.LBB17_123
	movl	%ecx, 16(%rsp)
	movq	48(%rsp), %rsi
	movq	192(%rsp), %rdi
	movq	672(%rsp), %rdx
	movq	memcpy@GOTPCREL(%rip), %r14
	callq	*%r14
	movq	688(%rsp), %rax
	movq	%rax, 48(%rsp)
	movq	256(%rsp), %rax
	movq	%rax, 56(%rsp)
	#APP
	#NO_APP
	movq	56(%rsp), %rsi
	cmpq	%rsi, %r15
	jne	.LBB17_121
	movq	48(%rsp), %rsi
	movq	%rbp, %rdi
	leaq	(,%r15,4), %rdx
	callq	*%r14
	movq	408(%rsp), %rax
	movq	%rax, 48(%rsp)
	movq	680(%rsp), %rax
	movq	%rax, 56(%rsp)
	#APP
	#NO_APP
	movq	56(%rsp), %rsi
	movq	544(%rsp), %rax
	cmpq	%rsi, %rax
	jne	.LBB17_122
	movq	48(%rsp), %rsi
	movq	%r15, %r13
	movq	%r12, %r15
	movq	248(%rsp), %r12
	movq	%r12, %rdi
	leaq	(,%rax,4), %rdx
	callq	*%r14
	movq	32(%rsp), %rax
	movq	832(%rsp,%rax,8), %rax
	movq	192(%rsp), %rcx
	movq	%rcx, 48(%rsp)
	movq	%r15, %rbx
	movq	%r15, 56(%rsp)
	movq	%r13, %r15
	leaq	48(%rsp), %r13
	movq	%rbp, %r14
	movq	%rbp, 64(%rsp)
	movq	%r15, 72(%rsp)
	movq	%r12, 80(%rsp)
	movq	544(%rsp), %rcx
	movq	%rcx, 88(%rsp)
	#APP
	#NO_APP
	movl	400(%rsp), %ecx
	movl	%ecx, 608(%rsp)
	vmovupd	368(%rsp), %ymm0
	vmovupd	%ymm0, 576(%rsp)
.Ltmp31:
	movq	%r13, %rdi
	xorl	%esi, %esi
	leaq	576(%rsp), %rdx
	vzeroupper
	callq	*%rax
.Ltmp32:
	movq	%rax, 48(%rsp)
	#APP
	#NO_APP
	movl	16(%rsp), %ecx
	decl	%ecx
	movq	%r14, %rbp
	movq	%rbx, %r12
	jne	.LBB17_72
.Ltmp34:
	leaq	496(%rsp), %rdi
	callq	*_RNvMNtCsh0g7Hz38UNf_3std4timeNtB2_7Instant7elapsed@GOTPCREL(%rip)
.Ltmp35:
	movq	32(%rsp), %rcx
	leaq	(%rcx,%rcx,2), %rcx
	vmovq	%rax, %xmm0
	vpunpckldq	.LCPI17_19(%rip), %xmm0, %xmm0
	vsubpd	.LCPI17_20(%rip), %xmm0, %xmm0
	vshufpd	$1, %xmm0, %xmm0, %xmm1
	vaddsd	%xmm0, %xmm1, %xmm0
	vcvtsi2sd	%edx, %xmm15, %xmm1
	leaq	272(%rsp,%rcx,8), %r14
	vmovsd	.LCPI17_21(%rip), %xmm2
	vdivsd	%xmm2, %xmm1, %xmm1
	vaddsd	%xmm0, %xmm1, %xmm0
	vmulsd	%xmm2, %xmm0, %xmm0
	vdivsd	.LCPI17_22(%rip), %xmm0, %xmm0
	vmovsd	%xmm0, 16(%rsp)
	movq	288(%rsp,%rcx,8), %rbx
	cmpq	272(%rsp,%rcx,8), %rbx
	jne	.LBB17_69
.Ltmp36:
	movq	%r14, %rdi
	callq	_RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCsaU0XrY2Q1Mf_22flat_natural_cast_main
.Ltmp37:
	jmp	.LBB17_69
	.p2align	4
.LBB17_80:
	movq	648(%rsp), %rax
	cmpq	$24, %rax
	jne	.LBB17_68
	leaq	272(%rsp), %rax
	movq	%rax, 576(%rsp)
	leaq	296(%rsp), %rax
	movq	%rax, 584(%rsp)
	leaq	320(%rsp), %rax
	movq	%rax, 592(%rsp)
	leaq	344(%rsp), %rax
	movq	%rax, 600(%rsp)
	xorl	%ebx, %ebx
	.p2align	4
.LBB17_82:
	movq	576(%rsp,%rbx,8), %rax
	movq	8(%rax), %r15
	movq	16(%rax), %r12
	cmpq	$2, %r12
	jae	.LBB17_86
	testq	%r12, %r12
	je	.LBB17_129
	xorl	%r12d, %r12d
.LBB17_85:
	vmovsd	(%r15,%r12,8), %xmm0
	vmovsd	%xmm0, 48(%rsp,%rbx,8)
	incq	%rbx
	cmpq	$4, %rbx
	jne	.LBB17_82
	jmp	.LBB17_90
.LBB17_86:
	cmpq	$21, %r12
	jae	.LBB17_89
	movq	%r15, %rdi
	movq	%r12, %rsi
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort25insertion_sort_shift_leftdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECsaU0XrY2Q1Mf_22flat_natural_cast_main
.LBB17_88:
	shrq	%r12
	jmp	.LBB17_85
.LBB17_89:
.Ltmp39:
	movq	%r15, %rdi
	movq	%r12, %rsi
	callq	_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECsaU0XrY2Q1Mf_22flat_natural_cast_main
.Ltmp40:
	jmp	.LBB17_88
.LBB17_90:
	vmovups	48(%rsp), %ymm0
	vmovups	%ymm0, 496(%rsp)
	vmovsd	512(%rsp), %xmm0
	vdivsd	504(%rsp), %xmm0, %xmm0
	vaddsd	.LCPI17_23(%rip), %xmm0, %xmm0
	vmulsd	.LCPI17_24(%rip), %xmm0, %xmm0
	vmovsd	%xmm0, 576(%rsp)
	leaq	864(%rsp), %rax
	movq	%rax, 48(%rsp)
	leaq	_RNvXs1i_NtCs8uuN1nrGffB_4core3fmtReNtB6_7Display3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main(%rip), %rax
	movq	%rax, 56(%rsp)
	leaq	44(%rsp), %rax
	movq	%rax, 64(%rsp)
	movq	_RNvXs9_NtNtNtCs8uuN1nrGffB_4core3fmt3num3implNtB9_7Display3fmt@GOTPCREL(%rip), %rax
	movq	%rax, 72(%rsp)
	leaq	268(%rsp), %rax
	movq	%rax, 80(%rsp)
	movq	_RNvXs3_NtNtCs8uuN1nrGffB_4core3fmt5floatfNtB7_7Display3fmt@GOTPCREL(%rip), %rax
	movq	%rax, 88(%rsp)
	leaq	496(%rsp), %rax
	movq	%rax, 96(%rsp)
	movq	_RNvXs7_NtNtCs8uuN1nrGffB_4core3fmt5floatdNtB7_7Display3fmt@GOTPCREL(%rip), %rcx
	movq	%rcx, 104(%rsp)
	leaq	504(%rsp), %rax
	movq	%rax, 112(%rsp)
	movq	%rcx, 120(%rsp)
	leaq	512(%rsp), %rax
	movq	%rax, 128(%rsp)
	movq	%rcx, 136(%rsp)
	leaq	520(%rsp), %rax
	movq	%rax, 144(%rsp)
	movq	%rcx, 152(%rsp)
	leaq	576(%rsp), %rax
	movq	%rax, 160(%rsp)
	movq	%rcx, 168(%rsp)
.Ltmp42:
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.18(%rip), %rdi
	leaq	48(%rsp), %r12
	movq	%r12, %rsi
	movq	_RNvNtNtCsh0g7Hz38UNf_3std2io5stdio6__print@GOTPCREL(%rip), %rax
	vzeroupper
	callq	*%rax
.Ltmp43:
	testq	%r14, %r14
	leaq	64(%rsp), %r15
	movq	8(%rsp), %rbx
	je	.LBB17_93
	shlq	$2, %r14
	movl	$4, %edx
	movq	192(%rsp), %rdi
	movq	%r14, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_93:
	movq	(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_95
	shlq	$2, %rsi
	movl	$4, %edx
	movq	%rbp, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_95:
	testq	%rbx, %rbx
	je	.LBB17_97
	shlq	$2, %rbx
	movl	$4, %edx
	movq	248(%rsp), %rdi
	movq	%rbx, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_97:
	movq	272(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_99
	movq	280(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_99:
	movq	296(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_101
	movq	304(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_101:
	movq	320(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_103
	movq	328(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_103:
	movq	344(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_105
	movq	352(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_105:
	movq	720(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_107
	shlq	$2, %rsi
	movl	$4, %edx
	movq	184(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_107:
	movq	744(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_109
	shlq	$2, %rsi
	movl	$4, %edx
	movq	240(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_109:
	movq	768(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_111
	shlq	$2, %rsi
	movl	$4, %edx
	movq	232(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_111:
	movq	416(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_113
	shlq	$2, %rsi
	movl	$4, %edx
	movq	24(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_113:
	movq	440(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_115
	shlq	$2, %rsi
	movl	$4, %edx
	movq	688(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_115:
	movq	464(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_117
	shlq	$2, %rsi
	movl	$4, %edx
	movq	408(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB17_117:
	movq	640(%rsp), %rax
	cmpq	$3, %rax
	movabsq	$549755813920, %rsi
	vmovq	.LCPI17_4(%rip), %xmm1
	jne	.LBB17_4
	movq	632(%rsp), %rax
	cmpq	$3, %rax
	jne	.LBB17_3
	movq	984(%rsp), %rax
	cmpq	%rax, 992(%rsp)
	leaq	1000(%rsp), %rdx
	jne	.LBB17_1
.LBB17_120:
	addq	$1224, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.LBB17_121:
	.cfi_def_cfa_offset 1280
	movq	%r15, %r12
	jmp	.LBB17_123
.LBB17_122:
	movq	%rax, %r12
.LBB17_123:
.Ltmp29:
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.19(%rip), %rdx
	movq	%r12, %rdi
	movq	8(%rsp), %rbx
	movq	(%rsp), %r12
	movq	176(%rsp), %r14
	callq	*_RNvNvNtCs8uuN1nrGffB_4core5slice20copy_from_slice_impl17len_mismatch_fail@GOTPCREL(%rip)
.Ltmp30:
	jmp	.LBB17_130
.LBB17_124:
.Ltmp21:
	leaq	272(%rsp), %rdi
	leaq	720(%rsp), %rsi
	callq	_RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_BM_ECsaU0XrY2Q1Mf_22flat_natural_cast_main
.Ltmp22:
	jmp	.LBB17_130
.LBB17_125:
	movl	$2, %eax
	jmp	.LBB17_127
.LBB17_126:
	movl	$1, %eax
.LBB17_127:
	movq	%rax, 224(%rsp)
.LBB17_128:
.Ltmp9:
	movl	$4, %edi
	movq	%rbx, %rsi
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.Ltmp10:
	jmp	.LBB17_130
.LBB17_129:
.Ltmp45:
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.10(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	movq	8(%rsp), %rbx
	movq	(%rsp), %r12
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp46:
.LBB17_130:
	ud2
.LBB17_131:
.Ltmp41:
	jmp	.LBB17_133
.LBB17_132:
.Ltmp44:
.LBB17_133:
	movq	%rax, %r15
	movq	8(%rsp), %rbx
	movq	(%rsp), %r12
	jmp	.LBB17_149
.LBB17_134:
.Ltmp26:
	movq	%rax, %r15
	movq	272(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_153
	jmp	.LBB17_174
.LBB17_135:
.Ltmp14:
	movq	%rax, %r15
	movq	416(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_162
	jmp	.LBB17_168
.LBB17_136:
.Ltmp11:
	movq	%rax, %r15
	leaq	48(%rsp), %rdi
	movq	224(%rsp), %rsi
	callq	_RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtB4_5array5GuardINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEEECsaU0XrY2Q1Mf_22flat_natural_cast_main
	movq	%r15, %rdi
	callq	_Unwind_Resume@PLT
.LBB17_137:
.Ltmp17:
	movq	%rax, %r15
	movq	720(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_165
	jmp	.LBB17_159
.LBB17_138:
.Ltmp20:
	jmp	.LBB17_142
.LBB17_139:
.Ltmp38:
	movq	%rax, %r15
	movq	8(%rsp), %rbx
	movq	(%rsp), %r12
	movq	176(%rsp), %r14
	jmp	.LBB17_149
.LBB17_140:
.Ltmp33:
	movq	%rax, %r15
	movq	8(%rsp), %rbx
	movq	(%rsp), %r12
	movq	%r14, %rbp
	movq	176(%rsp), %r14
	jmp	.LBB17_149
.LBB17_141:
.Ltmp23:
.LBB17_142:
	movq	%rax, %r15
	movq	272(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_145
	movq	296(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_146
.LBB17_144:
	movq	320(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_147
	jmp	.LBB17_158
.LBB17_145:
	shlq	$2, %rsi
	movl	$4, %edx
	movq	%rbp, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	296(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_144
.LBB17_146:
	shlq	$2, %rsi
	movl	$4, %edx
	movq	192(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	320(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_158
.LBB17_147:
	shlq	$2, %rsi
	movl	$4, %edx
	movq	544(%rsp), %rdi
	jmp	.LBB17_157
.LBB17_148:
.Ltmp47:
	movq	%rax, %r15
.LBB17_149:
	testq	%r14, %r14
	jne	.LBB17_171
	testq	%r12, %r12
	jne	.LBB17_172
.LBB17_151:
	testq	%rbx, %rbx
	jne	.LBB17_173
.LBB17_152:
	movq	272(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_174
.LBB17_153:
	movq	296(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_175
.LBB17_154:
	movq	320(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_176
.LBB17_155:
	movq	344(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_156
.LBB17_158:
	movq	720(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_165
.LBB17_159:
	movq	744(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_166
.LBB17_160:
	movq	768(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_167
.LBB17_161:
	movq	416(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_168
.LBB17_162:
	movq	440(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_169
.LBB17_163:
	movq	464(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB17_170
.LBB17_164:
	movq	%r15, %rdi
	callq	_Unwind_Resume@PLT
.LBB17_171:
	shlq	$2, %r14
	movl	$4, %edx
	movq	192(%rsp), %rdi
	movq	%r14, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	testq	%r12, %r12
	je	.LBB17_151
.LBB17_172:
	shlq	$2, %r12
	movl	$4, %edx
	movq	%rbp, %rdi
	movq	%r12, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	testq	%rbx, %rbx
	je	.LBB17_152
.LBB17_173:
	shlq	$2, %rbx
	movl	$4, %edx
	movq	248(%rsp), %rdi
	movq	%rbx, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	272(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_153
.LBB17_174:
	movq	280(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	296(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_154
.LBB17_175:
	movq	304(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	320(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_155
.LBB17_176:
	movq	328(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	344(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_158
.LBB17_156:
	movq	352(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
.LBB17_157:
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	720(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_159
.LBB17_165:
	shlq	$2, %rsi
	movl	$4, %edx
	movq	184(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	744(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_160
.LBB17_166:
	shlq	$2, %rsi
	movl	$4, %edx
	movq	240(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	768(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_161
.LBB17_167:
	shlq	$2, %rsi
	movl	$4, %edx
	movq	232(%rsp), %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	416(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_162
.LBB17_168:
	movq	424(%rsp), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	440(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_163
.LBB17_169:
	movq	448(%rsp), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	464(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_164
.LBB17_170:
	movq	472(%rsp), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	%r15, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end17:
	.size	_RNvCsaU0XrY2Q1Mf_22flat_natural_cast_main4main, .Lfunc_end17-_RNvCsaU0XrY2Q1Mf_22flat_natural_cast_main4main
	.cfi_endproc
	.section	.gcc_except_table._RNvCsaU0XrY2Q1Mf_22flat_natural_cast_main4main,"a",@progbits
	.p2align	2, 0x0
GCC_except_table17:
.Lexception3:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end3-.Lcst_begin3
.Lcst_begin3:
	.uleb128 .Ltmp12-.Lfunc_begin3
	.uleb128 .Ltmp13-.Ltmp12
	.uleb128 .Ltmp14-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp15-.Lfunc_begin3
	.uleb128 .Ltmp16-.Ltmp15
	.uleb128 .Ltmp17-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp18-.Lfunc_begin3
	.uleb128 .Ltmp19-.Ltmp18
	.uleb128 .Ltmp20-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp19-.Lfunc_begin3
	.uleb128 .Ltmp24-.Ltmp19
	.byte	0
	.byte	0
	.uleb128 .Ltmp24-.Lfunc_begin3
	.uleb128 .Ltmp25-.Ltmp24
	.uleb128 .Ltmp26-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp27-.Lfunc_begin3
	.uleb128 .Ltmp28-.Ltmp27
	.uleb128 .Ltmp38-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp28-.Lfunc_begin3
	.uleb128 .Ltmp31-.Ltmp28
	.byte	0
	.byte	0
	.uleb128 .Ltmp31-.Lfunc_begin3
	.uleb128 .Ltmp32-.Ltmp31
	.uleb128 .Ltmp33-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp34-.Lfunc_begin3
	.uleb128 .Ltmp37-.Ltmp34
	.uleb128 .Ltmp38-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp39-.Lfunc_begin3
	.uleb128 .Ltmp40-.Ltmp39
	.uleb128 .Ltmp41-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp42-.Lfunc_begin3
	.uleb128 .Ltmp43-.Ltmp42
	.uleb128 .Ltmp44-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp29-.Lfunc_begin3
	.uleb128 .Ltmp30-.Ltmp29
	.uleb128 .Ltmp47-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp21-.Lfunc_begin3
	.uleb128 .Ltmp22-.Ltmp21
	.uleb128 .Ltmp23-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp9-.Lfunc_begin3
	.uleb128 .Ltmp10-.Ltmp9
	.uleb128 .Ltmp11-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp45-.Lfunc_begin3
	.uleb128 .Ltmp46-.Ltmp45
	.uleb128 .Ltmp47-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp46-.Lfunc_begin3
	.uleb128 .Lfunc_end17-.Ltmp46
	.byte	0
	.byte	0
.Lcst_end3:
	.p2align	2, 0x0

	.section	.text.unlikely._RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end18, nop
	.type	_RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	subq	$24, %rsp
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	%rdi, %rbx
	movq	(%rdi), %rsi
	leaq	(%rsi,%rsi), %rax
	cmpq	$5, %rax
	movl	$4, %r14d
	cmovaeq	%rax, %r14
	movq	8(%rdi), %rdx
	movq	%rsp, %rdi
	movl	$8, %r8d
	movl	$8, %r9d
	movq	%r14, %rcx
	callq	_RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsaU0XrY2Q1Mf_22flat_natural_cast_main
	cmpl	$1, (%rsp)
	je	.LBB18_2
	movq	8(%rsp), %rax
	movq	%rax, 8(%rbx)
	movq	%r14, (%rbx)
	addq	$24, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.LBB18_2:
	.cfi_def_cfa_offset 48
	movq	8(%rsp), %rdi
	movq	16(%rsp), %rsi
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.Lfunc_end18:
	.size	_RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end18-_RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text.unlikely._RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end19, nop
	.type	_RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r12
	.cfi_def_cfa_offset 32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	pushq	%rax
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -40
	.cfi_offset %r12, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	%r8, %r15
	movq	%rdx, %r8
	movq	%rdi, %rbx
	movq	%r9, %rax
	mulq	%rcx
	movq	%rax, %r14
	seto	%al
	movabsq	$-9223372036854775808, %rcx
	subq	%r15, %rcx
	cmpq	%rcx, %r14
	seta	%cl
	orb	%al, %cl
	movl	$1, %r12d
	je	.LBB19_2
	movl	$8, %eax
	xorl	%r14d, %r14d
	jmp	.LBB19_10
.LBB19_2:
	testq	%rsi, %rsi
	je	.LBB19_4
	imulq	%rsi, %r9
	movq	%r8, %rdi
	movq	%r9, %rsi
	movq	%r15, %rdx
	movq	%r14, %rcx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_realloc@GOTPCREL(%rip)
	testq	%rax, %rax
	jne	.LBB19_6
	jmp	.LBB19_9
.LBB19_4:
	testq	%r14, %r14
	je	.LBB19_5
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movq	%r14, %rdi
	movq	%r15, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	jne	.LBB19_6
.LBB19_9:
	movq	%r15, 8(%rbx)
	movl	$16, %eax
	jmp	.LBB19_10
.LBB19_5:
	movq	%r15, %rax
.LBB19_6:
	movq	%rax, 8(%rbx)
	movl	$16, %eax
	xorl	%r12d, %r12d
.LBB19_10:
	movq	%r14, (%rbx,%rax)
	movq	%r12, (%rbx)
	addq	$8, %rsp
	.cfi_def_cfa_offset 40
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r12
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end19:
	.size	_RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end19-_RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI20_0:
	.long	0x3fb374bc
.LCPI20_1:
	.long	0xbeb031cf
.LCPI20_2:
	.long	0xbf36d1e1
.LCPI20_3:
	.long	0x3fe2d0e5
.LCPI20_4:
	.long	0x4f000000
	.section	.text._RNvNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7control18inverse_and_finish,"ax",@progbits
	.prefalign	4, .Lfunc_end20, nop
	.type	_RNvNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7control18inverse_and_finish,@function
_RNvNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7control18inverse_and_finish:
	.cfi_startproc
	movq	8(%rdi), %rcx
	movl	(%rdx), %eax
	movl	4(%rdx), %r9d
	movl	8(%rdx), %r8d
	movl	12(%rdx), %r10d
	subl	%eax, %r9d
	vmovd	%r9d, %xmm0
	movl	%r8d, %r9d
	subl	%eax, %r9d
	vmovd	%r9d, %xmm1
	movl	16(%rdx), %r11d
	vmovd	%eax, %xmm2
	movl	20(%rdx), %r9d
	subl	%r10d, %r11d
	vmovd	%r11d, %xmm3
	movl	%r9d, %eax
	subl	%r10d, %eax
	vmovd	%eax, %xmm4
	movl	24(%rdx), %r11d
	vmovd	%r10d, %xmm5
	movl	28(%rdx), %eax
	movl	32(%rdx), %r10d
	subl	%r11d, %eax
	vmovd	%eax, %xmm6
	movl	%r10d, %eax
	subl	%r11d, %eax
	vmovd	%eax, %xmm7
	movq	24(%rdi), %rax
	vmovd	%r11d, %xmm8
	movq	40(%rdi), %rdx
	shrq	$3, %rcx
	shrq	$3, %rax
	shrq	$3, %rdx
	cmpq	%rax, %rcx
	cmovbq	%rcx, %rax
	movq	(%rdi), %rcx
	cmpq	%rdx, %rax
	cmovaeq	%rdx, %rax
	movq	16(%rdi), %rdx
	movq	32(%rdi), %rdi
	vpbroadcastd	%xmm0, %ymm0
	vpbroadcastd	%xmm1, %ymm1
	vpbroadcastd	%xmm2, %ymm2
	vpbroadcastd	%xmm3, %ymm3
	vpbroadcastd	%xmm4, %ymm4
	vpbroadcastd	%xmm5, %ymm5
	vpbroadcastd	%xmm6, %ymm6
	vpbroadcastd	%xmm7, %ymm7
	vpbroadcastd	%xmm8, %ymm8
	testl	%esi, %esi
	je	.LBB20_1
	testq	%rax, %rax
	je	.LBB20_7
	xorl	%esi, %esi
	movq	%rax, %r8
	.p2align	4
.LBB20_6:
	vmovdqu	(%rcx,%rsi), %ymm9
	vmovdqu	(%rdx,%rsi), %ymm10
	vmovdqu	(%rdi,%rsi), %ymm11
	vpaddd	%ymm10, %ymm11, %ymm12
	vpsrad	$2, %ymm12, %ymm12
	vpsubd	%ymm12, %ymm9, %ymm9
	vpaddd	%ymm11, %ymm9, %ymm11
	vpmaxsd	%ymm0, %ymm11, %ymm11
	vpminsd	%ymm1, %ymm11, %ymm11
	vpaddd	%ymm2, %ymm11, %ymm11
	vmovdqu	%ymm11, (%rcx,%rsi)
	vpmaxsd	%ymm3, %ymm9, %ymm11
	vpminsd	%ymm4, %ymm11, %ymm11
	vpaddd	%ymm5, %ymm11, %ymm11
	vmovdqu	%ymm11, (%rdx,%rsi)
	vpaddd	%ymm10, %ymm9, %ymm9
	vpmaxsd	%ymm6, %ymm9, %ymm9
	vpminsd	%ymm7, %ymm9, %ymm9
	vpaddd	%ymm8, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdi,%rsi)
	addq	$32, %rsi
	decq	%r8
	jne	.LBB20_6
	jmp	.LBB20_7
.LBB20_1:
	testq	%rax, %rax
	je	.LBB20_7
	vmovd	%r8d, %xmm9
	vpbroadcastd	%xmm9, %ymm9
	vmovdqu	%ymm9, -40(%rsp)
	vmovd	%r9d, %xmm10
	vpbroadcastd	%xmm10, %ymm9
	vmovdqu	%ymm9, -72(%rsp)
	vmovd	%r10d, %xmm11
	vpbroadcastd	%xmm11, %ymm9
	vmovdqu	%ymm9, -104(%rsp)
	xorl	%esi, %esi
	vbroadcastss	.LCPI20_4(%rip), %ymm12
	movq	%rax, %r8
	.p2align	4
.LBB20_3:
	vmovups	(%rcx,%rsi), %ymm13
	vmovups	(%rdx,%rsi), %ymm14
	vmovups	(%rdi,%rsi), %ymm15
	vbroadcastss	.LCPI20_0(%rip), %ymm9
	vmulps	%ymm9, %ymm15, %ymm9
	vaddps	%ymm9, %ymm13, %ymm9
	vbroadcastss	.LCPI20_1(%rip), %ymm10
	vmulps	%ymm10, %ymm14, %ymm10
	vaddps	%ymm10, %ymm13, %ymm10
	vbroadcastss	.LCPI20_2(%rip), %ymm11
	vmulps	%ymm11, %ymm15, %ymm15
	vaddps	%ymm15, %ymm10, %ymm10
	vroundps	$8, %ymm9, %ymm15
	vcvttps2dq	%ymm15, %ymm15
	vcmpge_oqps	%ymm12, %ymm9, %ymm9
	vpmaxsd	%ymm0, %ymm15, %ymm15
	vpminsd	%ymm1, %ymm15, %ymm15
	vpaddd	%ymm2, %ymm15, %ymm15
	vpblendvb	%ymm9, -40(%rsp), %ymm15, %ymm9
	vbroadcastss	.LCPI20_3(%rip), %ymm11
	vmulps	%ymm11, %ymm14, %ymm14
	vmovdqu	%ymm9, (%rcx,%rsi)
	vroundps	$8, %ymm10, %ymm9
	vaddps	%ymm14, %ymm13, %ymm13
	vcvttps2dq	%ymm9, %ymm9
	vcmpge_oqps	%ymm12, %ymm10, %ymm10
	vpmaxsd	%ymm3, %ymm9, %ymm9
	vpminsd	%ymm4, %ymm9, %ymm9
	vpaddd	%ymm5, %ymm9, %ymm9
	vpblendvb	%ymm10, -72(%rsp), %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdx,%rsi)
	vroundps	$8, %ymm13, %ymm9
	vcvttps2dq	%ymm9, %ymm9
	vcmpge_oqps	%ymm12, %ymm13, %ymm10
	vpmaxsd	%ymm6, %ymm9, %ymm9
	vpminsd	%ymm7, %ymm9, %ymm9
	vpaddd	%ymm8, %ymm9, %ymm9
	vpblendvb	%ymm10, -104(%rsp), %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdi,%rsi)
	addq	$32, %rsi
	decq	%r8
	jne	.LBB20_3
.LBB20_7:
	shlq	$3, %rax
	vzeroupper
	retq
.Lfunc_end20:
	.size	_RNvNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7control18inverse_and_finish, .Lfunc_end20-_RNvNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7control18inverse_and_finish
	.cfi_endproc

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI21_0:
	.long	3
.LCPI21_1:
	.long	0x3fb374bc
.LCPI21_2:
	.long	0xbeb031cf
.LCPI21_3:
	.long	0xbf36d1e1
.LCPI21_4:
	.long	0x3fe2d0e5
.LCPI21_6:
	.long	1
.LCPI21_7:
	.long	0x4b400000
	.section	.rodata.cst8,"aM",@progbits,8
	.p2align	3, 0x0
.LCPI21_5:
	.quad	0x4338000000000000
	.section	.text._RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel18inverse_and_finish,"ax",@progbits
	.prefalign	4, .Lfunc_end21, nop
	.type	_RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel18inverse_and_finish,@function
_RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel18inverse_and_finish:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$88, %rsp
	.cfi_def_cfa_offset 144
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r10
	movl	%esi, %r11d
	movq	%rdi, %rax
	movq	(%rdi), %rdi
	movq	8(%rax), %rsi
	movq	16(%rax), %rdx
	movq	24(%rax), %rcx
	movq	32(%rax), %r8
	movq	40(%rax), %r9
	movq	%rsi, %rbx
	shrq	$3, %rbx
	movq	%rcx, %rax
	shrq	$3, %rax
	movq	%r9, %r15
	shrq	$3, %r15
	testl	%r11d, %r11d
	movq	%rbx, 40(%rsp)
	movq	%r15, 32(%rsp)
	je	.LBB21_1
	cmpq	%rax, %rbx
	movq	%rax, %rcx
	cmovbq	%rbx, %rcx
	cmpq	%r15, %rcx
	cmovaeq	%r15, %rcx
	testq	%rcx, %rcx
	je	.LBB21_31
	movl	(%r10), %ebx
	movl	4(%r10), %r14d
	movl	8(%r10), %ebp
	subl	%ebx, %r14d
	subl	%ebx, %ebp
	cmpl	%ebp, %r14d
	jg	.LBB21_41
	movl	12(%r10), %r15d
	movl	16(%r10), %esi
	movl	20(%r10), %r9d
	subl	%r15d, %esi
	subl	%r15d, %r9d
	movl	24(%r10), %r12d
	movl	28(%r10), %r11d
	movl	32(%r10), %r10d
	subl	%r12d, %r11d
	subl	%r12d, %r10d
	vmovd	%r14d, %xmm0
	vpbroadcastd	%xmm0, %ymm0
	vmovd	%ebp, %xmm1
	vpbroadcastd	%xmm1, %ymm1
	vmovd	%ebx, %xmm2
	vpbroadcastd	%xmm2, %ymm2
	vmovd	%esi, %xmm3
	vpbroadcastd	%xmm3, %ymm3
	vmovd	%r9d, %xmm4
	vpbroadcastd	%xmm4, %ymm4
	vmovd	%r15d, %xmm5
	vpbroadcastd	%xmm5, %ymm5
	vmovd	%r11d, %xmm6
	vpbroadcastd	%xmm6, %ymm6
	vmovd	%r10d, %xmm7
	vpbroadcastd	%xmm7, %ymm7
	vmovd	%r12d, %xmm8
	vpbroadcastd	%xmm8, %ymm8
	xorl	%ebx, %ebx
	.p2align	4
.LBB21_20:
	vmovdqu	(%rdi,%rbx), %ymm10
	vmovdqu	(%rdx,%rbx), %ymm9
	vmovdqu	(%r8,%rbx), %ymm11
	vpaddd	%ymm9, %ymm11, %ymm12
	vpsrad	$2, %ymm12, %ymm12
	vpsubd	%ymm12, %ymm10, %ymm10
	vpaddd	%ymm11, %ymm10, %ymm11
	vpcmpgtd	%ymm11, %ymm0, %ymm12
	vpminsd	%ymm1, %ymm11, %ymm11
	vblendvps	%ymm12, %ymm0, %ymm11, %ymm11
	vpaddd	%ymm2, %ymm11, %ymm11
	vmovdqu	%ymm11, (%rdi,%rbx)
	cmpl	%r9d, %esi
	jg	.LBB21_21
	vpcmpgtd	%ymm10, %ymm3, %ymm11
	vpminsd	%ymm4, %ymm10, %ymm12
	vblendvps	%ymm11, %ymm3, %ymm12, %ymm11
	vpaddd	%ymm5, %ymm11, %ymm11
	vmovdqu	%ymm11, (%rdx,%rbx)
	cmpl	%r10d, %r11d
	jg	.LBB21_44
	vpaddd	%ymm9, %ymm10, %ymm9
	vpcmpgtd	%ymm9, %ymm6, %ymm10
	vpminsd	%ymm7, %ymm9, %ymm9
	vblendvps	%ymm10, %ymm6, %ymm9, %ymm9
	vpaddd	%ymm8, %ymm9, %ymm9
	vmovdqu	%ymm9, (%r8,%rbx)
	addq	$32, %rbx
	decq	%rcx
	jne	.LBB21_20
	jmp	.LBB21_31
.LBB21_1:
	movl	(%r10), %r12d
	movl	4(%r10), %r11d
	vmovdqu	12(%r10), %xmm0
	movq	%r11, 80(%rsp)
	movl	%r11d, %r13d
	subl	%r12d, %r13d
	cmpl	$-4194304, %r13d
	jl	.LBB21_2
	movl	8(%r10), %r11d
	subl	%r12d, %r11d
	cmpl	$4194304, %r11d
	jg	.LBB21_2
	vmovd	%r13d, %xmm1
	vpinsrd	$1, %r11d, %xmm1, %xmm1
	vcvtdq2ps	%xmm1, %xmm1
	movb	$1, %r11b
	movl	%r11d, 4(%rsp)
	jmp	.LBB21_5
.LBB21_2:
	movl	$0, 4(%rsp)
.LBB21_5:
	movl	28(%r10), %ebx
	vmovd	%xmm0, %r11d
	vpextrd	$1, %xmm0, %ebp
	subl	%r11d, %ebp
	vpextrd	$2, %xmm0, %r14d
	cmpl	$-4194304, %ebp
	movl	%r12d, 20(%rsp)
	movq	%r14, 24(%rsp)
	jl	.LBB21_6
	subl	%r11d, %r14d
	cmpl	$4194304, %r14d
	jg	.LBB21_6
	vmovd	%ebp, %xmm2
	vpinsrd	$1, %r14d, %xmm2, %xmm2
	vcvtdq2ps	%xmm2, %xmm2
	jmp	.LBB21_9
.LBB21_6:
	movl	$0, 4(%rsp)
.LBB21_9:
	vmovq	(%r10), %xmm3
	movl	32(%r10), %r15d
	vpextrd	$3, %xmm0, %r12d
	movl	%ebx, %r14d
	movl	%r12d, 8(%rsp)
	subl	%r12d, %r14d
	cmpl	$-4194304, %r14d
	jl	.LBB21_12
	movl	%r15d, %r12d
	subl	8(%rsp), %r12d
	cmpl	$4194304, %r12d
	jg	.LBB21_12
	cmpb	$0, 4(%rsp)
	je	.LBB21_12
	movq	40(%rsp), %rsi
	cmpq	%rax, %rsi
	movq	%rax, %rcx
	cmovbq	%rsi, %rcx
	movq	32(%rsp), %rsi
	cmpq	%rsi, %rcx
	cmovaeq	%rsi, %rcx
	testq	%rcx, %rcx
	je	.LBB21_31
	vcvtsi2ss	%r12d, %xmm15, %xmm0
	vcvtsi2ss	%r14d, %xmm15, %xmm4
	movl	20(%rsp), %r15d
	addl	$-1262485504, %r15d
	addl	$-1262485504, %r11d
	movl	%r11d, %r12d
	addl	$-1262485504, 8(%rsp)
	cmpq	$7, %rcx
	ja	.LBB21_26
.LBB21_24:
	vbroadcastss	%xmm1, %ymm3
	vbroadcastss	.LCPI21_6(%rip), %ymm7
	vpermps	%ymm1, %ymm7, %ymm1
	vmovd	%r15d, %xmm5
	vpbroadcastd	%xmm5, %ymm5
	vbroadcastss	%xmm2, %ymm6
	vpermps	%ymm2, %ymm7, %ymm2
	vmovd	%r12d, %xmm7
	vpbroadcastd	%xmm7, %ymm7
	vbroadcastss	%xmm4, %ymm4
	vbroadcastss	%xmm0, %ymm0
	vmovd	8(%rsp), %xmm8
	vpbroadcastd	%xmm8, %ymm8
	xorl	%esi, %esi
	vbroadcastss	.LCPI21_7(%rip), %ymm10
	vbroadcastss	.LCPI21_4(%rip), %ymm13
	.p2align	4
.LBB21_25:
	vmovups	(%rdi,%rsi), %ymm14
	vmovups	(%r8,%rsi), %ymm15
	vmovups	(%rdx,%rsi), %ymm9
	vbroadcastss	.LCPI21_2(%rip), %ymm11
	vmulps	%ymm11, %ymm9, %ymm11
	vaddps	%ymm11, %ymm14, %ymm11
	vbroadcastss	.LCPI21_3(%rip), %ymm12
	vmulps	%ymm12, %ymm15, %ymm12
	vaddps	%ymm12, %ymm11, %ymm11
	vbroadcastss	.LCPI21_1(%rip), %ymm12
	vmulps	%ymm12, %ymm15, %ymm12
	vaddps	%ymm12, %ymm14, %ymm12
	vmaxps	%ymm3, %ymm12, %ymm12
	vminps	%ymm1, %ymm12, %ymm12
	vaddps	%ymm10, %ymm12, %ymm12
	vpaddd	%ymm5, %ymm12, %ymm12
	vmovdqu	%ymm12, (%rdi,%rsi)
	vmaxps	%ymm6, %ymm11, %ymm11
	vminps	%ymm2, %ymm11, %ymm11
	vaddps	%ymm10, %ymm11, %ymm11
	vpaddd	%ymm7, %ymm11, %ymm11
	vmovdqu	%ymm11, (%rdx,%rsi)
	vmulps	%ymm13, %ymm9, %ymm9
	vaddps	%ymm9, %ymm14, %ymm9
	vmaxps	%ymm4, %ymm9, %ymm9
	vminps	%ymm0, %ymm9, %ymm9
	vaddps	%ymm10, %ymm9, %ymm9
	vpaddd	%ymm9, %ymm8, %ymm9
	vmovdqu	%ymm9, (%r8,%rsi)
	addq	$32, %rsi
	decq	%rcx
	jne	.LBB21_25
	jmp	.LBB21_31
.LBB21_12:
	movl	%r11d, 4(%rsp)
	vpshufd	$36, %xmm3, %xmm1
	vpinsrd	$2, 8(%r10), %xmm1, %xmm1
	xorl	80(%rsp), %ebx
	movq	24(%rsp), %r11
	movl	%r11d, %r12d
	xorl	%r15d, %r12d
	vpcmpeqd	%xmm0, %xmm1, %xmm1
	vmovmskps	%xmm1, %r11d
	xorl	$15, %r11d
	orl	%ebx, %r11d
	orl	%r12d, %r11d
	jne	.LBB21_13
	movq	%rax, %rbx
	andq	$-8, %rsi
	andq	$-8, %rcx
	andq	$-8, %r9
	cmpl	$0, 20(%rsp)
	je	.LBB21_37
	subq	$8, %rsp
	.cfi_adjust_cfa_offset 8
	pushq	%r10
	.cfi_adjust_cfa_offset 8
	vzeroupper
	callq	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb0_EB6_
	jmp	.LBB21_40
.LBB21_13:
	.cfi_def_cfa_offset 144
	movq	40(%rsp), %rsi
	cmpq	%rax, %rsi
	movq	%rax, %rcx
	cmovbq	%rsi, %rcx
	movq	32(%rsp), %rsi
	cmpq	%rsi, %rcx
	cmovaeq	%rsi, %rcx
	testq	%rcx, %rcx
	movl	4(%rsp), %r9d
	je	.LBB21_31
	vcvtsi2sd	%r13d, %xmm15, %xmm2
	movl	8(%r10), %esi
	subl	20(%rsp), %esi
	vcvtsi2sd	%esi, %xmm15, %xmm4
	vcvtsi2sd	%ebp, %xmm15, %xmm5
	movq	24(%rsp), %rsi
	subl	%r9d, %esi
	vcvtsi2sd	%esi, %xmm15, %xmm6
	vcvtsi2sd	%r14d, %xmm15, %xmm7
	subl	8(%rsp), %r15d
	vcvtsi2sd	%r15d, %xmm15, %xmm8
	cmpq	$7, %rcx
	ja	.LBB21_32
.LBB21_15:
	vbroadcastsd	%xmm2, %ymm1
	vbroadcastsd	%xmm4, %ymm2
	vpbroadcastd	%xmm3, %ymm3
	vbroadcastsd	%xmm5, %ymm4
	vbroadcastsd	%xmm6, %ymm5
	vpbroadcastd	%xmm0, %ymm6
	vbroadcastsd	%xmm7, %ymm7
	vbroadcastsd	%xmm8, %ymm8
	vpbroadcastd	.LCPI21_0(%rip), %ymm9
	vpermd	%ymm0, %ymm9, %ymm0
	xorl	%esi, %esi
	vbroadcastsd	.LCPI21_5(%rip), %ymm10
	vbroadcastss	.LCPI21_3(%rip), %ymm13
	.p2align	4
.LBB21_16:
	vmovups	(%rdi,%rsi), %ymm14
	vmovups	(%r8,%rsi), %ymm15
	vbroadcastss	.LCPI21_1(%rip), %ymm9
	vmulps	%ymm9, %ymm15, %ymm9
	vaddps	%ymm9, %ymm14, %ymm9
	vcvtps2pd	%xmm9, %ymm11
	vextractf128	$1, %ymm9, %xmm9
	vcvtps2pd	%xmm9, %ymm9
	vmaxpd	%ymm1, %ymm9, %ymm9
	vmaxpd	%ymm1, %ymm11, %ymm11
	vminpd	%ymm2, %ymm11, %ymm11
	vminpd	%ymm2, %ymm9, %ymm9
	vaddpd	%ymm10, %ymm9, %ymm9
	vaddpd	%ymm10, %ymm11, %ymm11
	vshufps	$136, %ymm9, %ymm11, %ymm9
	vmovups	(%rdx,%rsi), %ymm11
	vbroadcastss	.LCPI21_2(%rip), %ymm12
	vmulps	%ymm12, %ymm11, %ymm12
	vaddps	%ymm12, %ymm14, %ymm12
	vmulps	%ymm13, %ymm15, %ymm15
	vaddps	%ymm15, %ymm12, %ymm12
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm3, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdi,%rsi)
	vcvtps2pd	%xmm12, %ymm9
	vextractf128	$1, %ymm12, %xmm12
	vcvtps2pd	%xmm12, %ymm12
	vmaxpd	%ymm4, %ymm12, %ymm12
	vmaxpd	%ymm4, %ymm9, %ymm9
	vminpd	%ymm5, %ymm9, %ymm9
	vminpd	%ymm5, %ymm12, %ymm12
	vaddpd	%ymm10, %ymm12, %ymm12
	vaddpd	%ymm10, %ymm9, %ymm9
	vshufps	$136, %ymm12, %ymm9, %ymm9
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm6, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdx,%rsi)
	vbroadcastss	.LCPI21_4(%rip), %ymm9
	vmulps	%ymm9, %ymm11, %ymm9
	vaddps	%ymm9, %ymm14, %ymm9
	vcvtps2pd	%xmm9, %ymm11
	vextractf128	$1, %ymm9, %xmm9
	vcvtps2pd	%xmm9, %ymm9
	vmaxpd	%ymm7, %ymm9, %ymm9
	vmaxpd	%ymm7, %ymm11, %ymm11
	vminpd	%ymm8, %ymm11, %ymm11
	vminpd	%ymm8, %ymm9, %ymm9
	vaddpd	%ymm10, %ymm9, %ymm9
	vaddpd	%ymm10, %ymm11, %ymm11
	vshufps	$136, %ymm9, %ymm11, %ymm9
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm0, %ymm9, %ymm9
	vmovdqu	%ymm9, (%r8,%rsi)
	addq	$32, %rsi
	decq	%rcx
	jne	.LBB21_16
	jmp	.LBB21_31
.LBB21_37:
	pushq	24(%rsp)
	.cfi_adjust_cfa_offset 8
	pushq	88(%rsp)
	.cfi_adjust_cfa_offset 8
	vzeroupper
	callq	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel29inverse_and_finish_typed_rowsKb1_EB6_
.LBB21_40:
	addq	$16, %rsp
	.cfi_adjust_cfa_offset -16
	movq	%rbx, %rax
.LBB21_31:
	movq	40(%rsp), %rcx
	cmpq	%rax, %rcx
	cmovbq	%rcx, %rax
	movq	32(%rsp), %rcx
	cmpq	%rcx, %rax
	cmovaeq	%rcx, %rax
	shlq	$3, %rax
	addq	$88, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.LBB21_32:
	.cfi_def_cfa_offset 144
	movq	%rcx, %rsi
	shlq	$5, %rsi
	leaq	(%rdi,%rsi), %r10
	leaq	(%rdx,%rsi), %rbx
	addq	%r8, %rsi
	cmpq	%rbx, %rdi
	setb	%bpl
	cmpq	%r10, %rdx
	setb	%r14b
	cmpq	%rsi, %rdi
	setb	%r9b
	cmpq	%r10, %r8
	setb	%r11b
	cmpq	%rsi, %rdx
	setb	%sil
	cmpq	%rbx, %r8
	setb	%r10b
	testb	%r14b, %bpl
	jne	.LBB21_15
	andb	%r11b, %r9b
	jne	.LBB21_15
	andb	%r10b, %sil
	jne	.LBB21_15
	vpbroadcastd	%xmm3, %ymm1
	vbroadcastsd	%xmm2, %ymm2
	vbroadcastsd	%xmm4, %ymm3
	vpbroadcastd	%xmm0, %ymm4
	vbroadcastsd	%xmm5, %ymm5
	vbroadcastsd	%xmm6, %ymm6
	vpbroadcastd	.LCPI21_0(%rip), %ymm9
	vpermd	%ymm0, %ymm9, %ymm0
	vbroadcastsd	%xmm7, %ymm7
	vbroadcastsd	%xmm8, %ymm8
	xorl	%esi, %esi
	vbroadcastss	.LCPI21_4(%rip), %ymm12
	vbroadcastsd	.LCPI21_5(%rip), %ymm13
	.p2align	4
.LBB21_36:
	vmovups	(%rdi,%rsi), %ymm14
	vmovups	(%rdx,%rsi), %ymm15
	vmovups	(%r8,%rsi), %ymm9
	vbroadcastss	.LCPI21_2(%rip), %ymm10
	vmulps	%ymm10, %ymm15, %ymm10
	vaddps	%ymm10, %ymm14, %ymm10
	vbroadcastss	.LCPI21_3(%rip), %ymm11
	vmulps	%ymm11, %ymm9, %ymm11
	vaddps	%ymm11, %ymm10, %ymm10
	vbroadcastss	.LCPI21_1(%rip), %ymm11
	vmulps	%ymm11, %ymm9, %ymm9
	vaddps	%ymm9, %ymm14, %ymm9
	vmulps	%ymm12, %ymm15, %ymm11
	vaddps	%ymm11, %ymm14, %ymm11
	vcvtps2pd	%xmm9, %ymm14
	vextractf128	$1, %ymm9, %xmm9
	vcvtps2pd	%xmm9, %ymm9
	vmaxpd	%ymm2, %ymm9, %ymm9
	vmaxpd	%ymm2, %ymm14, %ymm14
	vminpd	%ymm3, %ymm14, %ymm14
	vminpd	%ymm3, %ymm9, %ymm9
	vaddpd	%ymm13, %ymm9, %ymm9
	vaddpd	%ymm13, %ymm14, %ymm14
	vshufps	$136, %ymm9, %ymm14, %ymm9
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm1, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdi,%rsi)
	vcvtps2pd	%xmm10, %ymm9
	vextractf128	$1, %ymm10, %xmm10
	vcvtps2pd	%xmm10, %ymm10
	vmaxpd	%ymm5, %ymm10, %ymm10
	vmaxpd	%ymm5, %ymm9, %ymm9
	vminpd	%ymm6, %ymm9, %ymm9
	vminpd	%ymm6, %ymm10, %ymm10
	vaddpd	%ymm13, %ymm10, %ymm10
	vaddpd	%ymm13, %ymm9, %ymm9
	vshufps	$136, %ymm10, %ymm9, %ymm9
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm4, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdx,%rsi)
	vcvtps2pd	%xmm11, %ymm9
	vextractf128	$1, %ymm11, %xmm10
	vcvtps2pd	%xmm10, %ymm10
	vmaxpd	%ymm7, %ymm10, %ymm10
	vmaxpd	%ymm7, %ymm9, %ymm9
	vminpd	%ymm8, %ymm9, %ymm9
	vminpd	%ymm8, %ymm10, %ymm10
	vaddpd	%ymm13, %ymm10, %ymm10
	vaddpd	%ymm13, %ymm9, %ymm9
	vshufps	$136, %ymm10, %ymm9, %ymm9
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm0, %ymm9, %ymm9
	vmovdqu	%ymm9, (%r8,%rsi)
	addq	$32, %rsi
	decq	%rcx
	jne	.LBB21_36
	jmp	.LBB21_31
.LBB21_26:
	movq	%rcx, %rsi
	shlq	$5, %rsi
	leaq	(%rdi,%rsi), %r10
	leaq	(%rdx,%rsi), %rbx
	addq	%r8, %rsi
	cmpq	%rbx, %rdi
	setb	%bpl
	cmpq	%r10, %rdx
	setb	%r14b
	cmpq	%rsi, %rdi
	setb	%r9b
	cmpq	%r10, %r8
	setb	%r11b
	cmpq	%rsi, %rdx
	setb	%sil
	cmpq	%rbx, %r8
	setb	%r10b
	testb	%r14b, %bpl
	jne	.LBB21_24
	andb	%r11b, %r9b
	jne	.LBB21_24
	andb	%r10b, %sil
	jne	.LBB21_24
	vmovd	%r15d, %xmm3
	vpbroadcastd	%xmm3, %ymm3
	vbroadcastss	%xmm1, %ymm5
	vbroadcastss	.LCPI21_6(%rip), %ymm8
	vpermps	%ymm1, %ymm8, %ymm1
	vmovd	%r12d, %xmm6
	vpbroadcastd	%xmm6, %ymm6
	vbroadcastss	%xmm2, %ymm7
	vpermps	%ymm2, %ymm8, %ymm2
	vmovd	8(%rsp), %xmm8
	vpbroadcastd	%xmm8, %ymm8
	vbroadcastss	%xmm4, %ymm4
	vbroadcastss	%xmm0, %ymm0
	xorl	%esi, %esi
	vbroadcastss	.LCPI21_4(%rip), %ymm12
	vbroadcastss	.LCPI21_7(%rip), %ymm13
	.p2align	4
.LBB21_30:
	vmovups	(%rdi,%rsi), %ymm14
	vmovups	(%rdx,%rsi), %ymm15
	vmovups	(%r8,%rsi), %ymm9
	vbroadcastss	.LCPI21_2(%rip), %ymm10
	vmulps	%ymm10, %ymm15, %ymm10
	vaddps	%ymm10, %ymm14, %ymm10
	vbroadcastss	.LCPI21_3(%rip), %ymm11
	vmulps	%ymm11, %ymm9, %ymm11
	vaddps	%ymm11, %ymm10, %ymm10
	vbroadcastss	.LCPI21_1(%rip), %ymm11
	vmulps	%ymm11, %ymm9, %ymm9
	vaddps	%ymm9, %ymm14, %ymm9
	vmulps	%ymm12, %ymm15, %ymm11
	vaddps	%ymm11, %ymm14, %ymm11
	vmaxps	%ymm5, %ymm9, %ymm9
	vminps	%ymm1, %ymm9, %ymm9
	vaddps	%ymm13, %ymm9, %ymm9
	vpaddd	%ymm3, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdi,%rsi)
	vmaxps	%ymm7, %ymm10, %ymm9
	vminps	%ymm2, %ymm9, %ymm9
	vaddps	%ymm13, %ymm9, %ymm9
	vpaddd	%ymm6, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdx,%rsi)
	vmaxps	%ymm4, %ymm11, %ymm9
	vminps	%ymm0, %ymm9, %ymm9
	vaddps	%ymm13, %ymm9, %ymm9
	vpaddd	%ymm9, %ymm8, %ymm9
	vmovdqu	%ymm9, (%r8,%rsi)
	addq	$32, %rsi
	decq	%rcx
	jne	.LBB21_30
	jmp	.LBB21_31
.LBB21_21:
	movl	%esi, 12(%rsp)
	movl	%r9d, 16(%rsp)
	jmp	.LBB21_42
.LBB21_44:
	movl	%r11d, 12(%rsp)
	movl	%r10d, 16(%rsp)
.LBB21_42:
	leaq	12(%rsp), %rax
	movq	%rax, 48(%rsp)
	leaq	_RNvXsQ_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_5Debug3fmt(%rip), %rax
	movq	%rax, 56(%rsp)
	leaq	16(%rsp), %rcx
	movq	%rcx, 64(%rsp)
	movq	%rax, 72(%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.22(%rip), %rdi
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.11(%rip), %rdx
	leaq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.LBB21_41:
	movl	%r14d, 12(%rsp)
	movl	%ebp, 16(%rsp)
	jmp	.LBB21_42
.Lfunc_end21:
	.size	_RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel18inverse_and_finish, .Lfunc_end21-_RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main4safe6kernel18inverse_and_finish
	.cfi_endproc

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI22_0:
	.long	3
.LCPI22_1:
	.long	0x3fb374bc
.LCPI22_2:
	.long	0xbeb031cf
.LCPI22_3:
	.long	0xbf36d1e1
.LCPI22_4:
	.long	0x3fe2d0e5
.LCPI22_6:
	.long	1
.LCPI22_7:
	.long	0x4b400000
	.section	.rodata.cst8,"aM",@progbits,8
	.p2align	3, 0x0
.LCPI22_5:
	.quad	0x4338000000000000
	.section	.text._RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel18inverse_and_finish,"ax",@progbits
	.prefalign	4, .Lfunc_end22, nop
	.type	_RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel18inverse_and_finish,@function
_RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel18inverse_and_finish:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$88, %rsp
	.cfi_def_cfa_offset 144
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r10
	movl	%esi, %r11d
	movq	%rdi, %rax
	movq	(%rdi), %rdi
	movq	8(%rax), %rsi
	movq	16(%rax), %rdx
	movq	24(%rax), %rcx
	movq	32(%rax), %r8
	movq	40(%rax), %r9
	movq	%rsi, %rbx
	shrq	$3, %rbx
	movq	%rcx, %rax
	shrq	$3, %rax
	movq	%r9, %r15
	shrq	$3, %r15
	testl	%r11d, %r11d
	movq	%rbx, 40(%rsp)
	movq	%r15, 32(%rsp)
	je	.LBB22_1
	cmpq	%rax, %rbx
	movq	%rax, %rcx
	cmovbq	%rbx, %rcx
	cmpq	%r15, %rcx
	cmovaeq	%r15, %rcx
	testq	%rcx, %rcx
	je	.LBB22_31
	movl	(%r10), %ebx
	movl	4(%r10), %r14d
	movl	8(%r10), %ebp
	subl	%ebx, %r14d
	subl	%ebx, %ebp
	cmpl	%ebp, %r14d
	jg	.LBB22_41
	movl	12(%r10), %r15d
	movl	16(%r10), %esi
	movl	20(%r10), %r9d
	subl	%r15d, %esi
	subl	%r15d, %r9d
	movl	24(%r10), %r12d
	movl	28(%r10), %r11d
	movl	32(%r10), %r10d
	subl	%r12d, %r11d
	subl	%r12d, %r10d
	vmovd	%r14d, %xmm0
	vpbroadcastd	%xmm0, %ymm0
	vmovd	%ebp, %xmm1
	vpbroadcastd	%xmm1, %ymm1
	vmovd	%ebx, %xmm2
	vpbroadcastd	%xmm2, %ymm2
	vmovd	%esi, %xmm3
	vpbroadcastd	%xmm3, %ymm3
	vmovd	%r9d, %xmm4
	vpbroadcastd	%xmm4, %ymm4
	vmovd	%r15d, %xmm5
	vpbroadcastd	%xmm5, %ymm5
	vmovd	%r11d, %xmm6
	vpbroadcastd	%xmm6, %ymm6
	vmovd	%r10d, %xmm7
	vpbroadcastd	%xmm7, %ymm7
	vmovd	%r12d, %xmm8
	vpbroadcastd	%xmm8, %ymm8
	xorl	%ebx, %ebx
	.p2align	4
.LBB22_20:
	vmovdqu	(%rdi,%rbx), %ymm10
	vmovdqu	(%rdx,%rbx), %ymm9
	vmovdqu	(%r8,%rbx), %ymm11
	vpaddd	%ymm9, %ymm11, %ymm12
	vpsrad	$2, %ymm12, %ymm12
	vpsubd	%ymm12, %ymm10, %ymm10
	vpaddd	%ymm11, %ymm10, %ymm11
	vpcmpgtd	%ymm11, %ymm0, %ymm12
	vpminsd	%ymm1, %ymm11, %ymm11
	vblendvps	%ymm12, %ymm0, %ymm11, %ymm11
	vpaddd	%ymm2, %ymm11, %ymm11
	vmovdqu	%ymm11, (%rdi,%rbx)
	cmpl	%r9d, %esi
	jg	.LBB22_21
	vpcmpgtd	%ymm10, %ymm3, %ymm11
	vpminsd	%ymm4, %ymm10, %ymm12
	vblendvps	%ymm11, %ymm3, %ymm12, %ymm11
	vpaddd	%ymm5, %ymm11, %ymm11
	vmovdqu	%ymm11, (%rdx,%rbx)
	cmpl	%r10d, %r11d
	jg	.LBB22_44
	vpaddd	%ymm9, %ymm10, %ymm9
	vpcmpgtd	%ymm9, %ymm6, %ymm10
	vpminsd	%ymm7, %ymm9, %ymm9
	vblendvps	%ymm10, %ymm6, %ymm9, %ymm9
	vpaddd	%ymm8, %ymm9, %ymm9
	vmovdqu	%ymm9, (%r8,%rbx)
	addq	$32, %rbx
	decq	%rcx
	jne	.LBB22_20
	jmp	.LBB22_31
.LBB22_1:
	movl	(%r10), %r12d
	movl	4(%r10), %r11d
	vmovdqu	12(%r10), %xmm0
	movq	%r11, 80(%rsp)
	movl	%r11d, %r13d
	subl	%r12d, %r13d
	cmpl	$-4194304, %r13d
	jl	.LBB22_2
	movl	8(%r10), %r11d
	subl	%r12d, %r11d
	cmpl	$4194304, %r11d
	jg	.LBB22_2
	vmovd	%r13d, %xmm1
	vpinsrd	$1, %r11d, %xmm1, %xmm1
	vcvtdq2ps	%xmm1, %xmm1
	movb	$1, %r11b
	movl	%r11d, 4(%rsp)
	jmp	.LBB22_5
.LBB22_2:
	movl	$0, 4(%rsp)
.LBB22_5:
	movl	28(%r10), %ebx
	vmovd	%xmm0, %r11d
	vpextrd	$1, %xmm0, %ebp
	subl	%r11d, %ebp
	vpextrd	$2, %xmm0, %r14d
	cmpl	$-4194304, %ebp
	movl	%r12d, 20(%rsp)
	movq	%r14, 24(%rsp)
	jl	.LBB22_6
	subl	%r11d, %r14d
	cmpl	$4194304, %r14d
	jg	.LBB22_6
	vmovd	%ebp, %xmm2
	vpinsrd	$1, %r14d, %xmm2, %xmm2
	vcvtdq2ps	%xmm2, %xmm2
	jmp	.LBB22_9
.LBB22_6:
	movl	$0, 4(%rsp)
.LBB22_9:
	vmovq	(%r10), %xmm3
	movl	32(%r10), %r15d
	vpextrd	$3, %xmm0, %r12d
	movl	%ebx, %r14d
	movl	%r12d, 8(%rsp)
	subl	%r12d, %r14d
	cmpl	$-4194304, %r14d
	jl	.LBB22_12
	movl	%r15d, %r12d
	subl	8(%rsp), %r12d
	cmpl	$4194304, %r12d
	jg	.LBB22_12
	cmpb	$0, 4(%rsp)
	je	.LBB22_12
	movq	40(%rsp), %rsi
	cmpq	%rax, %rsi
	movq	%rax, %rcx
	cmovbq	%rsi, %rcx
	movq	32(%rsp), %rsi
	cmpq	%rsi, %rcx
	cmovaeq	%rsi, %rcx
	testq	%rcx, %rcx
	je	.LBB22_31
	vcvtsi2ss	%r12d, %xmm15, %xmm0
	vcvtsi2ss	%r14d, %xmm15, %xmm4
	movl	20(%rsp), %r15d
	addl	$-1262485504, %r15d
	addl	$-1262485504, %r11d
	movl	%r11d, %r12d
	addl	$-1262485504, 8(%rsp)
	cmpq	$7, %rcx
	ja	.LBB22_26
.LBB22_24:
	vbroadcastss	%xmm1, %ymm3
	vbroadcastss	.LCPI22_6(%rip), %ymm7
	vpermps	%ymm1, %ymm7, %ymm1
	vmovd	%r15d, %xmm5
	vpbroadcastd	%xmm5, %ymm5
	vbroadcastss	%xmm2, %ymm6
	vpermps	%ymm2, %ymm7, %ymm2
	vmovd	%r12d, %xmm7
	vpbroadcastd	%xmm7, %ymm7
	vbroadcastss	%xmm4, %ymm4
	vbroadcastss	%xmm0, %ymm0
	vmovd	8(%rsp), %xmm8
	vpbroadcastd	%xmm8, %ymm8
	xorl	%esi, %esi
	vbroadcastss	.LCPI22_7(%rip), %ymm10
	vbroadcastss	.LCPI22_4(%rip), %ymm13
	.p2align	4
.LBB22_25:
	vmovups	(%rdi,%rsi), %ymm14
	vmovups	(%r8,%rsi), %ymm15
	vmovups	(%rdx,%rsi), %ymm9
	vbroadcastss	.LCPI22_2(%rip), %ymm11
	vmulps	%ymm11, %ymm9, %ymm11
	vaddps	%ymm11, %ymm14, %ymm11
	vbroadcastss	.LCPI22_3(%rip), %ymm12
	vmulps	%ymm12, %ymm15, %ymm12
	vaddps	%ymm12, %ymm11, %ymm11
	vbroadcastss	.LCPI22_1(%rip), %ymm12
	vmulps	%ymm12, %ymm15, %ymm12
	vaddps	%ymm12, %ymm14, %ymm12
	vmaxps	%ymm3, %ymm12, %ymm12
	vminps	%ymm1, %ymm12, %ymm12
	vaddps	%ymm10, %ymm12, %ymm12
	vpaddd	%ymm5, %ymm12, %ymm12
	vmovdqu	%ymm12, (%rdi,%rsi)
	vmaxps	%ymm6, %ymm11, %ymm11
	vminps	%ymm2, %ymm11, %ymm11
	vaddps	%ymm10, %ymm11, %ymm11
	vpaddd	%ymm7, %ymm11, %ymm11
	vmovdqu	%ymm11, (%rdx,%rsi)
	vmulps	%ymm13, %ymm9, %ymm9
	vaddps	%ymm9, %ymm14, %ymm9
	vmaxps	%ymm4, %ymm9, %ymm9
	vminps	%ymm0, %ymm9, %ymm9
	vaddps	%ymm10, %ymm9, %ymm9
	vpaddd	%ymm9, %ymm8, %ymm9
	vmovdqu	%ymm9, (%r8,%rsi)
	addq	$32, %rsi
	decq	%rcx
	jne	.LBB22_25
	jmp	.LBB22_31
.LBB22_12:
	movl	%r11d, 4(%rsp)
	vpshufd	$36, %xmm3, %xmm1
	vpinsrd	$2, 8(%r10), %xmm1, %xmm1
	xorl	80(%rsp), %ebx
	movq	24(%rsp), %r11
	movl	%r11d, %r12d
	xorl	%r15d, %r12d
	vpcmpeqd	%xmm0, %xmm1, %xmm1
	vmovmskps	%xmm1, %r11d
	xorl	$15, %r11d
	orl	%ebx, %r11d
	orl	%r12d, %r11d
	jne	.LBB22_13
	movq	%rax, %rbx
	andq	$-8, %rsi
	andq	$-8, %rcx
	andq	$-8, %r9
	cmpl	$0, 20(%rsp)
	je	.LBB22_37
	subq	$8, %rsp
	.cfi_adjust_cfa_offset 8
	pushq	%r10
	.cfi_adjust_cfa_offset 8
	vzeroupper
	callq	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb0_EB6_
	jmp	.LBB22_40
.LBB22_13:
	.cfi_def_cfa_offset 144
	movq	40(%rsp), %rsi
	cmpq	%rax, %rsi
	movq	%rax, %rcx
	cmovbq	%rsi, %rcx
	movq	32(%rsp), %rsi
	cmpq	%rsi, %rcx
	cmovaeq	%rsi, %rcx
	testq	%rcx, %rcx
	movl	4(%rsp), %r9d
	je	.LBB22_31
	vcvtsi2sd	%r13d, %xmm15, %xmm2
	movl	8(%r10), %esi
	subl	20(%rsp), %esi
	vcvtsi2sd	%esi, %xmm15, %xmm4
	vcvtsi2sd	%ebp, %xmm15, %xmm5
	movq	24(%rsp), %rsi
	subl	%r9d, %esi
	vcvtsi2sd	%esi, %xmm15, %xmm6
	vcvtsi2sd	%r14d, %xmm15, %xmm7
	subl	8(%rsp), %r15d
	vcvtsi2sd	%r15d, %xmm15, %xmm8
	cmpq	$7, %rcx
	ja	.LBB22_32
.LBB22_15:
	vbroadcastsd	%xmm2, %ymm1
	vbroadcastsd	%xmm4, %ymm2
	vpbroadcastd	%xmm3, %ymm3
	vbroadcastsd	%xmm5, %ymm4
	vbroadcastsd	%xmm6, %ymm5
	vpbroadcastd	%xmm0, %ymm6
	vbroadcastsd	%xmm7, %ymm7
	vbroadcastsd	%xmm8, %ymm8
	vpbroadcastd	.LCPI22_0(%rip), %ymm9
	vpermd	%ymm0, %ymm9, %ymm0
	xorl	%esi, %esi
	vbroadcastsd	.LCPI22_5(%rip), %ymm10
	vbroadcastss	.LCPI22_3(%rip), %ymm13
	.p2align	4
.LBB22_16:
	vmovups	(%rdi,%rsi), %ymm14
	vmovups	(%r8,%rsi), %ymm15
	vbroadcastss	.LCPI22_1(%rip), %ymm9
	vmulps	%ymm9, %ymm15, %ymm9
	vaddps	%ymm9, %ymm14, %ymm9
	vcvtps2pd	%xmm9, %ymm11
	vextractf128	$1, %ymm9, %xmm9
	vcvtps2pd	%xmm9, %ymm9
	vmaxpd	%ymm1, %ymm9, %ymm9
	vmaxpd	%ymm1, %ymm11, %ymm11
	vminpd	%ymm2, %ymm11, %ymm11
	vminpd	%ymm2, %ymm9, %ymm9
	vaddpd	%ymm10, %ymm9, %ymm9
	vaddpd	%ymm10, %ymm11, %ymm11
	vshufps	$136, %ymm9, %ymm11, %ymm9
	vmovups	(%rdx,%rsi), %ymm11
	vbroadcastss	.LCPI22_2(%rip), %ymm12
	vmulps	%ymm12, %ymm11, %ymm12
	vaddps	%ymm12, %ymm14, %ymm12
	vmulps	%ymm13, %ymm15, %ymm15
	vaddps	%ymm15, %ymm12, %ymm12
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm3, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdi,%rsi)
	vcvtps2pd	%xmm12, %ymm9
	vextractf128	$1, %ymm12, %xmm12
	vcvtps2pd	%xmm12, %ymm12
	vmaxpd	%ymm4, %ymm12, %ymm12
	vmaxpd	%ymm4, %ymm9, %ymm9
	vminpd	%ymm5, %ymm9, %ymm9
	vminpd	%ymm5, %ymm12, %ymm12
	vaddpd	%ymm10, %ymm12, %ymm12
	vaddpd	%ymm10, %ymm9, %ymm9
	vshufps	$136, %ymm12, %ymm9, %ymm9
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm6, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdx,%rsi)
	vbroadcastss	.LCPI22_4(%rip), %ymm9
	vmulps	%ymm9, %ymm11, %ymm9
	vaddps	%ymm9, %ymm14, %ymm9
	vcvtps2pd	%xmm9, %ymm11
	vextractf128	$1, %ymm9, %xmm9
	vcvtps2pd	%xmm9, %ymm9
	vmaxpd	%ymm7, %ymm9, %ymm9
	vmaxpd	%ymm7, %ymm11, %ymm11
	vminpd	%ymm8, %ymm11, %ymm11
	vminpd	%ymm8, %ymm9, %ymm9
	vaddpd	%ymm10, %ymm9, %ymm9
	vaddpd	%ymm10, %ymm11, %ymm11
	vshufps	$136, %ymm9, %ymm11, %ymm9
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm0, %ymm9, %ymm9
	vmovdqu	%ymm9, (%r8,%rsi)
	addq	$32, %rsi
	decq	%rcx
	jne	.LBB22_16
	jmp	.LBB22_31
.LBB22_37:
	pushq	24(%rsp)
	.cfi_adjust_cfa_offset 8
	pushq	88(%rsp)
	.cfi_adjust_cfa_offset 8
	vzeroupper
	callq	_RINvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel29inverse_and_finish_typed_rowsKb1_EB6_
.LBB22_40:
	addq	$16, %rsp
	.cfi_adjust_cfa_offset -16
	movq	%rbx, %rax
.LBB22_31:
	movq	40(%rsp), %rcx
	cmpq	%rax, %rcx
	cmovbq	%rcx, %rax
	movq	32(%rsp), %rcx
	cmpq	%rcx, %rax
	cmovaeq	%rcx, %rax
	shlq	$3, %rax
	addq	$88, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.LBB22_32:
	.cfi_def_cfa_offset 144
	movq	%rcx, %rsi
	shlq	$5, %rsi
	leaq	(%rdi,%rsi), %r10
	leaq	(%rdx,%rsi), %rbx
	addq	%r8, %rsi
	cmpq	%rbx, %rdi
	setb	%bpl
	cmpq	%r10, %rdx
	setb	%r14b
	cmpq	%rsi, %rdi
	setb	%r9b
	cmpq	%r10, %r8
	setb	%r11b
	cmpq	%rsi, %rdx
	setb	%sil
	cmpq	%rbx, %r8
	setb	%r10b
	testb	%r14b, %bpl
	jne	.LBB22_15
	andb	%r11b, %r9b
	jne	.LBB22_15
	andb	%r10b, %sil
	jne	.LBB22_15
	vpbroadcastd	%xmm3, %ymm1
	vbroadcastsd	%xmm2, %ymm2
	vbroadcastsd	%xmm4, %ymm3
	vpbroadcastd	%xmm0, %ymm4
	vbroadcastsd	%xmm5, %ymm5
	vbroadcastsd	%xmm6, %ymm6
	vpbroadcastd	.LCPI22_0(%rip), %ymm9
	vpermd	%ymm0, %ymm9, %ymm0
	vbroadcastsd	%xmm7, %ymm7
	vbroadcastsd	%xmm8, %ymm8
	xorl	%esi, %esi
	vbroadcastss	.LCPI22_4(%rip), %ymm12
	vbroadcastsd	.LCPI22_5(%rip), %ymm13
	.p2align	4
.LBB22_36:
	vmovups	(%rdi,%rsi), %ymm14
	vmovups	(%rdx,%rsi), %ymm15
	vmovups	(%r8,%rsi), %ymm9
	vbroadcastss	.LCPI22_2(%rip), %ymm10
	vmulps	%ymm10, %ymm15, %ymm10
	vaddps	%ymm10, %ymm14, %ymm10
	vbroadcastss	.LCPI22_3(%rip), %ymm11
	vmulps	%ymm11, %ymm9, %ymm11
	vaddps	%ymm11, %ymm10, %ymm10
	vbroadcastss	.LCPI22_1(%rip), %ymm11
	vmulps	%ymm11, %ymm9, %ymm9
	vaddps	%ymm9, %ymm14, %ymm9
	vmulps	%ymm12, %ymm15, %ymm11
	vaddps	%ymm11, %ymm14, %ymm11
	vcvtps2pd	%xmm9, %ymm14
	vextractf128	$1, %ymm9, %xmm9
	vcvtps2pd	%xmm9, %ymm9
	vmaxpd	%ymm2, %ymm9, %ymm9
	vmaxpd	%ymm2, %ymm14, %ymm14
	vminpd	%ymm3, %ymm14, %ymm14
	vminpd	%ymm3, %ymm9, %ymm9
	vaddpd	%ymm13, %ymm9, %ymm9
	vaddpd	%ymm13, %ymm14, %ymm14
	vshufps	$136, %ymm9, %ymm14, %ymm9
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm1, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdi,%rsi)
	vcvtps2pd	%xmm10, %ymm9
	vextractf128	$1, %ymm10, %xmm10
	vcvtps2pd	%xmm10, %ymm10
	vmaxpd	%ymm5, %ymm10, %ymm10
	vmaxpd	%ymm5, %ymm9, %ymm9
	vminpd	%ymm6, %ymm9, %ymm9
	vminpd	%ymm6, %ymm10, %ymm10
	vaddpd	%ymm13, %ymm10, %ymm10
	vaddpd	%ymm13, %ymm9, %ymm9
	vshufps	$136, %ymm10, %ymm9, %ymm9
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm4, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdx,%rsi)
	vcvtps2pd	%xmm11, %ymm9
	vextractf128	$1, %ymm11, %xmm10
	vcvtps2pd	%xmm10, %ymm10
	vmaxpd	%ymm7, %ymm10, %ymm10
	vmaxpd	%ymm7, %ymm9, %ymm9
	vminpd	%ymm8, %ymm9, %ymm9
	vminpd	%ymm8, %ymm10, %ymm10
	vaddpd	%ymm13, %ymm10, %ymm10
	vaddpd	%ymm13, %ymm9, %ymm9
	vshufps	$136, %ymm10, %ymm9, %ymm9
	vpermpd	$216, %ymm9, %ymm9
	vpaddd	%ymm0, %ymm9, %ymm9
	vmovdqu	%ymm9, (%r8,%rsi)
	addq	$32, %rsi
	decq	%rcx
	jne	.LBB22_36
	jmp	.LBB22_31
.LBB22_26:
	movq	%rcx, %rsi
	shlq	$5, %rsi
	leaq	(%rdi,%rsi), %r10
	leaq	(%rdx,%rsi), %rbx
	addq	%r8, %rsi
	cmpq	%rbx, %rdi
	setb	%bpl
	cmpq	%r10, %rdx
	setb	%r14b
	cmpq	%rsi, %rdi
	setb	%r9b
	cmpq	%r10, %r8
	setb	%r11b
	cmpq	%rsi, %rdx
	setb	%sil
	cmpq	%rbx, %r8
	setb	%r10b
	testb	%r14b, %bpl
	jne	.LBB22_24
	andb	%r11b, %r9b
	jne	.LBB22_24
	andb	%r10b, %sil
	jne	.LBB22_24
	vmovd	%r15d, %xmm3
	vpbroadcastd	%xmm3, %ymm3
	vbroadcastss	%xmm1, %ymm5
	vbroadcastss	.LCPI22_6(%rip), %ymm8
	vpermps	%ymm1, %ymm8, %ymm1
	vmovd	%r12d, %xmm6
	vpbroadcastd	%xmm6, %ymm6
	vbroadcastss	%xmm2, %ymm7
	vpermps	%ymm2, %ymm8, %ymm2
	vmovd	8(%rsp), %xmm8
	vpbroadcastd	%xmm8, %ymm8
	vbroadcastss	%xmm4, %ymm4
	vbroadcastss	%xmm0, %ymm0
	xorl	%esi, %esi
	vbroadcastss	.LCPI22_4(%rip), %ymm12
	vbroadcastss	.LCPI22_7(%rip), %ymm13
	.p2align	4
.LBB22_30:
	vmovups	(%rdi,%rsi), %ymm14
	vmovups	(%rdx,%rsi), %ymm15
	vmovups	(%r8,%rsi), %ymm9
	vbroadcastss	.LCPI22_2(%rip), %ymm10
	vmulps	%ymm10, %ymm15, %ymm10
	vaddps	%ymm10, %ymm14, %ymm10
	vbroadcastss	.LCPI22_3(%rip), %ymm11
	vmulps	%ymm11, %ymm9, %ymm11
	vaddps	%ymm11, %ymm10, %ymm10
	vbroadcastss	.LCPI22_1(%rip), %ymm11
	vmulps	%ymm11, %ymm9, %ymm9
	vaddps	%ymm9, %ymm14, %ymm9
	vmulps	%ymm12, %ymm15, %ymm11
	vaddps	%ymm11, %ymm14, %ymm11
	vmaxps	%ymm5, %ymm9, %ymm9
	vminps	%ymm1, %ymm9, %ymm9
	vaddps	%ymm13, %ymm9, %ymm9
	vpaddd	%ymm3, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdi,%rsi)
	vmaxps	%ymm7, %ymm10, %ymm9
	vminps	%ymm2, %ymm9, %ymm9
	vaddps	%ymm13, %ymm9, %ymm9
	vpaddd	%ymm6, %ymm9, %ymm9
	vmovdqu	%ymm9, (%rdx,%rsi)
	vmaxps	%ymm4, %ymm11, %ymm9
	vminps	%ymm0, %ymm9, %ymm9
	vaddps	%ymm13, %ymm9, %ymm9
	vpaddd	%ymm9, %ymm8, %ymm9
	vmovdqu	%ymm9, (%r8,%rsi)
	addq	$32, %rsi
	decq	%rcx
	jne	.LBB22_30
	jmp	.LBB22_31
.LBB22_21:
	movl	%esi, 12(%rsp)
	movl	%r9d, 16(%rsp)
	jmp	.LBB22_42
.LBB22_44:
	movl	%r11d, 12(%rsp)
	movl	%r10d, 16(%rsp)
.LBB22_42:
	leaq	12(%rsp), %rax
	movq	%rax, 48(%rsp)
	leaq	_RNvXsQ_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_5Debug3fmt(%rip), %rax
	movq	%rax, 56(%rsp)
	leaq	16(%rsp), %rcx
	movq	%rcx, 64(%rsp)
	movq	%rax, 72(%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.22(%rip), %rdi
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.13(%rip), %rdx
	leaq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.LBB22_41:
	movl	%r14d, 12(%rsp)
	movl	%ebp, 16(%rsp)
	jmp	.LBB22_42
.Lfunc_end22:
	.size	_RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel18inverse_and_finish, .Lfunc_end22-_RNvNtNtCsaU0XrY2Q1Mf_22flat_natural_cast_main7current6kernel18inverse_and_finish
	.cfi_endproc

	.section	.text._RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_NtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end23, nop
	.type	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_NtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_NtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r13
	.cfi_def_cfa_offset 32
	pushq	%r12
	.cfi_def_cfa_offset 40
	pushq	%rbx
	.cfi_def_cfa_offset 48
	subq	$32, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r13, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	(%rdi), %r12
	leaq	16(%rsp), %rbx
	movq	%rbx, %rdi
	callq	*_RNvMsa_NtCs8uuN1nrGffB_4core3fmtNtB5_9Formatter10debug_list@GOTPCREL(%rip)
	movq	%r12, 8(%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.0(%rip), %r14
	movq	_RNvMs6_NtNtCs8uuN1nrGffB_4core3fmt8buildersNtB5_9DebugList5entry@GOTPCREL(%rip), %r13
	leaq	8(%rsp), %r15
	movq	%rbx, %rdi
	movq	%r15, %rsi
	movq	%r14, %rdx
	callq	*%r13
	leaq	24(%r12), %rax
	movq	%rax, 8(%rsp)
	movq	%rbx, %rdi
	movq	%r15, %rsi
	movq	%r14, %rdx
	callq	*%r13
	addq	$48, %r12
	movq	%r12, 8(%rsp)
	movq	%rbx, %rdi
	movq	%r15, %rsi
	movq	%r14, %rdx
	callq	*%r13
	movq	%rbx, %rdi
	callq	*_RNvMs6_NtNtCs8uuN1nrGffB_4core3fmt8buildersNtB5_9DebugList6finish@GOTPCREL(%rip)
	addq	$32, %rsp
	.cfi_def_cfa_offset 48
	popq	%rbx
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end23:
	.size	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_NtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end23-_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_NtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end24, nop
	.type	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$24, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	(%rdi), %rax
	movq	8(%rax), %r12
	movq	16(%rax), %r13
	leaq	8(%rsp), %rdi
	callq	*_RNvMsa_NtCs8uuN1nrGffB_4core3fmtNtB5_9Formatter10debug_list@GOTPCREL(%rip)
	testq	%r13, %r13
	je	.LBB24_3
	shlq	$2, %r13
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.1(%rip), %rbx
	leaq	8(%rsp), %r14
	movq	%rsp, %r15
	movq	_RNvMs6_NtNtCs8uuN1nrGffB_4core3fmt8buildersNtB5_9DebugList5entry@GOTPCREL(%rip), %rbp
	.p2align	4
.LBB24_2:
	movq	%r12, (%rsp)
	addq	$4, %r12
	movq	%r14, %rdi
	movq	%r15, %rsi
	movq	%rbx, %rdx
	callq	*%rbp
	addq	$-4, %r13
	jne	.LBB24_2
.LBB24_3:
	leaq	8(%rsp), %rdi
	callq	*_RNvMs6_NtNtCs8uuN1nrGffB_4core3fmt8buildersNtB5_9DebugList6finish@GOTPCREL(%rip)
	addq	$24, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end24:
	.size	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end24-_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end25, nop
	.type	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	movq	(%rdi), %rdi
	movl	16(%rsi), %eax
	testl	$33554432, %eax
	jne	.LBB25_3
	testl	$67108864, %eax
	jne	.LBB25_2
	jmpq	*_RNvXs9_NtNtNtCs8uuN1nrGffB_4core3fmt3num3implNtB9_7Display3fmt@GOTPCREL(%rip)
.LBB25_3:
	jmpq	*_RNvXsv_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_8LowerHex3fmt@GOTPCREL(%rip)
.LBB25_2:
	jmpq	*_RNvXsx_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_8UpperHex3fmt@GOTPCREL(%rip)
.Lfunc_end25:
	.size	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end25-_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RNvXs1i_NtCs8uuN1nrGffB_4core3fmtReNtB6_7Display3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main,"ax",@progbits
	.prefalign	4, .Lfunc_end26, nop
	.type	_RNvXs1i_NtCs8uuN1nrGffB_4core3fmtReNtB6_7Display3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main,@function
_RNvXs1i_NtCs8uuN1nrGffB_4core3fmtReNtB6_7Display3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main:
	.cfi_startproc
	movq	%rsi, %rdx
	movq	(%rdi), %rax
	movq	8(%rdi), %rsi
	movq	%rax, %rdi
	jmpq	*_RNvXsi_NtCs8uuN1nrGffB_4core3fmteNtB5_7Display3fmt@GOTPCREL(%rip)
.Lfunc_end26:
	.size	_RNvXs1i_NtCs8uuN1nrGffB_4core3fmtReNtB6_7Display3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main, .Lfunc_end26-_RNvXs1i_NtCs8uuN1nrGffB_4core3fmtReNtB6_7Display3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main
	.cfi_endproc

	.section	.text._RNvXsQ_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_5Debug3fmt,"ax",@progbits
	.prefalign	4, .Lfunc_end27, nop
	.type	_RNvXsQ_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_5Debug3fmt,@function
_RNvXsQ_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_5Debug3fmt:
	.cfi_startproc
	movl	16(%rsi), %eax
	testl	$33554432, %eax
	jne	.LBB27_3
	testl	$67108864, %eax
	jne	.LBB27_2
	jmpq	*_RNvXs9_NtNtNtCs8uuN1nrGffB_4core3fmt3num3implNtB9_7Display3fmt@GOTPCREL(%rip)
.LBB27_3:
	jmpq	*_RNvXsv_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_8LowerHex3fmt@GOTPCREL(%rip)
.LBB27_2:
	jmpq	*_RNvXsx_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_8UpperHex3fmt@GOTPCREL(%rip)
.Lfunc_end27:
	.size	_RNvXsQ_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_5Debug3fmt, .Lfunc_end27-_RNvXsQ_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_5Debug3fmt
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.prefalign	4, .Lfunc_end28, nop
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rcx
	movslq	%edi, %rdx
	leaq	_RNvCsaU0XrY2Q1Mf_22flat_natural_cast_main4main(%rip), %rax
	movq	%rax, (%rsp)
	leaq	.Lanon.7fa2ded3383697916f6395d26ea5f4df.3(%rip), %rsi
	movq	%rsp, %rdi
	xorl	%r8d, %r8d
	callq	*_RNvNtCsh0g7Hz38UNf_3std2rt19lang_start_internal@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end28:
	.size	main, .Lfunc_end28-main
	.cfi_endproc

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.0,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.0,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.0, 32

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.1,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.1,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.1, 32

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.2,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.2,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.2:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRAINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEj3_NtB6_5Debug3fmtCsaU0XrY2Q1Mf_22flat_natural_cast_main
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.2, 32

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.3,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.3,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.3:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCsaU0XrY2Q1Mf_22flat_natural_cast_main
	.quad	_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0CsaU0XrY2Q1Mf_22flat_natural_cast_main
	.quad	_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0CsaU0XrY2Q1Mf_22flat_natural_cast_main
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.3, 48

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.4,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.7fa2ded3383697916f6395d26ea5f4df.4:
	.asciz	"/home/dev/safe-crate-audit/parent-mct/flat-natural-cast.rs"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.4, 59

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.5,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.5,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.5:
	.quad	.Lanon.7fa2ded3383697916f6395d26ea5f4df.4
	.asciz	":\000\000\000\000\000\000\000\365\000\000\000\r\000\000"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.5, 24

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.6,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.7fa2ded3383697916f6395d26ea5f4df.6:
	.asciz	"/rustc/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/slice/sort/stable/quicksort.rs"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.6, 96

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.7,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.7,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.7:
	.quad	.Lanon.7fa2ded3383697916f6395d26ea5f4df.6
	.asciz	"_\000\000\000\000\000\000\000M\000\000\000\037\000\000"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.7, 24

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.8,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.8,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.8:
	.quad	.Lanon.7fa2ded3383697916f6395d26ea5f4df.6
	.asciz	"_\000\000\000\000\000\000\000G\000\000\000\027\000\000"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.8, 24

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.9,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.7fa2ded3383697916f6395d26ea5f4df.9:
	.asciz	"flat-natural-cast-main.rs"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.9, 26

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.10,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.10,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.10:
	.quad	.Lanon.7fa2ded3383697916f6395d26ea5f4df.9
	.asciz	"\031\000\000\000\000\000\000\000T\000\000\0009\000\000"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.10, 24

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.11,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.11,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.11:
	.quad	.Lanon.7fa2ded3383697916f6395d26ea5f4df.4
	.asciz	":\000\000\000\000\000\000\000\026\000\000\000\025\000\000"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.11, 24

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.12,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.7fa2ded3383697916f6395d26ea5f4df.12:
	.asciz	"/home/dev/safe-crate-audit/parent-mct/typed-combo-fixed.rs"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.12, 59

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.13,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.13,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.13:
	.quad	.Lanon.7fa2ded3383697916f6395d26ea5f4df.12
	.asciz	":\000\000\000\000\000\000\000\026\000\000\000\025\000\000"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.13, 24

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.14,@object
	.section	.rodata.cst8,"aM",@progbits,8
.Lanon.7fa2ded3383697916f6395d26ea5f4df.14:
	.ascii	"signed31"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.14, 8

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.15,@object
	.section	.rodata..Lanon.7fa2ded3383697916f6395d26ea5f4df.15,"a",@progbits
.Lanon.7fa2ded3383697916f6395d26ea5f4df.15:
	.ascii	"unsigned31"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.15, 10

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.16,@object
	.section	.rodata..Lanon.7fa2ded3383697916f6395d26ea5f4df.16,"a",@progbits
.Lanon.7fa2ded3383697916f6395d26ea5f4df.16:
	.ascii	"mixed"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.16, 5

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.17,@object
	.section	.rodata..Lanon.7fa2ded3383697916f6395d26ea5f4df.17,"a",@progbits
.Lanon.7fa2ded3383697916f6395d26ea5f4df.17:
	.ascii	"unsigned8"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.17, 9

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.18,@object
	.section	.rodata..Lanon.7fa2ded3383697916f6395d26ea5f4df.18,"a",@progbits
.Lanon.7fa2ded3383697916f6395d26ea5f4df.18:
	.asciz	"\300\002|w\300\004|mag\300\004|ref\305 \000\000p\002\000\b|current\305 \000\000p\002\000\b|uniform\305 \000\000p\002\000\b|control\305 \000\000p\002\000\020|uniform/current\305 \000 p\002\000\002%\n"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.18, 99

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.19,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.19,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.19:
	.quad	.Lanon.7fa2ded3383697916f6395d26ea5f4df.9
	.asciz	"\031\000\000\000\000\000\000\000S\000\000\000\206\000\000"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.19, 24

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.20,@object
	.section	.data.rel.ro..Lanon.7fa2ded3383697916f6395d26ea5f4df.20,"aw",@progbits
	.p2align	3, 0x0
.Lanon.7fa2ded3383697916f6395d26ea5f4df.20:
	.quad	.Lanon.7fa2ded3383697916f6395d26ea5f4df.9
	.asciz	"\031\000\000\000\000\000\000\000Q\000\000\000W\000\000"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.20, 24

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.21,@object
	.section	.rodata..Lanon.7fa2ded3383697916f6395d26ea5f4df.21,"a",@progbits
.Lanon.7fa2ded3383697916f6395d26ea5f4df.21:
	.ascii	"mid > len"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.21, 9

	.type	.Lanon.7fa2ded3383697916f6395d26ea5f4df.22,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.7fa2ded3383697916f6395d26ea5f4df.22:
	.asciz	"\021min > max. min = \300\b, max = \300"
	.size	.Lanon.7fa2ded3383697916f6395d26ea5f4df.22, 30

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.ident	"rustc version 1.100.0 (0ef1bc386 2026-11-09)"
	.section	".note.GNU-stack","",@progbits
