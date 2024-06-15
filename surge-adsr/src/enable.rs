crate::ix!();

impl CheckIsModulationSourceEnabled for AdsrEnvelope {

    /// returns whether the ADSR envelope is
    /// currently enabled or not.
    ///
    fn enabled(&self) -> bool {
        self.enabled
    }
}

impl Enable for AdsrEnvelope {

    /// sets the enabled state of the ADSR
    /// envelope to `v`.
    ///
    fn enable(&mut self, v: bool) {
        self.enabled = v;
    }
}
