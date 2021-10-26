ix!();

use crate::C;

#[cfg(target_arch = "x86_64")] 
pub fn svf_bp24_a_quad(qfu: &mut QuadFilterUnitState, mut input: __m128) -> __m128 {

    unsafe {

        qfu.coeff[C::X0] = _mm_add_ps(
            qfu.coeff[C::X0], 
            qfu.dcoeff[C::X0]
        ); // F1

        qfu.coeff[C::X1] = _mm_add_ps(
            qfu.coeff[C::X1], 
            qfu.dcoeff[C::X1]
        ); // Q1

        let mut l: __m128 = _mm_add_ps(
            qfu.reg[C::X1], 
            _mm_mul_ps(
                qfu.coeff[C::X0], 
                qfu.reg[C::X0])
        );

        let mut h: __m128 = _mm_sub_ps(
            _mm_sub_ps(
                input, 
                l), 
            _mm_mul_ps(
                qfu.coeff[C::X1], 
                qfu.reg[C::X0])
        );

        let mut b: __m128 = _mm_add_ps(
            qfu.reg[C::X0], 
            _mm_mul_ps(
                qfu.coeff[C::X0], 
                h)
        );

        l = _mm_add_ps(
            l, 
            _mm_mul_ps(
                qfu.coeff[C::X0], 
                b)
        );

        h = _mm_sub_ps(
            _mm_sub_ps(
                input, 
                l), 
            _mm_mul_ps(
                qfu.coeff[C::X1], 
                b)
        );

        b = _mm_add_ps(
            b, 
            _mm_mul_ps(
                qfu.coeff[C::X0], 
                h)
        );

        qfu.reg[C::X0] = _mm_mul_ps(
            b, 
            qfu.reg[C::X2]
        );

        qfu.reg[C::X1] = _mm_mul_ps(
            l, 
            qfu.reg[C::X2]
        );

        input = b;

        l = _mm_add_ps(
            qfu.reg[C::X4], 
            _mm_mul_ps(
                qfu.coeff[C::X0], 
                qfu.reg[C::X3])
        );

        h = _mm_sub_ps(
            _mm_sub_ps(
                input, 
                l), 
            _mm_mul_ps(
                qfu.coeff[C::X1], 
                qfu.reg[C::X3])
        );

        b = _mm_add_ps(
            qfu.reg[C::X3], 
            _mm_mul_ps(
                qfu.coeff[C::X0], 
                h)
        );

        l = _mm_add_ps(
            l, 
            _mm_mul_ps(
                qfu.coeff[C::X0], 
                b)
        );

        h = _mm_sub_ps(
            _mm_sub_ps(
                input, 
                l), 
            _mm_mul_ps(
                qfu.coeff[C::X1], 
                b)
        );

        b = _mm_add_ps(
            b, 
            _mm_mul_ps(
                qfu.coeff[C::X0], 
                h));


        qfu.reg[C::X3] = _mm_mul_ps(
            b, 
            qfu.reg[C::X2]
        );

        qfu.reg[C::X4] = _mm_mul_ps(
            l, 
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

        _mm_mul_ps(b, qfu.coeff[C::X3])
    }
}
