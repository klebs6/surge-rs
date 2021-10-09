ix!();

use crate::{
    FM2OscillatorParam,
    FM2OscillatorParamArrayRT,
};

#[derive(Debug)]
pub struct FM2Oscillator<'sr> {
    pub tuner:           TunerHandle<'sr>,
    pub srunit:          SampleRateHandle<'sr>,
    pub out:             OscillatorOut,
    pub master_osc:      *mut f32,
    pub params:          FM2OscillatorParamArrayRT,
    pub osc_params:      OscillatorParamArrayRT,
    pub phase:           f64, 
    pub lastoutput:      f64,
    pub rm1:             QuadrOsc,
    pub rm2:             QuadrOsc,
    pub driftlfo:        f32,
    pub driftlfo2:       f32,
    pub fm_depth:        Lag<f64>,
    pub rel_mod_depth1:  Lag<f64>,
    pub rel_mod_depth2:  Lag<f64>,
    pub feedback_depth:  Lag<f64>,
    pub phase_offset:    Lag<f64>,
}

no_op!         [FM2Oscillator<'sr>, HandleStreamingMismatches];
oscillator!    [FM2Oscillator<'sr>,        FM2OscillatorParam];
name!          [FM2Oscillator<'sr>,          "oscillator.fm2"];
no_op!         [FM2Oscillator<'sr>,                  SetPitch];
no_op!         [FM2Oscillator<'sr>,                  AssignFM];
allow_display! [FM2Oscillator<'sr>,                      true];
