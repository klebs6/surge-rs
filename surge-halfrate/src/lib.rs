#![feature(stdarch)]

macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( 
    #[allow(unused_imports)]
    use crate::{ imports::* , };) 
}

#[macro_use] pub mod imports;

x![coeff];
x![halfratefilter];
x![loadstore];
x![process_block];
x![process_block_d2];
x![process_block_u2];
x![softer];
x![steep];
