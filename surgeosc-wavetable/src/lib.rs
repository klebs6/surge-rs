#![feature(stdarch)]
#![feature(in_band_lifetimes)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }

macro_rules! ix { 
    () => { 
        #[allow(unused_imports)]
        use crate::{ 
            imports::* , 
        };
    }
}

#[macro_use] mod imports;

x![convolute];
x![init];
x![new];
x![param];
x![process];
x![set_pitch];
x![wavetable];
x![update];
