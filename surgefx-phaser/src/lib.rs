#![feature(in_band_lifetimes)]

#[macro_use] mod imports; use imports::*;
#[cfg(test)] mod tests;

x![block];
x![clear];
x![constants];
x![init];
x![param];
x![phaser];
x![process];
x![update];
