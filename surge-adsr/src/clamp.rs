crate::ix!();

impl Clamp01 for AdsrEnvelope {

    fn clamp01(&mut self) {
        self.output = limit_range(self.output, 0.0, 1.0);
    }
}
