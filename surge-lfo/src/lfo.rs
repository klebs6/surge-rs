crate::ix!();

enhanced_enum![

    // `LfoEnvState` is an enumeration that defines
    // the different states that the envelope of the
    // Low-Frequency Oscillator (LFO) module can be
    // in. Each state corresponds to a different
    // phase of the LFO envelope, which describes how
    // the LFO output changes over time.
    // 
    // The possible states are:
    // 
    // - `Off`: The LFO is not currently active.
    //
    // - `Delay`: The LFO is in the delay phase,
    // where the output is held at zero for
    // a specified amount of time before the attack
    // phase begins.
    //
    // - `Attack`: The LFO is in the attack phase,
    // where the output ramps up to its maximum
    // value.
    //
    // - `Hold`: The LFO is in the hold phase, where
    // the output is held at its maximum value for
    // a specified amount of time before the decay
    // phase begins.
    //
    // - `Decay`: The LFO is in the decay phase,
    // where the output ramps down from its maximum
    // value to its sustain level.
    //
    // - `Release`: The LFO is in the release phase,
    // where the output ramps down from its sustain
    // level to zero.
    //
    // - `Stuck`: The LFO is stuck at its current
    // value due to some error or issue.
    // 
    // Each state represents a different behavior of
    // the LFO envelope, and the LFO module
    // transitions between states as it is triggered
    // and released.
    //
    LfoEnvState {
        Off,
        Delay,
        Attack,
        Hold,
        Decay,
        Release,
        Stuck,
    }
];

/// The Lfo struct defines the behavior of an LFO
/// (Low-Frequency Oscillator) module.
///
#[derive(Debug)]
#[name("lfo")]
#[modulation_source_control]
pub struct Lfo {

    /// The params field contains runtime
    /// parameters for the LFO.
    ///
    pub params:             LfoParamArray::<ParamRT::<LfoParam>>,

    /// The output field contains the current
    /// output value of the LFO.
    ///
    pub output:             f64,

    /// The stepsequencer field contains
    /// a StepSequencer object used for LFO
    /// modulation.
    ///
    pub stepsequencer:      StepSequencer,

    /// The phase_initialized field indicates
    /// whether the LFO phase has been
    /// initialized.
    ///
    pub phase_initialized:  bool,

    /// The env_val field contains the current
    /// value of the LFO envelope.
    ///
    pub env_val:            f32,

    /// The env_state field contains the current
    /// state of the LFO envelope.
    ///
    pub env_state:          LfoEnvState,

    /// The retrigger_feg field indicates whether
    /// the LFO's filter envelope generator should
    /// be retrigged.
    ///
    pub retrigger_feg:      bool,

    /// The retrigger_aeg field indicates whether
    /// the LFO's amplitude envelope generator
    /// should be retrigged.
    ///
    pub retrigger_aeg:      bool,

    /// The phase field contains the current phase
    /// of the LFO waveform.
    ///
    pub phase:              f32,

    /// The target field contains the target value
    /// of the LFO waveform.
    ///
    pub target:             f32,

    /// The noise field contains the current value
    /// of the LFO noise generator.
    ///
    pub noise:              f32,

    /// The noised1 field contains the previous
    /// value of the LFO noise generator.
    ///
    pub noised1:            f32,

    /// The env_phase field contains the current
    /// phase of the LFO envelope.
    ///
    pub env_phase:          f32,

    /// The ratemult field contains a multiplier
    /// for the LFO rate.
    ///
    pub ratemult:           f32,

    /// The env_releasestart field contains the
    /// release start time for the LFO envelope.
    ///
    pub env_releasestart:   f32,

    /// The iout field contains the current output
    /// index for the LFO step sequencer.
    ///
    pub iout:               f32,

    /// The wf_history field contains a history
    /// buffer for the LFO waveform.
    ///
    pub wf_history:         [f32; 4],

    /// The step field contains the current step
    /// index for the LFO step sequencer.
    ///
    pub step:               usize,

    /// The shuffle_id field contains the current
    /// shuffle ID for the LFO step sequencer.
    ///
    pub shuffle_id:         isize,

    /// The sine field contains a QuadrOsc object
    /// used for LFO sine wave generation.
    ///
    pub sine:               QuadrOsc,

    /// The time_unit field contains a handle to
    /// the TimeUnit object used for LFO timing.
    ///
    pub time_unit:          TimeUnitHandle,

    /// used for LFO table lookups.
    ///
    pub tables:             TablesHandle,

    /// The enabled field indicates whether the
    /// LFO is currently enabled.
    ///
    pub enabled:            bool,
}

impl_trait_defaults!{
    Lfo;
    Release,
    Attack
}
