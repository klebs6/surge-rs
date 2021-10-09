
macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( use crate::{ imports::* , };) }

#[macro_use] pub mod imports;

x![effect];
x![filter];
x![modsource];
x![oscillator];
x![shaper];
x![synth];
x![encapsulated];
