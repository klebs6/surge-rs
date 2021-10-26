ix!();

pub const CONVOLVE_NODC: bool = false;

use crate::SSOParam;

impl Convolute for crate::SurgeSuperOscillator {

    /**
      | I've carefully documented the non-fm
      | non-sync case here. The other cases are
      | similar. See the comment above. Remeber,
      | this function exists to calculate the next
      | impulse in our digital sequence, which
      | occurs at time 'oscstate' convolve it into
      | our output stream, and advance out phase
      | state space by the amount just covered.
      */
    fn convolute(&mut self, cfg: ConvolutionCfg) {

        let voice  = cfg.voice;
        let stereo = cfg.stereo;
        let fm     = cfg.fm;

        //TODO: why was this assertion here?
        //assert!(self.blitter.oscstate[voice] >= 0.0);

        //Detune by a combination of the LFO drify and the unison voice spread.
        let mut detune: f32 = self.drift * self.blitter.driftlfo[voice];

        if self.blitter.n_unison > 1 {
            let f: f32 = self.pvalf_extended(SSOParam::UniSpread);
            detune += f * ( self.blitter.detune_bias * 
                (voice as f32) + 
                self.blitter.detune_offset );
        }

        let wf:  f32 = self.l_shape.v;
        let sub: f32 = self.l_sub.v;
        let p24: f32 = (1 << 24) as f32;

        // ipos is a value between 0 and 2^24 indicating 
        // how far along in oscstate (phase space for ** our state) we are
        let ipos: u32;

        if self.l_sync.v > 0.0 && self.blitter.syncstate[voice] < self.blitter.oscstate[voice] {

            ipos = match fm {
                true  => p24 * (self.blitter.syncstate[voice] * self.blitter.pitchmult_inv * self.fm_mul_inv),
                false => p24 * (self.blitter.syncstate[voice] * self.blitter.pitchmult_inv),

            } as u32;

            let t: f32 = (self.tuner.n2pinv_tuningctr(detune as f64) as f32) * 2.0;

            self.blitter.state[voice] = 0;
            self.last_level[voice]    += self.dc_uni[voice] * (self.blitter.oscstate[voice] - self.blitter.syncstate[voice]);

            self.blitter.oscstate[voice]  = self.blitter.syncstate[voice];
            self.blitter.syncstate[voice] += t;
            self.blitter.syncstate[voice] = maxf(0.0, self.blitter.syncstate[voice]);

        } else {
            ipos = match fm {
                true  => p24 * (self.blitter.oscstate[voice] * self.blitter.pitchmult_inv * self.fm_mul_inv),
                false => p24 * (self.blitter.oscstate[voice] * self.blitter.pitchmult_inv),
            } as u32;
        }

        let delay = self.get_delay(fm,ipos);

        // m and lipol128 are the integer and fractional part of the number of 256ths
        // (FIRipol_N ths really) that our current position places us at. These are obviously
        // not great variable names. Especially lipolui16 doesn't seem to be fractional at all
        // it seems to range between 0 and 0xffff, but it is multiplied by the sinctable
        // derivative block (see comment above and also see the SurgeSynthesizer constructor
        // second sinctable block), which is pre-scaled down by 65536, so lipol * sinctable[j * + 1]
        // is the fractional derivative of the sinc table with repsect to time. (The calculation is
        // numerical not analytical in SurgeSynthesizer).
        let m:         i32 = (((ipos >> 16) & 0xff) as i32) * ((FIR_IPOL_N << 1) as i32);
        let lipolui16: i32 = (ipos & 0xffff).try_into().unwrap();

        let lipol128 = unsafe {
            let mut lipol128: __m128 = z128![];
            lipol128 = _mm_cvtsi32_ss(lipol128, lipolui16);
            lipol128 = _mm_shuffle_ps(lipol128, lipol128, _MM_SHUFFLE(0, 0, 0, 0));
            lipol128
        };

        let sync: f64 = mind(
            self.l_sync.v as f64, 
            (12.0_f64 + 72.0 + 72.0) - (self.pitch as f64)
        );

        let (t, t_inv) = self.get_t(sync, detune as f64);
        let (g, g_r) = self.get_g(voice, wf, sub, stereo);

        match stereo {
            true  => self.convolve_stereo(g, g_r, delay as usize, m as usize, lipol128),
            false => self.convolve_mono(g,       delay as usize, m as usize, lipol128),
        }

        if !CONVOLVE_NODC {

            let olddc: f32 = self.dc_uni[voice];

            let idx = (self.blitter.bufpos as usize) + (FIR_OFFSET as usize) + (delay as usize);

            self.dc_uni[voice] = t_inv * (1.0 + wf) * (1.0 - sub); // *pitchmult;

            self.blitter.dcbuffer[idx] += self.dc_uni[voice] - olddc;
        }

        self.blitter.rate[voice] = match self.blitter.state[voice] & 1 {
            0 => t * self.pwidth[voice],
            _ => t * (1.0 - self.pwidth[voice]),
        };

        self.blitter.rate[voice] *= match (self.blitter.state[voice] + 1) & 2 {
            0 => self.pwidth2[voice],
            _ => (2.0 - self.pwidth2[voice]),
        };

        self.blitter.oscstate[voice] += self.blitter.rate[voice];
        self.blitter.oscstate[voice]  = maxf(0.0, self.blitter.oscstate[voice]);
        self.blitter.state[voice]     = (self.blitter.state[voice] + 1) & 3;
    }
}
