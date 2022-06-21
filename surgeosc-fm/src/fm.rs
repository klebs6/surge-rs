crate::ix!();

#[derive(Debug)]
pub struct FMOscillator {
    pub tuner:           TunerHandle,
    pub out:             OscillatorOut,
    pub master_osc:      *mut f32,
    pub params:          FMOscillatorParamArrayRT,
    pub osc_params:      OscillatorParamArrayRT,
    pub phase:           f64,
    pub lastoutput:      f64,
    pub rm1:             QuadrOsc,
    pub rm2:             QuadrOsc,
    pub am:              QuadrOsc,
    pub driftlfo:        f32,
    pub driftlfo2:       f32,
    pub fm_depth:        Lag<f64>,
    pub abs_mod_depth:   Lag<f64>,
    pub rel_mod_depth1:  Lag<f64>,
    pub rel_mod_depth2:  Lag<f64>,
    pub feedback_depth:  Lag<f64>,
}

no_op!         [FMOscillator, HandleStreamingMismatches];
oscillator!    [FMOscillator,         FMOscillatorParam];
name!          [FMOscillator,           "oscillator.fm"];
no_op!         [FMOscillator,                  SetPitch];
no_op!         [FMOscillator,                  AssignFM];
allow_display! [FMOscillator,                      true];
