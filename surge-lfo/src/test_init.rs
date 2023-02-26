crate::ix!();

/// These tests cover the `init_phase_from_start_phase()` method with different phase values, as
/// well as the `new()` method.
///
#[cfg(test)]
mod tests {

    use super::*;

    /// This test creates a new LFO instance, sets the starting phase to 0.25, and calls the
    /// `init_phase_from_start_phase()` method. 
    ///
    /// It then checks that the LFO phase is set to 0.25 and that the `phase_initialized` flag is
    /// set to true.
    ///
    #[test]
    fn test_init_phase_from_start_phase() {
        let mut lfo = Lfo::new(TimeUnitHandle::default(), TablesHandle::default());
        lfo.params[LfoParam::StartPhase] = 0.25; // Set starting phase to 0.25

        lfo.init_phase_from_start_phase();

        assert_eq!(lfo.phase, 0.25); // Phase should be set to starting phase
        assert!(lfo.phase_initialized); // Phase should be initialized
    }

    #[test]
    fn test_new() {
        let time_unit = TimeUnitHandle::default();
        let tables = TablesHandle::default();

        let lfo = Lfo::new(time_unit.clone(), tables.clone());

        assert_eq!(lfo.time_unit, time_unit);
        assert_eq!(lfo.tables, tables);
        assert_eq!(lfo.params, LfoParam::new_runtime());
        assert_eq!(lfo.output, 0.0);
        assert_eq!(lfo.phase_initialized, false);
        assert_eq!(lfo.env_val, 0.0);
        assert_eq!(lfo.env_state, LfoEnvState::Delay);
        assert_eq!(lfo.retrigger_feg, false);
        assert_eq!(lfo.retrigger_aeg, false);
        assert_eq!(lfo.phase, 0.0);
        assert_eq!(lfo.target, 0.0);
        assert_eq!(lfo.noise, 0.0);
        assert_eq!(lfo.noised1, 0.0);
        assert_eq!(lfo.env_phase, 0.0);
        assert_eq!(lfo.ratemult, 1.0);
        assert_eq!(lfo.env_releasestart, 0.0);
        assert_eq!(lfo.iout, 0.0);
        assert_eq!(lfo.wf_history, [0.0; 4]);
        assert_eq!(lfo.step, 0);
        assert_eq!(lfo.shuffle_id, 0);
        assert_eq!(lfo.enabled, true);
    }

    #[test]
    fn test_init_phase_from_start_phase_with_negative_phase() {
        let mut lfo = Lfo::new(TimeUnitHandle::default(), TablesHandle::default());
        lfo.params[LfoParam::StartPhase] = -0.25; // Set starting phase to -0.25

        lfo.init_phase_from_start_phase();

        assert_eq!(lfo.phase, 0.75); // Phase should be wrapped to 0.75
        assert!(lfo.phase_initialized); // Phase should be initialized
    }

    #[test]
    fn test_init_phase_from_start_phase_with_phase_greater_than_one() {
        let mut lfo = Lfo::new(TimeUnitHandle::default(), TablesHandle::default());
        lfo.params[LfoParam::StartPhase] = 1.25; // Set starting phase to 1.25

        lfo.init_phase_from_start_phase();

        assert_eq!(lfo.phase, 0.25); // Phase should be wrapped to 0.25
        assert!(lfo.phase_initialized); // Phase should be initialized
    }
}
