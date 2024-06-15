crate::ix!();

impl Attack for AdsrEnvelope {

    /// starts the attack phase of the ADSR
    /// envelope, resetting the internal state and
    /// setting the envelope state to
    /// `AdsrState::Attack`.
    ///
    fn attack(&mut self) {
        self.phase      = 0.0;
        self.output     = 0.0;
        self.idlecount  = 0;
        self.scalestage = 1.0;

        // Reset the analog state machine too
        self._v_c1         = 0.0;
        self._v_c1_delayed = 0.0;
        self._discharge    = 0.0;

        self.envstate = AdsrState::Attack;

        let v1: f32   = pvalf![self.params[AdsrParam::Attack]];
        let v2: f32   = pvalminf![self.params[AdsrParam::Attack]];
        let diff: f32 = v1 - v2;

        if diff < 0.01 {
            self.envstate = AdsrState::Decay;
            self.output   = 1.0;
            self.phase    = 1.0;
        }
    }
}
