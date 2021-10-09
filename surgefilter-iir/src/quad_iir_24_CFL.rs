ix!();

#[cfg(target_arch = "x86_64")] 
/// State-space with softer limiter
pub fn iir_24_cfl_quad(
    qfu: &mut QuadFilterUnitState<'_>, 
    input: __m128) -> __m128 
{
    coeffidx![
        C;
        Ar,
        Ai,
        B1,
        Unused,
        C1,
        C2,
        D
    ];

    unsafe {

        qfu.coeff[C::Ar] = _mm_add_ps(qfu.coeff[C::Ar], qfu.dcoeff[C::Ar]); // (ar)
        qfu.coeff[C::Ai] = _mm_add_ps(qfu.coeff[C::Ai], qfu.dcoeff[C::Ai]); // (ai)
        qfu.coeff[C::B1] = _mm_add_ps(qfu.coeff[C::B1], qfu.dcoeff[C::B1]); // b1
        qfu.coeff[C::C1] = _mm_add_ps(qfu.coeff[C::C1], qfu.dcoeff[C::C1]); // c1
        qfu.coeff[C::C2] = _mm_add_ps(qfu.coeff[C::C2], qfu.dcoeff[C::C2]); // c2
        qfu.coeff[C::D]  = _mm_add_ps(qfu.coeff[C::D], qfu.dcoeff[C::D]); // d

        let ar: __m128 = _mm_mul_ps(qfu.coeff[C::Ar], qfu.reg[C::B1]);
        let ai: __m128 = _mm_mul_ps(qfu.coeff[C::Ai], qfu.reg[C::B1]);

        let y: __m128 = _mm_add_ps(
            _mm_add_ps(
                _mm_mul_ps(qfu.coeff[C::C1], qfu.reg[C::Ar]), 
                _mm_mul_ps(qfu.coeff[C::D], input)
            ),
            _mm_mul_ps(qfu.coeff[C::C2], qfu.reg[C::Ai])
        );

        let s1: __m128 = _mm_add_ps(
            _mm_mul_ps(input, qfu.coeff[C::B1]),
            _mm_sub_ps(
                _mm_mul_ps(ar, qfu.reg[C::Ar]), 
                _mm_mul_ps(ai, qfu.reg[C::Ai])
            )
        );

        let s2: __m128 = _mm_add_ps(
            _mm_mul_ps(ai, qfu.reg[C::Ar]), 
            _mm_mul_ps(ar, qfu.reg[C::Ai])
        );

        qfu.reg[C::Ar] = s1;
        qfu.reg[C::Ai] = s2;

        let y2: __m128 = _mm_add_ps(
            _mm_add_ps(
                _mm_mul_ps(qfu.coeff[C::C1], qfu.reg[C::Unused]), 
                _mm_mul_ps(qfu.coeff[C::D], y)
            ),
            _mm_mul_ps(qfu.coeff[C::C2], qfu.reg[C::C1])
        );

        let s3: __m128 = _mm_add_ps(
            _mm_mul_ps(y, qfu.coeff[C::B1]),
            _mm_sub_ps(
                _mm_mul_ps(ar, qfu.reg[C::Unused]), 
                _mm_mul_ps(ai, qfu.reg[C::C1])
            )
        );

        let s4: __m128 = _mm_add_ps(
            _mm_mul_ps(ai, qfu.reg[C::Unused]), 
            _mm_mul_ps(ar, qfu.reg[C::C1])
        );

        qfu.reg[C::Unused] = s3;
        qfu.reg[C::C1] = s4;

        /*m = 1 ./ max(1,abs(y(i)));
          mr = mr.*0.99 + m.*0.01;*/

        // Limiter
        let m001: __m128 = _mm_set1_ps(0.001);
        let m099: __m128 = _mm_set1_ps(0.999);
        let m1:   __m128 = _mm_set1_ps(1.0);
        let m2:   __m128 = _mm_set1_ps(2.0);

        let m: __m128 = _mm_rsqrt_ps(
            _mm_max_ps(
                m1, 
                _mm_mul_ps(
                    m2, 
                    _mm_and_ps(y2, m128_mask_absval![])
                )
            )
        );

        qfu.reg[C::B1] = _mm_add_ps(
            _mm_mul_ps(qfu.reg[C::B1], m099), 
            _mm_mul_ps(m, m001)
        );

        y2
    }
}
