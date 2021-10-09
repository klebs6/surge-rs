#![feature(in_band_lifetimes)]
#![feature(stdarch)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( use crate::{ imports::* , };) }

#[macro_use] pub mod imports;

x![sine];

#[cfg(not(target_arch = "x86_64"))] 
x![sse1];

#[cfg(target_arch = "x86_64")] 
x![sse2];

#[test] fn smoke() {
    ix!();

    let srunit = SampleRateHandle::new();
    let tables = TablesHandle::new(&srunit);

    let shaper = SineShaper::new(&tables);

    let result = shaper.shape(m128_one![], m128_half![]);
    println!("result: {:?}", result);

    let result = shaper.shape(result, m128_half![]);
    println!("result: {:?}", result);
}
