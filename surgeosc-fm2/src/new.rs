crate::ix!();

impl FM2Oscillator {

    pub fn new( 
        tuner:  TunerHandle, 
        srunit: SampleRateHandle,) -> Self {
        Self {
            tuner,
            srunit,
            out:             OscillatorOut::default(),
            master_osc:      std::ptr::null_mut(),//TODO
            params:          FM2OscillatorParam::new_runtime(),
            osc_params:      OscillatorParam::new_runtime(),
            phase:           0.0,
            lastoutput:      0.0,
            rm1:             QuadrOsc::default(),
            rm2:             QuadrOsc::default(),
            driftlfo:        0.0,
            driftlfo2:       0.0,
            fm_depth:        Lag::<f64>::new(0.0),
            rel_mod_depth1:  Lag::<f64>::new(0.0),
            rel_mod_depth2:  Lag::<f64>::new(0.0),
            feedback_depth:  Lag::<f64>::new(0.0),
            phase_offset:    Lag::<f64>::new(0.0),
        }
    }
}
