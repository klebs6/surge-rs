#![feature(in_band_lifetimes)]

#[macro_use] mod imports; use imports::*;

#[cfg(test)] mod tests;

x![process_quad];
x![coeff];
x![diode];
x![lpf];

