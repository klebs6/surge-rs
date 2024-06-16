crate::ix!();

#[derive(Debug)]
#[name("oscillator.sine")]
pub struct SineWaveOscillator {
    pub tuner:       TunerHandle,
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

oscillator!    [SineWaveOscillator, SineWaveOscillatorParam];
no_op!         [SineWaveOscillator,                SetPitch];
no_op!         [SineWaveOscillator,                AssignFM];
allow_display! [SineWaveOscillator,                    true];
