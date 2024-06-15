crate::ix!();

impl DigitalRelease for AdsrEnvelope {

    /// Processes a block of samples during the
    /// release stage of the envelope.
    ///
    /// Overall, this function is responsible for
    /// decreasing the amplitude of the envelope
    /// over time during the release stage,
    /// applying a shape to the release stage of
    /// the envelope, and setting the envelope to
    /// its idle state once the release stage is
    /// complete.
    ///
    fn digital_release(&mut self) {

        let release = self.get_release_parameter();

        // It subtracts the envelope rate (calculated from the release parameter) multiplied by the
        // release time sync ratio from the phase of the envelope. 
        //
        // This is done to decrease the amplitude of the envelope over time during the release
        // stage. 
        //
        // The result is stored in the `phase` variable.
        //
        self.phase -= 
            self.tables.envelope_rate_linear(release ) 
            * tsyncratio![self,Release];

        self.output = self.phase;

        // loop based on the value of the release shape parameter. 
        //
        // For each iteration of the loop, the `output` variable is multiplied by the current value
        // of the `phase` variable. 
        //
        // This is done to apply a shape to the release stage of the envelope.
        //
        for _i in 0..pvali![self.params[AdsrParam::ReleaseShape]] {
            self.output *= self.phase;
        }

        // If the `phase` variable is less than 0, this means that the release stage is complete
        // and the envelope has reached its idle state. 
        //
        // The `envstate` variable is set to `AdsrState::Idle` and the `output` variable is set to
        // 0.
        //
        if self.phase < 0.0
        {
            self.envstate = AdsrState::Idle;
            self.output = 0.0;
        }

        // Finally, the `output` variable is multiplied by the `scalestage` variable. 
        //
        // This is done to scale the envelope by the appropriate amount for the current stage of
        // the envelope.
        //
        self.output *= self.scalestage;
    }
}
