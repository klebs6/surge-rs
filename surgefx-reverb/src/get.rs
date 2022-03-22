ix!();

use crate::{
    Reverb,
    REVERB_MAX_DELAY,
    REVERB_TAPS,
};

impl Reverb {

    pub fn get_fb(&mut self, predelay_time: i32) -> __m128 {

        //TODO: what does fb stand for? feedback?

        unsafe {

            let mut fb: __m128 = {

                let p = self.out_tap.as_mut_ptr();

                let o0  = _mm_load_ps(p);
                let o4  = _mm_load_ps(p.offset(4));
                let o8  = _mm_load_ps(p.offset(8));
                let o12 = _mm_load_ps(p.offset(12));

                let s0 = _mm_add_ps(o0, o4);
                let s1 = _mm_add_ps(o8, o12);

                _mm_add_ps( s0, s1 )
            };

            fb = sum_ps_to_ss(fb);

            let ca: __m128 = _mm_set_ss(-2.0 / (REVERB_TAPS as f32));

            fb = {

                let pos  = self.delay_pos - (predelay_time as usize);
                let mask = (REVERB_MAX_DELAY - 1) as usize;
                let idx  = (pos as usize) & mask;

                let s0 = _mm_mul_ss(ca, fb);
                let s1 = _mm_load_ss(&self.predelay[idx]);

                _mm_add_ss( s0, s1 )
            };

            fb
        }
    }
}
