#![feature(in_band_lifetimes)]
#![feature(stdarch)]

#[cfg(test)] pub mod tests;
#[macro_use] mod imports; use imports::*;

x!{biquad}
x!{calc}
x!{init}
x!{biquad_traits}
x!{process}
x!{process_sample}
x!{coeff}
