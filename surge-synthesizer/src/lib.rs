#![allow(dead_code)]
#![feature(const_mut_refs)]
#![feature(associated_type_defaults)]
#![feature(trait_alias)]
#![feature(box_patterns)]
#![feature(stdarch_x86_mm_shuffle)]
#![allow(non_camel_case_types)]
#![feature(test)]

#[macro_use] mod imports; use imports::*;

#[macro_use] extern crate vst;

x!{synth_traits}
x!{freq}
x!{host}
x!{vst_host}
x!{synth}
x!{run_host}

