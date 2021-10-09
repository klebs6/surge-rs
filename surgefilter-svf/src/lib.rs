#![feature(in_band_lifetimes)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( use crate::{ imports::* , };) }

#[macro_use] pub mod imports;
#[cfg(test)] pub mod tests;

x![quad_svf_bp12_a];
x![quad_svf_bp24_a];
x![quad_svf_hp12_a];
x![quad_svf_hp24_a];
x![quad_svf_lp12_a];
x![quad_svf_lp24_a];
x![svf];
x![svf_process];
