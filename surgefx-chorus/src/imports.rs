#[cfg(target_arch = "x86_64")] 
pub use core::arch::x86_64::*;

pub use std::sync::atomic;
pub use enhanced_enum::enhanced_enum;
pub use std::convert::TryInto;
pub use surge_tables::TablesHandle;
pub use surge_timeunit::TimeUnitHandle;
pub use surge_tuning::TunerHandle;
pub use surge_samplerate::SampleRateHandle;
pub use surge_halfrate::HalfRateFilterSSE;

pub use surge_lipol::{LiPol,LipolPs};

pub use surge_lag::{
    Lag
};

pub use surge_types::{
    NumberOfBlocks,
    Ringout,
    WaveshapeType,
};

pub use surge_traits::{
    HandleStreamingMismatches,
    ProcessRingout,
    ProcessBlock,
    Update,
    Named,
    Init,
    Effect,
    Suspend,
    Process,
    ProcessOnlyControl,
};

pub use surge_constants::{
    RINGOUT_DEFAULT,
    SLOWRATE_M1,
    FIR_IPOL_N,
    FIR_IPOL_M,
};

pub use surge_math::{
    Align16,
    z128,
    A1d,
    A2d,
    WetBlock2,
    TBuffer,
    limit_range,
    sum_ps_to_ss,
    add_block,
    accumulate_block,
    hardclip_block,
    MSBlock,
    encode_mid_side,
    decode_mid_side,
    amp_to_linear,
    copy_block,
};

pub use surge_param::{
    AssocParam,
    pval,
    pvalf,
    pvali,
    ControlGroup,
    ValType,
    ControlType,
    PData,
    Param,
    ParamRT,
};

pub use surge_macros::{
    block_size_quad,
    rt,
    no_op,
    has_timeunit,
    name,
    effect,
    surge_base,
};

pub use surge_biquad::{
    BiquadFilter,
    BiquadCoeffHP,
    BiquadCoeffLP2B,
    ProcessBlockStereo,
    ProcessSample,
    BiquadCalcFreq,
};
