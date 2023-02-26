crate::ix!();

impl Lfo {

    /// Updates the envelope state based on the
    /// given delay, attack and hold parameters.
    ///
    /// If the delay parameter is equal to the
    /// current min delay parameter, the envelope
    /// state is set to Attack. 
    ///
    /// If the attack parameter is equal to the
    /// current min attack parameter, the envelope
    /// state is set to Hold, and the envelope
    /// value is set to
    /// 1.0. 
    ///
    /// If the hold parameter is equal to the
    /// current min hold parameter, the envelope
    /// state is set to Decay.
    ///
    /// # Arguments
    ///
    /// * `delay` - The delay parameter.
    /// * `attack` - The attack parameter.
    /// * `hold` - The hold parameter.
    ///
    pub fn maybe_update_env_state_for_attack(
        &mut self, 
        delay: f32, 
        attack: f32, 
        hold: f32) 
    {
        if (delay - pvalminf![self.params[LfoParam::Delay]]).abs() < f32::EPSILON {

            self.env_state = LfoEnvState::Attack;

            if (attack - pvalminf![self.params[LfoParam::Attack]]).abs() < f32::EPSILON
            {
                self.env_state = LfoEnvState::Hold;
                self.env_val   = 1.0;

                if (hold - pvalminf![self.params[LfoParam::Hold]]).abs() < f32::EPSILON  
                {
                    self.env_state = LfoEnvState::Decay;
                }
            }
        }
    }

    /// Updates the phase and step for the LFO in
    /// keytrigger mode.
    ///
    /// # Arguments
    ///
    /// * `phaseslider` - The phase slider value.
    ///
    pub fn update_phase_and_step_for_lfo_mode_keytrigger(&mut self, phaseslider: f32) {
        self.phase = phaseslider;
        self.step = 0;
    }

    /// Updates the phase and step for the LFO in
    /// random mode.
    ///
    pub fn update_phase_and_step_for_lfo_mode_random(&mut self) {

        self.phase = rand01();

        self.step = {
            let randi = rand01() as usize;
            let mask  = N_STEPSEQUENCER_STEPS - 1;
            let step  = randi % self.stepsequencer.loop_end();

            step & mask
        };
    }

    /// Updates the phase and step for the LFO in
    /// free run mode.
    ///
    /// # Arguments
    ///
    /// * `phaseslider` - The phase slider value.
    /// * `rate` - The rate parameter.
    ///
    pub fn update_phase_and_step_for_lfo_mode_free_run(&mut self, 
        phaseslider: f32, 
        rate:        f32) 
    {
        let x: f32 =  {
            let songpos = self.time_unit.songpos() as f32;
            let ratemod = 2.0_f32.powf( rate );

            phaseslider + 0.5 * songpos * ratemod
        };

        let (integral_part, _fractional_part) = split_float(x);

        let loop_start = self.stepsequencer.loop_start();
        let loop_end   = self.stepsequencer.loop_end();

        self.step = {

            let ipart               = integral_part as usize;
            let stepsequencer_delta = loop_end - loop_start;
            let delta_clamped       = std::cmp::max( 1, stepsequencer_delta);
            let offset              = loop_start;

            (ipart % delta_clamped ) + offset
        };
    }

    /// Resets the phase and step of the LFO to
    /// zero.
    ///
    pub fn rezero_phase_and_step(&mut self) {
        self.step  = 0;
        self.phase = 0.0;
    }

    /// Updates the phase and step of the LFO
    /// based on the given parameters and mode.
    ///
    /// # Arguments
    ///
    /// * `lfo_shape` - The shape of the LFO.
    /// * `lfo_mode` - The mode of the LFO.
    /// * `start_phase` - The starting phase value.
    /// * `rate` - The rate parameter.
    pub fn maybe_update_phase_and_step_for_attack(
        &mut self,
        lfo_shape:   LfoShape,
        lfo_mode:    LfoMode,
        start_phase: f32,
        rate:        f32)
    {
        let phaseslider = self.get_phaseslider(lfo_shape, start_phase);

        match lfo_mode {

            LfoMode::KeyTrigger => 
                self.update_phase_and_step_for_lfo_mode_keytrigger(phaseslider),

            LfoMode::Random =>
                self.update_phase_and_step_for_lfo_mode_random(),

            LfoMode::FreeRun =>
                self.update_phase_and_step_for_lfo_mode_free_run(phaseslider, rate),

        }
    }
}
