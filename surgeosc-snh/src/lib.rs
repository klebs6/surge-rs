#![feature(in_band_lifetimes)]
#![feature(stdarch)]

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

x![clear];
x![convolute];
x![constants];
x![init];
x![new];
x![param];
x![process];
x![set_pitch];
x![snh];
x![update];
