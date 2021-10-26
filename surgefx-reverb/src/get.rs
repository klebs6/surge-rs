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
            let mut fb: __m128 = _mm_add_ps(
                _mm_add_ps(
                    _mm_load_ps(
                        self.out_tap.as_mut_ptr()
                    ), _mm_load_ps(
                    self.out_tap.as_mut_ptr().offset(4)
                    )
                ),
                _mm_add_ps(
                    _mm_load_ps(
                        self.out_tap.as_mut_ptr().offset(8)), 
                    _mm_load_ps(
                        self.out_tap.as_mut_ptr().offset(12))
                )
            );

            fb = sum_ps_to_ss(fb);

            let ca: __m128 = _mm_set_ss(-2.0 / (REVERB_TAPS as f32));

            fb = _mm_add_ss(
                _mm_mul_ss(ca, fb),
                _mm_load_ss(&self.predelay[(( self.delay_pos - (predelay_time as usize)) as usize) & ((REVERB_MAX_DELAY - 1) as usize)])
            );

            fb
        }
    }
}
