#![feature(in_band_lifetimes)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( use crate::{ imports::* , };) }

#[macro_use] pub mod imports;
#[cfg(test)] pub mod tests;

x![bandreject];
x![coeffs];
x![coupled];
x![iir];
x![iir_process_quad];
x![lattice];
x![quad_iir_12_cfc];
x![quad_iir_12_cfl];
x![quad_iir_12_wdf];
x![quad_iir_12_a];
x![quad_iir_12_b];
x![quad_iir_24_b];
x![quad_iir_24_cfg];
x![quad_iir_24_cfl];
x![reso];
