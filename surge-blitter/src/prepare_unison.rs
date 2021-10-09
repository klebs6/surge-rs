ix!();

impl crate::AbstractBlitter {

    #[inline] pub fn update_out_attenuation(&mut self) {
        self.out_attenuation_inv = self.n_unison as f32;
        self.out_attenuation     = 1.0 / self.out_attenuation_inv;
    }

    #[inline] pub fn prepare_single_voice(&mut self) {
        self.detune_bias    = 1.0;
        self.detune_offset  = 0.0;
        self.pan_l[0]       = 1.0;
        self.pan_r[0]       = 1.0;
    }

    #[inline] pub fn prepare_multi_voice(&mut self, voices: usize) {

        self.detune_bias   = 2.0 / (self.n_unison as f32 - 1.0);
        self.detune_offset = -1.0;

        let odd:  bool   = (voices & 1) != 0;
        let mid:  f32    = (voices as f32) * 0.5 - 0.5;
        let half: usize  = voices >> 1;

        for i in 0..voices {

            let mut d: f32 = (i as f32 - mid).abs() / mid;

            if odd && (i >= half) {
                d = -d;
            }

            if (i & 1) != 0 {
                d = -d;
            }

            self.pan_l[i] = 1.0 - d;
            self.pan_r[i] = 1.0 + d;
        }
    }

    pub fn prepare_unison(&mut self, voices: usize) {

        self.update_out_attenuation();

        match voices {
            1 => self.prepare_single_voice(),
            _ => self.prepare_multi_voice(voices),
        }
    }
}
