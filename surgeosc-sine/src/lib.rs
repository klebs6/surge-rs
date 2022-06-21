#![feature(in_band_lifetimes)]

#[macro_use] mod imports; use imports::*;
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
