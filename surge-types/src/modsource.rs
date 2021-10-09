ix!();

enhanced_enum![
    LfoMode {
        FreeRun,
        KeyTrigger,
        Random,
    }
];

enhanced_enum![
    LfoShape {
        Sine,
        Tri,
        Square,
        Ramp,
        Noise,
        SampleAndHold,
        Envelope,
        StepSequencer,
    }
];

enhanced_enum![
    ModSrcType {
        Undefined,
        Controller,
        Adsr,
        LFO,
        StepSequencer,
    }
];

enhanced_enum![
    EnvelopeMode {
        Digital,
        Analog,
    }
];

#[derive(Clone)]
pub struct PitchBendCfg {
    pub range_up:   f32,
    pub range_down: f32,
}
