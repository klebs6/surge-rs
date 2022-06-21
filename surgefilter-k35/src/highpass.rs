crate::ix!();

impl crate::K35Filter {

    pub fn process_hp(
        f: &mut QuadFilterUnitState, 
        input: __m128) -> __m128 
    {
        unsafe {
            for i in 0..N_COEFFMAKER_COEFFS {
                qfu.coeff[i] = _mm_add_ps(qfu.coeff[i], qfu.dcoeff[i]); 
            }

            let y1:  __m128 = doHpf(
                qfu.coeff[C::G], 
                input, 
                qfu.reg[R::Hpf1z1]
            );

            // (lpf beta * lpf2 feedback) + (hpf beta * hpf1 feedback)
            let s35: __m128 = _mm_add_ps(
                _mm_mul_ps(qfu.coeff[C::HpfBeta], qfu.reg[R::Xpf2z1]), 
                _mm_mul_ps(qfu.coeff[C::LpfBeta], qfu.reg[R::Lpf1z1])
            );

            // alpha * (y1 + s35)
            let u:   __m128 = _mm_mul_ps(
                qfu.coeff[C::Alpha], 
                _mm_add_ps(y1, s35)
            );

            // mk * lpf2(u)
            let y_clean:  __m128 = _mm_mul_ps(qfu.coeff[C::KModded], u);

            let y_driven: __m128 = fasttanhSSEclamped(
                _mm_mul_ps(
                    y_clean, 
                    qfu.coeff[C::Saturation]
                )
            );

            let y: __m128 = _mm_add_ps(
                _mm_mul_ps(y_clean,  qfu.coeff[C::SaturationBlendInv]), 
                _mm_mul_ps(y_driven, qfu.coeff[C::SaturationBlend])
            );

            doLpf(
                qfu.coeff[C::G], 
                doHpf(
                    qfu.coeff[C::G], 
                    y, 
                    qfu.reg[R::Xpf2z1]
                ), 
                qfu.reg[R::Lpf1z1]
            );

            _mm_div_ps(y, qfu.coeff[C::KModded])
        }
    }
}
