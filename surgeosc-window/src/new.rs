ix!();

use crate::{
    WindowOscillator,
    WindowOscillatorParam,
};

impl WindowOscillator<'sr> {

    pub fn new( 
        tuner:  TunerHandle<'sr>,
        tables: TablesHandle<'sr>,
        srunit: SampleRateHandle<'sr>) -> Self {
        Self {
            drift:              Align16(0.0),
            master_osc:         Align16(std::ptr::null_mut()),//TODO
            params:             WindowOscillatorParam::new_runtime(),
            osc_params:         OscillatorParam::runtime_array(),
            out:                Align16(OscillatorOut::default()),
            pos:                A1d::<u32>::zeros(WINDOW_OSCILLATOR_NUM_SUBOSCS),
            sub_pos:            A1d::<u32>::zeros(WINDOW_OSCILLATOR_NUM_SUBOSCS),
            ratio:              A1d::<u32>::zeros(WINDOW_OSCILLATOR_NUM_SUBOSCS),
            table:              A1d::<u32>::zeros(WINDOW_OSCILLATOR_NUM_SUBOSCS),
            formant_mul:        A1d::<u32>::zeros(WINDOW_OSCILLATOR_NUM_SUBOSCS),
            dispatch_delay:     A1d::<u32>::zeros(WINDOW_OSCILLATOR_NUM_SUBOSCS), 
            gain:               A2d::<u32>::zeros((2,WINDOW_OSCILLATOR_NUM_SUBOSCS)),
            drift_lfo:          A2d::<f32>::zeros((2,WINDOW_OSCILLATOR_NUM_SUBOSCS)),
            fm_ratio:           A2d::<i32>::zeros((BLOCK_SIZE_OS,WINDOW_OSCILLATOR_NUM_SUBOSCS)),
            fm_depth:           Default::default(),
            out_attenuation:    0.0,
            detune_bias:        0.0, 
            detune_offset:      0.0, 
            active_sub_oscs:    0,
            window_wavetable:   WaveTableBase::<i16>::default(),
            wave_wavetable:     WaveTableBase::<i16>::default(),
            tuner,
            tables,
            srunit,
        }
    }
}
