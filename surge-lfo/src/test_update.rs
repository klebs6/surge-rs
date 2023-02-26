crate::ix!();

/// These tests cover all three modes of operation and different start phases and rates.
///
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_maybe_update_env_state_for_attack() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Delay] = 0.0;
        lfo.params[LfoParam::Attack] = 0.1;
        lfo.params[LfoParam::Hold] = 0.2;

        lfo.maybe_update_env_state_for_attack(0.0, 0.1, 0.2);

        assert_eq!(lfo.env_state, LfoEnvState::Hold);
        assert_eq!(lfo.env_val, 1.0);

        lfo.maybe_update_env_state_for_attack(0.0, 0.1, 0.1);

        assert_eq!(lfo.env_state, LfoEnvState::Attack);

        lfo.maybe_update_env_state_for_attack(0.0, 0.2, 0.1);

        assert_eq!(lfo.env_state, LfoEnvState::Decay);
    }

    #[test]
    fn test_update_phase_and_step_for_lfo_mode_keytrigger() {
        let mut lfo = Lfo::new();
        lfo.update_phase_and_step_for_lfo_mode_keytrigger(0.5);

        assert_eq!(lfo.phase, 0.5);
        assert_eq!(lfo.step, 0);
    }

    #[test]
    fn test_update_phase_and_step_for_lfo_mode_random() {
        let mut lfo = Lfo::new();
        lfo.update_phase_and_step_for_lfo_mode_random();

        assert!(lfo.phase >= 0.0 && lfo.phase < 1.0);
        assert!(lfo.step >= 0 && lfo.step < N_STEPSEQUENCER_STEPS);
    }

    #[test]
    fn test_update_phase_and_step_for_lfo_mode_free_run() {
        let mut lfo = Lfo::new();
        lfo.time_unit = TimeUnit::new(120.0, 48000.0);
        lfo.stepsequencer = StepSequencer::new(8, 16);

        lfo.update_phase_and_step_for_lfo_mode_free_run(0.5, 0.0);

        assert_eq!(lfo.step, lfo.stepsequencer.loop_start());

        lfo.update_phase_and_step_for_lfo_mode_free_run(0.5, 1.0);

        assert_eq!(lfo.step, lfo.stepsequencer.loop_start() + 8);
    }

    #[test]
    fn test_rezero_phase_and_step() {
        let mut lfo = Lfo::new();
        lfo.step = 10;
        lfo.phase = 0.5;

        lfo.rezero_phase_and_step();

        assert_eq!(lfo.step, 0);
        assert_eq!(lfo.phase, 0.0);
    }

    #[test]
    fn test_maybe_update_phase_and_step_for_attack() {
        let mut lfo = Lfo::new();
        lfo.set_time_unit(TimeUnit::new(120.0));

        // Test for KeyTrigger mode
        lfo.maybe_update_phase_and_step_for_attack(
            LfoShape::Sine,
            LfoMode::KeyTrigger,
            0.0,
            0.5);

        assert_eq!(lfo.phase, 0.0);
        assert_eq!(lfo.step, 0);

        // Test for Random mode
        lfo.maybe_update_phase_and_step_for_attack(
            LfoShape::Sine,
            LfoMode::Random,
            0.0,
            0.5);

        assert!(lfo.phase >= 0.0 && lfo.phase <= 1.0);
        assert!(lfo.step >= 0 && lfo.step < N_STEPSEQUENCER_STEPS);

        // Test for FreeRun mode
        lfo.maybe_update_phase_and_step_for_attack(
            LfoShape::Sine,
            LfoMode::FreeRun,
            0.0,
            0.5);

        assert_eq!(lfo.step, 0);
        assert_eq!(lfo.phase, 0.0);
    }

    #[test]
    fn test_maybe_update_phase_and_step_for_attack_keytrigger() {
        let mut lfo = Lfo::new();
        lfo.set_params(&[
            0.0, // Delay
            0.0, // Attack
            0.0, // Hold
            0.0, // Release
            0.0, // Depth
            0.0, // Rate
        ]);

        // KeyTrigger mode with start phase 0.0 and rate 1.0
        lfo.maybe_update_phase_and_step_for_attack(
            LfoShape::Sine, LfoMode::KeyTrigger, 0.0, 1.0);

        assert_eq!(lfo.phase, 0.0);
        assert_eq!(lfo.step, 0);

        // KeyTrigger mode with start phase 0.5 and rate 2.0
        lfo.maybe_update_phase_and_step_for_attack(
            LfoShape::Sine, LfoMode::KeyTrigger, 0.5, 2.0);

        assert_eq!(lfo.phase, 0.5);
        assert_eq!(lfo.step, 0);
    }

    #[test]
    fn test_maybe_update_phase_and_step_for_attack_random() {
        let mut lfo = Lfo::new();
        lfo.set_params(&[
            0.0, // Delay
            0.0, // Attack
            0.0, // Hold
            0.0, // Release
            0.0, // Depth
            0.0, // Rate
        ]);

        // Random mode
        lfo.maybe_update_phase_and_step_for_attack(
            LfoShape::Sine, LfoMode::Random, 0.0, 1.0);

        assert!(lfo.phase >= 0.0 && lfo.phase < 1.0);
        assert!(lfo.step >= 0 && lfo.step < N_STEPSEQUENCER_STEPS);
    }

    #[test]
    fn test_maybe_update_phase_and_step_for_attack_free_run() {
        let mut lfo = Lfo::new();
        lfo.set_params(&[
            0.0, // Delay
            0.0, // Attack
            0.0, // Hold
            0.0, // Release
            0.0, // Depth
            0.0, // Rate
        ]);

        // FreeRun mode with start phase 0.0 and rate 1.0
        lfo.maybe_update_phase_and_step_for_attack(
            LfoShape::Sine, LfoMode::FreeRun, 0.0, 1.0);

        assert!(lfo.phase >= 0.0 && lfo.phase < 1.0);
        assert!(lfo.step >= 0 && lfo.step < N_STEPSEQUENCER_STEPS);

        // FreeRun mode with start phase 0.5 and rate 2.0
        lfo.maybe_update_phase_and_step_for_attack(
            LfoShape::Sine, LfoMode::FreeRun, 0.5, 2.0);

        assert!(lfo.phase >= 0.5 && lfo.phase < 1.5);
        assert!(lfo.step >= 0 && lfo.step < N_STEPSEQUENCER_STEPS);
    }
}
