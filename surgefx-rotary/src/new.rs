ix!();

use crate::{
    RotarySpeaker,
    RotarySpeakerParam,
    ROTARY_SPEAKER_MAX_DELAY_LENGTH,
    ROTARY_SPEAKER_RINGOUT,
};

impl RotarySpeaker<'sr> {

    pub fn new( 
        tuner:     &'sr TunerHandle<'sr>,
        tables:    &'sr TablesHandle<'sr>,
        srunit:    &'sr SampleRateHandle<'sr>,
        time_unit: &'sr TimeUnitHandle<'sr>) -> Self {

        Self {
            ringout:       Ringout::blocks(ROTARY_SPEAKER_RINGOUT as NumberOfBlocks),
            srunit:        srunit.clone(),
            params:        RotarySpeakerParam::new_runtime(),
            buffer:        A1d::<f32>::zeros(ROTARY_SPEAKER_MAX_DELAY_LENGTH),
            wpos:          0_i32,
            xover:         BiquadFilter::new(tuner,tables,srunit),
            lowbass:       BiquadFilter::new(tuner,tables,srunit),
            lfo:           QuadrOsc::new(),
            lf_lfo:        QuadrOsc::new(),
            d_l:           LiPol::<f32>::new(BLOCK_SIZE),
            d_r:           LiPol::<f32>::new(BLOCK_SIZE),
            drive:         LiPol::<f32>::new(BLOCK_SIZE),
            hornamp: [
                LiPol::<f32>::new(BLOCK_SIZE),
                LiPol::<f32>::new(BLOCK_SIZE),
            ],
            first_run:   true,
            time_unit:   time_unit.clone(),
            tables:      tables.clone(),
        }
    }
}


