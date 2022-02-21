ix!();

use crate::{
    AdsrParamArrayRT,
};

enhanced_enum![
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

/**
  | Thus, we have a mystic who is also a highly
  | ordered thinker.
  |
  | and yet -- all of his thinking is used, not to
  | prove the correctness of his mind, but to prove
  | the sublimity of the divine plan.
  |
  | In this, we have the use of mind, the use of
  | words.
  |
  | and plotinus is said to have weeped over the
  | sorrow of words:
  |
  | how little words can tell?
  |
  | how poor they are in the search for terms
  | suitable to clear the human soul?
  |
  | and perhaps there is no one in antiquity who used
  | words more lovingly, more beautifully, and more
  | exactly than this man -- who would have been an
  | inspiration to the best part of modern semntic
  | thinking.
  |
  | -Manly Hall (on Plotinus)
  |
  */
#[derive(Debug)]
pub struct AdsrEnvelope {
    pub params:        AdsrParamArrayRT,
    pub output:        f32,
    pub phase:         f32,
    pub sustain:       f32,
    pub scalestage:    f32,
    pub idlecount:     i32,
    pub envstate:      AdsrState,
    pub _v_c1:         f32,
    pub _v_c1_delayed: f32,
    pub _discharge:    f32,
    pub time_unit:     TimeUnitHandle,
    pub tables:        TablesHandle,
    pub srunit:        SampleRateHandle,
    pub enabled:       bool,
}

name![AdsrEnvelope, "envelope"];

impl AdsrEnvelope {

    pub fn get_env_state(&self) -> AdsrState { 
        self.envstate
    }

    pub fn uber_release(&mut self) 
    {
        //note, there was some other commented logic here before the port
        self.scalestage = self.output;
        self.phase = 1.0;
        self.envstate = AdsrState::UberRelease;
    }

    pub fn is_idle(&self) -> bool 
    {
        self.envstate == AdsrState::Idle && self.idlecount > 0
    }

    pub fn retrigger(&mut self) {
        if self.envstate < AdsrState::Release {
            self.attack();
        }
    }
}
