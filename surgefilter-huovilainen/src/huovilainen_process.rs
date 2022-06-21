crate::ix!();

impl FilterProcessQuad for HuovilainenLadder {

    /**
      Huovilainen developed an improved and physically correct model of the Moog Ladder
      filter that builds upon the work done by Smith and Stilson. This model inserts
      nonlinearities inside each of the 4 one-pole sections on account of the smoothly
      saturating function of analog transistors. The base-emitter voltages of the
      transistors are considered with an experimental value of 1.22070313 which maintains
      the characteristic sound of the analog Moog. This model also permits
      self-oscillation for resonances greater than 1. The model depends on five hyperbolic
      tangent functions (tanh) for each sample, and an oversampling factor of two
      (preferably higher, if possible). Although a more faithful representation of the
      Moog ladder, these dependencies increase the processing time of the filter
      significantly. Lastly, a half-sample delay is introduced for phase compensation at
      the final stage of the filter. 
     
     References: Huovilainen (2004), Huovilainen (2010), DAFX - Zolzer (ed) (2nd ed)

     Original implementation: Victor Lazzarini for CSound5
    
     Considerations for oversampling:
     http://music.columbia.edu/pipermail/music-dsp/2005-February/062778.html
     http://www.synthmaker.co.uk/dokuwiki/doku.php?id=tutorials:oversampling
     */ 
    fn process_quad(&self, qfu: &mut QuadFilterUnitState, mut input: __m128) -> __m128 {

        unsafe {

            let d_fac: __m128 = _mm_mul_ps( 
                _mm_set_ps1( 0.5 ), 
                _mm_set_ps1( HUOVILAINEN_EXTRA_OVERSAMPLE_INV ) 
            );

            let half:           __m128 = _mm_set_ps1(0.5);
            let one:            __m128 = _mm_set_ps1(1.0);
            let four:           __m128 = _mm_set_ps1(4.0);
            let m18730:         __m128 = _mm_set_ps1(1.8730);
            let m04955:         __m128 = _mm_set_ps1(0.4995);
            let mneg06490:      __m128 = _mm_set_ps1(-0.6490);
            let m09988:         __m128 = _mm_set_ps1(0.9988);
            let mneg39364:      __m128 = _mm_set_ps1(-3.9364 );
            let m18409:         __m128 = _mm_set_ps1(1.8409);
            let m09968:         __m128 = _mm_set_ps1(0.9968 );
            let thermal:        __m128 = _mm_set_ps1(0.000025);
            let oneoverthermal: __m128 = _mm_set_ps1(40000.0);
            let neg2pi:         __m128 = _mm_set_ps1( -2.0 * PI_32 );


            let mut output_os = [z128![]; 2 * (HUOVILAINEN_EXTRA_OVERSAMPLE as usize)];

            for jdx in 0..(2 * HUOVILAINEN_EXTRA_OVERSAMPLE) {
                let jdx = jdx as usize;

                let fc = qfu.coeff[C::Fc];
                let res = qfu.coeff[C::Res];

                let fr:  __m128 = _mm_mul_ps(fc, half);
                let fc2: __m128 = _mm_mul_ps( fc, fc );
                let fc3: __m128 = _mm_mul_ps( fc, fc2 );

                // double fcr = 1.8730 * fc3 + 0.4955 * fc2 - 0.6490 * fc + 0.9988;
                let fcr: __m128 = _mm_add_ps( 
                    _mm_mul_ps( 
                        m18730, 
                        fc3 ), 
                    _mm_add_ps( 
                        _mm_mul_ps( 
                            m04955, 
                            fc2 ), 
                        _mm_add_ps( 
                            _mm_mul_ps( 
                                mneg06490, 
                                fc ), 
                            m09988)));

                //auto acr = -3.9364 * fc2 + 1.8409 * fc + 0.9968;
                let acr: __m128 = _mm_add_ps( 
                    _mm_mul_ps( 
                        mneg39364, 
                        fc2),
                        _mm_add_ps( 
                            _mm_mul_ps( 
                                m18409, 
                                fc),
                                m09968));

                // auto tune = (1.0 - exp(-((2 * PI_32) * f * fcr))) / thermal;
                let tune: __m128 = _mm_mul_ps( 
                    _mm_sub_ps( 
                        one, 
                        fastexp_sse( 
                            _mm_mul_ps( 
                                neg2pi, 
                                _mm_mul_ps( 
                                    fr, 
                                    fcr)))), 
                    oneoverthermal);

                // auto resquad = 4.0 * res * arc;
                let resquad: __m128 = _mm_mul_ps(
                    four,
                    _mm_mul_ps(
                        res,
                        acr)
                );

                for k in 0..HuovilainenCoeff::count() {
                    qfu.coeff[k] = _mm_add_ps(
                        qfu.coeff[k], 
                        _mm_mul_ps( 
                            d_fac, 
                            qfu.dcoeff[k])
                    );
                }

                // float input = input - resQuad * delay[5]. Model as an impulse stream
                // float input = input - resQuad * ( delay[5] - gComp * input ). Model as an impulse stream
                let mut inputx: __m128 = _mm_sub_ps( 
                    input,  
                    _mm_mul_ps( 
                        resquad, 
                        _mm_sub_ps( 
                            qfu.reg[R::Delay as usize + 5], 
                            _mm_mul_ps( 
                                qfu.coeff[C::GainCompensation], 
                                input)))
                );

                // single sample input
                input = _mm_setzero_ps();

                // delay[0] = stage[0] = delay[0] + tune * (tanh(inputx * thermal) - stageTanh[0]);
                qfu.reg[R::Stage] = _mm_add_ps(
                    qfu.reg[R::Delay], 
                    _mm_mul_ps(
                        tune, 
                        _mm_sub_ps(
                            fasttanh_sse_clamped(
                                _mm_mul_ps(
                                    inputx, 
                                    thermal)), 
                            qfu.reg[R::StageTanh]))
                );

                qfu.reg[R::Delay] = qfu.reg[R::Stage];

                for k in 1..4 {

                    // inputx = stage[k-1];
                    inputx = qfu.reg[R::Stage as usize + k - 1 ];

                    // stage[k] = delay[k] + tune * ((stageTanh[k-1] = tanh(inputx * thermal)) - (k != 3 ? stageTanh[k] : tanh(delay[k] * thermal)));
                    qfu.reg[R::StageTanh as usize + k - 1 ] = fasttanh_sse_clamped( 
                        _mm_mul_ps(
                            inputx, 
                            thermal)
                    );

                    qfu.reg[R::Stage as usize + k ] = _mm_add_ps(
                        qfu.reg[ R::Delay as usize + k ],
                        _mm_mul_ps(
                            tune, 
                            _mm_sub_ps(
                                qfu.reg[R::StageTanh as usize + k - 1 ],
                                match k != 3 {
                                    true => qfu.reg[R::StageTanh as usize + k ],
                                    false =>  fasttanh_sse_clamped( 
                                        _mm_mul_ps( 
                                            qfu.reg[R::Delay as usize + k ], 
                                            thermal)),
                                }
                            ))
                    );

                    // delay[k] = stage[k];
                    qfu.reg[R::Delay as usize + k ] = qfu.reg[R::Stage as usize + k];
                }

                // 0.5 sample delay for phase compensation
                // delay[5] = (stage[3] + delay[4]) * 0.5;
                qfu.reg[R::Delay as usize + 5] = _mm_mul_ps( 
                    _mm_set_ps1( 0.5 ), 
                    _mm_add_ps( 
                        qfu.reg[R::Stage as usize +3], 
                        qfu.reg[R::Delay as usize + 4]
                    ) 
                );

                // delay[4] = stage[3];
                qfu.reg[R::Delay as usize + 4] = qfu.reg[R::Stage as usize +3];

                output_os[jdx] = qfu.reg[R::Delay as usize + 5];
            }

            let mut ov: __m128 = _mm_setzero_ps();

            /*
             ** OK this is a bit of a hack but... these are then lanczos factors
             ** sinc( x ) sinc (x / 2 ) backwards only. Really we should do a proper little 
             ** fir around the whole thing but this at least gives us a reconstruction with
             ** some aliasing supression.
             **
             ** Not entirely valid but...
             **
             ** Anyway 2 sin( pi x ) sin ( pi x / 2 ) / ( pi^2 x^2 ) for points -1.5, -1, 0.5, and 0.  
             */ 
            let window_factors: [__m128; 4] = [
                _mm_set_ps1( -0.0636844 ),
                _mm_setzero_ps(),
                _mm_set_ps1( 0.57315916 ),
                _mm_set_ps1(1.0),
            ];

            for idx in 0..(2 * HUOVILAINEN_EXTRA_OVERSAMPLE) {
                let idx = idx as usize;
                ov = _mm_add_ps(
                    ov,
                    _mm_mul_ps(
                        output_os[idx], 
                        window_factors[idx])
                );
            }

            _mm_mul_ps( 
                _mm_set_ps1( 1.5 ), 
                ov 
            )
        }
    }
}
