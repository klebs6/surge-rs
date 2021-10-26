ix!();

use crate::C;

#[cfg(target_arch = "x86_64")] 
pub fn svf_lp12_a_quad(qfu: &mut QuadFilterUnitState, input: __m128) -> __m128 {

    unsafe {

        qfu.coeff[C::X0] = _mm_add_ps(
            qfu.coeff[C::X0], 
            qfu.dcoeff[C::X0]
        ); // F1

        qfu.coeff[C::X1] = _mm_add_ps(
            qfu.coeff[C::X1], 
            qfu.dcoeff[C::X1]
        ); // Q1

        let l: __m128 = _mm_add_ps(
            qfu.reg[C::X1], 
            _mm_mul_ps(qfu.coeff[C::X0], qfu.reg[C::X0])
        );

        let h: __m128 = _mm_sub_ps(
            _mm_sub_ps(
                input, 
                l), 
            _mm_mul_ps(
                qfu.coeff[C::X1], 
                qfu.reg[C::X0])
        );

        let b: __m128 = _mm_add_ps(
            qfu.reg[C::X0], 
            _mm_mul_ps(qfu.coeff[C::X0], h)
        );

        let l2: __m128 = _mm_add_ps(
            l, 
            _mm_mul_ps(qfu.coeff[C::X0], b)
        );

        let h2: __m128 = _mm_sub_ps(
            _mm_sub_ps(
                input, 
                l2), 
            _mm_mul_ps(
                qfu.coeff[C::X1], 
                b)
        );

        let b2: __m128  = _mm_add_ps(
            b, 
            _mm_mul_ps(qfu.coeff[C::X0], h2)
        );

        qfu.reg[C::X0] = _mm_mul_ps(
            b2, 
            qfu.reg[C::X2]
        );

        qfu.reg[C::X1] = _mm_mul_ps(
            l2, 
            qfu.reg[C::X2]
        );

        qfu.coeff[C::X2] = _mm_add_ps(
            qfu.coeff[C::X2], 
            qfu.dcoeff[C::X2]
        );

        let m01: __m128 = _mm_set1_ps(0.1);
        let m1:  __m128 = _mm_set1_ps(1.0);

        qfu.reg[C::X2] = _mm_max_ps(
            m01, 
            _mm_sub_ps(
                m1, 
                _mm_mul_ps(
                    qfu.coeff[C::X2], 
                    _mm_mul_ps(
                        b, 
                        b)))
        );

        qfu.coeff[C::X3] = _mm_add_ps(
            qfu.coeff[C::X3], 
            qfu.dcoeff[C::X3]
        ); // Gain

        _mm_mul_ps(
            l2, 
            qfu.coeff[C::X3]
        )
    }
}
