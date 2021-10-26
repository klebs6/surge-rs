ix!();

use crate::{
    WTOscillator,
    WTOscillatorParam,
};

impl WTOscillator {

    pub fn new(
        tuner:  TunerHandle,
        tables: TablesHandle,
        srunit: SampleRateHandle,
    ) -> Self 
    {
        let mut x = Self {
            drift:               0.0,
            master_osc:          std::ptr::null_mut(),
            blitter:             AbstractBlitter::new(&srunit),
            out:                 OscillatorOut::default(),
            params:              WTOscillatorParam::new_runtime(),
            osc_params:          OscillatorParam::runtime_array(),
            li_hpf:              LipolPs::new(),
            li_dc:               LipolPs::new(),
            li_integratormult:   LipolPs::new(),
            first_run:           true,
            oscpitch:            A1d::<f32>::zeros(MAX_UNISON),
            dc:                  0.0,
            dc_uni:              A1d::<f32>::zeros(MAX_UNISON),
            last_level:          A1d::<f32>::zeros(MAX_UNISON),
            pitch:               432.0,
            mipmap:              A1d::<i32>::zeros(MAX_UNISON),
            mipmap_ofs:          A1d::<i32>::zeros(MAX_UNISON),
            fm_depth:             Lag::<f32>::default(),
            hpf_coeff:           Lag::<f32>::default(),
            integrator_mult:     Lag::<f32>::default(),
            l_hskew:             Lag::<f32>::default(),
            l_vskew:             Lag::<f32>::default(),
            l_clip:              Lag::<f32>::default(),
            l_shape:             Lag::<f32>::default(),
            formant_t:           0.0,
            formant_last:        0.0,
            pitch_last:          0.0,
            pitch_t:             0.0,
            tableipol:           0.0,
            last_tableipol:      0.0,
            hskew:               0.0,
            last_hskew:          0.0,
            tableid:             0, 
            last_tableid:        0,
            fm_delay:            0,
            fm_mul_inv:          0.0,
            sampleloop:          0,
            wave_wavetable:      WaveTableBase::<i16>::default(),
            tables,
            tuner,
            srunit,
        };
        x.init();
        x.set_pitch(432.0,false);
        x
    }
}
