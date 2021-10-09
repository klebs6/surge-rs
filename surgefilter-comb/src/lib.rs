#![feature(in_band_lifetimes)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( #[allow(unused_imports)] use crate::{ imports::* , };) }

#[macro_use] pub mod imports;
x![coeff];
x![comb];
x![process_sse1];
x![process_sse2];
