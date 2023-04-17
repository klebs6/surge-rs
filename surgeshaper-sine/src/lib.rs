#![feature(stdarch)]

#[macro_use] mod imports; use imports::*;

x![sine];

#[cfg(not(target_arch = "x86_64"))] 
x![sse1];

#[cfg(target_arch = "x86_64")] 
x![sse2];

x![test_sineshaper];
