
ix!();

use crate::{
    WTOscillator,
};

impl WTOscillator {

    /// each block overlap FIR_IPOL_N samples into the next (due to impulses not being wrapped around
    /// the block edges copy the overlapping samples to the new block position
    /// only needed if the new self.blitter.bufpos == 0
    pub fn maybe_handle_overlap(&mut self, stereo: bool) { 
        if self.blitter.bufpos == 0 {

            let mut overlap_l = [unsafe{ z128![] }; FIR_IPOL_N >> 2];
            let mut overlap_r = [unsafe{ z128![] }; FIR_IPOL_N >> 2];

            for k in (0..(FIR_IPOL_N)).step_by(4) {
                unsafe {

                    overlap_l[k >> 2] = _mm_load_ps(&self.blitter.oscbuffer_l[OB_LENGTH + k]);

                    _mm_store_ps(&mut self.blitter.oscbuffer_l[k], overlap_l[k >> 2]);
                    _mm_store_ps(&mut self.blitter.oscbuffer_l[OB_LENGTH + k], z128![]);

                    if stereo {

                        overlap_r[k >> 2] = _mm_load_ps(&self.blitter.oscbuffer_r[OB_LENGTH + k]);

                        _mm_store_ps(&mut self.blitter.oscbuffer_r[k], overlap_r[k >> 2]);
                        _mm_store_ps(&mut self.blitter.oscbuffer_r[OB_LENGTH + k], z128![]);
                    }
                }
            }
        }
    }
}
