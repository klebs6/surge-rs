ix!();

impl crate::SurgeSuperOscillator<'sr> {

    /** 
     | each block overlap FIRipol_N samples into
     | the next (due to impulses not being wrapped
     | around the block edges copy the overlapping
     | samples to the new block position only
     | needed if the new bufpos == 0 
     */
    pub fn maybe_handle_wrap(&mut self,stereo: bool) {

        if self.blitter.bufpos == 0 {

            const SZ: usize = FIR_IPOL_N >> 2;

            unsafe {

                let mut dcoverlap: [__m128; SZ] = [z128![]; SZ];
                let mut overlap_l:  [__m128; SZ] = [z128![]; SZ];
                let mut overlap_r:  [__m128; SZ] = [z128![]; SZ];

                let zero: __m128 = _mm_setzero_ps();

                for k in (0..FIR_IPOL_N).step_by(4) {

                    overlap_l[k >> 2] = _mm_load_ps(&mut self.blitter.oscbuffer_l[OB_LENGTH + k] as *mut f32);
                    _mm_store_ps(&mut self.blitter.oscbuffer_l[k], overlap_l[k >> 2]);
                    _mm_store_ps(&mut self.blitter.oscbuffer_l[OB_LENGTH + k], zero);

                    dcoverlap[k >> 2] = _mm_load_ps(&mut self.blitter.dcbuffer[OB_LENGTH + k] as *mut f32);
                    _mm_store_ps(&mut self.blitter.dcbuffer[k], dcoverlap[k >> 2]);
                    _mm_store_ps(&mut self.blitter.dcbuffer[OB_LENGTH + k], zero);

                    if stereo {
                        overlap_r[k >> 2] = _mm_load_ps(&mut self.blitter.oscbuffer_r[OB_LENGTH + k] as *mut f32);
                        _mm_store_ps(&mut self.blitter.oscbuffer_r[k], overlap_r[k >> 2]);
                        _mm_store_ps(&mut self.blitter.oscbuffer_r[OB_LENGTH + k], zero);
                    }
                }
            }
        }
    }
}
