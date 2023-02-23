/// This code implements two traits,
/// `BiquadCoeffBP` and `BiquadCoeffBP2A`, that
/// set the coefficients of a biquad filter for
/// bandpass filtering with different
/// parameterizations.
/// 
crate::ix!();

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

/// The `BiquadCoeffBP2A` trait sets the
/// coefficients for a bandpass filter using the
/// center frequency `omega` and the bandwidth
/// `bandwidth`. It calculates the necessary
/// filter coefficients and passes them to the
/// `set_coef` method of the `BiquadFilter`
/// struct.
///
impl BiquadCoeffBP2A for BiquadFilter {

    fn coeff_bp2a(&mut self, omega: f64, bandwidth: f64)
    {
        let cosi:  f64 = omega.cos();
        let sinu:  f64 = omega.sin();
        let q:     f64 = 1.0 / (0.02 + 30.0 * bandwidth * bandwidth);
        let alpha: f64 = sinu / (2.0 * q);
        let b0:    f64 = alpha;
        let b2:    f64 = -alpha;
        let a0:    f64 = 1.0 + alpha;
        let a1:    f64 = -2.0 * cosi;
        let a2:    f64 = 1.0 - alpha;

        self.set_coef(a0, a1, a2, b0, 0.0, b2);
    }
}
