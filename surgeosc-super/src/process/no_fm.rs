crate::ix!();

impl SurgeSuperOscillator {

    pub fn process_block_nofm(&mut self, cfg: &OscillatorProcessBlockCfg) {

        /* The amount of phase space we need to cover is the oversample block size * the wavelength */
        let a: f32 = (BLOCK_SIZE_OS as f32) * self.blitter.pitchmult;

        for l in 0_usize..(self.blitter.n_unison as usize) {

            self.blitter.driftlfo[l] = drift_noise(self.blitter.driftlfo2[l]);

            /* Either while sync is active and we need to fill syncstate traversal,
               or while we need to fill oscstate traversal to cover the expected request, */
            while 
                ((self.l_sync.v > 0.0) && (self.blitter.syncstate[l] < a)) 
                || (self.blitter.oscstate[l] < a)
            {
                /* Fill the buffer for the voice */
                self.convolute(
                    ConvolutionCfg { voice: l, stereo: cfg.stereo, fm: false }
                );
            }

            /* And take the amount of phase space we just covered from both the
               oscillator and sync state */
            self.blitter.oscstate[l] -= a;

            if self.l_sync.v > 0.0 {
                self.blitter.syncstate[l] -= a;
            }

            /* At this point we are guaranteed that the oscbuffer contains enough
               generated sample to cover at least the amount of sample space (which
               is block size * wavelength as above) that we need to cover. So we can go
               ahead and process */
        }
    }
}

