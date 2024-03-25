/*
crate::ix!();

/// 1. Test all four cases of the `match`
/// statement to ensure that the correct waveform
/// is generated for each case.
///
/// 2. Test edge cases where `deform` is at the
/// boundary between two cases to ensure that the
/// correct waveform is generated.
///
/// 3. Test with different input values for
/// `phase` and `wf_history` to ensure that the
/// waveform is generated correctly.
///
/// 4. Test the function's performance with large
/// datasets to ensure that it runs efficiently.
///
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_shape_stepseq() {

        let mut lfo = Lfo::new();

        lfo.params[LfoParam::Deform] = 0.7;
        lfo.phase = 0.5;
        lfo.wf_history = [0.0, 0.5, 1.0, 0.5];
        lfo.process_shape_stepseq();

        assert_eq!(lfo.iout, 0.75); // expected value based on calculation

        lfo.params[LfoParam::Deform] = 0.0;
        lfo.phase = 0.2;
        lfo.wf_history = [0.0, 0.5, 1.0, 0.5];
        lfo.process_shape_stepseq();

        assert_eq!(lfo.iout, 0.4); // expected value based on calculation

        lfo.params[LfoParam::Deform] = -0.3;
    }

    #[test]
    fn test_process_shape_stepseq() {
        let mut lfo = Lfo::default();

        // Test deform > 0.5
        lfo.params[LfoParam::Deform] = 0.75;
        lfo.phase = 0.5;
        lfo.wf_history = [0.0, 0.25, 0.5, 0.75];
        lfo.process_shape_stepseq();
        assert_approx_eq!(lfo.iout, 0.5625);

        // Test deform ≈ 0.0
        lfo.params[LfoParam::Deform] = -0.00001;
        lfo.phase = 0.75;
        lfo.wf_history = [0.0, 0.25, 0.5, 0.75];
        lfo.process_shape_stepseq();
        assert_approx_eq!(lfo.iout, 0.375);

        // Test deform ≈ -0.5
        lfo.params[LfoParam::Deform] = -0.49999;
        lfo.phase = 0.25;
        lfo.wf_history = [0.0, 0.25, 0.5, 0.75];
        lfo.process_shape_stepseq();
        assert_approx_eq!(lfo.iout, 0.125);

        // Test deform < -0.5
        lfo.params[LfoParam::Deform] = -0.75;
        lfo.phase = 0.25;
        lfo.wf_history = [0.0, 0.25, 0.5, 0.75];
        lfo.process_shape_stepseq();
        assert_approx_eq!(lfo.iout, 0.25);
    }

    /// These test cases cover all four possible
    /// branches of the `match` statement based on
    /// the value of `deform`.
    ///
    /// For the test case I was showing you
    /// earlier, assuming the `Lfo` struct is
    /// properly initialized and the
    /// `cubic_interpolate` function is correctly
    /// implemented, you could use the following
    /// test:
    ///
    /// This test case covers the `deform` > 0.5
    /// branch with cubic interpolation, using
    /// a specific set of values for `phase` and
    /// `wf_history`. 
    ///
    /// You would need to adjust these values
    /// depending on what specific inputs you want
    /// to test.
    ///
    #[test]
    fn test_process_shape_stepseq_cubic() {
        let mut lfo = Lfo::default();

        // Test deform > 0.5 with cubic interpolation
        lfo.params[LfoParam::Deform] = 0.75;
        lfo.phase = 0.25;
        lfo.wf_history = [0.0, 0.25, 0.5, 0.75];
        lfo.process_shape_stepseq();
        assert_approx_eq!(lfo.iout, 0.3125);
    }
}
*/
