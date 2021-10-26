ix!();

pub struct SnhFilter {
    tuner:  TunerHandle, 
    srunit: SampleRateHandle,
}

impl FilterProcessQuad for SnhFilter<'_> {


    #[cfg(target_arch = "x86_64")] 
    fn process_quad(&self, qfu: &mut QuadFilterUnitState<'_>, input: __m128) -> __m128 {

        coeffidx![ C; X0, X1, X2, X3, X4, X5, X6, X7 ];

        unsafe {
            qfu.coeff[C::X0] = _mm_add_ps(qfu.coeff[C::X0], qfu.dcoeff[C::X0]);
            qfu.coeff[C::X1] = _mm_add_ps(qfu.coeff[C::X1], qfu.dcoeff[C::X1]);

            qfu.reg[C::X0] = _mm_add_ps(qfu.reg[C::X0], qfu.coeff[C::X0]);

            let mask: __m128 = _mm_cmpgt_ps(qfu.reg[C::X0], _mm_setzero_ps());

            qfu.reg[C::X1] = _mm_or_ps(_mm_andnot_ps(mask, qfu.reg[C::X1]),
            _mm_and_ps(mask, softclip_ps(_mm_sub_ps(input, _mm_mul_ps(qfu.coeff[C::X1], qfu.reg[C::X1])))));

            let m1: __m128 = _mm_set1_ps(-1.0);
            qfu.reg[C::X0] = _mm_add_ps(qfu.reg[C::X0], _mm_and_ps(m1, mask));

            qfu.reg[C::X1]
        }
    }
}

impl CoeffMake for SnhFilter<'_> {

    fn coeff_make(&self, freq: f32, reso: f32) -> [f32; N_COEFFMAKER_COEFFS]
    {
        let note_pitch_ignoring_tuning =
            self.tuner.n2p::<f64,true>(-freq as f64);

        let dtime: f64 = (1.0 / CONCERT_A_HZ) * 
            note_pitch_ignoring_tuning * 
            self.srunit.dsamplerate_os();

        let v1: f64    = 1.0 / dtime;

        let mut c = [0.0_f32; N_COEFFMAKER_COEFFS];
        c[0] = v1 as f32;
        c[1] = reso;
        c
    }
}

