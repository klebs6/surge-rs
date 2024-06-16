crate::ix!();

enhanced_enum![OscRoute {
    Filter1,
    Both,
    Filter2,
}];


enhanced_enum![OscillatorType {
    Off,
    AbstractBlit,
    AudioInput,
    SurgeSuperOscillator,
    FM,
    FM2,
    SampleAndHold,
    Sine,
    Wavetable,
    Window,
}];

impl OscillatorType {
    pub fn uses_wavetabledata(&self) -> bool {
        matches![&self,
        OscillatorType::Wavetable 
            | OscillatorType::Window]
    }
}

enhanced_enum![
    FmConfiguration{
        Off,
        OneToZero,
        TwoToOneToZero,
        OneAndTwoToZero,
    }
];

impl FmConfiguration {
    pub fn on(&self) -> bool {
        !matches![self,FmConfiguration::Off]
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum OscillatorParam {
    Type,
    Pitch,
    Octave,
    KeyTrack,
    Retrigger,
}

/* TODO: possibly, some of these need to be non-default */
impl_trait_defaults!{
    OscillatorParam;
    CheckIfAffectsOtherParameters,
    CheckIfCanBeAbsolute,
    CheckIfCanExtendRange,
    CheckIfCanSnap,
    CheckIfCanTemposync,
    CheckIfModulateable,
    GetControlStyle,
    GetDefaultValueF01,
    GetExtendedValue,
    GetParameterValueType,
    GetMaxParameterValue,
    GetMinParameterValue,
    GetDefaultParameterValue,
    GetControlType,
    GetModulation,
    GetExtendRange,
    GetMoverate,
    GetSnap,
    SetModulation,
}

impl GetControlGroup for OscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 
}

pub type OscillatorOut = WetBlock2::<BLOCK_SIZE_OS>;
