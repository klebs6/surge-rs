ix!();

use crate::{
    Vocoder,
    VocoderParam,
    N_VOCODER_BANDS,
};

impl Update for Vocoder<'sr> {

    fn update(&mut self) {

        let f_quality:   f32 = self.pvalf(VocoderParam::Quality);
        let f_lo:        f32 = self.pvalf(VocoderParam::FreqLo);
        let f_hi:        f32 = self.pvalf(VocoderParam::FreqHi);
        let f_modcenter: f32 = self.pvalf(VocoderParam::ModCenter);
        let f_modexpand: f32 = self.pvalf(VocoderParam::ModExpand);
        let i_num_bands: i32 = self.pvali(VocoderParam::NumBands);

        let mut freq:   [f32; 4] = [0.0; 4];
        let mut freq_m: [f32; 4] = [0.0; 4];

        let quality_factor:      f32 = 20.0 * (1.0 + 0.5 * f_quality);
        let spread: f32 = 0.4 / quality_factor;

        self.active_bands = i_num_bands;

        // FIXME - adjust the UI to be chunks of 4
        self.active_bands = self.active_bands - ( self.active_bands % 4 ); 

        let mut flo: f32 = f_lo;
        let mut fhi: f32 = f_hi;

        if flo > fhi {
            std::mem::swap(&mut fhi, &mut flo);
        }

        let abands_f32: f32 = self.active_bands as f32;

        let df: f32 = (fhi - flo) / (abands_f32 - 1.0);

        let hzlo: f32 = (CONCERT_A_HZ as f32) * 2.0_f32.powf( flo / 12.0 );
        let dhz:  f32 = 2.0_f32.powf( df / 12.0 );

        let mut fb: f32 = hzlo;

        let mut mb:     f32 = fb;
        let mut mdhz:   f32 = dhz;
        let mut sep_mod: bool = false;

        let m_c: f32 = f_modcenter;
        let m_x: f32 = f_modexpand;

        if m_c != 0.0 || m_x != 0.0 {

            sep_mod = true;

            let f_dist:      f32 = fhi - flo;
            let f_dist_half: f32 = f_dist / 2.0;

            // that 0.3 is a tuning choice about how far we can move center
            let mid:      f32 = f_dist_half + flo + 0.3 * m_c * f_dist_half; 

            // as is that 0.7
            let lo: f32 = mid - f_dist_half * ( 1.0 + 0.7 * m_x ); 

            let mut d_m: f32 = f_dist_half * 2.0 * ( 1.0 + 0.7 * m_x ) / (abands_f32 - 1.0);

            let hi:  f32 = lo + d_m * (abands_f32 - 1.0);

            if hi > 60.0 {
                d_m = ( 60.0 - lo ) / ((abands_f32 as f32) - 1.0);
            }

            // value assigned to hi is never read
            /*
            hi = lo + d_m * (abands_f32 - 1.0);
            */

            mb   = (CONCERT_A_HZ as f32) * 2.0_f32.powf( lo / 12.0 );

            mdhz = 2.0_f32.powf( d_m / 12.0 );
        }

        for i in 0..self.active_bands {

            assert!(i < N_VOCODER_BANDS as i32);

            freq[(i & 3) as usize]   = fb * self.srunit.samplerate_inv();
            freq_m[(i & 3) as usize] = mb * self.srunit.samplerate_inv();

            if (i & 3) == 3 {

                let j: i32 = i >> 2;
                let ju = j as usize;

                self.carrier_l[ju].set_coeff(freq, quality_factor, spread);
                self.carrier_r[ju].copy_coeff(&mut self.carrier_l[ju]);

                if sep_mod {
                    self.modulator[ju].set_coeff(freq_m, quality_factor, spread);
                } else {
                    self.modulator[ju].copy_coeff(&mut self.carrier_l[ju]);
                }

            }
            fb *= dhz;
            mb *= mdhz;
        }
    }
}
