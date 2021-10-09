#![feature(destructuring_assignment)]
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

x![allpass];
x![delay];
x![effect];
x![constants];
x![one_pole_filter];
x![param];
x![predelay];
x![process];
x![update];
x![init];
x![calc_size];
x![block];
