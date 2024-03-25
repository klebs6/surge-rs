crate::ix!();

coeffidx![
    C;
    K1,
    K2,
    Q1,
    Q2,
    V1,
    V2,
    V3,
    Clipgain
];


#[cfg(target_arch = "x86_64")] 
pub fn iir_24_b_quad(
    qfu: &mut QuadFilterUnitState, 
    input: __m128) -> __m128 {

    unsafe {

        qfu.coeff[C::K1] = _mm_add_ps(qfu.coeff[C::K1], qfu.dcoeff[C::K1]); // K1
        qfu.coeff[C::K2] = _mm_add_ps(qfu.coeff[C::K2], qfu.dcoeff[C::K2]); // K2
        qfu.coeff[C::Q1] = _mm_add_ps(qfu.coeff[C::Q1], qfu.dcoeff[C::Q1]); // Q1
        qfu.coeff[C::Q2] = _mm_add_ps(qfu.coeff[C::Q2], qfu.dcoeff[C::Q2]); // Q2
        qfu.coeff[C::V1] = _mm_add_ps(qfu.coeff[C::V1], qfu.dcoeff[C::V1]); // V1
        qfu.coeff[C::V2] = _mm_add_ps(qfu.coeff[C::V2], qfu.dcoeff[C::V2]); // V2
        qfu.coeff[C::V3] = _mm_add_ps(qfu.coeff[C::V3], qfu.dcoeff[C::V3]); // V3

        // Q2*in - K2*R1
        let mut f2: __m128 = _mm_sub_ps(
            _mm_mul_ps(qfu.coeff[C::Q2], input), 
            _mm_mul_ps(qfu.coeff[C::K2], qfu.reg[C::K2])
        );

        // K2*in + Q2*R1
        let mut g2: __m128 = _mm_add_ps(
            _mm_mul_ps(qfu.coeff[C::K2], input), 
            _mm_mul_ps(qfu.coeff[C::Q2], qfu.reg[C::K2])
        );

        // Q1*f2 - K1*R0
        let mut f1: __m128 = _mm_sub_ps(
            _mm_mul_ps(qfu.coeff[C::Q1], f2), 
            _mm_mul_ps(qfu.coeff[C::K1], qfu.reg[C::K1])
        );

        // K1*f2 + Q1*R0
        let mut g1: __m128 = _mm_add_ps(
            _mm_mul_ps(qfu.coeff[C::K1], f2), 
            _mm_mul_ps(qfu.coeff[C::Q1], qfu.reg[C::K1])
        );


        qfu.reg[C::K1] = _mm_mul_ps(f1, qfu.reg[C::V1]);
        qfu.reg[C::K2] = _mm_mul_ps(g1, qfu.reg[C::V1]);

        let y1: __m128 = _mm_add_ps(
            _mm_add_ps(
                _mm_mul_ps(qfu.coeff[C::V3], g2), 
                _mm_mul_ps(qfu.coeff[C::V2], g1)
            ),
            _mm_mul_ps(qfu.coeff[C::V1], f1)
        );

        // Q2*in - K2*R1
        f2 = _mm_sub_ps(
            _mm_mul_ps(qfu.coeff[C::Q2], y1), _mm_mul_ps(qfu.coeff[C::K2], qfu.reg[C::Q2])
        );

        // K2*in + Q2*R1
        g2 = _mm_add_ps(
            _mm_mul_ps(qfu.coeff[C::K2], y1), _mm_mul_ps(qfu.coeff[C::Q2], qfu.reg[C::Q2])
        );

        // Q1*f2 - K1*R0
        f1 = _mm_sub_ps(
            _mm_mul_ps(qfu.coeff[C::Q1], f2), _mm_mul_ps(qfu.coeff[C::K1], qfu.reg[C::Q1])
        );

        // K1*f2 + Q1*R0
        g1 = _mm_add_ps(
            _mm_mul_ps(qfu.coeff[C::K1], f2), _mm_mul_ps(qfu.coeff[C::Q1], qfu.reg[C::Q1])
        );

        qfu.reg[C::Q1] = _mm_mul_ps(f1, qfu.reg[C::V1]);
        qfu.reg[C::Q2] = _mm_mul_ps(g1, qfu.reg[C::V1]);

        let y2: __m128 = _mm_add_ps(
            _mm_add_ps(
                _mm_mul_ps(qfu.coeff[C::V3], g2), 
                _mm_mul_ps(qfu.coeff[C::V2], g1)
            ),
            _mm_mul_ps(qfu.coeff[C::V1], f1)
        );

        // Clipgain
        qfu.coeff[C::Clipgain] = _mm_add_ps(qfu.coeff[C::Clipgain], qfu.dcoeff[C::Clipgain]);

        let m01: __m128 = _mm_set1_ps(0.1);
        let m1:  __m128 = _mm_set1_ps(1.0);

        qfu.reg[C::V1] = _mm_max_ps(
            m01, 
            _mm_sub_ps(
                m1, 
                _mm_mul_ps(
                    qfu.coeff[C::Clipgain], 
                    _mm_mul_ps(y2, y2)
                )
            )
        );

        y2
    }
}
