#![feature(in_band_lifetimes)]
#![feature(test)]

#[macro_use] mod imports; use imports::*;

extern crate test;

x!{process}
x!{process_2pole}
x!{process_4pole}
x!{coeff}
x!{obxd_types}
x!{obxd}

#[cfg(test)]
x!{bench}
