#![feature(stdarch)]
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

#[cfg(test)] mod tests;
#[macro_use] mod imports;

x![update_lagvals];
x![convolute];
x![process];
x![set_pitch];
x![init];
x![set];
x![new];
x![params];
x![constants];
x![sso];
