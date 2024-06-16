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

pub trait GetAdsrEnvelopeState {

    fn get_envstate(&self) -> AdsrState;
}
