crate::ix!();

impl DiodeLadderFilter {

    #[cfg(target_arch = "x86_64")] 
    unsafe fn update_qfu(qfu: &mut QuadFilterUnitState) {

        for i in 0..N_COEFFMAKER_COEFFS {
            qfu.coeff[i] = _mm_add_ps(qfu.coeff[i], qfu.dcoeff[i]); 
        }
    }
}

impl FilterProcessQuad for DiodeLadderFilter {

    #[cfg(target_arch = "x86_64")] 
    fn process_quad(&self, qfu: &mut QuadFilterUnitState, input: __m128) -> __m128 {

        unsafe {

            Self::update_qfu(qfu);

            let zero: __m128 = _mm_set_ps1(0.0);
            let one:  __m128 = _mm_set_ps1(1.0);
            let half: __m128 = _mm_set_ps1(0.5);

            macro_rules! one_plus {
                ($x:ident) => {
                    _mm_add_ps($x, one)
                }
            }

            macro_rules! half_of {
                ($x:ident) => {
                    _mm_mul_ps( qfu.coeff[C::$x], half)
                }
            }

            let sg3: __m128 = qfu.coeff[C::G4];

            let sg2: __m128 = _mm_mul_ps(sg3, qfu.coeff[C::G3]);
            let sg1: __m128 = _mm_mul_ps(sg2, qfu.coeff[C::G2]);

            // sg4 is 1.0, just inline it

            let g: __m128 = qfu.coeff[C::G];

            // g plus one, common so do it only once
            let gp1: __m128 = one_plus![g];

            let hg: __m128 = half_of![G];

            /// 1.0 / ($a - $b * $c)
            macro_rules! calc_beta {
                ($a:ident, $b:ident, $c:ident) => {
                    _mm_rcp_ps(
                        _mm_sub_ps(
                            $a, 
                            _mm_mul_ps( $b, qfu.coeff[C::$c])
                        )
                    )
                }
            }

            /// $a * $b + 1.0
            macro_rules! calc_gamma {
                ($a:ident, $b:ident) => {
                    _mm_add_ps(
                        _mm_mul_ps(
                            qfu.coeff[C::$a], 
                            qfu.coeff[C::$b]
                        ), 
                        one
                    )
                }
            }

            let beta1: __m128 = calc_beta![gp1, g, G2];
            let beta2: __m128 = calc_beta![gp1, hg, G3];
            let beta3: __m128 = calc_beta![gp1, hg, G4];

            // 1.0 / gp1
            let beta4: __m128 = _mm_rcp_ps(gp1);

            // nothing to compute for deltas, inline them

            let gamma1: __m128 = calc_gamma![G1, G2];
            let gamma2: __m128 = calc_gamma![G2, G3];
            let gamma3: __m128 = calc_gamma![G3, G4];

            // gamma4 is always 1.0, just inline it

            // nothing to compute for epsilons or ma0, inline them

            macro_rules! fb_output {
                ($beta:ident, $z:ident) => {
                    Self::get_feedback_output(
                        &$beta, 
                        &zero, 
                        &zero, 
                        &qfu.reg[R::$z]
                    )
                };
                ($beta:ident, $g:ident, $fb:ident, $z:ident) => {
                    Self::get_feedback_output(
                        &$beta, 
                        &$g, 
                        &qfu.reg[R::$fb], 
                        &qfu.reg[R::$z]
                    )
                }
            }

            // feedback4 is always zero, inline it
            let feedback3: __m128 = fb_output![beta4, Z4];
            let feedback2: __m128 = fb_output![beta3, hg, Feedback3, Z3];
            let feedback1: __m128 = fb_output![beta2, hg, Feedback2, Z2];

            macro_rules! sigma_summand {
                ($sg:ident, $beta:ident, $g:ident, $fb:ident, $z:ident) => {
                    _mm_mul_ps( 
                        $sg, 
                        Self::get_feedback_output(
                            &$beta, 
                            &$g,  
                            &$fb, 
                            &qfu.reg[R::$z]
                        ) 
                    )
                }
            }

            let sigma: __m128 = _mm_add_ps(
                _mm_add_ps( 
                    _mm_add_ps( 
                        sigma_summand![sg1, beta1, g,  feedback1, Z1],
                        sigma_summand![sg2, beta2, hg, feedback2, Z2],
                    ), 
                    sigma_summand![sg3, beta3, hg, feedback3, Z3]
                ),
                sigma_summand![one, beta4, zero, zero, Z4]
            );

            qfu.reg[R::Feedback3] = feedback3;
            qfu.reg[R::Feedback2] = feedback2;
            qfu.reg[R::Feedback1] = feedback1;

            macro_rules! kmodded_by_pt3_plus_one {
                () => {
                    _mm_add_ps(
                        _mm_mul_ps(
                            _mm_set_ps1(0.3), 
                            qfu.coeff[C::KModded]
                        ), 
                        one
                    )
                }
            }

            macro_rules! calculate_gain_compensation {
                ($input:ident) => {
                    _mm_mul_ps(
                        kmodded_by_pt3_plus_one![], 
                        $input
                    )
                }
            }

            // gain compensation
            let comp: __m128 = calculate_gain_compensation![input];

            macro_rules! comp_minus_kmodded_by_sigma {
                ($comp:ident, $sigma:ident) => {
                    _mm_sub_ps(
                        $comp, 
                        _mm_mul_ps(
                            qfu.coeff[C::KModded], 
                            $sigma
                        )
                    )
                }
            }

            macro_rules! kmodded_by_gamma_plus_one {
                () => {
                    _mm_add_ps(
                        _mm_mul_ps(
                            qfu.coeff[C::KModded], 
                            qfu.coeff[C::Gamma]
                        ), 
                        one
                    )
                }
            }

            // (comp - kmodded * sigma) / (kmodded * gamma + 1.0)
            let u: __m128 = _mm_div_ps(
                comp_minus_kmodded_by_sigma![comp, sigma], 
                kmodded_by_gamma_plus_one![]
            );

            let result1: __m128 = Self::do_lpf(
                DiodeLpfConfig {
                    input:    &u, 
                    alpha:    &qfu.coeff[C::Alpha], 
                    gamma:    &gamma1,    
                    epsilon:  &qfu.coeff[C::G2],  
                    ma0:      &one, 
                    feedback: &feedback1,
                    feedback_output: &Self::get_feedback_output(
                        &beta1, 
                        &g, 
                        &feedback1, 
                        &qfu.reg[R::Z1]
                    ), 
                    output: &mut qfu.reg[R::Z1],
                },
            );

            let result2: __m128 = Self::do_lpf(
                DiodeLpfConfig {
                    input:           &result1, 
                    alpha:           &qfu.coeff[C::Alpha], 
                    gamma:           &gamma2,   
                    epsilon:         &qfu.coeff[C::G3], 
                    ma0:             &half, 
                    feedback:        &feedback2,
                    feedback_output: &Self::get_feedback_output(
                        &beta2, 
                        &hg, 
                        &feedback2, 
                        &qfu.reg[R::Z2]
                    ), 
                    output: &mut qfu.reg[R::Z2],
                },
            );

            let result3: __m128 = Self::do_lpf(
                DiodeLpfConfig {
                    input:           &result2, 
                    alpha:           &qfu.coeff[C::Alpha], 
                    gamma:           &gamma3, 
                    epsilon:         &qfu.coeff[C::G4], 
                    ma0:             &half, 
                    feedback:        &feedback3,
                    feedback_output: &Self::get_feedback_output( 
                        &beta3, 
                        &hg, 
                        &feedback3, 
                        &qfu.reg[R::Z3]
                    ), 
                    output: &mut qfu.reg[R::Z3],
                },
            );

            Self::do_lpf(
                DiodeLpfConfig {
                    input:           &result3, 
                    alpha:           &qfu.coeff[C::Alpha], 
                    gamma:           &one, 
                    epsilon:         &zero, 
                    ma0:             &half, 
                    feedback:        &zero,
                    feedback_output: &Self::get_feedback_output(
                        &beta4, 
                        &zero, 
                        &zero, 
                        &qfu.reg[R::Z4]
                    ), 
                    output: &mut qfu.reg[R::Z4],
                },
            )
        }
    }
}
