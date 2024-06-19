/*!
  | This code defines a BiquadFilter struct that
  | implements a low-pass biquad filter.
  |
  | It provides two functions for calculating the
  | coefficients for the filter: `coeff_lp` and
  | `coeff_lp_with_bw`.
  |
  */

crate::ix!();

/// This trait defines two functions for
/// calculating the coefficients of a low-pass
/// filter:
/// 
pub trait BiquadCoeffLP {

    /// Sets the filter coefficients to produce
    /// a low-pass filter with the specified
    /// cutoff frequency and resonance.
    ///
    fn coeff_lp(&mut self, omega: f64, quality_factor: f64);

    /// Sets the filter coefficients for a given
    /// cutoff frequency and bandwidth
    ///
    fn coeff_lp_with_bw(&mut self, omega: f64, bandwidth: f64);
}

/// The `BiquadCoeffLP` trait represents
/// a low-pass biquad filter. It provides methods
/// for calculating the filter coefficients based
/// on the cutoff frequency and Q factor. The
/// coefficients are then used by the
/// `BiquadFilter` struct to implement the filter.
/// 
/// The `coeff_lp` method calculates the filter
/// coefficients for a given cutoff frequency and
/// Q factor using the formula:
/// 
/// ```pseudocode
/// b0 = (1 - cos(omega)) / 2
/// b1 = 1 - cos(omega)
/// b2 = (1 - cos(omega)) / 2
/// a0 = 1 + alpha
/// a1 = -2 * cos(omega)
/// a2 = 1 - alpha
/// ```
/// 
/// where `omega` is the cutoff frequency in
/// radians, `cos` and `sin` are the cosine and
/// sine functions, and `alpha` is the resonance
/// gain.
/// 
/// The `coeff_lp_with_bw` method is a convenience
/// method that calculates the Q factor from
/// a given bandwidth and passes it to `coeff_lp`.
///
impl BiquadCoeffLP for BiquadFilter {

    /// The `coeff_lp` function takes two
    /// arguments: `omega` and `q`. `omega` is the
    /// filter cutoff frequency in radians per
    /// sample, and `q` is the filter quality
    /// factor. It calculates the filter
    /// coefficients and sets them in the
    /// BiquadFilter struct using the `set_coef`
    /// function.
    /// 
    /// Sets the coefficients for a low-pass
    /// filter using a given cutoff frequency and
    /// quality factor.
    ///
    /// # Arguments
    ///
    /// * `omega` - Cutoff frequency in radians per sample.
    /// * `q` - Quality factor of the filter.
    ///
    /// # Panics
    ///
    /// This function will panic if the cutoff
    /// frequency exceeds half the sampling rate
    /// (i.e. the Nyquist frequency).
    ///
    fn coeff_lp(&mut self, omega: f64, q: f64) {

        if omega > PI {

            // Cutoff frequency exceeds Nyquist
            // frequency, set coefficients to
            // bypass filter
            //
            self.set_coef(
                1.0, 
                0.0, 
                0.0, 
                1.0, 
                0.0, 
                0.0
            );

        } else {

            let cosi:  f64 = omega.cos();
            let sinu:  f64 = omega.sin(); 
            let alpha: f64 = sinu / (2.0 * q); 
            let b0:    f64 = (1.0 - cosi) * 0.5;
            let b1:    f64 = 1.0 - cosi; 
            let b2:    f64 = (1.0 - cosi) * 0.5; 
            let a0:    f64 = 1.0 + alpha; 
            let a1:    f64 = -2.0 * cosi; 
            let a2:    f64 = 1.0 - alpha;

            self.set_coef(a0, a1, a2, b0, b1, b2);
        }
    }

    /// The `coeff_lp_with_bw` function is
    /// a wrapper for `coeff_lp` that takes two
    /// arguments: `omega` and
    /// `bandwidth`. `omega` is the filter cutoff
    /// frequency in radians per sample, and
    /// `bandwidth` is the filter bandwidth in
    /// Hertz. It calculates the `q` factor from
    /// the `bandwidth` and passes it along to the
    /// `coeff_lp` function to calculate the
    /// filter coefficients.
    /// 
    fn coeff_lp_with_bw(&mut self, omega: f64, bandwidth: f64)
    {
        self.coeff_lp(omega, 1.0 / bandwidth);
    }
}
