ix!();

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

enhanced_enum![
    OscillatorParam {
        Type,
        Pitch,
        Octave,
        KeyTrack,
        Retrigger,
    }
];

rt![OscillatorParam];

impl OscillatorParam {
    pub fn runtime_array() -> OscillatorParamArrayRT {
        OscillatorParamArrayRT::new_with(|x| match x {
            OscillatorParam::Type      => OscillatorParamRT::new(OscillatorParam::Type),
            OscillatorParam::Pitch     => OscillatorParamRT::new(OscillatorParam::Pitch),
            OscillatorParam::Octave    => OscillatorParamRT::new(OscillatorParam::Octave),
            OscillatorParam::KeyTrack  => OscillatorParamRT::new(OscillatorParam::KeyTrack),
            OscillatorParam::Retrigger => OscillatorParamRT::new(OscillatorParam::Retrigger),
        })
    }
}

impl Param for OscillatorParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 
    /* TODO */
}



pub type OscillatorOut = WetBlock2::<BLOCK_SIZE_OS>;
