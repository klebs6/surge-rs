#![feature(in_band_lifetimes)]

#[macro_use] mod imports; use imports::*;
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
