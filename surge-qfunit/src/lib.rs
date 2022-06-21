#![feature(in_band_lifetimes)]
#![feature(box_syntax)]
#![feature(stdarch)]

#[macro_use] mod imports; use imports::*;
#[macro_use] pub mod macros;

x![config];
x![dispatch];
x![do_dual1];
x![do_dual2];
x![do_ring];
x![do_serial1];
x![do_serial2];
x![do_serial3];
x![do_stereo];
x![do_wide];
x![qf_chain_state];
x![qf_unit_state];
x![waveshaper];
x![write_outputs];
