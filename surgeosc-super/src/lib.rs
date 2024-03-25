#![feature(stdarch_x86_mm_shuffle)]

#[cfg(test)] mod tests;
#[macro_use] mod imports; use imports::*;

x![update_lagvals];
x![convolute];
x![process];
x![set_pitch];
x![init];
x![set];
x![new];
x![params];
x![sso];
