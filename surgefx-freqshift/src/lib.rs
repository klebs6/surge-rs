//#![feature(const_generics)]
#![feature(in_band_lifetimes)]

#[macro_use] mod imports; use imports::*;
#[cfg(test)] mod tests;

x![constants];
x![freq_shift];
x![init];
x![maybe];
x![param];
x![post_block];
x![pre_block];
x![process];
x![update];
