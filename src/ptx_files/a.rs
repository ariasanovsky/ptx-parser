const _PTX: &str = 
"// 
// Generated by LLVM NVPTX Back-End
//

.version 6.0
.target sm_30
.address_size 64

.func  (.param .align 16 .b8 func_retval0[16]) _ZN12libgdx_xs1283rng6Random3new17hf4ea8272c085a319E
(
	.param .b64 _ZN12libgdx_xs1283rng6Random3new17hf4ea8272c085a319E_param_0
)
;
.func  (.param .b64 func_retval0) _ZN12libgdx_xs1283rng6Random15next_capped_u6417hfcf43a4580a75759E
(
	.param .b64 _ZN12libgdx_xs1283rng6Random15next_capped_u6417hfcf43a4580a75759E_param_0,
	.param .b64 _ZN12libgdx_xs1283rng6Random15next_capped_u6417hfcf43a4580a75759E_param_1
)
;
.func _ZN4core3ptr88drop_in_place$LT$core$$panic$$panic_info$$PanicInfo$$internal_constructor$$NoPayload$GT$17h87ccdb342c4f9dd2E
(
	.param .b64 _ZN4core3ptr88drop_in_place$LT$core$$panic$$panic_info$$PanicInfo$$internal_constructor$$NoPayload$GT$17h87ccdb342c4f9dd2E_param_0
)
;
.func  (.param .b64 func_retval0) _ZN36_$LT$T$u20$as$u20$core$$any$$Any$GT$7type_id17h2da4916f5531a2feE
(
	.param .b64 _ZN36_$LT$T$u20$as$u20$core$$any$$Any$GT$7type_id17h2da4916f5531a2feE_param_0
)
;
.func _ZN4core9panicking5panic17h691abfa2aca02139E
(
	.param .b64 _ZN4core9panicking5panic17h691abfa2aca02139E_param_0,
	.param .b64 _ZN4core9panicking5panic17h691abfa2aca02139E_param_1,
	.param .b64 _ZN4core9panicking5panic17h691abfa2aca02139E_param_2
)
;
.global .align 1 .b8 anon_$_03c68ec27500ea325c9bf6f32c376c28_$_0[40] = {105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110, 116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99, 111, 100, 101};
.global .align 1 .b8 anon_$_03c68ec27500ea325c9bf6f32c376c28_$_1[10] = {115, 114, 99, 47, 108, 105, 98, 46, 114, 115};
.global .align 8 .u64 anon_$_03c68ec27500ea325c9bf6f32c376c28_$_2[3] = {generic(anon_$_03c68ec27500ea325c9bf6f32c376c28_$_1), 10, 21474836492};
.global .align 1 .b8 anon_$_05858f77f17aa106f387de81a58eb46f_$_0[37] = {47, 104, 111, 109, 101, 47, 115, 116, 115, 47, 108, 105, 98, 103, 100, 120, 45, 120, 115, 49, 50, 56, 47, 115, 114, 99, 47, 114, 110, 103, 47, 109, 111, 100, 46, 114, 115};
.global .align 8 .u64 anon_$_05858f77f17aa106f387de81a58eb46f_$_1[3] = {generic(anon_$_05858f77f17aa106f387de81a58eb46f_$_0), 37, 115964117049};
.global .align 1 .b8 str_$_0[57] = {97, 116, 116, 101, 109, 112, 116, 32, 116, 111, 32, 99, 97, 108, 99, 117, 108, 97, 116, 101, 32, 116, 104, 101, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114, 32, 119, 105, 116, 104, 32, 97, 32, 100, 105, 118, 105, 115, 111, 114, 32, 111, 102, 32, 122, 101, 114, 111};
.global .align 8 .b8 anon_$_af85108618407798382bf1e18eed69f7_$_2;
.global .align 8 .u64 anon_$_af85108618407798382bf1e18eed69f7_$_262[4] = {_ZN4core3ptr88drop_in_place$LT$core$$panic$$panic_info$$PanicInfo$$internal_constructor$$NoPayload$GT$17h87ccdb342c4f9dd2E, 0, 1, _ZN36_$LT$T$u20$as$u20$core$$any$$Any$GT$7type_id17h2da4916f5531a2feE};

.func rust_begin_unwind(
	.param .b64 rust_begin_unwind_param_0
)
{
	.reg .b64 	%rd<6>;

	mov.u64 	%rd1, anon_$_03c68ec27500ea325c9bf6f32c376c28_$_0;
	cvta.global.u64 	%rd2, %rd1;
	mov.u64 	%rd3, anon_$_03c68ec27500ea325c9bf6f32c376c28_$_2;
	cvta.global.u64 	%rd4, %rd3;
	mov.u64 	%rd5, 40;
	{ // callseq 0, 0
	.reg .b32 temp_param_reg;
	.param .b64 param0;
	st.param.b64 	[param0+0], %rd2;
	.param .b64 param1;
	st.param.b64 	[param1+0], %rd5;
	.param .b64 param2;
	st.param.b64 	[param2+0], %rd4;
	call.uni 
	_ZN4core9panicking5panic17h691abfa2aca02139E, 
	(
	param0, 
	param1, 
	param2
	);
	} // callseq 0

}
	// .globl	run_length
.visible .entry run_length(
	.param .u64 run_length_param_0,
	.param .u32 run_length_param_1
)
{
	.local .align 8 .b8 	__local_depot1[16];
	.reg .b64 	%SP;
	.reg .b64 	%SPL;
	.reg .pred 	%p<6>;
	.reg .b32 	%r<12>;
	.reg .b64 	%rd<25>;

	mov.u64 	%SPL, __local_depot1;
	cvta.local.u64 	%SP, %SPL;
	ld.param.u32 	%r5, [run_length_param_1];
	mov.u32 	%r1, %tid.x;
	mov.u32 	%r6, %ctaid.x;
	mov.u32 	%r7, %ntid.x;
	mad.lo.s32 	%r2, %r6, %r7, %r1;
	setp.ge.s32 	%p1, %r2, %r5;
	@%p1 bra 	$L__BB1_8;
	cvt.s64.s32 	%rd10, %r1;
	add.s64 	%rd24, %rd10, 1;
	setp.gt.u64 	%p2, %rd24, 999999999999;
	@%p2 bra 	$L__BB1_8;
	ld.param.u64 	%rd8, [run_length_param_0];
	cvta.to.global.u64 	%rd1, %rd8;
	add.u64 	%rd9, %SP, 0;
	add.u64 	%rd2, %SPL, 0;
	mov.u32 	%r8, %nctaid.x;
	mul.lo.s32 	%r9, %r7, %r8;
	cvt.s64.s32 	%rd3, %r9;
	mov.u64 	%rd16, 71;
	mov.u32 	%r10, -1;
$L__BB1_3:
	{ // callseq 1, 0
	.reg .b32 temp_param_reg;
	.param .b64 param0;
	st.param.b64 	[param0+0], %rd24;
	.param .align 16 .b8 retval0[16];
	call.uni (retval0), 
	_ZN12libgdx_xs1283rng6Random3new17hf4ea8272c085a319E, 
	(
	param0
	);
	ld.param.v2.b64 	{%rd11, %rd12}, [retval0+0];
	} // callseq 1
	st.local.u64 	[%rd2], %rd11;
	st.local.u64 	[%rd2+8], %rd12;
	{ // callseq 2, 0
	.reg .b32 temp_param_reg;
	.param .b64 param0;
	st.param.b64 	[param0+0], %rd9;
	.param .b64 param1;
	st.param.b64 	[param1+0], %rd16;
	.param .b64 retval0;
	call.uni (retval0), 
	_ZN12libgdx_xs1283rng6Random15next_capped_u6417hfcf43a4580a75759E, 
	(
	param0, 
	param1
	);
	ld.param.b64 	%rd17, [retval0+0];
	} // callseq 2
	mov.u32 	%r11, %r10;
$L__BB1_4:
	{ // callseq 3, 0
	.reg .b32 temp_param_reg;
	.param .b64 param0;
	st.param.b64 	[param0+0], %rd9;
	.param .b64 param1;
	st.param.b64 	[param1+0], %rd16;
	.param .b64 retval0;
	call.uni (retval0), 
	_ZN12libgdx_xs1283rng6Random15next_capped_u6417hfcf43a4580a75759E, 
	(
	param0, 
	param1
	);
	ld.param.b64 	%rd20, [retval0+0];
	} // callseq 3
	setp.eq.s64 	%p3, %rd20, %rd17;
	add.s32 	%r11, %r11, 1;
	@%p3 bra 	$L__BB1_4;
	setp.gt.s32 	%p4, %r11, 4;
	@%p4 bra 	$L__BB1_7;
	add.s64 	%rd24, %rd24, %rd3;
	setp.lt.u64 	%p5, %rd24, 1000000000000;
	@%p5 bra 	$L__BB1_3;
	bra.uni 	$L__BB1_8;
$L__BB1_7:
	mul.wide.s32 	%rd22, %r2, 8;
	add.s64 	%rd23, %rd1, %rd22;
	st.global.u64 	[%rd23], %rd24;
$L__BB1_8:
	ret;

}
	// .globl	thread_id
.visible .entry thread_id(
	.param .u64 thread_id_param_0,
	.param .u32 thread_id_param_1
)
{
	.reg .pred 	%p<2>;
	.reg .b32 	%r<6>;
	.reg .f32 	%f<2>;
	.reg .b64 	%rd<5>;

	ld.param.u32 	%r1, [thread_id_param_1];
	mov.u32 	%r2, %tid.x;
	mov.u32 	%r3, %ctaid.x;
	mov.u32 	%r4, %ntid.x;
	mad.lo.s32 	%r5, %r3, %r4, %r2;
	setp.ge.s32 	%p1, %r5, %r1;
	@%p1 bra 	$L__BB2_2;
	ld.param.u64 	%rd2, [thread_id_param_0];
	cvta.to.global.u64 	%rd3, %rd2;
	cvt.rn.f32.s32 	%f1, %r5;
	mul.wide.s32 	%rd4, %r5, 4;
	add.s64 	%rd1, %rd3, %rd4;
	st.global.f32 	[%rd1], %f1;
$L__BB2_2:
	ret;

}
.func  (.param .align 16 .b8 func_retval0[16]) _ZN107_$LT$libgdx_xs128$$rng$$Random$u20$as$u20$core$$convert$$From$LT$libgdx_xs128$$rng$$SeedInitializer$GT$$GT$4from17hbbe8d20273de3197E(
	.param .b64 _ZN107_$LT$libgdx_xs128$$rng$$Random$u20$as$u20$core$$convert$$From$LT$libgdx_xs128$$rng$$SeedInitializer$GT$$GT$4from17hbbe8d20273de3197E_param_0
)
{
	.local .align 8 .b8 	__local_depot3[24];
	.reg .b64 	%SP;
	.reg .b64 	%SPL;
	.reg .pred 	%p<4>;
	.reg .b64 	%rd<60>;

	mov.u64 	%SPL, __local_depot3;
	cvta.local.u64 	%SP, %SPL;
	ld.param.u64 	%rd14, [_ZN107_$LT$libgdx_xs128$$rng$$Random$u20$as$u20$core$$convert$$From$LT$libgdx_xs128$$rng$$SeedInitializer$GT$$GT$4from17hbbe8d20273de3197E_param_0];
	add.u64 	%rd16, %SP, 0;
	add.u64 	%rd1, %SPL, 0;
	ld.u64 	%rd15, [%rd14];
	setp.gt.s64 	%p1, %rd15, 1;
	@%p1 bra 	$L__BB3_5;
	setp.eq.s64 	%p3, %rd15, 0;
	@%p3 bra 	$L__BB3_2;
	bra.uni 	$L__BB3_3;
$L__BB3_2:
	ld.u64 	%rd59, [%rd14+8];
	ld.u64 	%rd58, [%rd14+16];
	bra.uni 	$L__BB3_7;
$L__BB3_5:
	setp.eq.s64 	%p2, %rd15, 2;
	@%p2 bra 	$L__BB3_4;
	bra.uni 	$L__BB3_6;
$L__BB3_4:
	ld.u64 	%rd32, [%rd14+8];
	shr.u64 	%rd33, %rd32, 33;
	xor.b64  	%rd34, %rd33, %rd32;
	mul.lo.s64 	%rd35, %rd34, -49064778989728563;
	shr.u64 	%rd36, %rd35, 33;
	xor.b64  	%rd37, %rd36, %rd35;
	mul.lo.s64 	%rd38, %rd37, -4265267296055464877;
	shr.u64 	%rd39, %rd38, 33;
	xor.b64  	%rd40, %rd39, %rd38;
	st.local.u64 	[%rd1+8], %rd32;
	st.local.u64 	[%rd1+16], %rd40;
	mov.u64 	%rd41, 0;
	st.local.u64 	[%rd1], %rd41;
	{ // callseq 5, 0
	.reg .b32 temp_param_reg;
	.param .b64 param0;
	st.param.b64 	[param0+0], %rd16;
	.param .align 16 .b8 retval0[16];
	call.uni (retval0), 
	_ZN107_$LT$libgdx_xs128$$rng$$Random$u20$as$u20$core$$convert$$From$LT$libgdx_xs128$$rng$$SeedInitializer$GT$$GT$4from17hbbe8d20273de3197E, 
	(
	param0
	);
	ld.param.v2.b64 	{%rd59, %rd58}, [retval0+0];
	} // callseq 5
	bra.uni 	$L__BB3_7;
$L__BB3_3:
	ld.u64 	%rd45, [%rd14+8];
	shr.u64 	%rd46, %rd45, 33;
	xor.b64  	%rd47, %rd46, %rd45;
	mul.lo.s64 	%rd48, %rd47, -49064778989728563;
	shr.u64 	%rd49, %rd48, 33;
	xor.b64  	%rd50, %rd49, %rd48;
	mul.lo.s64 	%rd51, %rd50, -4265267296055464877;
	shr.u64 	%rd52, %rd51, 33;
	xor.b64  	%rd53, %rd52, %rd51;
	st.local.u64 	[%rd1+8], %rd53;
	mov.u64 	%rd54, 2;
	st.local.u64 	[%rd1], %rd54;
	{ // callseq 6, 0
	.reg .b32 temp_param_reg;
	.param .b64 param0;
	st.param.b64 	[param0+0], %rd16;
	.param .align 16 .b8 retval0[16];
	call.uni (retval0), 
	_ZN107_$LT$libgdx_xs128$$rng$$Random$u20$as$u20$core$$convert$$From$LT$libgdx_xs128$$rng$$SeedInitializer$GT$$GT$4from17hbbe8d20273de3197E, 
	(
	param0
	);
	ld.param.v2.b64 	{%rd59, %rd58}, [retval0+0];
	} // callseq 6
	bra.uni 	$L__BB3_7;
$L__BB3_6:
	ld.u64 	%rd19, [%rd14+8];
	shr.u64 	%rd20, %rd19, 33;
	xor.b64  	%rd21, %rd20, %rd19;
	mul.lo.s64 	%rd22, %rd21, -7154897129451604005;
	shr.u64 	%rd23, %rd22, 33;
	xor.b64  	%rd24, %rd23, %rd22;
	mul.lo.s64 	%rd25, %rd24, 5725274745694666757;
	shr.u64 	%rd26, %rd25, 33;
	xor.b64  	%rd27, %rd26, %rd25;
	st.local.u64 	[%rd1+8], %rd27;
	st.local.u64 	[%rd1+16], %rd19;
	mov.u64 	%rd28, 0;
	st.local.u64 	[%rd1], %rd28;
	{ // callseq 4, 0
	.reg .b32 temp_param_reg;
	.param .b64 param0;
	st.param.b64 	[param0+0], %rd16;
	.param .align 16 .b8 retval0[16];
	call.uni (retval0), 
	_ZN107_$LT$libgdx_xs128$$rng$$Random$u20$as$u20$core$$convert$$From$LT$libgdx_xs128$$rng$$SeedInitializer$GT$$GT$4from17hbbe8d20273de3197E, 
	(
	param0
	);
	ld.param.v2.b64 	{%rd59, %rd58}, [retval0+0];
	} // callseq 4
$L__BB3_7:
	st.param.v2.b64 	[func_retval0+0], {%rd59, %rd58};
	ret;

}
.func  (.param .align 16 .b8 func_retval0[16]) _ZN12libgdx_xs1283rng6Random3new17hf4ea8272c085a319E(
	.param .b64 _ZN12libgdx_xs1283rng6Random3new17hf4ea8272c085a319E_param_0
)
{
	.local .align 8 .b8 	__local_depot4[24];
	.reg .b64 	%SP;
	.reg .b64 	%SPL;
	.reg .pred 	%p<2>;
	.reg .b64 	%rd<10>;

	mov.u64 	%SPL, __local_depot4;
	cvta.local.u64 	%SP, %SPL;
	ld.param.u64 	%rd1, [_ZN12libgdx_xs1283rng6Random3new17hf4ea8272c085a319E_param_0];
	add.u64 	%rd2, %SP, 0;
	add.u64 	%rd3, %SPL, 0;
	setp.eq.s64 	%p1, %rd1, 0;
	selp.b64 	%rd4, -9223372036854775808, %rd1, %p1;
	st.local.u64 	[%rd3+8], %rd4;
	mov.u64 	%rd5, 1;
	st.local.u64 	[%rd3], %rd5;
	{ // callseq 7, 0
	.reg .b32 temp_param_reg;
	.param .b64 param0;
	st.param.b64 	[param0+0], %rd2;
	.param .align 16 .b8 retval0[16];
	call.uni (retval0), 
	_ZN107_$LT$libgdx_xs128$$rng$$Random$u20$as$u20$core$$convert$$From$LT$libgdx_xs128$$rng$$SeedInitializer$GT$$GT$4from17hbbe8d20273de3197E, 
	(
	param0
	);
	ld.param.v2.b64 	{%rd6, %rd7}, [retval0+0];
	} // callseq 7
	st.param.v2.b64 	[func_retval0+0], {%rd6, %rd7};
	ret;

}
.func  (.param .b64 func_retval0) _ZN12libgdx_xs1283rng6Random15next_capped_u6417hfcf43a4580a75759E(
	.param .b64 _ZN12libgdx_xs1283rng6Random15next_capped_u6417hfcf43a4580a75759E_param_0,
	.param .b64 _ZN12libgdx_xs1283rng6Random15next_capped_u6417hfcf43a4580a75759E_param_1
)
{
	.reg .pred 	%p<4>;
	.reg .b32 	%r<4>;
	.reg .b64 	%rd<30>;

	ld.param.u64 	%rd11, [_ZN12libgdx_xs1283rng6Random15next_capped_u6417hfcf43a4580a75759E_param_1];
	ld.param.u64 	%rd10, [_ZN12libgdx_xs1283rng6Random15next_capped_u6417hfcf43a4580a75759E_param_0];
	ld.u64 	%rd28, [%rd10];
	ld.u64 	%rd2, [%rd10+8];
	setp.eq.s64 	%p1, %rd11, 0;
	cvt.u32.u64 	%r1, %rd11;
	mov.u64 	%rd27, %rd2;
	bra.uni 	$L__BB5_1;
$L__BB5_4:
	rem.u64 	%rd29, %rd6, %rd11;
$L__BB5_5:
	add.s64 	%rd21, %rd6, %rd11;
	setp.gt.u64 	%p3, %rd21, %rd29;
	mov.u64 	%rd28, %rd3;
	@%p3 bra 	$L__BB5_6;
$L__BB5_1:
	mov.u64 	%rd3, %rd27;
	shl.b64 	%rd12, %rd28, 23;
	xor.b64  	%rd13, %rd12, %rd28;
	shr.u64 	%rd14, %rd13, 17;
	shr.u64 	%rd15, %rd3, 26;
	xor.b64  	%rd16, %rd15, %rd14;
	xor.b64  	%rd17, %rd16, %rd3;
	xor.b64  	%rd27, %rd17, %rd13;
	@%p1 bra 	$L__BB5_7;
	add.s64 	%rd18, %rd27, %rd3;
	shr.u64 	%rd6, %rd18, 1;
	or.b64  	%rd19, %rd6, %rd11;
	and.b64  	%rd20, %rd19, -4294967296;
	setp.ne.s64 	%p2, %rd20, 0;
	@%p2 bra 	$L__BB5_4;
	cvt.u32.u64 	%r2, %rd6;
	rem.u32 	%r3, %r2, %r1;
	cvt.u64.u32 	%rd29, %r3;
	bra.uni 	$L__BB5_5;
$L__BB5_6:
	st.u64 	[%rd10], %rd3;
	st.u64 	[%rd10+8], %rd27;
	st.param.b64 	[func_retval0+0], %rd29;
	ret;
$L__BB5_7:
	st.u64 	[%rd10], %rd2;
	st.u64 	[%rd10+8], %rd27;
	mov.u64 	%rd22, str_$_0;
	cvta.global.u64 	%rd23, %rd22;
	mov.u64 	%rd24, anon_$_05858f77f17aa106f387de81a58eb46f_$_1;
	cvta.global.u64 	%rd25, %rd24;
	mov.u64 	%rd26, 57;
	{ // callseq 8, 0
	.reg .b32 temp_param_reg;
	.param .b64 param0;
	st.param.b64 	[param0+0], %rd23;
	.param .b64 param1;
	st.param.b64 	[param1+0], %rd26;
	.param .b64 param2;
	st.param.b64 	[param2+0], %rd25;
	call.uni 
	_ZN4core9panicking5panic17h691abfa2aca02139E, 
	(
	param0, 
	param1, 
	param2
	);
	} // callseq 8

}
.func _ZN4core9panicking9panic_fmt17hd63e3b70f89f45acE(
	.param .b64 _ZN4core9panicking9panic_fmt17hd63e3b70f89f45acE_param_0,
	.param .b64 _ZN4core9panicking9panic_fmt17hd63e3b70f89f45acE_param_1
)
{
	.local .align 8 .b8 	__local_depot6[40];
	.reg .b64 	%SP;
	.reg .b64 	%SPL;
	.reg .b16 	%rs<2>;
	.reg .b64 	%rd<9>;

	mov.u64 	%SPL, __local_depot6;
	cvta.local.u64 	%SP, %SPL;
	ld.param.u64 	%rd1, [_ZN4core9panicking9panic_fmt17hd63e3b70f89f45acE_param_0];
	ld.param.u64 	%rd2, [_ZN4core9panicking9panic_fmt17hd63e3b70f89f45acE_param_1];
	add.u64 	%rd3, %SP, 0;
	add.u64 	%rd4, %SPL, 0;
	mov.u64 	%rd5, anon_$_af85108618407798382bf1e18eed69f7_$_2;
	cvta.global.u64 	%rd6, %rd5;
	st.local.u64 	[%rd4], %rd6;
	mov.u64 	%rd7, anon_$_af85108618407798382bf1e18eed69f7_$_262;
	cvta.global.u64 	%rd8, %rd7;
	st.local.u64 	[%rd4+8], %rd8;
	st.local.u64 	[%rd4+24], %rd1;
	st.local.u64 	[%rd4+16], %rd2;
	mov.u16 	%rs1, 1;
	st.local.u8 	[%rd4+32], %rs1;
	{ // callseq 9, 0
	.reg .b32 temp_param_reg;
	.param .b64 param0;
	st.param.b64 	[param0+0], %rd3;
	call.uni 
	rust_begin_unwind, 
	(
	param0
	);
	} // callseq 9

}
.func _ZN4core3ptr88drop_in_place$LT$core$$panic$$panic_info$$PanicInfo$$internal_constructor$$NoPayload$GT$17h87ccdb342c4f9dd2E(
	.param .b64 _ZN4core3ptr88drop_in_place$LT$core$$panic$$panic_info$$PanicInfo$$internal_constructor$$NoPayload$GT$17h87ccdb342c4f9dd2E_param_0
)
{


	ret;

}
.func  (.param .b64 func_retval0) _ZN36_$LT$T$u20$as$u20$core$$any$$Any$GT$7type_id17h2da4916f5531a2feE(
	.param .b64 _ZN36_$LT$T$u20$as$u20$core$$any$$Any$GT$7type_id17h2da4916f5531a2feE_param_0
)
{
	.reg .b64 	%rd<2>;

	mov.u64 	%rd1, -578382020746933338;
	st.param.b64 	[func_retval0+0], %rd1;
	ret;

}
.func _ZN4core9panicking5panic17h691abfa2aca02139E(
	.param .b64 _ZN4core9panicking5panic17h691abfa2aca02139E_param_0,
	.param .b64 _ZN4core9panicking5panic17h691abfa2aca02139E_param_1,
	.param .b64 _ZN4core9panicking5panic17h691abfa2aca02139E_param_2
)
{
	.local .align 8 .b8 	__local_depot9[64];
	.reg .b64 	%SP;
	.reg .b64 	%SPL;
	.reg .b64 	%rd<12>;

	mov.u64 	%SPL, __local_depot9;
	cvta.local.u64 	%SP, %SPL;
	ld.param.u64 	%rd1, [_ZN4core9panicking5panic17h691abfa2aca02139E_param_0];
	ld.param.u64 	%rd2, [_ZN4core9panicking5panic17h691abfa2aca02139E_param_1];
	add.u64 	%rd3, %SP, 0;
	add.u64 	%rd4, %SPL, 0;
	ld.param.u64 	%rd5, [_ZN4core9panicking5panic17h691abfa2aca02139E_param_2];
	add.u64 	%rd6, %SP, 16;
	add.u64 	%rd7, %SPL, 16;
	st.local.u64 	[%rd4], %rd1;
	st.local.u64 	[%rd4+8], %rd2;
	st.local.u64 	[%rd7], %rd3;
	mov.u64 	%rd8, 1;
	st.local.u64 	[%rd7+8], %rd8;
	mov.u64 	%rd9, 0;
	st.local.u64 	[%rd7+32], %rd9;
	mov.u64 	%rd10, anon_$_af85108618407798382bf1e18eed69f7_$_2;
	cvta.global.u64 	%rd11, %rd10;
	st.local.u64 	[%rd7+16], %rd11;
	st.local.u64 	[%rd7+24], %rd9;
	{ // callseq 10, 0
	.reg .b32 temp_param_reg;
	.param .b64 param0;
	st.param.b64 	[param0+0], %rd6;
	.param .b64 param1;
	st.param.b64 	[param1+0], %rd5;
	call.uni 
	_ZN4core9panicking9panic_fmt17hd63e3b70f89f45acE, 
	(
	param0, 
	param1
	);
	} // callseq 10

}
";