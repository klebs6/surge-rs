crate::ix!();

impl Lfo {

    /// Triggers an attack phase of the LFO.
    ///
    /// If the LFO's phase has not been
    /// initialized, initializes it from the start
    /// phase.
    ///
    /// Sets the LFO's envelope state to `Delay`
    /// and its envelope value and phase to 0.0.
    ///
    /// Takes in the current LFO parameters and
    /// computes the attack phase shape and
    /// updates the LFO's envelope state and phase
    /// accordingly.
    ///
    /// # Examples
    ///
    /// ```
    /// use synth::Lfo;
    ///
    /// let mut lfo = Lfo::new();
    ///
    /// // Set parameters for the LFO
    /// lfo.set_param(LfoParam::Delay, 0.0);
    /// lfo.set_param(LfoParam::Attack, 1.0);
    /// lfo.set_param(LfoParam::Hold, 0.0);
    /// lfo.set_param(LfoParam::Rate, 1.0);
    /// lfo.set_param(LfoParam::Shape, 0);
    /// lfo.set_param(LfoParam::Trigmode, 0);
    /// lfo.set_param(LfoParam::StartPhase, 0.0);
    ///
    /// lfo.attack();
    /// ```
    ///
    pub fn attack(&mut self) {

        // If the LFO's phase has not been
        // initialized (e.g. if it was manually
        // set to a specific value), set it to the
        // starting phase specified by the user
        //
        if ! self.phase_initialized {
            self.init_phase_from_start_phase();
        }

        // Set the LFO's envelope state to Delay
        // (i.e. it's waiting to start the attack
        // phase)
        //
        self.env_state = LfoEnvState::Delay;

        // Set the envelope value to 0.0 (i.e. the
        // envelope is at the start of its range)
        //
        self.env_val   = 0.0;

        // Set the envelope phase to 0.0 (i.e. the
        // envelope is at the start of its cycle)
        //
        self.env_phase = 0.0;

        // Get the starting phase, shape, mode,
        // delay, attack, hold, and rate
        // parameters from the LFO's parameter
        // list
        //
        let start_phase = pvalf![self.params[LfoParam::StartPhase]];
        let shape       = LfoShape::try_from(pvali![self.params[LfoParam::Shape]] as usize).unwrap();
        let mode        = LfoMode::try_from(pvali![self.params[LfoParam::Trigmode]] as usize).unwrap();
        let delay       = pvalf![self.params[LfoParam::Delay]];
        let attack      = pvalf![self.params[LfoParam::Attack]];
        let hold        = pvalf![self.params[LfoParam::Hold]];
        let rate        = pvalf![self.params[LfoParam::Rate]];

        // Update the LFO's envelope state if
        // necessary (i.e. if the attack phase
        // starts immediately or after a delay)
        //
        self.maybe_update_env_state_for_attack(delay,attack,hold);

        // Update the LFO's phase and step values
        // for the attack phase
        //
        self.maybe_update_phase_and_step_for_attack(shape,mode,start_phase,rate);

        // Start the attack phase with the
        // specified shape and starting phase
        //
        self.attack_shape(shape,start_phase);
    }
}
