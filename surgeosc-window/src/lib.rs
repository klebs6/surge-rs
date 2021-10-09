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

#[macro_use] mod imports;

x![clear];
x![constants];
x![window];
x![param];
x![init];
x![new];
x![process];
x![sub];
