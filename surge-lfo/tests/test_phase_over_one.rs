/*
crate::ix!();

// TODO: maybe there are some duplicates here which should be merged
//
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_update_noise_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.update_noise_for_phase_over_one();
        assert_eq!(lfo.wf_history, [0.0, 0.0, 0.0, 0.0]); // wf_history should be [0,0,0,correlated_noise_o2mk2(...)]
    }

    #[test]
    fn test_update_sample_and_hold_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.update_sample_and_hold_for_phase_over_one();
        assert_eq!(lfo.iout, correlated_noise_o2mk2(lfo.target, lfo.noised1, pvalf![lfo.params[LfoParam::Deform]]));
    }

    #[test]
    fn test_update_step_sequencer_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.update_step_sequencer_for_phase_over_one();
        assert_eq!(lfo.step, 1);
        assert!(lfo.ratemult > 0.0);
        assert!(lfo.ratemult.is_finite());
        assert_eq!(lfo.wf_history, [0.0, 0.0, 0.0, lfo.stepsequencer.steps[0]]);
    }

    #[test]
    fn test_update_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.update_for_phase_over_one(LfoShape::SampleAndHold);
        assert_eq!(lfo.iout, correlated_noise_o2mk2(lfo.target, lfo.noised1, pvalf![lfo.params[LfoParam::Deform]]));
    }

    #[test]
    fn test_lfo_update_noise_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.update_noise_for_phase_over_one();
        assert_eq!(lfo.wf_history, [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_lfo_update_sample_and_hold_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.update_sample_and_hold_for_phase_over_one();
        assert_eq!(lfo.iout, 0.0);
    }

    #[test]
    fn test_lfo_update_step_sequencer_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.update_step_sequencer_for_phase_over_one();
        assert_eq!(lfo.wf_history, [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_sample_and_hold() {
        let mut lfo = Lfo::new();
        lfo.update_for_phase_over_one(LfoShape::SampleAndHold);
        assert_eq!(lfo.iout, 0.0);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_noise() {
        let mut lfo = Lfo::new();
        lfo.update_for_phase_over_one(LfoShape::Noise);
        assert_eq!(lfo.wf_history, [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_step_sequencer() {
        let mut lfo = Lfo::new();
        lfo.update_for_phase_over_one(LfoShape::StepSequencer);
        assert_eq!(lfo.wf_history, [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_invalid_shape() {
        let mut lfo = Lfo::new();
        lfo.update_for_phase_over_one(LfoShape::None);
        assert_eq!(lfo.iout, 0.0);
        assert_eq!(lfo.wf_history, [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_lfo_update_step_sequencer_for_phase_over_one_loop_end() {
        let mut lfo = Lfo::new();
        lfo.stepsequencer.loop_end = 4;
        lfo.step = 4;
        lfo.update_step_sequencer_for_phase_over_one();
        assert_eq!(lfo.step, 0);
    }

    #[test]
    fn test_lfo_update_step_sequencer_for_phase_over_one_shuffle_id() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::StartPhase] = 0.5;
        lfo.update_step_sequencer_for_phase_over_one();
        assert_eq!(lfo.shuffle_id, 1);
        assert_eq!(lfo.ratemult, 100.0);
        lfo.update_step_sequencer_for_phase_over_one();
        assert_eq!(lfo.shuffle_id, 0);
        assert_eq!(lfo.ratemult, 0.67);
    }

    #[test]
    fn test_lfo_update_step_sequencer_for_phase_over_one_shuffle_id() {
        let mut lfo = Lfo::default();
        lfo.params[LfoParam::StartPhase] = 0.5;
        lfo.stepsequencer.steps[0] = 0.5;
        lfo.stepsequencer.steps[1] = 0.7;
        lfo.stepsequencer.steps[2] = 0.9;
        lfo.stepsequencer.steps[3] = 1.0;
        lfo.stepsequencer.loop_start = 0;
        lfo.stepsequencer.loop_end = 3;
        lfo.step = 0;
        lfo.shuffle_id = 0;
        lfo.update_step_sequencer_for_phase_over_one();
        assert_eq!(lfo.retrigger_feg, false);
        assert_eq!(lfo.retrigger_aeg, false);
        assert_eq!(lfo.step, 1);
        assert_eq!(lfo.shuffle_id, 1);
        assert_eq!(lfo.ratemult, 0.9900990099009901);

        lfo.update_step_sequencer_for_phase_over_one();
        assert_eq!(lfo.retrigger_feg, false);
        assert_eq!(lfo.retrigger_aeg, false);
        assert_eq!(lfo.step, 2);
        assert_eq!(lfo.shuffle_id, 0);
        assert_eq!(lfo.ratemult, 0.9950248756218906);

        lfo.update_step_sequencer_for_phase_over_one();
        assert_eq!(lfo.retrigger_feg, false);
        assert_eq!(lfo.retrigger_aeg, false);
        assert_eq!(lfo.step, 3);
        assert_eq!(lfo.shuffle_id, 1);
        assert_eq!(lfo.ratemult, 1.0101010101010102);

        lfo.update_step_sequencer_for_phase_over_one();
        assert_eq!(lfo.retrigger_feg, false);
        assert_eq!(lfo.retrigger_aeg, false);
        assert_eq!(lfo.step, 0);
        assert_eq!(lfo.shuffle_id, 0);
        assert_eq!(lfo.ratemult, 1.0050251256281406);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_noise() {
        let mut lfo = Lfo::default();
        lfo.params[LfoParam::Deform] = 0.5;
        lfo.noised1 = 0.1;
        lfo.target = 0.2;
        lfo.update_for_phase_over_one(LfoShape::Noise);
        assert_eq!(lfo.wf_history[0], correlated_noise_o2mk2(0.2, 0.1, 0.5));
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_sample_and_hold() {
        let mut lfo = Lfo::default();
        lfo.params[LfoParam::Deform] = 0.5;
        lfo.noised1 = 0.1;
        lfo.target = 0.2;
        lfo.update_for_phase_over_one(LfoShape::SampleAndHold);
        assert_eq!(lfo.iout, correlated_noise_o2mk2(0.2, 0.1, 0.5));
    }

    #[test]
    fn test_lfo_update_noise_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Deform] = 0.5;
        lfo.update_noise_for_phase_over_one();
        assert_ne!(lfo.wf_history[0], lfo.wf_history[1]);
    }

    #[test]
    fn test_lfo_update_sample_and_hold_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Deform] = 0.5;
        lfo.update_sample_and_hold_for_phase_over_one();
        assert_ne!(lfo.iout, lfo.noised1);
    }

    #[test]
    fn test_lfo_update_step_sequencer_for_phase_over_one_step() {
        let mut lfo = Lfo::new();
        lfo.stepsequencer.steps[0] = 0.5;
        lfo.update_step_sequencer_for_phase_over_one();
        assert_eq!(lfo.wf_history[0], lfo.stepsequencer.steps[0]);
    }

    #[test]
    fn test_lfo_update_step_sequencer_for_phase_over_one_shuffle_id() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::StartPhase] = 0.5;
        lfo.update_step_sequencer_for_phase_over_one();
        assert_eq!(lfo.shuffle_id, 1);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_noise() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Deform] = 0.5;
        lfo.update_for_phase_over_one(LfoShape::Noise);
        assert_ne!(lfo.wf_history[0], lfo.wf_history[1]);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_sample_and_hold() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Deform] = 0.5;
        lfo.update_for_phase_over_one(LfoShape::SampleAndHold);
        assert_ne!(lfo.iout, lfo.noised1);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_step_sequencer() {
        let mut lfo = Lfo::new();
        lfo.stepsequencer.steps[0] = 0.5;
        lfo.update_for_phase_over_one(LfoShape::StepSequencer);
        assert_eq!(lfo.wf_history[0], lfo.stepsequencer.steps[0]);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_sine() {
        let mut lfo = Lfo::new();
        lfo.update_for_phase_over_one(LfoShape::Sine);
        assert_eq!(lfo.wf_history[0], lfo.iout);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_square() {
        let mut lfo = Lfo::new();
        lfo.update_for_phase_over_one(LfoShape::Square);
        assert_eq!(lfo.wf_history[0], lfo.iout);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_triangle() {
        let mut lfo = Lfo::new();
        lfo.update_for_phase_over_one(LfoShape::Triangle);
        //assert_eq!(lfo.wf_history[0], lfo.i
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_noise() {
        let mut lfo = Lfo::new();
        lfo.set_shape(LfoShape::Noise);
        lfo.set_params(&[0.5, 0.0, 0.5]);
        lfo.update_for_phase_over_one(LfoShape::Noise);
        assert!(lfo.wf_history[0] != 0.0);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_sample_and_hold() {
        let mut lfo = Lfo::new();
        lfo.set_shape(LfoShape::SampleAndHold);
        lfo.set_params(&[0.5, 0.0, 0.5]);
        lfo.update_for_phase_over_one(LfoShape::SampleAndHold);
        assert!(lfo.iout != 0.0);
    }

    #[test]
    fn test_lfo_update_for_phase_over_one_unknown_shape() {
        let mut lfo = Lfo::new();
        lfo.set_shape(LfoShape::Unknown);
        lfo.set_params(&[0.5, 0.0, 0.5]);
        lfo.update_for_phase_over_one(LfoShape::Unknown);
        assert!(lfo.phase == 0.0);
    }

    #[test]
    fn test_lfo_update_noise_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.set_shape(LfoShape::Noise);
        lfo.set_params(&[0.5, 0.0, 0.5]);
        lfo.update_noise_for_phase_over_one();
        assert!(lfo.wf_history[0] != 0.0);
    }

    #[test]
    fn test_lfo_update_sample_and_hold_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.set_shape(LfoShape::SampleAndHold);
        lfo.set_params(&[0.5, 0.0, 0.5]);
        lfo.update_sample_and_hold_for_phase_over_one();
        assert!(lfo.iout != 0.0);
    }

    #[test]
    fn test_lfo_update_step_sequencer_for_phase_over_one() {
        let mut lfo = Lfo::new();
        lfo.set_shape(LfoShape::StepSequencer);
        lfo.set_params(&[0.5, 0.0, 0.5]);
        lfo.update_step_sequencer_for_phase_over_one();
        assert!(lfo.step >= 0);
    }
}
*/
