crate::ix!();

#[derive(Debug)]
pub struct SurgeSuperOscillator {
    pub drift:             f32,
    pub master_osc:        *mut f32,
    pub out:               OscillatorOut,
    pub blitter:           AbstractBlitter,
    pub params:            SSOParamArrayRT,
    pub osc_params:        OscillatorParamArrayRT,
    pub li_hpf:            LipolPs,
    pub li_dc:             LipolPs,
    pub li_integratormult: LipolPs,

    pub fm_phase:          Align16<[f32; BLOCK_SIZE_OS + 4]>,
    pub first_run:         bool,
    pub dc:                f32,
    pub dc_uni:            A1d::<f32>,
    pub elapsed_time:      A1d::<f32>,
    pub last_level:        A1d::<f32>,
    pub pwidth:            A1d::<f32>,
    pub pwidth2:           A1d::<f32>,
    pub pitch:             f32,
    pub fm_depth:          Lag::<f32>,
    pub integrator_mult:   Lag::<f32>,
    pub l_pw:              Lag::<f32>,
    pub l_pw2:             Lag::<f32>,
    pub l_shape:           Lag::<f32>,
    pub l_sub:             Lag::<f32>,
    pub l_sync:            Lag::<f32>,
    pub fm_delay:          i32,
    pub fm_mul_inv:        f32,
    pub coeff_b0:          f32,
    pub coeff_b1:          f32,
    pub coeff_a1:          f32,
    pub tables:            TablesHandle,
    pub tuner:             TunerHandle,
    pub srunit:            SampleRateHandle,
}

oscillator!    [SurgeSuperOscillator,                  SSOParam];
name!          [SurgeSuperOscillator,          "oscillator.sso"];
no_op!         [SurgeSuperOscillator, HandleStreamingMismatches];
no_op!         [SurgeSuperOscillator,                  AssignFM];
allow_display! [SurgeSuperOscillator,                      true];
