ix!();

use crate::{
    WindowOscillator,
    WindowOscillatorParam,
};

impl Init for WindowOscillator<'sr> {

    fn init(&mut self) {

        self.clear();

        let uni_count = self.pvali(WindowOscillatorParam::UniCount);

        self.active_sub_oscs = 
            limit_range_i( uni_count, 1, WINDOW_OSCILLATOR_NUM_SUBOSCS as i32 - 1);

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
            let mid: f32 = (self.active_sub_oscs as f32) * 0.5 - 0.5;
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

                self.gain[[i,0]] = limit_range_i(
                    ((128.0 * megapan_left(d)) as f32) as i32, 
                    0, 255) as u32;

                self.gain[[i,1]] = limit_range_i(
                    ((128.0 * megapan_right(d)) as f32) as i32, 
                    0, 255) as u32;

                if pvalb![self.osc_params[OscillatorParam::Retrigger]] {
                    self.pos[i] = (
                        (
                            window_wt_size + 
                            (
                                (window_wt_size * (i as usize)) / 
                                (self.active_sub_oscs as usize)
                            )
                        ) 
                        << 16
                    ) as u32;
                } else {
                    let randi32 = rand01() as i32;

                    self.pos[i] = ((
                            window_wt_size + 
                            (
                                randi32 & (
                                    (
                                        window_wt_size - 1
                                    ) as i32
                                )
                            ) as usize
                    ) << 16) as u32;
                }
            }
        }
    }
}
