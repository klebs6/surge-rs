crate::ix!();

#[derive(Debug,Clone,Default)]
pub struct OnePoleFilter {
    a0: f32,
}

impl OnePoleFilter {

    pub fn process_lowpass(&mut self, x: f32, c0: f32) -> f32 {
        self.a0 = self.a0 * c0 + x * (1.0 - c0);
        self.a0
    }

    pub fn process_highpass(&mut self, x: f32, c0: f32) -> f32 {
        self.a0 = self.a0 * (1.0 - c0) + x * c0;
        x - self.a0
    }
}

