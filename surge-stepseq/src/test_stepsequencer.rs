crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    /// The first test checks that the default
    /// implementation sets all fields to their
    /// default values. 
    ///
    #[test]
    fn test_stepsequencer_default() {

        let seq = StepSequencer::default();

        assert_eq!(seq.steps, [0.0; N_STEPSEQUENCER_STEPS]);
        assert_eq!(seq.loop_start, 0);
        assert_eq!(seq.loop_end, 0);
        assert_eq!(seq.shuffle, 0.0);
        assert_eq!(seq.trigmask, 0);
    }

    /// The second test sets all fields to
    /// non-default values, and checks that the
    /// values are correctly set by accessing the
    /// struct's fields directly.
    ///
    #[test]
    fn test_stepsequencer_setters() {

        let mut seq = StepSequencer::default();

        seq.steps[0] = 1.0;
        seq.loop_start = 1;
        seq.loop_end = 2;
        seq.shuffle = 0.5;
        seq.trigmask = 0b1010;

        assert_eq!(seq.steps[0], 1.0);
        assert_eq!(seq.loop_start, 1);
        assert_eq!(seq.loop_end, 2);
        assert_eq!(seq.shuffle, 0.5);
        assert_eq!(seq.trigmask, 0b1010);
    }
}
