crate::ix!();

impl AdsrEnvelope {

    fn get_attack_diff(&self) -> f32 {
        let v1 = self.get_attack_parameter();
        let v2 = self.get_attack_parameter_minimum();

        let diff: f32 = v1 - v2;

        diff
    }

    fn state_machine_trigger_decay_stage(&mut self) {
        self.set_envstate(AdsrState::Decay);
        self.set_output(1.0);
        self.set_phase(1.0);
    }
}

impl Attack for AdsrEnvelope {

    /// starts the attack phase of the ADSR
    /// envelope, resetting the internal state and
    /// setting the envelope state to
    /// `AdsrState::Attack`.
    ///
    fn attack(&mut self) {

        self.clear_phase();
        self.clear_output();
        self.clear_idlecount();
        self.set_scalestage(1.0);

        // we do this here, too
        self.reset_analog_state_machine();

        self.set_envstate(AdsrState::Attack);

        if self.get_attack_diff() < 0.01 {
            self.state_machine_trigger_decay_stage();
        }
    }
}
