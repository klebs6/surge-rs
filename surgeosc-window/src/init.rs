crate::ix!();

impl Initialize for WindowOscillator {

    fn init(&mut self) -> Result<(),SurgeError> {

        self.clear();

        let uni_count = self.pvali(WindowOscillatorParam::UniCount);

        self.active_sub_oscs = 
            limit_range( uni_count, 1, WINDOW_OSCILLATOR_NUM_SUBOSCS as i32 - 1);

        //TODO for all
        //self.set_pitch(...)

        let out_attenuation_inv: f32 = (self.active_sub_oscs as f32).sqrt();

        self.out_attenuation = 1.00 / (out_attenuation_inv * 16777216.0);

        let window_wt_size = self.window_wavetable.num_samples_per_table();

        if self.active_sub_oscs == 1
        {
            self.detune_bias = 1.0;
            self.detune_offset = 0.0;
            self.gain[[0, 0]] = 128;
            self.gain[[0, 1]] = 128; // unity gain
            self.pos[0] = (window_wt_size << 16) as u32;

        } else {

            self.detune_bias = 2.0 / (self.active_sub_oscs as f32 - 1.0);
            self.detune_offset = -1.0;

            let odd: bool = (self.active_sub_oscs & 1) != 0;
            let mid:  f32 = (self.active_sub_oscs as f32) * 0.5 - 0.5;
            let half = (self.active_sub_oscs >> 1) as usize;

            for i in (0_usize..self.active_sub_oscs as usize).step_by(1) 
            {
                let mut d: f32 = (i as f32 - mid).abs() / mid;

                if odd && (i >= half) {
                    d = -d;
                }

                if (i & 1) != 0 {
                    d = -d;
                }

                self.gain[[i,0]] = {

                    let b0 = 128.0 * megapan_left(d);

                    let b1 = (b0 as f32) as i32;

                    limit_range( b1, 0, 255) as u32
                };

                self.gain[[i,1]] = {

                    let b0 = 128.0 * megapan_right(d);

                    let b1 = (b0 as f32) as i32;

                    limit_range(b1, 0, 255) as u32
                };

                if pvalb![self.osc_params[OscillatorParam::Retrigger]] {

                    self.pos[i] = {
                        let b0 = window_wt_size * (i as usize);
                        let b1 = self.active_sub_oscs as usize;
                        let b2 = b0 / b1;
                        let b3 = window_wt_size + b2;
                        ( b3 << 16) as u32
                    };

                } else {

                    let randi32 = rand01() as i32;

                    self.pos[i] = {
                        let b0 = window_wt_size - 1;
                        let b1 = randi32 & ( b0 as i32);
                        let b2 = window_wt_size + b1 as usize;
                        (b2 << 16) as u32 
                    };
                }
            }
        }

        Ok(())
    }
}
