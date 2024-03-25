#![feature(stdarch_x86_mm_shuffle)]

#[macro_use] mod imports; use imports::*;

x!{add}
x!{fade}
x!{sse_block}
x!{lipol}
x!{lipol_ps}
x!{mac}
x!{multiply}
x!{trixpan}
