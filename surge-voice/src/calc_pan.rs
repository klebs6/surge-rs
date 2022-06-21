crate::ix!();

impl SurgeVoice {

    pub fn calc_pan(&mut self, 
        cfg: VoiceRuntimeHandle,
        mut qfcs: Option<&mut QuadFilterChainState>, 
        e: i32) 
    {
        let cfg = cfg.borrow();

        let pan_id_f    = cfg.pan;
        let width_id_f  = cfg.width;
        let volume_id_f = cfg.volume;

        let mut pan1: f32 = limit_range(
            pan_id_f + 
            unsafe { (*self.state.voice_channel_state).pan } + 
            unsafe { (*self.state.main_channel_state).pan  },  
            -1.0, 1.0);

        // the *0.5 multiplication will be eliminated
        // by the 2x gain of the halfband filter
        let mut amp: f32 =
            0.5 * amp_to_linear(volume_id_f); 

        let fbc = cfg.filterblock_cfg;

        // Volume correcting/correction 
        // (fb_stereo updated since v1.2.2)
        match fbc {
            FilterBlockConfiguration::Wide => {
                amp *= 0.6666666;
            },
            FilterBlockConfiguration::Stereo => {
                amp *= 1.3333333;
            },
            _ => {},
        }

        if fbc.is_wide() || fbc.is_stereo() {

            pan1 -= width_id_f;

            let pan2:  f32 = pan_id_f + width_id_f;
            let amp_2l: f32 = amp * megapan_left(pan2);
            let amp_2r: f32 = amp * megapan_right(pan2);

            if let Some(ref mut qfcs) = qfcs {
                unsafe {
                    set1f(&mut qfcs.out_2l,  e, self.fbp.out_2l);
                    set1f(&mut qfcs.dout_2l, e, (amp_2l - self.fbp.out_2l) * BLOCK_SIZE_OS_INV);
                    set1f(&mut qfcs.out_2r,  e, self.fbp.out_2r);
                    set1f(&mut qfcs.dout_2r, e, (amp_2r - self.fbp.out_2r) * BLOCK_SIZE_OS_INV);
                }
            }

            self.fbp.out_2l = amp_2l;
            self.fbp.out_2r = amp_2r;
        }

        let amp_l: f32 = amp * megapan_left(pan1);
        let amp_r: f32 = amp * megapan_right(pan1);

        if let Some(ref mut qfcs) = qfcs {
            unsafe {
                set1f(&mut qfcs.out_l,  e, self.fbp.out_l);
                set1f(&mut qfcs.dout_l, e, (amp_l - self.fbp.out_l) * BLOCK_SIZE_OS_INV);
                set1f(&mut qfcs.out_r,  e, self.fbp.out_r);
                set1f(&mut qfcs.dout_r, e, (amp_r - self.fbp.out_r) * BLOCK_SIZE_OS_INV);
            }
        }

        self.fbp.out_l = amp_l;
        self.fbp.out_r = amp_r;
    }
}
