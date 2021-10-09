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

#[derive(Debug)]
pub struct AdsrEnvelope<'sr> {
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
    pub time_unit:     TimeUnitHandle<'sr>,
    pub tables:        TablesHandle<'sr>,
    pub srunit:        SampleRateHandle<'sr>,
    pub enabled:       bool,
}

name![AdsrEnvelope<'sr>, "envelope"];

impl AdsrEnvelope<'sr> {

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
