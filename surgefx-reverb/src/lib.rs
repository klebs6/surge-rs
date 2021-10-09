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
x![clear];
x![constants];
x![get];
x![init];
x![new];
x![param];
x![post_tap];
x![pre_tap];
x![preset];
x![process];
x![reverb];
x![update];
