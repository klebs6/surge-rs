#[cfg(target_arch = "x86_64")] 
pub use core::arch::x86_64::*;

pub use std::convert::{TryInto,TryFrom};

pub use surge_param::{
    AssocParam,
    pval,
    pvalb,
    pvali,
    pvalf,
    ControlGroup,
    ControlType,
    ValType,
    PData,
    Param,
    ParamRT,
};

pub use enhanced_enum::enhanced_enum;
pub use surge_lipol::LipolPs;
pub use surge_lag::Lag;

pub use surge_tables::TablesHandle;
pub use surge_samplerate::SampleRateHandle;
pub use surge_tuning::TunerHandle;

pub use surge_types::{
    OscillatorParam,
    OscillatorParamArrayRT,
    CharacterMode,
    OscillatorOut,
};

pub use surge_traits::{
    OscillatorStereoOut,
    Named,
    Convolute,
    ConvolutionCfg,
    AllowDisplay,
    Init,
    SetPitch,
    HandleStreamingMismatches,
    AssignFM,
    Oscillator,
    OscillatorProcess,
    OscillatorProcessBlockCfg,
};

pub use surge_macros::{
    rt,
    impl_oscillator_stereo_out,
    allow_display,
    oscillator,
    no_op,
    name,
    surge_base,
};

pub use std::sync::atomic;
pub use std::rc::Rc;
pub use std::cell::RefCell;

pub use surge_math::{
    Align16,
    A1d,
    z128,
    maxf,
    minf,
    maxd,
    drift_noise,
    WetBlock1,
    clear_block,
    rand01,
    mind,
    limit_range,
    limit_range_i,
    rcp,
};

pub use surge_constants::{
    FIR_IPOL_N,
    FIR_OFFSET,
    BLOCK_SIZE_OS,
    BLOCK_SIZE_OS_QUAD,
    OB_LENGTH,
    MAX_UNISON,
};

pub use surge_blitter::{
    AbstractBlitter,
};
