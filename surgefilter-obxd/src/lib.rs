#![feature(in_band_lifetimes)]
#![feature(test)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { 
    () => { 
        use crate::{ 
            imports::* , 
        };
    }
}

#[macro_use] mod imports;

extern crate test;

x![process];
x![process_2pole];
x![process_4pole];
x![coeff];
x![types];
x![obxd];

#[cfg(test)]
x![bench];
