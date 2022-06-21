crate::ix!();

impl ModulationSourceControl for AdsrEnvelope {

    fn get_type(&self) -> ModSrcType
    {
        ModSrcType::Adsr
    }

    fn set_output(&mut self, x: f64) {
        self.output = x as f32;
    }

    //TODO: is this correct? it seems incorrect
    fn get_output(&self) -> f64 {
        0.0
    }

    //TODO: is this correct? it seems incorrect
    fn get_output01(&self) -> f64 {
        0.0
    }

    fn per_voice(&self) -> bool { 
        true 
    }

    fn is_bipolar(&self) -> bool { 
        false
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self, v: bool) {
        self.enabled = v;
    }

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

    fn release(&mut self) {
        //note, there was some other commented logic here before the port
        self.scalestage = self.output as f32;
        self.phase      = 1.0;
        self.envstate   = AdsrState::Release;
    }

    //TODO: is this correct? it seems incorrect
    fn reset(&mut self) {

    }

    #[inline] fn set_bipolar(&mut self, _b: bool) { /* no-op */ }
}
