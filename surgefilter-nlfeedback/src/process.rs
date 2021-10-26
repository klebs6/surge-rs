ix!();

use crate::{
    NLFFSaturator,
    NonlinearFeedbackFilter,
    R,C,
};

impl FilterProcessQuad for NonlinearFeedbackFilter {

    fn process_quad(&self, qfu: &mut QuadFilterUnitState, input: __m128) -> __m128 {
        let mut input = input;

        // lower 2 bits of subtype is the stage count
        let stages: i32 = qfu.comb_write_position[0] & 3;

        // next two bits after that select the saturator
        let sat = NLFFSaturator::try_from(((qfu.comb_write_position[0] >> 2) & 3) as usize).unwrap();

        // n.b. stages are zero-indexed so use <=
        for stage in 0..=stages {

            let stage = stage as usize;

            let a1     = &qfu.coeff[C::A1]; 
            let a2     = &qfu.coeff[C::A2]; 
            let b0     = &qfu.coeff[C::B0]; 
            let b1     = &qfu.coeff[C::B1]; 
            let b2     = &qfu.coeff[C::B2];
            let makeup = &qfu.coeff[C::Makeup]; 

            let z1     = &qfu.reg[R::Z1 as usize + stage * 2];
            let z2     = &qfu.reg[R::Z2 as usize + stage * 2];

            input = unsafe {

                // out = z1 + b0 * input
                let out: __m128 = _mm_add_ps(*z1, _mm_mul_ps(*b0, input));

                // nonlinear feedback = saturator(out)
                let nf: __m128 = match sat {

                    NLFFSaturator::Tanh => fasttanh_sse_clamped(out),

                    // note, this is a bit different to Jatin's softclipper
                    NLFFSaturator::Soft => softclip_ps(out), 

                    NLFFSaturator::Ojd  => Self::ojd_waveshaper_ps(&out),

                    NLFFSaturator::Sine => Self::fastsin_ps(&out),
                };

                // z1 = z2 + b1 * input - a1 * nf
                qfu.reg[R::Z1 as usize + stage * 2] = _mm_add_ps(
                    *z2, 
                    _mm_sub_ps(
                        _mm_mul_ps(*b1, input), 
                        _mm_mul_ps(*a1, nf)
                    )
                );

                // z2 = b2 * input - a2 * nf
                qfu.reg[R::Z2 as usize + stage * 2] = _mm_sub_ps(
                    _mm_mul_ps(*b2, input), 
                    _mm_mul_ps(*a2, nf)
                );

                _mm_mul_ps(out, *makeup)
            }
        }

        for coeff in 0..C::len() {
            let idx = coeff as usize;
            qfu.coeff[idx] = unsafe { _mm_add_ps(qfu.coeff[idx], qfu.dcoeff[idx]) };
        }

        input
    }
}
