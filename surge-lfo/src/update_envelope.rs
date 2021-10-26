ix!();

use crate::{
    Lfo,
    LfoParam,
    LfoEnvState,
};

impl Lfo {
    pub fn update_envelope_for_process(&mut self,temposyncratio: f32) {

        let mut envrate: f32 = 0.0;

        match self.env_state {
            LfoEnvState::Delay => {
                envrate = self.tables.envelope_rate_linear(
                    pvalf![self.params[LfoParam::Delay]]
                );
                if self.params[LfoParam::Delay].temposync {
                    envrate *= temposyncratio;
                }
            },
            LfoEnvState::Attack => {
                envrate = self.tables.envelope_rate_linear(
                    pvalf![self.params[LfoParam::Attack]]
                );
                if self.params[LfoParam::Attack].temposync {
                    envrate *= temposyncratio;
                }
            },
            LfoEnvState::Hold => {
                envrate = self.tables.envelope_rate_linear(
                    pvalf![self.params[LfoParam::Hold]]
                );
                if self.params[LfoParam::Hold].temposync {
                    envrate *= temposyncratio;
                }
            },
            LfoEnvState::Decay => {
                envrate = self.tables.envelope_rate_linear(
                    pvalf![self.params[LfoParam::Decay]]
                );
                if self.params[LfoParam::Decay].temposync {
                    envrate *= temposyncratio;
                }
            },
            LfoEnvState::Release => {
                envrate = self.tables.envelope_rate_linear(
                    pvalf![self.params[LfoParam::Release]]
                );
                if self.params[LfoParam::Release].temposync {
                    envrate *= temposyncratio;
                }
            },
            _ => {},
        }

        self.env_phase += envrate;

        let sustainlevel: f32 = 
            pvali![self.params[LfoParam::Sustain]] as f32;

        if self.env_phase > 1.0 {
            match self.env_state {
                LfoEnvState::Delay => {
                    self.env_state = LfoEnvState::Attack;
                    self.env_phase = 0.0;
                },
                LfoEnvState::Attack => {
                    self.env_state = LfoEnvState::Hold;
                    self.env_phase = 0.0;
                },
                LfoEnvState::Hold => {
                    self.env_state = LfoEnvState::Decay;
                    self.env_phase = 0.0;
                },
                LfoEnvState::Decay => {
                    self.env_state = LfoEnvState::Stuck;
                    self.env_phase = 0.0;
                    self.env_val = sustainlevel;
                },
                LfoEnvState::Release => {
                    self.env_state = LfoEnvState::Stuck;
                    self.env_phase = 0.0;
                    self.env_val = 0.0;
                },
                _ => {},
            }
        }

        match self.env_state {
            LfoEnvState::Delay   => self.env_val = 0.0,
            LfoEnvState::Attack  => self.env_val = self.env_phase,
            LfoEnvState::Hold    => self.env_val = 1.0,
            LfoEnvState::Decay   => self.env_val = (1.0 - self.env_phase) + self.env_phase * sustainlevel,
            LfoEnvState::Release => self.env_val = (1.0 - self.env_phase) + self.env_releasestart,
            _ => {},
        }
    }
}
