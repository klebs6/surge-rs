ix!();

use crate::{
    WTOscillatorParam,
    WTOscillatorParamArrayRT,
};

#[derive(Debug)]
pub struct WTOscillator {
    pub drift:               f32,
    pub master_osc:          *mut f32,
    pub blitter:             AbstractBlitter,
    pub out:                 OscillatorOut,
    pub params:              WTOscillatorParamArrayRT,
    pub osc_params:          OscillatorParamArrayRT,
    pub li_hpf:              LipolPs,
    pub li_dc:               LipolPs,
    pub li_integratormult:   LipolPs,
    pub first_run:           bool,
    pub oscpitch:            A1d::<f32>,
    pub dc:                  f32,
    pub dc_uni:              A1d::<f32>,
    pub last_level:          A1d::<f32>,
    pub pitch:               f32,
    pub mipmap:              A1d::<i32>,
    pub mipmap_ofs:          A1d::<i32>,
    pub fm_depth:            Lag::<f32>,
    pub hpf_coeff:           Lag::<f32>,
    pub integrator_mult:     Lag::<f32>,
    pub l_hskew:             Lag::<f32>,
    pub l_vskew:             Lag::<f32>,
    pub l_clip:              Lag::<f32>,
    pub l_shape:             Lag::<f32>,
    pub formant_t:           f32,
    pub formant_last:        f32,
    pub pitch_last:          f32,
    pub pitch_t:             f32,
    pub tableipol:           f32,
    pub last_tableipol:      f32,
    pub hskew:               f32,
    pub last_hskew:          f32,
    pub tableid:             i32, 
    pub last_tableid:        i32,
    pub fm_delay:            i32,
    pub fm_mul_inv:          f32,
    pub sampleloop:          i32,
    pub tables:              TablesHandle,
    pub tuner:               TunerHandle,
    pub srunit:              SampleRateHandle,
    pub wave_wavetable:      WaveTableBase::<i16>,
    //pub FMFilter:          BiquadFilter,
    //float wavetable[wavetable_steps];
}

name!             [WTOscillator, "oscillator.wavetable_oscillator"];
no_op!            [WTOscillator, HandleStreamingMismatches];
oscillator!       [WTOscillator, WTOscillatorParam];
no_op!            [WTOscillator, AssignFM];
allow_display!    [WTOscillator, true];
default_default!  [WTOscillator  ];
