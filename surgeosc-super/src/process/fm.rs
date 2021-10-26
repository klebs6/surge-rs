ix!();

use crate::SurgeSuperOscillator;

impl SurgeSuperOscillator {

    pub fn process_block_fm(&mut self, cfg: &OscillatorProcessBlockCfg) {

        for l in 0_usize..(self.blitter.n_unison as usize) {

            self.blitter.driftlfo[l] = drift_noise(self.blitter.driftlfo2[l]);
        }

        for s in 0..BLOCK_SIZE_OS {

            let master_osc: f32 = unsafe { *self.master_osc.add(s) };

            let fmmul: f32 = limit_range(1.0 + cfg.fm_depth * master_osc, 0.1, 1.9);
            let a:     f32 = self.blitter.pitchmult * fmmul;

            self.fm_delay = s as i32;

            for l in 0_usize..(self.blitter.n_unison as usize) {

                while ((self.l_sync.v > 0.0) && (self.blitter.syncstate[l] < a)) || (self.blitter.oscstate[l] < a)
                {
                    self.fm_mul_inv = rcp(fmmul);

                    /* The division races with the growth of the oscstate so that 
                       it never comes out of/gets out of the loop
                       this becomes unsafe, don't fuck with the oscstate but 
                       make a division within the convolute instead.*/
                    self.convolute(
                        ConvolutionCfg{ voice: l, stereo: cfg.stereo, fm: true }
                    );
                }

                self.blitter.oscstate[l] -= a;
                if self.l_sync.v > 0.0 {
                    self.blitter.syncstate[l] -= a;
                }
            }
        }
    }
}
