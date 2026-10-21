	.syntax	unified
	.eabi_attribute	67, "2.09"
	.eabi_attribute	6, 13
	.eabi_attribute	7, 77
	.eabi_attribute	8, 0
	.eabi_attribute	9, 2
	.fpu	fpv4-sp-d16
	.eabi_attribute	27, 1
	.eabi_attribute	36, 1
	.eabi_attribute	34, 1
	.eabi_attribute	17, 1
	.eabi_attribute	20, 1
	.eabi_attribute	21, 1
	.eabi_attribute	23, 3
	.eabi_attribute	24, 1
	.eabi_attribute	25, 1
	.eabi_attribute	28, 1
	.eabi_attribute	38, 1
	.eabi_attribute	18, 4
	.eabi_attribute	14, 0
	.file	"arm_bitwise.460195c9a5e0578f-cgu.0"
	.section	.text.set_bit,"ax",%progbits
	.globl	set_bit
	.p2align	1
	.type	set_bit,%function
	.code	16
	.thumb_func
set_bit:
	.fnstart
	.save	{r7, lr}
	push	{r7, lr}
	.setfp	r7, sp
	mov	r7, sp
	and	r1, r1, #31
	movs	r2, #1
	lsl.w	r1, r2, r1
	orrs	r0, r1
	pop	{r7, pc}
.Lfunc_end0:
	.size	set_bit, .Lfunc_end0-set_bit
	.cantunwind
	.fnend

	.ident	"rustc version 1.99.1 (58cf8f9c3 2026-10-13)"
	.section	".note.GNU-stack","",%progbits
	.eabi_attribute	30, 2
