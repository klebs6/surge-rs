#![feature(in_band_lifetimes)]

#[macro_use] mod imports; use imports::*;
#[cfg(test)] mod tests;

x![eq3band];
x![init];
x![param];
x![process];
x![update];
