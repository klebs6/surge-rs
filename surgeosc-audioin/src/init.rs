crate::ix!();

impl AudioInputOscillator {

    pub fn new( 
        tables:   TablesHandle,
        synth_in: SynthInputHandle) -> Self {
        Self {
            tables,
            synth_in,
            out:          OscillatorOut::default(),
            params:       AudioInputOscillatorParam::new_runtime(),
            osc_params:   OscillatorParam::runtime_array(),
        }
    }
}
