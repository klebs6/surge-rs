#[cfg(target_arch = "x86_64")] 
pub use core::arch::x86_64::*;

pub use std::marker::PhantomData;
pub use std::cell::RefMut;
pub use std::ops::{Deref,DerefMut};
pub use ndarray::*;
pub use enum_dispatch::enum_dispatch;
pub use std::io::{Error,ErrorKind};
pub use byteorder::{ByteOrder,BigEndian,LittleEndian};
pub use std::fs::File;
pub use serde::{Serialize, Deserialize};
pub use mopa::mopafy;
pub use std::sync::atomic;
pub use paste::paste;
pub use std::any::Any;
pub use downcast_rs::Downcast;
pub use std::cell::RefCell;
pub use std::rc::Rc;
pub use proc_macro::*;
pub use atomic_float::{AtomicF64,AtomicF32};
pub use coreaudio::audio_unit::render_callback::{self, data};
pub use coreaudio::audio_unit::{AudioUnit, IOType, SampleFormat};
pub use coreaudio::audio_unit::*;
pub use derive_more::{DerefMut,Deref};
pub use enhanced_enum::enhanced_enum;
pub use float_ord::*;
pub use getset::{Getters,Setters,MutGetters,CopyGetters};
pub use half::f16;
pub use indoc::{formatdoc,indoc};
pub use lockfree::map::Map as LFMap;
pub use num::complex::Complex64;
pub use num::Float;
pub use num_traits::{Num,Zero};
pub use std::cmp::Ordering;
pub use std::collections::{BTreeMap,BTreeSet};
pub use std::convert::TryFrom;
pub use std::convert::TryInto;
pub use std::f32::consts::PI as PI_32;
pub use std::f64::consts::PI as PI;
pub use std::ffi::c_void;
pub use std::ffi;
pub use std::fmt::Debug;
pub use std::fmt;
pub use std::io::{BufReader,BufWriter,Write, Read, BufRead};
pub use std::ops::{AddAssign};
pub use std::path::Path;
pub use std::sync::atomic::{AtomicI32,AtomicI16};
pub use std::sync::{Arc, Mutex};
pub use std::thread;
pub use std::time::Duration;
pub use std::time;
pub use vst::api::{Supported, Events};
pub use vst::buffer::AudioBuffer;
pub use vst::event::Event;
pub use vst::host::{Host, PluginLoader, PluginInstance};
pub use vst::plugin::{Category, Plugin, Info, CanDo};

pub use surge_adsr::*;
pub use surge_biquad::*;
pub use surge_blitter::*;
pub use surge_coeffmaker::*;

pub use surge_traits::{
    SaveInto,
    Init,
    MaybeEffects,
    ModulationSourceControl,
    ProcessBlock,
    ProcessBlockU2,
    ProcessBlockD2,
    Reset,
    Suspend,
    WaveTableController,
};

pub use surge_types::{
    FilterBlockConfiguration,
    FilterSubType,
    FilterType,
    FxBypassType,
    MpeEnableSwitch,
    NumVoices,
    OscillatorType,
    PatchDataSize,
    PitchBendRange,
    PitchBendValue,
    PolyMode,
    SceneMode,
    ShouldKeepPlaying,
    WaveshapeType,
};

pub use surge_constants::{
    BLOCK_SIZE,
    BLOCK_SIZE_OS,
    BLOCK_SIZE_QUAD,
    BLOCK_SIZE_OS_QUAD,
    MAX_MIPMAP_LEVELS,
    MAX_SUBTABLES,
    MAX_VOICES,
    METAPARAM_OFFSET,
    NUM_CONTROLINTERPOLATORS,
    N_CUSTOMCONTROLLERS,
    N_GLOBAL_PARAMS,
    N_INPUTS,
    N_LFOS_PER_SCENE,
    N_OSCS,
    N_OUTPUTS,
    N_SCENES,
    N_SCENE_PARAMS,
    N_TOTAL_PARAMS,
};

pub use surge_filter::*;
pub use surge_halfrate::*;
pub use surge_input::*;
pub use surge_lag::*;
pub use surge_lfo::*;
pub use surge_lipol::*;
pub use surge_math::*;
pub use surge_midi::*;
pub use surge_modulation::{
    ModSource,
    ModSourceArray,
    ModulationSource,
    ModulationRouting,
    ControllerModulationSource,
};

pub use surge_mpe::*;
pub use surge_types::*;
pub use surge_traits::*;
pub use surge_macros::*;
pub use surge_constants::*;
pub use surge_output::*;
pub use surge_param::*;
pub use surge_scene::*;
pub use surge_qfunit::*;
pub use surge_quadrosc::*;
pub use surge_samplerate::*;
pub use surge_svf::*;
pub use surge_tables::*;
pub use surge_timeunit::*;
pub use surge_tuning::*;
pub use surge_wavetable::*;
pub use surgefilter_comb::*;
pub use surgefilter_diode::*;
pub use surgefilter_iir::*;
pub use surgefilter_k35::*;
pub use surgefilter_moog::*;
pub use surgefilter_nlfeedback::*;
pub use surgefilter_nlstates::*;
pub use surgefilter_obxd::*;
pub use surgefilter_snh::*;
pub use surgefilter_svf::*;
pub use surgefilter_huovilainen::*;
pub use surgefilter_rungekutta::*;
pub use surgefx::SurgeEffect;
pub use surgefx_allpass::*;
pub use surgefx_chorus::*;
pub use surgefx_conditioner::*;
pub use surgefx_distortion::*;
pub use surgefx_dualdelay::*;
pub use surgefx_emphasize::*;
pub use surgefx_eq3band::*;
pub use surgefx_flanger::*;
pub use surgefx_freqshift::*;
pub use surgefx_phaser::*;
pub use surgefx_reverb::*;
pub use surgefx_ringmod::*;
pub use surgefx_rotary::*;
pub use surgefx_vocoder::*;
pub use surgeosc_audioin::*;
pub use surgeosc_fm::*;
pub use surgeosc_fm2::*;
pub use surgeosc_sine::*;
pub use surgeosc_snh::*;
pub use surgeosc_super::*;
pub use surgeosc_wavetable::*;
pub use surgeosc_window::*;
pub use surgeshaper_asym::*;
pub use surgeshaper_clip::*;
pub use surgeshaper_digi::*;
pub use surgeshaper_sine::*;
pub use surgeshaper_tanh::*;

