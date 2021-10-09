pub use std::convert::TryInto;
pub use enhanced_enum::enhanced_enum;
pub use surge_constants::BLOCK_SIZE_OS;
pub use surge_input::SynthInputHandle;
pub use surge_tables::TablesHandle;
pub use std::rc::Rc;
pub use std::cell::RefCell;
pub use surge_math::limit_range;

pub use surge_param::{
    AssocParam,
    pval,
    pvali,
    pvalf,
    ControlGroup,
    ValType,
    ControlType,
    PData,
    Param,
    ParamRT,
};

pub use surge_macros::{
    rt,
    no_op,
    name,
    oscillator,
    allow_display,
    surge_base,
    impl_oscillator_stereo_out,
};

pub use surge_types::{
    NumberOfBlocks,
    OscillatorOut,
    OscillatorParam,
    OscillatorParamArrayRT,
    OscillatorParamRT,
};

pub use surge_traits::{
    Oscillator,
    OscillatorStereoOut,
    OscillatorProcess,
    OscillatorProcessBlockCfg,
    ProcessRingout,
    Update,
    Named,
    HandleStreamingMismatches,
    SetPitch,
    AssignFM,
    Init,
    AllowDisplay,
};
