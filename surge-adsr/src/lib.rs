#![feature(in_band_lifetimes)]

#![allow(clippy::redundant_field_names)]
#[allow(non_camel_case_types)]

macro_rules! x { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( use crate::{ imports::* , };) }

#[macro_use] pub mod imports;

#[macro_export] macro_rules! tsyncratio {
    ($self:ident, $x:ident) => ({
        let temposync = $self.params[AdsrParam::$x].temposync;
        match temposync {
            true => $self.time_unit.temposyncratio(),
            false => 1.0,
        }
    })
}

x![adsr];
x![analog];
x![digital];
x![digital_attack];
x![digital_decay];
x![digital_release];
x![digital_uberrelease];
x![init];
x![param];
x![source];
