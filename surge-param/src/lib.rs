#![feature(box_syntax)]

macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }

macro_rules! ix { () => ( 
    #[allow(unused_imports)]
    use crate::{ imports::* , };) 
}

#[macro_use] mod imports;

x![bound_value];
x![control];
x![display];
x![getset];
x![morph];
x![normalize];
x![parameter];
x![pdata];
x![pval];
x![traits];
