ix!();

pub struct SSOBlockCfg<'a> {
    pub hpfblock: &'a WetBlock1::<BLOCK_SIZE_OS>,
    pub k:        usize, 
    pub stereo:   bool, 
    pub mdc:      &'a mut __m128, 
    pub oa:       &'a mut __m128, 
    pub char_a1:  &'a __m128, 
    pub char_b0:  &'a __m128, 
    pub char_b1:  &'a __m128
}

impl crate::SurgeSuperOscillator {

    pub fn do_block(&mut self, cfg: SSOBlockCfg<'_>) {

        unsafe {

            let bufpos = ((self.blitter.bufpos as usize) + (cfg.k as usize)) as usize;

            let dcb: __m128 = _mm_load_ss(&self.blitter.dcbuffer[bufpos]);
            let hpf: __m128 = _mm_load_ss(&cfg.hpfblock.buf[cfg.k]);
            let mut ob: __m128 = _mm_load_ss(&self.blitter.oscbuffer_l[bufpos]);

            /* a = prior output * hpf value */
            let mut a: __m128 = _mm_mul_ss(self.blitter.osc_out_l, hpf);

            /* cfg.mdc += dc level */
            *cfg.mdc = _mm_add_ss(*cfg.mdc, dcb);

            /* Output Buffer += DC * Out Attenuation */
            ob = _mm_sub_ss(ob, _mm_mul_ss(*cfg.mdc, *cfg.oa));

            /* Stow away the last output and make the new output 
               the oscbuffer + the filter controbution */
            let last_osc_out_l: __m128 = self.blitter.osc_out_l;
            self.blitter.osc_out_l = _mm_add_ss(a, ob);

            /* So at that point osc_out_l = a + ob; = prior_out * hpf + oscbuffer + dc * attenuation; */

            /* character filter (hifalloff/neutral/boost)
               This formula is out2 = out2 * cfg.char_a1 + out * cfg.char_b0 + last_out * cfg.char_b1
               which is the classic biquad formula. */

            self.blitter.osc_out_2l =
                _mm_add_ss(_mm_mul_ss(self.blitter.osc_out_2l, *cfg.char_a1),
                _mm_add_ss(_mm_mul_ss(self.blitter.osc_out_l, *cfg.char_b0), 
                    _mm_mul_ss(last_osc_out_l, *cfg.char_b1)));

            /* And so store the output of the HPF as the output */
            _mm_store_ss(&mut self.out.l[cfg.k], self.blitter.osc_out_2l);

            // And do it all again if we are stereo
            if cfg.stereo {
                ob = _mm_load_ss(&mut self.blitter.oscbuffer_r[bufpos] as *mut f32);

                a = _mm_mul_ss(self.blitter.osc_out_r, hpf);

                ob = _mm_sub_ss(ob, _mm_mul_ss(*cfg.mdc, *cfg.oa));

                let last_osc_out_r: __m128 = self.blitter.osc_out_r;

                self.blitter.osc_out_r = _mm_add_ss(a, ob);

                self.blitter.osc_out_2r = _mm_add_ss(
                    _mm_mul_ss(self.blitter.osc_out_2r, *cfg.char_a1),
                    _mm_add_ss(_mm_mul_ss(self.blitter.osc_out_r, *cfg.char_b0), 
                        _mm_mul_ss(last_osc_out_r, *cfg.char_b1)));

                _mm_store_ss(&mut self.out.r[cfg.k], self.blitter.osc_out_2r);
            }
        }
    }
}
