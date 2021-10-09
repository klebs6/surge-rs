ix!();

use crate::{
    RotarySpeaker,
    RotarySpeakerParam,
};

impl RotarySpeaker<'sr> {

    #[inline] pub fn set_lfos(&mut self) {

        let frate: f32 = 
            self.pvalf(RotarySpeakerParam::HornRate) *
            self.maybe_temposyncratio(RotarySpeakerParam::HornRate);

        let x = 2.0 * 
            PI_32 * 
            2.0_f32.powf(frate) * 
            self.srunit.samplerate_inv();

        self.lfo.set_rate(
            (x * BLOCK_SIZE as f32).into()
        );

        self.lf_lfo.set_rate(
            (0.7 * x * BLOCK_SIZE as f32).into()
        );
    }
}
