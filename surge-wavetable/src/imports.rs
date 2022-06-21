pub(crate) use surge_math::*;
pub(crate) use surge_types::*;
pub(crate) use surge_constants::*;
pub(crate) use surge_hound as hound;
pub(crate) use surge_imports::*;

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
