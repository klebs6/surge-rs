#![feature(trait_alias)]
#![feature(associated_type_bounds)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( use crate::{ imports::* , };) }

#[macro_use] pub mod imports;
#[cfg(test)] pub mod tests;

x![base];
x![calc];
x![dim];
x![error];
x![init];
x![mipmap];
x![table];
x![try_from_datafile];
x![try_from_wavfile];
x![initial];
