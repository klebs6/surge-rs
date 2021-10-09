//#![feature(const_generics)]
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

x![constants];
x![freq_shift];
x![init];
x![maybe];
x![param];
x![post_block];
x![pre_block];
x![process];
x![update];
