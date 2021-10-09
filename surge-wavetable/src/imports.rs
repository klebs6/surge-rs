pub use enum_dispatch::enum_dispatch;
pub use enhanced_enum::enhanced_enum;
pub use std::fmt::Debug;
pub use std::fs::OpenOptions;
pub use bitflags::bitflags;
pub use core::ffi::c_void;
pub use num_traits::{Zero,SaturatingMul};
pub use std::convert::TryFrom;

pub use surge_math::{
    fasttan,
    A3d,
    i152float_block,
    float2i15_block,
    vt_copyblock_dw_le,
    vt_copyblock_w_le,
    vt_read_int16_le,
    vt_read_int32_le,
    bitscan_reverse,
    NO_PADDING,
    Padding
};

pub use surge_types::{
    WaveTableCategoryID,
    WaveTableDataFilename,
    WaveTableFilename,
    WaveTableID,
    WaveTableWavFilename,
};

pub use surge_traits::{
    WaveTableController,
    Init,
};

pub use surge_constants::{
    FIR_IPOL_I16_N,
    FIR_OFFSET_I16,
    MAX_MIPMAP_LEVELS,
    MAX_SUBTABLES,
    MAX_WAVETABLE_SAMPLES,
    MAX_WAVETABLE_SIZE,
};

pub use std::fs::File;
pub use std::convert::TryInto;
pub use hound;
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
