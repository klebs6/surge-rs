#![allow(soft_unstable)]
#![allow(internal_features)]
#![allow(unused_imports)]
#![feature(test)]
#![feature(core_intrinsics)]
#![feature(platform_intrinsics)]
#![feature(stdarch)]
#![feature(trait_alias)]

#[macro_use] mod imports; use imports::*;
#[macro_use] pub mod m128;

x!{align}
x!{allpass}
x!{value_cast}
x!{clamp}
x!{clear}
x!{convert}
x!{copy}
x!{correlated_noise}
x!{encode}
x!{fast}
x!{flops}
x!{hardclip}
x!{interp}
x!{limit_range}
x!{basic_ops}
x!{ord}
x!{padding}
x!{pan}
x!{saturate}
x!{scan}
x!{scratch}
x!{simd}
x!{sinc}
x!{sine}
x!{softclip}
x!{square}
x!{tanh}
x!{tests}
x!{trixpan}
x!{util}
x!{white_noise}
x!{window}
