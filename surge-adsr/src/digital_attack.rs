crate::ix!();

impl AdsrEnvelope {

    /// The purpose of this method is to process the
    /// attack phase of the ADSR envelope.
    ///
    pub fn process_block_digital_attack(&mut self) {

        // This line retrieves the attack
        // parameter value from the envelope's
        // `params` field using the
        // `AdsrParam::Attack` key. `pvalf!` is
        // a macro that retrieves the float value
        // of the parameter from the params field.
        //
        let lc_a: f32 = pvalf![self.params[AdsrParam::Attack]];

        // The attack phase of an ADSR envelope is
        // the first phase and represents the
        // increase in amplitude over time from
        // 0 to the maximum value. This line
        // calculates the rate of the attack using
        // the `envelope_rate_linear` method from
        // the `tables` field of the
        // `AdsrEnvelope` struct, passing in the
        // `lc_a` parameter. It then multiplies
        // this rate by `tsyncratio![self,
        // Attack]`, which is a macro that
        // retrieves the current sync ratio for
        // the attack phase. Finally, the
        // calculated rate is added to the current
        // phase of the envelope to update its
        // value.
        //
        self.phase += self.tables.envelope_rate_linear(lc_a) * tsyncratio![self,Attack];

        // If the phase of the envelope has
        // reached or exceeded the maximum value
        // of 1.0, then the attack phase is
        // complete and it is time to transition
        // to the decay phase. 
        //
        // The method sets the `envstate` field of
        // the envelope to `AdsrState::Decay` and
        // retrieves the value of the
        // `AdsrParam::Sustain` parameter from the
        // envelope's `params` field to set as the
        // `sustain` field of the envelope.
        //
        if self.phase >= 1.0
        {
            self.phase = 1.0;
            self.envstate = AdsrState::Decay;
            self.sustain = 
                pvalf![self.params[AdsrParam::Sustain]];
        }

        // Finally, this line sets the `output`
        // field of the envelope to a value based
        // on the `AdsrParam::AttackShape`
        // parameter in the envelope's `params`
        // field. 
        //
        // The `pvali!` macro retrieves the
        // integer value of the parameter. 
        //
        // Depending on the value of this
        // parameter, the method calculates the
        // output value using a different formula. 
        //
        // If the parameter value is 0, the output
        // is the square root of the phase. 
        //
        // If the parameter value is 1, the output
        // is just the phase. 
        //
        // If the parameter value is 2, the output
        // is the square of the phase. 
        //
        // If the parameter value is anything
        // else, the method panics with an error
        // message indicating a logic bug.
        //
        self.output = match 
            pvali![self.params[AdsrParam::AttackShape]] 
        {
            0 => self.phase.sqrt(),
            1 => self.phase,
            2 => self.phase * self.phase,
            _ => panic!( "logic bug")
        };
    }
}
