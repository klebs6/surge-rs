#![feature(trait_alias)]

#[macro_use] mod imports; use imports::*;

#[cfg(test)] pub mod tests;

x!{base}
x!{calc}
x!{dim}
x!{error}
x!{init}
x!{mipmap}
x!{table}
x!{try_from_datafile}
x!{try_from_wavfile}
x!{initial}
