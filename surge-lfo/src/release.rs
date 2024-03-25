crate::ix!();

impl Lfo {

    /// The release function sets the LFO envelope state to release mode, triggering
    /// a linear decay of the output signal. The amount of time it takes for the
    /// release phase to complete is determined by the `Release` parameter of the
    /// LFO.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use your_crate::Lfo;
    /// let mut lfo = Lfo::new();
    /// lfo.process();
    /// lfo.release();
    /// lfo.process(); // output signal starts to decay
    /// ```
    ///
    pub fn release(&mut self) {

        // Extract the `Release` parameter from the `params` field of the `Lfo`.
        //
        let release_f = pvalf![
            self.params[LfoParam::Release]
        ];

        // Extract the maximum value for `Release` from the `params` field of the `Lfo`.
        //
        let release_f_max = pvalmaxf![
            self.params[LfoParam::Release]
        ];

        // If the current value of `Release` is less than its maximum value, set the `env_state`
        // field of the `Lfo` to `LfoEnvState::Release`, and initialize the `env_releasestart` and
        // `env_phase` fields to start the release phase.
        //
        if release_f < release_f_max {
            self.env_state = LfoEnvState::Release;
            self.env_releasestart = self.env_val;
            self.env_phase = 0.0;
        }
    }

    pub fn reset(&mut self) {
        todo!();
    }
}
