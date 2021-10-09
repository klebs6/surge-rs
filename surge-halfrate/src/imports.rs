#[cfg(target_arch = "x86_64")] 
pub use core::arch::x86_64::*;

pub use surge_math::{z128,m128_half,A1d,A2d};

pub use surge_traits::{
    ProcessBlock,
    ProcessBlockD2,
    ProcessBlockU2,
    CoefficientLoadStore,
    Reset
};

pub use std::convert::TryInto;
