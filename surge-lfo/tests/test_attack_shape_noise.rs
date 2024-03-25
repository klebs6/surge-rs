/*
crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    /// In this test, we create a new `Lfo` instance and set the `Deform` parameter to 0.5 and the
    /// `phase` to 0.5. We then call `attack_shape_noise()` on the `Lfo` instance and check that
    /// the `noise`, `noised1`, and `target` fields are all set to 0.0, as expected. We also check
    /// that the `wf_history` field is updated correctly using the `correlated_noise_o2mk2`
    /// function, with the expected input parameters and scaling by the `phase`. Finally, we check
    /// that the `phase` field is reset to 0.0.
    ///
    #[test]
    fn test_attack_shape_noise() {

        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Deform] = 0.5;
        lfo.phase = 0.5;

        lfo.attack_shape_noise();

        assert_eq!(lfo.noise, 0.0);
        assert_eq!(lfo.noised1, 0.0);
        assert_eq!(lfo.target, 0.0);

        let expected_wf_history = [
            correlated_noise_o2mk2(0.0, 0.0, 0.5) * 0.5,
            correlated_noise_o2mk2(0.0, 0.0, 0.5) * 0.5,
            correlated_noise_o2mk2(0.0, 0.0, 0.5) * 0.5,
            correlated_noise_o2mk2(0.0, 0.0, 0.5) * 0.5,
        ];

        assert_eq!(lfo.wf_history, expected_wf_history);
        assert_eq!(lfo.phase, 0.0);
    }
    
    #[test]
    fn test_attack_shape_noise_wf_history() {

        let mut lfo = Lfo::new(44100.0);

        // Set LFO parameters
        lfo.params[LfoParam::Shape] = 4.0; // Noise
        lfo.params[LfoParam::Deform] = 0.5;
        lfo.params[LfoParam::Unipolar] = 0.0;

        lfo.attack_shape(LfoShape::Noise, 0.0);

        // Verify that the waveform history is updated correctly
        assert_approx_eq!(lfo.wf_history[0], lfo.wf_history[1], 1e-6);
        assert_approx_eq!(lfo.wf_history[1], lfo.wf_history[2], 1e-6);
        assert_approx_eq!(lfo.wf_history[2], lfo.wf_history[3], 1e-6);
    }

    #[test]
    fn test_attack_shape_noise_phase_reset() {

        let mut lfo = Lfo::new(44100.0);

        // Set LFO parameters
        lfo.params[LfoParam::Shape] = 4.0; // Noise
        lfo.params[LfoParam::Deform] = 0.5;
        lfo.params[LfoParam::Unipolar] = 0.0;

        lfo.phase = 0.75;

        lfo.attack_shape(LfoShape::Noise, 0.0);

        // Verify that the phase is reset to 0.0
        assert_approx_eq!(lfo.phase, 0.0, 1e-6);
    }

    #[test]
    fn test_attack_shape_noise_target_and_noised1_reset() {

        let mut lfo = Lfo::new(44100.0);

        // Set LFO parameters
        lfo.params[LfoParam::Shape] = 4.0; // Noise
        lfo.params[LfoParam::Deform] = 0.5;
        lfo.params[LfoParam::Unipolar] = 0.0;

        lfo.noise = 0.25;
        lfo.noised1 = 0.5;
        lfo.target = 0.75;

        lfo.attack_shape(LfoShape::Noise, 0.0);

        // Verify that the target, noised1, and noise variables are reset to 0.0
        assert_approx_eq!(lfo.target, 0.0, 1e-6);
        assert_approx_eq!(lfo.noised1, 0.0, 1e-6);
        assert_approx_eq!(lfo.noise, 0.0, 1e-6);
    }

    #[test]
    fn test_attack_shape_noise_output() {
        let mut lfo = Lfo::new(44100.0);

        // Set LFO parameters
        lfo.params[LfoParam::Shape] = 4.0; // Noise
        lfo.params[LfoParam::Deform] = 0.5;
        lfo.params[LfoParam::Unipolar] = 0.0;

        let output = lfo.process();

        // Verify that the output is in the range [-1, 1]
        assert!(output >= -1.0 && output <= 1.0);
    }
}
*/
