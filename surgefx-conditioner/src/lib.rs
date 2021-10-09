#![feature(in_band_lifetimes)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { 
    () => { 
        #[allow(unused_imports)]
        use crate::{ 
            imports::* , 
            constants::* , 
        };
    }
}

#[macro_use] mod imports;
#[cfg(test)] mod tests;

x![block];
x![cfg];
x![conditioner];
x![constants];
x![init];
x![param];
x![process];
x![lookahead];
x![update];
