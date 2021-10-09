#[cfg(target_arch = "x86_64")] 
pub use core::arch::x86_64::*;

pub use enhanced_enum::enhanced_enum;
pub use std::convert::TryInto;

pub use core::f64::consts::PI;

pub use surge_samplerate::SampleRateHandle;

pub use surge_constants::{
    BLOCK_SIZE_OS,
    BLOCK_SIZE_OS_QUAD,
    FIR_IPOL_N,
    MAX_UNISON,
    OB_LENGTH,
};

pub use std::rc::Rc;
pub use std::cell::RefCell;
pub use surge_tables::TablesHandle;
pub use surge_tuning::TunerHandle;

pub use surge_lipol::{
    LipolPs,
};

pub use surge_math::{
    A1d,
    limit_range_i,
    limit_range,
    WetBlock1,
    maxd,
    drift_noise,
    z128,
    clear_block,
    rand11,
    rand01,
    rcp,
    maxf,
    minf,
};

pub use surge_quadrosc::QuadrOsc;
pub use surge_lag::Lag;

pub use surge_param::{
    AssocParam,
    pval,
    pvalb,
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
    default_default,
    rt,
    no_op,
    name,
    oscillator,
    allow_display,
    surge_base,
    impl_oscillator_stereo_out,
};

pub use surge_blitter::AbstractBlitter;

pub use surge_traits::{
    AllowDisplay,
    AssignFM,
    Convolute,
    ConvolutionCfg,
    HandleStreamingMismatches,
    Init,
    Named,
    Oscillator,
    OscillatorProcess,
    OscillatorProcessBlockCfg,
    OscillatorStereoOut,
    ProcessRingout,
    SetPitch,
    Update,
};

pub use surge_types::{
    OscillatorParamArrayRT,
    OscillatorOut,
    OscillatorParam,
    NumberOfBlocks,
};
