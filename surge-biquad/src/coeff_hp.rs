crate::ix!();

/// The `BiquadCoeffHP` trait provides methods for
/// calculating the coefficients of a high-pass
/// biquad filter. 
///
/// A high-pass filter allows frequencies above
/// a certain cutoff frequency to pass through,
/// while attenuating frequencies below the cutoff
/// frequency. 
///
/// The `coeff_hp` method calculates the
/// coefficients based on the filter's cutoff
/// frequency and quality factor, while the
/// `coeff_hp_with_bw` method calculates the
/// coefficients based on the filter's cutoff
/// frequency and bandwidth.
///
pub trait BiquadCoeffHP {

    /// Generates high-pass coefficients for the
    /// biquad filter with the given corner
    /// frequency `omega` (in radians per second)
    /// and quality factor `q`.
    ///
    /// The coefficients are stored in the object
    /// that the method is called on.
    ///
    fn coeff_hp(&mut self, omega: f64, quality_factor: f64);

    /// Generates high-pass coefficients for the
    /// biquad filter with the given corner
    /// frequency `omega` (in radians per second)
    /// and bandwidth `bandwidth` (in Hz). 
    ///
    /// The coefficients are stored in the object
    /// that the method is called on.
    ///
    fn coeff_hp_with_bw(&mut self, omega: f64, bandwidth: f64);
}

/// Implementation of the `BiquadCoeffHP` trait
/// for the `BiquadFilter` struct.
///
/// This code implements two methods for computing
/// the coefficients of a high-pass biquad filter
/// for audio signal processing. The high-pass
/// filter is a type of filter that attenuates
/// frequencies below a certain cutoff frequency
/// and allows higher frequencies to pass through.
/// 
/// The first method `coeff_hp` computes the
/// filter coefficients based on the given cutoff
/// frequency `omega` and quality factor `q`. If
/// the given cutoff frequency is greater than the
/// Nyquist frequency (PI), then the method sets
/// the coefficients to zero to avoid
/// aliasing. Otherwise, the method uses the
/// cutoff frequency and quality factor to compute
/// the filter coefficients using standard
/// formulas for second-order filters.
/// 
/// The second method `coeff_hp_with_bw` is
/// a convenience method that calls `coeff_hp`
/// with the given cutoff frequency and
/// a bandwidth parameter instead of quality
/// factor. The bandwidth is inversely
/// proportional to the quality factor, so the
/// method computes the quality factor as
/// 1/bandwidth and passes it to `coeff_hp`.
/// 
/// To improve the code, it would be helpful to
/// provide more detailed documentation explaining
/// how the filter coefficients are computed and
/// how they affect the filter's frequency
/// response. Additionally, it would be useful to
/// add error handling to `coeff_hp` to ensure
/// that the cutoff frequency is within the valid
/// range of 0 to PI.
impl BiquadCoeffHP for BiquadFilter {

     /// Calculates the coefficients for
     /// a high-pass filter with the given cutoff
     /// frequency and Q factor.
     ///
     /// The `omega` parameter specifies the
     /// cutoff frequency in radians per
     /// sample. The `q` parameter specifies the
     /// Q factor of the filter, which controls
     /// the bandwidth of the filter. Higher
     /// Q values result in a narrower bandwidth
     /// and a more resonant response.
     ///
     /// # Examples
     ///
     /// ```ignore
     /// use biquad::BiquadFilter;
     /// use biquad::coefficients::{BiquadCoeffHP, BiquadCoeff};
     ///
     /// let mut filter = BiquadFilter::default();
     /// filter.coeff_hp(0.5, 0.7);
     /// assert_eq!(filter.process(0.0), 0.0);
     /// ```
    fn coeff_hp(&mut self, omega: f64, q: f64) {

        if omega > PI {
            self.set_coef(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        } else {
            let cosi:  f64 = omega.cos();
            let sinu:  f64 = omega.sin();
            let alpha: f64 = sinu / (2.0 * q);
            let b0:    f64 = (1.0 + cosi) * 0.5;
            let b1:    f64 = -(1.0 + cosi);
            let b2:    f64 = (1.0 + cosi) * 0.5;
            let a0:    f64 = 1.0 + alpha;
            let a1:    f64 = -2.0 * cosi;
            let a2:    f64 = 1.0 - alpha;

            self.set_coef(a0, a1, a2, b0, b1, b2);
        }
    }

     /// Calculates the coefficients for
     /// a high-pass filter with the given cutoff
     /// frequency and bandwidth.
     ///
     /// The `omega` parameter specifies the
     /// cutoff frequency in radians per
     /// sample. The `bandwidth` parameter
     /// specifies the bandwidth of the filter in
     /// octaves, which is the distance between
     /// the -3dB points on the filter response
     /// curve.
     ///
     /// # Examples
     ///
     /// ```ignore
     /// use biquad::BiquadFilter;
     /// use biquad::coefficients::{BiquadCoeffHP, BiquadCoeff};
     ///
     /// let mut filter = BiquadFilter::default();
     /// filter.coeff_hp_with_bw(0.5, 0.1);
     /// assert_eq!(filter.process(0.0), 0.0);
     /// ```
    fn coeff_hp_with_bw(&mut self, omega: f64, bandwidth: f64)
    {
        self.coeff_hp(omega, 1.0 / bandwidth);
    }
}
