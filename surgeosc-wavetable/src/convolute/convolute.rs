ix!();

use crate::{
    WTOscillator,
    WTOscillatorParam,
};

impl WTOscillator {

    pub fn convolute(&mut self, voice: i32, fm: bool, stereo: bool) 
    {
        let vidx = voice as usize;

        let block_pos: f32 = 
            self.blitter.oscstate[vidx] * 
            (BLOCK_SIZE_OS_INV as f32) * 
            self.blitter.pitchmult_inv;

        let mut detune: f64 = (self.drift * 
            self.blitter.driftlfo[vidx]) as f64;

        if self.blitter.n_unison > 1 {
            detune += (self.pvalf_extended(WTOscillatorParam::UniSpread) as f64) *
                (self.blitter.detune_bias * (voice as f32) + self.blitter.detune_offset) as f64;
        }

        let ipos = self.get_ipos(fm,vidx);

        if self.blitter.state[vidx] == 0 {
            self.do_blitstate_zero_for_convolute(vidx);
        }

        // generate pulse
        let delay: u32 = match fm {
            true  => self.fm_delay as u32,
            false => ((ipos >> 24) & 0x3f) as u32,
        };

        let wt_inc: i32 = 1 << self.mipmap[vidx];

        let mut dt: f32 = self.wave_wavetable.dt() * (wt_inc as f32);

        // add time until next statechange
        let tempt: f64 = match self.params[WTOscillatorParam::UniSpread].absolute 
        {
            /*TODO: is UniSpread the right parameter? match with p[5] from the C*/
            true => {
                // See the comment in SurgeSuperOscillator.cpp at the absolute treatment
                let tempt = self.tuner.n2pinv::<f64,true>( 
                    detune * self.tuner.n2pinv::<f64,true>( self.pitch_t as f64) * 16.0 / 0.9443 
                );

                maxd(tempt,0.1)
            },
            false => {
                self.tuner.n2pinv_tuningctr(detune)
            },
        };

        let xt: f32 = {
            let mut xt: f32 = ((self.blitter.state[vidx] as f32) + 0.50) * dt;
            // xt = (1 - self.hskew + 2*self.hskew*xt);
            // xt = (1 + self.hskew *sin(xt*2.0*M_PI));

            // 1 + a.*(1 - 2.*x + (2.*x-1).^3).*sqrt(27/4) = 1 + 4*x*a*(x-1)*(2x-1)
            let taylorscale: f32 = (27.0_f32 / 4.0).sqrt();
            xt = 1.0 + self.hskew * 4.0 * xt * (xt - 1.0) * (2.0 * xt - 1.0) * taylorscale;
            xt
        };

        let formant: f32 = {
            let ft: f32 =  lerp(block_pos, self.formant_last, self.formant_t);
            self.tuner.n2p_tuningctr(-ft)
        };

        dt *= formant * xt;

        let wtsize: usize = self.wave_wavetable.num_samples_per_table() >> self.mipmap[vidx];

        if self.blitter.state[vidx] >= ((wtsize - 1)  as i32){
            dt += 1.0 - formant;
        }

        let t: f32 = dt * (tempt as f32);

        self.blitter.state[vidx] &= (wtsize - 1) as i32;

        let newlevel: f32 = self.get_newlevel(vidx, block_pos);

        let mut g: f32 = newlevel - self.last_level[vidx];

        self.last_level[vidx] = newlevel;

        g *= self.blitter.out_attenuation;

        if stereo {

            //unused variable
            //let g_r: f32 = g * self.blitter.pan_r[vidx];
            
            g *= self.blitter.pan_l[vidx];
        }

        match stereo {
            true  => self.convolute_stereo(g, delay, ipos),
            false => self.convolute_mono(g, delay, ipos),
        }

        self.blitter.rate[vidx]      = t;
        self.blitter.oscstate[vidx] += self.blitter.rate[vidx];
        self.blitter.oscstate[vidx]  = maxf(0.0, self.blitter.oscstate[vidx]);
        self.blitter.state[vidx]     = (self.blitter.state[vidx] + 1) & 
            (((self.wave_wavetable.num_samples_per_table() >> self.mipmap[vidx]) - 1) as i32);
    }
}
