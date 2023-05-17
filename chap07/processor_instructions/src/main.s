	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.p2align	2
__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h074fc9456b17fe2eE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4core3ops8function6FnOnce9call_once17h444efb08380f190eE
	; InlineAsm Start
	; InlineAsm End
	b	LBB0_1
LBB0_1:
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__ZN3std2rt10lang_start17hed80f55b4a401277E
	.globl	__ZN3std2rt10lang_start17hed80f55b4a401277E
	.p2align	2
__ZN3std2rt10lang_start17hed80f55b4a401277E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	str	x1, [sp]
	mov	x0, x2
	ldr	x2, [sp]
	str	x0, [sp, #8]
	mov	x4, x3
	ldr	x3, [sp, #8]
	sub	x0, x29, #8
	stur	x8, [x29, #-8]
	adrp	x1, l___unnamed_1@PAGE
	add	x1, x1, l___unnamed_1@PAGEOFF
	bl	__ZN3std2rt19lang_start_internal17h00a235e820a7f01cE
	str	x0, [sp, #16]
	ldr	x0, [sp, #16]
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d8515c94fef1241E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h074fc9456b17fe2eE
	bl	__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hcbcbac39db48d307E
	sturb	w0, [x29, #-1]
	ldurb	w0, [x29, #-1]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt10ArgumentV111new_display17heb9c8cbb3f97bf31E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	adrp	x8, __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h6ecfe5442ff7301eE@GOTPAGE
	ldr	x8, [x8, __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h6ecfe5442ff7301eE@GOTPAGEOFF]
	str	x8, [sp, #16]
	ldr	x8, [sp, #16]
	str	x0, [sp, #24]
	ldr	x9, [sp, #24]
	str	x9, [sp]
	str	x8, [sp, #8]
	ldr	x0, [sp]
	ldr	x1, [sp, #8]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt9Arguments6new_v117h581b5b5dad9278ceE:
	.cfi_startproc
	sub	sp, sp, #144
	.cfi_def_cfa_offset 144
	stp	x29, x30, [sp, #128]
	add	x29, sp, #128
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp, #16]
	str	x0, [sp, #24]
	str	x1, [sp, #32]
	str	x2, [sp, #40]
	str	x3, [sp, #48]
	subs	x8, x1, x3
	cset	w8, lo
	tbnz	w8, #0, LBB4_2
	b	LBB4_1
LBB4_1:
	ldr	x8, [sp, #32]
	ldr	x9, [sp, #48]
	add	x9, x9, #1
	subs	x8, x8, x9
	cset	w8, hi
	and	w8, w8, #0x1
	strb	w8, [sp, #63]
	b	LBB4_3
LBB4_2:
	mov	w8, #1
	strb	w8, [sp, #63]
	b	LBB4_3
LBB4_3:
	ldrb	w8, [sp, #63]
	tbnz	w8, #0, LBB4_5
	b	LBB4_4
LBB4_4:
	ldr	x8, [sp, #48]
	ldr	x9, [sp, #16]
	ldr	x10, [sp, #40]
	ldr	x11, [sp, #32]
	ldr	x12, [sp, #24]
	stur	xzr, [x29, #-16]
	str	x12, [x9, #16]
	str	x11, [x9, #24]
	ldur	x12, [x29, #-16]
	ldur	x11, [x29, #-8]
	str	x12, [x9]
	str	x11, [x9, #8]
	str	x10, [x9, #32]
	str	x8, [x9, #40]
	.cfi_def_cfa wsp, 144
	ldp	x29, x30, [sp, #128]
	add	sp, sp, #144
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB4_5:
	.cfi_restore_state
	add	x8, sp, #64
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_2@PAGE
	add	x0, x0, l___unnamed_2@PAGEOFF
	mov	w9, #1
	mov	x1, x9
	adrp	x2, l___unnamed_3@PAGE
	add	x2, x2, l___unnamed_3@PAGEOFF
	mov	x3, #0
	bl	__ZN4core3fmt9Arguments6new_v117h581b5b5dad9278ceE
	ldr	x0, [sp, #8]
	adrp	x1, l___unnamed_4@PAGE
	add	x1, x1, l___unnamed_4@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17h23ae44661fec0889E
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h96ecab37bda04cd5E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN4core3ops8function6FnOnce9call_once17h727b737e335b745cE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h444efb08380f190eE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h727b737e335b745cE:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	add	x0, sp, #16
	str	x8, [sp, #16]
Ltmp1:
	bl	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d8515c94fef1241E
	str	w0, [sp, #12]
Ltmp2:
	b	LBB7_3
LBB7_1:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB7_2:
Ltmp3:
	mov	x8, x1
	stur	x0, [x29, #-16]
	stur	w8, [x29, #-8]
	b	LBB7_1
LBB7_3:
	ldr	w0, [sp, #12]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table7:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Ltmp2-Ltmp1
	.uleb128 Ltmp3-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp2-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp2
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h74ae027a9720512eE:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hcbcbac39db48d307E:
	.cfi_startproc
	mov	w0, #0
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7add_ten17h1b6c124dc41b5205E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp]
	ldr	w8, [x0]
	adds	w8, w8, #10
	stur	w8, [x29, #-4]
	cset	w8, vs
	tbnz	w8, #0, LBB10_2
	b	LBB10_1
LBB10_1:
	ldur	w8, [x29, #-4]
	ldr	x9, [sp]
	str	w8, [x9]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB10_2:
	.cfi_restore_state
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_5@PAGE
	add	x2, x2, l___unnamed_5@PAGEOFF
	bl	__ZN4core9panicking5panic17hce667fe29ffebbcdE
	.cfi_endproc

	.p2align	2
__ZN4main4main17h2a68d4d833d7495aE:
	.cfi_startproc
	sub	sp, sp, #176
	.cfi_def_cfa_offset 176
	stp	x29, x30, [sp, #160]
	add	x29, sp, #160
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	add	x8, sp, #40
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_6@PAGE
	add	x0, x0, l___unnamed_6@PAGEOFF
	mov	w9, #1
	mov	x1, x9
	str	x1, [sp, #24]
	adrp	x2, l___unnamed_3@PAGE
	add	x2, x2, l___unnamed_3@PAGEOFF
	mov	x3, #0
	bl	__ZN4core3fmt9Arguments6new_v117h581b5b5dad9278ceE
	ldr	x0, [sp, #8]
	bl	__ZN3std2io5stdio6_print17h8769d5b8d07b47a7E
	sub	x0, x29, #68
	str	x0, [sp, #16]
	mov	w8, #1
	stur	w8, [x29, #-68]
	bl	__ZN4main7add_ten17h1b6c124dc41b5205E
	ldr	x0, [sp, #16]
	bl	__ZN4core3fmt10ArgumentV111new_display17heb9c8cbb3f97bf31E
	ldr	x3, [sp, #24]
	sub	x2, x29, #16
	stur	x0, [x29, #-16]
	stur	x1, [x29, #-8]
	sub	x8, x29, #64
	str	x8, [sp, #32]
	adrp	x0, l___unnamed_7@PAGE
	add	x0, x0, l___unnamed_7@PAGEOFF
	mov	w9, #2
	mov	x1, x9
	bl	__ZN4core3fmt9Arguments6new_v117h581b5b5dad9278ceE
	ldr	x0, [sp, #32]
	bl	__ZN3std2io5stdio6_print17h8769d5b8d07b47a7E
	.cfi_def_cfa wsp, 176
	ldp	x29, x30, [sp, #160]
	add	sp, sp, #176
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x2, x1
	mov	x8, x0
	sxtw	x1, w8
	adrp	x0, __ZN4main4main17h2a68d4d833d7495aE@PAGE
	add	x0, x0, __ZN4main4main17h2a68d4d833d7495aE@PAGEOFF
	mov	w3, #0
	bl	__ZN3std2rt10lang_start17hed80f55b4a401277E
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3
l___unnamed_1:
	.quad	__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h74ae027a9720512eE
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h96ecab37bda04cd5E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d8515c94fef1241E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5d8515c94fef1241E

	.section	__TEXT,__const
l___unnamed_8:
	.ascii	"invalid args"

	.section	__DATA,__const
	.p2align	3
l___unnamed_2:
	.quad	l___unnamed_8
	.asciz	"\f\000\000\000\000\000\000"

	.section	__TEXT,__const
	.p2align	3
l___unnamed_3:
	.byte	0

l___unnamed_9:
	.ascii	"/rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/fmt/mod.rs"

	.section	__DATA,__const
	.p2align	3
l___unnamed_4:
	.quad	l___unnamed_9
	.asciz	"K\000\000\000\000\000\000\000\214\001\000\000\r\000\000"

	.section	__TEXT,__const
l___unnamed_10:
	.ascii	"main.rs"

	.section	__DATA,__const
	.p2align	3
l___unnamed_5:
	.quad	l___unnamed_10
	.asciz	"\007\000\000\000\000\000\000\000\002\000\000\000\005\000\000"

	.section	__TEXT,__const
	.p2align	4
_str.0:
	.ascii	"attempt to add with overflow"

l___unnamed_11:
	.ascii	"Hello, world!\n"

	.section	__DATA,__const
	.p2align	3
l___unnamed_6:
	.quad	l___unnamed_11
	.asciz	"\016\000\000\000\000\000\000"

	.section	__TEXT,__const
l___unnamed_12:
	.byte	10

	.section	__DATA,__const
	.p2align	3
l___unnamed_7:
	.quad	l___unnamed_3
	.space	8
	.quad	l___unnamed_12
	.asciz	"\001\000\000\000\000\000\000"

.subsections_via_symbols
