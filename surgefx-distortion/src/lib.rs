#![allow(incomplete_features)]
#![feature(in_band_lifetimes)]
#![feature(const_evaluatable_checked)]
#![feature(const_generics)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { 
    () => { 
        use crate::{ 
            imports::* , 
        };
    }
}

#[macro_use] mod imports;
#[cfg(test)] mod tests;

x![block];
x![distortion];
x![get];
x![init];
x![mismatch];
x![param];
x![process];
x![update];
