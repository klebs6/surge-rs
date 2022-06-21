crate::ix!();

impl SurgeSuperOscillator {

    pub fn new(
        tuner:    TunerHandle,
        tables:   TablesHandle,
        srunit:   SampleRateHandle,
    ) -> Self {

        Self {
            drift:             0.0,
            master_osc:        std::ptr::null_mut(),//TODO
            out:               OscillatorOut::default(),
            blitter:           AbstractBlitter::new(&srunit),
            params:            SSOParam::new_runtime(),
            osc_params:        OscillatorParam::runtime_array(),
            li_hpf:            LipolPs::new(),
            li_dc:             LipolPs::new(),
            li_integratormult: LipolPs::new(),
            fm_phase:          Align16([0.0; BLOCK_SIZE_OS + 4]),
            first_run:         true,
            dc:                0.0,
            dc_uni:            A1d::<f32>::zeros(MAX_UNISON),
            elapsed_time:      A1d::<f32>::zeros(MAX_UNISON),
            last_level:        A1d::<f32>::zeros(MAX_UNISON),
            pwidth:            A1d::<f32>::zeros(MAX_UNISON),
            pwidth2:           A1d::<f32>::zeros(MAX_UNISON),
            pitch:             0.0,
            fm_depth:          Lag::<f32>::default(),
            integrator_mult:   Lag::<f32>::default(),
            l_pw:              Lag::<f32>::default(),
            l_pw2:             Lag::<f32>::default(),
            l_shape:           Lag::<f32>::default(),
            l_sub:             Lag::<f32>::default(),
            l_sync:            Lag::<f32>::default(),
            fm_delay:          0,
            fm_mul_inv:        0.0,
            coeff_b0:          0.0,
            coeff_b1:          0.0,
            coeff_a1:          0.0,
            tables:            tables.clone(),
            tuner:             tuner.clone(),
            srunit:            srunit.clone(),
        }
    }
}
