crate::ix!();

impl OscillatorProcess for SampleAndHoldOscillator {

    ///defaults: float drift = 0.0, bool stereo = false, bool fm = false, float self.fm_depth = 0.0
    fn process_block(&mut self, cfg: OscillatorProcessBlockCfg)
    {
        let stereo = cfg.stereo;
        let pitch0 = cfg.pitch;
        let drift  = cfg.drift;
        let depth  = cfg.fm_depth;
        let fm     = cfg.fm;

        self.pitch = minf(148.0, pitch0);
        self.drift = drift;
        self.blitter.pitchmult_inv = maxd(
            1.0_f64, 
            self.srunit.dsamplerate_os()
            * (1.0 / 8.175798915) 
            * self.tuner.n2pinv::<f64,false>(self.pitch as f64)
        ) as f32;

        // This must be a real division, reciprocal-approximation is not precise enough
        self.blitter.pitchmult = 1.0 / self.blitter.pitchmult_inv;

        // if (fm) self.fm_depth.new_value(depth);
        self.update_lagvals::<false>();

        self.l_pw.process();
        self.l_shape.process();
        self.l_smooth.process();
        self.l_sub.process();
        self.l_sync.process();

        // self.hpf_coeff.process();
        // self.integrator_mult.process();

        if fm {

            for l in 0_usize..(self.blitter.n_unison as usize) {
                self.blitter.driftlfo[l] = drift_noise(self.blitter.driftlfo2[l]);
            }

            for s in 0..BLOCK_SIZE_OS {

                let master_osc: f32 = unsafe { *self.master_osc.add(s) };

                let fmmul: f32 = limit_range(1.0 + depth * master_osc, 0.1, 1.9);
                let a: f32 = self.blitter.pitchmult * fmmul;

                self.fm_delay = s as i32;

                for l in (0_usize..self.blitter.n_unison as usize).step_by(1) {

                    while self.blitter.oscstate[l] < a {

                        self.fm_mul_inv = rcp(fmmul);

                        self.convolute(
                            ConvolutionCfg{ voice: l, fm: true, stereo } 
                        );

                    }

                    self.blitter.oscstate[l] -= a;
                }
            }

        } else {

            let a: f32 = (BLOCK_SIZE_OS as f32) * self.blitter.pitchmult;

            for l in 0_usize..(self.blitter.n_unison as usize) {

                self.blitter.driftlfo[l] = drift_noise(self.blitter.driftlfo2[l]);

                while (self.blitter.syncstate[l] < a) || (self.blitter.oscstate[l] < a)
                {
                    self.convolute(
                        ConvolutionCfg{ voice: l, fm: false, stereo }
                    );
                }

                self.blitter.oscstate[l] -= a;

                if self.l_sync.v > 0.0 {
                    self.blitter.syncstate[l] -= a;
                }
            }

        }

        let mut hpfblock = WetBlock1::<BLOCK_SIZE_OS>::default();

        unsafe {

            self.li_hpf.store_block(hpfblock.buf.as_mut_ptr(), BLOCK_SIZE_OS_QUAD);

            let mdc:    __m128 = _mm_load_ss(&self.dc);
            let mut oa: __m128 = _mm_load_ss(&self.blitter.out_attenuation);

            oa = _mm_mul_ss(oa, _mm_load_ss(&self.blitter.pitchmult));

            for k in (0..BLOCK_SIZE_OS).step_by(1) 
            {
                let hpf: __m128 = _mm_load_ss(&hpfblock.buf[k]);
                let mut ob: __m128 = _mm_load_ss(&self.blitter.oscbuffer_l[(self.blitter.bufpos + (k as i32)) as usize]);
                let mut a: __m128 = _mm_mul_ss(self.blitter.osc_out_l, hpf);

                ob = _mm_sub_ss(ob, _mm_mul_ss(mdc, oa));
                self.blitter.osc_out_l = _mm_add_ss(a, ob);

                _mm_store_ss(&mut self.out.l[k], self.blitter.osc_out_l);

                if stereo {

                    ob = _mm_load_ss(&self.blitter.oscbuffer_r[(self.blitter.bufpos + (k as i32)) as usize]);
                    a = _mm_mul_ss(self.blitter.osc_out_r, hpf);
                    ob = _mm_sub_ss(ob, _mm_mul_ss(mdc, oa));

                    self.blitter.osc_out_r = _mm_add_ss(a, ob);
                    _mm_store_ss(&mut self.out.r[k], self.blitter.osc_out_r);
                }
            }
            _mm_store_ss(&mut self.dc, mdc);
        }

        self.clear_blocks(stereo);

        self.blitter.bufpos = (self.blitter.bufpos + BLOCK_SIZE_OS as i32) & (OB_LENGTH as i32 - 1);

        // each block overlap FIR_IPOL_N samples
        // into the next (due to impulses not
        // being wrapped around the block edges
        // copy the overlapping samples to the new
        // block position
        //
        // only needed if the new
        // self.blitter.bufpos == 0
        if self.blitter.bufpos == 0  {

            let mut overlap_l = unsafe { [z128![]; FIR_IPOL_N >> 2] };
            let mut overlap_r = unsafe { [z128![]; FIR_IPOL_N >> 2] };

            for k in (0..FIR_IPOL_N).step_by(4) 
            {
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
