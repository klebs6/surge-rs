crate::ix!();

/// Here's an updated test suite for the `Lfo` struct that includes tests for the `get_phaseslider`
/// method:
///
/// These test cases check that the `get_phaseslider` function correctly handles all three possible
/// `LfoShape` values. 
///
/// It correctly handles input values that are outside the range `[0, 1]`.
///
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lfo_new() {
        let lfo = Lfo::new();
        assert_eq!(lfo.phase, 0.0);
        assert_eq!(lfo.wf_history, [0.0; 4]);
        assert_eq!(lfo.params, [0.0; 14]);
        assert_eq!(lfo.target, 0.0);
        assert_eq!(lfo.noised1, 0.0);
        assert_eq!(lfo.noised2, 0.0);
        assert_eq!(lfo.iout, 0.0);
        assert_eq!(lfo.retrigger_feg, false);
        assert_eq!(lfo.retrigger_aeg, false);
        assert_eq!(lfo.ratemult, 0.0);
        assert_eq!(lfo.shuffle_id, 0);
        assert_eq!(lfo.step, 0);
        assert_eq!(lfo.stepsequencer.trigmask, 0);
        assert_eq!(lfo.stepsequencer.loop_start, 0);
        assert_eq!(lfo.stepsequencer.loop_end, 15);
        assert_eq!(lfo.stepsequencer.steps, [0; N_STEPSEQUENCER_STEPS]);
        assert_eq!(lfo.shape, LfoShape::Sine);
    }

    #[test]
    fn test_lfo_update_noise_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.update_noise_for_phase_over_one();
        assert_ne!(lfo.wf_history, [0.0; 4]);
    }

    #[test]
    fn test_lfo_update_sample_and_hold_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.update_sample_and_hold_for_phase_over_one();
        assert_ne!(lfo.iout, 0.0);
    }

    #[test]
    fn test_lfo_update_step_sequencer_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.update_step_sequencer_for_phase_over_one();
        assert_ne!(lfo.wf_history, [0.0; 4]);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_sine() {
        let mut lfo = Lfo::new();
        lfo.update_for_phase_over_one(LfoShape::Sine);
        assert_eq!(lfo.phase, -1.0);
    }

    #[test]
    fn test_lfo_get_phaseslider_step_sequencer() {
        let lfo = Lfo::new();

        let phaseslider = lfo.get_phaseslider(LfoShape::StepSequencer, 0.5);
        assert_eq!(phaseslider, 0.0);

        let phaseslider = lfo.get_phaseslider(LfoShape::StepSequencer, 1.5);
        assert_eq!(phaseslider, 0.5);

        let phaseslider = lfo.get_phaseslider(LfoShape::StepSequencer, -0.5);
        assert_eq!(phaseslider, 0.5);
    }

    #[test]
    fn test_lfo_get_phaseslider_noise() {
        let lfo = Lfo::new();

        let phaseslider = lfo.get_phaseslider(LfoShape::Noise, 0.5);
        assert_eq!(phaseslider, 0.5);

        let phaseslider = lfo.get_phaseslider(LfoShape::Noise, 1.5);
        assert_eq!(phaseslider, 0.5);

        let phaseslider = lfo.get_phaseslider(LfoShape::Noise, -0.5);
        assert_eq!(phaseslider, 0.5);
    }

    #[test]
    fn test_lfo_get_phaseslider_other() {
        let lfo = Lfo::new();

        let phaseslider = lfo.get_phaseslider(LfoShape::Triangle, 0.5);
        assert_eq!(phaseslider, 0.5);

        let phaseslider = lfo.get_phaseslider(LfoShape::Sine, 1.5);
        assert_eq!(phaseslider, 0.5);

        let phaseslider = lfo.get_phaseslider(LfoShape::Square, -0.5);
        assert_eq!(phaseslider, 0.5);
    }
}
