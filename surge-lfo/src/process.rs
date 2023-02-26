crate::ix!();

/// The `LfoProcess` trait provides a `process` method that is implemented for the `Lfo`
/// struct. This method calculates the output of the LFO for the current processing cycle based on
/// its parameters and state.
///
impl LfoProcess for Lfo {


    /// Process a single cycle of the LFO.
    ///
    /// This method calculates the output of the LFO for the current processing cycle based on its
    /// parameters and state. If the phase ofk the LFO is uninitialized, it will be initialized
    /// based on the start phase parameter. The processing algorithm is as follows:
    ///
    fn process(&mut self) {

        if ! self.phase_initialized {

            // Initialize phase if it is uninitialized.
            self.init_phase_from_start_phase();
        }

        // Reset retrigger counters.
        self.zero_retriggers();

        // Determine LFO shape.
        let  shape = self.get_shape();

        // Calculate tempo-synced phase.
        let temposyncratio = self.time_unit.temposyncratio();

        self.set_phase_for_process(temposyncratio);

        if self.env_state != LfoEnvState::Stuck {

            // Update envelope if not stuck.
            self.update_envelope_for_process(temposyncratio);
        }

        if self.phase > 1.0 {

            // Handle phase greater than 1.0.
            self.update_for_phase_over_one(shape);
        }

        self.process_shape(shape);

        let mut io2: f32 = self.iout;

        let unipolar  = pvalb![self.params[LfoParam::Unipolar]];
        let magnitude = pvalf![self.params[LfoParam::Magnitude]];

        // Handle unipolar mode and magnitude.
        if unipolar && (shape != LfoShape::StepSequencer) {
                io2 = 0.5 + 0.5 * io2;
        }

        // Set output.
        self.output = (self.env_val * magnitude * io2) as f64;
    }
}

