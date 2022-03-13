#![feature(in_band_lifetimes)]

macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( use crate::{ imports::* , };) }

#[macro_use] pub mod imports;
x![unit];
x![handle];
x![hold_pedal];
x![channel];
