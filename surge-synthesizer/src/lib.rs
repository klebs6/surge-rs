#![allow(dead_code)]
#![feature(const_mut_refs)]
#![feature(in_band_lifetimes)]
#![feature(associated_type_defaults)]
#![feature(trait_alias)]
#![feature(core_intrinsics)]
#![feature(platform_intrinsics)]
#![feature(stdsimd)]
#![feature(associated_type_bounds)]
#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(stdarch)]
#![allow(non_camel_case_types)]
#![feature(test)]

macro_rules! x { 
    ($x:ident) => { 
        mod $x; 
        pub use $x::*; 
    } 
}

macro_rules! ix { 
    () => { 
        #[allow(unused_imports)]
        use crate::{ imports::* , };
    }
}

#[macro_use] extern crate vst;
#[macro_use] extern crate lazy_static;

extern crate num;
extern crate test;

extern crate proc_macro;

#[macro_use] mod imports;
use crate::imports::*;

x![traits];
x![freq];
x![host];
x![vst_host];
x![synth];
x![run_host];

