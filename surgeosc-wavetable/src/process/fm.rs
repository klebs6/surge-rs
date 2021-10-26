ix!();

use crate::{
    WTOscillator,
};

impl WTOscillator {

    #[inline] pub fn process_block_fm(&mut self, depth: f32, stereo: bool) { 

        for l in 0..(self.blitter.n_unison) {
            self.blitter.driftlfo[l as usize] = 
                drift_noise(self.blitter.driftlfo2[l as usize]);
        }

        for s in (0..BLOCK_SIZE_OS).step_by(1) {

            let master_osc = master_osc![self,s];

            let fmmul: f32 = limit_range(1.0 + depth *
                master_osc, 0.10, 1.90);

            let a: f32 = self.blitter.pitchmult * fmmul;

            self.fm_delay = s as i32;

            for l in (0_usize..self.blitter.n_unison as usize).step_by(1) {

                while self.blitter.oscstate[l] < a {

                    self.fm_mul_inv = rcp(fmmul);
                    self.convolute(l as i32, true, stereo);
                }

                self.blitter.oscstate[l] -= a;
            }
        }
    }
}
