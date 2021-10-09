#![feature(in_band_lifetimes)]
#![feature(stdarch)]

macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( 
    #[allow(unused_imports)]
    use crate::{ imports::* , };) 
}

#[macro_use] pub mod imports;
#[cfg(test)] pub mod tests;

x![biquad];
x![calc];
x![init];
x![traits];
x![process];
x![process_sample];
x![coeff];
