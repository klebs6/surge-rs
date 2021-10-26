#![feature(in_band_lifetimes)]

macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( use crate::{ imports::* , };) }

#[macro_use] mod imports;

x![envelope];
x![gain];
x![handle];
x![sinc];
x![sine];
x![surge];
x![waveshape];
x![traits];
