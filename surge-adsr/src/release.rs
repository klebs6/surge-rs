crate::ix!();

impl Release for AdsrEnvelope {

    /// starts the release phase of the ADSR
    /// envelope, setting the internal state to
    /// the beginning of the release phase and
    /// setting the envelope state to
    /// `AdsrState::Release`.
    ///
    fn release(&mut self) {

        // note, there was some other commented
        // logic here before the port
        //
        self.scalestage = self.output as f32;
        self.phase      = 1.0;
        self.envstate   = AdsrState::Release;
    }
}
