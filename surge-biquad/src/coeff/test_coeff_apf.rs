/// One way to test this code is to write unit
/// tests that ensure the method is correctly
/// calculating the filter coefficients. Here's an
/// example of a unit test:
///
crate::ix!();

/// The first test verifies that the filter
/// coefficients are correctly calculated when
/// `omega` and `quality_factor` are within the
/// valid range, while the second test ensures
/// that the filter coefficients are set to
/// a simple identity filter when `omega` is not
/// within the range of 0 to pi.
///
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_coeff_apf() {

        let mut filter = BiquadFilter::new();

        filter.coeff_apf(0.5, 0.7);

        assert_eq!(filter.a(), [1.204316613051758, -0.3190300091064021, -0.14673802730161053]);
        assert_eq!(filter.b(), [0.8532616934741209, -1.1704224161035156, 1.204316613051758]);
    }

    #[test]
    fn test_coeff_apf_identity_filter() {

        let mut filter = BiquadFilter::new();

        filter.coeff_apf(4.0, 0.7); // omega is not within 0 to pi

        assert_eq!(filter.a(), [1.0, 0.0, 0.0]);
        assert_eq!(filter.b(), [1.0, 0.0, 0.0]);
    }
}
