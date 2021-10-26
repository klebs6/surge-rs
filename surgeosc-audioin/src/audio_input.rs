ix!();

use crate::{
    AudioInputOscillatorParam,
    AudioInputOscillatorParamArrayRT,
};

///audio input oscillator
///TODO: add controls? {input l/r, gain, limiter}
#[derive(Debug)]
pub struct AudioInputOscillator {
    pub out:          OscillatorOut,
    pub params:       AudioInputOscillatorParamArrayRT,
    pub osc_params:   OscillatorParamArrayRT,
    pub synth_in:     SynthInputHandle,
    pub tables:       TablesHandle,
}

no_op!         [AudioInputOscillator, HandleStreamingMismatches];
oscillator!    [AudioInputOscillator, AudioInputOscillatorParam];
name!          [AudioInputOscillator,  "oscillator.audio_input"];
no_op!         [AudioInputOscillator,                  SetPitch];
no_op!         [AudioInputOscillator,                  AssignFM];
allow_display! [AudioInputOscillator,                     false];
no_op!         [AudioInputOscillator,                      Init];
