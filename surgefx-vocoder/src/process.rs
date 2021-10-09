ix!();

use crate::{
    Vocoder,
    VocoderParam,
    VocoderBlockCfg,
};

impl Vocoder<'sr> {
    pub fn gain(&self) -> f32 {
        let f_gain: f32 = self.pvalf(VocoderParam::Gain);
        let gain:   f32 = f_gain + 24.0;
        gain
    }

    pub fn set_gain(&mut self) {
        let gain = self.gain();
        self.gain.set_target_smoothed(self.tables.db_to_linear(gain));
    }
}

impl Process for Vocoder<'sr> {

    fn process<const N: usize>(&mut self, data_l: &mut [f32; N], data_r: &mut [f32; N]) {

        self.bi = (self.bi + 1) & 0x3f;

        if self.bi == 0 {
            self.update();
        }

        let mut modulator_in = WetBlock1::<BLOCK_SIZE>::default();

        add_block(
            self.synth_in.non_os_audio_in0_ptr(0), 
            self.synth_in.non_os_audio_in1_ptr(0), 
            modulator_in.buf.as_mut_ptr(), 
            BLOCK_SIZE_QUAD);

        self.set_gain();

        unsafe {
            self.gain.multiply_block(modulator_in.buf.as_mut_ptr(), BLOCK_SIZE_QUAD);
        }

        let cfg = VocoderBlockCfg::new(self);

        for k in 0..BLOCK_SIZE {
            self.do_vocoder_block(k, &mut modulator_in, &cfg, data_l, data_r);
        }
    }
}
