crate::ix!();

/// This trait defines a function for calculating
/// the coefficients of a notch filter:
/// 
pub trait BiquadCoeffNotch {

    /// This function takes an angular frequency
    /// (`omega`) and a quality factor
    /// (`quality_factor`) and sets the filter
    /// coefficients to produce a notch filter
    /// with
    ///
    fn set_notch_filter_coefficients(&mut self, omega: f64, quality_factor: f64);
}
