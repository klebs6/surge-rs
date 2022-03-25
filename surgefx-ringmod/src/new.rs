ix!();

use crate::*;

impl RingModulator {

    pub fn new(
        tuner:     &TunerHandle,
        tables:    &TablesHandle,
        srunit:    &SampleRateHandle,
    ) -> Self {
        Self {
            ringout:       Ringout::blocks(1000),
            params:        RingModulatorParam::new_runtime(),
            lp:            BiquadFilter::new(tuner,tables,srunit),
            hp:            BiquadFilter::new(tuner,tables,srunit),
            halfband_out:  HalfRateFilterSSE::new(6,true),
            halfband_in:   HalfRateFilterSSE::new(6,true),
            phase:         A1d::<f32>::zeros(RINGMOD_MAX_UNISON as usize),
            detune_offset: A1d::<f32>::zeros(RINGMOD_MAX_UNISON as usize),
            pan_l:         A1d::<f32>::zeros(RINGMOD_MAX_UNISON as usize),
            pan_r:         A1d::<f32>::zeros(RINGMOD_MAX_UNISON as usize),
            last_unison:   -1,
            tuner:         tuner.clone(),
            srunit:        srunit.clone(),
        }
    }
}

