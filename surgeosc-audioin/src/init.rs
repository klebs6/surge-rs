ix!();

use crate::{
    AudioInputOscillator,
    AudioInputOscillatorParam,
};

impl AudioInputOscillator<'sr> {

    pub fn new( 
        tables:   TablesHandle<'sr>,
        synth_in: SynthInputHandle<'sr>) -> Self {
        Self {
            tables,
            synth_in,
            out:          OscillatorOut::default(),
            params:       AudioInputOscillatorParam::new_runtime(),
            osc_params:   OscillatorParam::runtime_array(),
        }
    }
}
