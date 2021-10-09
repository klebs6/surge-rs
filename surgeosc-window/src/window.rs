ix!();

use crate::{
    WindowOscillatorParam,
    WindowOscillatorParamArrayRT,
};

#[derive(Debug)]
pub struct WindowOscillator<'sr> {
    pub drift:             Align16<f32>,
    pub master_osc:        Align16<*mut f32>,
    pub out:               Align16<OscillatorOut>,

    pub pos:               A1d::<u32>,
    pub sub_pos:           A1d::<u32>,
    pub ratio:             A1d::<u32>,
    pub table:             A1d::<u32>,
    pub formant_mul:       A1d::<u32>,

    // samples until playback should start (for per-sample scheduling)
    pub dispatch_delay:    A1d::<u32>,
    pub gain:              A2d::<u32>,
    pub drift_lfo:         A2d::<f32>,
    pub fm_ratio:          A2d::<i32>,

    pub params:            WindowOscillatorParamArrayRT,
    pub osc_params:        OscillatorParamArrayRT,
    pub fm_depth:          [Lag<f64>; WINDOW_OSCILLATOR_NUM_SUBOSCS],
    pub out_attenuation:   f32,
    pub detune_bias:       f32, 
    pub detune_offset:     f32, 
    pub active_sub_oscs:   i32,
    pub window_wavetable:  WaveTableBase::<i16>,
    pub wave_wavetable:    WaveTableBase::<i16>,
    pub tables:            TablesHandle<'sr>,
    pub tuner:             TunerHandle<'sr>,
    pub srunit:            SampleRateHandle<'sr>,
}

name!          [WindowOscillator<'sr>, "oscillator.window_oscillator"];
no_op!         [WindowOscillator<'sr>,      HandleStreamingMismatches];
oscillator!    [WindowOscillator<'sr>,          WindowOscillatorParam];
no_op!         [WindowOscillator<'sr>,                       AssignFM];
allow_display! [WindowOscillator<'sr>,                           true];

impl SetPitch for WindowOscillator<'sr> {
    fn set_pitch(&mut self, _pitch: f32, is_display: bool) {
        if is_display {
            self.active_sub_oscs = 1;
        }
    }
}
