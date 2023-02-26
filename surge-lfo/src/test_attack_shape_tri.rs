crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    /// This test suite tests the behavior of the `attack_shape_tri` function for both unipolar and
    /// bipolar modes, as well as checking that the phase cycles back to 0.0 after a full period.
    ///
    #[test]
    fn test_attack_shape_tri() {

        // Set up LFO with default values
        let mut lfo = Lfo::new();

        // Set unipolar to false
        lfo.params[LfoParam::Unipolar] = 0.0;

        // Set initial phase to 0.0
        lfo.phase = 0.0;

        // Call function once
        lfo.attack_shape_tri();

        // Check that phase has been updated correctly
        assert_eq!(lfo.phase, 0.25);

        // Call function four more times
        for _ in 0..4 {
            lfo.attack_shape_tri();
        }

        // Check that phase has cycled back to 0.0
        assert_eq!(lfo.phase, 0.0);

        // Set unipolar to true
        lfo.params[LfoParam::Unipolar] = 1.0;

        // Call function once
        lfo.attack_shape_tri();

        // Check that phase has not changed
        assert_eq!(lfo.phase, 0.0);
    }
}

