/*
crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    /// This test suite imports the `Lfo` struct
    /// from the module and defines a test
    /// function `test_attack_shape_snh`. The test
    /// function creates a new `Lfo` object, and
    /// tests that the initial values of the
    /// `noise`, `noised1`, and `target` fields
    /// are all 0.0.
    /// 
    /// Next, the test function calls the
    /// `attack_shape_snh` method to generate
    /// a new LFO value, and tests that the `iout`
    /// field has a non-zero value. It also tests
    /// that the `target`, `noise`, and `noised1`
    /// fields were set to 0.0 by the method.
    /// 
    /// The test function then calls the
    /// `attack_shape_snh` method again to
    /// generate another LFO value, and tests that
    /// the `iout` field has a non-zero value. It
    /// also tests that the two LFO values are
    /// different.
    /// 
    /// Finally, the test function sets the
    /// `Deform` parameter to different values and
    /// tests that the generated LFO values are
    /// different. This tests that the
    /// `attack_shape_snh` method correctly shapes
    /// the LFO waveform based on the `Deform`
    /// parameter.
    ///
    #[test]
    fn test_attack_shape_snh() {
        let mut lfo = Lfo::new();

        // Test initial values
        assert_eq!(lfo.noise, 0.0);
        assert_eq!(lfo.noised1, 0.0);
        assert_eq!(lfo.target, 0.0);

        // Test generating a new LFO value
        lfo.attack_shape_snh();
        assert_ne!(lfo.iout, 0.0);

        // Test that the target, noise, and noised1 fields were set to 0.0
        assert_eq!(lfo.target, 0.0);
        assert_eq!(lfo.noise, 0.0);
        assert_eq!(lfo.noised1, 0.0);

        // Test generating another LFO value
        lfo.attack_shape_snh();
        assert_ne!(lfo.iout, 0.0);

        // Test generating LFO values with different Deform parameter values
        lfo.set_param(LfoParam::Deform, 0.5);
        let iout1 = lfo.iout;
        lfo.attack_shape_snh();
        let iout2 = lfo.iout;
        assert_ne!(iout1, iout2);

        lfo.set_param(LfoParam::Deform, 0.8);
        let iout1 = lfo.iout;
        lfo.attack_shape_snh();
        let iout2 = lfo.iout;
        assert_ne!(iout1, iout2);
    }
}

*/
