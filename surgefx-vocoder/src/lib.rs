#![feature(in_band_lifetimes)]

#[macro_use] mod imports; use imports::*;
#[cfg(test)] mod tests;

x![block];
x![cfg];
x![constants];
x![init];
x![mismatch];
x![param];
x![process];
x![update];
x![vocoder];
