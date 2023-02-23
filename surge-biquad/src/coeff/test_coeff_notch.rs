crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;
    use std::f64::consts::PI;

    /// In the first test, we're testing the case
    /// where `omega` is greater than PI. In this
    /// case, the filter should be set to a bypass
    /// state, where the output is the same as the
    /// input. We're checking that the filter
    /// coefficients are set accordingly.
    ///
    #[test]
    fn test_coeff_notch_omega_gt_pi() {
        let mut filter = BiquadFilter::new();
        filter.set_notch_filter_coefficients(PI + 1.0, 0.5);
        assert_eq!(filter.get_a0(), 1.0);
        assert_eq!(filter.get_a1(), 0.0);
        assert_eq!(filter.get_a2(), 0.0);
        assert_eq!(filter.get_b0(), 1.0);
        assert_eq!(filter.get_b1(), 0.0);
        assert_eq!(filter.get_b2(), 0.0);
    }

    /// In the second test, we're testing a more
    /// general case where `omega` is less than
    /// PI. We're checking that the filter
    /// coefficients are set to the correct values
    /// based on the input `omega` and `qq`.
    ///
    #[test]
    fn test_coeff_notch() {
        let mut filter = BiquadFilter::new();
        filter.set_notch_filter_coefficients(PI / 4.0, 0.5);
        assert_eq!(filter.get_a0(), 1.1273355043198083);
        assert_eq!(filter.get_a1(), -1.3814544901888912);
        assert_eq!(filter.get_a2(), 0.8533280356801916);
        assert_eq!(filter.get_b0(), 1.0);
        assert_eq!(filter.get_b1(), -1.3814544901888912);
        assert_eq!(filter.get_b2(), 0.9806635400000001);
    }
}

/// These tests cover some edge cases such as when
/// `omega` is greater than pi, when the quality
/// factor is zero or one, and when the quality
/// factor is between zero and one.
///
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_coeff_notch_when_omega_is_greater_than_pi() {

        let mut filter = BiquadFilter::default();
        filter.set_notch_filter_coefficients(PI + 1.0, 0.5);

        let expected = BiquadCoefficients::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(filter.get_coefs(), expected);
    }

    #[test]
    fn test_coeff_notch_when_omega_is_less_than_pi() {

        let mut filter = BiquadFilter::default();
        filter.set_notch_filter_coefficients(2.0, 0.5);

        let cosi = 2.0_f64.cos();
        let sinu = 2.0_f64.sin();
        let reso = 0.5_f64;
        let q = 1.0 / (0.02 + 30.0 * reso * reso);
        let alpha = sinu / (2.0 * q);
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cosi;
        let a2 = 1.0 - alpha;
        let b0 = 1.0;
        let b1 = -2.0 * cosi;
        let b2 = 1.0;

        let expected = BiquadCoefficients::new(a0, a1, a2, b0, b1, b2);
        assert_eq!(filter.get_coefs(), expected);
    }

    #[test]
    fn test_coeff_notch_with_zero_quality_factor() {

        let mut filter = BiquadFilter::default();
        filter.set_notch_filter_coefficients(0.5, 0.0);

        let expected = BiquadCoefficients::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(filter.get_coefs(), expected);
    }

    #[test]
    fn test_coeff_notch_with_one_quality_factor() {

        let mut filter = BiquadFilter::default();
        filter.set_notch_filter_coefficients(0.5, 1.0);

        let cosi = 0.5_f64.cos();
        let sinu = 0.5_f64.sin();
        let reso = 1.0_f64;
        let q = 1.0 / (0.02 + 30.0 * reso * reso);
        let alpha = sinu / (2.0 * q);
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cosi;
        let a2 = 1.0 - alpha;
        let b0 = 1.0;
        let b1 = -2.0 * cosi;
        let b2 = 1.0;

        let expected = BiquadCoefficients::new(a0, a1, a2, b0, b1, b2);
        assert_eq!(filter.get_coefs(), expected);
    }
}

