#[cfg(target_arch = "x86_64")] 
pub use core::arch::x86_64::*;

pub use std::convert::TryInto;

pub use surge_wavetable::{
    WaveTableFlags,
    WaveTableProperties,
    WaveTableBase,
};

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
    SetPitch,
};

pub use surge_constants::{
    HPF_CYCLE_LOSS,
    MAX_UNISON,
    FIR_IPOL_N,
    FIR_OFFSET,
    BLOCK_SIZE_OS,
    BLOCK_SIZE_OS_INV,
    BLOCK_SIZE_OS_QUAD,
    OB_LENGTH,
};

pub use surge_types::{
    OscillatorParam,
    OscillatorOut,
    OscillatorParamArrayRT,
};

pub use surge_macros::{
    allow_display,
    default_default,
    impl_oscillator_stereo_out,
    master_osc,
    name,
    no_op,
    oscillator,
    rt,
    surge_base,
    wt_flag,
};

pub use std::sync::atomic;
pub use std::rc::Rc;
pub use std::cell::RefCell;

pub use surge_math::{
    A1d,
    lerp,
    split_float,
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

pub use surge_blitter::{
    AbstractBlitter,
};

