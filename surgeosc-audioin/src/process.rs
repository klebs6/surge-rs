ix!();

use crate::{
    AudioInputOscillator,
    AudioInputOscillatorParam,
};

impl OscillatorProcess for AudioInputOscillator {

    fn process_block(&mut self, cfg: OscillatorProcessBlockCfg) { 
        match cfg.stereo {
            true  => self.process_block_stereo(cfg),
            false => self.process_block_mono(cfg),
        };
    }
}

impl AudioInputOscillator {

    fn process_block_mono(&mut self, _cfg: OscillatorProcessBlockCfg) { 

        let g:   f32 = self.tables.db_to_linear(self.pvalf(AudioInputOscillatorParam::Gain));
        let inp: f32 = limit_range(self.pvalf(AudioInputOscillatorParam::Input), -1.0, 1.0);

        let a: f32 = g * (1.0 - inp);
        let b: f32 = g * (1.0 + inp);

        for k in 0..BLOCK_SIZE_OS {
            self.out.l[k] = 
                a * self.synth_in.audio_in0(k) +
                b * self.synth_in.audio_in1(k);
        }
    }

    fn process_block_stereo(&mut self, _cfg: OscillatorProcessBlockCfg) { 

        let g:   f32 = self.tables.db_to_linear(self.pvalf(AudioInputOscillatorParam::Gain));
        let inp: f32 = limit_range(self.pvalf(AudioInputOscillatorParam::Input), -1.0, 1.0);

        let a: f32 = g * (1.0 - inp);
        let b: f32 = g * (1.0 + inp);

        for k in 0..BLOCK_SIZE_OS
        {
            self.out.l[k] = a * self.synth_in.audio_in0(k);
            self.out.r[k] = b * self.synth_in.audio_in1(k);
        }
    }
}
