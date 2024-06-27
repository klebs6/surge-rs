crate::ix!();

impl Flanger {

    fn adjust_longphase(&mut self, rate: f32) {

        self.longphase += rate;

        if self.longphase >= (FLANGER_COMBS_PER_CHANNEL as f32) {
            self.longphase -= FLANGER_COMBS_PER_CHANNEL as f32;
        }
    }
}

impl StereoProcess for Flanger {

    fn stereo_process<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]

    ) -> Result<(),SurgeError> {

        // So here is a flanger with everything fixed
        let rate_temposyncratio = self.maybe_temposyncratio(FlangerParam::Rate);
        let rate_raw            = self.pvalf(FlangerParam::Rate);

        let rate: f32 = self.tables.envelope_rate_linear( - rate_raw ) * rate_temposyncratio;

        let mode:           usize = pvali![self.params[FlangerParam::Mode]] as usize;
        let v0:             f32   = pvalf![self.params[FlangerParam::VoiceZeroPitch]];
        let depth:          f32   = pvalf![self.params[FlangerParam::Depth]];
        let mix:            f32   = pvalf![self.params[FlangerParam::Mix]];
        let mut fbv:        f32   = pvalf![self.params[FlangerParam::Feedback]];
        let fb_lf_damping:  f32   = pvalf![self.params[FlangerParam::FbLFDamping]];
        let gain:           f32   = pvalf![self.params[FlangerParam::Gain]];

        let mtype: FlangerType = (mode / 4).try_into().unwrap();
        let mwave: FlangerWave = (mode % 4).try_into().unwrap();

        let fbscale = mtype.feedback_scale();

        self.adjust_longphase(rate);

        for channel_idx in 0_usize..2_usize {
            for comb_idx in 0_usize..FLANGER_COMBS_PER_CHANNEL {
                self.init_comb(mtype, mwave, v0, rate, channel_idx, comb_idx);
            }
        }

        self.depth.new_value( depth );
        self.mix.new_value( mix );

        fbv = fbv * fbv * fbv;

        self.feedback.new_value( fbscale * fbv ); 
        self.fb_lf_damping.new_value( 0.4 * fb_lf_damping);
        self.gain.new_value( gain );


        // Obviously when we implement stereo spread this will be different
        let mut vweights = WetBlock1Dual::<FLANGER_COMBS_PER_CHANNEL>::default();

        for channel_idx in 0_usize..2 {

            self.fill_weights(channel_idx, mtype, &mut vweights);
        }

        let mut combs = WetBlock1Dual::<BLOCK_SIZE>::default();

        for b in 0_usize..(BLOCK_SIZE as usize) {

            self.do_flanger_block(b, mtype, &mut combs, &mut vweights, data_l, data_r);

        }

        Ok(())
    }
}
