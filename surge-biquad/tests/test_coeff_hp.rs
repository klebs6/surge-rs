/*
crate::ix!();

/// For testing, you could write unit tests to
/// verify that the filter coefficients are
/// computed correctly for different cutoff
/// frequencies and quality factors or
/// bandwidths. You could also test the filter's
/// frequency response using known input signals
/// and verifying that the output signals match
/// the expected results.
///
/// To test the `BiquadFilter` and `BiquadCoeffHP`
/// implementations, you can create test cases
/// that input known audio signals and compare the
/// output to expected values. For example, you
/// could create a sine wave with a known
/// frequency and amplitude, filter it with
/// a biquad filter with known coefficients, and
/// compare the filtered output to expected
/// values. 
///
/// You could also test the filter's frequency
/// response by inputting a signal with a range of
/// frequencies and comparing the output to
/// expected values. Additionally, you could test
/// the `coeff_hp` and `coeff_hp_with_bw` methods
/// by comparing the calculated coefficients to
/// expected values for known cutoff frequencies
/// and quality factors or bandwidths.
///
/// Here are examples of tests for the
/// BiquadFilter with high-pass filter
/// coefficients:
///
#[cfg(test)]
mod chatgpt_generated_tests {

    use super::*;

    #[test]
    fn test_coeff_hp_base() {
        let mut filter = BiquadFilter::default();
        filter.coeff_hp(0.5, 0.7);
        assert_eq!(filter.process(0.0), 0.0);
    }

    /// These tests verify that the `coeff_hp()` and
    /// `coeff_hp_with_bw()` methods of `BiquadFilter`
    /// produce the expected coefficients for
    /// different input values of `omega`, `q`, and
    /// `bandwidth`.
    #[test]
    fn test_coeff_hp() {
        let mut filter = BiquadFilter::default();
        filter.coeff_hp(0.5, 0.5);
        assert_eq!(filter.a0, 1.0526315789473684);
        assert_eq!(filter.a1, -1.5789473684210527);
        assert_eq!(filter.a2, 0.631578947368421);
        assert_eq!(filter.b0, 0.5263157894736842);
        assert_eq!(filter.b1, -1.0526315789473684);
        assert_eq!(filter.b2, 0.5263157894736842);

        let mut filter = BiquadFilter::default();
        filter.coeff_hp(PI + 0.1, 0.5);
        assert_eq!(filter.a0, 1.0);
        assert_eq!(filter.a1, 0.0);
        assert_eq!(filter.a2, 0.0);
        assert_eq!(filter.b0, 0.0);
        assert_eq!(filter.b1, 0.0);
        assert_eq!(filter.b2, 0.0);
    }

    #[test]
    fn test_coeff_hp_with_bw() {
        let mut filter = BiquadFilter::default();
        filter.coeff_hp_with_bw(0.5, 0.2);
        assert_eq!(filter.a0, 1.4250976310728838);
        assert_eq!(filter.a1, -1.9999999999999987);
        assert_eq!(filter.a2, 0.5749023689271153);
        assert_eq!(filter.b0, 0.7125488155364419);
        assert_eq!(filter.b1, -1.4250976310728838);
        assert_eq!(filter.b2, 0.7125488155364419);
    }


    /// filters a sine wave at 500 Hz with
    /// a high-pass filter at 1000 Hz and Q=0.707,
    /// and checks that the DC offset has been
    /// removed from the output signal.
    ///
    #[test]
    fn test_biquad_hp_filter_sine_wave() {
        let mut filter = BiquadFilter::default();
        let mut output: [f64; 4] = [0.0; 4];

        // high-pass filter with 1000 Hz cutoff frequency and Q=0.707
        filter.coeff_hp(2.0 * PI * 1000.0, 0.707); 

        // input sine wave at 500 Hz
        let input: [f64; 4] = [0.0, 1.0, 0.0, -1.0];

        filter.process(&input, &mut output);

        // check that output signal has DC offset removed
        assert!((output[1] - 0.707).abs() < 1e-6); 
    }

    /// The second test calculates the frequency
    /// response of the high-pass filter at
    /// frequencies from
    /// 10 Hz to 10 kHz, and compares the output
    /// magnitude to the expected magnitude at each
    /// frequency. The expected magnitude is 0 below
    /// the cutoff frequency of 1000 Hz, and
    /// proportional to the frequency above the cutoff
    /// frequency. The tolerance for the output
    /// magnitude is set to
    /// 0.1.
    ///
    #[test]
    fn test_biquad_hp_filter_frequency_response() {

        let mut filter = BiquadFilter::default();

        // high-pass filter with 1000 Hz cutoff frequency and Q=0.707
        filter.coeff_hp(2.0 * PI * 1000.0, 0.707);

        let num_points = 100;
        let f_min = 10.0;
        let f_max = 10000.0;
        let df = (f_max - f_min) / (num_points as f64 - 1.0);
        let mut f = f_min;

        for _ in 0..num_points {
            let omega = 2.0 * PI * f;
            let mut expected_output = [0.0; 4];
            let mut input = [0.0; 4];
            let mut output = [0.0; 4];

            // calculate expected output magnitude
            let expected_mag = if f < 1000.0 {

                0.0 // below cutoff frequency, output should be 0

            } else {

                // above cutoff frequency, output
                // should be proportional to
                // frequency
                let alpha = (omega.sin() / (2.0 * 0.707));
                let numerator = alpha * alpha;
                let denominator = 1.0 + alpha;
                let mag = numerator / denominator;
                mag.sqrt()
            };

            // generate input signal at frequency f
            input[1] = 1.0;

            // filter input signal and measure output magnitude
            filter.process(&input, &mut output);
            let output_mag = output[1].abs();

            // check that output magnitude is within expected tolerance
            assert!((output_mag - expected_mag).abs() < 0.1);

            // increment frequency for next iteration
            f += df;
        }
    }
}
*/
