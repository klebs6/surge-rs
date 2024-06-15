crate::ix!();

/// defines the various states the ADSR envelope
/// can be in
///
#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum AdsrState {
    Attack,
    Decay,
    Sustain,
    Release,
    UberRelease,
    IdleWait1,
    Idle,
}

impl GetState for AdsrEnvelope {
    type State = AdsrState;

    /// `AdsrEnvelope::get_env_state`: This
    /// function returns the current state of the
    /// envelope.
    ///
    fn get_env_state(&self) -> Self::State { 
        self.envstate
    }
}
