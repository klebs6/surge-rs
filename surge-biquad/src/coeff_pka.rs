crate::ix!();

/// This trait defines a function for calculating
/// the coefficients of a peaking EQ filter:
/// 
pub trait BiquadCoeffPKA {

    /// This function takes an angular frequency
    /// (`omega`), a quality factor
    /// (`quality_factor`), and a gain (`gain`)
    /// and sets the filter coefficients to
    /// produce a peaking EQ filter with the
    /// specified center frequency, bandwidth, and
    /// gain.
    ///
    fn coeff_pka(&mut self, omega: f64, quality_factor: f64);
}

/// Based on the name of the struct and the method
/// `coeff_pka`, it appears that this code is
/// implementing a biquad filter with peak/notch
/// equalization. 
///
/// The `BiquadFilter` struct likely represents
/// a biquad filter with coefficients that can be
/// set using the `coeff_pka` method, and the
/// `PKA` in `BiquadCoeffPKA` may stand for
/// "peak/notch, gain, and frequency" - parameters
/// that can be adjusted to shape the filter's
/// frequency response.
///
impl BiquadCoeffPKA for BiquadFilter {

    fn coeff_pka(&mut self, omega: f64, qq: f64)
    {
        let cosi:  f64 = omega.cos();
        let sinu:  f64 = omega.sin();
        let reso:  f64 = limit_range(qq as f32, 0.0_f32, 1.0_f32) as f64;
        let q:     f64 = 0.1 + 10.0 * reso * reso;
        let alpha: f64 = sinu / (2.0 * q);
        let b0:    f64 = q * alpha;
        let b2:    f64 = -q * alpha;
        let a0:    f64 = 1.0 + alpha;
        let a1:    f64 = -2.0 * cosi;
        let a2:    f64 = 1.0 - alpha;

        self.set_coef(a0, a1, a2, b0, 0.0, b2);
    }
}
