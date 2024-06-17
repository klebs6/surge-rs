crate::ix!();

impl RotarySpeaker {

    pub fn new( 
        tuner:     & TunerHandle,
        tables:    & TablesHandle,
        srunit:    & SampleRateHandle,
        time_unit: & TimeUnitHandle) -> Self {

        Self {
            ringout:       Ringout::blocks(ROTARY_SPEAKER_RINGOUT as NumberOfBlocks),
            srunit:        srunit.clone(),
            params:        RotarySpeakerParam::new_runtime(),
            buffer:        A1d::<f32>::zeros(ROTARY_SPEAKER_MAX_DELAY_LENGTH),
            wpos:          0_i32,
            xover:         BiquadFilter::new(tuner,tables,srunit),
            lowbass:       BiquadFilter::new(tuner,tables,srunit),
            lfo:           QuadrOsc::default(),
            lf_lfo:        QuadrOsc::default(),
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


