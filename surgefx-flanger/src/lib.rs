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

x![aux];
x![block];
x![flanger];
x![constants];
x![filter];
x![init];
x![init_comb];
x![interp_delay];
x![param];
x![process];
x![weights];
