crate::ix!();

impl DigitalAttack for AdsrEnvelope {

    /// The purpose of this method is to process the attack phase of the digital
    /// ADSR envelope.
    ///
    fn digital_attack(&mut self) {

        let lc_a = self.get_attack_parameter();

        // The attack phase of an ADSR envelope is the first phase and
        // represents the increase in amplitude over time from
        // 0 to the maximum value. 
        //
        // This line calculates the rate of the attack using the
        // `envelope_rate_linear` method from the `tables` field of the
        // `AdsrEnvelope` struct, passing in the `lc_a` parameter. It then
        // multiplies this rate by `tsyncratio![self, Attack]`, which is a macro
        // that retrieves the current sync ratio for the attack phase. 
        //
        // Finally, the calculated rate is added to the current phase of the
        // envelope to update its value.
        //
        self.phase += self.tables.envelope_rate_linear(lc_a) * tsyncratio![self,Attack];

        // If the phase of the envelope has reached or exceeded the maximum
        // value of 1.0, then the attack phase is complete and it is time to
        // transition to the decay phase. 
        //
        // The method sets the `envstate` field of the envelope to
        // `AdsrState::Decay` and retrieves the value of the
        // `AdsrParam::Sustain` parameter from the envelope's `params` field to
        // set as the `sustain` field of the envelope.
        //
        if self.phase >= 1.0
        {
            self.phase    = 1.0;
            self.envstate = AdsrState::Decay;
            self.sustain  = self.get_sustain_parameter();
        }

        // Depending on the value of this parameter, the method calculates the
        // output value using a different formula. 
        //
        self.output = match self.get_attack_shape() {
            AdsrEnvelopeAttackShape::Zero => self.phase.sqrt(),
            AdsrEnvelopeAttackShape::One  => self.phase,
            AdsrEnvelopeAttackShape::Two  => self.phase * self.phase,
        }
    }
}
