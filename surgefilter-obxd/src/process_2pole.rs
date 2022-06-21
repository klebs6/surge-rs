crate::ix!();

impl crate::ObxdFilter {

    pub unsafe fn process_2_pole(qfu: &mut QuadFilterUnitState, sample: __m128) -> __m128 {

        for i in 0..Obxd12dBCoeff::count() {
            qfu.coeff[i] = _mm_add_ps(qfu.coeff[i], qfu.dcoeff[i]);
        }

        // float v = ((sample- R * s1*2 - g2*s1 - s2)/(1+ R*g1*2 + g1*g2));
        let v: __m128 = obxd_filter::newton_raphson_12db(sample, qfu);

        let zero       = _mm_set1_ps(0.0);
        let half       = _mm_set1_ps(0.5);
        let one        = _mm_set1_ps(1.0);
        let g12        = qfu.coeff[Obxd12dBCoeff::G12 as usize];
        let multi_mode = qfu.coeff[Obxd12dBCoeff::MultiMode as usize];

        // float y1 = v * g + s1;
        let y1: __m128 = _mm_add_ps(
            _mm_mul_ps(v, g12), 
            qfu.reg[ObxdParams::S1 as usize]
        );

        // s1 = v * g + y1;
        qfu.reg[ObxdParams::S1 as usize] = _mm_add_ps(
            _mm_mul_ps(v, g12), 
            y1
        );

        // float y2 = y1 * g + s2;
        let y2: __m128 = _mm_add_ps(
            _mm_mul_ps(y1, g12), 
            qfu.reg[ObxdParams::S2 as usize]
        );

        // s2 = y1 * g + y2;
        qfu.reg[ObxdParams::S2 as usize] = _mm_add_ps(
            _mm_mul_ps(y1, g12), 
            y2
        );

        let mask_bp: __m128  = _mm_cmpeq_ps(
            qfu.coeff[Obxd12dBCoeff::BandPass as usize], 
            zero);

        let bp_false: __m128 = _mm_add_ps(
            _mm_mul_ps(
                _mm_sub_ps(
                    one, 
                    multi_mode), 
                y2), 
            _mm_mul_ps(
                multi_mode, 
                v));

        let mask: __m128 = _mm_cmplt_ps(
            multi_mode, 
            half);

        let val1: __m128 = _mm_add_ps(
            _mm_mul_ps(
                _mm_sub_ps(
                    half, 
                    multi_mode), 
                y2), 
            _mm_mul_ps(
                multi_mode, 
                y1));

        let val2: __m128 = _mm_add_ps(
            _mm_mul_ps(
                _mm_sub_ps(
                    one, 
                    multi_mode), 
                y1), 
            _mm_mul_ps(
                _mm_sub_ps(
                    multi_mode, 
                    half), 
                v));

        let bp_true: __m128  = _mm_add_ps(
            _mm_and_ps(mask, val1), 
            _mm_andnot_ps(mask, val2));


        let mc: __m128 =_mm_add_ps(
            _mm_and_ps(
                mask_bp, 
                bp_false), 
            _mm_andnot_ps(
                mask_bp, 
                bp_true));

        _mm_mul_ps(
            mc, 
            m128_gain_adjustment_2pole![])
    }
}
