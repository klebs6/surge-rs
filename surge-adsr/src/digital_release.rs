crate::ix!();

pub trait DigitalRelease {

    fn digital_release(&mut self);
}

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

        // It subtracts the envelope rate (calculated from the release parameter) multiplied by the
        // release time sync ratio from the phase of the envelope. 
        //
        // This is done to decrease the amplitude of the envelope over time during the release
        // stage. 
        //
        // The result is stored in the `phase` variable.
        //
        self.decrement_phase(self.release_rate());

        self.set_output(self.phase());

        // loop based on the value of the release shape parameter. 
        //
        // For each iteration of the loop, the `output` variable is multiplied by the current value
        // of the `phase` variable. 
        //
        // This is done to apply a shape to the release stage of the envelope.
        //
        for _i in 0..self.release_shape() {

            self.scale_output(self.phase());
        }

        // If the `phase` variable is less than 0, this means that the release stage is complete
        // and the envelope has reached its idle state. 
        //
        // The `envstate` variable is set to `AdsrState::Idle` and the `output` variable is set to
        // 0.
        //
        if self.phase_is_negative()
        {
            self.set_envstate(AdsrState::Idle);
            self.clear_output();
        }

        let scalestage = self.scalestage();

        // Finally, the `output` variable is multiplied by the `scalestage` variable. 
        //
        // This is done to scale the envelope by the appropriate amount for the current stage of
        // the envelope.
        //
        self.scale_output(scalestage);
    }
}
