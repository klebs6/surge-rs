crate::ix!();

///audio input oscillator
///TODO: add controls? {input l/r, gain, limiter}
#[derive(Debug)]
#[name("oscillator.audio_input")]
pub struct AudioInputOscillator {
    pub out:          OscillatorOut,
    pub params:       AudioInputOscillatorParamArrayRT,
    pub osc_params:   OscillatorParamArrayRT,
    pub synth_in:     SynthInputHandle,
    pub tables:       TablesHandle,
}

no_op!         [AudioInputOscillator, HandleStreamingMismatches];
oscillator!    [AudioInputOscillator, AudioInputOscillatorParam];
no_op!         [AudioInputOscillator,                  SetPitch];
no_op!         [AudioInputOscillator,                  AssignFM];
allow_display! [AudioInputOscillator,                     false];
no_op!         [AudioInputOscillator,                      Initialize];
