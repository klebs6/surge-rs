crate::ix!();

/// A trait defining a function `coeff_apf` to set
/// the coefficients of an all-pass filter. The
/// all-pass filter is a type of IIR filter where
/// all frequencies are passed through, but with
/// a phase shift. This trait is used for creating
/// and modifying all-pass filters.
///
pub trait BiquadCoeffAPF {

    /// Calculates the coefficients of a biquad
    /// all-pass filter based on the given `omega`
    /// and `quality_factor` parameters.
    ///
    /// If `omega` is not within the range of 0 to
    /// pi, the filter coefficients are set to
    /// a simple identity filter.
    ///
    /// # Arguments
    ///
    /// * `omega` - Angular frequency of the filter (in radians)
    /// * `quality_factor` - Quality factor of the filter
    ///
    /// ```ignore
    /// use my_crate::BiquadFilter;
    ///
    /// let mut filter = BiquadFilter::new();
    ///
    /// filter.coeff_apf(0.5, 0.7);
    /// ```
    ///
    fn coeff_apf(&mut self, omega: f64, quality_factor: f64);
}

impl BiquadCoeffAPF for BiquadFilter {

    /// This code implements a method called
    /// `coeff_apf` that calculates the
    /// coefficients of a biquad all-pass filter
    /// based on the given `omega` and
    /// `quality_factor` parameters. 
    ///
    /// If `omega` is not within the range of 0 to
    /// pi, the method sets the filter
    /// coefficients to a simple identity filter.
    ///
    fn coeff_apf(&mut self, omega: f64, quality_factor: f64)
    {
        if !(0.0..=PI).contains(&omega) {

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
            let alpha: f64 = sinu / (2.0 * quality_factor);
            let b0:    f64 = 1.0 - alpha;
            let b1:    f64 = -2.0 * cosi;
            let b2:    f64 = 1.0 + alpha;
            let a0:    f64 = 1.0 + alpha;
            let a1:    f64 = -2.0 * cosi;
            let a2:    f64 = 1.0 - alpha;

            self.set_coef(a0, a1, a2, b0, b1, b2);
        }
    }
}
