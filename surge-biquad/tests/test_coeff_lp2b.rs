/*
crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    /// This test case creates a new
    /// `BiquadFilter` and sets its coefficients
    /// using the `coeff_lp2b` method with
    /// a center frequency of
    /// 500 Hz and a quality factor of 0.707. It
    /// then creates an input signal with
    /// a frequency of
    /// 500 Hz and an amplitude of 1.0, and
    /// processes this signal through the filter. 
    ///
    /// The output signal is then compared to an
    /// expected output signal with the same
    /// frequency and an amplitude of 0.707, using
    /// the `similar` method with a tolerance of
    /// 1e-2.
    /// 
    #[test]
    fn test_lp2b_filter_sine_wave() {

        let mut filter = BiquadFilter::new();

        filter.coeff_lp2b(2.0 * PI * 500.0, 0.707);

        let mut input_signal = Signal::new(44100, 1);
        let mut output_signal = Signal::new(44100, 1);

        input_signal.set_frequency(500.0);
        input_signal.set_amplitude(1.0);

        for i in 0..44100 {
            output_signal.set_sample(i, filter.process(input_signal.sample(i)));
        }

        let expected_output = Signal::new(44100, 1);

        expected_output.set_frequency(500.0);
        expected_output.set_amplitude(0.707);

        assert!(output_signal.similar(&expected_output, 1e-2));
    }
}
*/
