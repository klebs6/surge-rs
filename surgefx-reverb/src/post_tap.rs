crate::ix!();

impl Reverb {

    #[inline] pub fn do_post_tap(&mut self, 
        tap: usize, 
        fb4: __m128, 
        l: &mut __m128, 
        r: &mut __m128) 
    {
        unsafe {

            let ot: __m128 = _mm_load_ps(&self.out_tap[tap]);

            let dfb: __m128 = _mm_load_ps(&self.delay_fb[tap]);

            let a: __m128 = _mm_mul_ps(dfb, _mm_add_ps(fb4, ot));

            _mm_store_ps(&mut self.delay[( self.delay_pos << REVERB_TAP_BITS) + tap as usize], a);

            *l = _mm_add_ps(*l, _mm_mul_ps(ot, _mm_load_ps(&
                        self.delay_pan_l[tap])));

            *r = _mm_add_ps(*r, _mm_mul_ps(ot, _mm_load_ps(&
                        self.delay_pan_r[tap])));
        }
    }
}
