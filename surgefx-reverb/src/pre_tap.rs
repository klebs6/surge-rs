ix!();

use crate::*;

impl Reverb {

    #[inline] pub fn do_pre_tap(&mut self, 
        tap: usize,
        damp4: __m128,
        damp4m1: __m128) 
    {
        unsafe {

            let mut newa: __m128 = _mm_load_ss(&self.delay[self.delay_idx(tap,0)]);
            let newb:     __m128 = _mm_load_ss(&self.delay[self.delay_idx(tap,1)]);

            newa = _mm_unpacklo_ps(newa, newb); // a,b,0,0

            let mut newc: __m128 = _mm_load_ss(&self.delay[self.delay_idx(tap,2)]);
            let newd:     __m128 = _mm_load_ss(&self.delay[self.delay_idx(tap,3)]);

            newc = _mm_unpacklo_ps(newc, newd); // c,d,0,0

            let new4:         __m128 = _mm_movelh_ps(newa, newc); // a,b,c,d
            let mut out_tap4: __m128 = _mm_load_ps(& self.out_tap[tap]);

            out_tap4 = _mm_add_ps(
                _mm_mul_ps(out_tap4, damp4), 
                _mm_mul_ps(new4, damp4m1)
            );

            _mm_store_ps(&mut self.out_tap[tap], out_tap4);
        }
    }
}
