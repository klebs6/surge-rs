crate::ix!();

impl Init for Chorus {

    fn init(&mut self) {

        self.buffer = Align16(Self::new_chorus_buffer());

        self.wpos = 0;
        self.envf = 0.0;

        let gainscale: f64 = 1.0 / (CHORUS_DEPTH as f64).sqrt();

        for idx in 0_usize..(CHORUS_DEPTH as usize) {

            self.time[idx].set_rate(0.001);

            let mut x: f64 = idx as f64;

            x /= (CHORUS_DEPTH - 1) as f64;

            self.lfophase[idx] = x;

            x = 2.0 * x - 1.0;

            self.voicepan[[idx, 0]] = ((0.5 - 0.5 * x).sqrt() * gainscale) as f32;
            self.voicepan[[idx, 1]] = ((0.5 + 0.5 * x).sqrt() * gainscale) as f32;
            unsafe {
                self.voicepan_l4[idx] = _mm_set1_ps(self.voicepan[[idx,0]]);
                self.voicepan_r4[idx] = _mm_set1_ps(self.voicepan[[idx,1]]);
            }
        }

        self.initial_update();
    }
}

