# ptx-parser

`ptx-parser` is a `no_std` Rust crate for parsing PTX files.
PTX is a low-level parallel thread execution virtual machine instruction set architecture used by NVIDIA GPUs.

## Installation

To install, run

```bash
cargo add ptx-parser
```

## Planned usage

Syntax is subject to change.
Expect a parser function similar to what follows.

```rust
use ptx_parser::parse_ptx;

let ptx = "
.version 7.5
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

let result = parse_ptx(ptx);
println!("{:?}", result);
```

## License

Dual-licensed to be compatible with the `Rust` project.

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option.
