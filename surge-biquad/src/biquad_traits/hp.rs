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
