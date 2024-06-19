/*!
  | This code implements two traits,
  | `BiquadCoeffBP` and `BiquadCoeffBP2A`, that
  | set the coefficients of a biquad filter for
  | bandpass filtering with different
  | parameterizations.
  |
  */

crate::ix!();

/// This trait defines a function for calculating
/// the coefficients of a band-pass filter:
/// 
pub trait BiquadCoeffBP {

    /// This function takes an angular frequency
    /// (`omega`) and a quality factor
    /// (`quality_factor`) and sets the filter
    /// coefficients to produce a band-pass filter
    /// with the specified center frequency and
    /// resonance.
    ///
    fn coeff_bp(&mut self, omega: f64, quality_factor: f64);
}

/// The `BiquadCoeffBP` trait sets the
/// coefficients for a bandpass filter using the
/// center frequency `omega` and the quality
/// factor `quality_factor`. It calculates the
/// necessary filter coefficients and passes them
/// to the `set_coef` method of the `BiquadFilter`
/// struct.
///
impl BiquadCoeffBP for BiquadFilter {

    fn coeff_bp(&mut self, omega: f64, quality_factor: f64)
    {
        let cosi:  f64 = omega.cos();
        let sinu:  f64 = omega.sin();
        let alpha: f64 = sinu / (2.0 * quality_factor);
        let b0:    f64 = alpha;
        let b2:    f64 = -alpha;
        let a0:    f64 = 1.0 + alpha;
        let a1:    f64 = -2.0 * cosi;
        let a2:    f64 = 1.0 - alpha;

        self.set_coef(a0, a1, a2, b0, 0.0, b2);
    }
}
