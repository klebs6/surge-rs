crate::ix!();

pub trait DigitalAttack {

    fn digital_attack(&mut self);
}

impl DigitalAttack for AdsrEnvelope {

    /// The purpose of this method is to process the attack phase of the digital
    /// ADSR envelope.
    ///
    fn digital_attack(&mut self) {

        // The calculated rate is added to the current phase of the
        // envelope to update its value.
        //
        let phase_increment = self.attack_rate();

        self.increment_phase(phase_increment);

        // If the phase of the envelope has reached or exceeded the maximum
        // value of 1.0, then the attack phase is complete and it is time to
        // transition to the decay phase. 
        //
        // The method sets the `envstate` field of the envelope to
        // `AdsrState::Decay` and retrieves the value of the
        // `AdsrParam::Sustain` parameter from the envelope's `params` field to
        // set as the `sustain` field of the envelope.
        //
        if self.phase_gte_one()
        {
            self.set_phase(1.0);
            self.set_envstate(AdsrState::Decay);
            self.set_sustain(self.get_sustain_parameter());
        }

        let new_output = match self.get_attack_shape() {
            AdsrEnvelopeAttackShape::Zero => self.phase().sqrt(),
            AdsrEnvelopeAttackShape::One  => self.phase(),
            AdsrEnvelopeAttackShape::Two  => self.phase() * self.phase(),
        };

        self.set_output(new_output)
    }
}
