ix!();

use crate::{
    SurgeSuperOscillator,
    SSOParam,
};

impl SurgeSuperOscillator<'sr> {

    #[inline] pub fn get_character(&mut self) -> CharacterMode {
        CharacterMode::try_from(pvali![self.params[SSOParam::Character]] as usize).unwrap()
    }

    #[inline] pub fn set_rate_all(&mut self, rate: f32) {
        self.l_pw.set_rate(rate);
        self.l_pw2.set_rate(rate);
        self.l_shape.set_rate(rate);
        self.l_sub.set_rate(rate);
        self.l_sync.set_rate(rate);
    }

    #[inline] pub fn clear_tracking(&mut self) {
        for x in self.last_level.iter_mut() {
            *x = 0.0;
        }
        for x in self.elapsed_time.iter_mut() {
            *x = 0.0;
        }
    }

    #[inline] pub fn set_coeff_by_character(&mut self, mode: CharacterMode) {

        match mode {
            CharacterMode::Warm => {
                let mut filt: f32 = 1.0 - 2.0 * 5000.0 * self.srunit.samplerate_inv();
                filt *= filt;
                self.coeff_b0 = 1.0 - filt;
                self.coeff_b1 = 0.0;
                self.coeff_a1 = filt;
            },
            CharacterMode::Neutral => {
                self.coeff_b0 = 1.0;
                self.coeff_b1 = 0.0;
                self.coeff_a1 = 0.0;
            },
            CharacterMode::Bright => {
                let mut filt: f32 = 1.0 - 2.0 * 5000.0 * self.srunit.samplerate_inv();
                filt *= filt;
                let a0: f32 = 1.0 / (1.0 - filt);
                self.coeff_b0 = 1.0 * a0;
                self.coeff_b1 = -filt * a0;
                self.coeff_a1 = 0.0;
            },
        }
    }
}
