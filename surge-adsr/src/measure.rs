crate::ix!();

impl GetModulationSourceOutput for AdsrEnvelope {

    /// returns the current output value of the
    /// ADSR envelope
    ///
    fn get_output(&self) -> f64 {
        todo!();
    }

    /// returns the current output value of the
    /// ADSR envelope, scaled to a range between
    /// 0 and 1
    ///
    fn get_output01(&self) -> f64 {
        todo!();
    }
}
