crate::ix!();

impl SetModulationSourceOutput for AdsrEnvelope {

    /// sets the output value of the ADSR envelope
    /// to `x`, casted to a `f32`.
    ///
    fn set_output(&mut self, x: f64) {
        self.output = x as f32;
    }
}
