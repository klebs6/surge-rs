/// To test this code, you could write unit tests
/// that check if the coefficients are set
/// correctly and if the filter processes the
/// input signal as expected. You could also write
/// integration tests that simulate processing an
/// audio signal with a known input and compare
/// the output of the filter to the expected
/// output.
/// 
crate::ix!();

/// These tests create a `BiquadFilter` instance
/// and set its coefficients using the `set_coef`
/// method. They then process some input samples
/// using `process_sample` and check the output
/// against the expected values using `assert_eq`.
/// 
/// There is also a test for stereo processing
/// using the `BiquadFilterStereo`
/// implementation. The `process_block` method is
/// used to process blocks of samples instead of
/// individual samples.
/// 
/// You can run the tests using `cargo test` in
/// the terminal.
///
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_biquad_filter() {

        let mut filter = BiquadFilter::new();

        // Set coefficients
        filter.set_coef(0.5, 0.5, 0.0, 0.5, 0.0, 0.0);

        // Test filter output
        assert_eq!(filter.process_sample(1.0), 0.5);
        assert_eq!(filter.process_sample(2.0), 1.25);
        assert_eq!(filter.process_sample(3.0), 2.125);

        // Change coefficients and test again
        filter.set_coef(1.0, -1.0, 0.0, 1.0, -1.0, 0.0);

        assert_eq!(filter.process_sample(1.0), 1.0);
        assert_eq!(filter.process_sample(2.0), -1.0);
        assert_eq!(filter.process_sample(3.0), 2.0);

        // Test stereo processing
        let mut filter_stereo = BiquadFilterStereo::new();

        filter_stereo.set_coef(0.5, 0.5, 0.0, 0.5, 0.0, 0.0);

        let mut left_input = [1.0, 2.0, 3.0];
        let mut right_input = [4.0, 5.0, 6.0];
        let mut left_output = [0.0; 3];
        let mut right_output = [0.0; 3];

        filter_stereo.process_block(
            &mut left_input, 
            &mut right_input, 
            &mut left_output, 
            &mut right_output
        );

        assert_eq!(left_output, [0.5, 1.25, 2.125]);
        assert_eq!(right_output, [2.0, 2.625, 3.0625]);
    }
}
