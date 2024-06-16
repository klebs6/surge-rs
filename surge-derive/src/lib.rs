#![warn(dead_code)]
#![warn(unused_imports)]
extern crate proc_macro;

#[macro_use] mod imports; use imports::*;

xp!{modulation_source_control}
xp!{named}
xp!{parameter_interface}
xp!{synth_parameters}

#[proc_macro_attribute]
pub fn modulation_source_control(attr: TokenStream, item: TokenStream) -> TokenStream {
    _modulation_source_control(attr,item)
}

#[proc_macro_attribute]
pub fn name(attr: TokenStream, item: TokenStream) -> TokenStream {
    _name(attr,item)
}

#[proc_macro_attribute]
pub fn parameter_interface(attr: TokenStream, item: TokenStream) -> TokenStream {
    _parameter_interface(attr,item)
}

#[proc_macro_attribute]
pub fn synth_parameters(attr: TokenStream, item: TokenStream) -> TokenStream {
    _synth_parameters(attr,item)
}
