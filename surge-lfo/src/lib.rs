#![feature(in_band_lifetimes)]

macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( #[allow(unused_imports)] use crate::{ imports::* , };) }

#[macro_use] pub mod imports;

x![attack];
x![attack_shapes];
x![bend];
x![constants];
x![get];
x![init];
x![lfo];
x![modsource];
x![param];
x![phase_over_one];
x![phaseslider];
x![process];
x![process_shapes];
x![release];
x![set];
x![stepsequencer];
x![update];
x![update_envelope];
