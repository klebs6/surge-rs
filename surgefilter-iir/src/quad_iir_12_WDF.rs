crate::ix!();

#[cfg(target_arch = "x86_64")] 
pub fn iir_12_wdf_quad(qfu: &mut QuadFilterUnitState, input: __m128) -> __m128 {

    coeffidx![C;
        E1bySC,
        E2bySC,
        MinusE1overSC,
        MinusE2overSC,
        C1,
        C2,
        D,
        Clipgain
    ];

    unsafe {

        qfu.coeff[C::E1bySC]        = _mm_add_ps(qfu.coeff[C::E1bySC], qfu.dcoeff[C::E1bySC]); // E1 * sc
        qfu.coeff[C::E2bySC]        = _mm_add_ps(qfu.coeff[C::E2bySC], qfu.dcoeff[C::E2bySC]); // E2 * sc
        qfu.coeff[C::MinusE1overSC] = _mm_add_ps(qfu.coeff[C::MinusE1overSC], qfu.dcoeff[C::MinusE1overSC]); // -E1 / sc
        qfu.coeff[C::MinusE2overSC] = _mm_add_ps(qfu.coeff[C::MinusE2overSC], qfu.dcoeff[C::MinusE2overSC]); // -E2 / sc
        qfu.coeff[C::C1]            = _mm_add_ps(qfu.coeff[C::C1], qfu.dcoeff[C::C1]); // C1
        qfu.coeff[C::C2]            = _mm_add_ps(qfu.coeff[C::C2], qfu.dcoeff[C::C2]); // C2
        qfu.coeff[C::D]             = _mm_add_ps(qfu.coeff[C::D], qfu.dcoeff[C::D]); // D

        let y: __m128 = _mm_add_ps(
            _mm_add_ps(
                _mm_mul_ps(qfu.coeff[C::C1], qfu.reg[C::E1bySC]), 
                _mm_mul_ps(qfu.coeff[C::D], input)
            ),
            _mm_mul_ps(qfu.coeff[C::C2], qfu.reg[C::E2bySC])
        );

        let t: __m128 = _mm_add_ps(
            input, 
            _mm_add_ps(
                _mm_mul_ps(qfu.coeff[C::MinusE1overSC], qfu.reg[C::E1bySC]),
                _mm_mul_ps(qfu.coeff[C::MinusE2overSC], qfu.reg[C::E2bySC])
            )
        );

        let s1: __m128 = _mm_add_ps(
            _mm_mul_ps(t, qfu.coeff[C::E1bySC]), 
            qfu.reg[C::E1bySC]
        );

        let s2: __m128 = _mm_sub_ps(
            _mm_setzero_ps(), 
            _mm_add_ps(
                _mm_mul_ps(t, qfu.coeff[C::E2bySC]), 
                qfu.reg[C::E2bySC]
            )
        );

        // qfu.reg[C::E1bySC] = s1;
        // qfu.reg[C::E2bySC] = s2;

        qfu.reg[C::E1bySC] = _mm_mul_ps(s1, qfu.reg[C::MinusE1overSC]);
        qfu.reg[C::E2bySC] = _mm_mul_ps(s2, qfu.reg[C::MinusE1overSC]);

        // Clipgain
        qfu.coeff[C::Clipgain] = _mm_add_ps(qfu.coeff[C::Clipgain], qfu.dcoeff[C::Clipgain]); 

        let m01: __m128 = _mm_set1_ps(0.1);
        let m1:  __m128 = _mm_set1_ps(1.0);

        qfu.reg[C::MinusE1overSC] = _mm_max_ps(
            m01, 
            _mm_sub_ps(
                m1, 
                _mm_mul_ps(qfu.coeff[C::Clipgain], _mm_mul_ps(y, y))
            )
        );

        y
    }
}
