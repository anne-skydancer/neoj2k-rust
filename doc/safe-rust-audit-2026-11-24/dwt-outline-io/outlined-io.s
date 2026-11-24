	.att_syntax
	.file	"outlined_io_main.3cbed8abed6b3041-cgu.0"
	.section	.text._RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtNtB4_5array5drain5DrainINtNtCs3BN1jqgSbDf_5alloc3vec3VecdENCINvMNtNtB4_3ops9try_traitINtB1I_17NeverShortCircuitdE10wrap_mut_1B13_NCNvCs5dlvf2fXWpN_16outlined_io_main4mains1_0E0EEB2P_,"ax",@progbits
	.prefalign	4, .Lfunc_end0, nop
	.type	_RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtNtB4_5array5drain5DrainINtNtCs3BN1jqgSbDf_5alloc3vec3VecdENCINvMNtNtB4_3ops9try_traitINtB1I_17NeverShortCircuitdE10wrap_mut_1B13_NCNvCs5dlvf2fXWpN_16outlined_io_main4mains1_0E0EEB2P_,@function
_RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtNtB4_5array5drain5DrainINtNtCs3BN1jqgSbDf_5alloc3vec3VecdENCINvMNtNtB4_3ops9try_traitINtB1I_17NeverShortCircuitdE10wrap_mut_1B13_NCNvCs5dlvf2fXWpN_16outlined_io_main4mains1_0E0EEB2P_:
	.cfi_startproc
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	movabsq	$-6148914691236517205, %rax
	mulxq	%rax, %r14, %r14
	je	.LBB0_5
	movq	%rdi, %rbx
	shrq	$4, %r14
	addq	$8, %rbx
	movq	_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip), %r15
	jmp	.LBB0_2
	.p2align	4
.LBB0_4:
	addq	$24, %rbx
	decq	%r14
	je	.LBB0_5
.LBB0_2:
	movq	-8(%rbx), %rsi
	testq	%rsi, %rsi
	je	.LBB0_4
	movq	(%rbx), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*%r15
	jmp	.LBB0_4
.LBB0_5:
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	_RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtNtB4_5array5drain5DrainINtNtCs3BN1jqgSbDf_5alloc3vec3VecdENCINvMNtNtB4_3ops9try_traitINtB1I_17NeverShortCircuitdE10wrap_mut_1B13_NCNvCs5dlvf2fXWpN_16outlined_io_main4mains1_0E0EEB2P_, .Lfunc_end0-_RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtNtB4_5array5drain5DrainINtNtCs3BN1jqgSbDf_5alloc3vec3VecdENCINvMNtNtB4_3ops9try_traitINtB1I_17NeverShortCircuitdE10wrap_mut_1B13_NCNvCs5dlvf2fXWpN_16outlined_io_main4mains1_0E0EEB2P_
	.cfi_endproc

	.section	.text.unlikely._RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEBM_ECs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.type	_RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEBM_ECs5dlvf2fXWpN_16outlined_io_main,@function
_RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEBM_ECs5dlvf2fXWpN_16outlined_io_main:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	leaq	24(%rsp), %rax
	movq	%rdi, (%rax)
	leaq	32(%rsp), %rcx
	movq	%rsi, (%rcx)
	movq	%rdx, 8(%rsp)
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.2(%rip), %rdx
	xorl	%edi, %edi
	movq	%rax, %rsi
	movq	%rdx, %r8
	xorl	%r9d, %r9d
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking19assert_failed_inner@GOTPCREL(%rip)
.Lfunc_end1:
	.size	_RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEBM_ECs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end1-_RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEBM_ECs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.text._RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.hidden	_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECs5dlvf2fXWpN_16outlined_io_main
	.globl	_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECs5dlvf2fXWpN_16outlined_io_main
	.prefalign	4, .Lfunc_end2, nop
	.type	_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECs5dlvf2fXWpN_16outlined_io_main,@function
_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECs5dlvf2fXWpN_16outlined_io_main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	%ecx, %r8d
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, (%rsp)
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.3(%rip), %rsi
	movq	%rsp, %rdi
	callq	*_RNvNtCsh0g7Hz38UNf_3std2rt19lang_start_internal@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end2-_RINvNtCsh0g7Hz38UNf_3std2rt10lang_startuECs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.text._RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end3, nop
	.type	_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs5dlvf2fXWpN_16outlined_io_main,@function
_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs5dlvf2fXWpN_16outlined_io_main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	*%rdi
	#APP
	#NO_APP
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end3:
	.size	_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end3-_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.text._RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end4, nop
	.type	_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECs5dlvf2fXWpN_16outlined_io_main,@function
_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECs5dlvf2fXWpN_16outlined_io_main:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
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
	jb	.LBB4_1
	leaq	(,%rbx,8), %r15
	shrq	$61, %rax
	setne	%al
	movabsq	$9223372036854775800, %rcx
	cmpq	%rcx, %r15
	seta	%cl
	orb	%al, %cl
	je	.LBB4_7
	xorl	%r12d, %r12d
.LBB4_8:
	movq	%r12, %rdi
	movq	%r15, %rsi
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.LBB4_1:
	movq	%rsp, %r14
	movl	$512, %ecx
	jmp	.LBB4_2
.LBB4_7:
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
	je	.LBB4_8
.LBB4_2:
	xorl	%r8d, %r8d
	cmpq	$65, %rsi
	setb	%r8b
.Ltmp0:
	movq	%r14, %rdx
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
.Ltmp1:
	cmpq	$512, %rbx
	jbe	.LBB4_4
	shlq	$3, %rbx
	movl	$8, %edx
	movq	%r14, %rdi
	movq	%rbx, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB4_4:
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
.LBB4_10:
	.cfi_def_cfa_offset 4144
.Ltmp2:
	movq	%rax, %r15
	cmpq	$512, %rbx
	jbe	.LBB4_12
	shlq	$3, %rbx
	movl	$8, %edx
	movq	%r14, %rdi
	movq	%rbx, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB4_12:
	movq	%r15, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end4:
	.size	_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end4-_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc
	.section	.gcc_except_table._RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECs5dlvf2fXWpN_16outlined_io_main,"a",@progbits
	.p2align	2, 0x0
GCC_except_table4:
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
	.uleb128 .Lfunc_end4-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	.text._RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end5, nop
	.type	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main,@function
_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main:
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
	jb	.LBB5_2
	shrq	$3, %rcx
	movq	%rcx, %r15
	shlq	$5, %r15
	leaq	(%rdi,%r15), %rsi
	imulq	$56, %rcx, %r12
	leaq	(%rdi,%r12), %rdx
	movq	%rcx, %r13
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	movq	%rax, %rbp
	leaq	(%rbx,%r15), %rsi
	leaq	(%rbx,%r12), %rdx
	movq	%rbx, %rdi
	movq	%r13, %rcx
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	movq	%rax, %rbx
	addq	%r14, %r15
	addq	%r14, %r12
	movq	%r14, %rdi
	movq	%r15, %rsi
	movq	%r12, %rdx
	movq	%r13, %rcx
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	movq	%rbp, %rdi
	movq	%rax, %r14
.LBB5_2:
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
.Lfunc_end5:
	.size	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end5-_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.text._RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end6, nop
	.type	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main,@function
_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main:
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
	jne	.LBB6_1
	testb	%cl, %cl
	jne	.LBB6_4
.LBB6_3:
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
.LBB6_1:
	.cfi_def_cfa_offset 64
	movb	$1, %cl
	testb	%cl, %cl
	je	.LBB6_3
.LBB6_4:
	callq	*_RNvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort22panic_on_ord_violation@GOTPCREL(%rip)
.Lfunc_end6:
	.size	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end6-_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.text._RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end7, nop
	.type	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main,@function
_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main:
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
	je	.LBB7_1
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB7_3
.LBB7_1:
	xorl	%edx, %edx
	divl	%esi
.LBB7_3:
	movq	%rax, 72(%rsp)
	cmpq	$4097, %rsi
	movq	%rcx, 24(%rsp)
	movq	%rsi, 32(%rsp)
	jae	.LBB7_4
	movq	%rsi, %rax
	shrq	%rax
	movq	%rsi, %rdx
	subq	%rax, %rdx
	cmpq	$64, %rdx
	movl	$64, %eax
	cmovbq	%rdx, %rax
	jmp	.LBB7_6
.LBB7_4:
	movq	%rsi, %rdi
	callq	*_RNvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift11sqrt_approx@GOTPCREL(%rip)
	movq	24(%rsp), %rcx
.LBB7_6:
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
.LBB7_7:
	movq	32(%rsp), %r10
	movq	16(%rsp), %rax
	leaq	(%rax,%rbp,8), %rax
	movq	%rax, 8(%rsp)
	subq	%rbp, %r10
	jbe	.LBB7_8
	cmpq	(%rsp), %r10
	jae	.LBB7_27
.LBB7_25:
	cmpb	$0, 68(%rsp)
	je	.LBB7_26
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
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	movq	%r15, %r11
	movq	24(%rsp), %rcx
	movl	%r12d, %eax
	jmp	.LBB7_47
	.p2align	4
.LBB7_8:
	xorl	%esi, %esi
	movl	$1, %r10d
	cmpq	$2, %r11
	jae	.LBB7_10
	jmp	.LBB7_57
	.p2align	4
.LBB7_27:
	cmpq	$2, %r10
	jae	.LBB7_29
	movq	%r10, %rax
.LBB7_47:
	leaq	1(,%rax,2), %r10
	jmp	.LBB7_48
.LBB7_26:
	movq	(%rsp), %rax
	cmpq	%r10, %rax
	cmovbq	%rax, %r10
	addq	%r10, %r10
.LBB7_48:
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
	jb	.LBB7_57
.LBB7_10:
	movq	80(%rsp), %rax
	leaq	(%rax,%rbp,8), %rax
	movq	%rax, 112(%rsp)
	movq	%r10, 40(%rsp)
	movq	%rbp, 120(%rsp)
	movq	%rsi, 88(%rsp)
	jmp	.LBB7_11
.LBB7_62:
	movq	%r14, %rbp
	movq	%r11, %rsi
.LBB7_63:
	subq	%rsi, %rdx
	movq	%rbp, %rdi
	callq	*memcpy@GOTPCREL(%rip)
	movq	24(%rsp), %rcx
	movq	56(%rsp), %r11
	movq	40(%rsp), %r10
	movq	88(%rsp), %rsi
.LBB7_64:
	leaq	1(,%r15,2), %r14
	movq	48(%rsp), %rbx
	movq	120(%rsp), %rbp
	cmpq	$1, %r11
	jbe	.LBB7_56
.LBB7_11:
	cmpb	%sil, 133(%rsp,%r11)
	jb	.LBB7_57
	decq	%r11
	movq	200(%rsp,%r11,8), %rax
	movq	%rax, %r12
	shrq	%r12
	movq	%r14, %r13
	shrq	%r13
	leaq	(%r12,%r13), %r15
	cmpq	%rcx, %r15
	ja	.LBB7_14
	movl	%eax, %edx
	orl	%r14d, %edx
	testb	$1, %dl
	jne	.LBB7_14
	addq	%r15, %r15
	movq	%r15, %r14
	cmpq	$1, %r11
	ja	.LBB7_11
	jmp	.LBB7_56
	.p2align	4
.LBB7_14:
	movq	%rbp, %rdx
	subq	%r15, %rdx
	movq	16(%rsp), %rsi
	leaq	(%rsi,%rdx,8), %rbp
	testb	$1, %al
	movq	%r11, 56(%rsp)
	je	.LBB7_59
	testb	$1, %r14b
	je	.LBB7_16
.LBB7_17:
	testq	%r13, %r13
	movq	88(%rsp), %rsi
	jne	.LBB7_18
	jmp	.LBB7_64
	.p2align	4
.LBB7_59:
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
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	movq	40(%rsp), %r10
	movq	56(%rsp), %r11
	movq	24(%rsp), %rcx
	testb	$1, %r14b
	jne	.LBB7_17
.LBB7_16:
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
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	movq	40(%rsp), %r10
	movq	56(%rsp), %r11
	movq	24(%rsp), %rcx
	testq	%r13, %r13
	movq	88(%rsp), %rsi
	je	.LBB7_64
.LBB7_18:
	testq	%r12, %r12
	je	.LBB7_64
	cmpq	%r13, %r12
	movq	%r13, %rbx
	cmovbq	%r12, %rbx
	cmpq	%rbx, %rcx
	jb	.LBB7_64
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
	jbe	.LBB7_21
	movq	112(%rsp), %rax
	.p2align	4
.LBB7_61:
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
	je	.LBB7_62
	addq	$-8, %rax
	cmpq	%r11, %rdx
	jne	.LBB7_61
	jmp	.LBB7_62
.LBB7_21:
	movq	%r11, %rsi
	movq	8(%rsp), %r11
	.p2align	4
.LBB7_22:
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
	je	.LBB7_63
	movb	%cl, %al
	leaq	(%r14,%rax,8), %r14
	cmpq	%r11, %r14
	jne	.LBB7_22
	jmp	.LBB7_63
	.p2align	4
.LBB7_56:
	movl	$1, %r11d
.LBB7_57:
	movq	%r14, 200(%rsp,%r11,8)
	movb	%sil, 134(%rsp,%r11)
	cmpq	%rbp, 32(%rsp)
	jbe	.LBB7_66
	incq	%r11
	movq	%r10, %rax
	shrq	%rax
	addq	%rax, %rbp
	movq	%r10, %r14
	jmp	.LBB7_7
.LBB7_29:
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
	jge	.LBB7_30
	cmpq	$2, %r10
	jne	.LBB7_34
	cmpq	$2, (%rsp)
	ja	.LBB7_25
	movl	$2, %eax
	movl	$1, %r10d
	jmp	.LBB7_41
.LBB7_30:
	movl	$2, %eax
	cmpq	$2, %r10
	jne	.LBB7_31
	cmpq	$2, (%rsp)
	ja	.LBB7_25
	jmp	.LBB7_47
	.p2align	4
.LBB7_31:
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
	jl	.LBB7_38
	incq	%rax
	cmpq	%rax, %r10
	jne	.LBB7_31
	jmp	.LBB7_37
.LBB7_34:
	movl	$2, %eax
	.p2align	4
.LBB7_35:
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
	jge	.LBB7_38
	incq	%rax
	cmpq	%rax, %r10
	jne	.LBB7_35
.LBB7_37:
	movq	%r10, %rax
.LBB7_38:
	cmpq	(%rsp), %rax
	jb	.LBB7_25
	cmpq	%rdx, %r9
	jge	.LBB7_47
	movq	%rax, %r10
	shrq	%r10
	je	.LBB7_47
.LBB7_41:
	cmpq	$8, %r10
	jae	.LBB7_49
	xorl	%edx, %edx
	jmp	.LBB7_52
.LBB7_49:
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
.LBB7_50:
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
	jne	.LBB7_50
	cmpq	%rdx, %r10
	je	.LBB7_47
.LBB7_52:
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
.LBB7_53:
	vmovsd	(%rdx), %xmm0
	movq	(%rdi,%rsi,8), %r8
	movq	%r8, (%rdx)
	vmovsd	%xmm0, (%rdi,%rsi,8)
	decq	%rsi
	addq	$8, %rdx
	cmpq	%rsi, %r10
	jne	.LBB7_53
	jmp	.LBB7_47
.LBB7_66:
	testb	$1, %r14b
	jne	.LBB7_67
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
	jmp	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
.LBB7_67:
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
.Lfunc_end7:
	.size	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end7-_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.text._RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end8, nop
	.type	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main,@function
_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main:
.Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception1
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
	jae	.LBB8_7
.LBB8_1:
	movq	%r12, %rbp
.LBB8_2:
	cmpq	$2, %r14
	jb	.LBB8_47
	leaq	16(%r14), %rax
	cmpq	%rax, 8(%rsp)
	jb	.LBB8_37
	movq	%r14, %r12
	shrq	%r12
	cmpq	$15, %r14
	movq	%rbp, (%rsp)
	jbe	.LBB8_5
	leaq	(%rbx,%r14,8), %rdx
	movq	%rbp, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	leaq	(,%r12,8), %rdi
	addq	%rbp, %rdi
	leaq	(%rbx,%r12,8), %rsi
	leaq	(%rbx,%r14,8), %rdx
	addq	$64, %rdx
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort12sort8_stabledNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	movl	$8, %ecx
	movq	%r14, %rdx
	subq	%r12, %rdx
	cmpq	%r12, %rcx
	jb	.LBB8_13
	jmp	.LBB8_18
.LBB8_7:
	movl	%r8d, (%rsp)
.LBB8_8:
	movl	(%rsp), %r15d
	testl	%r15d, %r15d
	je	.LBB8_46
	leaq	96(%r12), %rbp
	movq	%r14, %r13
	.p2align	4
.LBB8_44:
	movq	%r13, %rcx
	shrq	$3, %rcx
	movq	%rcx, %rax
	shlq	$5, %rax
	addq	%r12, %rax
	imulq	$56, %rcx, %rdx
	addq	%r12, %rdx
	cmpq	$64, %r13
	jae	.LBB8_45
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
	jmp	.LBB8_49
	.p2align	4
.LBB8_45:
	movq	%r12, %rdi
	movq	%rax, %rsi
	vzeroupper
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared5pivot11median3_recdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
.LBB8_49:
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
	je	.LBB8_51
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
	jge	.LBB8_78
.LBB8_51:
	cmpq	%r13, 8(%rsp)
	jb	.LBB8_37
	leaq	(%rbx,%r13,8), %rax
	xorl	%r14d, %r14d
	movq	%r12, %rcx
	movq	16(%rsp), %rdx
	.p2align	4
.LBB8_53:
	movq	%rdx, %rsi
	subq	$3, %rsi
	movl	$0, %edi
	cmovaeq	%rsi, %rdi
	leaq	(%r12,%rdi,8), %rsi
	cmpq	%rsi, %rcx
	jae	.LBB8_56
	movq	(%r12,%r15), %r8
	movq	%r8, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%r8, %rdi
	.p2align	4
.LBB8_55:
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
	jb	.LBB8_55
.LBB8_56:
	leaq	(%r12,%rdx,8), %rsi
	cmpq	%rsi, %rcx
	jae	.LBB8_59
	movq	(%r12,%r15), %r8
	movq	%r8, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%r8, %rdi
	.p2align	4
.LBB8_58:
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
	jb	.LBB8_58
.LBB8_59:
	cmpq	%r13, %rdx
	je	.LBB8_61
	movq	(%rcx), %rdx
	movq	%rdx, -8(%rax,%r14,8)
	addq	$-8, %rax
	addq	$8, %rcx
	movq	%r13, %rdx
	jmp	.LBB8_53
	.p2align	4
.LBB8_61:
	leaq	(,%r14,8), %rdx
	movq	%r12, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	movq	%r13, %rsi
	subq	%r14, %rsi
	je	.LBB8_75
	cmpq	$4, %rsi
	jae	.LBB8_64
	xorl	%eax, %eax
	jmp	.LBB8_73
	.p2align	4
.LBB8_64:
	cmpq	$16, %rsi
	jae	.LBB8_66
	xorl	%eax, %eax
	jmp	.LBB8_70
.LBB8_66:
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
.LBB8_67:
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
	jne	.LBB8_67
	cmpq	%rax, %rsi
	je	.LBB8_75
	testb	$12, %sil
	je	.LBB8_73
.LBB8_70:
	movq	%rax, %rcx
	leaq	(%r12,%r14,8), %rdx
	movq	%rsi, %rax
	andq	$-4, %rax
	leaq	(,%rcx,8), %r8
	leaq	-32(%rbx), %rdi
	leaq	(%rdi,%r13,8), %rdi
	subq	%r8, %rdi
	.p2align	4
.LBB8_71:
	vpermq	$27, (%rdi), %ymm0
	vmovdqu	%ymm0, (%rdx,%rcx,8)
	addq	$4, %rcx
	addq	$-32, %rdi
	cmpq	%rcx, %rax
	jne	.LBB8_71
	cmpq	%rax, %rsi
	je	.LBB8_75
.LBB8_73:
	leaq	(%rax,%r14), %rcx
	shlq	$3, %rax
	leaq	-8(%rbx), %rdx
	leaq	(%rdx,%r13,8), %rdx
	subq	%rax, %rdx
	.p2align	4
.LBB8_74:
	movq	(%rdx), %rax
	movq	%rax, (%r12,%rcx,8)
	incq	%rcx
	addq	$-8, %rdx
	cmpq	%rcx, %r13
	jne	.LBB8_74
.LBB8_75:
	testq	%r14, %r14
	je	.LBB8_78
	cmpq	%r14, %r13
	jb	.LBB8_105
	leaq	(%r12,%r14,8), %rdi
	movq	%rbx, %rdx
	movq	8(%rsp), %rcx
	movl	(%rsp), %r15d
	movl	%r15d, %r8d
	leaq	64(%rsp), %r9
	vzeroupper
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	cmpq	$33, %r14
	jb	.LBB8_1
	movq	%r14, %r13
	testl	%r15d, %r15d
	jne	.LBB8_44
	jmp	.LBB8_46
	.p2align	4
.LBB8_78:
	cmpq	%r13, 8(%rsp)
	jb	.LBB8_37
	leaq	(%rbx,%r13,8), %rax
	xorl	%ebp, %ebp
	movq	%r12, %rcx
	movq	16(%rsp), %r14
	.p2align	4
.LBB8_80:
	movq	%r14, %rdx
	subq	$3, %rdx
	movl	$0, %esi
	cmovaeq	%rdx, %rsi
	leaq	(%r12,%rsi,8), %rdx
	cmpq	%rdx, %rcx
	jae	.LBB8_83
	movq	(%r12,%r15), %rdi
	movq	%rdi, %rsi
	sarq	$63, %rsi
	shrq	%rsi
	xorq	%rdi, %rsi
	.p2align	4
.LBB8_82:
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
	jb	.LBB8_82
.LBB8_83:
	leaq	(%r12,%r14,8), %rdx
	cmpq	%rdx, %rcx
	jae	.LBB8_86
	movq	(%r12,%r15), %rdi
	movq	%rdi, %rsi
	sarq	$63, %rsi
	shrq	%rsi
	xorq	%rdi, %rsi
	.p2align	4
.LBB8_85:
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
	jb	.LBB8_85
.LBB8_86:
	cmpq	%r13, %r14
	je	.LBB8_88
	addq	$-8, %rax
	movq	(%rcx), %rdx
	movq	%rdx, (%rbx,%rbp,8)
	incq	%rbp
	addq	$8, %rcx
	movq	%r13, %r14
	jmp	.LBB8_80
	.p2align	4
.LBB8_88:
	leaq	(,%rbp,8), %rdx
	movq	%r12, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	movq	%r13, %r14
	subq	%rbp, %r14
	je	.LBB8_47
	leaq	(%r12,%rbp,8), %r15
	cmpq	$4, %r14
	jae	.LBB8_91
	xorl	%eax, %eax
	jmp	.LBB8_100
.LBB8_91:
	cmpq	$16, %r14
	jae	.LBB8_93
	xorl	%eax, %eax
	jmp	.LBB8_97
.LBB8_93:
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
.LBB8_94:
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
	jne	.LBB8_94
	cmpq	%rax, %r14
	je	.LBB8_102
	testb	$12, %r14b
	je	.LBB8_100
.LBB8_97:
	movq	%rax, %rcx
	movq	%r14, %rax
	andq	$-4, %rax
	leaq	(,%rcx,8), %rdx
	leaq	-32(%rbx), %rsi
	subq	%rdx, %rsi
	leaq	(%rsi,%r13,8), %rdx
	movq	%r15, %rsi
	.p2align	4
.LBB8_98:
	vpermq	$27, (%rdx), %ymm0
	vmovdqu	%ymm0, (%rsi,%rcx,8)
	addq	$4, %rcx
	addq	$-32, %rdx
	cmpq	%rcx, %rax
	jne	.LBB8_98
	cmpq	%rax, %r14
	je	.LBB8_102
.LBB8_100:
	leaq	(%rax,%rbp), %rcx
	shlq	$3, %rax
	leaq	-8(%rbx), %rdx
	subq	%rax, %rdx
	leaq	(%rdx,%r13,8), %rax
	.p2align	4
.LBB8_101:
	movq	(%rax), %rdx
	movq	%rdx, (%r12,%rcx,8)
	incq	%rcx
	addq	$-8, %rax
	cmpq	%rcx, %r13
	jne	.LBB8_101
.LBB8_102:
	cmpq	%rbp, %r13
	jb	.LBB8_104
	movq	%r15, %rbp
	movq	%r15, %r12
	movl	$0, %eax
	movq	%rax, 24(%rsp)
	cmpq	$33, %r14
	jae	.LBB8_8
	jmp	.LBB8_2
.LBB8_46:
	movq	%r12, %rdi
	movq	%r14, %rsi
	movq	%rbx, %rdx
	movq	8(%rsp), %rcx
	movl	$1, %r8d
	vzeroupper
	callq	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable5drift4sortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	jmp	.LBB8_47
.LBB8_5:
	cmpq	$7, %r14
	jbe	.LBB8_11
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
	jb	.LBB8_13
	jmp	.LBB8_18
.LBB8_11:
	movq	(%rbp), %rax
	movq	%rax, (%rbx)
	movq	(%rbp,%r12,8), %rax
	movq	%rax, (%rbx,%r12,8)
	movl	$1, %ecx
	movq	%r14, %rdx
	subq	%r12, %rdx
	cmpq	%r12, %rcx
	jae	.LBB8_18
.LBB8_13:
	leaq	-8(,%rcx,8), %rax
	movq	%rcx, %rsi
	jmp	.LBB8_14
.LBB8_17:
	addq	%rbx, %r9
	movq	%r9, %r10
.LBB8_41:
	movq	%rdi, (%r10)
.LBB8_42:
	incq	%rsi
	addq	$8, %rax
	cmpq	%r12, %rsi
	je	.LBB8_18
.LBB8_14:
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
	jge	.LBB8_42
	movq	%r9, (%rbx,%rsi,8)
	movq	%rax, %r9
	movq	%rbx, %r10
	cmpq	$1, %rsi
	je	.LBB8_41
	.p2align	4
.LBB8_16:
	movq	-8(%rbx,%r9), %r10
	movq	%r10, %r11
	sarq	$63, %r11
	shrq	%r11
	xorq	%r10, %r11
	cmpq	%r11, %r8
	jge	.LBB8_17
	movq	%r10, (%rbx,%r9)
	addq	$-8, %r9
	jne	.LBB8_16
	movq	%rbx, %r10
	jmp	.LBB8_41
.LBB8_18:
	leaq	(%rbx,%r12,8), %rax
	cmpq	%rdx, %rcx
	jae	.LBB8_28
	leaq	(,%r12,8), %rsi
	addq	%rbp, %rsi
	leaq	-8(,%rcx,8), %rdi
	jmp	.LBB8_20
.LBB8_25:
	addq	%rax, %r10
	movq	%r10, %r11
.LBB8_26:
	movq	%r8, (%r11)
.LBB8_27:
	incq	%rcx
	addq	$8, %rdi
	cmpq	%rdx, %rcx
	je	.LBB8_28
.LBB8_20:
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
	jge	.LBB8_27
	movq	%r10, (%rax,%rcx,8)
	movq	%rdi, %r10
	movq	%rax, %r11
	cmpq	$1, %rcx
	je	.LBB8_26
	.p2align	4
.LBB8_22:
	movq	-8(%rax,%r10), %r11
	movq	%r11, %r13
	sarq	$63, %r13
	shrq	%r13
	xorq	%r11, %r13
	cmpq	%r13, %r9
	jge	.LBB8_25
	movq	%r11, (%rax,%r10)
	addq	$-8, %r10
	jne	.LBB8_22
	movq	%rax, %r11
	jmp	.LBB8_26
.LBB8_28:
	leaq	-1(%r14), %rdi
	leaq	(%rbx,%r14,8), %rdx
	addq	$-8, %rdx
	leaq	-8(%rax), %r15
	xorl	%r8d, %r8d
	movq	%rbx, %rsi
	movq	%r12, 8(%rsp)
	movq	(%rsp), %rcx
	.p2align	4
.LBB8_29:
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
	jne	.LBB8_29
	addq	$8, %r15
	addq	$8, %rdx
	testb	$1, %r14b
	jne	.LBB8_32
	movq	(%rsp), %r13
	jmp	.LBB8_33
.LBB8_32:
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
.LBB8_33:
	cmpq	%rdx, %rax
	setne	%al
	cmpq	%r15, %rsi
	jne	.LBB8_34
	testb	%al, %al
	jne	.LBB8_36
.LBB8_47:
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
.LBB8_105:
	.cfi_def_cfa_offset 128
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.28(%rip), %rdi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.5(%rip), %rdx
	movl	$19, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.LBB8_34:
	movb	$1, %al
	testb	%al, %al
	je	.LBB8_47
.LBB8_36:
.Ltmp3:
	vzeroupper
	callq	*_RNvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6shared9smallsort22panic_on_ord_violation@GOTPCREL(%rip)
.Ltmp4:
.LBB8_37:
	ud2
.LBB8_104:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.6(%rip), %rcx
	movq	%rbp, %rdi
	movq	%r13, %rsi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB8_38:
.Ltmp5:
	movq	%rax, %r12
	shlq	$3, %r14
	movq	%r13, %rdi
	movq	%rbx, %rsi
	movq	%r14, %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	%r12, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end8:
	.size	_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end8-_RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc
	.section	.gcc_except_table._RINvNtNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable9quicksort9quicksortdNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtBa_3f64d9total_cmpE0ECs5dlvf2fXWpN_16outlined_io_main,"a",@progbits
	.p2align	2, 0x0
GCC_except_table8:
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
	.uleb128 .Lfunc_end8-.Ltmp4
	.byte	0
	.byte	0
.Lcst_end1:
	.p2align	2, 0x0

	.section	.text._RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0Cs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end9, nop
	.type	_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0Cs5dlvf2fXWpN_16outlined_io_main,@function
_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0Cs5dlvf2fXWpN_16outlined_io_main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs5dlvf2fXWpN_16outlined_io_main
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end9:
	.size	_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0Cs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end9-_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0Cs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.text._RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end10, nop
	.type	_RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCs5dlvf2fXWpN_16outlined_io_main,@function
_RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCs5dlvf2fXWpN_16outlined_io_main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_RINvNtNtCsh0g7Hz38UNf_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs5dlvf2fXWpN_16outlined_io_main
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end10:
	.size	_RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end10-_RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.rodata.cst32,"aM",@progbits,32
	.p2align	5, 0x0
.LCPI11_0:
	.quad	3
	.quad	32
	.quad	64
	.quad	128
.LCPI11_3:
	.quad	0
	.quad	1
	.quad	2
	.quad	3
.LCPI11_4:
	.quad	3
	.quad	2
	.quad	1
	.quad	0
.LCPI11_5:
	.quad	1
	.quad	3
	.quad	0
	.quad	2
.LCPI11_6:
	.quad	2
	.quad	0
	.quad	3
	.quad	1
.LCPI11_7:
	.quad	0
	.quad	2
	.quad	1
	.quad	3
.LCPI11_8:
	.quad	3
	.quad	1
	.quad	2
	.quad	0
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.LCPI11_1:
	.quad	128
	.quad	1024
.LCPI11_10:
	.long	1127219200
	.long	1160773632
	.long	0
	.long	0
.LCPI11_11:
	.quad	0x4330000000000000
	.quad	0x4530000000000000
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI11_2:
	.long	305419896
	.section	.rodata.cst8,"aM",@progbits,8
	.p2align	3, 0x0
.LCPI11_9:
	.quad	0x41cdcd6500000000
.LCPI11_12:
	.quad	0x412e848000000000
.LCPI11_13:
	.quad	0x40c3880000000000
	.section	.text._RNvCs5dlvf2fXWpN_16outlined_io_main4main,"ax",@progbits
	.hidden	_RNvCs5dlvf2fXWpN_16outlined_io_main4main
	.globl	_RNvCs5dlvf2fXWpN_16outlined_io_main4main
	.prefalign	4, .Lfunc_end11, nop
	.type	_RNvCs5dlvf2fXWpN_16outlined_io_main4main,@function
_RNvCs5dlvf2fXWpN_16outlined_io_main4main:
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
	subq	$968, %rsp
	.cfi_def_cfa_offset 1024
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	vmovaps	.LCPI11_0(%rip), %ymm0
	vmovups	%ymm0, 744(%rsp)
	xorl	%ebx, %ebx
	movq	_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip), %rbp
	movq	_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip), %r15
	movq	_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip), %r14
	leaq	_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control15inverse_97_rows(%rip), %rax
	vmovq	%rax, %xmm0
	leaq	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel15inverse_97_rows(%rip), %rax
	vmovq	%rax, %xmm1
	vpunpcklqdq	%xmm0, %xmm1, %xmm1
	vmovdqa	%xmm1, 400(%rsp)
	leaq	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel15inverse_97_rows(%rip), %rax
	vmovq	%rax, %xmm1
	vpunpcklqdq	%xmm1, %xmm0, %xmm0
	vmovdqa	%xmm0, 688(%rsp)
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.21(%rip), %r12
.LBB11_1:
	movq	%rbx, %rax
	incq	%rbx
	movq	%rbx, 736(%rsp)
	movq	752(%rsp,%rax,8), %rax
	movq	%rax, 112(%rsp)
	vzeroupper
	callq	*%rbp
	movl	$32, %edi
	movl	$8, %esi
	callq	*%r15
	testq	%rax, %rax
	je	.LBB11_269
	movq	%rax, %r13
	movq	112(%rsp), %rax
	leaq	8(%r13), %rdx
	leaq	16(%r13), %rcx
	movq	%rax, %rsi
	vmovaps	.LCPI11_1(%rip), %xmm0
	vmovups	%xmm0, 16(%r13)
	subq	$-3, %rsi
	vmovq	%rax, %xmm0
	vmovq	%rsi, %xmm1
	vpunpcklqdq	%xmm1, %xmm0, %xmm0
	vmovdqu	%xmm0, (%r13)
	jb	.LBB11_9
	movq	%rax, 8(%r13)
	movq	%rsi, (%r13)
	movq	%rax, 16(%r13)
	movq	%rax, %rsi
.LBB11_4:
	movq	%rdx, %rdi
.LBB11_5:
	movq	$128, (%rdi)
	movq	8(%r13), %r8
	movq	24(%r13), %rdi
	cmpq	%rsi, %rdi
	jae	.LBB11_12
	movq	%rsi, 24(%r13)
	movq	%rcx, %rsi
	cmpq	%r8, %rdi
	jae	.LBB11_16
	movq	%r8, 16(%r13)
	movq	(%r13), %rsi
	cmpq	%rsi, %rdi
	jae	.LBB11_15
	movq	%rsi, (%rdx)
	movq	%r13, %rsi
	jmp	.LBB11_16
	.p2align	4
.LBB11_9:
	cmpq	$129, %rsi
	jb	.LBB11_13
	movq	%rsi, (%rcx)
	cmpq	$129, %rax
	jb	.LBB11_4
	movq	%rax, (%rdx)
	movq	%r13, %rdi
	jmp	.LBB11_5
	.p2align	4
.LBB11_12:
	movq	%r8, %rsi
.LBB11_13:
	movl	$1, %edx
	cmpq	(%r13), %rsi
	jne	.LBB11_17
	jmp	.LBB11_18
.LBB11_15:
	movq	%rdx, %rsi
.LBB11_16:
	movq	%rdi, (%rsi)
	movq	8(%r13), %rsi
	movl	$1, %edx
	cmpq	(%r13), %rsi
	je	.LBB11_18
.LBB11_17:
	movq	(%rcx), %rcx
	movl	$2, %edx
	cmpq	%rsi, %rcx
	jne	.LBB11_23
.LBB11_18:
	leaq	1(%rdx), %rcx
	movq	8(%r13,%rdx,8), %rsi
	cmpq	-8(%r13,%rdx,8), %rsi
	je	.LBB11_20
	movq	%rsi, (%r13,%rdx,8)
	movq	%rcx, %rdx
.LBB11_20:
	cmpl	$3, %ecx
	je	.LBB11_24
	movq	24(%r13), %rcx
	cmpq	-8(%r13,%rdx,8), %rcx
	je	.LBB11_24
	movq	%rcx, (%r13,%rdx,8)
	incq	%rdx
	jmp	.LBB11_24
	.p2align	4
.LBB11_23:
	xorl	%edx, %edx
	cmpq	%rcx, 24(%r13)
	setne	%dl
	addq	$3, %rdx
.LBB11_24:
	leaq	(,%rdx,8), %rdx
	addq	%r13, %rdx
	movq	%r13, %r15
	.p2align	4
.LBB11_25:
	cmpq	%rdx, %r15
	je	.LBB11_258
	movq	(%r15), %rcx
	addq	$8, %r15
	cmpq	%rax, %rcx
	jb	.LBB11_25
	movq	%r12, 272(%rsp)
	movq	%rcx, 144(%rsp)
	movq	$2, 664(%rsp)
	movq	$0, 672(%rsp)
	movq	$1, 680(%rsp)
	xorl	%eax, %eax
.LBB11_28:
	leaq	1(%rax), %rsi
	movq	%rsi, 656(%rsp)
	movq	672(%rsp,%rax,8), %rax
	movq	%rax, 208(%rsp)
	movq	144(%rsp), %rbp
	leaq	(,%rbp,8), %rdi
	movq	%rbp, %r12
	shlq	$5, %r12
	movq	%rdi, %rax
	shrq	$62, %rax
	setne	%al
	movabsq	$9223372036854775804, %rcx
	cmpq	%rcx, %r12
	seta	%cl
	orb	%al, %cl
	movq	%r12, 168(%rsp)
	jne	.LBB11_260
	movq	%rdi, 40(%rsp)
	movq	%rsi, 456(%rsp)
	movq	%rdx, 464(%rsp)
	movq	%rbx, 472(%rsp)
	testq	%r12, %r12
	je	.LBB11_33
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %ebx
	movl	$4, %esi
	movq	%r12, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	movq	%rbp, %r12
	leaq	(,%rbp,8), %rcx
	movq	%rcx, 24(%rsp)
	testq	%rax, %rax
	je	.LBB11_261
	testq	%rcx, %rcx
	movabsq	$595056260442243601, %rbx
	movq	_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip), %rbp
	je	.LBB11_32
.LBB11_34:
	leaq	(,%r12,8), %rcx
	leaq	-1(%rcx), %rsi
	movl	$1, %ecx
	movq	%rax, %r8
	cmpq	$7, %rsi
	je	.LBB11_42
	cmpq	$32, %rsi
	jae	.LBB11_37
	xorl	%edx, %edx
	jmp	.LBB11_40
.LBB11_33:
	movl	$4, %eax
	movq	$0, 24(%rsp)
	movq	%rbp, %r12
	leaq	(,%rbp,8), %rcx
	testq	%rcx, %rcx
	movabsq	$595056260442243601, %rbx
	movq	_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip), %rbp
	jne	.LBB11_34
	.p2align	4
.LBB11_32:
	cmpq	$0, 112(%rsp)
	movq	%rax, 96(%rsp)
	movq	%r13, 256(%rsp)
	jne	.LBB11_45
	jmp	.LBB11_80
.LBB11_37:
	movq	%rsi, %rdx
	andq	$-32, %rdx
	leaq	(%rax,%rdx,4), %r8
	leaq	1(%rdx), %rcx
	xorl	%edi, %edi
	.p2align	4
.LBB11_38:
	vpbroadcastd	.LCPI11_2(%rip), %ymm0
	vmovdqu	%ymm0, (%rax,%rdi,4)
	vmovdqu	%ymm0, 32(%rax,%rdi,4)
	vmovdqu	%ymm0, 64(%rax,%rdi,4)
	vmovdqu	%ymm0, 96(%rax,%rdi,4)
	addq	$32, %rdi
	cmpq	%rdi, %rdx
	jne	.LBB11_38
	testb	$24, %sil
	je	.LBB11_42
.LBB11_40:
	leaq	(,%r12,8), %rcx
	leaq	-8(%rcx), %rsi
	leaq	(%rax,%rcx,4), %r8
	addq	$-32, %r8
	addq	$-7, %rcx
	.p2align	4
.LBB11_41:
	vpbroadcastd	.LCPI11_2(%rip), %ymm0
	vmovdqu	%ymm0, (%rax,%rdx,4)
	addq	$8, %rdx
	cmpq	%rdx, %rsi
	jne	.LBB11_41
.LBB11_42:
	leaq	(,%r12,8), %rdx
	subq	%rcx, %rdx
	xorl	%ecx, %ecx
	.p2align	4
.LBB11_43:
	movl	$305419896, (%r8,%rcx,4)
	incq	%rcx
	cmpq	%rcx, %rdx
	jne	.LBB11_43
	movl	$305419896, (%r8,%rcx,4)
	cmpq	$0, 112(%rsp)
	movq	%rax, 96(%rsp)
	movq	%r13, 256(%rsp)
	je	.LBB11_80
.LBB11_45:
	leaq	(,%r12,8), %rcx
	testq	%rcx, %rcx
	je	.LBB11_347
	shlq	$3, %r12
	negq	%r12
	movl	$4, %r8d
	movl	$3, %ebp
	movl	$2, %ecx
	movl	$1, %esi
	xorl	%edi, %edi
	.p2align	4
.LBB11_47:
	movq	%rbp, %rdx
	mulxq	%rbx, %r9, %r9
	movq	%rcx, %rdx
	mulxq	%rbx, %r10, %r10
	movq	%rsi, %rdx
	mulxq	%rbx, %r11, %r11
	movq	%rdi, %rdx
	mulxq	%rbx, %rdx, %rdx
	movq	%rdi, %rbx
	subq	%rdx, %rbx
	shrq	%rbx
	addq	%rdx, %rbx
	shrq	$4, %rbx
	movq	%rbx, %rdx
	shlq	$5, %rdx
	subq	%rdx, %rbx
	leaq	(%r8,%rbx), %rdx
	addq	$-19, %rdx
	vcvtsi2ss	%rdx, %xmm15, %xmm0
	leaq	-3(%r8), %rbx
	vmovss	%xmm0, -16(%rax,%r8,4)
	movq	112(%rsp), %rdx
	cmpq	%rdx, %rbx
	jae	.LBB11_52
	movq	%rsi, %rdx
	subq	%r11, %rdx
	shrq	%rdx
	addq	%r11, %rdx
	shrq	$4, %rdx
	movq	%rdx, %r11
	shlq	$5, %r11
	negq	%r11
	addq	%r11, %rdx
	addq	$-14, %rdx
	leaq	-4(%r8,%rdx), %rdx
	vcvtsi2ss	%rdx, %xmm15, %xmm0
	leaq	-2(%r8), %r11
	vmovss	%xmm0, -12(%rax,%r8,4)
	movq	112(%rsp), %rdx
	cmpq	%rdx, %r11
	jae	.LBB11_52
	movq	%rcx, %rdx
	subq	%r10, %rdx
	shrq	%rdx
	addq	%r10, %rdx
	shrq	$4, %rdx
	movq	%rdx, %r10
	shlq	$5, %r10
	negq	%r10
	addq	%r10, %rdx
	addq	$-13, %rdx
	leaq	-4(%r8,%rdx), %rdx
	vcvtsi2ss	%rdx, %xmm15, %xmm0
	leaq	-1(%r8), %r10
	vmovss	%xmm0, -8(%rax,%r8,4)
	movq	112(%rsp), %rdx
	cmpq	%rdx, %r10
	jae	.LBB11_52
	movq	%rbp, %rdx
	subq	%r9, %rdx
	shrq	%rdx
	addq	%r9, %rdx
	shrq	$4, %rdx
	movq	%rdx, %r9
	shlq	$5, %r9
	negq	%r9
	addq	%r9, %rdx
	addq	$-12, %rdx
	leaq	-4(%r8,%rdx), %rdx
	vcvtsi2ss	%rdx, %xmm15, %xmm0
	vmovss	%xmm0, -4(%rax,%r8,4)
	movq	112(%rsp), %rdx
	cmpq	%rdx, %r8
	jae	.LBB11_52
	addq	$4, %rbp
	addq	$4, %rcx
	addq	$4, %rsi
	leaq	(%r12,%r8), %rdx
	addq	$4, %rdx
	addq	$4, %r8
	addq	$4, %rdi
	cmpq	$4, %rdx
	movabsq	$595056260442243601, %rbx
	jne	.LBB11_47
	jmp	.LBB11_347
	.p2align	4
.LBB11_52:
	testq	%rdx, %rdx
	movq	_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip), %rbp
	movabsq	$595056260442243601, %r9
	je	.LBB11_80
	xorl	%r8d, %r8d
	.p2align	4
.LBB11_54:
	movq	144(%rsp), %rdi
	addq	%r8, %rdi
	cmpq	40(%rsp), %rdi
	jae	.LBB11_348
	addq	%r8, %rdx
	mulxq	%r9, %rcx, %rcx
	movq	%rdx, %rsi
	subq	%rcx, %rsi
	shrq	%rsi
	addq	%rcx, %rsi
	shrq	$4, %rsi
	movq	%rsi, %rcx
	shlq	$5, %rcx
	subq	%rcx, %rsi
	leaq	(%rdx,%rsi), %rcx
	addq	$-15, %rcx
	vcvtsi2ss	%rcx, %xmm15, %xmm0
	incq	%r8
	vmovss	%xmm0, (%rax,%rdi,4)
	movq	112(%rsp), %rdx
	cmpq	%rdx, %r8
	jb	.LBB11_54
	testq	%rdx, %rdx
	je	.LBB11_80
	xorl	%r8d, %r8d
	.p2align	4
.LBB11_58:
	movq	144(%rsp), %rcx
	leaq	(%r8,%rcx,2), %rdi
	cmpq	40(%rsp), %rdi
	jae	.LBB11_348
	leaq	(%r8,%rdx,2), %rdx
	mulxq	%r9, %rsi, %rsi
	movq	%rdx, %rdi
	subq	%rsi, %rdi
	shrq	%rdi
	addq	%rsi, %rdi
	shrq	$4, %rdi
	movq	%rdi, %rsi
	shlq	$5, %rsi
	subq	%rsi, %rdi
	addq	%rdi, %rdx
	addq	$-15, %rdx
	vcvtsi2ss	%rdx, %xmm15, %xmm0
	leaq	(%rax,%rcx,8), %rcx
	vmovss	%xmm0, (%rcx,%r8,4)
	incq	%r8
	movq	112(%rsp), %rdx
	cmpq	%rdx, %r8
	jb	.LBB11_58
	testq	%rdx, %rdx
	je	.LBB11_80
	xorl	%r8d, %r8d
	.p2align	4
.LBB11_62:
	movq	144(%rsp), %rcx
	leaq	(%rcx,%rcx,2), %rcx
	leaq	(%r8,%rcx), %rdi
	cmpq	40(%rsp), %rdi
	jae	.LBB11_348
	leaq	(%rdx,%rdx,2), %rsi
	leaq	(%r8,%rsi), %rdx
	mulxq	%r9, %rdi, %rdi
	subq	%rdi, %rdx
	shrq	%rdx
	addq	%rdi, %rdx
	shrq	$4, %rdx
	movq	%rdx, %rdi
	shlq	$5, %rdi
	subq	%rdi, %rdx
	addq	%rsi, %rdx
	addq	%r8, %rdx
	addq	$-15, %rdx
	vcvtsi2ss	%rdx, %xmm15, %xmm0
	leaq	(%rax,%rcx,4), %rcx
	vmovss	%xmm0, (%rcx,%r8,4)
	incq	%r8
	movq	112(%rsp), %rdx
	cmpq	%rdx, %r8
	jb	.LBB11_62
	testq	%rdx, %rdx
	je	.LBB11_80
	xorl	%r8d, %r8d
	.p2align	4
.LBB11_66:
	movq	144(%rsp), %rcx
	leaq	(%r8,%rcx,4), %rdi
	cmpq	40(%rsp), %rdi
	jae	.LBB11_348
	leaq	(%r8,%rdx,4), %rdx
	mulxq	%r9, %rsi, %rsi
	movq	%rdx, %rdi
	subq	%rsi, %rdi
	shrq	%rdi
	addq	%rsi, %rdi
	shrq	$4, %rdi
	movq	%rdi, %rsi
	shlq	$5, %rsi
	subq	%rsi, %rdi
	addq	%rdi, %rdx
	addq	$-15, %rdx
	vcvtsi2ss	%rdx, %xmm15, %xmm0
	shlq	$4, %rcx
	addq	%rax, %rcx
	vmovss	%xmm0, (%rcx,%r8,4)
	incq	%r8
	movq	112(%rsp), %rdx
	cmpq	%rdx, %r8
	jb	.LBB11_66
	testq	%rdx, %rdx
	je	.LBB11_80
	xorl	%r8d, %r8d
	.p2align	4
.LBB11_70:
	movq	144(%rsp), %rcx
	leaq	(%rcx,%rcx,4), %rcx
	leaq	(%r8,%rcx), %rdi
	cmpq	40(%rsp), %rdi
	jae	.LBB11_348
	leaq	(%rdx,%rdx,4), %rsi
	leaq	(%r8,%rsi), %rdx
	mulxq	%r9, %rdi, %rdi
	subq	%rdi, %rdx
	shrq	%rdx
	addq	%rdi, %rdx
	shrq	$4, %rdx
	movq	%rdx, %rdi
	shlq	$5, %rdi
	subq	%rdi, %rdx
	addq	%rsi, %rdx
	addq	%r8, %rdx
	addq	$-15, %rdx
	vcvtsi2ss	%rdx, %xmm15, %xmm0
	leaq	(%rax,%rcx,4), %rcx
	vmovss	%xmm0, (%rcx,%r8,4)
	incq	%r8
	movq	112(%rsp), %rdx
	cmpq	%rdx, %r8
	jb	.LBB11_70
	testq	%rdx, %rdx
	je	.LBB11_80
	xorl	%r8d, %r8d
	.p2align	4
.LBB11_74:
	movq	144(%rsp), %rcx
	leaq	(%rcx,%rcx,2), %rcx
	leaq	(%r8,%rcx,2), %rdi
	cmpq	40(%rsp), %rdi
	jae	.LBB11_348
	addq	%rdx, %rdx
	leaq	(%rdx,%rdx,2), %rsi
	leaq	(%r8,%rsi), %rdx
	mulxq	%r9, %rdi, %rdi
	subq	%rdi, %rdx
	shrq	%rdx
	addq	%rdi, %rdx
	shrq	$4, %rdx
	movq	%rdx, %rdi
	shlq	$5, %rdi
	subq	%rdi, %rdx
	addq	%rsi, %rdx
	addq	%r8, %rdx
	addq	$-15, %rdx
	vcvtsi2ss	%rdx, %xmm15, %xmm0
	leaq	(%rax,%rcx,8), %rcx
	vmovss	%xmm0, (%rcx,%r8,4)
	incq	%r8
	movq	112(%rsp), %rdx
	cmpq	%rdx, %r8
	jb	.LBB11_74
	testq	%rdx, %rdx
	je	.LBB11_80
	xorl	%r8d, %r8d
	.p2align	4
.LBB11_78:
	movq	144(%rsp), %rcx
	leaq	(%r8,%rcx,8), %rdi
	subq	%rcx, %rdi
	cmpq	40(%rsp), %rdi
	jae	.LBB11_348
	leaq	(,%rdx,8), %rsi
	subq	%rdx, %rsi
	leaq	(%r8,%rsi), %rdx
	mulxq	%r9, %rdi, %rdi
	subq	%rdi, %rdx
	shrq	%rdx
	addq	%rdi, %rdx
	shrq	$4, %rdx
	movq	%rdx, %rdi
	shlq	$5, %rdi
	subq	%rdi, %rdx
	addq	%rsi, %rdx
	addq	%r8, %rdx
	addq	$-15, %rdx
	vcvtsi2ss	%rdx, %xmm15, %xmm0
	leaq	(%rcx,%rcx,8), %rdx
	leaq	(%rdx,%rdx,2), %rdx
	addq	%rax, %rcx
	addq	%rdx, %rcx
	vmovss	%xmm0, (%rcx,%r8,4)
	incq	%r8
	movq	112(%rsp), %rdx
	cmpq	%rdx, %r8
	jb	.LBB11_78
	.p2align	4
.LBB11_80:
	vmovaps	400(%rsp), %xmm0
	vmovups	%xmm0, 720(%rsp)
	vmovapd	688(%rsp), %xmm0
	vmovupd	%xmm0, 704(%rsp)
	vzeroupper
	callq	*%rbp
	movl	$96, %edi
	movl	$8, %esi
	movq	_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip), %rbx
	callq	*%rbx
	testq	%rax, %rax
	je	.LBB11_349
	movq	%rax, %rbp
	movq	%rax, 16(%rsp)
	cmpq	$0, 40(%rsp)
	je	.LBB11_84
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	168(%rsp), %r12
	movq	%r12, %rdi
	callq	*%rbx
	testq	%rax, %rax
	je	.LBB11_350
	movq	%rax, 8(%rsp)
	movq	%rax, %rdi
	movq	96(%rsp), %rsi
	movq	%r12, %rdx
	callq	*memcpy@GOTPCREL(%rip)
	jmp	.LBB11_85
.LBB11_84:
	movl	$4, %eax
	movq	%rax, 8(%rsp)
.LBB11_85:
	movq	112(%rsp), %rbx
	movq	%rbx, %r13
	shlq	$5, %r13
	movq	%rbx, %rax
	shrq	$59, %rax
	setne	%al
	movabsq	$9223372036854775804, %rcx
	cmpq	%rcx, %r13
	seta	%cl
	orb	%al, %cl
	jne	.LBB11_262
	testq	%r13, %r13
	je	.LBB11_89
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	%r13, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc19___rust_alloc_zeroed@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB11_352
	movq	%rbx, 48(%rsp)
	jmp	.LBB11_90
.LBB11_89:
	movl	$4, %eax
	movq	$0, 48(%rsp)
.LBB11_90:
	movq	144(%rsp), %rdx
	movq	208(%rsp), %rcx
.Ltmp8:
	xorl	%r12d, %r12d
	movq	8(%rsp), %rdi
	movq	40(%rsp), %rsi
	movq	%rax, %r13
	movq	%rax, %r8
	movq	%rbx, %r9
	callq	_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control15inverse_97_rows
.Ltmp9:
	movq	48(%rsp), %rsi
	testq	%rsi, %rsi
	movq	memcpy@GOTPCREL(%rip), %rbx
	je	.LBB11_93
	shlq	$5, %rsi
	movl	$4, %edx
	movq	%r13, %rdi
	callq	*%r14
.LBB11_93:
	movq	16(%rsp), %rax
	movq	40(%rsp), %rcx
	movq	%rcx, (%rax)
	movq	8(%rsp), %rdx
	movq	%rdx, 8(%rax)
	movq	%rcx, 16(%rax)
	testq	%rcx, %rcx
	je	.LBB11_96
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	168(%rsp), %r12
	movq	%r12, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB11_354
	movq	%rax, 8(%rsp)
	movq	%rax, %rdi
	movq	96(%rsp), %rsi
	movq	%r12, %rdx
	callq	*%rbx
	jmp	.LBB11_97
.LBB11_96:
	movl	$4, %eax
	movq	%rax, 8(%rsp)
.LBB11_97:
	movq	112(%rsp), %rbx
	movq	%rbx, %r13
	shlq	$5, %r13
	movabsq	$576460752303423487, %rax
	cmpq	%rax, %rbx
	seta	%al
	movabsq	$9223372036854775804, %rcx
	cmpq	%rcx, %r13
	seta	%cl
	orb	%al, %cl
	movl	$1, %r12d
	jne	.LBB11_265
	testq	%r13, %r13
	je	.LBB11_101
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	%r13, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc19___rust_alloc_zeroed@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB11_353
	movq	%rbx, 48(%rsp)
	jmp	.LBB11_102
.LBB11_101:
	movl	$4, %eax
	movq	$0, 48(%rsp)
.LBB11_102:
	movq	144(%rsp), %rdx
	movl	$1, %r12d
	movq	208(%rsp), %rcx
.Ltmp10:
	movq	8(%rsp), %rdi
	movq	40(%rsp), %rsi
	movq	%rax, %r13
	movq	%rax, %r8
	movq	%rbx, %r9
	callq	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel15inverse_97_rows
.Ltmp11:
	movq	48(%rsp), %rsi
	testq	%rsi, %rsi
	movq	memcpy@GOTPCREL(%rip), %rbx
	je	.LBB11_105
	shlq	$5, %rsi
	movl	$4, %edx
	movq	%r13, %rdi
	callq	*%r14
.LBB11_105:
	movq	16(%rsp), %rax
	movq	40(%rsp), %rcx
	movq	%rcx, 24(%rax)
	movq	8(%rsp), %rdx
	movq	%rdx, 32(%rax)
	movq	%rcx, 40(%rax)
	testq	%rcx, %rcx
	je	.LBB11_108
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	168(%rsp), %r12
	movq	%r12, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB11_355
	movq	%rax, 8(%rsp)
	movq	%rax, %rdi
	movq	96(%rsp), %rsi
	movq	%r12, %rdx
	callq	*%rbx
	jmp	.LBB11_109
.LBB11_108:
	movl	$4, %eax
	movq	%rax, 8(%rsp)
.LBB11_109:
	movq	112(%rsp), %rbx
	movq	%rbx, %r13
	shlq	$5, %r13
	movabsq	$576460752303423487, %rax
	cmpq	%rax, %rbx
	seta	%al
	movabsq	$9223372036854775804, %rcx
	cmpq	%rcx, %r13
	seta	%cl
	orb	%al, %cl
	movl	$2, %r12d
	jne	.LBB11_265
	testq	%r13, %r13
	je	.LBB11_113
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	%r13, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc19___rust_alloc_zeroed@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB11_353
	movq	%rbx, 48(%rsp)
	jmp	.LBB11_114
.LBB11_113:
	movl	$4, %eax
	movq	$0, 48(%rsp)
.LBB11_114:
	movq	144(%rsp), %rdx
	movl	$2, %r12d
	movq	208(%rsp), %rcx
.Ltmp12:
	movq	8(%rsp), %rdi
	movq	40(%rsp), %rsi
	movq	%rax, %r13
	movq	%rax, %r8
	movq	%rbx, %r9
	callq	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel15inverse_97_rows
.Ltmp13:
	movq	48(%rsp), %rsi
	testq	%rsi, %rsi
	movq	memcpy@GOTPCREL(%rip), %rbx
	je	.LBB11_117
	shlq	$5, %rsi
	movl	$4, %edx
	movq	%r13, %rdi
	callq	*%r14
.LBB11_117:
	movq	16(%rsp), %rax
	movq	40(%rsp), %rcx
	movq	%rcx, 48(%rax)
	movq	8(%rsp), %rdx
	movq	%rdx, 56(%rax)
	movq	%rcx, 64(%rax)
	testq	%rcx, %rcx
	je	.LBB11_120
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	168(%rsp), %r12
	movq	%r12, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB11_357
	movq	%rax, 8(%rsp)
	movq	%rax, %rdi
	movq	96(%rsp), %rsi
	movq	%r12, %rdx
	callq	*%rbx
	jmp	.LBB11_121
.LBB11_120:
	movl	$4, %eax
	movq	%rax, 8(%rsp)
.LBB11_121:
	movq	112(%rsp), %rbx
	movq	%rbx, %r13
	shlq	$5, %r13
	movabsq	$576460752303423487, %rax
	cmpq	%rax, %rbx
	seta	%al
	movabsq	$9223372036854775804, %rcx
	cmpq	%rcx, %r13
	seta	%cl
	orb	%al, %cl
	movl	$3, %r12d
	jne	.LBB11_265
	testq	%r13, %r13
	je	.LBB11_125
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	%r13, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc19___rust_alloc_zeroed@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB11_353
	movq	%rbx, 48(%rsp)
	jmp	.LBB11_126
.LBB11_125:
	movl	$4, %eax
	movq	$0, 48(%rsp)
.LBB11_126:
	movq	144(%rsp), %rdx
	movl	$3, %r12d
	movq	208(%rsp), %rcx
.Ltmp17:
	movq	8(%rsp), %rdi
	movq	40(%rsp), %rsi
	movq	%rax, %r13
	movq	%rax, %r8
	movq	%rbx, %r9
	callq	_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control15inverse_97_rows
.Ltmp18:
	movq	48(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB11_129
	shlq	$5, %rsi
	movl	$4, %edx
	movq	%r13, %rdi
	callq	*%r14
.LBB11_129:
	leaq	24(%rbp), %rax
	movq	%rax, 232(%rsp)
	addq	$48, %rbp
	movq	16(%rsp), %rbx
	movq	40(%rsp), %rax
	movq	%rax, 72(%rbx)
	movq	8(%rsp), %rcx
	movq	%rcx, 80(%rbx)
	movq	%rax, 88(%rbx)
	movq	16(%rbx), %r13
	cmpq	40(%rbx), %r13
	jne	.LBB11_312
	movq	8(%rbx), %rdi
	movq	32(%rbx), %rsi
	leaq	(,%r13,4), %r12
	movq	%rdi, 8(%rsp)
	movq	%r12, %rdx
	callq	*bcmp@GOTPCREL(%rip)
	testl	%eax, %eax
	jne	.LBB11_312
	cmpq	64(%rbx), %r13
	jne	.LBB11_285
	movq	56(%rbx), %rsi
	movq	8(%rsp), %rdi
	movq	%r12, %rdx
	callq	*bcmp@GOTPCREL(%rip)
	testl	%eax, %eax
	movq	_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip), %r12
	jne	.LBB11_285
	leaq	560(%rsp), %rax
	vxorpd	%xmm0, %xmm0, %xmm0
	vmovupd	%xmm0, (%rax)
	vmovupd	%xmm0, 24(%rax)
	vmovupd	%xmm0, 48(%rax)
	movq	$0, 544(%rsp)
	movq	$8, 552(%rsp)
	movq	$8, 576(%rsp)
	movq	$8, 600(%rsp)
	movq	$8, 624(%rsp)
	vmovaps	.LCPI11_3(%rip), %ymm0
	vmovups	%ymm0, 776(%rsp)
	vmovaps	.LCPI11_4(%rip), %ymm0
	vmovups	%ymm0, 808(%rsp)
	vmovaps	.LCPI11_5(%rip), %ymm0
	vmovups	%ymm0, 840(%rsp)
	vmovaps	.LCPI11_6(%rip), %ymm0
	vmovups	%ymm0, 872(%rsp)
	vmovaps	.LCPI11_7(%rip), %ymm0
	vmovups	%ymm0, 904(%rsp)
	movq	$0, 632(%rsp)
	vmovaps	.LCPI11_8(%rip), %ymm0
	vmovups	%ymm0, 936(%rsp)
	xorl	%eax, %eax
	leaq	304(%rsp), %rdx
	movq	memcpy@GOTPCREL(%rip), %rbx
.LBB11_134:
	movq	%rax, %rcx
	subq	$6, %rcx
	cmovbq	%rax, %rcx
	incq	%rax
	movq	%rax, 440(%rsp)
	shlq	$5, %rcx
	vmovupd	776(%rsp,%rcx), %ymm0
	vmovupd	%ymm0, (%rdx)
	movq	$4, 296(%rsp)
	xorl	%r13d, %r13d
.LBB11_135:
	movq	%r13, 136(%rsp)
	movq	304(%rsp,%r13,8), %rdi
	cmpq	$4, %rdi
	jae	.LBB11_356
	movq	704(%rsp,%rdi,8), %rax
	movq	%rax, 536(%rsp)
	movq	112(%rsp), %r13
	movq	144(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	208(%rsp), %rax
	movq	%rax, 528(%rsp)
	cmpq	$0, 40(%rsp)
	movq	%r13, 48(%rsp)
	movq	%rdi, 448(%rsp)
	je	.LBB11_139
	vzeroupper
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %esi
	movq	168(%rsp), %r13
	movq	%r13, %rdi
	callq	*%r12
	testq	%rax, %rax
	je	.LBB11_360
	movq	40(%rsp), %r12
	movq	%r12, 64(%rsp)
	movq	%rax, 72(%rsp)
	movq	%rax, %rdi
	movq	96(%rsp), %rsi
	movq	%r13, %rdx
	callq	*%rbx
	movq	48(%rsp), %r13
	jmp	.LBB11_140
.LBB11_139:
	movq	$0, 64(%rsp)
	movq	$4, 72(%rsp)
	movq	40(%rsp), %r12
.LBB11_140:
	movq	%r12, 80(%rsp)
	movq	%r13, %rbx
	shlq	$5, %rbx
	movabsq	$576460752303423487, %rax
	cmpq	%rax, %r13
	seta	%al
	movabsq	$9223372036854775804, %rcx
	cmpq	%rcx, %rbx
	seta	%cl
	orb	%al, %cl
	jne	.LBB11_267
	testq	%rbx, %rbx
	je	.LBB11_144
	vzeroupper
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$4, %r12d
	movl	$4, %esi
	movq	%rbx, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc19___rust_alloc_zeroed@GOTPCREL(%rip)
	movq	%rax, 248(%rsp)
	testq	%rax, %rax
	je	.LBB11_268
	movq	%r13, 32(%rsp)
	jmp	.LBB11_145
.LBB11_144:
	movl	$4, %eax
	movq	%rax, 248(%rsp)
	movq	$0, 32(%rsp)
.LBB11_145:
.Ltmp28:
	vzeroupper
	callq	*_RNvMNtCsh0g7Hz38UNf_3std4timeNtB2_7Instant3now@GOTPCREL(%rip)
.Ltmp29:
	incq	136(%rsp)
	movq	%rax, 640(%rsp)
	movl	%edx, 648(%rsp)
	cmpq	40(%rsp), %r13
	ja	.LBB11_314
	leaq	(,%r13,4), %r8
	movq	8(%rsp), %rax
	leaq	(%rax,%r13), %rcx
	cmpq	%rax, %rcx
	movq	%r8, 56(%rsp)
	movq	%rcx, 104(%rsp)
	jb	.LBB11_286
	cmpq	40(%rsp), %rcx
	ja	.LBB11_318
	movq	8(%rsp), %rcx
	leaq	(%rcx,%rcx), %rdx
	leaq	(,%rcx,2), %rsi
	addq	%r13, %rsi
	movq	96(%rsp), %rax
	leaq	(%rax,%rcx,4), %rax
	movq	%rax, 176(%rsp)
	movq	%rdx, 120(%rsp)
	cmpq	%rdx, %rsi
	movq	%rsi, 128(%rsp)
	jb	.LBB11_289
	cmpq	40(%rsp), %rsi
	ja	.LBB11_321
	movq	8(%rsp), %rcx
	leaq	(%rcx,%rcx,2), %rdx
	leaq	(%rdx,%r13), %rsi
	movq	96(%rsp), %rax
	leaq	(%rax,%rcx,8), %rax
	movq	%rax, 200(%rsp)
	movq	%rdx, 152(%rsp)
	cmpq	%rdx, %rsi
	movq	%rsi, 160(%rsp)
	jb	.LBB11_293
	cmpq	40(%rsp), %rsi
	ja	.LBB11_326
	movq	8(%rsp), %rax
	leaq	(,%rax,4), %rdx
	leaq	(,%rax,4), %rsi
	addq	%r13, %rsi
	movq	96(%rsp), %rax
	movq	152(%rsp), %rcx
	leaq	(%rax,%rcx,4), %rax
	movq	%rax, 216(%rsp)
	movq	%rdx, 184(%rsp)
	movq	%rsi, %rax
	cmpq	%rdx, %rsi
	jb	.LBB11_298
	movq	%rax, 192(%rsp)
	cmpq	40(%rsp), %rax
	ja	.LBB11_332
	movq	8(%rsp), %rax
	leaq	(%rax,%rax,4), %rdx
	leaq	(%rdx,%r13), %rcx
	shlq	$4, %rax
	addq	96(%rsp), %rax
	movq	%rax, 264(%rsp)
	movq	%rdx, 224(%rsp)
	cmpq	%rdx, %rcx
	jb	.LBB11_303
	movq	%rcx, 240(%rsp)
	cmpq	40(%rsp), %rcx
	ja	.LBB11_339
	movq	120(%rsp), %rax
	leaq	(%rax,%rax,2), %rdx
	leaq	(%rdx,%r13), %r12
	movq	8(%rsp), %rax
	leaq	(,%rax,8), %rsi
	subq	%rax, %rsi
	addq	%rsi, %r13
	movq	96(%rsp), %rax
	movq	224(%rsp), %rcx
	leaq	(%rax,%rcx,4), %rcx
	movq	%rcx, 512(%rsp)
	movq	%rdx, 520(%rsp)
	leaq	(%rax,%rdx,4), %rcx
	movq	%rcx, 488(%rsp)
	movq	%rsi, 504(%rsp)
	leaq	(%rax,%rsi,4), %rax
	movq	%rax, 480(%rsp)
	movl	$10000, %eax
	movq	%r13, 496(%rsp)
	.p2align	4
.LBB11_158:
	movq	80(%rsp), %rdx
	cmpq	%rdx, 48(%rsp)
	ja	.LBB11_270
	movl	%eax, 284(%rsp)
	movq	%r12, %r13
	movq	72(%rsp), %rdi
	movq	96(%rsp), %rsi
	movq	%r8, %rdx
	movq	%r8, %r12
	movq	memcpy@GOTPCREL(%rip), %rbx
	callq	*%rbx
	movq	80(%rsp), %rdx
	movq	104(%rsp), %rsi
	cmpq	%rdx, %rsi
	ja	.LBB11_277
	movq	8(%rsp), %rax
	leaq	(,%rax,4), %rdi
	addq	72(%rsp), %rdi
	movq	176(%rsp), %rsi
	movq	%r12, %rdx
	callq	*%rbx
	movq	80(%rsp), %rdx
	movq	128(%rsp), %rsi
	cmpq	%rdx, %rsi
	ja	.LBB11_271
	movq	120(%rsp), %rax
	leaq	(,%rax,4), %rdi
	addq	72(%rsp), %rdi
	movq	200(%rsp), %rsi
	movq	%r12, %rdx
	callq	*%rbx
	movq	80(%rsp), %rdx
	movq	160(%rsp), %rsi
	cmpq	%rdx, %rsi
	ja	.LBB11_272
	movq	152(%rsp), %rax
	leaq	(,%rax,4), %rdi
	addq	72(%rsp), %rdi
	movq	216(%rsp), %rsi
	movq	%r12, %rdx
	callq	*%rbx
	movq	80(%rsp), %rdx
	cmpq	%rdx, 192(%rsp)
	ja	.LBB11_273
	movq	184(%rsp), %rax
	leaq	(,%rax,4), %rdi
	addq	72(%rsp), %rdi
	movq	264(%rsp), %rsi
	movq	%r12, %rdx
	callq	*%rbx
	movq	80(%rsp), %rdx
	cmpq	%rdx, 240(%rsp)
	ja	.LBB11_274
	movq	224(%rsp), %rax
	leaq	(,%rax,4), %rdi
	addq	72(%rsp), %rdi
	movq	512(%rsp), %rsi
	movq	%r12, %rdx
	callq	*%rbx
	movq	80(%rsp), %rdx
	movq	%r13, %r12
	movq	520(%rsp), %rcx
	cmpq	%rcx, %r13
	jb	.LBB11_275
	cmpq	%rdx, %r12
	ja	.LBB11_275
	cmpq	40(%rsp), %r12
	ja	.LBB11_282
	movq	72(%rsp), %rax
	leaq	(%rax,%rcx,4), %rdi
	movq	488(%rsp), %rsi
	movq	56(%rsp), %rbx
	movq	%rbx, %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	movq	504(%rsp), %rcx
	movq	496(%rsp), %r13
	cmpq	%rcx, %r13
	jb	.LBB11_276
	cmpq	%rdx, %r13
	ja	.LBB11_276
	cmpq	40(%rsp), %r13
	ja	.LBB11_283
	movq	72(%rsp), %rax
	leaq	(%rax,%rcx,4), %rdi
	movq	480(%rsp), %rsi
	movq	%rbx, %rdx
	callq	*memcpy@GOTPCREL(%rip)
	vmovupd	72(%rsp), %xmm0
	vmovupd	%xmm0, 424(%rsp)
	leaq	424(%rsp), %rbx
	#APP
	#NO_APP
	movq	8(%rsp), %rax
	movq	%rax, 416(%rsp)
	leaq	416(%rsp), %rax
	#APP
	#NO_APP
	movq	424(%rsp), %rdi
	movq	432(%rsp), %rsi
	movq	416(%rsp), %rdx
.Ltmp30:
	movq	528(%rsp), %rcx
	movq	248(%rsp), %r8
	movq	48(%rsp), %r9
	callq	*536(%rsp)
.Ltmp31:
	leaq	64(%rsp), %rax
	movq	%rax, 424(%rsp)
	#APP
	#NO_APP
	movl	284(%rsp), %eax
	decl	%eax
	movq	56(%rsp), %r8
	jne	.LBB11_158
.Ltmp33:
	leaq	640(%rsp), %rdi
	callq	*_RNvMNtCsh0g7Hz38UNf_3std4timeNtB2_7Instant7elapsed@GOTPCREL(%rip)
.Ltmp34:
	movq	%rax, %rbx
	movl	%edx, %r12d
	movq	32(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB11_175
	shlq	$5, %rsi
	movl	$4, %edx
	movq	248(%rsp), %rdi
	callq	*%r14
.LBB11_175:
	movq	64(%rsp), %rsi
	testq	%rsi, %rsi
	movq	136(%rsp), %r13
	je	.LBB11_177
	movq	72(%rsp), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*%r14
.LBB11_177:
	vcvtsi2sd	%r12d, %xmm15, %xmm0
	vdivsd	.LCPI11_9(%rip), %xmm0, %xmm0
	vmovq	%rbx, %xmm1
	vpunpckldq	.LCPI11_10(%rip), %xmm1, %xmm1
	vsubpd	.LCPI11_11(%rip), %xmm1, %xmm1
	vshufpd	$1, %xmm1, %xmm1, %xmm2
	vaddsd	%xmm1, %xmm2, %xmm1
	vaddsd	%xmm1, %xmm0, %xmm0
	vmulsd	.LCPI11_12(%rip), %xmm0, %xmm0
	vdivsd	.LCPI11_13(%rip), %xmm0, %xmm0
	vmovsd	%xmm0, 8(%rsp)
	movq	448(%rsp), %rax
	leaq	(%rax,%rax,2), %rax
	leaq	(%rsp,%rax,8), %rbx
	addq	$544, %rbx
	movq	560(%rsp,%rax,8), %r12
	cmpq	544(%rsp,%rax,8), %r12
	jne	.LBB11_179
.Ltmp36:
	movq	%rbx, %rdi
	callq	_RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCs5dlvf2fXWpN_16outlined_io_main
.Ltmp37:
.LBB11_179:
	movq	8(%rbx), %rax
	vmovsd	8(%rsp), %xmm0
	vmovsd	%xmm0, (%rax,%r12,8)
	incq	%r12
	movq	%r12, 16(%rbx)
	cmpq	$4, %r13
	leaq	304(%rsp), %rdx
	movq	memcpy@GOTPCREL(%rip), %rbx
	movq	_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip), %r12
	jne	.LBB11_135
	movq	440(%rsp), %rax
	cmpq	$12, %rax
	jne	.LBB11_134
	vmovupd	544(%rsp), %ymm0
	vmovdqu	576(%rsp), %ymm1
	vmovupd	608(%rsp), %ymm2
	vmovupd	%ymm0, 288(%rsp)
	vmovupd	%ymm2, 352(%rsp)
	vmovdqu	%ymm1, 320(%rsp)
	movq	288(%rsp), %rbx
	movq	296(%rsp), %r12
	movq	304(%rsp), %r13
	cmpq	$2, %r13
	jae	.LBB11_209
	testq	%r13, %r13
	je	.LBB11_368
	xorl	%r13d, %r13d
	vmovsd	(%r12,%r13,8), %xmm0
	vmovsd	%xmm0, 40(%rsp)
	testq	%rbx, %rbx
	je	.LBB11_185
.LBB11_184:
	shlq	$3, %rbx
	movl	$8, %edx
	movq	%r12, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*%r14
.LBB11_185:
	movq	312(%rsp), %rbx
	movq	320(%rsp), %r12
	movq	328(%rsp), %r13
	cmpq	$2, %r13
	jae	.LBB11_219
	testq	%r13, %r13
	je	.LBB11_369
	xorl	%r13d, %r13d
	vmovsd	(%r12,%r13,8), %xmm0
	vmovsd	%xmm0, 168(%rsp)
	testq	%rbx, %rbx
	je	.LBB11_189
.LBB11_188:
	shlq	$3, %rbx
	movl	$8, %edx
	movq	%r12, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*%r14
.LBB11_189:
	movq	336(%rsp), %rbx
	movq	344(%rsp), %r12
	movq	352(%rsp), %r13
	cmpq	$2, %r13
	jae	.LBB11_229
	testq	%r13, %r13
	je	.LBB11_370
	xorl	%r13d, %r13d
	vmovsd	(%r12,%r13,8), %xmm0
	vmovsd	%xmm0, 8(%rsp)
	testq	%rbx, %rbx
	je	.LBB11_193
.LBB11_192:
	shlq	$3, %rbx
	movl	$8, %edx
	movq	%r12, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*%r14
.LBB11_193:
	movq	360(%rsp), %rbx
	movq	368(%rsp), %r12
	movq	376(%rsp), %r13
	cmpq	$2, %r13
	jae	.LBB11_239
	testq	%r13, %r13
	je	.LBB11_371
	xorl	%r13d, %r13d
	vmovsd	(%r12,%r13,8), %xmm0
	vmovsd	%xmm0, 48(%rsp)
	testq	%rbx, %rbx
	je	.LBB11_197
.LBB11_196:
	shlq	$3, %rbx
	movl	$8, %edx
	movq	%r12, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*%r14
.LBB11_197:
	vmovsd	40(%rsp), %xmm0
	vmovsd	%xmm0, 64(%rsp)
	vmovsd	168(%rsp), %xmm0
	vmovsd	%xmm0, 72(%rsp)
	vmovsd	8(%rsp), %xmm0
	vmovsd	%xmm0, 80(%rsp)
	vmovsd	48(%rsp), %xmm0
	vmovsd	%xmm0, 88(%rsp)
	leaq	112(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	_RNvXsi_NtNtNtCs8uuN1nrGffB_4core3fmt3num3impjNtB9_7Display3fmt@GOTPCREL(%rip), %rax
	movq	%rax, 296(%rsp)
	leaq	144(%rsp), %rcx
	movq	%rcx, 304(%rsp)
	movq	%rax, 312(%rsp)
	leaq	208(%rsp), %rcx
	movq	%rcx, 320(%rsp)
	movq	%rax, 328(%rsp)
	leaq	64(%rsp), %rax
	movq	%rax, 336(%rsp)
	movq	_RNvXs7_NtNtCs8uuN1nrGffB_4core3fmt5floatdNtB7_7Display3fmt@GOTPCREL(%rip), %rcx
	movq	%rcx, 344(%rsp)
	leaq	72(%rsp), %rax
	movq	%rax, 352(%rsp)
	movq	%rcx, 360(%rsp)
	leaq	80(%rsp), %rax
	movq	%rax, 368(%rsp)
	movq	%rcx, 376(%rsp)
	leaq	88(%rsp), %rax
	movq	%rax, 384(%rsp)
	movq	%rcx, 392(%rsp)
.Ltmp48:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.23(%rip), %rdi
	leaq	288(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtNtCsh0g7Hz38UNf_3std2io5stdio6__print@GOTPCREL(%rip)
.Ltmp49:
	movq	16(%rsp), %rax
	movq	(%rax), %rsi
	testq	%rsi, %rsi
	je	.LBB11_200
	movq	16(%rsp), %rax
	movq	8(%rax), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*%r14
.LBB11_200:
	movq	232(%rsp), %rax
	movq	(%rax), %rsi
	testq	%rsi, %rsi
	je	.LBB11_202
	movq	16(%rsp), %rax
	movq	32(%rax), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*%r14
.LBB11_202:
	movq	(%rbp), %rsi
	testq	%rsi, %rsi
	je	.LBB11_204
	movq	16(%rsp), %rax
	movq	56(%rax), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*%r14
.LBB11_204:
	movq	16(%rsp), %rax
	movq	72(%rax), %rsi
	testq	%rsi, %rsi
	je	.LBB11_206
	movq	16(%rsp), %rax
	movq	80(%rax), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*%r14
.LBB11_206:
	movl	$96, %esi
	movl	$8, %edx
	movq	16(%rsp), %rdi
	callq	*%r14
	cmpq	$0, 24(%rsp)
	je	.LBB11_208
	movq	24(%rsp), %rsi
	shlq	$2, %rsi
	movl	$4, %edx
	movq	96(%rsp), %rdi
	callq	*%r14
.LBB11_208:
	movl	$1, %eax
	cmpq	$2, 456(%rsp)
	movq	256(%rsp), %r13
	movq	472(%rsp), %rbx
	movq	_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip), %rbp
	movq	464(%rsp), %rdx
	jne	.LBB11_28
	jmp	.LBB11_257
.LBB11_209:
	cmpq	$21, %r13
	jae	.LBB11_249
	leaq	(%r12,%r13,8), %rax
	leaq	8(%r12), %rsi
	xorl	%ecx, %ecx
	movq	%r12, %r8
	jmp	.LBB11_214
.LBB11_211:
	addq	%r12, %r9
	movq	%r9, %r10
.LBB11_212:
	movq	%rsi, (%r10)
.LBB11_213:
	leaq	8(%rdx), %rsi
	addq	$8, %rcx
	movq	%rdx, %r8
	cmpq	%rax, %rsi
	je	.LBB11_250
.LBB11_214:
	movq	%rsi, %rdx
	movq	(%r8), %r9
	movq	8(%r8), %rsi
	movq	%rsi, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%rsi, %rdi
	movq	%r9, %r10
	sarq	$63, %r10
	shrq	%r10
	xorq	%r9, %r10
	cmpq	%r10, %rdi
	jge	.LBB11_213
	movq	%r9, 8(%r8)
	movq	%rcx, %r9
	movq	%r12, %r10
	cmpq	%r12, %r8
	je	.LBB11_212
.LBB11_216:
	movq	-8(%r12,%r9), %r8
	movq	%r8, %r10
	sarq	$63, %r10
	shrq	%r10
	xorq	%r8, %r10
	cmpq	%r10, %rdi
	jge	.LBB11_211
	movq	%r8, (%r12,%r9)
	addq	$-8, %r9
	jne	.LBB11_216
	movq	%r12, %r10
	jmp	.LBB11_212
.LBB11_219:
	cmpq	$21, %r13
	jae	.LBB11_251
	leaq	(%r12,%r13,8), %rax
	leaq	8(%r12), %rsi
	xorl	%ecx, %ecx
	movq	%r12, %r8
	jmp	.LBB11_224
.LBB11_221:
	addq	%r12, %r9
	movq	%r9, %r10
.LBB11_222:
	movq	%rsi, (%r10)
.LBB11_223:
	leaq	8(%rdx), %rsi
	addq	$8, %rcx
	movq	%rdx, %r8
	cmpq	%rax, %rsi
	je	.LBB11_252
.LBB11_224:
	movq	%rsi, %rdx
	movq	(%r8), %r9
	movq	8(%r8), %rsi
	movq	%rsi, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%rsi, %rdi
	movq	%r9, %r10
	sarq	$63, %r10
	shrq	%r10
	xorq	%r9, %r10
	cmpq	%r10, %rdi
	jge	.LBB11_223
	movq	%r9, 8(%r8)
	movq	%rcx, %r9
	movq	%r12, %r10
	cmpq	%r12, %r8
	je	.LBB11_222
.LBB11_226:
	movq	-8(%r12,%r9), %r8
	movq	%r8, %r10
	sarq	$63, %r10
	shrq	%r10
	xorq	%r8, %r10
	cmpq	%r10, %rdi
	jge	.LBB11_221
	movq	%r8, (%r12,%r9)
	addq	$-8, %r9
	jne	.LBB11_226
	movq	%r12, %r10
	jmp	.LBB11_222
.LBB11_229:
	cmpq	$21, %r13
	jae	.LBB11_253
	leaq	(%r12,%r13,8), %rax
	leaq	8(%r12), %rsi
	xorl	%ecx, %ecx
	movq	%r12, %r8
	jmp	.LBB11_234
.LBB11_231:
	addq	%r12, %r9
	movq	%r9, %r10
.LBB11_232:
	movq	%rsi, (%r10)
.LBB11_233:
	leaq	8(%rdx), %rsi
	addq	$8, %rcx
	movq	%rdx, %r8
	cmpq	%rax, %rsi
	je	.LBB11_254
.LBB11_234:
	movq	%rsi, %rdx
	movq	(%r8), %r9
	movq	8(%r8), %rsi
	movq	%rsi, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%rsi, %rdi
	movq	%r9, %r10
	sarq	$63, %r10
	shrq	%r10
	xorq	%r9, %r10
	cmpq	%r10, %rdi
	jge	.LBB11_233
	movq	%r9, 8(%r8)
	movq	%rcx, %r9
	movq	%r12, %r10
	cmpq	%r12, %r8
	je	.LBB11_232
.LBB11_236:
	movq	-8(%r12,%r9), %r8
	movq	%r8, %r10
	sarq	$63, %r10
	shrq	%r10
	xorq	%r8, %r10
	cmpq	%r10, %rdi
	jge	.LBB11_231
	movq	%r8, (%r12,%r9)
	addq	$-8, %r9
	jne	.LBB11_236
	movq	%r12, %r10
	jmp	.LBB11_232
.LBB11_239:
	cmpq	$21, %r13
	jae	.LBB11_255
	leaq	(%r12,%r13,8), %rax
	leaq	8(%r12), %rsi
	xorl	%ecx, %ecx
	movq	%r12, %r8
	jmp	.LBB11_244
.LBB11_241:
	addq	%r12, %r9
	movq	%r9, %r10
.LBB11_242:
	movq	%rsi, (%r10)
.LBB11_243:
	leaq	8(%rdx), %rsi
	addq	$8, %rcx
	movq	%rdx, %r8
	cmpq	%rax, %rsi
	je	.LBB11_256
.LBB11_244:
	movq	%rsi, %rdx
	movq	(%r8), %r9
	movq	8(%r8), %rsi
	movq	%rsi, %rdi
	sarq	$63, %rdi
	shrq	%rdi
	xorq	%rsi, %rdi
	movq	%r9, %r10
	sarq	$63, %r10
	shrq	%r10
	xorq	%r9, %r10
	cmpq	%r10, %rdi
	jge	.LBB11_243
	movq	%r9, 8(%r8)
	movq	%rcx, %r9
	movq	%r12, %r10
	cmpq	%r12, %r8
	je	.LBB11_242
.LBB11_246:
	movq	-8(%r12,%r9), %r8
	movq	%r8, %r10
	sarq	$63, %r10
	shrq	%r10
	xorq	%r8, %r10
	cmpq	%r10, %rdi
	jge	.LBB11_241
	movq	%r8, (%r12,%r9)
	addq	$-8, %r9
	jne	.LBB11_246
	movq	%r12, %r10
	jmp	.LBB11_242
.LBB11_249:
	movl	$24, %eax
	movq	%rax, 48(%rsp)
.Ltmp39:
	movq	%r12, %rdi
	movq	%r13, %rsi
	vzeroupper
	callq	_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECs5dlvf2fXWpN_16outlined_io_main
.Ltmp40:
.LBB11_250:
	shrq	%r13
	vmovsd	(%r12,%r13,8), %xmm0
	vmovsd	%xmm0, 40(%rsp)
	testq	%rbx, %rbx
	jne	.LBB11_184
	jmp	.LBB11_185
.LBB11_251:
	movl	$48, %eax
	movq	%rax, 48(%rsp)
.Ltmp41:
	movq	%r12, %rdi
	movq	%r13, %rsi
	vzeroupper
	callq	_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECs5dlvf2fXWpN_16outlined_io_main
.Ltmp42:
.LBB11_252:
	shrq	%r13
	vmovsd	(%r12,%r13,8), %xmm0
	vmovsd	%xmm0, 168(%rsp)
	testq	%rbx, %rbx
	jne	.LBB11_188
	jmp	.LBB11_189
.LBB11_253:
	movl	$72, %eax
	movq	%rax, 48(%rsp)
.Ltmp43:
	movq	%r12, %rdi
	movq	%r13, %rsi
	vzeroupper
	callq	_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECs5dlvf2fXWpN_16outlined_io_main
.Ltmp44:
.LBB11_254:
	shrq	%r13
	vmovsd	(%r12,%r13,8), %xmm0
	vmovsd	%xmm0, 8(%rsp)
	testq	%rbx, %rbx
	jne	.LBB11_192
	jmp	.LBB11_193
.LBB11_255:
	movl	$96, %eax
	movq	%rax, 48(%rsp)
.Ltmp45:
	movq	%r12, %rdi
	movq	%r13, %rsi
	vzeroupper
	callq	_RINvNtNtNtCs8uuN1nrGffB_4core5slice4sort6stable14driftsort_maindNCINvMNtCs3BN1jqgSbDf_5alloc5sliceSd7sort_byNvMNtB8_3f64d9total_cmpE0INtNtB18_3vec3VecdEECs5dlvf2fXWpN_16outlined_io_main
.Ltmp46:
.LBB11_256:
	shrq	%r13
	vmovsd	(%r12,%r13,8), %xmm0
	vmovsd	%xmm0, 48(%rsp)
	testq	%rbx, %rbx
	jne	.LBB11_196
	jmp	.LBB11_197
.LBB11_257:
	movq	112(%rsp), %rax
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.21(%rip), %r12
	jmp	.LBB11_25
	.p2align	4
.LBB11_258:
	movl	$32, %esi
	movl	$8, %edx
	movq	%r13, %rdi
	callq	*%r14
	cmpq	$3, %rbx
	movq	_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip), %r15
	jne	.LBB11_1
	addq	$968, %rsp
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
.LBB11_260:
	.cfi_def_cfa_offset 1024
	xorl	%ebx, %ebx
.LBB11_261:
.Ltmp66:
	movq	%rbx, %rdi
	movq	168(%rsp), %rsi
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.Ltmp67:
	jmp	.LBB11_373
.LBB11_262:
	xorl	%r12d, %r12d
.LBB11_265:
	xorl	%edi, %edi
.LBB11_266:
.Ltmp60:
	movq	%r13, %rsi
	movq	16(%rsp), %rbx
	movq	40(%rsp), %r15
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.Ltmp61:
	jmp	.LBB11_373
.LBB11_267:
	xorl	%r12d, %r12d
.LBB11_268:
.Ltmp57:
	movq	%r12, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.Ltmp58:
	jmp	.LBB11_373
.LBB11_269:
	movl	$8, %edi
	movl	$32, %esi
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc5alloc18handle_alloc_error@GOTPCREL(%rip)
.LBB11_270:
	movq	$0, 8(%rsp)
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rcx
	movq	48(%rsp), %rsi
	jmp	.LBB11_278
.LBB11_271:
	movq	120(%rsp), %rax
	movq	%rax, 8(%rsp)
	jmp	.LBB11_277
.LBB11_272:
	movq	152(%rsp), %rax
	movq	%rax, 8(%rsp)
	jmp	.LBB11_277
.LBB11_273:
	movq	184(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	192(%rsp), %rsi
	jmp	.LBB11_277
.LBB11_274:
	movq	224(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	240(%rsp), %rsi
	jmp	.LBB11_277
.LBB11_275:
	movq	%rcx, 8(%rsp)
	movq	%r12, %rsi
	jmp	.LBB11_277
.LBB11_276:
	movq	%rcx, 8(%rsp)
	movq	%r13, %rsi
.LBB11_277:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rcx
.LBB11_278:
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
.LBB11_279:
	movq	24(%rsp), %r14
.LBB11_280:
	movq	16(%rsp), %r15
	movq	32(%rsp), %rbx
.LBB11_281:
.Ltmp54:
	movq	8(%rsp), %rdi
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp55:
	jmp	.LBB11_373
.LBB11_282:
	movq	%rcx, 8(%rsp)
	movq	%r12, %rsi
	jmp	.LBB11_284
.LBB11_283:
	movq	%rcx, 8(%rsp)
	movq	%r13, %rsi
.LBB11_284:
	movq	40(%rsp), %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.26(%rip), %rcx
	jmp	.LBB11_278
.LBB11_312:
	movq	232(%rsp), %rsi
	jmp	.LBB11_313
.LBB11_285:
	movq	%rbp, %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.22(%rip), %rax
	movq	%rax, 272(%rsp)
.LBB11_313:
.Ltmp20:
	movq	16(%rsp), %rdi
	movq	272(%rsp), %rdx
	callq	_RINvNtCs8uuN1nrGffB_4core9panicking13assert_failedINtNtCs3BN1jqgSbDf_5alloc3vec3VeclEBM_ECs5dlvf2fXWpN_16outlined_io_main
.Ltmp21:
	jmp	.LBB11_373
.LBB11_286:
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	80(%rsp), %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rbx
	cmpq	%rdx, %r13
	ja	.LBB11_351
	movq	72(%rsp), %rdi
	movq	96(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
.LBB11_288:
	movq	104(%rsp), %rsi
	jmp	.LBB11_310
.LBB11_289:
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	80(%rsp), %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rbx
	cmpq	%rdx, %r13
	ja	.LBB11_351
	movq	72(%rsp), %rdi
	movq	96(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 104(%rsp)
	ja	.LBB11_288
	movq	8(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	176(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
.LBB11_292:
	movq	120(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	128(%rsp), %rsi
	jmp	.LBB11_310
.LBB11_293:
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	80(%rsp), %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rbx
	cmpq	%rdx, %r13
	ja	.LBB11_351
	movq	72(%rsp), %rdi
	movq	96(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 104(%rsp)
	ja	.LBB11_288
	movq	8(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	176(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 128(%rsp)
	ja	.LBB11_292
	movq	120(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	200(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
.LBB11_297:
	movq	152(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	160(%rsp), %rsi
	jmp	.LBB11_310
.LBB11_298:
	movq	%rax, %r14
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	80(%rsp), %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rbx
	cmpq	%rdx, %r13
	ja	.LBB11_351
	movq	72(%rsp), %rdi
	movq	96(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 104(%rsp)
	ja	.LBB11_288
	movq	8(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	176(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 128(%rsp)
	ja	.LBB11_292
	movq	120(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	200(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 160(%rsp)
	ja	.LBB11_297
	movq	152(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	216(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	movq	184(%rsp), %rax
	jmp	.LBB11_309
.LBB11_303:
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	80(%rsp), %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rbx
	cmpq	%rdx, %r13
	ja	.LBB11_351
	movq	%rcx, %r14
	movq	72(%rsp), %rdi
	movq	96(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 104(%rsp)
	ja	.LBB11_288
	movq	8(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	176(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 128(%rsp)
	ja	.LBB11_292
	movq	120(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	200(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 160(%rsp)
	ja	.LBB11_297
	movq	152(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	216(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 192(%rsp)
	ja	.LBB11_359
	movq	184(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	264(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	movq	224(%rsp), %rax
.LBB11_309:
	movq	%rax, 8(%rsp)
	movq	%r14, %rsi
.LBB11_310:
	movq	24(%rsp), %r14
	movq	%rbx, %rcx
	jmp	.LBB11_280
.LBB11_314:
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	80(%rsp), %rdx
	cmpq	%rdx, %r13
	ja	.LBB11_362
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.26(%rip), %rcx
	movq	40(%rsp), %rdx
.LBB11_316:
	movq	$0, 8(%rsp)
	movq	24(%rsp), %r14
.LBB11_317:
	movq	16(%rsp), %r15
	movq	32(%rsp), %rbx
	movq	48(%rsp), %rsi
	jmp	.LBB11_281
.LBB11_318:
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	80(%rsp), %rdx
	cmpq	%rdx, %r13
	ja	.LBB11_362
	movq	72(%rsp), %rdi
	movq	96(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 104(%rsp)
	ja	.LBB11_363
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.26(%rip), %rcx
	movq	40(%rsp), %rdx
	movq	104(%rsp), %rsi
	jmp	.LBB11_279
.LBB11_321:
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	80(%rsp), %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rbx
	cmpq	%rdx, %r13
	ja	.LBB11_351
	movq	72(%rsp), %rdi
	movq	96(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 104(%rsp)
	ja	.LBB11_288
	movq	8(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	176(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 128(%rsp)
	ja	.LBB11_364
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.26(%rip), %rcx
	movq	40(%rsp), %rdx
.LBB11_325:
	movq	120(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	128(%rsp), %rsi
	jmp	.LBB11_279
.LBB11_326:
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	80(%rsp), %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rbx
	cmpq	%rdx, %r13
	ja	.LBB11_351
	movq	72(%rsp), %rdi
	movq	96(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 104(%rsp)
	ja	.LBB11_288
	movq	8(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	176(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 128(%rsp)
	ja	.LBB11_292
	movq	120(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	200(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 160(%rsp)
	ja	.LBB11_365
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.26(%rip), %rcx
	movq	40(%rsp), %rdx
.LBB11_331:
	movq	152(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	160(%rsp), %rsi
	jmp	.LBB11_279
.LBB11_332:
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	80(%rsp), %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rbx
	cmpq	%rdx, %r13
	ja	.LBB11_351
	movq	72(%rsp), %rdi
	movq	96(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 104(%rsp)
	ja	.LBB11_288
	movq	8(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	176(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 128(%rsp)
	ja	.LBB11_292
	movq	120(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	200(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 160(%rsp)
	ja	.LBB11_297
	movq	152(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	216(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 192(%rsp)
	ja	.LBB11_366
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.26(%rip), %rcx
	movq	40(%rsp), %rdx
.LBB11_338:
	movq	184(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	192(%rsp), %rsi
	jmp	.LBB11_279
.LBB11_339:
	movq	136(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	80(%rsp), %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rbx
	cmpq	%rdx, %r13
	ja	.LBB11_351
	movq	72(%rsp), %rdi
	movq	96(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 104(%rsp)
	ja	.LBB11_288
	movq	8(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	176(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 128(%rsp)
	ja	.LBB11_292
	movq	120(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	200(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 160(%rsp)
	ja	.LBB11_297
	movq	152(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	216(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 192(%rsp)
	ja	.LBB11_359
	movq	184(%rsp), %rdi
	shlq	$2, %rdi
	addq	72(%rsp), %rdi
	movq	264(%rsp), %rsi
	movq	56(%rsp), %rdx
	callq	*memcpy@GOTPCREL(%rip)
	movq	80(%rsp), %rdx
	cmpq	%rdx, 240(%rsp)
	ja	.LBB11_367
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.26(%rip), %rcx
	movq	40(%rsp), %rdx
.LBB11_346:
	movq	224(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	240(%rsp), %rsi
	jmp	.LBB11_279
.LBB11_347:
	movq	40(%rsp), %rdi
.LBB11_348:
.Ltmp6:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.25(%rip), %rdx
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp7:
	jmp	.LBB11_373
.LBB11_349:
.Ltmp63:
	movl	$8, %edi
	movl	$96, %esi
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.Ltmp64:
	jmp	.LBB11_373
.LBB11_350:
	xorl	%r12d, %r12d
	jmp	.LBB11_358
.LBB11_351:
	movq	$0, 8(%rsp)
	movq	24(%rsp), %r14
	movq	%rbx, %rcx
	jmp	.LBB11_317
.LBB11_352:
	xorl	%r12d, %r12d
.LBB11_353:
	movl	$4, %edi
	jmp	.LBB11_266
.LBB11_354:
	movl	$1, %r12d
	jmp	.LBB11_358
.LBB11_355:
	movl	$2, %r12d
	jmp	.LBB11_358
.LBB11_356:
.Ltmp23:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.24(%rip), %rdx
	movl	$4, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp24:
	jmp	.LBB11_373
.LBB11_357:
	movl	$3, %r12d
.LBB11_358:
.Ltmp14:
	movl	$4, %edi
	movq	168(%rsp), %rsi
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.Ltmp15:
	jmp	.LBB11_373
.LBB11_359:
	movq	184(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	192(%rsp), %rsi
	jmp	.LBB11_310
.LBB11_360:
.Ltmp25:
	movl	$4, %edi
	movq	%r13, %rsi
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.Ltmp26:
	jmp	.LBB11_373
.LBB11_362:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rcx
	jmp	.LBB11_316
.LBB11_363:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rcx
	movq	104(%rsp), %rsi
	jmp	.LBB11_279
.LBB11_364:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rcx
	jmp	.LBB11_325
.LBB11_365:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rcx
	jmp	.LBB11_331
.LBB11_366:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rcx
	jmp	.LBB11_338
.LBB11_367:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.27(%rip), %rcx
	jmp	.LBB11_346
.LBB11_368:
	movl	$24, %eax
	jmp	.LBB11_372
.LBB11_369:
	movl	$48, %eax
	jmp	.LBB11_372
.LBB11_370:
	movl	$72, %eax
	jmp	.LBB11_372
.LBB11_371:
	movl	$96, %eax
.LBB11_372:
	movq	%rax, 48(%rsp)
.Ltmp51:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.8(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp52:
.LBB11_373:
	ud2
.LBB11_374:
.Ltmp47:
	jmp	.LBB11_377
.LBB11_375:
.Ltmp50:
	jmp	.LBB11_392
.LBB11_376:
.Ltmp53:
.LBB11_377:
	movq	%rax, %r13
	testq	%rbx, %rbx
	je	.LBB11_379
	shlq	$3, %rbx
	movl	$8, %edx
	movq	%r12, %rdi
	movq	%rbx, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB11_379:
	movq	48(%rsp), %rax
	leaq	(%rsp,%rax), %rdi
	addq	$288, %rdi
	leaq	384(%rsp), %rsi
	callq	_RINvNtCs8uuN1nrGffB_4core3ptr9drop_glueINtNtNtB4_5array5drain5DrainINtNtCs3BN1jqgSbDf_5alloc3vec3VecdENCINvMNtNtB4_3ops9try_traitINtB1I_17NeverShortCircuitdE10wrap_mut_1B13_NCNvCs5dlvf2fXWpN_16outlined_io_main4mains1_0E0EEB2P_
	movq	24(%rsp), %r14
	movq	16(%rsp), %r15
	movq	%r13, %r12
	movq	(%r15), %rsi
	testq	%rsi, %rsi
	je	.LBB11_403
	jmp	.LBB11_414
.LBB11_380:
.Ltmp38:
	jmp	.LBB11_383
.LBB11_381:
.Ltmp35:
	jmp	.LBB11_389
.LBB11_382:
.Ltmp27:
.LBB11_383:
	movq	%rax, %r12
	movq	24(%rsp), %r14
	movq	16(%rsp), %r15
	movq	544(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB11_399
	jmp	.LBB11_410
.LBB11_384:
.Ltmp19:
	movq	%rax, 400(%rsp)
	cmpq	$0, 48(%rsp)
	je	.LBB11_386
	movq	48(%rsp), %rsi
	shlq	$5, %rsi
	movl	$4, %edx
	movq	%r13, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB11_386:
	movq	16(%rsp), %rbx
	movq	40(%rsp), %r15
	jmp	.LBB11_417
.LBB11_387:
.Ltmp16:
	movq	%rax, 400(%rsp)
	movq	16(%rsp), %rbx
	jmp	.LBB11_419
.LBB11_388:
.Ltmp32:
.LBB11_389:
	movq	%rax, %r12
	movq	24(%rsp), %r14
	movq	16(%rsp), %r15
	movq	32(%rsp), %rbx
	jmp	.LBB11_394
.LBB11_390:
.Ltmp65:
	movq	%rax, %r12
	movq	256(%rsp), %r13
	movq	96(%rsp), %rdi
	movq	24(%rsp), %r14
	jmp	.LBB11_421
.LBB11_391:
.Ltmp22:
.LBB11_392:
	movq	%rax, %r12
	movq	24(%rsp), %r14
	movq	16(%rsp), %r15
	movq	(%r15), %rsi
	testq	%rsi, %rsi
	je	.LBB11_403
	jmp	.LBB11_414
.LBB11_393:
.Ltmp56:
	movq	%rax, %r12
.LBB11_394:
	testq	%rbx, %rbx
	je	.LBB11_397
	shlq	$5, %rbx
	movl	$4, %edx
	movq	248(%rsp), %rdi
	movq	%rbx, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	jmp	.LBB11_397
.LBB11_396:
.Ltmp59:
	movq	%rax, %r12
	movq	24(%rsp), %r14
	movq	16(%rsp), %r15
.LBB11_397:
	movq	64(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB11_409
	movq	544(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB11_410
.LBB11_399:
	movq	568(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB11_411
.LBB11_400:
	movq	592(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB11_412
.LBB11_401:
	movq	616(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB11_413
.LBB11_402:
	movq	(%r15), %rsi
	testq	%rsi, %rsi
	jne	.LBB11_414
.LBB11_403:
	movq	232(%rsp), %rax
	movq	(%rax), %rsi
	testq	%rsi, %rsi
	jne	.LBB11_415
.LBB11_404:
	movq	(%rbp), %rsi
	testq	%rsi, %rsi
	je	.LBB11_406
.LBB11_405:
	movq	56(%r15), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB11_406:
	movq	72(%r15), %rsi
	testq	%rsi, %rsi
	movq	256(%rsp), %r13
	movq	96(%rsp), %rbx
	je	.LBB11_408
	movq	80(%r15), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB11_408:
	movl	$96, %esi
	movl	$8, %edx
	movq	%r15, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	%rbx, %rdi
	jmp	.LBB11_421
.LBB11_409:
	movq	72(%rsp), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	544(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB11_399
.LBB11_410:
	movq	552(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	568(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB11_400
.LBB11_411:
	movq	576(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	592(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB11_401
.LBB11_412:
	movq	600(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	616(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB11_402
.LBB11_413:
	movq	624(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	(%r15), %rsi
	testq	%rsi, %rsi
	je	.LBB11_403
.LBB11_414:
	movq	8(%r15), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	232(%rsp), %rax
	movq	(%rax), %rsi
	testq	%rsi, %rsi
	je	.LBB11_404
.LBB11_415:
	movq	32(%r15), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	(%rbp), %rsi
	testq	%rsi, %rsi
	jne	.LBB11_405
	jmp	.LBB11_406
.LBB11_416:
.Ltmp62:
	movq	%rax, 400(%rsp)
.LBB11_417:
	testq	%r15, %r15
	je	.LBB11_419
	movl	$4, %edx
	movq	8(%rsp), %rdi
	movq	168(%rsp), %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB11_419:
	testq	%r12, %r12
	jne	.LBB11_423
.LBB11_420:
	movl	$96, %esi
	movl	$8, %edx
	movq	%rbx, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	256(%rsp), %r13
	movq	96(%rsp), %rdi
	movq	24(%rsp), %r14
	movq	400(%rsp), %r12
.LBB11_421:
	testq	%r14, %r14
	je	.LBB11_428
	shlq	$2, %r14
	movl	$4, %edx
	movq	%r14, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	jmp	.LBB11_428
.LBB11_423:
	addq	$8, %rbp
	jmp	.LBB11_425
	.p2align	4
.LBB11_424:
	addq	$24, %rbp
	decq	%r12
	je	.LBB11_420
.LBB11_425:
	movq	-8(%rbp), %rsi
	testq	%rsi, %rsi
	je	.LBB11_424
	movq	(%rbp), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	*%r14
	jmp	.LBB11_424
.LBB11_427:
.Ltmp68:
	movq	%rax, %r12
.LBB11_428:
	movl	$32, %esi
	movl	$8, %edx
	movq	%r13, %rdi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	%r12, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end11:
	.size	_RNvCs5dlvf2fXWpN_16outlined_io_main4main, .Lfunc_end11-_RNvCs5dlvf2fXWpN_16outlined_io_main4main
	.cfi_endproc
	.section	.gcc_except_table._RNvCs5dlvf2fXWpN_16outlined_io_main4main,"a",@progbits
	.p2align	2, 0x0
GCC_except_table11:
.Lexception2:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end2-.Lcst_begin2
.Lcst_begin2:
	.uleb128 .Lfunc_begin2-.Lfunc_begin2
	.uleb128 .Ltmp8-.Lfunc_begin2
	.byte	0
	.byte	0
	.uleb128 .Ltmp8-.Lfunc_begin2
	.uleb128 .Ltmp9-.Ltmp8
	.uleb128 .Ltmp19-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp9-.Lfunc_begin2
	.uleb128 .Ltmp10-.Ltmp9
	.byte	0
	.byte	0
	.uleb128 .Ltmp10-.Lfunc_begin2
	.uleb128 .Ltmp11-.Ltmp10
	.uleb128 .Ltmp19-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp11-.Lfunc_begin2
	.uleb128 .Ltmp12-.Ltmp11
	.byte	0
	.byte	0
	.uleb128 .Ltmp12-.Lfunc_begin2
	.uleb128 .Ltmp13-.Ltmp12
	.uleb128 .Ltmp19-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp13-.Lfunc_begin2
	.uleb128 .Ltmp17-.Ltmp13
	.byte	0
	.byte	0
	.uleb128 .Ltmp17-.Lfunc_begin2
	.uleb128 .Ltmp18-.Ltmp17
	.uleb128 .Ltmp19-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp18-.Lfunc_begin2
	.uleb128 .Ltmp28-.Ltmp18
	.byte	0
	.byte	0
	.uleb128 .Ltmp28-.Lfunc_begin2
	.uleb128 .Ltmp29-.Ltmp28
	.uleb128 .Ltmp35-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp29-.Lfunc_begin2
	.uleb128 .Ltmp30-.Ltmp29
	.byte	0
	.byte	0
	.uleb128 .Ltmp30-.Lfunc_begin2
	.uleb128 .Ltmp31-.Ltmp30
	.uleb128 .Ltmp32-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp33-.Lfunc_begin2
	.uleb128 .Ltmp34-.Ltmp33
	.uleb128 .Ltmp35-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp34-.Lfunc_begin2
	.uleb128 .Ltmp36-.Ltmp34
	.byte	0
	.byte	0
	.uleb128 .Ltmp36-.Lfunc_begin2
	.uleb128 .Ltmp37-.Ltmp36
	.uleb128 .Ltmp38-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp37-.Lfunc_begin2
	.uleb128 .Ltmp48-.Ltmp37
	.byte	0
	.byte	0
	.uleb128 .Ltmp48-.Lfunc_begin2
	.uleb128 .Ltmp49-.Ltmp48
	.uleb128 .Ltmp50-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp49-.Lfunc_begin2
	.uleb128 .Ltmp39-.Ltmp49
	.byte	0
	.byte	0
	.uleb128 .Ltmp39-.Lfunc_begin2
	.uleb128 .Ltmp46-.Ltmp39
	.uleb128 .Ltmp47-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp46-.Lfunc_begin2
	.uleb128 .Ltmp66-.Ltmp46
	.byte	0
	.byte	0
	.uleb128 .Ltmp66-.Lfunc_begin2
	.uleb128 .Ltmp67-.Ltmp66
	.uleb128 .Ltmp68-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp60-.Lfunc_begin2
	.uleb128 .Ltmp61-.Ltmp60
	.uleb128 .Ltmp62-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp57-.Lfunc_begin2
	.uleb128 .Ltmp58-.Ltmp57
	.uleb128 .Ltmp59-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp58-.Lfunc_begin2
	.uleb128 .Ltmp54-.Ltmp58
	.byte	0
	.byte	0
	.uleb128 .Ltmp54-.Lfunc_begin2
	.uleb128 .Ltmp55-.Ltmp54
	.uleb128 .Ltmp56-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp20-.Lfunc_begin2
	.uleb128 .Ltmp21-.Ltmp20
	.uleb128 .Ltmp22-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp21-.Lfunc_begin2
	.uleb128 .Ltmp6-.Ltmp21
	.byte	0
	.byte	0
	.uleb128 .Ltmp6-.Lfunc_begin2
	.uleb128 .Ltmp64-.Ltmp6
	.uleb128 .Ltmp65-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp23-.Lfunc_begin2
	.uleb128 .Ltmp24-.Ltmp23
	.uleb128 .Ltmp27-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp14-.Lfunc_begin2
	.uleb128 .Ltmp15-.Ltmp14
	.uleb128 .Ltmp16-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp25-.Lfunc_begin2
	.uleb128 .Ltmp26-.Ltmp25
	.uleb128 .Ltmp27-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp51-.Lfunc_begin2
	.uleb128 .Ltmp52-.Ltmp51
	.uleb128 .Ltmp53-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp52-.Lfunc_begin2
	.uleb128 .Lfunc_end11-.Ltmp52
	.byte	0
	.byte	0
.Lcst_end2:
	.p2align	2, 0x0

	.section	.text.unlikely._RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end12, nop
	.type	_RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCs5dlvf2fXWpN_16outlined_io_main,@function
_RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCs5dlvf2fXWpN_16outlined_io_main:
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
	callq	_RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs5dlvf2fXWpN_16outlined_io_main
	cmpl	$1, (%rsp)
	je	.LBB12_2
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
.LBB12_2:
	.cfi_def_cfa_offset 48
	movq	8(%rsp), %rdi
	movq	16(%rsp), %rsi
	callq	*_RNvNtCs3BN1jqgSbDf_5alloc7raw_vec12handle_error@GOTPCREL(%rip)
.Lfunc_end12:
	.size	_RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end12-_RNvMs7_NtCs3BN1jqgSbDf_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.text.unlikely._RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end13, nop
	.type	_RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs5dlvf2fXWpN_16outlined_io_main,@function
_RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs5dlvf2fXWpN_16outlined_io_main:
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
	je	.LBB13_2
	movl	$8, %eax
	xorl	%r14d, %r14d
	jmp	.LBB13_10
.LBB13_2:
	testq	%rsi, %rsi
	je	.LBB13_4
	imulq	%rsi, %r9
	movq	%r8, %rdi
	movq	%r9, %rsi
	movq	%r15, %rdx
	movq	%r14, %rcx
	callq	*_RNvCs5WqGp7zLRnA_7___rustc14___rust_realloc@GOTPCREL(%rip)
	testq	%rax, %rax
	jne	.LBB13_6
	jmp	.LBB13_9
.LBB13_4:
	testq	%r14, %r14
	je	.LBB13_5
	callq	*_RNvCs5WqGp7zLRnA_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movq	%r14, %rdi
	movq	%r15, %rsi
	callq	*_RNvCs5WqGp7zLRnA_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	jne	.LBB13_6
.LBB13_9:
	movq	%r15, 8(%rbx)
	movl	$16, %eax
	jmp	.LBB13_10
.LBB13_5:
	movq	%r15, %rax
.LBB13_6:
	movq	%rax, 8(%rbx)
	movl	$16, %eax
	xorl	%r12d, %r12d
.LBB13_10:
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
.Lfunc_end13:
	.size	_RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end13-_RNvMs8_NtCs3BN1jqgSbDf_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.text._RNvNtCs5dlvf2fXWpN_16outlined_io_main7control11gather_band,"ax",@progbits
	.prefalign	4, .Lfunc_end14, nop
	.type	_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control11gather_band,@function
_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control11gather_band:
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
	subq	$136, %rsp
	.cfi_def_cfa_offset 192
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%r8, 80(%rsp)
	movq	%rsi, 48(%rsp)
	movq	%rdi, 88(%rsp)
	movq	%rdx, 56(%rsp)
	movq	%rdx, %r11
	shrq	$3, %r11
	je	.LBB14_6
	movq	88(%rsp), %r8
	movq	8(%r8), %rax
	movq	%rax, (%rsp)
	movq	24(%r8), %rax
	movq	%rax, 40(%rsp)
	movq	40(%r8), %rax
	movq	%rax, 32(%rsp)
	movq	56(%r8), %rax
	movq	%rax, 24(%rsp)
	movq	72(%r8), %rax
	movq	%rax, 16(%rsp)
	movq	88(%r8), %rax
	movq	%rax, 8(%rsp)
	movq	%rcx, %rax
	shlq	$5, %rax
	movq	48(%rsp), %r10
	leaq	(,%r10,4), %rbp
	movq	(%r8), %rdx
	addq	%rbp, %rdx
	movq	%rdx, 72(%rsp)
	movq	16(%r8), %rdx
	addq	%rbp, %rdx
	movq	%rdx, 64(%rsp)
	movq	32(%r8), %rbx
	addq	%rbp, %rbx
	movq	48(%r8), %rdx
	addq	%rbp, %rdx
	movq	64(%r8), %rsi
	addq	%rbp, %rsi
	movq	80(%r8), %rdi
	addq	%rbp, %rdi
	movq	96(%r8), %r14
	addq	%rbp, %r14
	addq	112(%r8), %rbp
	movq	80(%rsp), %r15
	leaq	(%rax,%r15), %r12
	addq	$448, %r12
	movq	104(%r8), %r15
	movq	120(%r8), %r13
	xorl	%eax, %eax
	.p2align	4
.LBB14_2:
	cmpq	$-9, %r10
	ja	.LBB14_4
	leaq	8(%r10), %r8
	cmpq	(%rsp), %r8
	ja	.LBB14_4
	cmpq	40(%rsp), %r8
	ja	.LBB14_13
	cmpq	32(%rsp), %r8
	ja	.LBB14_15
	cmpq	24(%rsp), %r8
	ja	.LBB14_17
	cmpq	16(%rsp), %r8
	ja	.LBB14_19
	cmpq	8(%rsp), %r8
	ja	.LBB14_21
	cmpq	%r15, %r8
	ja	.LBB14_23
	cmpq	%r13, %r8
	ja	.LBB14_25
	leaq	(%rcx,%rax), %r10
	cmpq	%r9, %r10
	jae	.LBB14_44
	movq	72(%rsp), %r10
	vmovups	(%r10,%rax,2), %ymm4
	movq	64(%rsp), %r10
	vmovups	(%r10,%rax,2), %ymm5
	vmovups	(%rbx,%rax,2), %ymm6
	vmovups	(%rdx,%rax,2), %ymm8
	vmovups	(%rsi,%rax,2), %ymm7
	vmovups	(%rdi,%rax,2), %ymm9
	vmovups	(%r14,%rax,2), %ymm10
	vmovups	(%rbp,%rax,2), %ymm11
	vunpcklps	%ymm5, %ymm4, %ymm2
	vunpcklps	%ymm8, %ymm6, %ymm3
	vunpcklpd	%ymm3, %ymm2, %ymm0
	vunpcklps	%ymm9, %ymm7, %ymm12
	vunpcklps	%ymm11, %ymm10, %ymm13
	vunpcklpd	%ymm13, %ymm12, %ymm1
	vinsertf128	$1, %xmm1, %ymm0, %ymm14
	vmovups	%ymm14, -448(%r12)
	leaq	(%rcx,%rax), %r10
	addq	$2, %r10
	cmpq	%r9, %r10
	jae	.LBB14_44
	vunpckhpd	%ymm3, %ymm2, %ymm2
	vunpckhpd	%ymm13, %ymm12, %ymm3
	vinsertf128	$1, %xmm3, %ymm2, %ymm12
	vmovups	%ymm12, -384(%r12)
	leaq	(%rcx,%rax), %r10
	addq	$4, %r10
	cmpq	%r9, %r10
	jae	.LBB14_44
	vunpckhps	%ymm5, %ymm4, %ymm5
	vunpckhps	%ymm8, %ymm6, %ymm8
	vunpcklpd	%ymm8, %ymm5, %ymm4
	vunpckhps	%ymm9, %ymm7, %ymm7
	vunpckhps	%ymm11, %ymm10, %ymm9
	vunpcklpd	%ymm9, %ymm7, %ymm6
	vinsertf128	$1, %xmm6, %ymm4, %ymm10
	vmovups	%ymm10, -320(%r12)
	leaq	(%rcx,%rax), %r10
	addq	$6, %r10
	cmpq	%r9, %r10
	jae	.LBB14_44
	vunpckhpd	%ymm8, %ymm5, %ymm5
	vunpckhpd	%ymm9, %ymm7, %ymm7
	vinsertf128	$1, %xmm7, %ymm5, %ymm8
	vmovups	%ymm8, -256(%r12)
	leaq	(%rcx,%rax), %r10
	addq	$8, %r10
	cmpq	%r9, %r10
	jae	.LBB14_44
	vperm2f128	$49, %ymm1, %ymm0, %ymm0
	vmovups	%ymm0, -192(%r12)
	leaq	(%rcx,%rax), %r10
	addq	$10, %r10
	cmpq	%r9, %r10
	jae	.LBB14_44
	vperm2f128	$49, %ymm3, %ymm2, %ymm0
	vmovups	%ymm0, -128(%r12)
	leaq	(%rcx,%rax), %r10
	addq	$12, %r10
	cmpq	%r9, %r10
	jae	.LBB14_44
	vperm2f128	$49, %ymm6, %ymm4, %ymm0
	vmovups	%ymm0, -64(%r12)
	leaq	(%rcx,%rax), %r10
	addq	$14, %r10
	cmpq	%r9, %r10
	jae	.LBB14_44
	vperm2f128	$49, %ymm7, %ymm5, %ymm0
	vmovups	%ymm0, (%r12)
	decq	%r11
	addq	$512, %r12
	addq	$16, %rax
	movq	%r8, %r10
	testq	%r11, %r11
	jne	.LBB14_2
.LBB14_6:
	movq	56(%rsp), %rax
	movq	%rax, %r11
	andq	$-8, %r11
	cmpq	%rax, %r11
	je	.LBB14_39
	movq	88(%rsp), %rdi
	movq	8(%rdi), %rsi
	movq	24(%rdi), %rax
	movq	%rax, (%rsp)
	movq	40(%rdi), %rax
	movq	%rax, 40(%rsp)
	movq	56(%rdi), %rax
	movq	%rax, 32(%rsp)
	movq	48(%rsp), %rax
	leaq	(,%rax,4), %r8
	movq	112(%rdi), %rax
	addq	%r8, %rax
	movq	%rax, 64(%rsp)
	movq	96(%rdi), %rax
	addq	%r8, %rax
	movq	%rax, 128(%rsp)
	movq	80(%rdi), %rax
	addq	%r8, %rax
	movq	%rax, 120(%rsp)
	movq	64(%rdi), %rax
	addq	%r8, %rax
	movq	%rax, 112(%rsp)
	movq	48(%rdi), %rax
	addq	%r8, %rax
	movq	%rax, 104(%rsp)
	movq	56(%rsp), %rax
	shrq	$3, %rax
	movq	%rax, %r10
	shlq	$4, %r10
	addq	%rcx, %r10
	shlq	$9, %rax
	shlq	$5, %rcx
	addq	%rax, %rcx
	movq	32(%rdi), %rax
	addq	%r8, %rax
	movq	%rax, 96(%rsp)
	movq	80(%rsp), %rax
	addq	%rax, %rcx
	addq	$28, %rcx
	movq	16(%rdi), %rdx
	addq	%r8, %rdx
	addq	(%rdi), %r8
	movq	%r8, 72(%rsp)
	movq	72(%rdi), %rax
	movq	%rax, 24(%rsp)
	movq	88(%rdi), %rax
	movq	%rax, 16(%rsp)
	movq	104(%rdi), %rax
	movq	%rax, 8(%rsp)
	movq	120(%rdi), %r15
	.p2align	4
.LBB14_8:
	movq	48(%rsp), %rax
	leaq	(%rax,%r11), %rdi
	cmpq	%rsi, %rdi
	jae	.LBB14_11
	cmpq	(%rsp), %rdi
	jae	.LBB14_10
	cmpq	40(%rsp), %rdi
	jae	.LBB14_27
	cmpq	32(%rsp), %rdi
	jae	.LBB14_29
	cmpq	24(%rsp), %rdi
	jae	.LBB14_31
	cmpq	16(%rsp), %rdi
	jae	.LBB14_33
	cmpq	8(%rsp), %rdi
	jae	.LBB14_35
	cmpq	%r15, %rdi
	jae	.LBB14_37
	cmpq	%r9, %r10
	jae	.LBB14_41
	movq	72(%rsp), %rax
	movl	(%rax,%r11,4), %edi
	movl	(%rdx,%r11,4), %r12d
	movq	96(%rsp), %rax
	movl	(%rax,%r11,4), %r13d
	movq	104(%rsp), %rax
	movl	(%rax,%r11,4), %ebp
	movq	112(%rsp), %rax
	movl	(%rax,%r11,4), %ebx
	movq	120(%rsp), %rax
	movl	(%rax,%r11,4), %eax
	movq	64(%rsp), %r8
	movl	(%r8,%r11,4), %r14d
	movq	128(%rsp), %r8
	movl	(%r8,%r11,4), %r8d
	movl	%edi, -28(%rcx)
	movl	%r12d, -24(%rcx)
	movl	%r13d, -20(%rcx)
	movl	%ebp, -16(%rcx)
	movl	%ebx, -12(%rcx)
	movl	%eax, -8(%rcx)
	movl	%r8d, -4(%rcx)
	movl	%r14d, (%rcx)
	incq	%r11
	addq	$2, %r10
	addq	$64, %rcx
	cmpq	56(%rsp), %r11
	jb	.LBB14_8
.LBB14_39:
	addq	$136, %rsp
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
.LBB14_4:
	.cfi_def_cfa_offset 192
	movq	(%rsp), %rdx
	leaq	8(%r10), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.10(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB14_13:
	movq	40(%rsp), %rdx
	leaq	8(%r10), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.10(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB14_15:
	movq	32(%rsp), %rdx
	leaq	8(%r10), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.10(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB14_17:
	movq	24(%rsp), %rdx
	leaq	8(%r10), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.10(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB14_19:
	movq	16(%rsp), %rdx
	leaq	8(%r10), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.10(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB14_21:
	movq	8(%rsp), %rdx
	leaq	8(%r10), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.10(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB14_23:
	movq	%r15, %rdx
	leaq	8(%r10), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.10(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB14_25:
	movq	%r13, %rdx
	leaq	8(%r10), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.10(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB14_44:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.30(%rip), %rdx
	movq	%r10, %rdi
	movq	%r9, %rsi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB14_41:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.29(%rip), %rdx
	movq	%r10, %rdi
	movq	%r9, %rsi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB14_37:
	movq	%r15, %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.11(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB14_35:
	movq	8(%rsp), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.11(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB14_33:
	movq	16(%rsp), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.11(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB14_31:
	movq	24(%rsp), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.11(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB14_29:
	movq	32(%rsp), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.11(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB14_27:
	movq	40(%rsp), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.11(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB14_10:
	movq	(%rsp), %rsi
.LBB14_11:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.11(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Lfunc_end14:
	.size	_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control11gather_band, .Lfunc_end14-_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control11gather_band
	.cfi_endproc

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI15_0:
	.long	0x3f9d7658
.LCPI15_1:
	.long	0x3fd01800
.LCPI15_2:
	.long	0xbee31355
.LCPI15_3:
	.long	0xbf631355
.LCPI15_4:
	.long	0xbf620676
.LCPI15_5:
	.long	0xbfe20676
.LCPI15_6:
	.long	0x3d5901ae
.LCPI15_7:
	.long	0x3dd901ae
.LCPI15_8:
	.long	0x3fcb0673
.LCPI15_9:
	.long	0x404b0673
	.section	.text._RNvNtCs5dlvf2fXWpN_16outlined_io_main7control15inverse_97_rows,"ax",@progbits
	.prefalign	4, .Lfunc_end15, nop
	.type	_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control15inverse_97_rows,@function
_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control15inverse_97_rows:
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
	subq	$248, %rsp
	.cfi_def_cfa_offset 304
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%r9, %rbx
	movq	%rsi, %r10
	movl	$1, %r12d
	subq	%rcx, %r12
	testb	$1, %bl
	movl	$0, %r15d
	cmovneq	%r12, %r15
	cmpq	%rsi, %r9
	ja	.LBB15_171
	movq	%rdx, %r14
	movq	%rbx, %rsi
	addq	%rdx, %rsi
	jb	.LBB15_186
	cmpq	%r10, %rsi
	ja	.LBB15_186
	leaq	(%r14,%r14), %rax
	movq	%rbx, %rsi
	addq	%rax, %rsi
	jb	.LBB15_175
	cmpq	%r10, %rsi
	ja	.LBB15_175
	leaq	(%r14,%r14,2), %rbp
	movq	%rbx, %rsi
	addq	%rbp, %rsi
	jb	.LBB15_177
	cmpq	%r10, %rsi
	ja	.LBB15_177
	leaq	(,%r14,4), %rdx
	movq	%rbx, %rsi
	addq	%rdx, %rsi
	jb	.LBB15_179
	cmpq	%r10, %rsi
	ja	.LBB15_179
	leaq	(%r14,%r14,4), %r11
	movq	%rbx, %rsi
	addq	%r11, %rsi
	jb	.LBB15_181
	cmpq	%r10, %rsi
	ja	.LBB15_181
	movq	%r8, %r13
	leaq	(%r14,%r14), %rsi
	leaq	(%rsi,%rsi,2), %r8
	movq	%rbx, %rsi
	addq	%r8, %rsi
	jb	.LBB15_183
	cmpq	%r10, %rsi
	ja	.LBB15_183
	leaq	(,%r14,8), %r9
	subq	%r14, %r9
	movq	%rbx, %rsi
	addq	%r9, %rsi
	jb	.LBB15_184
	cmpq	%r10, %rsi
	ja	.LBB15_185
	movq	%r10, 32(%rsp)
	movq	%rbx, %rsi
	shrq	%rsi
	addq	%rsi, %r15
	leaq	(%rdi,%r8,4), %rsi
	leaq	(%rdi,%r11,4), %r10
	leaq	(%rdi,%rdx,4), %rdx
	leaq	(%rdi,%rbp,4), %rbp
	leaq	(%rdi,%rax,4), %rax
	leaq	(%rdi,%r14,4), %r11
	leaq	(%rdi,%r9,4), %r8
	movq	%rdi, 24(%rsp)
	movq	%rdi, 120(%rsp)
	movq	%rbx, 128(%rsp)
	movq	%r11, (%rsp)
	movq	%r11, 136(%rsp)
	movq	%rbx, 144(%rsp)
	movq	%rax, 152(%rsp)
	movq	%rbx, 160(%rsp)
	movq	%rbp, 168(%rsp)
	movq	%rbx, 176(%rsp)
	movq	%rdx, 184(%rsp)
	movq	%rbx, 192(%rsp)
	movq	%r10, 200(%rsp)
	movq	%rbx, 208(%rsp)
	movq	%rsi, 216(%rsp)
	movq	%rbx, 224(%rsp)
	movq	%r8, 232(%rsp)
	movq	%rbx, 240(%rsp)
	movq	%rcx, 8(%rsp)
	leaq	120(%rsp), %rbp
	movq	%rbp, %rdi
	xorl	%esi, %esi
	movq	%r15, %rdx
	movq	8(%rsp), %rcx
	movq	%r13, %r8
	movq	%rbx, %r9
	callq	_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control11gather_band
	movq	%rbx, %rdx
	subq	%r15, %rdx
	movq	%rbp, %rdi
	movq	8(%rsp), %rbp
	movq	%r15, %rsi
	movq	%r12, %rcx
	movq	%r13, %r8
	movq	%rbx, %r9
	callq	_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control11gather_band
	movq	%rbx, %rdx
	shlq	$5, %rdx
	leaq	(%rdx,%r13), %rax
	testq	%rbp, %rbp
	je	.LBB15_18
	cmpq	%rbx, %rbp
	jae	.LBB15_22
	movq	%rbp, %rcx
	shlq	$5, %rcx
	addq	%r13, %rcx
	jmp	.LBB15_19
.LBB15_18:
	movq	%r13, %rcx
	testq	%rbx, %rbx
	je	.LBB15_24
.LBB15_19:
	vbroadcastss	.LCPI15_0(%rip), %ymm0
	vmulps	(%rcx), %ymm0, %ymm1
	vmovups	%ymm1, (%rcx)
	movq	%rax, %rsi
	subq	%rcx, %rsi
	addq	$-32, %rsi
	cmpq	$33, %rsi
	jb	.LBB15_22
	addq	$64, %rcx
	.p2align	4
.LBB15_21:
	vmulps	(%rcx), %ymm0, %ymm1
	vmovups	%ymm1, (%rcx)
	addq	$-64, %rsi
	addq	$64, %rcx
	cmpq	$32, %rsi
	ja	.LBB15_21
.LBB15_22:
	cmpq	$1, %rbp
	jne	.LBB15_24
	movq	%r13, %rcx
	testq	%rbx, %rbx
	jne	.LBB15_26
	jmp	.LBB15_29
.LBB15_24:
	cmpq	%rbx, %r12
	jae	.LBB15_29
	movq	%r12, %rcx
	shlq	$5, %rcx
	addq	%r13, %rcx
.LBB15_26:
	vbroadcastss	.LCPI15_1(%rip), %ymm0
	vmulps	(%rcx), %ymm0, %ymm1
	vmovups	%ymm1, (%rcx)
	subq	%rcx, %rax
	addq	$-32, %rax
	cmpq	$33, %rax
	jb	.LBB15_29
	addq	$64, %rcx
	.p2align	4
.LBB15_28:
	vmulps	(%rcx), %ymm0, %ymm1
	vmovups	%ymm1, (%rcx)
	addq	$-64, %rax
	addq	$64, %rcx
	cmpq	$32, %rax
	ja	.LBB15_28
.LBB15_29:
	leaq	-1(%rbx), %rsi
	xorl	%r11d, %r11d
	movq	%rsi, %rax
	subq	%rbp, %rax
	cmovaeq	%rax, %r11
	movq	%r11, %r10
	shrq	%r10
	subq	%r10, %r11
	je	.LBB15_35
	cmpq	%rsi, %rbp
	movq	%rsi, %rax
	cmovbq	%rbp, %rax
	leaq	(%rax,%r10), %r8
	incq	%r8
	movq	%rbp, %r9
	shlq	$5, %r9
	addq	%r13, %r9
	movl	$1, %r15d
	vbroadcastss	.LCPI15_2(%rip), %ymm0
	movq	%rbp, %rdi
	.p2align	4
.LBB15_31:
	movq	%rdi, %rax
	subq	$1, %rax
	cmovbq	%r15, %rax
	cmpq	%rbx, %rax
	jae	.LBB15_195
	leaq	1(%rdi), %rcx
	cmpq	%rbx, %rcx
	jae	.LBB15_194
	cmpq	%rbx, %rdi
	jae	.LBB15_193
	shlq	$5, %rax
	vmovups	(%r13,%rax), %ymm1
	vaddps	32(%r9), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r9), %ymm1, %ymm1
	vmovups	%ymm1, (%r9)
	incq	%r8
	addq	$64, %r9
	addq	$2, %rdi
	cmpq	%r8, %rbx
	jne	.LBB15_31
.LBB15_35:
	movl	%esi, %r8d
	andl	$1, %r8d
	cmpq	%rbp, %r8
	jne	.LBB15_38
	movq	%rbx, %rdi
	subq	$2, %rdi
	jb	.LBB15_198
	shlq	$5, %rdi
	vbroadcastss	.LCPI15_3(%rip), %ymm0
	vmulps	(%r13,%rdi), %ymm0, %ymm0
	movq	%rsi, %rax
	shlq	$5, %rax
	vaddps	(%r13,%rax), %ymm0, %ymm0
	vmovups	%ymm0, (%r13,%rax)
.LBB15_38:
	movq	%r10, 16(%rsp)
	xorl	%r9d, %r9d
	movq	%rsi, %rax
	movq	%r12, %r10
	subq	%r12, %rax
	cmovaeq	%rax, %r9
	movq	%r9, %rax
	shrq	%rax
	subq	%rax, %r9
	je	.LBB15_44
	movq	8(%rsp), %rax
	shlq	$5, %rax
	movq	%r13, %r15
	subq	%rax, %r15
	addq	$32, %r15
	movl	$1, %r12d
	vbroadcastss	.LCPI15_4(%rip), %ymm0
	movq	%r9, %rbp
	movq	%r10, %rdi
	.p2align	4
.LBB15_40:
	movq	%rdi, %rax
	subq	$1, %rax
	cmovbq	%r12, %rax
	cmpq	%rbx, %rax
	jae	.LBB15_195
	leaq	1(%rdi), %rcx
	cmpq	%rbx, %rcx
	jae	.LBB15_194
	cmpq	%rbx, %rdi
	jae	.LBB15_193
	shlq	$5, %rax
	vmovups	(%r13,%rax), %ymm1
	vaddps	32(%r15), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r15), %ymm1, %ymm1
	vmovups	%ymm1, (%r15)
	decq	%rbp
	addq	$64, %r15
	addq	$2, %rdi
	testq	%rbp, %rbp
	jne	.LBB15_40
.LBB15_44:
	movq	%r10, %r12
	cmpq	%r10, %r8
	movq	8(%rsp), %rbp
	movq	16(%rsp), %rcx
	jne	.LBB15_47
	movq	%rbx, %rdi
	subq	$2, %rdi
	jb	.LBB15_198
	shlq	$5, %rdi
	vbroadcastss	.LCPI15_5(%rip), %ymm0
	vmulps	(%r13,%rdi), %ymm0, %ymm0
	movq	%rsi, %rax
	shlq	$5, %rax
	vaddps	(%r13,%rax), %ymm0, %ymm0
	vmovups	%ymm0, (%r13,%rax)
.LBB15_47:
	testq	%r11, %r11
	je	.LBB15_53
	cmpq	%rsi, %rbp
	movq	%rsi, %rax
	cmovbq	%rbp, %rax
	leaq	(%rax,%rcx), %r10
	incq	%r10
	movq	%rbp, %r11
	shlq	$5, %r11
	addq	%r13, %r11
	movl	$1, %r15d
	vbroadcastss	.LCPI15_6(%rip), %ymm0
	movq	%rbp, %rdi
	.p2align	4
.LBB15_49:
	movq	%rdi, %rax
	subq	$1, %rax
	cmovbq	%r15, %rax
	cmpq	%rbx, %rax
	jae	.LBB15_195
	leaq	1(%rdi), %rcx
	cmpq	%rbx, %rcx
	jae	.LBB15_194
	cmpq	%rbx, %rdi
	jae	.LBB15_193
	shlq	$5, %rax
	vmovups	(%r13,%rax), %ymm1
	vaddps	32(%r11), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r11), %ymm1, %ymm1
	vmovups	%ymm1, (%r11)
	incq	%r10
	addq	$64, %r11
	addq	$2, %rdi
	cmpq	%r10, %rbx
	jne	.LBB15_49
.LBB15_53:
	cmpq	%rbp, %r8
	jne	.LBB15_56
	movq	%rbx, %rdi
	subq	$2, %rdi
	jb	.LBB15_198
	shlq	$5, %rdi
	vbroadcastss	.LCPI15_7(%rip), %ymm0
	vmulps	(%r13,%rdi), %ymm0, %ymm0
	movq	%rsi, %rax
	shlq	$5, %rax
	vaddps	(%r13,%rax), %ymm0, %ymm0
	vmovups	%ymm0, (%r13,%rax)
.LBB15_56:
	testq	%r9, %r9
	je	.LBB15_62
	shlq	$5, %rbp
	movq	%r13, %r10
	subq	%rbp, %r10
	addq	$32, %r10
	movl	$1, %r11d
	vbroadcastss	.LCPI15_8(%rip), %ymm0
	movq	%r12, %rdi
	.p2align	4
.LBB15_58:
	movq	%rdi, %rax
	subq	$1, %rax
	cmovbq	%r11, %rax
	cmpq	%rbx, %rax
	jae	.LBB15_195
	leaq	1(%rdi), %rcx
	cmpq	%rbx, %rcx
	jae	.LBB15_194
	cmpq	%rbx, %rdi
	jae	.LBB15_193
	shlq	$5, %rax
	vmovups	(%r13,%rax), %ymm1
	vaddps	32(%r10), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r10), %ymm1, %ymm1
	vmovups	%ymm1, (%r10)
	decq	%r9
	addq	$64, %r10
	addq	$2, %rdi
	testq	%r9, %r9
	jne	.LBB15_58
.LBB15_62:
	cmpq	%r12, %r8
	movq	32(%rsp), %rax
	jne	.LBB15_65
	movq	%rbx, %rdi
	subq	$2, %rdi
	jb	.LBB15_198
	shlq	$5, %rdi
	vbroadcastss	.LCPI15_9(%rip), %ymm0
	vmulps	(%r13,%rdi), %ymm0, %ymm0
	shlq	$5, %rsi
	vaddps	(%r13,%rsi), %ymm0, %ymm0
	vmovups	%ymm0, (%r13,%rsi)
.LBB15_65:
	andq	$-256, %rdx
	je	.LBB15_68
	testq	%r14, %r14
	je	.LBB15_188
	addq	%r13, %rdx
	subq	%r14, %rax
	cmpq	%r14, %rax
	movq	%r14, %rcx
	cmovbq	%rax, %rcx
	movq	%rax, %rsi
	subq	%rcx, %rsi
	cmpq	%r14, %rsi
	movq	%r14, %r8
	cmovbq	%rsi, %r8
	movq	%rsi, %r9
	subq	%r8, %r9
	cmpq	%r14, %r9
	movq	%r14, %r10
	cmovbq	%r9, %r10
	movq	%r9, %rdi
	subq	%r10, %rdi
	cmpq	%r14, %rdi
	movq	%r14, %rbp
	cmovbq	%rdi, %rbp
	movq	%rdi, 112(%rsp)
	subq	%rbp, %rdi
	cmpq	%r14, %rdi
	movq	%r14, %r11
	cmovbq	%rdi, %r11
	movq	%rdi, 96(%rsp)
	subq	%r11, %rdi
	cmpq	%r14, %rdi
	movq	%r14, %r12
	cmovbq	%rdi, %r12
	movq	%rdi, 80(%rsp)
	subq	%r12, %rdi
	cmpq	%r14, %rdi
	cmovaeq	%r14, %rdi
	movq	%rdi, 40(%rsp)
	movq	%r11, 56(%rsp)
	leaq	(,%r11,4), %rdi
	leaq	(%rdi,%r14,4), %rdi
	leaq	(%rdi,%rbp,4), %rdi
	leaq	(%rdi,%r10,4), %rdi
	leaq	(%rdi,%r8,4), %rdi
	leaq	(%rdi,%rcx,4), %r11
	movq	%r12, 48(%rsp)
	leaq	(%r11,%r12,4), %r12
	movq	24(%rsp), %rdi
	addq	%rdi, %r12
	movq	%r12, 64(%rsp)
	addq	%rdi, %r11
	movq	%r11, 72(%rsp)
	movq	(%rsp), %rdi
	leaq	(%rdi,%rcx,4), %rdi
	movq	%rdi, %r15
	leaq	(%rdi,%r8,4), %rdi
	movq	%rdi, 8(%rsp)
	leaq	(%rdi,%r10,4), %rdi
	movq	%rbp, 16(%rsp)
	movq	%rdi, 104(%rsp)
	leaq	(%rdi,%rbp,4), %rdi
	movq	%rdi, 88(%rsp)
	xorl	%edi, %edi
	movq	%r13, %r12
	jmp	.LBB15_151
.LBB15_68:
	testq	%r14, %r14
	je	.LBB15_187
.LBB15_69:
	movabsq	$288230376151711736, %rdi
	andq	%rbx, %rdi
	cmpq	%r14, %rbx
	ja	.LBB15_192
	movq	%rdi, %rax
	shlq	$5, %rax
	addq	%rax, %r13
	movl	%ebx, %eax
	andl	$7, %eax
	movq	24(%rsp), %rdx
	je	.LBB15_78
	movl	(%r13), %ecx
	movl	%ecx, (%rdx,%rdi,4)
	cmpl	$1, %eax
	je	.LBB15_78
	movl	32(%r13), %ecx
	movl	%ecx, 4(%rdx,%rdi,4)
	cmpl	$2, %eax
	je	.LBB15_78
	movl	64(%r13), %ecx
	movl	%ecx, 8(%rdx,%rdi,4)
	cmpl	$3, %eax
	je	.LBB15_78
	movl	96(%r13), %ecx
	movl	%ecx, 12(%rdx,%rdi,4)
	cmpl	$4, %eax
	je	.LBB15_78
	movl	128(%r13), %ecx
	movl	%ecx, 16(%rdx,%rdi,4)
	cmpl	$5, %eax
	je	.LBB15_78
	movl	160(%r13), %ecx
	movl	%ecx, 20(%rdx,%rdi,4)
	cmpl	$6, %eax
	je	.LBB15_78
	movl	192(%r13), %ecx
	movl	%ecx, 24(%rdx,%rdi,4)
.LBB15_78:
	movq	32(%rsp), %r8
	cmpq	%r14, %r8
	jbe	.LBB15_148
	subq	%r14, %r8
	cmpq	%r14, %r8
	movq	%r14, %rdx
	cmovbq	%r8, %rdx
	cmpq	%rdx, %rbx
	ja	.LBB15_172
	testq	%rax, %rax
	je	.LBB15_88
	movl	4(%r13), %ecx
	movq	(%rsp), %rsi
	movl	%ecx, (%rsi,%rdi,4)
	cmpl	$1, %eax
	je	.LBB15_88
	movl	36(%r13), %ecx
	movq	(%rsp), %rsi
	movl	%ecx, 4(%rsi,%rdi,4)
	cmpl	$2, %eax
	je	.LBB15_88
	movl	68(%r13), %ecx
	movq	(%rsp), %rsi
	movl	%ecx, 8(%rsi,%rdi,4)
	cmpl	$3, %eax
	je	.LBB15_88
	movl	100(%r13), %ecx
	movq	(%rsp), %rsi
	movl	%ecx, 12(%rsi,%rdi,4)
	cmpl	$4, %eax
	je	.LBB15_88
	movl	132(%r13), %ecx
	movq	(%rsp), %rsi
	movl	%ecx, 16(%rsi,%rdi,4)
	cmpl	$5, %eax
	je	.LBB15_88
	movl	164(%r13), %ecx
	movq	(%rsp), %rsi
	movl	%ecx, 20(%rsi,%rdi,4)
	cmpl	$6, %eax
	je	.LBB15_88
	movl	196(%r13), %ecx
	movq	(%rsp), %rsi
	movl	%ecx, 24(%rsi,%rdi,4)
.LBB15_88:
	cmpq	%r14, %r8
	jbe	.LBB15_148
	subq	%rdx, %r8
	cmpq	%r14, %r8
	movq	%r14, %rcx
	cmovbq	%r8, %rcx
	cmpq	%rcx, %rbx
	ja	.LBB15_191
	movq	(%rsp), %rsi
	leaq	(%rsi,%rdx,4), %rsi
	testq	%rax, %rax
	je	.LBB15_98
	movl	8(%r13), %edx
	movl	%edx, (%rsi,%rdi,4)
	cmpl	$1, %eax
	je	.LBB15_98
	movl	40(%r13), %edx
	movl	%edx, 4(%rsi,%rdi,4)
	cmpl	$2, %eax
	je	.LBB15_98
	movl	72(%r13), %edx
	movl	%edx, 8(%rsi,%rdi,4)
	cmpl	$3, %eax
	je	.LBB15_98
	movl	104(%r13), %edx
	movl	%edx, 12(%rsi,%rdi,4)
	cmpl	$4, %eax
	je	.LBB15_98
	movl	136(%r13), %edx
	movl	%edx, 16(%rsi,%rdi,4)
	cmpl	$5, %eax
	je	.LBB15_98
	movl	168(%r13), %edx
	movl	%edx, 20(%rsi,%rdi,4)
	cmpl	$6, %eax
	je	.LBB15_98
	movl	200(%r13), %edx
	movl	%edx, 24(%rsi,%rdi,4)
.LBB15_98:
	cmpq	%r14, %r8
	jbe	.LBB15_148
	subq	%rcx, %r8
	cmpq	%r14, %r8
	movq	%r14, %rdx
	cmovbq	%r8, %rdx
	cmpq	%rdx, %rbx
	ja	.LBB15_172
	leaq	(%rsi,%rcx,4), %rsi
	testq	%rax, %rax
	je	.LBB15_108
	movl	12(%r13), %ecx
	movl	%ecx, (%rsi,%rdi,4)
	cmpl	$1, %eax
	je	.LBB15_108
	movl	44(%r13), %ecx
	movl	%ecx, 4(%rsi,%rdi,4)
	cmpl	$2, %eax
	je	.LBB15_108
	movl	76(%r13), %ecx
	movl	%ecx, 8(%rsi,%rdi,4)
	cmpl	$3, %eax
	je	.LBB15_108
	movl	108(%r13), %ecx
	movl	%ecx, 12(%rsi,%rdi,4)
	cmpl	$4, %eax
	je	.LBB15_108
	movl	140(%r13), %ecx
	movl	%ecx, 16(%rsi,%rdi,4)
	cmpl	$5, %eax
	je	.LBB15_108
	movl	172(%r13), %ecx
	movl	%ecx, 20(%rsi,%rdi,4)
	cmpl	$6, %eax
	je	.LBB15_108
	movl	204(%r13), %ecx
	movl	%ecx, 24(%rsi,%rdi,4)
.LBB15_108:
	cmpq	%r14, %r8
	jbe	.LBB15_148
	subq	%rdx, %r8
	cmpq	%r14, %r8
	movq	%r14, %rcx
	cmovbq	%r8, %rcx
	cmpq	%rcx, %rbx
	ja	.LBB15_197
	leaq	(%rsi,%rdx,4), %rsi
	testq	%rax, %rax
	je	.LBB15_118
	movl	16(%r13), %edx
	movl	%edx, (%rsi,%rdi,4)
	cmpl	$1, %eax
	je	.LBB15_118
	movl	48(%r13), %edx
	movl	%edx, 4(%rsi,%rdi,4)
	cmpl	$2, %eax
	je	.LBB15_118
	movl	80(%r13), %edx
	movl	%edx, 8(%rsi,%rdi,4)
	cmpl	$3, %eax
	je	.LBB15_118
	movl	112(%r13), %edx
	movl	%edx, 12(%rsi,%rdi,4)
	cmpl	$4, %eax
	je	.LBB15_118
	movl	144(%r13), %edx
	movl	%edx, 16(%rsi,%rdi,4)
	cmpl	$5, %eax
	je	.LBB15_118
	movl	176(%r13), %edx
	movl	%edx, 20(%rsi,%rdi,4)
	cmpl	$6, %eax
	je	.LBB15_118
	movl	208(%r13), %edx
	movl	%edx, 24(%rsi,%rdi,4)
.LBB15_118:
	cmpq	%r14, %r8
	jbe	.LBB15_148
	subq	%rcx, %r8
	cmpq	%r14, %r8
	movq	%r14, %rdx
	cmovbq	%r8, %rdx
	cmpq	%rdx, %rbx
	ja	.LBB15_172
	leaq	(%rsi,%rcx,4), %rsi
	testq	%rax, %rax
	je	.LBB15_128
	movl	20(%r13), %ecx
	movl	%ecx, (%rsi,%rdi,4)
	cmpl	$1, %eax
	je	.LBB15_128
	movl	52(%r13), %ecx
	movl	%ecx, 4(%rsi,%rdi,4)
	cmpl	$2, %eax
	je	.LBB15_128
	movl	84(%r13), %ecx
	movl	%ecx, 8(%rsi,%rdi,4)
	cmpl	$3, %eax
	je	.LBB15_128
	movl	116(%r13), %ecx
	movl	%ecx, 12(%rsi,%rdi,4)
	cmpl	$4, %eax
	je	.LBB15_128
	movl	148(%r13), %ecx
	movl	%ecx, 16(%rsi,%rdi,4)
	cmpl	$5, %eax
	je	.LBB15_128
	movl	180(%r13), %ecx
	movl	%ecx, 20(%rsi,%rdi,4)
	cmpl	$6, %eax
	je	.LBB15_128
	movl	212(%r13), %ecx
	movl	%ecx, 24(%rsi,%rdi,4)
.LBB15_128:
	cmpq	%r14, %r8
	jbe	.LBB15_148
	subq	%rdx, %r8
	cmpq	%r14, %r8
	movq	%r14, %rcx
	cmovbq	%r8, %rcx
	cmpq	%rcx, %rbx
	ja	.LBB15_197
	leaq	(%rsi,%rdx,4), %rdx
	testq	%rax, %rax
	je	.LBB15_138
	movl	24(%r13), %esi
	movl	%esi, (%rdx,%rdi,4)
	cmpl	$1, %eax
	je	.LBB15_138
	movl	56(%r13), %esi
	movl	%esi, 4(%rdx,%rdi,4)
	cmpl	$2, %eax
	je	.LBB15_138
	movl	88(%r13), %esi
	movl	%esi, 8(%rdx,%rdi,4)
	cmpl	$3, %eax
	je	.LBB15_138
	movl	120(%r13), %esi
	movl	%esi, 12(%rdx,%rdi,4)
	cmpl	$4, %eax
	je	.LBB15_138
	movl	152(%r13), %esi
	movl	%esi, 16(%rdx,%rdi,4)
	cmpl	$5, %eax
	je	.LBB15_138
	movl	184(%r13), %esi
	movl	%esi, 20(%rdx,%rdi,4)
	cmpl	$6, %eax
	je	.LBB15_138
	movl	216(%r13), %esi
	movl	%esi, 24(%rdx,%rdi,4)
.LBB15_138:
	cmpq	%r14, %r8
	jbe	.LBB15_148
	subq	%rcx, %r8
	cmpq	%r14, %r8
	cmovbq	%r8, %r14
	cmpq	%r14, %rbx
	ja	.LBB15_192
	testq	%rax, %rax
	je	.LBB15_148
	leaq	(%rdx,%rcx,4), %rcx
	movl	28(%r13), %edx
	movl	%edx, (%rcx,%rdi,4)
	cmpl	$1, %eax
	je	.LBB15_148
	movl	60(%r13), %edx
	movl	%edx, 4(%rcx,%rdi,4)
	cmpl	$2, %eax
	je	.LBB15_148
	movl	92(%r13), %edx
	movl	%edx, 8(%rcx,%rdi,4)
	cmpl	$3, %eax
	je	.LBB15_148
	movl	124(%r13), %edx
	movl	%edx, 12(%rcx,%rdi,4)
	cmpl	$4, %eax
	je	.LBB15_148
	movl	156(%r13), %edx
	movl	%edx, 16(%rcx,%rdi,4)
	cmpl	$5, %eax
	je	.LBB15_148
	movl	188(%r13), %edx
	movl	%edx, 20(%rcx,%rdi,4)
	cmpl	$6, %eax
	je	.LBB15_148
	movl	220(%r13), %eax
	movl	%eax, 24(%rcx,%rdi,4)
.LBB15_148:
	addq	$248, %rsp
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
.LBB15_149:
	.cfi_def_cfa_offset 304
	vperm2f128	$49, %ymm7, %ymm6, %ymm0
	movq	64(%rsp), %r11
	vmovups	%ymm0, (%r11,%rdi,4)
	.p2align	4
.LBB15_150:
	addq	$8, %rdi
	addq	$256, %r12
	cmpq	%rdx, %r12
	je	.LBB15_69
.LBB15_151:
	leaq	7(%rdi), %rbp
	cmpq	%r14, %rbp
	jae	.LBB15_190
	vmovups	(%r12), %ymm4
	vmovups	32(%r12), %ymm5
	vmovups	64(%r12), %ymm6
	vmovups	96(%r12), %ymm7
	vmovups	128(%r12), %ymm8
	vmovups	160(%r12), %ymm9
	vmovups	192(%r12), %ymm10
	vmovups	224(%r12), %ymm11
	vunpcklps	%ymm5, %ymm4, %ymm2
	vunpcklps	%ymm7, %ymm6, %ymm3
	vunpcklps	%ymm9, %ymm8, %ymm12
	vunpcklps	%ymm11, %ymm10, %ymm13
	vunpcklpd	%ymm3, %ymm2, %ymm0
	vunpcklpd	%ymm13, %ymm12, %ymm1
	vinsertf128	$1, %xmm1, %ymm0, %ymm14
	movq	24(%rsp), %r11
	vmovups	%ymm14, (%r11,%rdi,4)
	cmpq	%r14, 32(%rsp)
	jbe	.LBB15_150
	cmpq	%rcx, %rbp
	jae	.LBB15_167
	vunpckhpd	%ymm3, %ymm2, %ymm2
	vunpckhpd	%ymm13, %ymm12, %ymm3
	vinsertf128	$1, %xmm3, %ymm2, %ymm12
	movq	(%rsp), %r11
	vmovups	%ymm12, (%r11,%rdi,4)
	cmpq	%r14, %rax
	jbe	.LBB15_150
	cmpq	%r8, %rbp
	jae	.LBB15_168
	vunpckhps	%ymm5, %ymm4, %ymm12
	vunpckhps	%ymm7, %ymm6, %ymm6
	vunpckhps	%ymm9, %ymm8, %ymm7
	vunpckhps	%ymm11, %ymm10, %ymm8
	vunpcklpd	%ymm6, %ymm12, %ymm4
	vunpcklpd	%ymm8, %ymm7, %ymm5
	vinsertf128	$1, %xmm5, %ymm4, %ymm9
	vmovups	%ymm9, (%r15,%rdi,4)
	cmpq	%r14, %rsi
	jbe	.LBB15_150
	cmpq	%r10, %rbp
	jae	.LBB15_169
	vunpckhpd	%ymm6, %ymm12, %ymm6
	vunpckhpd	%ymm8, %ymm7, %ymm7
	vinsertf128	$1, %xmm7, %ymm6, %ymm8
	movq	8(%rsp), %r11
	vmovups	%ymm8, (%r11,%rdi,4)
	cmpq	%r14, %r9
	jbe	.LBB15_150
	cmpq	16(%rsp), %rbp
	jae	.LBB15_170
	vperm2f128	$49, %ymm1, %ymm0, %ymm0
	movq	104(%rsp), %r11
	vmovups	%ymm0, (%r11,%rdi,4)
	cmpq	%r14, 112(%rsp)
	jbe	.LBB15_150
	cmpq	56(%rsp), %rbp
	jae	.LBB15_173
	vperm2f128	$49, %ymm3, %ymm2, %ymm0
	movq	88(%rsp), %r11
	vmovups	%ymm0, (%r11,%rdi,4)
	cmpq	%r14, 96(%rsp)
	jbe	.LBB15_150
	cmpq	48(%rsp), %rbp
	jae	.LBB15_189
	vperm2f128	$49, %ymm5, %ymm4, %ymm0
	movq	72(%rsp), %r11
	vmovups	%ymm0, (%r11,%rdi,4)
	cmpq	%r14, 80(%rsp)
	jbe	.LBB15_150
	cmpq	40(%rsp), %rbp
	jb	.LBB15_149
	movq	40(%rsp), %r14
	leaq	8(%rdi), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.34(%rip), %rcx
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_167:
	movq	%rcx, %r14
	leaq	8(%rdi), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.34(%rip), %rcx
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_168:
	movq	%r8, %r14
	leaq	8(%rdi), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.34(%rip), %rcx
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_169:
	movq	%r10, %r14
	leaq	8(%rdi), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.34(%rip), %rcx
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_170:
	movq	16(%rsp), %r14
	leaq	8(%rdi), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.34(%rip), %rcx
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_171:
	xorl	%r14d, %r14d
	movq	%rbx, %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.12(%rip), %rcx
	movq	%r14, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_175:
	movq	%rax, %r14
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.12(%rip), %rcx
	movq	%r14, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_177:
	movq	%rbp, %r14
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.12(%rip), %rcx
	movq	%r14, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_179:
	movq	%rdx, %r14
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.12(%rip), %rcx
	movq	%r14, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_181:
	movq	%r11, %r14
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.12(%rip), %rcx
	movq	%r14, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_183:
	movq	%r8, %r14
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.12(%rip), %rcx
	movq	%r14, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_172:
	movq	%rdx, %r14
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.32(%rip), %rcx
	movq	%rbx, %rsi
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_173:
	movq	56(%rsp), %r14
	leaq	8(%rdi), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.34(%rip), %rcx
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_184:
	movq	%r9, %r14
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.12(%rip), %rcx
	movq	%r14, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_185:
	movq	%r9, %r14
.LBB15_186:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.12(%rip), %rcx
	movq	%r14, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_187:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.0(%rip), %rdi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.31(%rip), %rdx
	movl	$55, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.LBB15_188:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.0(%rip), %rdi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.33(%rip), %rdx
	movl	$55, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.LBB15_189:
	movq	48(%rsp), %r14
.LBB15_190:
	leaq	8(%rdi), %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.34(%rip), %rcx
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_191:
	movq	%rcx, %r14
.LBB15_192:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.32(%rip), %rcx
	movq	%rbx, %rsi
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_193:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.38(%rip), %rdx
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB15_194:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.37(%rip), %rdx
	movq	%rcx, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB15_195:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.36(%rip), %rdx
	movq	%rax, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB15_197:
	movq	%rcx, %r14
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.32(%rip), %rcx
	movq	%rbx, %rsi
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB15_198:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.35(%rip), %rdx
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Lfunc_end15:
	.size	_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control15inverse_97_rows, .Lfunc_end15-_RNvNtCs5dlvf2fXWpN_16outlined_io_main7control15inverse_97_rows
	.cfi_endproc

	.section	.rodata.cst8,"aM",@progbits,8
	.p2align	3, 0x0
.LCPI16_0:
	.long	6
	.long	4
	.section	.text._RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel10fill_dense,"ax",@progbits
	.prefalign	4, .Lfunc_end16, nop
	.type	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel10fill_dense,@function
_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel10fill_dense:
	.cfi_startproc
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r12
	.cfi_def_cfa_offset 32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	.cfi_offset %rbx, -40
	.cfi_offset %r12, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	movq	48(%rsp), %rax
	cmpq	%rax, %r9
	cmovaeq	%rax, %r9
	movq	64(%rsp), %rax
	cmpq	%rax, %r9
	cmovaeq	%rax, %r9
	movq	80(%rsp), %rax
	cmpq	%rax, %r9
	cmovaeq	%rax, %r9
	movq	96(%rsp), %rax
	cmpq	%rax, %r9
	cmovaeq	%rax, %r9
	movq	112(%rsp), %rax
	cmpq	%rax, %r9
	cmovaeq	%rax, %r9
	movq	128(%rsp), %rax
	cmpq	%rax, %r9
	cmovaeq	%rax, %r9
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	testq	%r9, %r9
	je	.LBB16_8
	movq	120(%rsp), %rbx
	movq	104(%rsp), %r11
	movq	88(%rsp), %r10
	movq	72(%rsp), %rsi
	movq	56(%rsp), %rcx
	movq	40(%rsp), %rax
	cmpq	$8, %r9
	jae	.LBB16_3
	xorl	%r14d, %r14d
	jmp	.LBB16_6
.LBB16_3:
	movabsq	$288230376151711736, %r14
	andq	%r9, %r14
	vmovaps	%xmm0, -120(%rsp)
	vbroadcastss	%xmm0, %ymm1
	leaq	(,%r9,4), %r15
	andq	$-32, %r15
	xorl	%r12d, %r12d
	vbroadcastsd	.LCPI16_0(%rip), %ymm0
	vmovups	%ymm0, -104(%rsp)
	.p2align	4
.LBB16_4:
	vmulps	(%rcx,%r12), %ymm1, %ymm5
	vmulps	(%rax,%r12), %ymm1, %ymm6
	vmulps	(%r8,%r12), %ymm1, %ymm7
	vmulps	(%rdx,%r12), %ymm1, %ymm9
	vmulps	(%rbx,%r12), %ymm1, %ymm3
	vmulps	(%r11,%r12), %ymm1, %ymm4
	vmulps	(%r10,%r12), %ymm1, %ymm8
	vmulps	(%rsi,%r12), %ymm1, %ymm10
	vunpcklps	%xmm7, %xmm9, %xmm11
	vmovlhps	%xmm6, %xmm5, %xmm12
	vshufps	$36, %xmm12, %xmm11, %xmm11
	vunpcklps	%xmm8, %xmm10, %xmm12
	vinsertf128	$1, %xmm12, %ymm0, %ymm12
	vmovlhps	%xmm4, %xmm3, %xmm13
	vshufps	$36, %xmm13, %xmm13, %xmm13
	vinsertf128	$1, %xmm13, %ymm0, %ymm13
	vblendps	$192, %ymm13, %ymm12, %ymm12
	vblendps	$15, %ymm11, %ymm12, %ymm0
	vmovups	%ymm0, -40(%rsp)
	vunpcklps	%ymm8, %ymm10, %ymm12
	vunpcklpd	%ymm4, %ymm3, %ymm13
	vshufps	$36, %ymm13, %ymm12, %ymm12
	vunpcklps	%ymm7, %ymm9, %ymm13
	vextractf128	$1, %ymm13, %xmm13
	vunpcklps	%ymm5, %ymm6, %ymm14
	vpermpd	$170, %ymm14, %ymm14
	vblendps	$3, %xmm13, %xmm14, %xmm13
	vblendps	$15, %ymm13, %ymm12, %ymm0
	vmovups	%ymm0, -72(%rsp)
	vshufps	$17, %ymm10, %ymm8, %ymm13
	vunpcklps	%ymm3, %ymm4, %ymm14
	vshufps	$226, %ymm14, %ymm13, %ymm13
	vshufps	$17, %ymm9, %ymm7, %ymm14
	vextractf128	$1, %ymm14, %xmm14
	vshufps	$17, %ymm6, %ymm5, %ymm15
	vmovups	-104(%rsp), %ymm11
	vpermps	%ymm15, %ymm11, %ymm15
	vshufps	$226, %xmm15, %xmm14, %xmm14
	vblendps	$15, %ymm14, %ymm13, %ymm13
	vunpckhps	%ymm8, %ymm10, %ymm14
	vunpckhpd	%ymm4, %ymm3, %ymm15
	vshufps	$36, %ymm15, %ymm14, %ymm14
	vunpckhps	%ymm7, %ymm9, %ymm15
	vextractf128	$1, %ymm15, %xmm15
	vunpckhps	%ymm5, %ymm6, %ymm2
	vpermpd	$170, %ymm2, %ymm2
	vblendps	$3, %xmm15, %xmm2, %xmm2
	vblendps	$15, %ymm2, %ymm14, %ymm14
	vshufps	$51, %ymm10, %ymm8, %ymm2
	vunpckhps	%ymm3, %ymm4, %ymm15
	vshufps	$226, %ymm15, %ymm2, %ymm2
	vshufps	$51, %ymm9, %ymm7, %ymm15
	vextractf128	$1, %ymm15, %xmm15
	vshufps	$51, %ymm6, %ymm5, %ymm0
	vpermps	%ymm0, %ymm11, %ymm0
	vshufps	$226, %xmm0, %xmm15, %xmm0
	vblendps	$15, %ymm0, %ymm2, %ymm15
	vinsertps	$76, %xmm9, %xmm7, %xmm0
	vunpcklps	%xmm5, %xmm6, %xmm2
	vblendps	$3, %xmm0, %xmm2, %xmm0
	vinsertps	$76, %xmm10, %xmm8, %xmm2
	vinsertf128	$1, %xmm2, %ymm0, %ymm2
	vunpcklps	%xmm3, %xmm4, %xmm11
	vinsertf128	$1, %xmm11, %ymm0, %ymm11
	vblendps	$192, %ymm11, %ymm2, %ymm2
	vblendps	$15, %ymm0, %ymm2, %ymm0
	vunpckhps	%xmm7, %xmm9, %xmm2
	vinsertps	$179, %xmm5, %xmm6, %xmm11
	vblendps	$3, %xmm2, %xmm11, %xmm2
	vunpckhps	%xmm8, %xmm10, %xmm11
	vinsertf128	$1, %xmm11, %ymm0, %ymm11
	vinsertps	$179, %xmm3, %xmm4, %xmm12
	vinsertf128	$1, %xmm12, %ymm0, %ymm12
	vblendps	$192, %ymm12, %ymm11, %ymm11
	vblendps	$15, %ymm2, %ymm11, %ymm2
	vshufps	$51, %xmm9, %xmm7, %xmm7
	vunpckhps	%xmm5, %xmm6, %xmm5
	vshufps	$226, %xmm5, %xmm7, %xmm5
	vshufps	$51, %xmm10, %xmm8, %xmm6
	vunpckhps	%xmm3, %xmm4, %xmm3
	vshufps	$226, %xmm6, %xmm6, %xmm4
	vinsertf128	$1, %xmm4, %ymm0, %ymm4
	vinsertf128	$1, %xmm3, %ymm0, %ymm3
	vblendps	$192, %ymm3, %ymm4, %ymm3
	vblendps	$15, %ymm5, %ymm3, %ymm3
	vmovups	%ymm3, 96(%rdi,%r12,8)
	vmovups	%ymm2, 64(%rdi,%r12,8)
	vmovups	%ymm0, 32(%rdi,%r12,8)
	vmovups	%ymm15, 224(%rdi,%r12,8)
	vmovups	%ymm14, 192(%rdi,%r12,8)
	vmovups	%ymm13, 160(%rdi,%r12,8)
	vmovups	-72(%rsp), %ymm0
	vmovups	%ymm0, 128(%rdi,%r12,8)
	vmovups	-40(%rsp), %ymm0
	vmovups	%ymm0, (%rdi,%r12,8)
	addq	$32, %r12
	cmpq	%r12, %r15
	jne	.LBB16_4
	cmpq	%r14, %r9
	vmovaps	-120(%rsp), %xmm0
	je	.LBB16_8
.LBB16_6:
	vbroadcastss	%xmm0, %ymm0
	subq	%r14, %r9
	movq	%r14, %r15
	shlq	$5, %r15
	addq	%r15, %rdi
	leaq	(%rbx,%r14,4), %rbx
	leaq	(%r11,%r14,4), %r11
	leaq	(%r10,%r14,4), %r10
	leaq	(%rsi,%r14,4), %rsi
	leaq	(%rcx,%r14,4), %rcx
	leaq	(%rax,%r14,4), %rax
	leaq	(%r8,%r14,4), %r8
	leaq	(%rdx,%r14,4), %rdx
	xorl	%r14d, %r14d
	.p2align	4
.LBB16_7:
	vmovss	(%rsi,%r14), %xmm1
	vinsertps	$16, (%r10,%r14), %xmm1, %xmm1
	vinsertps	$32, (%r11,%r14), %xmm1, %xmm1
	vinsertps	$48, (%rbx,%r14), %xmm1, %xmm1
	vmovss	(%rdx,%r14), %xmm2
	vinsertps	$16, (%r8,%r14), %xmm2, %xmm2
	vinsertps	$32, (%rax,%r14), %xmm2, %xmm2
	vinsertps	$48, (%rcx,%r14), %xmm2, %xmm2
	vinsertf128	$1, %xmm1, %ymm2, %ymm1
	vmulps	%ymm1, %ymm0, %ymm1
	vmovups	%ymm1, (%rdi,%r14,8)
	addq	$4, %r14
	decq	%r9
	jne	.LBB16_7
.LBB16_8:
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r12
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.Lfunc_end16:
	.size	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel10fill_dense, .Lfunc_end16-_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel10fill_dense
	.cfi_endproc

	.section	.text._RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel13scatter_dense,"ax",@progbits
	.prefalign	4, .Lfunc_end17, nop
	.type	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel13scatter_dense,@function
_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel13scatter_dense:
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
	subq	$360, %rsp
	.cfi_def_cfa_offset 416
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	520(%rsp), %r10
	movq	504(%rsp), %r11
	movq	488(%rsp), %rbx
	movq	472(%rsp), %r14
	movq	456(%rsp), %r15
	movq	440(%rsp), %r12
	movq	424(%rsp), %r13
	movq	%r9, 24(%rsp)
	movq	%r9, %rax
	shrq	%rax
	movq	%r13, %r9
	shrq	%r9
	cmpq	%r9, %rax
	cmovbq	%rax, %r9
	movq	%r12, %rax
	shrq	%rax
	cmpq	%rax, %r9
	cmovbq	%r9, %rax
	movq	%r15, %r9
	shrq	%r9
	cmpq	%r9, %rax
	cmovbq	%rax, %r9
	movq	%r14, %rax
	shrq	%rax
	cmpq	%rax, %r9
	cmovbq	%r9, %rax
	movq	%rbx, %r9
	shrq	%r9
	cmpq	%r9, %rax
	cmovbq	%rax, %r9
	movq	%r11, %rax
	shrq	%rax
	cmpq	%rax, %r9
	cmovbq	%r9, %rax
	movq	%r10, %r9
	shrq	%r9
	cmpq	%r9, %rax
	cmovbq	%rax, %r9
	cmpq	%rcx, %rsi
	movq	%rsi, 16(%rsp)
	cmovbq	%rsi, %rcx
	movq	%r8, 8(%rsp)
	cmpq	%rcx, %r9
	cmovbq	%r9, %rcx
	testq	%rcx, %rcx
	je	.LBB17_8
	cmpq	$8, %rcx
	jae	.LBB17_3
	xorl	%r13d, %r13d
	jmp	.LBB17_6
.LBB17_3:
	movabsq	$288230376151711736, %r13
	andq	%rcx, %r13
	leaq	(,%rcx,8), %r12
	andq	$-64, %r12
	xorl	%r15d, %r15d
	movq	512(%rsp), %rax
	movq	496(%rsp), %rsi
	movq	480(%rsp), %r8
	movq	464(%rsp), %r9
	movq	448(%rsp), %r10
	movq	8(%rsp), %r11
	movq	432(%rsp), %rbx
	movq	416(%rsp), %r14
	.p2align	4
.LBB17_4:
	vmovups	(%rdx,%r15,4), %xmm11
	vmovups	32(%rdx,%r15,4), %xmm7
	vmovups	64(%rdx,%r15,4), %xmm5
	vmovups	(%rdi,%r15,4), %xmm12
	vmovups	32(%rdi,%r15,4), %xmm4
	vmovaps	%xmm4, 288(%rsp)
	vmovups	64(%rdi,%r15,4), %xmm6
	vunpcklps	%xmm11, %xmm12, %xmm1
	vmovaps	%xmm12, 80(%rsp)
	vbroadcastss	%xmm7, %xmm2
	vmovaps	%xmm7, 256(%rsp)
	vbroadcastss	%xmm4, %xmm3
	vunpcklps	%xmm2, %xmm3, %xmm2
	vblendps	$12, %xmm2, %xmm1, %xmm1
	vunpcklps	%xmm5, %xmm6, %xmm2
	vmovaps	%xmm6, %xmm4
	vmovaps	%xmm5, %xmm3
	vinsertf128	$1, %xmm2, %ymm0, %ymm2
	vmovups	96(%rdx,%r15,4), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vbroadcastss	%xmm0, %xmm5
	vmovups	96(%rdi,%r15,4), %xmm10
	vbroadcastss	%xmm10, %xmm6
	vunpcklps	%xmm5, %xmm6, %xmm5
	vinsertf128	$1, %xmm5, %ymm0, %ymm5
	vblendps	$192, %ymm5, %ymm2, %ymm2
	vblendps	$15, %ymm1, %ymm2, %ymm6
	vmovups	128(%rdx,%r15,4), %xmm5
	vmovups	128(%rdi,%r15,4), %xmm13
	vunpcklps	%xmm5, %xmm13, %xmm2
	vmovaps	%xmm13, 192(%rsp)
	vmovaps	%xmm5, 160(%rsp)
	vmovups	160(%rdx,%r15,4), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vbroadcastss	%xmm0, %xmm8
	vmovups	160(%rdi,%r15,4), %xmm0
	vmovaps	%xmm0, 128(%rsp)
	vbroadcastss	%xmm0, %xmm9
	vunpcklps	%xmm8, %xmm9, %xmm8
	vblendps	$12, %xmm8, %xmm2, %xmm2
	vmovups	224(%rdx,%r15,4), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vbroadcastss	%xmm0, %xmm14
	vmovups	224(%rdi,%r15,4), %xmm0
	vmovaps	%xmm0, 112(%rsp)
	vbroadcastss	%xmm0, %xmm15
	vunpcklps	%xmm14, %xmm15, %xmm0
	vmovups	192(%rdx,%r15,4), %xmm14
	vmovups	192(%rdi,%r15,4), %xmm15
	vunpcklps	%xmm14, %xmm15, %xmm1
	vinsertf128	$1, %xmm1, %ymm0, %ymm1
	vinsertf128	$1, %xmm0, %ymm0, %ymm0
	vblendps	$192, %ymm0, %ymm1, %ymm0
	vblendps	$15, %ymm2, %ymm0, %ymm0
	vmovups	%ymm0, 32(%r11,%r15)
	vmovups	%ymm6, (%r11,%r15)
	vshufps	$85, %xmm12, %xmm12, %xmm0
	vmovaps	%xmm11, %xmm8
	vblendps	$2, %xmm11, %xmm0, %xmm0
	vmovaps	288(%rsp), %xmm6
	vunpcklps	%xmm7, %xmm6, %xmm1
	vblendps	$12, %xmm1, %xmm0, %xmm7
	vmovaps	48(%rsp), %xmm0
	vunpcklps	%xmm0, %xmm10, %xmm1
	vmovaps	%xmm10, 224(%rsp)
	vinsertf128	$1, %xmm1, %ymm0, %ymm1
	vshufps	$85, %xmm4, %xmm4, %xmm2
	vblendps	$2, %xmm3, %xmm2, %xmm2
	vmovaps	%xmm3, %xmm11
	vinsertf128	$1, %xmm2, %ymm0, %ymm2
	vblendps	$192, %ymm1, %ymm2, %ymm1
	vblendps	$15, %ymm7, %ymm1, %ymm9
	vshufps	$85, %xmm13, %xmm13, %xmm1
	vblendps	$2, %xmm5, %xmm1, %xmm1
	vmovaps	144(%rsp), %xmm12
	vmovaps	128(%rsp), %xmm13
	vunpcklps	%xmm12, %xmm13, %xmm2
	vblendps	$12, %xmm2, %xmm1, %xmm1
	vmovaps	32(%rsp), %xmm3
	vmovaps	112(%rsp), %xmm5
	vunpcklps	%xmm3, %xmm5, %xmm2
	vinsertf128	$1, %xmm2, %ymm0, %ymm2
	vshufps	$85, %xmm15, %xmm15, %xmm7
	vblendps	$2, %xmm14, %xmm7, %xmm7
	vinsertf128	$1, %xmm7, %ymm0, %ymm7
	vblendps	$192, %ymm2, %ymm7, %ymm2
	vblendps	$15, %ymm1, %ymm2, %ymm1
	vmovups	%ymm1, 32(%r14,%r15)
	vmovups	%ymm9, (%r14,%r15)
	vmovaps	80(%rsp), %xmm1
	vunpckhps	%xmm8, %xmm1, %xmm2
	vunpckhps	%xmm11, %xmm4, %xmm8
	vinsertf128	$1, %xmm8, %ymm0, %ymm4
	vshufps	$170, %xmm0, %xmm0, %xmm7
	vblendps	$8, %xmm7, %xmm10, %xmm7
	vinsertf128	$1, %xmm7, %ymm0, %ymm7
	vblendps	$192, %ymm7, %ymm4, %ymm0
	vmovaps	256(%rsp), %xmm1
	vshufps	$170, %xmm1, %xmm1, %xmm7
	vblendps	$8, %xmm7, %xmm6, %xmm7
	vmovaps	%xmm6, %xmm4
	vblendps	$12, %xmm7, %xmm2, %xmm7
	vblendps	$15, %ymm7, %ymm0, %ymm10
	vmovups	160(%rdi,%r15,4), %ymm0
	vmovups	%ymm0, 80(%rsp)
	vmovaps	192(%rsp), %xmm0
	vunpckhps	160(%rsp), %xmm0, %xmm0
	vunpckhps	%xmm14, %xmm15, %xmm14
	vinsertf128	$1, %xmm14, %ymm0, %ymm7
	vshufps	$170, %xmm3, %xmm3, %xmm11
	vblendps	$8, %xmm11, %xmm5, %xmm11
	vmovaps	%xmm5, %xmm9
	vinsertf128	$1, %xmm11, %ymm0, %ymm11
	vblendps	$192, %ymm11, %ymm7, %ymm7
	vmovaps	%xmm12, %xmm5
	vshufps	$170, %xmm12, %xmm12, %xmm11
	vmovaps	%xmm13, %xmm6
	vblendps	$8, %xmm11, %xmm13, %xmm11
	vblendps	$12, %xmm11, %xmm0, %xmm11
	vblendps	$15, %ymm11, %ymm7, %ymm11
	vmovups	128(%rdi,%r15,4), %ymm3
	vmovups	%ymm3, 160(%rsp)
	vmovups	%ymm11, 32(%rbx,%r15)
	vmovups	%ymm10, (%rbx,%r15)
	vmovups	32(%rdi,%r15,4), %ymm10
	vmovups	%ymm10, 192(%rsp)
	vmovups	(%rdi,%r15,4), %ymm12
	vunpckhps	%xmm1, %xmm4, %xmm13
	vmovups	96(%rdi,%r15,4), %ymm15
	vmovups	%ymm15, 320(%rsp)
	vunpckhpd	%xmm13, %xmm2, %xmm1
	vmovups	64(%rdi,%r15,4), %ymm4
	vmovups	%ymm4, 288(%rsp)
	vmovaps	224(%rsp), %xmm2
	vunpckhps	48(%rsp), %xmm2, %xmm3
	vmovups	160(%rdx,%r15,4), %ymm13
	vmovups	%ymm13, 224(%rsp)
	vinsertf128	$1, %xmm3, %ymm0, %ymm3
	vshufps	$238, %xmm8, %xmm8, %xmm2
	vinsertf128	$1, %xmm2, %ymm0, %ymm2
	vblendps	$192, %ymm3, %ymm2, %ymm2
	vmovups	128(%rdx,%r15,4), %ymm3
	vblendps	$15, %ymm1, %ymm2, %ymm1
	vmovups	32(%rdx,%r15,4), %ymm7
	vmovups	%ymm7, 256(%rsp)
	vunpckhps	%xmm5, %xmm6, %xmm2
	vmovups	(%rdx,%r15,4), %ymm11
	vunpckhpd	%xmm2, %xmm0, %xmm0
	vmovups	96(%rdx,%r15,4), %ymm2
	vmovups	%ymm2, 48(%rsp)
	vunpckhps	32(%rsp), %xmm9, %xmm6
	vmovups	64(%rdx,%r15,4), %ymm8
	vinsertf128	$1, %xmm6, %ymm0, %ymm6
	vshufps	$238, %xmm14, %xmm14, %xmm9
	vinsertf128	$1, %xmm9, %ymm0, %ymm9
	vblendps	$192, %ymm6, %ymm9, %ymm6
	vblendps	$15, %ymm0, %ymm6, %ymm0
	vmovups	%ymm0, 32(%r10,%r15)
	vmovups	%ymm1, (%r10,%r15)
	vunpcklps	%ymm8, %ymm4, %ymm0
	vunpcklps	%ymm11, %ymm12, %ymm1
	vextractf128	$1, %ymm1, %xmm1
	vunpcklps	%ymm7, %ymm10, %ymm4
	vpermpd	$170, %ymm4, %ymm6
	vblendps	$12, %xmm6, %xmm1, %xmm1
	vunpcklps	%ymm2, %ymm15, %ymm6
	vunpcklpd	%ymm6, %ymm0, %ymm0
	vblendps	$15, %ymm1, %ymm0, %ymm7
	vmovups	160(%rsp), %ymm5
	vunpcklps	%ymm3, %ymm5, %ymm0
	vmovaps	%ymm3, %ymm10
	vextractf128	$1, %ymm0, %xmm0
	vmovups	80(%rsp), %ymm1
	vunpcklps	%ymm13, %ymm1, %ymm3
	vpermpd	$170, %ymm3, %ymm9
	vblendps	$12, %xmm9, %xmm0, %xmm2
	vmovups	224(%rdi,%r15,4), %ymm9
	vmovups	192(%rdi,%r15,4), %ymm14
	vmovups	224(%rdx,%r15,4), %ymm13
	vmovups	192(%rdx,%r15,4), %ymm15
	vunpcklps	%ymm15, %ymm14, %ymm0
	vunpcklps	%ymm13, %ymm9, %ymm1
	vunpcklpd	%ymm1, %ymm0, %ymm0
	vblendps	$15, %ymm2, %ymm0, %ymm0
	vmovups	%ymm0, 32(%r9,%r15)
	vmovups	%ymm7, (%r9,%r15)
	vbroadcastss	84(%rdi,%r15,4), %ymm0
	vblendps	$32, %ymm8, %ymm0, %ymm0
	vblendps	$192, %ymm6, %ymm0, %ymm0
	vextractf128	$1, %ymm4, %xmm2
	vshufps	$85, %ymm12, %ymm12, %ymm4
	vblendps	$34, %ymm11, %ymm4, %ymm4
	vextractf128	$1, %ymm4, %xmm4
	vblendps	$12, %xmm2, %xmm4, %xmm2
	vblendps	$15, %ymm2, %ymm0, %ymm0
	vbroadcastss	212(%rdi,%r15,4), %ymm2
	vblendps	$32, %ymm15, %ymm2, %ymm2
	vblendps	$192, %ymm1, %ymm2, %ymm1
	vextractf128	$1, %ymm3, %xmm2
	vmovaps	%ymm5, %ymm6
	vshufps	$85, %ymm5, %ymm5, %ymm3
	vblendps	$34, %ymm10, %ymm3, %ymm3
	vextractf128	$1, %ymm3, %xmm3
	vblendps	$12, %xmm2, %xmm3, %xmm2
	vblendps	$15, %ymm2, %ymm1, %ymm1
	vmovups	%ymm1, 32(%r8,%r15)
	vmovups	%ymm0, (%r8,%r15)
	vbroadcastss	120(%rdx,%r15,4), %ymm0
	vblendps	$127, 320(%rsp), %ymm0, %ymm0
	vmovups	288(%rsp), %ymm1
	vunpckhps	%ymm8, %ymm1, %ymm1
	vunpckhps	%ymm11, %ymm12, %ymm2
	vextractf128	$1, %ymm2, %xmm3
	vmovups	192(%rsp), %ymm4
	vunpckhps	256(%rsp), %ymm4, %ymm4
	vpermpd	$170, %ymm4, %ymm5
	vblendps	$12, %xmm5, %xmm3, %xmm3
	vblendps	$192, %ymm0, %ymm1, %ymm0
	vblendps	$15, %ymm3, %ymm0, %ymm0
	vbroadcastss	248(%rdx,%r15,4), %ymm3
	vblendps	$128, %ymm3, %ymm9, %ymm3
	vunpckhps	%ymm15, %ymm14, %ymm5
	vunpckhps	%ymm10, %ymm6, %ymm6
	vextractf128	$1, %ymm6, %xmm7
	vmovups	80(%rsp), %ymm8
	vunpckhps	224(%rsp), %ymm8, %ymm8
	vpermpd	$170, %ymm8, %ymm9
	vblendps	$12, %xmm9, %xmm7, %xmm7
	vblendps	$192, %ymm3, %ymm5, %ymm3
	vblendps	$15, %ymm7, %ymm3, %ymm3
	vmovups	%ymm3, 32(%rsi,%r15)
	vmovups	%ymm0, (%rsi,%r15)
	vbroadcastss	124(%rdi,%r15,4), %ymm0
	vunpckhps	48(%rsp), %ymm0, %ymm0
	vunpckhpd	%ymm0, %ymm1, %ymm0
	vextractf128	$1, %ymm4, %xmm1
	vshufps	$238, %ymm2, %ymm2, %ymm2
	vextractf128	$1, %ymm2, %xmm2
	vblendps	$12, %xmm1, %xmm2, %xmm1
	vblendps	$15, %ymm1, %ymm0, %ymm0
	vbroadcastss	252(%rdi,%r15,4), %ymm1
	vunpckhps	%ymm13, %ymm1, %ymm1
	vunpckhpd	%ymm1, %ymm5, %ymm1
	vextractf128	$1, %ymm8, %xmm2
	vshufps	$238, %ymm6, %ymm6, %ymm3
	vextractf128	$1, %ymm3, %xmm3
	vblendps	$12, %xmm2, %xmm3, %xmm2
	vblendps	$15, %ymm2, %ymm1, %ymm1
	vmovups	%ymm1, 32(%rax,%r15)
	vmovups	%ymm0, (%rax,%r15)
	addq	$64, %r15
	cmpq	%r15, %r12
	jne	.LBB17_4
	cmpq	%r13, %rcx
	je	.LBB17_8
.LBB17_6:
	subq	%r13, %rcx
	movq	512(%rsp), %rax
	leaq	(%rax,%r13,8), %r8
	addq	$4, %r8
	movq	496(%rsp), %rax
	leaq	4(%rax,%r13,8), %r12
	movq	480(%rsp), %rax
	leaq	4(%rax,%r13,8), %r9
	movq	464(%rsp), %rax
	leaq	4(%rax,%r13,8), %rsi
	movq	448(%rsp), %rax
	movq	%r13, %r11
	leaq	4(%rax,%r13,8), %r13
	movq	432(%rsp), %rax
	leaq	4(%rax,%r11,8), %rbp
	movq	416(%rsp), %rax
	leaq	(%rax,%r11,8), %rax
	addq	$4, %rax
	movq	8(%rsp), %r10
	leaq	(%r10,%r11,8), %r10
	addq	$4, %r10
	shlq	$5, %r11
	addq	%r11, %rdx
	addq	$28, %rdx
	addq	%rdi, %r11
	addq	$28, %r11
	xorl	%ebx, %ebx
	.p2align	4
.LBB17_7:
	movl	-28(%r11,%rbx,4), %r14d
	movl	-28(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%r10,%rbx)
	movl	%r15d, (%r10,%rbx)
	movl	-24(%r11,%rbx,4), %r14d
	movl	-24(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%rax,%rbx)
	movl	%r15d, (%rax,%rbx)
	movl	-20(%r11,%rbx,4), %r14d
	movl	-20(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%rbp,%rbx)
	movl	%r15d, (%rbp,%rbx)
	movl	-16(%r11,%rbx,4), %r14d
	movl	-16(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%r13,%rbx)
	movl	%r15d, (%r13,%rbx)
	movl	-12(%r11,%rbx,4), %r14d
	movl	-12(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%rsi,%rbx)
	movl	%r15d, (%rsi,%rbx)
	movl	-8(%r11,%rbx,4), %r14d
	movl	-8(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%r9,%rbx)
	movl	%r15d, (%r9,%rbx)
	movl	-4(%r11,%rbx,4), %r14d
	movl	-4(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%r12,%rbx)
	movl	%r15d, (%r12,%rbx)
	movl	(%r11,%rbx,4), %r14d
	movl	(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%r8,%rbx)
	movl	%r15d, (%r8,%rbx)
	addq	$8, %rbx
	decq	%rcx
	jne	.LBB17_7
.LBB17_8:
	movq	24(%rsp), %rbx
	testb	$1, %bl
	je	.LBB17_18
	movq	16(%rsp), %rax
	subq	$1, %rax
	jb	.LBB17_19
	shlq	$5, %rax
	movl	(%rdi,%rax), %r8d
	movl	4(%rdi,%rax), %r11d
	movl	8(%rdi,%rax), %r10d
	movl	12(%rdi,%rax), %r9d
	movl	16(%rdi,%rax), %esi
	movl	20(%rdi,%rax), %edx
	movl	24(%rdi,%rax), %ecx
	movl	28(%rdi,%rax), %eax
	movq	8(%rsp), %rdi
	movl	%r8d, -4(%rdi,%rbx,4)
	movq	424(%rsp), %rdi
	testq	%rdi, %rdi
	movq	416(%rsp), %r8
	je	.LBB17_20
	movl	%r11d, -4(%r8,%rdi,4)
	movq	440(%rsp), %rdi
	testq	%rdi, %rdi
	je	.LBB17_21
	movq	432(%rsp), %r8
	movl	%r10d, -4(%r8,%rdi,4)
	movq	456(%rsp), %rdi
	testq	%rdi, %rdi
	je	.LBB17_22
	movq	448(%rsp), %r8
	movl	%r9d, -4(%r8,%rdi,4)
	movq	472(%rsp), %rdi
	testq	%rdi, %rdi
	je	.LBB17_23
	movq	464(%rsp), %r8
	movl	%esi, -4(%r8,%rdi,4)
	movq	488(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB17_24
	movq	480(%rsp), %rdi
	movl	%edx, -4(%rdi,%rsi,4)
	movq	504(%rsp), %rdx
	testq	%rdx, %rdx
	je	.LBB17_25
	movq	496(%rsp), %rsi
	movl	%ecx, -4(%rsi,%rdx,4)
	movq	520(%rsp), %rcx
	testq	%rcx, %rcx
	je	.LBB17_26
	movq	512(%rsp), %rdx
	movl	%eax, -4(%rdx,%rcx,4)
.LBB17_18:
	addq	$360, %rsp
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
.LBB17_20:
	.cfi_def_cfa_offset 416
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.43(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB17_21:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.44(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB17_22:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.45(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB17_23:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.46(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB17_24:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.47(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB17_25:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.48(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB17_26:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.49(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB17_19:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.42(%rip), %rdx
	movq	%rax, %rdi
	xorl	%esi, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Lfunc_end17:
	.size	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel13scatter_dense, .Lfunc_end17-_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel13scatter_dense
	.cfi_endproc

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI18_0:
	.long	0x3f9d7658
.LCPI18_1:
	.long	0x3fd01800
.LCPI18_2:
	.long	0xbee31355
.LCPI18_3:
	.long	0x3ee31355
.LCPI18_4:
	.long	0xbf631355
.LCPI18_5:
	.long	0xbf620676
.LCPI18_6:
	.long	0x3f620676
.LCPI18_7:
	.long	0xbfe20676
.LCPI18_8:
	.long	0x3d5901ae
.LCPI18_9:
	.long	0x3dd901ae
.LCPI18_10:
	.long	0x3fcb0673
.LCPI18_11:
	.long	0x404b0673
	.section	.text._RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel15inverse_97_rows,"ax",@progbits
	.prefalign	4, .Lfunc_end18, nop
	.type	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel15inverse_97_rows,@function
_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel15inverse_97_rows:
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
	subq	$120, %rsp
	.cfi_def_cfa_offset 176
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rsi, %r10
	movl	$1, %r12d
	subq	%rcx, %r12
	xorl	%eax, %eax
	testb	$1, %r9b
	cmoveq	%rax, %r12
	cmpq	%rsi, %r9
	ja	.LBB18_112
	movq	%r9, %rsi
	addq	%rdx, %rsi
	jb	.LBB18_132
	cmpq	%r10, %rsi
	ja	.LBB18_132
	leaq	(%rdx,%rdx), %rbx
	movq	%r9, %rsi
	addq	%rbx, %rsi
	jb	.LBB18_123
	cmpq	%r10, %rsi
	ja	.LBB18_123
	leaq	(%rdx,%rdx,2), %rax
	movq	%r9, %rsi
	addq	%rax, %rsi
	jb	.LBB18_125
	cmpq	%r10, %rsi
	ja	.LBB18_125
	leaq	(,%rdx,4), %r11
	movq	%r9, %rsi
	addq	%r11, %rsi
	jb	.LBB18_127
	cmpq	%r10, %rsi
	ja	.LBB18_127
	movq	%rcx, 32(%rsp)
	leaq	(%rdx,%rdx,4), %r15
	movq	%r9, %rsi
	addq	%r15, %rsi
	jb	.LBB18_129
	cmpq	%r10, %rsi
	ja	.LBB18_129
	movq	%r8, %r13
	leaq	(%rdx,%rdx), %rsi
	leaq	(%rsi,%rsi,2), %r8
	movq	%r9, %rsi
	addq	%r8, %rsi
	jb	.LBB18_130
	cmpq	%r10, %rsi
	ja	.LBB18_131
	movq	%r10, 64(%rsp)
	leaq	(,%rdx,8), %r10
	subq	%rdx, %r10
	movq	%r9, %rsi
	addq	%r10, %rsi
	jb	.LBB18_113
	cmpq	64(%rsp), %rsi
	ja	.LBB18_113
	movq	%r9, %rsi
	shrq	%rsi
	addq	%rsi, %r12
	movq	%r9, %rbp
	subq	%r12, %rbp
	jb	.LBB18_133
	movq	%r9, 40(%rsp)
	leaq	(%rdi,%r10,4), %r10
	movq	%r10, 48(%rsp)
	movq	%rdx, 56(%rsp)
	leaq	(%rdi,%rdx,4), %r14
	movq	%rbx, %rcx
	movq	%rdi, %rbx
	leaq	(%rdi,%rcx,4), %rcx
	movq	%rcx, 24(%rsp)
	leaq	(%rdi,%rax,4), %rax
	movq	%rax, 16(%rsp)
	movq	%r12, (%rsp)
	leaq	(%rdi,%r11,4), %r11
	movq	%r11, 96(%rsp)
	leaq	(%rdi,%r15,4), %r15
	movq	%r15, 104(%rsp)
	leaq	(%rdi,%r8,4), %r12
	movq	%r12, 112(%rsp)
	movq	(%rsp), %rax
	shlq	$5, %rax
	movq	%rax, 8(%rsp)
	addq	%r13, %rax
	movq	%rax, 72(%rsp)
	vmovss	.LCPI18_0(%rip), %xmm0
	movq	%r13, %rdi
	movq	(%rsp), %rsi
	movq	%rbx, %rdx
	movq	(%rsp), %rcx
	movq	%r14, %r8
	movq	(%rsp), %r9
	pushq	(%rsp)
	.cfi_adjust_cfa_offset 8
	pushq	%r10
	.cfi_adjust_cfa_offset 8
	pushq	16(%rsp)
	.cfi_adjust_cfa_offset 8
	pushq	%r12
	.cfi_adjust_cfa_offset 8
	pushq	32(%rsp)
	.cfi_adjust_cfa_offset 8
	pushq	%r15
	.cfi_adjust_cfa_offset 8
	pushq	48(%rsp)
	.cfi_adjust_cfa_offset 8
	pushq	%r11
	.cfi_adjust_cfa_offset 8
	pushq	64(%rsp)
	.cfi_adjust_cfa_offset 8
	movq	88(%rsp), %r12
	pushq	%r12
	.cfi_adjust_cfa_offset 8
	pushq	80(%rsp)
	.cfi_adjust_cfa_offset 8
	movq	112(%rsp), %r15
	pushq	%r15
	.cfi_adjust_cfa_offset 8
	callq	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel10fill_dense
	addq	$96, %rsp
	.cfi_adjust_cfa_offset -96
	movq	%rbx, 88(%rsp)
	movq	(%rsp), %rax
	leaq	(%rbx,%rax,4), %rdx
	movq	(%rsp), %rax
	movq	%r14, 80(%rsp)
	leaq	(%r14,%rax,4), %r8
	movq	(%rsp), %rax
	leaq	(%r15,%rax,4), %rax
	movq	(%rsp), %rcx
	leaq	(%r12,%rcx,4), %r10
	movq	(%rsp), %rcx
	movq	96(%rsp), %rsi
	leaq	(%rsi,%rcx,4), %r11
	movq	(%rsp), %rbx
	movq	104(%rsp), %rcx
	leaq	(%rcx,%rbx,4), %r12
	movq	112(%rsp), %rcx
	leaq	(%rcx,%rbx,4), %r14
	movq	48(%rsp), %rcx
	leaq	(%rcx,%rbx,4), %r15
	vmovss	.LCPI18_1(%rip), %xmm0
	movq	72(%rsp), %rdi
	movq	%rbp, %rsi
	movq	%rbp, %rcx
	movq	%rbp, %r9
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r15
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r14
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r12
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r11
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r10
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	callq	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel10fill_dense
	addq	$96, %rsp
	.cfi_adjust_cfa_offset -96
	movq	32(%rsp), %rcx
	testq	%rcx, %rcx
	movq	%rbp, %r12
	je	.LBB18_19
	leaq	-1(%r12), %r15
	cmpq	%r15, %rbx
	movq	%r15, %rax
	cmovbq	%rbx, %rax
	testq	%r12, %r12
	je	.LBB18_114
	movq	%rax, %rsi
	shlq	$5, %rsi
	addq	%r13, %rsi
	movq	%rbx, %rdx
	subq	%rax, %rdx
	movq	%r13, %r14
	movq	40(%rsp), %rbp
	movq	8(%rsp), %rcx
	leaq	(%rcx,%r13), %rdi
	jmp	.LBB18_22
.LBB18_19:
	testq	%rbx, %rbx
	je	.LBB18_135
	testq	%r12, %r12
	je	.LBB18_137
	leaq	-1(%rbx), %rdx
	movq	8(%rsp), %rax
	leaq	(%rax,%r13), %rdi
	vmovups	(%rdi), %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI18_2(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vaddps	(%r13), %ymm0, %ymm0
	vmovups	%ymm0, (%r13)
	leaq	-1(%r12), %r15
	cmpq	%r15, %rdx
	movq	%r15, %rax
	cmovbq	%rdx, %rax
	leaq	32(%r13), %r14
	movq	%rax, %rcx
	shlq	$5, %rcx
	leaq	(%rcx,%r13), %rsi
	addq	$32, %rsi
	subq	%rax, %rdx
	movq	40(%rsp), %rbp
.LBB18_22:
	cmpq	%r12, %rax
	cmovaeq	%r12, %rax
	leaq	32(%rdi), %rcx
	cmpq	%r15, %rax
	cmovaeq	%r15, %rax
	testq	%rax, %rax
	movq	%r15, 16(%rsp)
	je	.LBB18_32
	movq	%rbp, %r15
	leaq	(,%rax,8), %r8
	movq	%rax, %r9
	shlq	$5, %r9
	addq	%r14, %r9
	leaq	(%rax,%rbx), %r10
	shlq	$5, %r10
	leaq	(%r10,%r13), %r11
	addq	%r13, %r10
	addq	$32, %r10
	cmpq	%r11, %r14
	setb	%bl
	cmpq	%r9, %rdi
	setb	%bpl
	cmpq	%r10, %r14
	setb	%r10b
	cmpq	%r9, %rcx
	setb	%r11b
	xorl	%r9d, %r9d
	testb	%bpl, %bl
	jne	.LBB18_29
	andb	%r11b, %r10b
	movq	(%rsp), %rbx
	movq	8(%rsp), %r10
	leaq	(%r13,%r10), %rdi
	movq	%r15, %rbp
	movq	16(%rsp), %r15
	jne	.LBB18_30
	cmpq	$1, %rax
	jne	.LBB18_44
	xorl	%r9d, %r9d
.LBB18_27:
	vbroadcastss	.LCPI18_2(%rip), %ymm0
	.p2align	4
.LBB18_28:
	vmovups	-32(%rcx,%r9,4), %ymm1
	vaddps	(%rcx,%r9,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r14,%r9,4), %ymm1, %ymm1
	vmovups	%ymm1, (%r14,%r9,4)
	addq	$8, %r9
	cmpq	%r9, %r8
	jne	.LBB18_28
	jmp	.LBB18_32
.LBB18_29:
	movq	(%rsp), %rbx
	movq	8(%rsp), %r10
	leaq	(%r10,%r13), %rdi
	movq	%r15, %rbp
	movq	16(%rsp), %r15
.LBB18_30:
	leaq	(%r10,%r13), %rax
	addq	$44, %rax
	vmovss	.LCPI18_3(%rip), %xmm0
	.p2align	4
.LBB18_31:
	vmovss	-44(%rax,%r9,4), %xmm1
	vaddss	-12(%rax,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vmovss	(%r14,%r9,4), %xmm2
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	4(%r14,%r9,4), %xmm2
	vmovss	%xmm1, (%r14,%r9,4)
	vmovss	-40(%rax,%r9,4), %xmm1
	vaddss	-8(%rax,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 4(%r14,%r9,4)
	vmovss	-36(%rax,%r9,4), %xmm1
	vmovss	8(%r14,%r9,4), %xmm2
	vaddss	-4(%rax,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 8(%r14,%r9,4)
	vmovss	-32(%rax,%r9,4), %xmm1
	vmovss	12(%r14,%r9,4), %xmm2
	vaddss	(%rax,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 12(%r14,%r9,4)
	addq	$4, %r9
	cmpq	%r9, %r8
	jne	.LBB18_31
.LBB18_32:
	cmpq	$1, %rdx
	jne	.LBB18_34
	movq	%r15, %rax
	shlq	$5, %rax
	vbroadcastss	.LCPI18_4(%rip), %ymm0
	vmulps	(%rdi,%rax), %ymm0, %ymm0
	vaddps	(%rsi), %ymm0, %ymm0
	vmovups	%ymm0, (%rsi)
.LBB18_34:
	movq	32(%rsp), %r14
	cmpq	$1, %r14
	jne	.LBB18_37
	testq	%rbx, %rbx
	je	.LBB18_137
	vmovups	(%r13), %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI18_5(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vaddps	(%rdi), %ymm0, %ymm0
	vmovups	%ymm0, (%rdi)
	leaq	-1(%rbx), %rax
	cmpq	%rax, %r15
	movq	%rax, %r11
	cmovbq	%r15, %r11
	movq	%r11, %r8
	shlq	$5, %r8
	addq	%rcx, %r8
	movq	%r15, %rdx
	subq	%r11, %rdx
	movq	%rcx, %r9
	jmp	.LBB18_39
.LBB18_37:
	leaq	-1(%rbx), %rax
	cmpq	%rax, %r12
	movq	%rax, %r11
	cmovbq	%r12, %r11
	testq	%rbx, %rbx
	je	.LBB18_114
	movq	%r11, %r8
	shlq	$5, %r8
	addq	%rdi, %r8
	movq	%r12, %rdx
	subq	%r11, %rdx
	movq	%rdi, %r9
.LBB18_39:
	cmpq	%rbx, %r11
	cmovaeq	%rbx, %r11
	leaq	32(%r13), %rsi
	cmpq	%rax, %r11
	movq	%rax, 24(%rsp)
	cmovaeq	%rax, %r11
	testq	%r11, %r11
	je	.LBB18_57
	movq	%rdx, 48(%rsp)
	movq	%r12, %rdx
	leaq	(,%r11,8), %r10
	movq	%r11, %rax
	shlq	$5, %rax
	leaq	(%r9,%rax), %r14
	leaq	(%rax,%r13), %rbx
	addq	%r13, %rax
	addq	$32, %rax
	cmpq	%rbx, %r9
	setb	%r15b
	cmpq	%r14, %r13
	setb	%r12b
	cmpq	%rax, %r9
	setb	%bl
	cmpq	%r14, %rsi
	setb	%bpl
	xorl	%eax, %eax
	testb	%r12b, %r15b
	jne	.LBB18_48
	andb	%bpl, %bl
	movq	40(%rsp), %rbp
	movq	(%rsp), %rbx
	movq	32(%rsp), %r14
	movq	%rdx, %r12
	movq	8(%rsp), %rdx
	leaq	(%r13,%rdx), %rdi
	movq	16(%rsp), %r15
	movq	48(%rsp), %rdx
	jne	.LBB18_49
	cmpq	$4, %r11
	jae	.LBB18_51
	xorl	%eax, %eax
	jmp	.LBB18_55
.LBB18_44:
	movabsq	$2305843009213693936, %r9
	andq	%r8, %r9
	addq	%r13, %r10
	addq	$64, %r10
	xorl	%r11d, %r11d
	vbroadcastss	.LCPI18_3(%rip), %ymm0
	.p2align	4
.LBB18_45:
	vmovups	-32(%r10,%r11,4), %ymm1
	vmovups	(%r14,%r11,4), %ymm2
	vmovups	32(%r14,%r11,4), %ymm3
	vaddps	-64(%r10,%r11,4), %ymm1, %ymm4
	vaddps	(%r10,%r11,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm4, %ymm4
	vsubps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm0, %ymm1, %ymm1
	vsubps	%ymm1, %ymm3, %ymm1
	vmovups	%ymm2, (%r14,%r11,4)
	vmovups	%ymm1, 32(%r14,%r11,4)
	addq	$16, %r11
	cmpq	%r11, %r9
	jne	.LBB18_45
	cmpq	%r9, %r8
	movq	8(%rsp), %r10
	je	.LBB18_32
	testb	$1, %al
	jne	.LBB18_27
	jmp	.LBB18_30
.LBB18_48:
	movq	40(%rsp), %rbp
	movq	(%rsp), %rbx
	movq	32(%rsp), %r14
	movq	%rdx, %r12
	movq	8(%rsp), %rdx
	leaq	(%rdx,%r13), %rdi
	movq	16(%rsp), %r15
	movq	48(%rsp), %rdx
.LBB18_49:
	vmovss	.LCPI18_6(%rip), %xmm0
	.p2align	4
.LBB18_50:
	vmovss	(%r13,%rax,4), %xmm1
	vaddss	32(%r13,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vmovss	(%r9,%rax,4), %xmm2
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	4(%r9,%rax,4), %xmm2
	vmovss	%xmm1, (%r9,%rax,4)
	vmovss	4(%r13,%rax,4), %xmm1
	vaddss	36(%r13,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 4(%r9,%rax,4)
	vmovss	8(%r13,%rax,4), %xmm1
	vmovss	8(%r9,%rax,4), %xmm2
	vaddss	40(%r13,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 8(%r9,%rax,4)
	vmovss	12(%r13,%rax,4), %xmm1
	vmovss	12(%r9,%rax,4), %xmm2
	vaddss	44(%r13,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 12(%r9,%rax,4)
	addq	$4, %rax
	cmpq	%rax, %r10
	jne	.LBB18_50
	jmp	.LBB18_57
.LBB18_51:
	movq	%rbx, %rdx
	movabsq	$9223372036854775776, %rax
	andq	%r10, %rax
	xorl	%ebx, %ebx
	vbroadcastss	.LCPI18_6(%rip), %ymm0
	.p2align	4
.LBB18_52:
	vmovups	32(%r13,%rbx,4), %ymm1
	vmovups	64(%r13,%rbx,4), %ymm2
	vmovups	96(%r13,%rbx,4), %ymm3
	vmovups	(%r9,%rbx,4), %ymm4
	vmovups	32(%r9,%rbx,4), %ymm5
	vmovups	64(%r9,%rbx,4), %ymm6
	vmovups	96(%r9,%rbx,4), %ymm7
	vaddps	(%r13,%rbx,4), %ymm1, %ymm8
	vaddps	%ymm2, %ymm1, %ymm1
	vaddps	%ymm3, %ymm2, %ymm2
	vaddps	128(%r13,%rbx,4), %ymm3, %ymm3
	vmulps	%ymm0, %ymm8, %ymm8
	vsubps	%ymm8, %ymm4, %ymm4
	vmulps	%ymm0, %ymm1, %ymm1
	vsubps	%ymm1, %ymm5, %ymm1
	vmulps	%ymm0, %ymm2, %ymm2
	vsubps	%ymm2, %ymm6, %ymm2
	vmulps	%ymm0, %ymm3, %ymm3
	vsubps	%ymm3, %ymm7, %ymm3
	vmovups	%ymm4, (%r9,%rbx,4)
	vmovups	%ymm1, 32(%r9,%rbx,4)
	vmovups	%ymm2, 64(%r9,%rbx,4)
	vmovups	%ymm3, 96(%r9,%rbx,4)
	addq	$32, %rbx
	cmpq	%rbx, %rax
	jne	.LBB18_52
	cmpq	%rax, %r10
	movq	%rdx, %rbx
	movq	48(%rsp), %rdx
	je	.LBB18_57
	testb	$3, %r11b
	je	.LBB18_49
.LBB18_55:
	vbroadcastss	.LCPI18_5(%rip), %ymm0
	.p2align	4
.LBB18_56:
	vmovups	-32(%rsi,%rax,4), %ymm1
	vaddps	(%rsi,%rax,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r9,%rax,4), %ymm1, %ymm1
	vmovups	%ymm1, (%r9,%rax,4)
	addq	$8, %rax
	cmpq	%rax, %r10
	jne	.LBB18_56
.LBB18_57:
	cmpq	$1, %rdx
	jne	.LBB18_59
	movq	24(%rsp), %rax
	shlq	$5, %rax
	vbroadcastss	.LCPI18_7(%rip), %ymm0
	vmulps	(%r13,%rax), %ymm0, %ymm0
	vaddps	(%r8), %ymm0, %ymm0
	vmovups	%ymm0, (%r8)
.LBB18_59:
	movq	%rbx, %rdx
	movq	%r13, %r8
	testq	%r14, %r14
	jne	.LBB18_61
	vmovups	(%rdi), %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI18_8(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vaddps	(%r13), %ymm0, %ymm0
	vmovups	%ymm0, (%r13)
	movq	24(%rsp), %rdx
	movq	%rsi, %r8
.LBB18_61:
	cmpq	%r15, %rdx
	movq	%r15, %r11
	cmovbq	%rdx, %r11
	movq	%r11, %r9
	shlq	$5, %r9
	addq	%r8, %r9
	subq	%r11, %rdx
	testq	%r11, %r11
	je	.LBB18_71
	leaq	(,%r11,8), %r10
	leaq	(%r11,%rbx), %rax
	shlq	$5, %rax
	leaq	(%rax,%r13), %rbx
	addq	%r13, %rax
	addq	$32, %rax
	cmpq	%rbx, %r8
	setb	%r14b
	cmpq	%r9, %rdi
	setb	%r15b
	cmpq	%rax, %r8
	setb	%bl
	cmpq	%r9, %rcx
	setb	%bpl
	xorl	%eax, %eax
	testb	%r15b, %r14b
	jne	.LBB18_68
	andb	%bpl, %bl
	movq	32(%rsp), %r14
	movq	8(%rsp), %r15
	leaq	(%r13,%r15), %rdi
	movq	40(%rsp), %rbp
	jne	.LBB18_69
	cmpq	$1, %r11
	jne	.LBB18_83
	xorl	%eax, %eax
.LBB18_66:
	vbroadcastss	.LCPI18_8(%rip), %ymm0
	movq	16(%rsp), %r15
	.p2align	4
.LBB18_67:
	vmovups	-32(%rcx,%rax,4), %ymm1
	vaddps	(%rcx,%rax,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r8,%rax,4), %ymm1, %ymm1
	vmovups	%ymm1, (%r8,%rax,4)
	addq	$8, %rax
	cmpq	%rax, %r10
	jne	.LBB18_67
	jmp	.LBB18_71
.LBB18_68:
	movq	32(%rsp), %r14
	movq	8(%rsp), %r15
	leaq	(%r15,%r13), %rdi
	movq	40(%rsp), %rbp
.LBB18_69:
	leaq	(%r15,%r13), %r11
	addq	$44, %r11
	vmovss	.LCPI18_8(%rip), %xmm0
	movq	16(%rsp), %r15
	.p2align	4
.LBB18_70:
	vmovss	-44(%r11,%rax,4), %xmm1
	vaddss	-12(%r11,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	(%r8,%rax,4), %xmm1, %xmm1
	vmovss	%xmm1, (%r8,%rax,4)
	vmovss	-40(%r11,%rax,4), %xmm1
	vaddss	-8(%r11,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	4(%r8,%rax,4), %xmm1, %xmm1
	vmovss	%xmm1, 4(%r8,%rax,4)
	vmovss	-36(%r11,%rax,4), %xmm1
	vaddss	-4(%r11,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	8(%r8,%rax,4), %xmm1, %xmm1
	vmovss	%xmm1, 8(%r8,%rax,4)
	vmovss	-32(%r11,%rax,4), %xmm1
	vaddss	(%r11,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	12(%r8,%rax,4), %xmm1, %xmm1
	vmovss	%xmm1, 12(%r8,%rax,4)
	addq	$4, %rax
	cmpq	%rax, %r10
	jne	.LBB18_70
.LBB18_71:
	cmpq	$1, %rdx
	jne	.LBB18_73
	movq	%r15, %rax
	shlq	$5, %rax
	vbroadcastss	.LCPI18_9(%rip), %ymm0
	vmulps	(%rdi,%rax), %ymm0, %ymm0
	vaddps	(%r9), %ymm0, %ymm0
	vmovups	%ymm0, (%r9)
.LBB18_73:
	movq	%r12, %r15
	movq	%rdi, %r8
	cmpq	$1, %r14
	movq	56(%rsp), %rdx
	jne	.LBB18_75
	vmovups	(%r13), %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI18_10(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vaddps	(%rdi), %ymm0, %ymm0
	vmovups	%ymm0, (%rdi)
	movq	16(%rsp), %r15
	movq	%rcx, %r8
.LBB18_75:
	movq	24(%rsp), %rax
	cmpq	%rax, %r15
	movq	%rax, %r10
	cmovbq	%r15, %r10
	movq	%r10, %r9
	shlq	$5, %r9
	leaq	(%r8,%r9), %rcx
	movq	%rcx, 8(%rsp)
	subq	%r10, %r15
	testq	%r10, %r10
	je	.LBB18_93
	movq	%r14, %rdi
	movq	%rbp, %rax
	leaq	(,%r10,8), %rcx
	leaq	(%r9,%r13), %r11
	addq	%r13, %r9
	addq	$32, %r9
	cmpq	%r11, %r8
	setb	%bpl
	movq	8(%rsp), %rdx
	cmpq	%rdx, %r13
	setb	%r14b
	cmpq	%r9, %r8
	setb	%r11b
	cmpq	%rdx, %rsi
	setb	%bl
	xorl	%r9d, %r9d
	testb	%r14b, %bpl
	jne	.LBB18_80
	andb	%bl, %r11b
	movq	56(%rsp), %rdx
	movq	72(%rsp), %r11
	movq	%rax, %rbp
	movq	%rdi, %r14
	movq	%r11, %rdi
	movq	24(%rsp), %rax
	jne	.LBB18_81
	cmpq	$4, %r10
	jae	.LBB18_87
	xorl	%r9d, %r9d
	jmp	.LBB18_91
.LBB18_80:
	movq	56(%rsp), %rdx
	movq	72(%rsp), %rsi
	movq	%rax, %rbp
	movq	%rdi, %r14
	movq	%rsi, %rdi
	movq	24(%rsp), %rax
.LBB18_81:
	vmovss	.LCPI18_10(%rip), %xmm0
	.p2align	4
.LBB18_82:
	vmovss	(%r13,%r9,4), %xmm1
	vaddss	32(%r13,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	(%r8,%r9,4), %xmm1, %xmm1
	vmovss	%xmm1, (%r8,%r9,4)
	vmovss	4(%r13,%r9,4), %xmm1
	vaddss	36(%r13,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	4(%r8,%r9,4), %xmm1, %xmm1
	vmovss	%xmm1, 4(%r8,%r9,4)
	vmovss	8(%r13,%r9,4), %xmm1
	vaddss	40(%r13,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	8(%r8,%r9,4), %xmm1, %xmm1
	vmovss	%xmm1, 8(%r8,%r9,4)
	vmovss	12(%r13,%r9,4), %xmm1
	vaddss	44(%r13,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	12(%r8,%r9,4), %xmm1, %xmm1
	vmovss	%xmm1, 12(%r8,%r9,4)
	addq	$4, %r9
	cmpq	%r9, %rcx
	jne	.LBB18_82
	jmp	.LBB18_93
.LBB18_83:
	movabsq	$9223372036854775776, %rax
	orq	$16, %rax
	andq	%r10, %rax
	movq	8(%rsp), %rbx
	addq	%r13, %rbx
	addq	$64, %rbx
	xorl	%r14d, %r14d
	vbroadcastss	.LCPI18_8(%rip), %ymm0
	.p2align	4
.LBB18_84:
	vmovups	-32(%rbx,%r14,4), %ymm1
	vaddps	-64(%rbx,%r14,4), %ymm1, %ymm2
	vaddps	(%rbx,%r14,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm2, %ymm2
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r8,%r14,4), %ymm2, %ymm2
	vaddps	32(%r8,%r14,4), %ymm1, %ymm1
	vmovups	%ymm2, (%r8,%r14,4)
	vmovups	%ymm1, 32(%r8,%r14,4)
	addq	$16, %r14
	cmpq	%r14, %rax
	jne	.LBB18_84
	cmpq	%rax, %r10
	movq	32(%rsp), %r14
	movq	16(%rsp), %r15
	je	.LBB18_71
	testb	$1, %r11b
	movq	8(%rsp), %r15
	jne	.LBB18_66
	jmp	.LBB18_69
.LBB18_87:
	movabsq	$9223372036854775776, %r9
	andq	%rcx, %r9
	xorl	%r11d, %r11d
	vbroadcastss	.LCPI18_10(%rip), %ymm0
	.p2align	4
.LBB18_88:
	vmovups	32(%r13,%r11,4), %ymm1
	vmovups	64(%r13,%r11,4), %ymm2
	vmovups	96(%r13,%r11,4), %ymm3
	vaddps	(%r13,%r11,4), %ymm1, %ymm4
	vaddps	%ymm2, %ymm1, %ymm1
	vaddps	%ymm3, %ymm2, %ymm2
	vaddps	128(%r13,%r11,4), %ymm3, %ymm3
	vmulps	%ymm0, %ymm4, %ymm4
	vmulps	%ymm0, %ymm1, %ymm1
	vmulps	%ymm0, %ymm2, %ymm2
	vmulps	%ymm0, %ymm3, %ymm3
	vaddps	(%r8,%r11,4), %ymm4, %ymm4
	vaddps	32(%r8,%r11,4), %ymm1, %ymm1
	vaddps	64(%r8,%r11,4), %ymm2, %ymm2
	vaddps	96(%r8,%r11,4), %ymm3, %ymm3
	vmovups	%ymm4, (%r8,%r11,4)
	vmovups	%ymm1, 32(%r8,%r11,4)
	vmovups	%ymm2, 64(%r8,%r11,4)
	vmovups	%ymm3, 96(%r8,%r11,4)
	addq	$32, %r11
	cmpq	%r11, %r9
	jne	.LBB18_88
	cmpq	%r9, %rcx
	movq	56(%rsp), %rdx
	je	.LBB18_93
	testb	$3, %r10b
	je	.LBB18_81
.LBB18_91:
	vbroadcastss	.LCPI18_10(%rip), %ymm0
	.p2align	4
.LBB18_92:
	vmovups	-32(%rsi,%r9,4), %ymm1
	vaddps	(%rsi,%r9,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r8,%r9,4), %ymm1, %ymm1
	vmovups	%ymm1, (%r8,%r9,4)
	addq	$8, %r9
	cmpq	%r9, %rcx
	jne	.LBB18_92
.LBB18_93:
	cmpq	$1, %r15
	jne	.LBB18_95
	shlq	$5, %rax
	vbroadcastss	.LCPI18_11(%rip), %ymm0
	vmulps	(%r13,%rax), %ymm0, %ymm0
	movq	8(%rsp), %rax
	vaddps	(%rax), %ymm0, %ymm0
	vmovups	%ymm0, (%rax)
.LBB18_95:
	testq	%rdx, %rdx
	movq	64(%rsp), %r10
	je	.LBB18_134
	cmpq	%rdx, %rbp
	ja	.LBB18_121
	cmpq	%rdx, %r10
	jbe	.LBB18_136
	subq	%rdx, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %rax
	cmovbq	%r10, %rax
	cmpq	%rax, %rbp
	ja	.LBB18_115
	cmpq	%rdx, %r10
	jbe	.LBB18_136
	subq	%rax, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %rcx
	cmovbq	%r10, %rcx
	cmpq	%rcx, %rbp
	ja	.LBB18_116
	cmpq	%rdx, %r10
	jbe	.LBB18_136
	subq	%rcx, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %r9
	cmovbq	%r10, %r9
	cmpq	%r9, %rbp
	ja	.LBB18_117
	cmpq	%rdx, %r10
	jbe	.LBB18_136
	subq	%r9, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %rbx
	cmovbq	%r10, %rbx
	cmpq	%rbx, %rbp
	ja	.LBB18_118
	cmpq	%rdx, %r10
	jbe	.LBB18_136
	movq	%r14, %rsi
	subq	%rbx, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %r14
	cmovbq	%r10, %r14
	cmpq	%r14, %rbp
	ja	.LBB18_119
	cmpq	%rdx, %r10
	jbe	.LBB18_136
	subq	%r14, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %r8
	cmovbq	%r10, %r8
	cmpq	%r8, %rbp
	ja	.LBB18_120
	cmpq	%rdx, %r10
	jbe	.LBB18_136
	subq	%r8, %r10
	cmpq	%rdx, %r10
	cmovbq	%r10, %rdx
	cmpq	%rdx, %rbp
	ja	.LBB18_121
	testq	%rsi, %rsi
	movq	%r12, %rsi
	movq	80(%rsp), %r12
	leaq	(%r12,%rax,4), %rax
	leaq	(%rax,%rcx,4), %r10
	leaq	(%r10,%r9,4), %r11
	leaq	(%r11,%rbx,4), %rbx
	leaq	(%rbx,%r14,4), %r14
	leaq	(%r14,%r8,4), %r15
	movq	(%rsp), %r8
	movq	%r8, %rcx
	cmoveq	%rsi, %rcx
	movq	%r13, %rdx
	cmoveq	%rdi, %rdx
	cmoveq	%r8, %rsi
	cmoveq	%r13, %rdi
	movq	88(%rsp), %r8
	movq	%rbp, %r9
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r15
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r14
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%rbx
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r11
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r10
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r12
	.cfi_adjust_cfa_offset 8
	vzeroupper
	callq	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel13scatter_dense
	addq	$232, %rsp
	.cfi_adjust_cfa_offset -232
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
.LBB18_112:
	.cfi_def_cfa_offset 176
	xorl	%edx, %edx
	movq	%r9, %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.14(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_123:
	movq	%rbx, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.14(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_125:
	movq	%rax, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.14(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_127:
	movq	%r11, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.14(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_129:
	movq	%r15, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.14(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_113:
	movq	%r10, %rdx
	movq	64(%rsp), %r10
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.14(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_114:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.41(%rip), %rcx
	movl	$1, %edi
	xorl	%esi, %esi
	xorl	%edx, %edx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_115:
	movq	%rax, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.16(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_116:
	movq	%rcx, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.16(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_117:
	movq	%r9, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.16(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_118:
	movq	%rbx, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.16(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_119:
	movq	%r14, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.16(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_120:
	movq	%r8, %rdx
.LBB18_121:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.16(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_130:
	movq	%r8, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.14(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_131:
	movq	%r8, %rdx
.LBB18_132:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.14(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB18_133:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.28(%rip), %rdi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.50(%rip), %rdx
	movl	$19, %esi
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.LBB18_134:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.0(%rip), %rdi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.51(%rip), %rdx
	movl	$55, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.LBB18_135:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.39(%rip), %rdi
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB18_136:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.15(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB18_137:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.40(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Lfunc_end18:
	.size	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel15inverse_97_rows, .Lfunc_end18-_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel15inverse_97_rows
	.cfi_endproc

	.section	.text._RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel13scatter_dense,"ax",@progbits
	.prefalign	4, .Lfunc_end19, nop
	.type	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel13scatter_dense,@function
_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel13scatter_dense:
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
	subq	$360, %rsp
	.cfi_def_cfa_offset 416
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	520(%rsp), %r10
	movq	504(%rsp), %r11
	movq	488(%rsp), %rbx
	movq	472(%rsp), %r14
	movq	456(%rsp), %r15
	movq	440(%rsp), %r12
	movq	424(%rsp), %r13
	movq	%r9, 24(%rsp)
	movq	%r9, %rax
	shrq	%rax
	movq	%r13, %r9
	shrq	%r9
	cmpq	%r9, %rax
	cmovbq	%rax, %r9
	movq	%r12, %rax
	shrq	%rax
	cmpq	%rax, %r9
	cmovbq	%r9, %rax
	movq	%r15, %r9
	shrq	%r9
	cmpq	%r9, %rax
	cmovbq	%rax, %r9
	movq	%r14, %rax
	shrq	%rax
	cmpq	%rax, %r9
	cmovbq	%r9, %rax
	movq	%rbx, %r9
	shrq	%r9
	cmpq	%r9, %rax
	cmovbq	%rax, %r9
	movq	%r11, %rax
	shrq	%rax
	cmpq	%rax, %r9
	cmovbq	%r9, %rax
	movq	%r10, %r9
	shrq	%r9
	cmpq	%r9, %rax
	cmovbq	%rax, %r9
	cmpq	%rcx, %rsi
	movq	%rsi, 16(%rsp)
	cmovbq	%rsi, %rcx
	movq	%r8, 8(%rsp)
	cmpq	%rcx, %r9
	cmovbq	%r9, %rcx
	testq	%rcx, %rcx
	je	.LBB19_8
	cmpq	$8, %rcx
	jae	.LBB19_3
	xorl	%r13d, %r13d
	jmp	.LBB19_6
.LBB19_3:
	movabsq	$288230376151711736, %r13
	andq	%rcx, %r13
	leaq	(,%rcx,8), %r12
	andq	$-64, %r12
	xorl	%r15d, %r15d
	movq	512(%rsp), %rax
	movq	496(%rsp), %rsi
	movq	480(%rsp), %r8
	movq	464(%rsp), %r9
	movq	448(%rsp), %r10
	movq	8(%rsp), %r11
	movq	432(%rsp), %rbx
	movq	416(%rsp), %r14
	.p2align	4
.LBB19_4:
	vmovups	(%rdx,%r15,4), %xmm11
	vmovups	32(%rdx,%r15,4), %xmm7
	vmovups	64(%rdx,%r15,4), %xmm5
	vmovups	(%rdi,%r15,4), %xmm12
	vmovups	32(%rdi,%r15,4), %xmm4
	vmovaps	%xmm4, 288(%rsp)
	vmovups	64(%rdi,%r15,4), %xmm6
	vunpcklps	%xmm11, %xmm12, %xmm1
	vmovaps	%xmm12, 80(%rsp)
	vbroadcastss	%xmm7, %xmm2
	vmovaps	%xmm7, 256(%rsp)
	vbroadcastss	%xmm4, %xmm3
	vunpcklps	%xmm2, %xmm3, %xmm2
	vblendps	$12, %xmm2, %xmm1, %xmm1
	vunpcklps	%xmm5, %xmm6, %xmm2
	vmovaps	%xmm6, %xmm4
	vmovaps	%xmm5, %xmm3
	vinsertf128	$1, %xmm2, %ymm0, %ymm2
	vmovups	96(%rdx,%r15,4), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vbroadcastss	%xmm0, %xmm5
	vmovups	96(%rdi,%r15,4), %xmm10
	vbroadcastss	%xmm10, %xmm6
	vunpcklps	%xmm5, %xmm6, %xmm5
	vinsertf128	$1, %xmm5, %ymm0, %ymm5
	vblendps	$192, %ymm5, %ymm2, %ymm2
	vblendps	$15, %ymm1, %ymm2, %ymm6
	vmovups	128(%rdx,%r15,4), %xmm5
	vmovups	128(%rdi,%r15,4), %xmm13
	vunpcklps	%xmm5, %xmm13, %xmm2
	vmovaps	%xmm13, 192(%rsp)
	vmovaps	%xmm5, 160(%rsp)
	vmovups	160(%rdx,%r15,4), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vbroadcastss	%xmm0, %xmm8
	vmovups	160(%rdi,%r15,4), %xmm0
	vmovaps	%xmm0, 128(%rsp)
	vbroadcastss	%xmm0, %xmm9
	vunpcklps	%xmm8, %xmm9, %xmm8
	vblendps	$12, %xmm8, %xmm2, %xmm2
	vmovups	224(%rdx,%r15,4), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vbroadcastss	%xmm0, %xmm14
	vmovups	224(%rdi,%r15,4), %xmm0
	vmovaps	%xmm0, 112(%rsp)
	vbroadcastss	%xmm0, %xmm15
	vunpcklps	%xmm14, %xmm15, %xmm0
	vmovups	192(%rdx,%r15,4), %xmm14
	vmovups	192(%rdi,%r15,4), %xmm15
	vunpcklps	%xmm14, %xmm15, %xmm1
	vinsertf128	$1, %xmm1, %ymm0, %ymm1
	vinsertf128	$1, %xmm0, %ymm0, %ymm0
	vblendps	$192, %ymm0, %ymm1, %ymm0
	vblendps	$15, %ymm2, %ymm0, %ymm0
	vmovups	%ymm0, 32(%r11,%r15)
	vmovups	%ymm6, (%r11,%r15)
	vshufps	$85, %xmm12, %xmm12, %xmm0
	vmovaps	%xmm11, %xmm8
	vblendps	$2, %xmm11, %xmm0, %xmm0
	vmovaps	288(%rsp), %xmm6
	vunpcklps	%xmm7, %xmm6, %xmm1
	vblendps	$12, %xmm1, %xmm0, %xmm7
	vmovaps	48(%rsp), %xmm0
	vunpcklps	%xmm0, %xmm10, %xmm1
	vmovaps	%xmm10, 224(%rsp)
	vinsertf128	$1, %xmm1, %ymm0, %ymm1
	vshufps	$85, %xmm4, %xmm4, %xmm2
	vblendps	$2, %xmm3, %xmm2, %xmm2
	vmovaps	%xmm3, %xmm11
	vinsertf128	$1, %xmm2, %ymm0, %ymm2
	vblendps	$192, %ymm1, %ymm2, %ymm1
	vblendps	$15, %ymm7, %ymm1, %ymm9
	vshufps	$85, %xmm13, %xmm13, %xmm1
	vblendps	$2, %xmm5, %xmm1, %xmm1
	vmovaps	144(%rsp), %xmm12
	vmovaps	128(%rsp), %xmm13
	vunpcklps	%xmm12, %xmm13, %xmm2
	vblendps	$12, %xmm2, %xmm1, %xmm1
	vmovaps	32(%rsp), %xmm3
	vmovaps	112(%rsp), %xmm5
	vunpcklps	%xmm3, %xmm5, %xmm2
	vinsertf128	$1, %xmm2, %ymm0, %ymm2
	vshufps	$85, %xmm15, %xmm15, %xmm7
	vblendps	$2, %xmm14, %xmm7, %xmm7
	vinsertf128	$1, %xmm7, %ymm0, %ymm7
	vblendps	$192, %ymm2, %ymm7, %ymm2
	vblendps	$15, %ymm1, %ymm2, %ymm1
	vmovups	%ymm1, 32(%r14,%r15)
	vmovups	%ymm9, (%r14,%r15)
	vmovaps	80(%rsp), %xmm1
	vunpckhps	%xmm8, %xmm1, %xmm2
	vunpckhps	%xmm11, %xmm4, %xmm8
	vinsertf128	$1, %xmm8, %ymm0, %ymm4
	vshufps	$170, %xmm0, %xmm0, %xmm7
	vblendps	$8, %xmm7, %xmm10, %xmm7
	vinsertf128	$1, %xmm7, %ymm0, %ymm7
	vblendps	$192, %ymm7, %ymm4, %ymm0
	vmovaps	256(%rsp), %xmm1
	vshufps	$170, %xmm1, %xmm1, %xmm7
	vblendps	$8, %xmm7, %xmm6, %xmm7
	vmovaps	%xmm6, %xmm4
	vblendps	$12, %xmm7, %xmm2, %xmm7
	vblendps	$15, %ymm7, %ymm0, %ymm10
	vmovups	160(%rdi,%r15,4), %ymm0
	vmovups	%ymm0, 80(%rsp)
	vmovaps	192(%rsp), %xmm0
	vunpckhps	160(%rsp), %xmm0, %xmm0
	vunpckhps	%xmm14, %xmm15, %xmm14
	vinsertf128	$1, %xmm14, %ymm0, %ymm7
	vshufps	$170, %xmm3, %xmm3, %xmm11
	vblendps	$8, %xmm11, %xmm5, %xmm11
	vmovaps	%xmm5, %xmm9
	vinsertf128	$1, %xmm11, %ymm0, %ymm11
	vblendps	$192, %ymm11, %ymm7, %ymm7
	vmovaps	%xmm12, %xmm5
	vshufps	$170, %xmm12, %xmm12, %xmm11
	vmovaps	%xmm13, %xmm6
	vblendps	$8, %xmm11, %xmm13, %xmm11
	vblendps	$12, %xmm11, %xmm0, %xmm11
	vblendps	$15, %ymm11, %ymm7, %ymm11
	vmovups	128(%rdi,%r15,4), %ymm3
	vmovups	%ymm3, 160(%rsp)
	vmovups	%ymm11, 32(%rbx,%r15)
	vmovups	%ymm10, (%rbx,%r15)
	vmovups	32(%rdi,%r15,4), %ymm10
	vmovups	%ymm10, 192(%rsp)
	vmovups	(%rdi,%r15,4), %ymm12
	vunpckhps	%xmm1, %xmm4, %xmm13
	vmovups	96(%rdi,%r15,4), %ymm15
	vmovups	%ymm15, 320(%rsp)
	vunpckhpd	%xmm13, %xmm2, %xmm1
	vmovups	64(%rdi,%r15,4), %ymm4
	vmovups	%ymm4, 288(%rsp)
	vmovaps	224(%rsp), %xmm2
	vunpckhps	48(%rsp), %xmm2, %xmm3
	vmovups	160(%rdx,%r15,4), %ymm13
	vmovups	%ymm13, 224(%rsp)
	vinsertf128	$1, %xmm3, %ymm0, %ymm3
	vshufps	$238, %xmm8, %xmm8, %xmm2
	vinsertf128	$1, %xmm2, %ymm0, %ymm2
	vblendps	$192, %ymm3, %ymm2, %ymm2
	vmovups	128(%rdx,%r15,4), %ymm3
	vblendps	$15, %ymm1, %ymm2, %ymm1
	vmovups	32(%rdx,%r15,4), %ymm7
	vmovups	%ymm7, 256(%rsp)
	vunpckhps	%xmm5, %xmm6, %xmm2
	vmovups	(%rdx,%r15,4), %ymm11
	vunpckhpd	%xmm2, %xmm0, %xmm0
	vmovups	96(%rdx,%r15,4), %ymm2
	vmovups	%ymm2, 48(%rsp)
	vunpckhps	32(%rsp), %xmm9, %xmm6
	vmovups	64(%rdx,%r15,4), %ymm8
	vinsertf128	$1, %xmm6, %ymm0, %ymm6
	vshufps	$238, %xmm14, %xmm14, %xmm9
	vinsertf128	$1, %xmm9, %ymm0, %ymm9
	vblendps	$192, %ymm6, %ymm9, %ymm6
	vblendps	$15, %ymm0, %ymm6, %ymm0
	vmovups	%ymm0, 32(%r10,%r15)
	vmovups	%ymm1, (%r10,%r15)
	vunpcklps	%ymm8, %ymm4, %ymm0
	vunpcklps	%ymm11, %ymm12, %ymm1
	vextractf128	$1, %ymm1, %xmm1
	vunpcklps	%ymm7, %ymm10, %ymm4
	vpermpd	$170, %ymm4, %ymm6
	vblendps	$12, %xmm6, %xmm1, %xmm1
	vunpcklps	%ymm2, %ymm15, %ymm6
	vunpcklpd	%ymm6, %ymm0, %ymm0
	vblendps	$15, %ymm1, %ymm0, %ymm7
	vmovups	160(%rsp), %ymm5
	vunpcklps	%ymm3, %ymm5, %ymm0
	vmovaps	%ymm3, %ymm10
	vextractf128	$1, %ymm0, %xmm0
	vmovups	80(%rsp), %ymm1
	vunpcklps	%ymm13, %ymm1, %ymm3
	vpermpd	$170, %ymm3, %ymm9
	vblendps	$12, %xmm9, %xmm0, %xmm2
	vmovups	224(%rdi,%r15,4), %ymm9
	vmovups	192(%rdi,%r15,4), %ymm14
	vmovups	224(%rdx,%r15,4), %ymm13
	vmovups	192(%rdx,%r15,4), %ymm15
	vunpcklps	%ymm15, %ymm14, %ymm0
	vunpcklps	%ymm13, %ymm9, %ymm1
	vunpcklpd	%ymm1, %ymm0, %ymm0
	vblendps	$15, %ymm2, %ymm0, %ymm0
	vmovups	%ymm0, 32(%r9,%r15)
	vmovups	%ymm7, (%r9,%r15)
	vbroadcastss	84(%rdi,%r15,4), %ymm0
	vblendps	$32, %ymm8, %ymm0, %ymm0
	vblendps	$192, %ymm6, %ymm0, %ymm0
	vextractf128	$1, %ymm4, %xmm2
	vshufps	$85, %ymm12, %ymm12, %ymm4
	vblendps	$34, %ymm11, %ymm4, %ymm4
	vextractf128	$1, %ymm4, %xmm4
	vblendps	$12, %xmm2, %xmm4, %xmm2
	vblendps	$15, %ymm2, %ymm0, %ymm0
	vbroadcastss	212(%rdi,%r15,4), %ymm2
	vblendps	$32, %ymm15, %ymm2, %ymm2
	vblendps	$192, %ymm1, %ymm2, %ymm1
	vextractf128	$1, %ymm3, %xmm2
	vmovaps	%ymm5, %ymm6
	vshufps	$85, %ymm5, %ymm5, %ymm3
	vblendps	$34, %ymm10, %ymm3, %ymm3
	vextractf128	$1, %ymm3, %xmm3
	vblendps	$12, %xmm2, %xmm3, %xmm2
	vblendps	$15, %ymm2, %ymm1, %ymm1
	vmovups	%ymm1, 32(%r8,%r15)
	vmovups	%ymm0, (%r8,%r15)
	vbroadcastss	120(%rdx,%r15,4), %ymm0
	vblendps	$127, 320(%rsp), %ymm0, %ymm0
	vmovups	288(%rsp), %ymm1
	vunpckhps	%ymm8, %ymm1, %ymm1
	vunpckhps	%ymm11, %ymm12, %ymm2
	vextractf128	$1, %ymm2, %xmm3
	vmovups	192(%rsp), %ymm4
	vunpckhps	256(%rsp), %ymm4, %ymm4
	vpermpd	$170, %ymm4, %ymm5
	vblendps	$12, %xmm5, %xmm3, %xmm3
	vblendps	$192, %ymm0, %ymm1, %ymm0
	vblendps	$15, %ymm3, %ymm0, %ymm0
	vbroadcastss	248(%rdx,%r15,4), %ymm3
	vblendps	$128, %ymm3, %ymm9, %ymm3
	vunpckhps	%ymm15, %ymm14, %ymm5
	vunpckhps	%ymm10, %ymm6, %ymm6
	vextractf128	$1, %ymm6, %xmm7
	vmovups	80(%rsp), %ymm8
	vunpckhps	224(%rsp), %ymm8, %ymm8
	vpermpd	$170, %ymm8, %ymm9
	vblendps	$12, %xmm9, %xmm7, %xmm7
	vblendps	$192, %ymm3, %ymm5, %ymm3
	vblendps	$15, %ymm7, %ymm3, %ymm3
	vmovups	%ymm3, 32(%rsi,%r15)
	vmovups	%ymm0, (%rsi,%r15)
	vbroadcastss	124(%rdi,%r15,4), %ymm0
	vunpckhps	48(%rsp), %ymm0, %ymm0
	vunpckhpd	%ymm0, %ymm1, %ymm0
	vextractf128	$1, %ymm4, %xmm1
	vshufps	$238, %ymm2, %ymm2, %ymm2
	vextractf128	$1, %ymm2, %xmm2
	vblendps	$12, %xmm1, %xmm2, %xmm1
	vblendps	$15, %ymm1, %ymm0, %ymm0
	vbroadcastss	252(%rdi,%r15,4), %ymm1
	vunpckhps	%ymm13, %ymm1, %ymm1
	vunpckhpd	%ymm1, %ymm5, %ymm1
	vextractf128	$1, %ymm8, %xmm2
	vshufps	$238, %ymm6, %ymm6, %ymm3
	vextractf128	$1, %ymm3, %xmm3
	vblendps	$12, %xmm2, %xmm3, %xmm2
	vblendps	$15, %ymm2, %ymm1, %ymm1
	vmovups	%ymm1, 32(%rax,%r15)
	vmovups	%ymm0, (%rax,%r15)
	addq	$64, %r15
	cmpq	%r15, %r12
	jne	.LBB19_4
	cmpq	%r13, %rcx
	je	.LBB19_8
.LBB19_6:
	subq	%r13, %rcx
	movq	512(%rsp), %rax
	leaq	(%rax,%r13,8), %r8
	addq	$4, %r8
	movq	496(%rsp), %rax
	leaq	4(%rax,%r13,8), %r12
	movq	480(%rsp), %rax
	leaq	4(%rax,%r13,8), %r9
	movq	464(%rsp), %rax
	leaq	4(%rax,%r13,8), %rsi
	movq	448(%rsp), %rax
	movq	%r13, %r11
	leaq	4(%rax,%r13,8), %r13
	movq	432(%rsp), %rax
	leaq	4(%rax,%r11,8), %rbp
	movq	416(%rsp), %rax
	leaq	(%rax,%r11,8), %rax
	addq	$4, %rax
	movq	8(%rsp), %r10
	leaq	(%r10,%r11,8), %r10
	addq	$4, %r10
	shlq	$5, %r11
	addq	%r11, %rdx
	addq	$28, %rdx
	addq	%rdi, %r11
	addq	$28, %r11
	xorl	%ebx, %ebx
	.p2align	4
.LBB19_7:
	movl	-28(%r11,%rbx,4), %r14d
	movl	-28(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%r10,%rbx)
	movl	%r15d, (%r10,%rbx)
	movl	-24(%r11,%rbx,4), %r14d
	movl	-24(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%rax,%rbx)
	movl	%r15d, (%rax,%rbx)
	movl	-20(%r11,%rbx,4), %r14d
	movl	-20(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%rbp,%rbx)
	movl	%r15d, (%rbp,%rbx)
	movl	-16(%r11,%rbx,4), %r14d
	movl	-16(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%r13,%rbx)
	movl	%r15d, (%r13,%rbx)
	movl	-12(%r11,%rbx,4), %r14d
	movl	-12(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%rsi,%rbx)
	movl	%r15d, (%rsi,%rbx)
	movl	-8(%r11,%rbx,4), %r14d
	movl	-8(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%r9,%rbx)
	movl	%r15d, (%r9,%rbx)
	movl	-4(%r11,%rbx,4), %r14d
	movl	-4(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%r12,%rbx)
	movl	%r15d, (%r12,%rbx)
	movl	(%r11,%rbx,4), %r14d
	movl	(%rdx,%rbx,4), %r15d
	movl	%r14d, -4(%r8,%rbx)
	movl	%r15d, (%r8,%rbx)
	addq	$8, %rbx
	decq	%rcx
	jne	.LBB19_7
.LBB19_8:
	movq	24(%rsp), %rbx
	testb	$1, %bl
	je	.LBB19_18
	movq	16(%rsp), %rax
	subq	$1, %rax
	jb	.LBB19_19
	shlq	$5, %rax
	movl	(%rdi,%rax), %r8d
	movl	4(%rdi,%rax), %r11d
	movl	8(%rdi,%rax), %r10d
	movl	12(%rdi,%rax), %r9d
	movl	16(%rdi,%rax), %esi
	movl	20(%rdi,%rax), %edx
	movl	24(%rdi,%rax), %ecx
	movl	28(%rdi,%rax), %eax
	movq	8(%rsp), %rdi
	movl	%r8d, -4(%rdi,%rbx,4)
	movq	424(%rsp), %rdi
	testq	%rdi, %rdi
	movq	416(%rsp), %r8
	je	.LBB19_20
	movl	%r11d, -4(%r8,%rdi,4)
	movq	440(%rsp), %rdi
	testq	%rdi, %rdi
	je	.LBB19_21
	movq	432(%rsp), %r8
	movl	%r10d, -4(%r8,%rdi,4)
	movq	456(%rsp), %rdi
	testq	%rdi, %rdi
	je	.LBB19_22
	movq	448(%rsp), %r8
	movl	%r9d, -4(%r8,%rdi,4)
	movq	472(%rsp), %rdi
	testq	%rdi, %rdi
	je	.LBB19_23
	movq	464(%rsp), %r8
	movl	%esi, -4(%r8,%rdi,4)
	movq	488(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB19_24
	movq	480(%rsp), %rdi
	movl	%edx, -4(%rdi,%rsi,4)
	movq	504(%rsp), %rdx
	testq	%rdx, %rdx
	je	.LBB19_25
	movq	496(%rsp), %rsi
	movl	%ecx, -4(%rsi,%rdx,4)
	movq	520(%rsp), %rcx
	testq	%rcx, %rcx
	je	.LBB19_26
	movq	512(%rsp), %rdx
	movl	%eax, -4(%rdx,%rcx,4)
.LBB19_18:
	addq	$360, %rsp
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
.LBB19_20:
	.cfi_def_cfa_offset 416
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.56(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB19_21:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.57(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB19_22:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.58(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB19_23:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.59(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB19_24:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.60(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB19_25:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.61(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB19_26:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.62(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB19_19:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.55(%rip), %rdx
	movq	%rax, %rdi
	xorl	%esi, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Lfunc_end19:
	.size	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel13scatter_dense, .Lfunc_end19-_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel13scatter_dense
	.cfi_endproc

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI20_0:
	.long	0x3f9d7658
.LCPI20_1:
	.long	0x3fd01800
.LCPI20_2:
	.long	0xbee31355
.LCPI20_3:
	.long	0x3ee31355
.LCPI20_4:
	.long	0xbf631355
.LCPI20_5:
	.long	0xbf620676
.LCPI20_6:
	.long	0x3f620676
.LCPI20_7:
	.long	0xbfe20676
.LCPI20_8:
	.long	0x3d5901ae
.LCPI20_9:
	.long	0x3dd901ae
.LCPI20_10:
	.long	0x3fcb0673
.LCPI20_11:
	.long	0x404b0673
	.section	.text._RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel15inverse_97_rows,"ax",@progbits
	.prefalign	4, .Lfunc_end20, nop
	.type	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel15inverse_97_rows,@function
_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel15inverse_97_rows:
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
	subq	$120, %rsp
	.cfi_def_cfa_offset 176
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rsi, %r10
	movl	$1, %r12d
	subq	%rcx, %r12
	xorl	%eax, %eax
	testb	$1, %r9b
	cmoveq	%rax, %r12
	cmpq	%rsi, %r9
	ja	.LBB20_112
	movq	%r9, %rsi
	addq	%rdx, %rsi
	jb	.LBB20_132
	cmpq	%r10, %rsi
	ja	.LBB20_132
	leaq	(%rdx,%rdx), %rbx
	movq	%r9, %rsi
	addq	%rbx, %rsi
	jb	.LBB20_123
	cmpq	%r10, %rsi
	ja	.LBB20_123
	leaq	(%rdx,%rdx,2), %rax
	movq	%r9, %rsi
	addq	%rax, %rsi
	jb	.LBB20_125
	cmpq	%r10, %rsi
	ja	.LBB20_125
	leaq	(,%rdx,4), %r11
	movq	%r9, %rsi
	addq	%r11, %rsi
	jb	.LBB20_127
	cmpq	%r10, %rsi
	ja	.LBB20_127
	movq	%rcx, 32(%rsp)
	leaq	(%rdx,%rdx,4), %r15
	movq	%r9, %rsi
	addq	%r15, %rsi
	jb	.LBB20_129
	cmpq	%r10, %rsi
	ja	.LBB20_129
	movq	%r8, %r13
	leaq	(%rdx,%rdx), %rsi
	leaq	(%rsi,%rsi,2), %r8
	movq	%r9, %rsi
	addq	%r8, %rsi
	jb	.LBB20_130
	cmpq	%r10, %rsi
	ja	.LBB20_131
	movq	%r10, 64(%rsp)
	leaq	(,%rdx,8), %r10
	subq	%rdx, %r10
	movq	%r9, %rsi
	addq	%r10, %rsi
	jb	.LBB20_113
	cmpq	64(%rsp), %rsi
	ja	.LBB20_113
	movq	%r9, %rsi
	shrq	%rsi
	addq	%rsi, %r12
	movq	%r9, %rbp
	subq	%r12, %rbp
	jb	.LBB20_133
	movq	%r9, 40(%rsp)
	leaq	(%rdi,%r10,4), %r10
	movq	%r10, 48(%rsp)
	movq	%rdx, 56(%rsp)
	leaq	(%rdi,%rdx,4), %r14
	movq	%rbx, %rcx
	movq	%rdi, %rbx
	leaq	(%rdi,%rcx,4), %rcx
	movq	%rcx, 24(%rsp)
	leaq	(%rdi,%rax,4), %rax
	movq	%rax, 16(%rsp)
	movq	%r12, (%rsp)
	leaq	(%rdi,%r11,4), %r11
	movq	%r11, 96(%rsp)
	leaq	(%rdi,%r15,4), %r15
	movq	%r15, 104(%rsp)
	leaq	(%rdi,%r8,4), %r12
	movq	%r12, 112(%rsp)
	movq	(%rsp), %rax
	shlq	$5, %rax
	movq	%rax, 8(%rsp)
	addq	%r13, %rax
	movq	%rax, 72(%rsp)
	vmovss	.LCPI20_0(%rip), %xmm0
	movq	%r13, %rdi
	movq	(%rsp), %rsi
	movq	%rbx, %rdx
	movq	(%rsp), %rcx
	movq	%r14, %r8
	movq	(%rsp), %r9
	pushq	(%rsp)
	.cfi_adjust_cfa_offset 8
	pushq	%r10
	.cfi_adjust_cfa_offset 8
	pushq	16(%rsp)
	.cfi_adjust_cfa_offset 8
	pushq	%r12
	.cfi_adjust_cfa_offset 8
	pushq	32(%rsp)
	.cfi_adjust_cfa_offset 8
	pushq	%r15
	.cfi_adjust_cfa_offset 8
	pushq	48(%rsp)
	.cfi_adjust_cfa_offset 8
	pushq	%r11
	.cfi_adjust_cfa_offset 8
	pushq	64(%rsp)
	.cfi_adjust_cfa_offset 8
	movq	88(%rsp), %r12
	pushq	%r12
	.cfi_adjust_cfa_offset 8
	pushq	80(%rsp)
	.cfi_adjust_cfa_offset 8
	movq	112(%rsp), %r15
	pushq	%r15
	.cfi_adjust_cfa_offset 8
	callq	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel10fill_dense
	addq	$96, %rsp
	.cfi_adjust_cfa_offset -96
	movq	%rbx, 88(%rsp)
	movq	(%rsp), %rax
	leaq	(%rbx,%rax,4), %rdx
	movq	(%rsp), %rax
	movq	%r14, 80(%rsp)
	leaq	(%r14,%rax,4), %r8
	movq	(%rsp), %rax
	leaq	(%r15,%rax,4), %rax
	movq	(%rsp), %rcx
	leaq	(%r12,%rcx,4), %r10
	movq	(%rsp), %rcx
	movq	96(%rsp), %rsi
	leaq	(%rsi,%rcx,4), %r11
	movq	(%rsp), %rbx
	movq	104(%rsp), %rcx
	leaq	(%rcx,%rbx,4), %r12
	movq	112(%rsp), %rcx
	leaq	(%rcx,%rbx,4), %r14
	movq	48(%rsp), %rcx
	leaq	(%rcx,%rbx,4), %r15
	vmovss	.LCPI20_1(%rip), %xmm0
	movq	72(%rsp), %rdi
	movq	%rbp, %rsi
	movq	%rbp, %rcx
	movq	%rbp, %r9
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r15
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r14
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r12
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r11
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r10
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	callq	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main4safe6kernel10fill_dense
	addq	$96, %rsp
	.cfi_adjust_cfa_offset -96
	movq	32(%rsp), %rcx
	testq	%rcx, %rcx
	movq	%rbp, %r12
	je	.LBB20_19
	leaq	-1(%r12), %r15
	cmpq	%r15, %rbx
	movq	%r15, %rax
	cmovbq	%rbx, %rax
	testq	%r12, %r12
	je	.LBB20_114
	movq	%rax, %rsi
	shlq	$5, %rsi
	addq	%r13, %rsi
	movq	%rbx, %rdx
	subq	%rax, %rdx
	movq	%r13, %r14
	movq	40(%rsp), %rbp
	movq	8(%rsp), %rcx
	leaq	(%rcx,%r13), %rdi
	jmp	.LBB20_22
.LBB20_19:
	testq	%rbx, %rbx
	je	.LBB20_135
	testq	%r12, %r12
	je	.LBB20_137
	leaq	-1(%rbx), %rdx
	movq	8(%rsp), %rax
	leaq	(%rax,%r13), %rdi
	vmovups	(%rdi), %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI20_2(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vaddps	(%r13), %ymm0, %ymm0
	vmovups	%ymm0, (%r13)
	leaq	-1(%r12), %r15
	cmpq	%r15, %rdx
	movq	%r15, %rax
	cmovbq	%rdx, %rax
	leaq	32(%r13), %r14
	movq	%rax, %rcx
	shlq	$5, %rcx
	leaq	(%rcx,%r13), %rsi
	addq	$32, %rsi
	subq	%rax, %rdx
	movq	40(%rsp), %rbp
.LBB20_22:
	cmpq	%r12, %rax
	cmovaeq	%r12, %rax
	leaq	32(%rdi), %rcx
	cmpq	%r15, %rax
	cmovaeq	%r15, %rax
	testq	%rax, %rax
	movq	%r15, 16(%rsp)
	je	.LBB20_32
	movq	%rbp, %r15
	leaq	(,%rax,8), %r8
	movq	%rax, %r9
	shlq	$5, %r9
	addq	%r14, %r9
	leaq	(%rax,%rbx), %r10
	shlq	$5, %r10
	leaq	(%r10,%r13), %r11
	addq	%r13, %r10
	addq	$32, %r10
	cmpq	%r11, %r14
	setb	%bl
	cmpq	%r9, %rdi
	setb	%bpl
	cmpq	%r10, %r14
	setb	%r10b
	cmpq	%r9, %rcx
	setb	%r11b
	xorl	%r9d, %r9d
	testb	%bpl, %bl
	jne	.LBB20_29
	andb	%r11b, %r10b
	movq	(%rsp), %rbx
	movq	8(%rsp), %r10
	leaq	(%r13,%r10), %rdi
	movq	%r15, %rbp
	movq	16(%rsp), %r15
	jne	.LBB20_30
	cmpq	$1, %rax
	jne	.LBB20_44
	xorl	%r9d, %r9d
.LBB20_27:
	vbroadcastss	.LCPI20_2(%rip), %ymm0
	.p2align	4
.LBB20_28:
	vmovups	-32(%rcx,%r9,4), %ymm1
	vaddps	(%rcx,%r9,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r14,%r9,4), %ymm1, %ymm1
	vmovups	%ymm1, (%r14,%r9,4)
	addq	$8, %r9
	cmpq	%r9, %r8
	jne	.LBB20_28
	jmp	.LBB20_32
.LBB20_29:
	movq	(%rsp), %rbx
	movq	8(%rsp), %r10
	leaq	(%r10,%r13), %rdi
	movq	%r15, %rbp
	movq	16(%rsp), %r15
.LBB20_30:
	leaq	(%r10,%r13), %rax
	addq	$44, %rax
	vmovss	.LCPI20_3(%rip), %xmm0
	.p2align	4
.LBB20_31:
	vmovss	-44(%rax,%r9,4), %xmm1
	vaddss	-12(%rax,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vmovss	(%r14,%r9,4), %xmm2
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	4(%r14,%r9,4), %xmm2
	vmovss	%xmm1, (%r14,%r9,4)
	vmovss	-40(%rax,%r9,4), %xmm1
	vaddss	-8(%rax,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 4(%r14,%r9,4)
	vmovss	-36(%rax,%r9,4), %xmm1
	vmovss	8(%r14,%r9,4), %xmm2
	vaddss	-4(%rax,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 8(%r14,%r9,4)
	vmovss	-32(%rax,%r9,4), %xmm1
	vmovss	12(%r14,%r9,4), %xmm2
	vaddss	(%rax,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 12(%r14,%r9,4)
	addq	$4, %r9
	cmpq	%r9, %r8
	jne	.LBB20_31
.LBB20_32:
	cmpq	$1, %rdx
	jne	.LBB20_34
	movq	%r15, %rax
	shlq	$5, %rax
	vbroadcastss	.LCPI20_4(%rip), %ymm0
	vmulps	(%rdi,%rax), %ymm0, %ymm0
	vaddps	(%rsi), %ymm0, %ymm0
	vmovups	%ymm0, (%rsi)
.LBB20_34:
	movq	32(%rsp), %r14
	cmpq	$1, %r14
	jne	.LBB20_37
	testq	%rbx, %rbx
	je	.LBB20_137
	vmovups	(%r13), %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI20_5(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vaddps	(%rdi), %ymm0, %ymm0
	vmovups	%ymm0, (%rdi)
	leaq	-1(%rbx), %rax
	cmpq	%rax, %r15
	movq	%rax, %r11
	cmovbq	%r15, %r11
	movq	%r11, %r8
	shlq	$5, %r8
	addq	%rcx, %r8
	movq	%r15, %rdx
	subq	%r11, %rdx
	movq	%rcx, %r9
	jmp	.LBB20_39
.LBB20_37:
	leaq	-1(%rbx), %rax
	cmpq	%rax, %r12
	movq	%rax, %r11
	cmovbq	%r12, %r11
	testq	%rbx, %rbx
	je	.LBB20_114
	movq	%r11, %r8
	shlq	$5, %r8
	addq	%rdi, %r8
	movq	%r12, %rdx
	subq	%r11, %rdx
	movq	%rdi, %r9
.LBB20_39:
	cmpq	%rbx, %r11
	cmovaeq	%rbx, %r11
	leaq	32(%r13), %rsi
	cmpq	%rax, %r11
	movq	%rax, 24(%rsp)
	cmovaeq	%rax, %r11
	testq	%r11, %r11
	je	.LBB20_57
	movq	%rdx, 48(%rsp)
	movq	%r12, %rdx
	leaq	(,%r11,8), %r10
	movq	%r11, %rax
	shlq	$5, %rax
	leaq	(%r9,%rax), %r14
	leaq	(%rax,%r13), %rbx
	addq	%r13, %rax
	addq	$32, %rax
	cmpq	%rbx, %r9
	setb	%r15b
	cmpq	%r14, %r13
	setb	%r12b
	cmpq	%rax, %r9
	setb	%bl
	cmpq	%r14, %rsi
	setb	%bpl
	xorl	%eax, %eax
	testb	%r12b, %r15b
	jne	.LBB20_48
	andb	%bpl, %bl
	movq	40(%rsp), %rbp
	movq	(%rsp), %rbx
	movq	32(%rsp), %r14
	movq	%rdx, %r12
	movq	8(%rsp), %rdx
	leaq	(%r13,%rdx), %rdi
	movq	16(%rsp), %r15
	movq	48(%rsp), %rdx
	jne	.LBB20_49
	cmpq	$4, %r11
	jae	.LBB20_51
	xorl	%eax, %eax
	jmp	.LBB20_55
.LBB20_44:
	movabsq	$2305843009213693936, %r9
	andq	%r8, %r9
	addq	%r13, %r10
	addq	$64, %r10
	xorl	%r11d, %r11d
	vbroadcastss	.LCPI20_3(%rip), %ymm0
	.p2align	4
.LBB20_45:
	vmovups	-32(%r10,%r11,4), %ymm1
	vmovups	(%r14,%r11,4), %ymm2
	vmovups	32(%r14,%r11,4), %ymm3
	vaddps	-64(%r10,%r11,4), %ymm1, %ymm4
	vaddps	(%r10,%r11,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm4, %ymm4
	vsubps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm0, %ymm1, %ymm1
	vsubps	%ymm1, %ymm3, %ymm1
	vmovups	%ymm2, (%r14,%r11,4)
	vmovups	%ymm1, 32(%r14,%r11,4)
	addq	$16, %r11
	cmpq	%r11, %r9
	jne	.LBB20_45
	cmpq	%r9, %r8
	movq	8(%rsp), %r10
	je	.LBB20_32
	testb	$1, %al
	jne	.LBB20_27
	jmp	.LBB20_30
.LBB20_48:
	movq	40(%rsp), %rbp
	movq	(%rsp), %rbx
	movq	32(%rsp), %r14
	movq	%rdx, %r12
	movq	8(%rsp), %rdx
	leaq	(%rdx,%r13), %rdi
	movq	16(%rsp), %r15
	movq	48(%rsp), %rdx
.LBB20_49:
	vmovss	.LCPI20_6(%rip), %xmm0
	.p2align	4
.LBB20_50:
	vmovss	(%r13,%rax,4), %xmm1
	vaddss	32(%r13,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vmovss	(%r9,%rax,4), %xmm2
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	4(%r9,%rax,4), %xmm2
	vmovss	%xmm1, (%r9,%rax,4)
	vmovss	4(%r13,%rax,4), %xmm1
	vaddss	36(%r13,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 4(%r9,%rax,4)
	vmovss	8(%r13,%rax,4), %xmm1
	vmovss	8(%r9,%rax,4), %xmm2
	vaddss	40(%r13,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 8(%r9,%rax,4)
	vmovss	12(%r13,%rax,4), %xmm1
	vmovss	12(%r9,%rax,4), %xmm2
	vaddss	44(%r13,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vsubss	%xmm1, %xmm2, %xmm1
	vmovss	%xmm1, 12(%r9,%rax,4)
	addq	$4, %rax
	cmpq	%rax, %r10
	jne	.LBB20_50
	jmp	.LBB20_57
.LBB20_51:
	movq	%rbx, %rdx
	movabsq	$9223372036854775776, %rax
	andq	%r10, %rax
	xorl	%ebx, %ebx
	vbroadcastss	.LCPI20_6(%rip), %ymm0
	.p2align	4
.LBB20_52:
	vmovups	32(%r13,%rbx,4), %ymm1
	vmovups	64(%r13,%rbx,4), %ymm2
	vmovups	96(%r13,%rbx,4), %ymm3
	vmovups	(%r9,%rbx,4), %ymm4
	vmovups	32(%r9,%rbx,4), %ymm5
	vmovups	64(%r9,%rbx,4), %ymm6
	vmovups	96(%r9,%rbx,4), %ymm7
	vaddps	(%r13,%rbx,4), %ymm1, %ymm8
	vaddps	%ymm2, %ymm1, %ymm1
	vaddps	%ymm3, %ymm2, %ymm2
	vaddps	128(%r13,%rbx,4), %ymm3, %ymm3
	vmulps	%ymm0, %ymm8, %ymm8
	vsubps	%ymm8, %ymm4, %ymm4
	vmulps	%ymm0, %ymm1, %ymm1
	vsubps	%ymm1, %ymm5, %ymm1
	vmulps	%ymm0, %ymm2, %ymm2
	vsubps	%ymm2, %ymm6, %ymm2
	vmulps	%ymm0, %ymm3, %ymm3
	vsubps	%ymm3, %ymm7, %ymm3
	vmovups	%ymm4, (%r9,%rbx,4)
	vmovups	%ymm1, 32(%r9,%rbx,4)
	vmovups	%ymm2, 64(%r9,%rbx,4)
	vmovups	%ymm3, 96(%r9,%rbx,4)
	addq	$32, %rbx
	cmpq	%rbx, %rax
	jne	.LBB20_52
	cmpq	%rax, %r10
	movq	%rdx, %rbx
	movq	48(%rsp), %rdx
	je	.LBB20_57
	testb	$3, %r11b
	je	.LBB20_49
.LBB20_55:
	vbroadcastss	.LCPI20_5(%rip), %ymm0
	.p2align	4
.LBB20_56:
	vmovups	-32(%rsi,%rax,4), %ymm1
	vaddps	(%rsi,%rax,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r9,%rax,4), %ymm1, %ymm1
	vmovups	%ymm1, (%r9,%rax,4)
	addq	$8, %rax
	cmpq	%rax, %r10
	jne	.LBB20_56
.LBB20_57:
	cmpq	$1, %rdx
	jne	.LBB20_59
	movq	24(%rsp), %rax
	shlq	$5, %rax
	vbroadcastss	.LCPI20_7(%rip), %ymm0
	vmulps	(%r13,%rax), %ymm0, %ymm0
	vaddps	(%r8), %ymm0, %ymm0
	vmovups	%ymm0, (%r8)
.LBB20_59:
	movq	%rbx, %rdx
	movq	%r13, %r8
	testq	%r14, %r14
	jne	.LBB20_61
	vmovups	(%rdi), %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI20_8(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vaddps	(%r13), %ymm0, %ymm0
	vmovups	%ymm0, (%r13)
	movq	24(%rsp), %rdx
	movq	%rsi, %r8
.LBB20_61:
	cmpq	%r15, %rdx
	movq	%r15, %r11
	cmovbq	%rdx, %r11
	movq	%r11, %r9
	shlq	$5, %r9
	addq	%r8, %r9
	subq	%r11, %rdx
	testq	%r11, %r11
	je	.LBB20_71
	leaq	(,%r11,8), %r10
	leaq	(%r11,%rbx), %rax
	shlq	$5, %rax
	leaq	(%rax,%r13), %rbx
	addq	%r13, %rax
	addq	$32, %rax
	cmpq	%rbx, %r8
	setb	%r14b
	cmpq	%r9, %rdi
	setb	%r15b
	cmpq	%rax, %r8
	setb	%bl
	cmpq	%r9, %rcx
	setb	%bpl
	xorl	%eax, %eax
	testb	%r15b, %r14b
	jne	.LBB20_68
	andb	%bpl, %bl
	movq	32(%rsp), %r14
	movq	8(%rsp), %r15
	leaq	(%r13,%r15), %rdi
	movq	40(%rsp), %rbp
	jne	.LBB20_69
	cmpq	$1, %r11
	jne	.LBB20_83
	xorl	%eax, %eax
.LBB20_66:
	vbroadcastss	.LCPI20_8(%rip), %ymm0
	movq	16(%rsp), %r15
	.p2align	4
.LBB20_67:
	vmovups	-32(%rcx,%rax,4), %ymm1
	vaddps	(%rcx,%rax,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r8,%rax,4), %ymm1, %ymm1
	vmovups	%ymm1, (%r8,%rax,4)
	addq	$8, %rax
	cmpq	%rax, %r10
	jne	.LBB20_67
	jmp	.LBB20_71
.LBB20_68:
	movq	32(%rsp), %r14
	movq	8(%rsp), %r15
	leaq	(%r15,%r13), %rdi
	movq	40(%rsp), %rbp
.LBB20_69:
	leaq	(%r15,%r13), %r11
	addq	$44, %r11
	vmovss	.LCPI20_8(%rip), %xmm0
	movq	16(%rsp), %r15
	.p2align	4
.LBB20_70:
	vmovss	-44(%r11,%rax,4), %xmm1
	vaddss	-12(%r11,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	(%r8,%rax,4), %xmm1, %xmm1
	vmovss	%xmm1, (%r8,%rax,4)
	vmovss	-40(%r11,%rax,4), %xmm1
	vaddss	-8(%r11,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	4(%r8,%rax,4), %xmm1, %xmm1
	vmovss	%xmm1, 4(%r8,%rax,4)
	vmovss	-36(%r11,%rax,4), %xmm1
	vaddss	-4(%r11,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	8(%r8,%rax,4), %xmm1, %xmm1
	vmovss	%xmm1, 8(%r8,%rax,4)
	vmovss	-32(%r11,%rax,4), %xmm1
	vaddss	(%r11,%rax,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	12(%r8,%rax,4), %xmm1, %xmm1
	vmovss	%xmm1, 12(%r8,%rax,4)
	addq	$4, %rax
	cmpq	%rax, %r10
	jne	.LBB20_70
.LBB20_71:
	cmpq	$1, %rdx
	jne	.LBB20_73
	movq	%r15, %rax
	shlq	$5, %rax
	vbroadcastss	.LCPI20_9(%rip), %ymm0
	vmulps	(%rdi,%rax), %ymm0, %ymm0
	vaddps	(%r9), %ymm0, %ymm0
	vmovups	%ymm0, (%r9)
.LBB20_73:
	movq	%r12, %r15
	movq	%rdi, %r8
	cmpq	$1, %r14
	movq	56(%rsp), %rdx
	jne	.LBB20_75
	vmovups	(%r13), %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI20_10(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vaddps	(%rdi), %ymm0, %ymm0
	vmovups	%ymm0, (%rdi)
	movq	16(%rsp), %r15
	movq	%rcx, %r8
.LBB20_75:
	movq	24(%rsp), %rax
	cmpq	%rax, %r15
	movq	%rax, %r10
	cmovbq	%r15, %r10
	movq	%r10, %r9
	shlq	$5, %r9
	leaq	(%r8,%r9), %rcx
	movq	%rcx, 8(%rsp)
	subq	%r10, %r15
	testq	%r10, %r10
	je	.LBB20_93
	movq	%r14, %rdi
	movq	%rbp, %rax
	leaq	(,%r10,8), %rcx
	leaq	(%r9,%r13), %r11
	addq	%r13, %r9
	addq	$32, %r9
	cmpq	%r11, %r8
	setb	%bpl
	movq	8(%rsp), %rdx
	cmpq	%rdx, %r13
	setb	%r14b
	cmpq	%r9, %r8
	setb	%r11b
	cmpq	%rdx, %rsi
	setb	%bl
	xorl	%r9d, %r9d
	testb	%r14b, %bpl
	jne	.LBB20_80
	andb	%bl, %r11b
	movq	56(%rsp), %rdx
	movq	72(%rsp), %r11
	movq	%rax, %rbp
	movq	%rdi, %r14
	movq	%r11, %rdi
	movq	24(%rsp), %rax
	jne	.LBB20_81
	cmpq	$4, %r10
	jae	.LBB20_87
	xorl	%r9d, %r9d
	jmp	.LBB20_91
.LBB20_80:
	movq	56(%rsp), %rdx
	movq	72(%rsp), %rsi
	movq	%rax, %rbp
	movq	%rdi, %r14
	movq	%rsi, %rdi
	movq	24(%rsp), %rax
.LBB20_81:
	vmovss	.LCPI20_10(%rip), %xmm0
	.p2align	4
.LBB20_82:
	vmovss	(%r13,%r9,4), %xmm1
	vaddss	32(%r13,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	(%r8,%r9,4), %xmm1, %xmm1
	vmovss	%xmm1, (%r8,%r9,4)
	vmovss	4(%r13,%r9,4), %xmm1
	vaddss	36(%r13,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	4(%r8,%r9,4), %xmm1, %xmm1
	vmovss	%xmm1, 4(%r8,%r9,4)
	vmovss	8(%r13,%r9,4), %xmm1
	vaddss	40(%r13,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	8(%r8,%r9,4), %xmm1, %xmm1
	vmovss	%xmm1, 8(%r8,%r9,4)
	vmovss	12(%r13,%r9,4), %xmm1
	vaddss	44(%r13,%r9,4), %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	12(%r8,%r9,4), %xmm1, %xmm1
	vmovss	%xmm1, 12(%r8,%r9,4)
	addq	$4, %r9
	cmpq	%r9, %rcx
	jne	.LBB20_82
	jmp	.LBB20_93
.LBB20_83:
	movabsq	$9223372036854775776, %rax
	orq	$16, %rax
	andq	%r10, %rax
	movq	8(%rsp), %rbx
	addq	%r13, %rbx
	addq	$64, %rbx
	xorl	%r14d, %r14d
	vbroadcastss	.LCPI20_8(%rip), %ymm0
	.p2align	4
.LBB20_84:
	vmovups	-32(%rbx,%r14,4), %ymm1
	vaddps	-64(%rbx,%r14,4), %ymm1, %ymm2
	vaddps	(%rbx,%r14,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm2, %ymm2
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r8,%r14,4), %ymm2, %ymm2
	vaddps	32(%r8,%r14,4), %ymm1, %ymm1
	vmovups	%ymm2, (%r8,%r14,4)
	vmovups	%ymm1, 32(%r8,%r14,4)
	addq	$16, %r14
	cmpq	%r14, %rax
	jne	.LBB20_84
	cmpq	%rax, %r10
	movq	32(%rsp), %r14
	movq	16(%rsp), %r15
	je	.LBB20_71
	testb	$1, %r11b
	movq	8(%rsp), %r15
	jne	.LBB20_66
	jmp	.LBB20_69
.LBB20_87:
	movabsq	$9223372036854775776, %r9
	andq	%rcx, %r9
	xorl	%r11d, %r11d
	vbroadcastss	.LCPI20_10(%rip), %ymm0
	.p2align	4
.LBB20_88:
	vmovups	32(%r13,%r11,4), %ymm1
	vmovups	64(%r13,%r11,4), %ymm2
	vmovups	96(%r13,%r11,4), %ymm3
	vaddps	(%r13,%r11,4), %ymm1, %ymm4
	vaddps	%ymm2, %ymm1, %ymm1
	vaddps	%ymm3, %ymm2, %ymm2
	vaddps	128(%r13,%r11,4), %ymm3, %ymm3
	vmulps	%ymm0, %ymm4, %ymm4
	vmulps	%ymm0, %ymm1, %ymm1
	vmulps	%ymm0, %ymm2, %ymm2
	vmulps	%ymm0, %ymm3, %ymm3
	vaddps	(%r8,%r11,4), %ymm4, %ymm4
	vaddps	32(%r8,%r11,4), %ymm1, %ymm1
	vaddps	64(%r8,%r11,4), %ymm2, %ymm2
	vaddps	96(%r8,%r11,4), %ymm3, %ymm3
	vmovups	%ymm4, (%r8,%r11,4)
	vmovups	%ymm1, 32(%r8,%r11,4)
	vmovups	%ymm2, 64(%r8,%r11,4)
	vmovups	%ymm3, 96(%r8,%r11,4)
	addq	$32, %r11
	cmpq	%r11, %r9
	jne	.LBB20_88
	cmpq	%r9, %rcx
	movq	56(%rsp), %rdx
	je	.LBB20_93
	testb	$3, %r10b
	je	.LBB20_81
.LBB20_91:
	vbroadcastss	.LCPI20_10(%rip), %ymm0
	.p2align	4
.LBB20_92:
	vmovups	-32(%rsi,%r9,4), %ymm1
	vaddps	(%rsi,%r9,4), %ymm1, %ymm1
	vmulps	%ymm0, %ymm1, %ymm1
	vaddps	(%r8,%r9,4), %ymm1, %ymm1
	vmovups	%ymm1, (%r8,%r9,4)
	addq	$8, %r9
	cmpq	%r9, %rcx
	jne	.LBB20_92
.LBB20_93:
	cmpq	$1, %r15
	jne	.LBB20_95
	shlq	$5, %rax
	vbroadcastss	.LCPI20_11(%rip), %ymm0
	vmulps	(%r13,%rax), %ymm0, %ymm0
	movq	8(%rsp), %rax
	vaddps	(%rax), %ymm0, %ymm0
	vmovups	%ymm0, (%rax)
.LBB20_95:
	testq	%rdx, %rdx
	movq	64(%rsp), %r10
	je	.LBB20_134
	cmpq	%rdx, %rbp
	ja	.LBB20_121
	cmpq	%rdx, %r10
	jbe	.LBB20_136
	subq	%rdx, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %rax
	cmovbq	%r10, %rax
	cmpq	%rax, %rbp
	ja	.LBB20_115
	cmpq	%rdx, %r10
	jbe	.LBB20_136
	subq	%rax, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %rcx
	cmovbq	%r10, %rcx
	cmpq	%rcx, %rbp
	ja	.LBB20_116
	cmpq	%rdx, %r10
	jbe	.LBB20_136
	subq	%rcx, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %r9
	cmovbq	%r10, %r9
	cmpq	%r9, %rbp
	ja	.LBB20_117
	cmpq	%rdx, %r10
	jbe	.LBB20_136
	subq	%r9, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %rbx
	cmovbq	%r10, %rbx
	cmpq	%rbx, %rbp
	ja	.LBB20_118
	cmpq	%rdx, %r10
	jbe	.LBB20_136
	movq	%r14, %rsi
	subq	%rbx, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %r14
	cmovbq	%r10, %r14
	cmpq	%r14, %rbp
	ja	.LBB20_119
	cmpq	%rdx, %r10
	jbe	.LBB20_136
	subq	%r14, %r10
	cmpq	%rdx, %r10
	movq	%rdx, %r8
	cmovbq	%r10, %r8
	cmpq	%r8, %rbp
	ja	.LBB20_120
	cmpq	%rdx, %r10
	jbe	.LBB20_136
	subq	%r8, %r10
	cmpq	%rdx, %r10
	cmovbq	%r10, %rdx
	cmpq	%rdx, %rbp
	ja	.LBB20_121
	testq	%rsi, %rsi
	movq	%r12, %rsi
	movq	80(%rsp), %r12
	leaq	(%r12,%rax,4), %rax
	leaq	(%rax,%rcx,4), %r10
	leaq	(%r10,%r9,4), %r11
	leaq	(%r11,%rbx,4), %rbx
	leaq	(%rbx,%r14,4), %r14
	leaq	(%r14,%r8,4), %r15
	movq	(%rsp), %r8
	movq	%r8, %rcx
	cmoveq	%rsi, %rcx
	movq	%r13, %rdx
	cmoveq	%rdi, %rdx
	cmoveq	%r8, %rsi
	cmoveq	%r13, %rdi
	movq	88(%rsp), %r8
	movq	%rbp, %r9
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r15
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r14
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%rbx
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r11
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r10
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	%rbp
	.cfi_adjust_cfa_offset 8
	pushq	%r12
	.cfi_adjust_cfa_offset 8
	vzeroupper
	callq	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel13scatter_dense
	addq	$232, %rsp
	.cfi_adjust_cfa_offset -232
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
.LBB20_112:
	.cfi_def_cfa_offset 176
	xorl	%edx, %edx
	movq	%r9, %rsi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.18(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_123:
	movq	%rbx, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.18(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_125:
	movq	%rax, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.18(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_127:
	movq	%r11, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.18(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_129:
	movq	%r15, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.18(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_113:
	movq	%r10, %rdx
	movq	64(%rsp), %r10
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.18(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_114:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.54(%rip), %rcx
	movl	$1, %edi
	xorl	%esi, %esi
	xorl	%edx, %edx
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_115:
	movq	%rax, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.20(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_116:
	movq	%rcx, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.20(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_117:
	movq	%r9, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.20(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_118:
	movq	%rbx, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.20(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_119:
	movq	%r14, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.20(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_120:
	movq	%r8, %rdx
.LBB20_121:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.20(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbp, %rsi
	vzeroupper
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_130:
	movq	%r8, %rdx
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.18(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_131:
	movq	%r8, %rdx
.LBB20_132:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.18(%rip), %rcx
	movq	%rdx, %rdi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs8uuN1nrGffB_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB20_133:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.28(%rip), %rdi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.63(%rip), %rdx
	movl	$19, %esi
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.LBB20_134:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.0(%rip), %rdi
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.64(%rip), %rdx
	movl	$55, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking9panic_fmt@GOTPCREL(%rip)
.LBB20_135:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.52(%rip), %rdi
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB20_136:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.19(%rip), %rdi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core6option13unwrap_failed@GOTPCREL(%rip)
.LBB20_137:
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.53(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	vzeroupper
	callq	*_RNvNtCs8uuN1nrGffB_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Lfunc_end20:
	.size	_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel15inverse_97_rows, .Lfunc_end20-_RNvNtNtCs5dlvf2fXWpN_16outlined_io_main7barrier6kernel15inverse_97_rows
	.cfi_endproc

	.section	.text._RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end21, nop
	.type	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main,@function
_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main:
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
	je	.LBB21_3
	shlq	$2, %r13
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.1(%rip), %rbx
	leaq	8(%rsp), %r14
	movq	%rsp, %r15
	movq	_RNvMs6_NtNtCs8uuN1nrGffB_4core3fmt8buildersNtB5_9DebugList5entry@GOTPCREL(%rip), %rbp
	.p2align	4
.LBB21_2:
	movq	%r12, (%rsp)
	addq	$4, %r12
	movq	%r14, %rdi
	movq	%r15, %rsi
	movq	%rbx, %rdx
	callq	*%rbp
	addq	$-4, %r13
	jne	.LBB21_2
.LBB21_3:
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
.Lfunc_end21:
	.size	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end21-_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.text._RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main,"ax",@progbits
	.prefalign	4, .Lfunc_end22, nop
	.type	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main,@function
_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main:
	.cfi_startproc
	movq	(%rdi), %rdi
	movl	16(%rsi), %eax
	testl	$33554432, %eax
	jne	.LBB22_3
	testl	$67108864, %eax
	jne	.LBB22_2
	jmpq	*_RNvXs9_NtNtNtCs8uuN1nrGffB_4core3fmt3num3implNtB9_7Display3fmt@GOTPCREL(%rip)
.LBB22_3:
	jmpq	*_RNvXsv_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_8LowerHex3fmt@GOTPCREL(%rip)
.LBB22_2:
	jmpq	*_RNvXsx_NtNtCs8uuN1nrGffB_4core3fmt3numlNtB7_8UpperHex3fmt@GOTPCREL(%rip)
.Lfunc_end22:
	.size	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main, .Lfunc_end22-_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.prefalign	4, .Lfunc_end23, nop
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rcx
	movslq	%edi, %rdx
	leaq	_RNvCs5dlvf2fXWpN_16outlined_io_main4main(%rip), %rax
	movq	%rax, (%rsp)
	leaq	.Lanon.a4e02af6b5901a84584d3e04d170619c.3(%rip), %rsi
	movq	%rsp, %rdi
	xorl	%r8d, %r8d
	callq	*_RNvNtCsh0g7Hz38UNf_3std2rt19lang_start_internal@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end23:
	.size	main, .Lfunc_end23-main
	.cfi_endproc

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.0,@object
	.section	.rodata..Lanon.a4e02af6b5901a84584d3e04d170619c.0,"a",@progbits
.Lanon.a4e02af6b5901a84584d3e04d170619c.0:
	.ascii	"chunk size must be non-zero"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.0, 27

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.1,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.1,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRlNtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.1, 32

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.2,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.2,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.2:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_RNvXs1g_NtCs8uuN1nrGffB_4core3fmtRINtNtCs3BN1jqgSbDf_5alloc3vec3VeclENtB6_5Debug3fmtCs5dlvf2fXWpN_16outlined_io_main
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.2, 32

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.3,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.3,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.3:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_RNSNvYNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0INtNtNtCs8uuN1nrGffB_4core3ops8function6FnOnceuE9call_once6vtableCs5dlvf2fXWpN_16outlined_io_main
	.quad	_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0Cs5dlvf2fXWpN_16outlined_io_main
	.quad	_RNCINvNtCsh0g7Hz38UNf_3std2rt10lang_startuE0Cs5dlvf2fXWpN_16outlined_io_main
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.3, 48

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.4,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.a4e02af6b5901a84584d3e04d170619c.4:
	.asciz	"/rustc/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/slice/sort/stable/quicksort.rs"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.4, 96

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.5,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.5,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.5:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.4
	.asciz	"_\000\000\000\000\000\000\000M\000\000\000\037\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.5, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.6,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.6,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.6:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.4
	.asciz	"_\000\000\000\000\000\000\000G\000\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.6, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.7,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.a4e02af6b5901a84584d3e04d170619c.7:
	.asciz	"outlined-io-main.rs"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.7, 20

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.8,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.8,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.8:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.7
	.asciz	"\023\000\000\000\000\000\000\000\031\000\000\0007\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.8, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.9,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.a4e02af6b5901a84584d3e04d170619c.9:
	.asciz	"/home/dev/safe-crate-audit/base/neo-j2k/src/simd/dwt.rs"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.9, 56

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.10,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.10,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.10:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000\231\000\000\000,\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.10, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.11,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.11,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.11:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000\237\000\000\0003\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.11, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.12,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.12,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.12:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000\251\000\000\0001\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.12, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.13,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.a4e02af6b5901a84584d3e04d170619c.13:
	.asciz	"/home/dev/safe-crate-audit/candidate/neo-j2k/src/simd/dwt.rs"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.13, 61

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.14,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.14,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.14:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000a\001\000\000;\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.14, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.15,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.15,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.15:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000\203\001\000\000\037\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.15, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.16,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.16,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.16:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000\204\001\000\000\021\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.16, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.17,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.a4e02af6b5901a84584d3e04d170619c.17:
	.asciz	"/home/dev/safe-crate-audit/transpose-probe/outlined-io.rs"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.17, 58

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.18,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.18,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.18:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000a\001\000\000;\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.18, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.19,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.19,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.19:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000\203\001\000\000\037\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.19, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.20,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.20,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.20:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000\204\001\000\000\021\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.20, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.21,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.21,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.21:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.7
	.asciz	"\023\000\000\000\000\000\000\000\026\000\000\000\251\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.21, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.22,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.22,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.22:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.7
	.asciz	"\023\000\000\000\000\000\000\000\026\000\000\000\313\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.22, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.23,@object
	.section	.rodata..Lanon.a4e02af6b5901a84584d3e04d170619c.23,"a",@progbits
.Lanon.a4e02af6b5901a84584d3e04d170619c.23:
	.asciz	"\005width\300\007|stride\300\007|parity\300\n|reference\305 \000\000p\004\000\005|safe\305 \000\000p\004\000\b|barrier\305 \000\000p\004\000\b|control\305 \000\000p\004\000\006|same\n"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.23, 96

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.24,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.24,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.24:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.7
	.asciz	"\023\000\000\000\000\000\000\000\030\000\000\000.\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.24, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.25,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.25,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.25:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.7
	.asciz	"\023\000\000\000\000\000\000\000\024\000\000\000V\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.25, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.26,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.26,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.26:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.7
	.asciz	"\023\000\000\000\000\000\000\000\020\000\000\000{\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.26, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.27,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.27,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.27:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.7
	.asciz	"\023\000\000\000\000\000\000\000\020\000\000\000V\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.27, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.28,@object
	.section	.rodata..Lanon.a4e02af6b5901a84584d3e04d170619c.28,"a",@progbits
.Lanon.a4e02af6b5901a84584d3e04d170619c.28:
	.ascii	"mid > len"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.28, 9

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.29,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.29,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.29:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000\237\000\000\000\t\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.29, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.30,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.30,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.30:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000\233\000\000\000\030\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.30, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.31,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.31,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.31:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000\270\000\000\000\035\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.31, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.32,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.32,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.32:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000\271\000\000\000$\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.32, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.33,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.33,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.33:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000\262\000\000\000$\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.33, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.34,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.34,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.34:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000\263\000\000\000!\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.34, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.35,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.35,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.35:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\0003\001\000\000L\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.35, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.36,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.36,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.36:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000-\001\000\000-\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.36, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.37,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.37,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.37:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000-\001\000\000S\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.37, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.38,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.38,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.38:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.9
	.asciz	"7\000\000\000\000\000\000\000/\001\000\0000\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.38, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.39,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.39,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.39:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000\016\001\000\0005\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.39, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.40,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.40,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.40:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000\017\001\000\000!\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.40, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.41,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.41,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.41:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000\031\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.41, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.42,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.42,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.42:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000Q\001\000\000\024\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.42, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.43,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.43,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.43:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000S\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.43, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.44,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.44,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.44:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000T\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.44, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.45,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.45,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.45:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000U\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.45, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.46,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.46,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.46:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000V\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.46, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.47,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.47,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.47:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000W\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.47, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.48,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.48,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.48:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000X\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.48, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.49,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.49,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.49:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000Y\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.49, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.50,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.50,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.50:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000b\001\000\000\037\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.50, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.51,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.51,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.51:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.13
	.asciz	"<\000\000\000\000\000\000\000\201\001\000\000\031\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.51, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.52,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.52,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.52:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000\016\001\000\0005\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.52, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.53,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.53,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.53:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000\017\001\000\000!\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.53, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.54,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.54,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.54:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000\031\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.54, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.55,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.55,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.55:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000Q\001\000\000\024\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.55, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.56,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.56,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.56:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000S\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.56, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.57,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.57,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.57:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000T\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.57, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.58,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.58,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.58:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000U\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.58, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.59,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.59,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.59:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000V\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.59, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.60,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.60,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.60:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000W\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.60, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.61,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.61,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.61:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000X\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.61, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.62,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.62,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.62:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000Y\001\000\000\027\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.62, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.63,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.63,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.63:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000b\001\000\000\037\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.63, 24

	.type	.Lanon.a4e02af6b5901a84584d3e04d170619c.64,@object
	.section	.data.rel.ro..Lanon.a4e02af6b5901a84584d3e04d170619c.64,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a4e02af6b5901a84584d3e04d170619c.64:
	.quad	.Lanon.a4e02af6b5901a84584d3e04d170619c.17
	.asciz	"9\000\000\000\000\000\000\000\201\001\000\000\031\000\000"
	.size	.Lanon.a4e02af6b5901a84584d3e04d170619c.64, 24

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
