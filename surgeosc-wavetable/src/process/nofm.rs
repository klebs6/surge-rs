ix!();

use crate::{
    WTOscillator,
};

impl WTOscillator {

    #[inline] pub fn process_block_nofm(&mut self, stereo: bool) { 

        let a: f32 = (BLOCK_SIZE_OS as f32) * self.blitter.pitchmult;

        for l in (0_usize..self.blitter.n_unison as usize).step_by(1) {

            self.blitter.driftlfo[l] = drift_noise(self.blitter.driftlfo2[l]);

            while self.blitter.oscstate[l] < a {
                self.convolute(l as i32, false, stereo);
            }

            self.blitter.oscstate[l] -= a;
        }
    }
}
