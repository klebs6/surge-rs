ix!();

use crate::{
    AudioInputOscillatorParam,
    AudioInputOscillatorParamArrayRT,
};

///audio input oscillator
///TODO: add controls? {input l/r, gain, limiter}
#[derive(Debug)]
pub struct AudioInputOscillator<'sr> {
    pub out:          OscillatorOut,
    pub params:       AudioInputOscillatorParamArrayRT,
    pub osc_params:   OscillatorParamArrayRT,
    pub synth_in:     SynthInputHandle<'sr>,
    pub tables:       TablesHandle<'sr>,
}

no_op!         [AudioInputOscillator<'sr>, HandleStreamingMismatches];
oscillator!    [AudioInputOscillator<'sr>, AudioInputOscillatorParam];
name!          [AudioInputOscillator<'sr>,  "oscillator.audio_input"];
no_op!         [AudioInputOscillator<'sr>,                  SetPitch];
no_op!         [AudioInputOscillator<'sr>,                  AssignFM];
allow_display! [AudioInputOscillator<'sr>,                     false];
no_op!         [AudioInputOscillator<'sr>,                      Init];
