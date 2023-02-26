crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    /// The first test case checks that the method
    /// works with default parameters and sets the
    /// `env_state`, `env_val`, and `env_phase`
    /// fields to their expected initial values. 
    ///
    #[test]
    fn test_attack_with_default_params() {
        let mut lfo = Lfo::default();
        lfo.attack();
        assert_eq!(lfo.env_state, LfoEnvState::Delay);
        assert_eq!(lfo.env_val, 0.0);
        assert_eq!(lfo.env_phase, 0.0);
    }

    /// The second test case sets custom
    /// parameters to test that the method
    /// properly updates the `env_state`,
    /// `env_val`, and `env_phase` fields based on
    /// those parameters.
    ///
    #[test]
    fn test_attack_with_custom_params() {
        let mut lfo = Lfo::default();
        lfo.set_param(LfoParam::Shape, 1.0); // square shape
        lfo.set_param(LfoParam::Trigmode, 1.0); // trigger mode
        lfo.set_param(LfoParam::Delay, 0.5);
        lfo.set_param(LfoParam::Attack, 0.1);
        lfo.set_param(LfoParam::Hold, 0.2);
        lfo.set_param(LfoParam::Rate, 4.0);
        lfo.attack();
        assert_eq!(lfo.env_state, LfoEnvState::Attack);
        assert_eq!(lfo.env_val, 0.0);
        assert_eq!(lfo.env_phase, 0.0);
    }

    /// One possible additional test for the
    /// `attack` function is to test if the
    /// `maybe_update_env_state_for_attack` and
    /// `maybe_update_phase_and_step_for_attack`
    /// methods are called with the correct
    /// parameters based on the `LfoParam`
    /// values. This can be done by adding the
    /// following test:
    ///
    /// This test sets the `Lfo` parameters to
    /// some values, calls the `attack` method,
    /// and then checks if the `env_state`,
    /// `phase`, and `step` fields of the `Lfo`
    /// instance are set to the expected values
    /// based on the `LfoParam` values. This helps
    /// ensure that the `attack` method is
    /// properly calling the
    /// `maybe_update_env_state_for_attack` and
    /// `maybe_update_phase_and_step_for_attack`
    /// methods with the correct parameters.
    #[test]
    fn test_attack_calls_update_methods_with_correct_params() {

        let mut lfo = Lfo::default();

        let params = [
            LfoParam::StartPhase, LfoParam::Shape, LfoParam::Trigmode,
            LfoParam::Delay, LfoParam::Attack, LfoParam::Hold, LfoParam::Rate,
        ];

        lfo.set_parameters(&params);
        lfo.attack();
        assert_eq!(lfo.env_state, LfoEnvState::Attack);
        assert_eq!(lfo.phase, lfo.params[LfoParam::StartPhase]);
        assert_eq!(lfo.step, (lfo.params[LfoParam::Rate] / SAMPLE_RATE) * (2.0 * PI));
    }

    // Based on the current implementation and the
    // existing test cases, I believe we have
    // covered most of the significant scenarios
    // for the `Lfo::attack` method. However,
    // there could be additional edge cases or
    // boundary conditions that we have not
    // considered yet.
    //
    // One area to consider is when the values of
    // `delay`, `attack`, and `hold` are very
    // close to zero or negative. We could test
    // how the method behaves in such scenarios
    // and whether it still functions
    // correctly. Additionally, we could test for
    // cases where the value of `rate` is very
    // high or very low to see if the method can
    // handle extreme values and still produce the
    // expected results.
}
