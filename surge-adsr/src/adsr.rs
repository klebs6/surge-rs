crate::ix!();

enhanced_enum![

    // defines the various states the ADSR envelope
    // can be in
    //
    AdsrState {
        Attack,
        Decay,
        Sustain,
        Release,
        UberRelease,
        IdleWait1,
        Idle,
    }
];

/// The ADSR envelope describes the change of
/// a sound's amplitude over time. 
///
/// This is a commonly used component in
/// electronic music synthesis. 
///
/// It contains various fields that store
/// information about the envelope's state,
///
#[derive(Debug)]
pub struct AdsrEnvelope {

    /// an array of ADSR parameter values
    ///
    pub params:        AdsrParamArrayRT,

    /// the current output of the envelope
    ///
    pub output:        f32,

    /// the current phase of the envelope
    ///
    pub phase:         f32,

    /// the sustain level of the envelope
    ///
    pub sustain:       f32,

    /// the current scale stage of the envelope
    ///
    pub scalestage:    f32,

    /// the current idle count of the envelope
    ///
    pub idlecount:     i32,

    /// the current state of the envelope
    ///
    pub envstate:      AdsrState,

    /// internal state variables used in the
    /// analog envelope processing
    pub _v_c1:         f32,
    pub _v_c1_delayed: f32,
    pub _discharge:    f32,

    /// a handle to a time unit object
    ///
    pub time_unit:     TimeUnitHandle,

    /// a handle to a tables object
    ///
    pub tables:        TablesHandle,

    /// a handle to a sample rate object
    ///
    pub srunit:        SampleRateHandle,

    /// indicates whether the envelope is enabled
    ///
    pub enabled:       bool
}

name![AdsrEnvelope, "envelope"];

impl AdsrEnvelope {

    /// `AdsrEnvelope::get_env_state`: This
    /// function returns the current state of the
    /// envelope.
    ///
    pub fn get_env_state(&self) -> AdsrState { 
        self.envstate
    }

    /// `AdsrEnvelope::uber_release`: This
    /// function puts the envelope into an "Uber
    /// Release" state, which sets the
    /// `scalestage` and `phase` fields and
    /// changes the `envstate` to
    /// `AdsrState::UberRelease`.
    ///
    pub fn uber_release(&mut self) 
    {
        //note, there was some other commented
        //logic here before the port
        self.scalestage = self.output;
        self.phase      = 1.0;
        self.envstate   = AdsrState::UberRelease;
    }

    /// `AdsrEnvelope::is_idle`: This function
    /// returns `true` if the envelope is
    /// currently in an "Idle" state and the
    /// `idlecount` is greater than 0, and `false`
    /// otherwise.
    ///
    pub fn is_idle(&self) -> bool 
    {
        self.envstate == AdsrState::Idle && self.idlecount > 0
    }

    /// `AdsrEnvelope::retrigger`: This function
    /// retriggers the envelope by setting it to
    /// the "Attack" state if it is currently in
    /// a state less than "Release".
    ///
    pub fn retrigger(&mut self) {

        if self.envstate < AdsrState::Release {
            self.attack();
        }
    }
}
