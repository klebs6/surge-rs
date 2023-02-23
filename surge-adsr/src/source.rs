crate::ix!();

impl ModulationSourceControl for AdsrEnvelope {

    /// returns the type of modulation source, in
    /// this case `ModSrcType::Adsr`.
    ///
    fn get_type(&self) -> ModSrcType
    {
        ModSrcType::Adsr
    }

    /// sets the output value of the ADSR envelope
    /// to `x`, casted to a `f32`.
    ///
    fn set_output(&mut self, x: f64) {
        self.output = x as f32;
    }

    /// returns the current output value of the
    /// ADSR envelope. This is currently not
    /// implemented and always returns `0.0`.
    ///
    /// TODO: is this correct? it seems incorrect
    fn get_output(&self) -> f64 {
        0.0
    }

    /// returns the current output value of the
    /// ADSR envelope, scaled to a range between
    /// 0 and 1, which is not implemented and
    /// always returns `0.0`.
    ///
    /// TODO: is this correct? it seems incorrect
    ///
    fn get_output01(&self) -> f64 {
        0.0
    }

    /// indicates whether this modulation source
    /// is per-voice or per-note, returning `true`
    /// since the ADSR envelope is per-voice.
    ///
    fn per_voice(&self) -> bool { 
        true 
    }

    /// indicates whether the output of this
    /// modulation source is bipolar, returning
    /// `false` since the ADSR envelope is
    /// unipolar.
    ///
    fn is_bipolar(&self) -> bool { 
        false
    }

    /// returns whether the ADSR envelope is
    /// currently enabled or not.
    ///
    fn enabled(&self) -> bool {
        self.enabled
    }

    /// sets the enabled state of the ADSR
    /// envelope to `v`.
    ///
    fn enable(&mut self, v: bool) {
        self.enabled = v;
    }

    /// processes a block of samples for the ADSR
    /// envelope, either using the analog or
    /// digital implementation depending on the
    /// value of the `Mode` parameter.
    ///
    fn process_block(&mut self)
    {
        let do_analog: bool = 
            pvalb![self.params[AdsrParam::Mode]];

        if do_analog {
            self.process_block_analog();

        } else {
            self.process_block_digital();
        }
    }

    /// starts the attack phase of the ADSR
    /// envelope, resetting the internal state and
    /// setting the envelope state to
    /// `AdsrState::Attack`.
    ///
    fn attack(&mut self) {
        self.phase         = 0.0;
        self.output        = 0.0;
        self.idlecount     = 0;
        self.scalestage    = 1.0;

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

    /// resets the internal state of the ADSR
    /// envelope. currently not implemented and does
    /// nothing.
    ///
    fn reset(&mut self) {

    }

    /// sets whether the output of the ADSR
    /// envelope should be bipolar or not, which
    /// is a no-op since the ADSR envelope is
    /// always unipolar.
    ///
    #[inline] fn set_bipolar(&mut self, _b: bool) { /* no-op */ }
}
