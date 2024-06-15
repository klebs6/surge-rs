crate::ix!();

impl Initialize for AdsrEnvelope {

    fn init(&mut self) 
    {
        self.envstate      = AdsrState::Attack;
        self.phase         = 0.0;
        self.output        = 0.0;
        self.idlecount     = 0;
        self.scalestage    = 1.0;
        self._v_c1         = 0.0;
        self._v_c1_delayed = 0.0;
        self._discharge    = 0.0;
    }
}
