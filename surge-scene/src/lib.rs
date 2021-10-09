#![feature(in_band_lifetimes)]
#![feature(box_syntax)]
#![feature(box_patterns)]

macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { 
    () => ( 
        #[allow(unused_imports)] 
        use crate::imports::*;
    ) 
}

#[macro_use] mod imports;

x![scene];
x![patch];
