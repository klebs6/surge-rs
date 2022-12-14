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
        self.process_lfos_and_delays_for_each_comb(block_idx, combs, vweights);

        let (fbl, fbr) = self.softclip_the_feedback_to_avoid_explosions(block_idx,combs);

        let (vl, vr) = {
            let vl: f32 = data_l[block_idx] - fbl;
            let vr: f32 = data_r[block_idx] - fbr;
            (vl, vr)
        };

        self.idels[0].push( vl );
        self.idels[1].push( vr );

        let origw: f32 = match mtype {
            FlangerType::Doppler => 0.0,
            _                    => 1.0,
        };

        let (mut outl, mut outr) = {

            let outl: f32 = {
                let a = origw;
                let x = data_l[block_idx];
                let b = self.mix.v;
                let y = combs.buf[0][block_idx];

                a * x + b * y
            };

            let outr: f32 = {
                let a = origw;
                let x = data_r[block_idx];
                let b = self.mix.v;
                let y = combs.buf[1][block_idx];

                a * x + b * y
            };

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
