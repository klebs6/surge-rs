#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(in_band_lifetimes)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*;       } }
macro_rules! ix { ()         => { use crate::{ imports::* , }; } }

#[macro_use] pub mod imports;

x![ed2];
x![handle];
x![ed3];
x![ed4];
x![hd2];
x![hd3];
x![hd4];
x![keyboard_mapping];
x![misc];
x![remap];
x![retune];
x![scale];
x![sd2];
x![sd3];
x![sd4];
x![tone];
x![tuner];
x![tuning];
x![tables];
x![traits];

