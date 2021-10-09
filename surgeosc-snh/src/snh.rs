ix!();

use crate::{
    SampleAndHoldOscillatorParam,
    SampleAndHoldOscillatorParamArrayRT,
};

#[derive(Debug)]
pub struct SampleAndHoldOscillator<'sr> {
    pub out:                 OscillatorOut,
    pub params:              SampleAndHoldOscillatorParamArrayRT,
    pub osc_params:          OscillatorParamArrayRT,
    pub master_osc:          *mut f32,
    pub drift:               f32,
    pub blitter:             AbstractBlitter,
    pub li_hpf:              LipolPs, 
    pub li_dc:               LipolPs, 
    pub li_integratormult:   LipolPs,
    pub first_run:           bool,
    pub dc:                  f32,
    pub dc_uni:              A1d::<f32>,
    pub elapsed_time:        A1d::<f32>,
    pub last_level:          A1d::<f32>,
    pub last_level2:         A1d::<f32>,
    pub pwidth:              A1d::<f32>,
    pub pitch:               f32,
    pub fm_depth:            Lag<f64>, 
    pub hpf_coeff:           Lag<f64>, 
    pub integrator_mult:     Lag<f64>, 
    pub l_pw:                Lag<f64>, 
    pub l_shape:             Lag<f64>, 
    pub l_smooth:            Lag<f64>, 
    pub l_sub:               Lag<f64>, 
    pub l_sync:              Lag<f64>,
    pub fm_delay:            i32,
    pub fm_mul_inv:          f32,
    pub tables:              TablesHandle<'sr>,
    pub tuner:               TunerHandle<'sr>,
    pub srunit:              SampleRateHandle<'sr>,
}

oscillator!      [SampleAndHoldOscillator<'sr>,  SampleAndHoldOscillatorParam];
no_op!           [SampleAndHoldOscillator<'sr>,     HandleStreamingMismatches];
name!            [SampleAndHoldOscillator<'sr>,              "oscillator.snh"];
no_op!           [SampleAndHoldOscillator<'sr>,                      AssignFM];
allow_display!   [SampleAndHoldOscillator<'sr>,                          true];
default_default! [SampleAndHoldOscillator<'sr>                                ];
