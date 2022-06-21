crate::ix!();

pub struct DiodeLpfConfig<'a> {
    pub input:            &'a __m128,
    pub alpha:            &'a __m128,
    pub gamma:            &'a __m128,
    pub epsilon:          &'a __m128,
    pub ma0:              &'a __m128,
    pub feedback:         &'a __m128,
    pub feedback_output:  &'a __m128,
    pub output:           &'a mut __m128,
}

impl crate::DiodeLadderFilter {

    #[inline] pub fn do_lpf(
        cfg: DiodeLpfConfig) -> __m128
    {
        // input * gamma + feedback + epsilon * feedback_output
        unsafe {

            let i: __m128 = _mm_add_ps(
                _mm_add_ps(
                    _mm_mul_ps(*cfg.input, *cfg.gamma), 
                    *cfg.feedback
                ), 
                _mm_mul_ps(*cfg.epsilon, *cfg.feedback_output)
            );

            let v: __m128 = _mm_mul_ps(
                _mm_sub_ps(
                    _mm_mul_ps(*cfg.ma0, i), 
                    *cfg.output
                ), *cfg.alpha
            );

            let result: __m128 = _mm_add_ps(v, *cfg.output);

            *cfg.output = _mm_add_ps(v, result);

            result
        }
    }
}
