ix!();

use crate::{
    FMOscillator,
    FMOscillatorParam,
};

impl FMOscillator<'sr> {

    pub fn new( tuner: TunerHandle<'sr>) -> Self {
        Self {
            tuner,
            out:             OscillatorOut::default(),
            master_osc:      std::ptr::null_mut(),
            params:          FMOscillatorParam::new_runtime(),
            osc_params:      OscillatorParam::runtime_array(),
            phase:           0.0,
            lastoutput:      0.0,
            rm1:             QuadrOsc::new(),
            rm2:             QuadrOsc::new(),
            am:              QuadrOsc::new(),
            driftlfo:        0.0,
            driftlfo2:       0.0,
            fm_depth:        Lag::<f64>::new(0.0),
            abs_mod_depth:   Lag::<f64>::new(0.0),
            rel_mod_depth1:  Lag::<f64>::new(0.0),
            rel_mod_depth2:  Lag::<f64>::new(0.0),
            feedback_depth:  Lag::<f64>::new(0.0),
        }
    }
}
