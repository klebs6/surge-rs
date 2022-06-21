crate::ix!();

impl SineWaveOscillator {

    pub fn new( tuner: TunerHandle) -> Self 
    {
        Self {
            tuner,
            master_osc:  std::ptr::null_mut(),//TODO
            out:         OscillatorOut::default(),
            params:      SineWaveOscillatorParam::new_runtime(),
            osc_params:  OscillatorParam::runtime_array(),
            sine:        QuadrOsc::new(),
            phase:       0.0,
            driftlfo1:   0.0,
            driftlfo2:   0.0,
            fm_depth:    Lag::<f64>::new(0.0),
            feedback:    Lag::<f64>::new(0.0),
            lastvalue:   0.0,
        }
    }
}
