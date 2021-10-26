ix!();

use crate::{
    SampleAndHoldOscillator,
    SampleAndHoldOscillatorParam,
};

//TODO: can we decompose this implementation into composable macros?
//      the goal is to share this between SSO and SNH implementations
//TODO: was p[5] UniCount? or UniSpread?
impl Convolute for SampleAndHoldOscillator {

    fn convolute(&mut self, cfg: ConvolutionCfg) {

        let voice: usize = cfg.voice;
        let stereo: bool = cfg.stereo;
        let fm:     bool = cfg.fm;

        let mut detune: f32 = self.drift * self.blitter.driftlfo[voice];

        if self.blitter.n_unison > 1 {
            detune += self.pvalf(SampleAndHoldOscillatorParam::UniCount) * 
                (self.blitter.detune_bias * (voice as f32) + self.blitter.detune_offset);
        }

        let p24: f32 = (1 << 24) as f32;

        let mut ipos: i32 = match fm {
            true  => ((p24 as f32) * (self.blitter.oscstate[voice] * self.blitter.pitchmult_inv * self.fm_mul_inv)) as i32,
            false => ((p24 as f32) * (self.blitter.oscstate[voice] * self.blitter.pitchmult_inv)) as i32,
        };

        let mut invertcorrelation: f32 = 1.0;

        if self.blitter.syncstate[voice] < self.blitter.oscstate[voice] {

            ipos = (p24 as f32 * (self.blitter.syncstate[voice] * self.blitter.pitchmult_inv)) as i32;

            let t: f32 = self.tuner.n2pinv_tuningctr(detune as f64) as f32;

            if self.blitter.state[voice] == 1 {
                invertcorrelation = -1.0;
            }

            self.blitter.state[voice] = 0;
            self.blitter.oscstate[voice] = self.blitter.syncstate[voice];
            self.blitter.syncstate[voice] += t;
        }

        let delay: u32 = match fm {
            true  => self.fm_delay as u32,
            false => ((ipos >> 24) & 0x3f) as u32,
        };

        let m: i32 = (((ipos >> 16) & 0xff) as i32) * ((FIR_IPOL_N << 1) as i32);

        let lipolui16: i32 = ipos & 0xffff;

        //TODO: macro this
        let lipol128 = unsafe {
            let mut lipol128: __m128 = z128![];
            lipol128 = _mm_cvtsi32_ss(lipol128, lipolui16);
            lipol128 = _mm_shuffle_ps(lipol128, lipol128, _MM_SHUFFLE(0, 0, 0, 0));
            lipol128
        };

        // add time until next statechange
        let t: f32 = {
            if self.params[SampleAndHoldOscillatorParam::UniCount].absolute {

                // see the comment in SurgeSuperOscillator in the absolute branch
                let t = self.tuner.n2pinv::<f32,true>(
                    detune * self.tuner.n2pinv::<f32,true>( self.pitch ) * 
                    16.0 / 0.9443 
                );

                maxf(0.1,t)

            } else {
                self.tuner.n2pinv_tuningctr((detune as f64) + self.l_sync.v) as f32
            }
        };

        let mut g:     f32;
        let mut g_r:    f32 = 0.0;
        let wf:        f32 = (self.l_shape.v * 0.8 * (invertcorrelation as f64)) as f32;
        let wfabs:     f32 = wf.abs();
        let rand11:    f32 = rand11();
        let mut randt: f32 = rand11 * (1.0 - wfabs) - wf * self.last_level[voice];

        randt *= rcp(1.0 - wfabs);
        randt = minf(0.5, maxf(-0.5, randt));

        if self.blitter.state[voice] == 0 {
            self.pwidth[voice] = self.l_pw.v as f32;
        }

        g = randt - self.last_level[voice];
        self.last_level[voice] = randt;

        g *= self.blitter.out_attenuation;

        if stereo {
            g_r = g * self.blitter.pan_r[voice];
            g *= self.blitter.pan_l[voice];
        }

        if stereo {
            unsafe {

                let mut g128_l: __m128 = _mm_load_ss(&g);
                g128_l = _mm_shuffle_ps(g128_l, g128_l, _MM_SHUFFLE(0, 0, 0, 0));

                let mut g128_r: __m128 = _mm_load_ss(&g_r);
                g128_r = _mm_shuffle_ps(g128_r, g128_r, _MM_SHUFFLE(0, 0, 0, 0));

                for k in (0..FIR_IPOL_N).step_by(4) {

                    let idx: usize = (self.blitter.bufpos as usize) + (k as usize) + (delay as usize);

                    let sincidx:  usize = m as usize + k;
                    let sincidx2: usize = (m as usize + k) + FIR_IPOL_N;

                    let obf_l = &mut self.blitter.oscbuffer_l[idx];
                    let obf_r = &mut self.blitter.oscbuffer_r[idx];

                    let mut ob_l: __m128 = _mm_loadu_ps(obf_l);
                    let mut ob_r: __m128 = _mm_loadu_ps(obf_r);
                    let mut st:   __m128 = _mm_load_ps(&mut self.tables.sinctable(sincidx) as *mut f32);
                    let mut so:   __m128 = _mm_load_ps(&mut self.tables.sinctable(sincidx2) as *mut f32);

                    so = _mm_mul_ps(so, lipol128);

                    st   = _mm_add_ps(st, so);
                    ob_l = _mm_add_ps(ob_l, _mm_mul_ps(st, g128_l));

                    _mm_storeu_ps(obf_l, ob_l);

                    ob_r = _mm_add_ps(ob_r, _mm_mul_ps(st, g128_r));

                    _mm_storeu_ps(obf_r, ob_r);
                }
            }
        } else {
            unsafe {

                let mut g128: __m128 = _mm_load_ss(&g);
                g128 = _mm_shuffle_ps(g128, g128, _MM_SHUFFLE(0, 0, 0, 0));

                for k in (0..FIR_IPOL_N).step_by(4) {

                    let idx: usize = (self.blitter.bufpos as usize) + (k as usize) + (delay as usize);
                    let sincidx: usize = m as usize + k;
                    let sincidx2: usize = m as usize + k + FIR_IPOL_N;

                    let obf = &mut self.blitter.oscbuffer_l[idx];

                    let mut ob: __m128 = _mm_loadu_ps(obf);
                    let mut st: __m128 = _mm_load_ps(&mut self.tables.sinctable(sincidx) as *mut f32);
                    let mut so: __m128 = _mm_load_ps(&mut self.tables.sinctable(sincidx2) as *mut f32);

                    so = _mm_mul_ps(so, lipol128);
                    st = _mm_add_ps(st, so);
                    st = _mm_mul_ps(st, g128);
                    ob = _mm_add_ps(ob, st);

                    _mm_storeu_ps(obf, ob);
                }
            }
        }

        if (self.blitter.state[voice] & 1) != 0 {
            self.blitter.rate[voice] = t * (1.0 - self.pwidth[voice]);
        } else {
            self.blitter.rate[voice] = t * self.pwidth[voice];
        }

        self.blitter.oscstate[voice] += self.blitter.rate[voice];
        self.blitter.oscstate[voice] = maxf(0.0, self.blitter.oscstate[voice]);
        self.blitter.state[voice] = (self.blitter.state[voice] + 1) & 1;
    }
}
