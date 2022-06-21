crate::ix!();

impl Flanger {

    pub fn do_flanger_block<const N: usize>(&mut self, 
        block_idx: usize, 
        mtype:     FlangerType,
        combs:     &mut WetBlock1Dual::<BLOCK_SIZE>,
        vweights:  &mut WetBlock1Dual::<FLANGER_COMBS_PER_CHANNEL>,
        data_l:    &mut [f32; N],
        data_r:    &mut [f32; N]) 
    {
        for channel_idx in 0_usize..2 {

            combs.buf[channel_idx][block_idx] = 0.0;

            for i in 0..FLANGER_COMBS_PER_CHANNEL {

                if vweights.buf[channel_idx][i as usize] > 0.0 {

                    let tap: f32 = 
                        self.delaybase[[ channel_idx, i as usize ]].v * 
                        ( 1.0 + self.lfoval[[channel_idx, i as usize]].v * self.depth.v ) + 1.0;

                    let v:   f32 = self.idels[channel_idx].value(tap);

                    combs.buf[channel_idx][block_idx] += vweights.buf[channel_idx][i as usize] * v;
                }

                self.lfoval[[ channel_idx, i ]].process();
                self.delaybase[[ channel_idx, i ]].process();
            }
        }

        // softclip the feedback to avoid explosive runaways
        let mut fbl: f32 = 0.0;
        let mut fbr: f32 = 0.0;

        if self.feedback.v > 0.0 {

            fbl = limit_range( self.feedback.v * combs.buf[0][block_idx], -1.0, 1.0 );
            fbr = limit_range( self.feedback.v * combs.buf[1][block_idx], -1.0, 1.0 );

            fbl = 1.5 * fbl - 0.5 * fbl * fbl * fbl;
            fbr = 1.5 * fbr - 0.5 * fbr * fbr * fbr;

            // and now we have clipped, apply the damping. 
            // FIXME - move to one mul form
            self.onepole_state.lpa_l = 
                self.onepole_state.lpa_l * ( 1.0 - self.fb_lf_damping.v ) + 
                fbl * self.fb_lf_damping.v;

            fbl -= self.onepole_state.lpa_l;

            self.onepole_state.lpa_r = 
                self.onepole_state.lpa_r * ( 1.0 - self.fb_lf_damping.v ) + 
                fbr * self.fb_lf_damping.v;

            fbr -= self.onepole_state.lpa_r;
        }

        let (vl, vr) = {
            let vl: f32 = data_l[block_idx] - fbl;
            let vr: f32 = data_r[block_idx] - fbr;
            (vl, vr)
        };

        self.idels[0].push( vl );
        self.idels[1].push( vr );

        let mut origw: f32 = 1.0;

        if mtype == FlangerType::Doppler {
            // doppler modes
            origw = 0.0;
        }

        let (mut outl, mut outr) = {

            let outl: f32 = origw * data_l[block_idx] + 
                self.mix.v * combs.buf[0][block_idx];

            let outr: f32 = origw * data_r[block_idx] + 
                self.mix.v * combs.buf[1][block_idx];

            (outl, outr)
        };

        outl = limit_range( (1.0 + self.gain.v ) * outl, -1.0, 1.0 );
        outr = limit_range( (1.0 + self.gain.v ) * outr, -1.0, 1.0 );

        outl = 1.5 * outl - 0.5 * outl * outl * outl;
        outr = 1.5 * outr - 0.5 * outr * outr * outr;

        data_l[block_idx] = outl;
        data_r[block_idx] = outr;

        self.depth.process();
        self.mix.process();
        self.feedback.process();
        self.fb_lf_damping.process();
        self.gain.process();
    }
}
