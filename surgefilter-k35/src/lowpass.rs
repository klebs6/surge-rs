crate::ix!();

impl crate::K35Filter {

    pub fn process_lp(
        f: &mut QuadFilterUnitState, 
        input: __m128) -> __m128 
    {
        unsafe {

            for i in 0..N_COEFFMAKER_COEFFS {
                qfu.C[i] = _mm_add_ps(qfu.C[i], qfu.dC[i]); 
            }

            let y1: __m128 = doLpf(
                qfu.C[C::G], 
                input, 
                qfu.R[R::Lpf1z1]
            );

            // (lpf beta * lpf2 feedback) + (hpf beta * hpf1 feedback)
            let s35: __m128 = _mm_add_ps(
                _mm_mul_ps(qfu.C[C::LpfBeta], qfu.R[R::Xpf2z1]), 
                _mm_mul_ps(qfu.C[C::HpfBeta], qfu.R[R::Hpf1z1])
            );

            // alpha * (y1 + s35)
            let u_clean: __m128 = _mm_mul_ps(
                qfu.C[C::Alpha], 
                _mm_add_ps(y1, s35)
            );

            let u_driven: __m128 = fasttanhSSEclamped(
                _mm_mul_ps(
                    u_clean, 
                    qfu.C[C::Saturation]
                )
            );

            let u: __m128 = _mm_add_ps(
                _mm_mul_ps(
                    u_clean, qfu.C[C::SaturationBlendInv]
                ), 
                _mm_mul_ps(
                    u_driven, 
                    qfu.C[C::SaturationBlend]
                )
            );

            // mk * lpf2(u)
            let y: __m128 = _mm_mul_ps(
                qfu.C[C::KModded], 
                doLpf(
                    qfu.C[C::G], 
                    u, 
                    qfu.R[R::Xpf2z1]
                )
            );

            doHpf(qfu.C[C::G], y, qfu.R[R::Hpf1z1]);

            _mm_div_ps(y, qfu.C[C::KModded])
        }
    }
}
