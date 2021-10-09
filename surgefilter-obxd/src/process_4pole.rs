ix!();

use crate::{
    obxd_filter,
    Obxd24dBCoeff,
    ObxdParams,
    OBXD_SSEW,
};

impl crate::ObxdFilter<'sr> {

    pub unsafe fn process_4_pole(qfu: &mut QuadFilterUnitState, sample: __m128) -> __m128 {

        let zero  = _mm_set1_ps(0.0);
        let one   = _mm_set1_ps(1.0);
        let two   = _mm_set1_ps(2.0);
        let three = _mm_set1_ps(3.0);

        let g24 = qfu.coeff[Obxd24dBCoeff::G24];
        let s1  = qfu.reg[ObxdParams::S1 as usize];

        let pole_mix_inv_int = qfu.coeff[Obxd24dBCoeff::PoleMixInvInt];
        let pole_mix_scaled  = qfu.coeff[Obxd24dBCoeff::PoleMixScaled];

        for i in 0..Obxd24dBCoeff::count() {
            qfu.coeff[i] = _mm_add_ps( qfu.coeff[i], qfu.dcoeff[i]);
        }

        // float lpc = qfu.coeff[g] / (1 + qfu.coeff[g]);
        let lpc: __m128 = _mm_div_ps(
            g24, 
            _mm_add_ps(one, g24)
        );

        // float y0 = NewtonRaphsonR24dB(sample,qfu.coeff[g],lpc);
        let y0: __m128 = obxd_filter::newton_raphson_r24_db(sample, lpc, qfu);

        // first lowpass in cascade
        // double v = (y0 - s1) * lpc;
        let v: __m128 = _mm_mul_ps(
            _mm_sub_ps(
                y0, 
                s1), 
            lpc);

        // double res = v + s1;
        let res: __m128 = _mm_add_ps( v, s1);

        // s1 = res + v;
        qfu.reg[ObxdParams::S1 as usize] = _mm_add_ps(res, v);

        // damping
        // qfu.reg[ObxdParams::S1 as usize] =atan(s1*rcor24)*Rcor24inv;
        let mut s1_rcor24: __m128 = _mm_mul_ps(
            qfu.reg[ObxdParams::S1 as usize], 
            qfu.coeff[Obxd24dBCoeff::Rcor24]);

        let mut s1_rcor24_arr = [0.0_f32; (OBXD_SSEW as usize)];

        _mm_store_ps(
            s1_rcor24_arr.as_mut_ptr(), 
            s1_rcor24);

        for idx in 0..OBXD_SSEW {
            let idx = idx as usize;

            if qfu.active[idx] != 0 {
                s1_rcor24_arr[idx] = (s1_rcor24_arr[idx]).atan();
            } else {
                s1_rcor24_arr[idx] = 0.0;
            }
        }

        s1_rcor24 = _mm_load_ps(s1_rcor24_arr.as_mut_ptr());

        qfu.reg[ObxdParams::S1 as usize] = _mm_mul_ps(
            s1_rcor24, 
            qfu.coeff[Obxd24dBCoeff::Rcor24inv]);

        let y1: __m128 = res;

        let y2: __m128 = obxd_filter::tptpc(
            &mut qfu.reg[ObxdParams::S2 as usize], 
            y1, 
            g24);

        let y3: __m128 = obxd_filter::tptpc(
            &mut qfu.reg[ObxdParams::S3 as usize], 
            y2, 
            g24);

        let y4: __m128 = obxd_filter::tptpc(
            &mut qfu.reg[ObxdParams::S4 as usize], 
            y3, 
            g24);

        let zero_val: __m128 = _mm_add_ps(
            _mm_mul_ps(
                _mm_sub_ps(
                    one, 
                    pole_mix_scaled), 
                y4), 
            _mm_add_ps(
                pole_mix_scaled, 
                y3));

        let zero_mask: __m128 = _mm_cmpeq_ps(
            pole_mix_inv_int, 
            zero);

        let one_mask: __m128 = _mm_cmpeq_ps(
            pole_mix_inv_int, 
            one);

        let one_val: __m128 = _mm_add_ps(
            _mm_mul_ps(
                _mm_sub_ps(
                    one, 
                    pole_mix_scaled), 
                y3), 
            _mm_mul_ps(
                pole_mix_scaled, 
                y2));

        let two_mask: __m128 = _mm_cmpeq_ps(
            pole_mix_inv_int, 
            two);

        let two_val: __m128 =_mm_add_ps(
            _mm_mul_ps(
                _mm_sub_ps(
                    one, 
                    pole_mix_scaled), 
                y2), 
            _mm_mul_ps(
                pole_mix_scaled, 
                y1));

        let three_mask: __m128 = _mm_cmpeq_ps(pole_mix_inv_int, three);

        let three_val: __m128 = y1;

        let mut mc: __m128 =_mm_add_ps(
            _mm_and_ps(
                zero_mask, 
                zero_val), 
            _mm_and_ps(
                one_mask, 
                one_val));

        mc =_mm_add_ps(
            mc, 
            _mm_add_ps(
                _mm_and_ps(
                    two_mask, 
                    two_val), 
                _mm_and_ps(
                    three_mask, 
                    three_val)));


        // half volume compensation
        let out = _mm_mul_ps(
            mc, 
            _mm_add_ps(
                one, 
                _mm_mul_ps(
                    qfu.coeff[Obxd24dBCoeff::R24], 
                    m128_zero_four_five![])));

        _mm_mul_ps( 
            out, 
            m128_gain_adjustment_4pole![])
    }
}
