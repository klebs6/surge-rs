crate::ix!();

pub struct CalculateDerivativesCfg {
    pub input:               __m128, 
    pub dstate:              *mut __m128, 
    pub state:               *mut __m128, 
    pub cutoff:              __m128, 
    pub resonance:           __m128, 
    pub saturation:          __m128, 
    pub saturation_inv:      __m128, 
    pub gain_compensation:   __m128,
}

impl RungeKuttaLadder {

    #[inline] pub fn calculate_derivatives(cfg: CalculateDerivativesCfg)
    {
        unsafe {
            let satstate0: __m128 = Self::clip( 
                *cfg.state.offset(0), 
                cfg.saturation, 
                cfg.saturation_inv 
            );

            let satstate1: __m128 = Self::clip( 
                *cfg.state.offset(1), 
                cfg.saturation, 
                cfg.saturation_inv 
            );

            let satstate2: __m128 = Self::clip( 
                *cfg.state.offset(2), 
                cfg.saturation, 
                cfg.saturation_inv 
            );

            // cfg.dstate[0] = cfg.cutoff * (clip(cfg.input - cfg.resonance * cfg.state[3], cfg.saturation, cfg.saturation_inv) - satstate0);
            // Modify
            // cfg.dstate[0] = cfg.cutoff * (clip(cfg.input - cfg.resonance * ( cfg.state[3] - cfg.gain_compensation * cfg.input ), cfg.saturation, cfg.saturation_inv) - satstate0);
            let startstate: __m128 = Self::clip( 
                _mm_sub_ps( 
                    cfg.input, 
                    _mm_mul_ps( 
                        cfg.resonance, 
                        _mm_sub_ps( 
                            *cfg.state.offset(3), 
                            _mm_mul_ps( 
                                cfg.gain_compensation, 
                                cfg.input 
                            ) 
                        ) 
                    ) 
                ), 
                cfg.saturation, 
                cfg.saturation_inv 
            );

            *cfg.dstate.offset(0) = _mm_mul_ps( 
                cfg.cutoff, 
                _mm_sub_ps( 
                    startstate, 
                    satstate0 
                ) 
            );

            // cfg.dstate[1] = cfg.cutoff * (satstate0 - satstate1);
            *cfg.dstate.offset(1) = _mm_mul_ps( 
                cfg.cutoff, 
                _mm_sub_ps( 
                    satstate0, 
                    satstate1
                )
            );

            // cfg.dstate[2] = cfg.cutoff * (satstate1 - satstate2);
            *cfg.dstate.offset(2) = _mm_mul_ps( 
                cfg.cutoff, 
                _mm_sub_ps( 
                    satstate1, satstate2
                )
            );

            // cfg.dstate[3] = cfg.cutoff * (satstate2 - clip(cfg.state[3], cfg.saturation, cfg.saturation_inv));
            *cfg.dstate.offset(3) = _mm_mul_ps( 
                cfg.cutoff, 
                _mm_sub_ps( 
                    satstate2, 
                    Self::clip( 
                        *cfg.state.offset(3), 
                        cfg.saturation, 
                        cfg.saturation_inv)
                )
            );
        }
    }
}
