pub(crate) const _PTX: &str = "//
// Generated by LLVM NVPTX Back-End
//

.version 6.0
.target sm_30
.address_size 64

	// .globl	square_kernel

.visible .entry square_kernel(
	.param .u64 square_kernel_param_0,
	.param .u64 square_kernel_param_1,
	.param .u32 square_kernel_param_2
)
{
	.reg .pred 	%p<2>;
	.reg .b32 	%r<6>;
	.reg .f32 	%f<3>;
	.reg .b64 	%rd<8>;

	ld.param.u32 	%r1, [square_kernel_param_2];
	mov.u32 	%r2, %tid.x;
	mov.u32 	%r3, %ctaid.x;
	mov.u32 	%r4, %ntid.x;
	mad.lo.s32 	%r5, %r3, %r4, %r2;
	setp.lt.s32 	%p1, %r5, %r1;
	@%p1 bra 	$L__BB0_2;
	bra.uni 	$L__BB0_1;
$L__BB0_2:
	ld.param.u64 	%rd3, [square_kernel_param_0];
	ld.param.u64 	%rd4, [square_kernel_param_1];
	cvta.to.global.u64 	%rd5, %rd4;
	cvta.to.global.u64 	%rd6, %rd3;
	mul.wide.s32 	%rd7, %r5, 4;
	add.s64 	%rd1, %rd5, %rd7;
	add.s64 	%rd2, %rd6, %rd7;
	ld.global.f32 	%f1, [%rd2];
	mul.rn.f32 	%f2, %f1, %f1;
	st.global.f32 	[%rd1], %f2;
$L__BB0_1:
	ret;

}
";
