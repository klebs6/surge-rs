#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(platform_intrinsics)]
#![feature(stdarch)]
#![feature(trait_alias)]

macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }

macro_rules! ix { () => ( 
    #[allow(unused_imports)]
    use crate::{ imports::* , };) 
}

#[macro_use] pub mod imports;
#[macro_use] pub mod m128;

x![align];
x![allpass];
x![cast];
x![clamp];
x![clear];
x![convert];
x![copy];
x![correlated_noise];
x![encode];
x![fast];
x![flops];
x![hardclip];
x![interp];
x![limit_range];
x![ops];
x![ord];
x![padding];
x![pan];
x![saturate];
x![scan];
x![scratch];
x![sign];
x![simd];
x![sinc];
x![sine];
x![softclip];
x![square];
x![tanh];
x![tests];
x![trixpan];
x![types];
x![util];
x![white_noise];
x![window];
