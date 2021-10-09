#[cfg(target_arch = "x86_64")] 
pub use core::arch::x86_64::*;

pub use surgefx::*;
pub use enum_dispatch::enum_dispatch;
pub use std::iter::Rev;
pub use std::ops::Range;
pub use core::ffi::c_void;
pub use enhanced_enum::enhanced_enum;
pub use regex::Regex;
pub use serde::{Serialize, Deserialize};
pub use std::cell::RefCell;
pub use std::cmp::Ordering;
pub use std::convert::{TryFrom,TryInto};
pub use std::rc::Rc;
pub use std::sync::atomic;
pub use surge_midi::{MIDIUnitHandle,HoldPedalUnitHandle};
pub use surge_mpe::MPEUnitHandle;

pub use surge_adsr::AdsrEnvelope;
pub use surge_biquad::{
    ProcessBlockStereo,
    BiquadFilter,
    BiquadCoeffHP,
    BiquadCalcFreq,
};
pub use surge_coeffmaker::FilterCoefficientMaker;
pub use surge_halfrate::HalfRateFilterSSE;
pub use surge_input::SynthInputHandle;
pub use surge_lipol::LipolPs;
pub use surge_samplerate::SampleRateHandle;
pub use surge_tables::TablesHandle;
pub use surge_timeunit::TimeUnitHandle;
pub use surgeosc_audioin::AudioInputOscillator;
pub use surgeosc_fm2::FM2Oscillator;
pub use surgeosc_fm::FMOscillator;
pub use surgeosc_sine::SineWaveOscillator;
pub use surgeosc_snh::SampleAndHoldOscillator;
pub use surgeosc_super::SurgeSuperOscillator;
pub use surgeosc_wavetable::WTOscillator;
pub use surgeosc_window::WindowOscillator;
pub use uuid::Uuid;

pub use surge_constants::{ 
    MAX_VOICES,
    BLOCK_SIZE, 
    BLOCK_SIZE_OS, 
    BLOCK_SIZE_OS_INV, 
    BLOCK_SIZE_OS_QUAD, 
    FIR_IPOL_N, 
    MAX_FB_COMB, 
    N_COEFFMAKER_COEFFS, 
    N_CUSTOMCONTROLLERS, 
    N_FILTER_REGISTERS, 
    N_GLOBAL_PARAMS, 
    N_LFOS, 
    N_LFOS_PER_SCENE,
    N_OSCS, 
    N_OSC_PARAMS, 
    N_SCENE_PARAMS, 
    ONE_TWELFTH, 
};

pub use surge_filter::{ 
    CoeffMake, 
};

pub use surge_lfo::{
    StepSequencer,
    Lfo
};

pub use surge_macros::{ 
    name, 
    compare_by, 
    rt, 
    default_default, 
}; 

pub use surge_math::{ 
    WetBlock1Dual,
    hardclip_block8,
    limit_range,
    amp_to_linear,
    megapan_left,
    megapan_right,
    set1f,
    accumulate_block,
    correlated_noise_o2mk2,
    mul_block,
    WetBlock1,
    lerp,
    clear_block,
    add_block,
    minf,
    get1f,
};

pub use surge_midi::{
    MidiKeyState, 
    MidiChannelState
};

pub use surge_modulation::{
    ControllerModulationSource,
    MaybeBoxedModulationSource,
    ModSource,
    ModSourceArray,
    ModulationRouting,
    ModulationSource,
    ModulationSourceArray,
}; 

pub use surge_param::{ 
    GetSetModulation,
    AssocParam, 
    ControlGroup, 
    ControlType, 
    PData, 
    Param, 
    ParamRT, 
    ValType, 
    pval, 
    pvalb, 
    pvalf, 
    pvali, 
    pvalmin, 
    pvalminf, 
};

pub use surge_qfunit::{ 
    FbqGlobal,
    get_fn_process_quad,
    WaveshaperState,
    get_quad_filter_ptr,
    get_quad_filter_waveshaper_ptr,
    QuadFilterChain,
    QuadFilterChainState, 
    WaveshaperParam, 
    WaveshaperUnit,
};

pub use surge_traits::{ 
    Reset,
    Process,
    ProcessOnlyControl,
    GetRingout,
    SetRingout,
    ProcessRingout,
    Suspend,
    Update,
    ClearBuffers,
    GetReturnLevel,
    ProcessBlockD2,
    Effect, 
    Init, 
    MaybeEffect, 
    MaybeEffects, 
    ModulationSourceControl, 
    Named, 
    Oscillator, 
    OscillatorProcessBlockCfg, 
    SaveInto, 
};

pub use surge_tuning::{
    SurgeTuner,
    TunerHandle
};

pub use surge_types::{ 
    FilterUnit,
    FilterParam,
    PitchBendCfg,
    NumberOfBlocks,
    OutputDataPresent,
    Ringout,
    WaveshapeType,
    FilterBlockConfiguration, 
    FilterSubType, 
    FilterType, 
    FmConfiguration, 
    MappingData, 
    ModSrcType, 
    MpeEnableSwitch, 
    OscillatorParam, 
    OscillatorType, 
    PatchDataSize, 
    PitchBendRange, 
    PolyMode, 
    ShouldKeepPlaying, 
    TuningData, 
};

pub use surge_voice::{
    VoiceRuntime,
    VoiceUpdateQFCSCfg,
    VoiceToggleSoloCfg,
    SurgeVoice,
    BoxedModRoutingIter,
    VoiceConstructor,
    SyncQFBRegistersCfg,
    VoiceRuntimeHandle,
};

pub use std::pin::Pin;

pub use surgefx::*;
