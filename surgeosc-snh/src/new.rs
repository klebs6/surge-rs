crate::ix!();

impl SampleAndHoldOscillator {

    pub fn new( 
        tuner:  TunerHandle, 
        tables: TablesHandle,
        srunit: SampleRateHandle,
    ) -> Result<Self,SurgeError> {

        let mut x = Self {
            out:                 OscillatorOut::default(),
            params:              SampleAndHoldOscillatorParam::new_runtime(),
            osc_params:          OscillatorParam::new_runtime(),
            master_osc:          std::ptr::null_mut(),//TODO
            drift:               0.0,
            blitter:             AbstractBlitter::new(&srunit),
            li_hpf:              LipolPs::new(), 
            li_dc:               LipolPs::new(), 
            li_integratormult:   LipolPs::new(),
            first_run:           true,
            dc:                  0.0,
            dc_uni:              A1d::<f32>::zeros(MAX_UNISON),
            elapsed_time:        A1d::<f32>::zeros(MAX_UNISON),
            last_level:          A1d::<f32>::zeros(MAX_UNISON),
            last_level2:         A1d::<f32>::zeros(MAX_UNISON),
            pwidth:              A1d::<f32>::zeros(MAX_UNISON),
            pitch:               432.0,
            fm_depth:            Lag::<f64>::new(0.0), 
            hpf_coeff:           Lag::<f64>::new(0.0), 
            integrator_mult:     Lag::<f64>::new(0.0), 
            l_pw:                Lag::<f64>::new(0.0), 
            l_shape:             Lag::<f64>::new(0.0), 
            l_smooth:            Lag::<f64>::new(0.0), 
            l_sub:               Lag::<f64>::new(0.0), 
            l_sync:              Lag::<f64>::new(0.0),
            fm_delay:            0,
            fm_mul_inv:          0.0,
            tables,
            tuner,
            srunit,
        };
        x.init()?;
        x.set_pitch(432.0, false);
        Ok(x)
    }
}
