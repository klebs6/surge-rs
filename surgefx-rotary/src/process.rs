crate::ix!();

impl StereoProcess for RotarySpeaker {

    fn stereo_process<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]

    ) -> Result<(),SurgeError> {

        self.set_lfos();

        let precalc0: f32 = -2.0 - (self.lfo.i as f32);
        let precalc1: f32 = -1.0 - (self.lfo.r as f32);
        let precalc2: f32 = 1.0 - (self.lfo.r as f32);
        let len_l:     f32 = (precalc0 * precalc0 + precalc1 * precalc1).sqrt();
        let len_r:     f32 = (precalc0 * precalc0 + precalc2 * precalc2).sqrt();

        let delay:    f32 = 
            self.srunit.samplerate() 
            * 0.0018 
            * self.pvalf(RotarySpeakerParam::Doppler);

        self.d_l.new_value(delay * len_l);
        self.d_r.new_value(delay * len_r);

        let dotp_l: f32 = (precalc1 * (self.lfo.r as f32) + precalc0 * (self.lfo.i as f32)) / len_l;
        let dotp_r: f32 = (precalc2 * (self.lfo.r as f32) + precalc0 * (self.lfo.i as f32)) / len_r;

        let a: f32 = self.pvalf(RotarySpeakerParam::AmpMod) * 0.6;

        self.hornamp[0].new_value((1.0 - a) + a * dotp_l);
        self.hornamp[1].new_value((1.0 - a) + a * dotp_r);

        self.lfo.process();

        //upper,lower,lower_sub
        let mut wetblock = WetBlockULS::<BLOCK_SIZE>::default();
        let mut tbuffer  = TBuffer::new(BLOCK_SIZE);

        for k in 0..BLOCK_SIZE {
            self.do_rotaryspeaker_pre_block(
                k, 
                &mut wetblock, 
                data_l, 
                data_r);
        }

        unsafe {
            self.xover.process_block_mono(
                wetblock.l.as_mut_ptr(), 
                None);
        }

        for k in 0..BLOCK_SIZE {
            self.do_rotaryspeaker_block(
                k, 
                &mut tbuffer, 
                &mut wetblock);
        }

        // f_rotor_lp[0][0] = 3.0f + dotp_l;
        // f_rotor_lp[1][0] = 3.0f + dotp_r;

        // rotor_lp[0]->process(tbufferL,0);
        // rotor_lp[1]->process(tbufferR,0);
        // lowbass->process(lower_sub,0);
        unsafe {
            self.lowbass.process_block_mono(
                wetblock.s.as_mut_ptr(), 
                None
            );
        }

        for k in 0..BLOCK_SIZE {
            self.do_rotaryspeaker_post_block(
                k, 
                &mut wetblock, 
                &mut tbuffer, 
                data_l, 
                data_r);
        }

        self.wpos += BLOCK_SIZE as i32;
        self.wpos &= ROTARY_SPEAKER_MAX_DELAY_LENGTH as i32 - 1;

        Ok(())
    }
}

impl ProcessOnlyControl for RotarySpeaker {

    fn process_only_control<const N: usize>(&mut self) {

        self.set_lfos();

        self.lfo.process();
        self.lf_lfo.process();
    }
}
