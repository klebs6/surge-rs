#[cfg(target_arch = "x86_64")] 
pub use core::arch::x86_64::*;

pub use core::f32::consts::PI as PI_32;

pub use std::convert::TryInto;

pub use surge_wavetable::{
    WaveTableFlags,
    WaveTableProperties,
    WaveTable,
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
    Mask32,
    OscillatorOut,
    OscillatorParam,
    OscillatorParamArrayRT,
};

pub use surge_macros::{
    rt,
    wt_flag,
    master_osc,
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

pub use num_traits::Zero;

pub use surge_math::{
    Align16,
    A1d,
    A2d,
    maxi,
    big_mul_r16,
    bitscan_reverse,
    lerp,
    megapan_left,
    megapan_right,
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

pub use ndarray::Axis;
