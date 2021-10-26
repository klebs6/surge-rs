ix!();

use crate::{
    CalculateDerivativesCfg,
    RungeKuttaCoeff,
    RUNGE_KUTTA_EXTRA_OVERSAMPLE,
    RUNGE_KUTTA_EXTRA_OVERSAMPLE_INV,
    RUNGE_KUTTA_DEFAULT_SATURATION,
    RUNGE_KUTTA_DEFAULT_SATURATION_INV,
};

impl FilterProcessQuad for crate::RungeKuttaLadder {

    /**
      | Imitates a Moog resonant filter by Runge-Kutte
      | numerical integration of a differential
      | equation approximately describing
      | the dynamics of the circuit.
      | 
      | Useful references:
      | 
      | Tim Stilson "Analyzing the Moog VCF
      | with Considerations for Digital Implementation"
      | 
      | Sections 1 and 2 are a reasonably good
      | introduction but the model they use
      | is highly idealized.
      | 
      | Timothy E. Stinchcombe "Analysis of
      | the Moog Transistor Ladder and Derivative
      | Filters"
      | 
      | Long, but a very thorough description
      | of how the filter works including its
      | nonlinearities
      | 
      | Antti Huovilainen "Non-linear digital
      | implementation of the moog ladder filter"
      | 
      | Comes close to giving a differential
      | equation for a reasonably realistic
      | model of the filter
      | 
      | The differential equations are:
      | 
      | Y1' = k * (S(x - r * y4) - S(y1))
      | 
      | Y2' = k * (S(y1) - S(y2))
      | 
      | Y3' = k * (S(y2) - S(y3))
      | 
      | Y4' = k * (S(y3) - S(y4)) where k controls
      | the cutoff frequency, r is feedback
      | (<= 4 for stability), and S(x) is a saturation
      | function.
      | 
      | Please see crate LICENCE for the original
      | copyright notice. Thank you!
      |
      */
    fn process_quad(&self, qfu: &mut QuadFilterUnitState<'tables>, 
        mut input: __m128) -> __m128 
    {
        unsafe {

            let mut deriv1:    [__m128; 4] = [z128![]; 4];
            let mut deriv2:    [__m128; 4] = [z128![]; 4];
            let mut deriv3:    [__m128; 4] = [z128![]; 4];
            let mut deriv4:    [__m128; 4] = [z128![]; 4];

            let mut temp_state: [__m128; 4] = [z128![]; 4];

            let state: *mut __m128 = qfu.reg.as_mut_ptr();

            let step_size      = _mm_set_ps1( self.srunit.samplerate_os_inv() * RUNGE_KUTTA_EXTRA_OVERSAMPLE_INV  );
            let half_step_size = _mm_set_ps1( 0.5 * self.srunit.samplerate_os_inv() * RUNGE_KUTTA_EXTRA_OVERSAMPLE_INV );

            let oneoversix     = _mm_set_ps1( 1.0 / 6.0 );
            let two            = _mm_set_ps1(2.0);
            let d_fac           = _mm_set_ps1(RUNGE_KUTTA_EXTRA_OVERSAMPLE_INV);

            let saturation     = _mm_set_ps1(RUNGE_KUTTA_DEFAULT_SATURATION);
            let saturation_inv  = _mm_set_ps1(RUNGE_KUTTA_DEFAULT_SATURATION_INV);

            let mut output_os  = [ z128![]; (RUNGE_KUTTA_EXTRA_OVERSAMPLE as usize) ];

            for item in output_os.iter_mut().take(RUNGE_KUTTA_EXTRA_OVERSAMPLE as usize) {
                for coeff in 0..RungeKuttaCoeff::count() {
                    qfu.coeff[coeff] = _mm_add_ps(
                        qfu.coeff[coeff], 
                        _mm_mul_ps(
                            d_fac,
                            qfu.dcoeff[coeff])
                    );
                }

                let cutoff:    __m128 = qfu.coeff[RungeKuttaCoeff::Cutoff];
                let resonance: __m128 = qfu.coeff[RungeKuttaCoeff::Reso];
                let gain_compensation:     __m128 = qfu.coeff[RungeKuttaCoeff::GainCompensation];

                Self::calculate_derivatives(
                    CalculateDerivativesCfg {
                        input, 
                        dstate: deriv1.as_mut_ptr(), 
                        state, 
                        cutoff, 
                        resonance, 
                        saturation, 
                        saturation_inv, 
                        gain_compensation
                    }
                );

                for i in 0..4 {
                    temp_state[i] = _mm_add_ps( 
                        *state.add(i), 
                        _mm_mul_ps( 
                            half_step_size, 
                            deriv1[i] ) );
                }

                Self::calculate_derivatives(
                    CalculateDerivativesCfg {
                        input, 
                        dstate: deriv2.as_mut_ptr(), 
                        state:  temp_state.as_mut_ptr(), 
                        cutoff, 
                        resonance, 
                        saturation, 
                        saturation_inv, 
                        gain_compensation
                    }
                );

                for i in 0..4 {
                    temp_state[i] = _mm_add_ps( 
                        *state.add(i), 
                        _mm_mul_ps( 
                            half_step_size, 
                            deriv2[i] ) );
                }

                Self::calculate_derivatives(
                    CalculateDerivativesCfg {
                        input, 
                        dstate: deriv3.as_mut_ptr(), 
                        state:  temp_state.as_mut_ptr(), 
                        cutoff, 
                        resonance, 
                        saturation, 
                        saturation_inv, 
                        gain_compensation 
                    }
                );

                for i in 0..4{
                    temp_state[i] = _mm_add_ps( 
                        *state.add(i), 
                        _mm_mul_ps( 
                            half_step_size, 
                            deriv3[i] ) );
                }

                Self::calculate_derivatives(
                    CalculateDerivativesCfg {
                        input, 
                        dstate: deriv4.as_mut_ptr(), 
                        state:  temp_state.as_mut_ptr(), 
                        cutoff, 
                        resonance, 
                        saturation, 
                        saturation_inv, 
                        gain_compensation
                    }
                );

                for i in 0..4 {
                    // state[i] +=(1.0 / 6.0) * step_size * (deriv1[i] + 2.0 * deriv2[i] + 2.0 * deriv3[i] + deriv4[i]);
                    *state.add(i) = _mm_add_ps(
                        *state.add(i), 
                        _mm_mul_ps( 
                            oneoversix, 
                            _mm_mul_ps( 
                                step_size, 
                                _mm_add_ps( 
                                    deriv1[i], 
                                    _mm_add_ps( 
                                        _mm_mul_ps( 
                                            two, 
                                            deriv2[i] 
                                        ), 
                                        _mm_add_ps( 
                                            _mm_mul_ps( 
                                                two, 
                                                deriv3[i] 
                                            ), 
                                            deriv4[i]))))) 
                    );
                }

                *item = *state.add(3);

                // Zero stuffing
                input = _mm_setzero_ps();
            }

            // See comment below
            let mut ov = _mm_setzero_ps();

            let window_factors: [__m128; 4] = [
                _mm_set_ps1( -0.0636844 ),
                _mm_setzero_ps(),
                _mm_set_ps1( 0.57315916 ),
                _mm_set_ps1( 1.0 ),
            ];

            for idx in 0_usize..(RUNGE_KUTTA_EXTRA_OVERSAMPLE as usize) {
                ov = _mm_add_ps( 
                    ov, 
                    _mm_mul_ps( 
                        output_os[idx], 
                        window_factors[idx]
                    ) 
                );
            }

            _mm_mul_ps( 
                _mm_set_ps1( 1.5 ), 
                ov 
            )
        }
    }
}
