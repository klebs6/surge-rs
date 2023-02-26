crate::ix!();

/// In the tests, we create a new `Lfo` instance and test each function. 
///
/// The `test_set_phase_for_process` function tests the `set_phase_for_process` method by verifying
/// that the `phase` value is correctly updated based on the tempo sync ratio and LFO rate. 
///
/// The `test_zero_retriggers` function tests the `zero_retriggers` method by verifying that the
/// `retrigger_aeg` and `retrigger_feg` fields are correctly reset to `false`.
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_phase_for_process() {
        let mut lfo = Lfo::default();
        lfo.set_phase_for_process(1.0);
        assert_eq!(lfo.phase, 0.02);

        lfo.set_phase_for_process(2.0);
        assert_eq!(lfo.phase, 0.04);
    }

    #[test]
    fn test_zero_retriggers() {
        let mut lfo = Lfo::default();
        lfo.retrigger_aeg = true;
        lfo.retrigger_feg = true;
        lfo.zero_retriggers();
        assert_eq!(lfo.retrigger_aeg, false);
        assert_eq!(lfo.retrigger_feg, false);
    }
}
