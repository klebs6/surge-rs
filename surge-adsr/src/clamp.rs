crate::ix!();

impl Clamp01 for AdsrEnvelope {

    fn clamp01(&mut self) {

        let output_clamped = limit_range(self.get_output(), 0.0, 1.0);

        self.set_output(output_clamped as f32);
    }
}
