// hello! could you please help me document and test this code? thanks!

crate::ix!();

impl Lfo {

    #[inline] pub fn calculate_envrate_with_lfo_param(
        &mut self,
        lfo_param:      LfoParam, 
        temposyncratio: f32) -> f32 {

        let mut envrate: f32 = self.tables.envelope_rate_linear(
            pvalf![self.params[lfo_param]]
        );

        if self.params[lfo_param].temposync {
            envrate *= temposyncratio;
        }

        envrate
    }

    #[inline] pub fn next_env_state(&mut self, sustainlevel: f32) {
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

    #[inline] pub fn calculate_envrate(&mut self, temposyncratio: f32) -> f32 {
        match self.env_state {
            LfoEnvState::Delay   => self.calculate_envrate_with_lfo_param(LfoParam::Delay,   temposyncratio),
            LfoEnvState::Attack  => self.calculate_envrate_with_lfo_param(LfoParam::Attack,  temposyncratio),
            LfoEnvState::Hold    => self.calculate_envrate_with_lfo_param(LfoParam::Hold,    temposyncratio),
            LfoEnvState::Decay   => self.calculate_envrate_with_lfo_param(LfoParam::Decay,   temposyncratio),
            LfoEnvState::Release => self.calculate_envrate_with_lfo_param(LfoParam::Release, temposyncratio),
            _ => 0.0,
        }
    }

    #[inline] pub fn update_envelope_value(&mut self, sustainlevel: f32) {

        match self.env_state {
            LfoEnvState::Delay   => self.env_val = 0.0,
            LfoEnvState::Attack  => self.env_val = self.env_phase,
            LfoEnvState::Hold    => self.env_val = 1.0,
            LfoEnvState::Decay   => self.env_val = (1.0 - self.env_phase) + self.env_phase * sustainlevel,
            LfoEnvState::Release => self.env_val = (1.0 - self.env_phase) + self.env_releasestart,
            _ => {},
        }
    }

    pub fn update_envelope_for_process(&mut self,temposyncratio: f32) {

        let envrate: f32 = self.calculate_envrate(temposyncratio);

        self.env_phase += envrate;

        let sustainlevel: f32 = 
            pvali![self.params[LfoParam::Sustain]] as f32;

        if self.env_phase > 1.0 {
            self.next_env_state(sustainlevel);
        }

        self.update_envelope_value(sustainlevel);
    }
}
