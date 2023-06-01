pub(crate) mod a;
pub(crate) mod b;
pub(crate) mod c;
pub(crate) mod d;
pub(crate) mod kernel;

pub(crate) const _EXAMPLE_FILE: &str = 
".version 7.5
.target sm_30
.address_size 64

.visible .entry _Z6kernelPiS_i
{
    .reg .f32   %f<4>;
    .reg .s32   %r<4>;
    .reg .pred  %p<3>;
    .reg .b32   %r4;
    .reg .b64   %rd<3>;

    ld.param.u64    %rd1, [__cudaparm__Z6kernelPiS_i_a];
    ld.param.u64    %rd2, [__cudaparm__Z6kernelPiS_i_b];
    ld.param.u64    %rd3, [__cudaparm__Z6kernelPiS_i_c];
    cvta.to.global.u64  %rd4, %rd1;
    cvta.to.global.u64  %rd5, %rd2;
    cvta.to.global.u64  %rd6, %rd3;
    ld.global.f32   %f1, [%rd4];
    ld.global.f32   %f2, [%rd5];
    ld.global.f32   %f3, [%rd6];
    add.f32     %f4, %f1, %f2;
    add.f32     %f4, %f4, %f3;
    st.global.f32   [%rd4], %f4;
    ret;
}
";
