crate::ix!();

/// Calculates and returns the high-pass filter
/// coefficient for the integrator.
///
/// The `samplerate_inv` parameter represents the
/// inverse of the sampling rate, and is used to
/// calculate the filter coefficient. The function
/// uses a formula to calculate the coefficient
/// based on the `samplerate_inv` value and
/// returns the result.
///
/// # Arguments
///
/// * `samplerate_inv` - The inverse of the sampling rate, expressed as a `f32` value.
///
/// # Example
///
/// ```ignore
/// use crate::AbstractBlitter;
///
/// let samplerate_inv = 44100.0_f32.recip();
/// let integrator_hpf = new_integrator_hpf(samplerate_inv);
///
/// assert_eq!(integrator_hpf, 0.9599225);
/// ```
///
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
            osc_out_l:            z128(),
            osc_out_2l:           z128(),
            osc_out_r:            z128(),
            osc_out_2r:           z128(),
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
