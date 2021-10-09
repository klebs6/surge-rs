pub use surge_tuning::TunerHandle;
pub use surge_samplerate::SampleRateHandle;
pub use enhanced_enum::enhanced_enum;
pub use float_ord::FloatOrd;
pub use surge_tables::TablesHandle;

pub use surge_constants::{
    N_COEFFMAKER_COEFFS,
    FIR_IPOL_N,
    FIR_IPOL_M,
    FIR_OFFSET,
    MAX_FB_COMB,
};

pub use surge_qfunit::{
    QuadFilterUnitState,
};

pub use surge_math::{
    boundfreq,
    m128_mask_absval,
    limit_range,
    sum_ps_to_ss,
    softclip_ps,
    simd_extract,
};

pub use surge_types::{
    coeffidx,
    FilterTypeIIR,
    PoleType,
    FilterSubType,
};

pub use surge_coeffmaker::{
    FilterCoefficientMaker,
};

pub use surge_filter::{
    CoeffMake,
    FilterProcessQuad,
};

#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::*;

pub use derivative::Derivative;
