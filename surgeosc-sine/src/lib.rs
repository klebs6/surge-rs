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
#[cfg(test)] mod tests;

x![block];
x![compute];
x![init];
x![new];
x![legacy];
x![mismatch];
x![param];
x![process];
x![sine];
