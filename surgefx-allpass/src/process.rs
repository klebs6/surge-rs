crate::ix!();

impl StereoProcess for crate::AllpassVerb {

    fn stereo_process<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]

    ) -> Result<(),SurgeError> {

        let scale: f32 = 2.0_f32.powf(1.0 * pvalf![self.params[AllpassReverbParam::RoomSize]]);

        self.calc_size(scale);

        let decay_time = self.pvalf(AllpassReverbParam::DecayTime);

        if (decay_time - self.last_decay_time).abs() > 0.001
        {
            self.update_rtime::<N>();
        }

        self.last_decay_time = decay_time;

        let samplerate      = self.srunit.samplerate();
        let dsamplerate_inv = self.srunit.dsamplerate_inv();

        let loop_time_s: f32 = 0.5508 * scale;

        let decay: f32 = (db60![] as f32).powf(loop_time_s / (4.0 * (2.0_f32.powf(decay_time))));

        self.decay_multiply.new_value(decay);
        self.diffusion.new_value(0.7 * self.pvalf(AllpassReverbParam::Diffusion));
        self.buildup.new_value(0.7 * self.pvalf(AllpassReverbParam::BuildUp));

        self.hf_damp_coefficient.new_value(0.8 * self.pvalf(AllpassReverbParam::HFDamping));
        self.lf_damp_coefficient.new_value(0.2 * self.pvalf(AllpassReverbParam::LFDamping));

        self.modulation.new_value(
            self.pvalf(AllpassReverbParam::Modulation) * samplerate * 0.001 * 5.0
        );

        let mix   = pvalf![self.params[AllpassReverbParam::Mix]];
        let width = pvalf![self.params[AllpassReverbParam::Width]];

        self.mix.set_target_smoothed(mix);
        self.width.set_target_smoothed(width);

        self.lfo.set_rate(2.0 * PI * 2.0_f64.powf(-2.0) * dsamplerate_inv);

        let pdt: i32 = ( samplerate * 2.0_f32.powf( self.pvalf(AllpassReverbParam::PreDelay))) as i32;

        let mut wet = WetBlock::new(N);

        for k in 0..N {
            self.do_process_block(k, pdt, &mut wet, data_l, data_r);
        }

        // scale width
        let mut mid_side = MSBlock::new(N);

        unsafe {
            encode_mid_side(
                wet.l.as_mut_ptr(), 
                wet.r.as_mut_ptr(), 
                mid_side.m.as_mut_ptr(), 
                mid_side.s.as_mut_ptr(), 
                N >> 2);

            self.width.multiply_block(
                mid_side.s.as_mut_ptr(), 
                N >> 2);

            decode_mid_side(
                mid_side.m.as_mut_ptr(), 
                mid_side.s.as_mut_ptr(), 
                wet.l.as_mut_ptr(), 
                wet.r.as_mut_ptr(), 
                N >> 2);

            self.mix.fade_2_blocks_to(
                data_l.as_mut_ptr(), 
                wet.l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                wet.r.as_mut_ptr(), 
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                N >> 2);
        }

        Ok(())
    }
}
