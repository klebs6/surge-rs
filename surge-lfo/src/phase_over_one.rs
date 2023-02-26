crate::ix!();

impl Lfo {

    /// This function updates the waveform history buffer for the LFO's correlated noise waveform. 
    ///
    /// It shifts the history buffer by one position to the right and calculates a new value for
    /// the first position using the `correlated_noise_o2mk2()` function with the current LFO
    /// target, noise seed value, and Deform parameter.
    ///
    pub fn update_noise_for_phase_over_one(&mut self) {

        self.wf_history[3] = self.wf_history[2];
        self.wf_history[2] = self.wf_history[1];
        self.wf_history[1] = self.wf_history[0];
        self.wf_history[0] = correlated_noise_o2mk2(self.target, self.noised1, 

            pvalf![self.params[LfoParam::Deform]]
        );
    }

    /// This function updates the value of the LFO's output for the Sample and Hold waveform shape. 
    ///
    /// It calculates a new value for the output using the `correlated_noise_o2mk2()` function with
    /// the current LFO target, noise seed value, and Deform parameter.
    ///
    pub fn update_sample_and_hold_for_phase_over_one(&mut self) {

        self.iout = correlated_noise_o2mk2(self.target, self.noised1, 
            pvalf![self.params[LfoParam::Deform]]
        );
    }

    /// This function updates the state of the LFO's Step Sequencer waveform shape. 
    ///
    /// It checks the LFO's `stepsequencer.trigmask` for triggers and sets the appropriate flags
    /// for re-triggering the LFO's Attack/Decay/Release envelopes. 
    ///
    /// It updates the LFO's `step` and `shuffle_id` values, recalculates the `ratemult` value
    /// based on the current `shuffle_id` and `StartPhase` parameter, and updates the waveform
    /// history buffer for the Step Sequencer.
    ///
    pub fn update_step_sequencer_for_phase_over_one(&mut self) {

        /*
        | You might thing we don't need
        | this and technically we don't but
        | I wanted to keep it here to retain
        | compatability with versions of
        | trigmask which were streamed in
        | older sessions
        */
        if (self.stepsequencer.trigmask() & (1_u64 << self.step)) != 0 {
            self.retrigger_feg = true;
            self.retrigger_aeg = true;
        }

        if (self.stepsequencer.trigmask() & (1_u64 << (16 + self.step))) != 0 {
            self.retrigger_feg = true;
        }

        if (self.stepsequencer.trigmask() & (1_u64 << (32 + self.step))) != 0 {
            self.retrigger_aeg = true;
        }

        self.step += 1;
        self.shuffle_id = (self.shuffle_id + 1) & 1;

        if self.shuffle_id != 0 {
            self.ratemult = 1.0 / minf(
                0.01, 
                1.0 - 0.5 * pvalf![self.params[LfoParam::StartPhase]]
            );

        } else {
            self.ratemult = 1.0 / (1.0 + 0.5 * 
                pvalf![self.params[LfoParam::StartPhase]]
            );
        }

        if self.step > self.stepsequencer.loop_end() {
            self.step = self.stepsequencer.loop_start();
        }

        self.wf_history[3] = self.wf_history[2];
        self.wf_history[2] = self.wf_history[1];
        self.wf_history[1] = self.wf_history[0];
        self.wf_history[0] = self.stepsequencer.step(self.step & (N_STEPSEQUENCER_STEPS - 1));
    }

    /// This function updates the LFO's state for the given waveform shape. 
    ///
    /// It decrements the LFO's `phase` value by one and calls the appropriate waveform
    /// shape-specific function (`update_noise_for_phase_over_one()`,
    /// `update_sample_and_hold_for_phase_over_one()`, or
    /// `update_step_sequencer_for_phase_over_one()`) based on the given `shape` parameter. 
    ///
    /// If the `shape` parameter is not one of the supported shapes, this function does nothing.
    ///
    pub fn update_for_phase_over_one(&mut self, shape: LfoShape) {

        self.phase -= 1.0;

        match shape {

            LfoShape::SampleAndHold =>
                self.update_sample_and_hold_for_phase_over_one(),

            LfoShape::Noise => 
                self.update_noise_for_phase_over_one(),

            LfoShape::StepSequencer =>
                self.update_step_sequencer_for_phase_over_one(),

            _ => {}
        }
    }
}
