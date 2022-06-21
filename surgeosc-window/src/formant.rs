crate::ix!();

impl WindowOscillator {

    pub fn get_formant_mul(&self, formant: f64) -> i32 {

        let formant_mul: i32 = {
            let t0 = self.tuner.n2p_tuningctr(formant as f64);
            ((65536.0 * t0) as f32) as i32
        };

        // We can actually get input tables bigger
        // than the convolution table
        let window_vs_wave_po2: i32 = {
            let t0 = self.window_wavetable.num_samples_per_table_po2();
            let t1 = self.wave_wavetable.num_samples_per_table_po2();
            (t0 - t1) as i32
        };

        match window_vs_wave_po2 < 0 {
            true  => maxi(formant_mul << -window_vs_wave_po2, 1),
            false => maxi(formant_mul >> window_vs_wave_po2, 1),
        }
    }
}
