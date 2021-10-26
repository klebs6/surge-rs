pub use enum_dispatch::enum_dispatch;
pub use enhanced_enum::enhanced_enum;
pub use std::fmt::Debug;
pub use std::fs::OpenOptions;
pub use bitflags::bitflags;
pub use core::ffi::c_void;
pub use num_traits::{Zero,SaturatingMul};
pub use std::convert::TryFrom;

pub use surge_math::*;
pub use surge_types::*;
pub use surge_traits::*;
pub use surge_constants::*;
pub use surge_hound as hound;

pub use std::fs::File;
pub use std::convert::TryInto;
pub use std::ops::Mul;

pub trait MaybeSaturatingMul {
    fn maybe_saturating_mul(self, v: Self) -> Self;
}

impl MaybeSaturatingMul for f32 {
    fn maybe_saturating_mul(self, v: Self) -> Self {
        self * v
    }
}

impl MaybeSaturatingMul for i16 {
    fn maybe_saturating_mul(self, v: Self) -> Self {
        self.saturating_mul(v)
    }
}
