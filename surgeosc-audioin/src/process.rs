crate::ix!();

impl OscillatorProcess for AudioInputOscillator {

    fn process_block(&mut self, cfg: OscillatorProcessBlockCfg) 
        -> Result<(),SurgeError> 
    {
        match cfg.stereo {
            true  => self.process_block_stereo(cfg),
            false => self.process_block_mono(cfg),
        }
    }
}

impl AudioInputOscillator {

    fn process_block_mono(&mut self, _cfg: OscillatorProcessBlockCfg) 
        -> Result<(),SurgeError> 
    {
        let g: f32 = {
            let gain = self.pvalf(AudioInputOscillatorParam::Gain);
            self.tables.db_to_linear(gain)
        };

        let inp: f32 = {
            let input = self.pvalf(AudioInputOscillatorParam::Input);
            limit_range(input, -1.0, 1.0)
        };

        let a: f32 = g * (1.0 - inp);
        let b: f32 = g * (1.0 + inp);

        for k in 0..BLOCK_SIZE_OS {

            let in0 = self.synth_in.audio_in0(k);
            let in1 = self.synth_in.audio_in1(k);

            self.out.l[k] = a * in0 + b * in1;
        }

        Ok(())
    }

    fn process_block_stereo(&mut self, _cfg: OscillatorProcessBlockCfg) -> Result<(),SurgeError> {

        let g:   f32 = {
            let gain = self.pvalf(AudioInputOscillatorParam::Gain);
            self.tables.db_to_linear(gain)
        };

        let inp: f32 = {
            let input = self.pvalf(AudioInputOscillatorParam::Input);
            limit_range(input, -1.0, 1.0)
        };

        let a: f32 = g * (1.0 - inp);
        let b: f32 = g * (1.0 + inp);

        for k in 0..BLOCK_SIZE_OS
        {
            self.out.l[k] = a * self.synth_in.audio_in0(k);
            self.out.r[k] = b * self.synth_in.audio_in1(k);
        }

        Ok(())
    }
}
