ix!();

use crate::A1d;

#[derive(Debug)]
pub struct AllpassFilter<const N: usize> {
    buffer: A1d::<f64>,
    a:      f64,
    wpos:   usize,
}

impl<const N: usize> Default for AllpassFilter<N> {

    fn default() -> Self {
        Self {
            wpos: 0,
            a: 0.3,
            buffer: A1d::<f64>::zeros(N)
        }
    }
}

impl<const N: usize> AllpassFilter<N> {

    pub fn process(&mut self, x: f64) -> f64 {

        self.wpos              = (self.wpos + 1) % N;
        let y: f64             = self.buffer[self.wpos];
        self.buffer[self.wpos] = y * -self.a + x;

        y + self.buffer[self.wpos] * self.a

    }

    pub fn set_a(&mut self, a: f64) {
        self.a = a;
    }
}
