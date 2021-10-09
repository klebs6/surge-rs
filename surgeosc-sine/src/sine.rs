ix!();

use crate::{
    SineWaveOscillatorParam,
    SineWaveOscillatorParamArrayRT,
};

#[derive(Debug)]
pub struct SineWaveOscillator<'sr> {
    pub tuner:       TunerHandle<'sr>,
    pub master_osc:  *mut f32,
    pub params:      SineWaveOscillatorParamArrayRT,
    pub osc_params:  OscillatorParamArrayRT,
    pub out:         OscillatorOut,
    pub sine:        QuadrOsc,
    pub phase:       f64,
    pub driftlfo1:   f32,
    pub driftlfo2:   f32,
    pub fm_depth:    Lag<f64>,
    pub feedback:    Lag<f64>,
    pub lastvalue:   f32,
}

oscillator!    [SineWaveOscillator<'sr>, SineWaveOscillatorParam];
name!          [SineWaveOscillator<'sr>,       "oscillator.sine"];
no_op!         [SineWaveOscillator<'sr>,                SetPitch];
no_op!         [SineWaveOscillator<'sr>,                AssignFM];
allow_display! [SineWaveOscillator<'sr>,                    true];
