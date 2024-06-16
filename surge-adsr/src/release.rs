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
        self.set_scalestage(self.get_output() as f32);
        self.set_phase(1.0);
        self.set_envstate(AdsrState::Release);
    }
}
