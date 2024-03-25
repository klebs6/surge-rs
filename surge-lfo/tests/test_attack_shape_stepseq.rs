/*
crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    /// This test suite checks that the step sequencer loops back to the beginning after hitting
    /// the end, that `shuffle_id` alternates between 0 and 1, and that `ratemult` is computed
    /// correctly based on the value of `start_phase`. You may need to modify the test values
    /// depending on the specific implementation and requirements of your `Lfo` class.
    ///
    #[test]
    fn test_attack_shape_stepseq() {

        let mut lfo = Lfo::new();

        lfo.set_stepsequencer(StepSequencer::new());
        lfo.attack_shape_stepseq(0.5);

        // Check that the step sequencer looped back to the beginning after hitting the end
        for _ in 0..N_STEPSEQUENCER_STEPS * 2 {
            lfo.attack_shape_stepseq(0.5);
        }

        assert_eq!(lfo.wf_history[1], lfo.stepsequencer.steps[0]);

        // Check that shuffle_id alternates between 0 and 1
        let mut prev_shuffle_id = lfo.shuffle_id;

        for _ in 0..10 {
            lfo.attack_shape_stepseq(0.5);
            assert_ne!(prev_shuffle_id, lfo.shuffle_id);
            prev_shuffle_id = lfo.shuffle_id;
        }

        // Check that ratemult is computed correctly
        lfo.attack_shape_stepseq(0.0);
        assert_eq!(lfo.ratemult, 1.0 / 1.5);

        lfo.attack_shape_stepseq(1.0);
        assert_eq!(lfo.ratemult, 1.0 / 0.75);
    }
}
*/
