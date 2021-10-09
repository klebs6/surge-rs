ix!();

use crate::{
    NLSFSaturator,
    R,C,
};

impl FilterProcessQuad for crate::NonlinearStatesFilter<'sr> {

    #[inline] fn process_quad(
        &self, 
        qfu: &mut QuadFilterUnitState<'_>, 
        mut input: __m128) -> __m128 
    {
        // lower 2 bits of subtype is the stage count
        let stages: i32 = qfu.comb_write_position[0] & 3;

        // next two bits after that select the saturator
        let sat = NLSFSaturator::try_from(((qfu.comb_write_position[0] >> 2) & 3) as usize).unwrap();

        // n.b. stages is zero-indexed so use <=
        for stage in 0..=stages {

            macro_rules! z1       { () => { qfu.reg[(R::Z1 as usize) + ((stage * 2) as usize)] } }
            macro_rules! z2       { () => { qfu.reg[(R::Z2 as usize) + ((stage * 2) as usize)] } }
            macro_rules! z1mutref { () => { &mut qfu.reg[(R::Z1 as usize) + ((stage * 2) as usize)] } }
            macro_rules! z2mutref { () => { &mut qfu.reg[(R::Z2 as usize) + ((stage * 2) as usize)] } }

            let a1 = &qfu.coeff[C::A1]; 
            let a2 = &qfu.coeff[C::A2]; 
            let b0 = &qfu.coeff[C::B0]; 
            let b1 = &qfu.coeff[C::B1];
            let b2 = &qfu.coeff[C::B2]; 

            input = unsafe {
                // out = z1 + b0 * input
                let out: __m128 = _mm_add_ps(
                    z1![], 
                    _mm_mul_ps(*b0, input)
                );

                // z1 = z2 + b1 * input - a1 * out
                qfu.reg[(R::Z1 as usize) + ((stage * 2) as usize)] = _mm_add_ps(
                    z2![], 
                    _mm_sub_ps(
                        _mm_mul_ps(*b1, input), 
                        _mm_mul_ps(*a1, out)
                    )
                );

                // z2 = b2 * input - a2 * out
                qfu.reg[(R::Z2 as usize) + ((stage * 2) as usize)] = _mm_sub_ps(
                    _mm_mul_ps(*b2, input), 
                    _mm_mul_ps(*a2, out)
                );

                // now apply a nonlinearity to z1 and z2
                match sat {
                    NLSFSaturator::Tanh => {
                        *z1mutref![] = fasttanh_sse_clamped(z1![]);
                        *z2mutref![] = fasttanh_sse_clamped(z2![]);
                    },
                    NLSFSaturator::SoftClip => {
                        // note, this is a bit different to Jatin's softclipper
                        *z1mutref![] = softclip_ps(z1![]); 
                        *z2mutref![] = softclip_ps(z2![]);
                    },
                }
                out
            }
        }

        for i in 0..C::count() {
            qfu.coeff[i] = unsafe { _mm_add_ps(qfu.coeff[i], qfu.dcoeff[i]) };
        }

        input
    }
}
