ix!();

#[derive(Debug,Clone)]
pub struct AllpassPreDelay {
    k: i32,
    data: A1d::<f32>,
}

impl Default for AllpassPreDelay {
    fn default() -> Self {
        Self {
            k: 0,
            data: A1d::<f32>::zeros(ALLPASS_REVERB_PREDELAY_BUFFER_SIZE as usize),
        }
    }
}

impl AllpassPreDelay {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(&mut self, input: f32, tap: i32) -> f32 {

        self.k += 1;

        if self.k == (ALLPASS_REVERB_PREDELAY_BUFFER_SIZE as i32) {
            self.k = 0;
        }

        let mut p: i32 = self.k - tap;

        if p < 0 {
            p += ALLPASS_REVERB_PREDELAY_BUFFER_SIZE as i32;
        }

        assert!(p > 0);

        let res = self.data[p as usize];
        self.data[self.k as usize] = input;
        res
    }
}
