ix!();

use crate::AbstractBlitter;

pub fn new_integrator_hpf(samplerate_inv: f32) -> f32 {
    let x = 1.0 - 2.0 * 20.0 * samplerate_inv;
    x * x
}

impl AbstractBlitter {

    pub fn new(srunit: & SampleRateHandle) -> Self {
        Self {
            oscbuffer_l:          Align16(A1d::<f32>::zeros( OB_LENGTH + FIR_IPOL_N )),
            oscbuffer_r:          Align16(A1d::<f32>::zeros( OB_LENGTH + FIR_IPOL_N )),
            dcbuffer:             Align16(A1d::<f32>::zeros( OB_LENGTH + FIR_IPOL_N )),
            osc_out_l:            unsafe{ z128![] },
            osc_out_2l:           unsafe{ z128![] },
            osc_out_r:            unsafe{ z128![] },
            osc_out_2r:           unsafe{ z128![] },
            integrator_hpf:       new_integrator_hpf(srunit.samplerate_inv()),
            pitchmult:            0.0,
            pitchmult_inv:        0.0,
            n_unison:             0,
            bufpos:               0,
            out_attenuation:      0.0,
            out_attenuation_inv:  0.0,
            detune_bias:          0.0,
            detune_offset:        0.0,
            oscstate:             A1d::<f32>::zeros(MAX_UNISON),
            syncstate:            A1d::<f32>::zeros(MAX_UNISON),
            rate:                 A1d::<f32>::zeros(MAX_UNISON),
            driftlfo:             A1d::<f32>::zeros(MAX_UNISON),
            driftlfo2:            A1d::<f32>::zeros(MAX_UNISON),
            pan_l:                A1d::<f32>::zeros(MAX_UNISON),
            pan_r:                A1d::<f32>::zeros(MAX_UNISON),
            state:                A1d::<i32>::zeros(MAX_UNISON)
        }
    }
}
