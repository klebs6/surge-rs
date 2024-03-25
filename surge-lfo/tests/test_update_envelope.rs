/*
crate::ix!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_envrate_with_lfo_param() {
        let mut lfo = Lfo::default();
        lfo.params[LfoParam::Delay] = 0.5;
        let temposyncratio = 1.0;
        let envrate = lfo.calculate_envrate_with_lfo_param(LfoParam::Delay, temposyncratio);
        assert_eq!(envrate, 0.025118864);
    }

    #[test]
    fn test_next_env_state() {
        let mut lfo = Lfo::default();
        lfo.env_state = LfoEnvState::Delay;
        lfo.next_env_state(0.5);
        assert_eq!(lfo.env_state, LfoEnvState::Attack);
        assert_eq!(lfo.env_phase, 0.0);
    }

    #[test]
    fn test_calculate_envrate() {
        let mut lfo = Lfo::default();
        lfo.env_state = LfoEnvState::Delay;
        let temposyncratio = 1.0;
        let envrate = lfo.calculate_envrate(temposyncratio);
        assert_eq!(envrate, 0.0);
    }

    #[test]
    fn test_calculate_envrate_with_lfo_param() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Attack] = pvali![LfoParam::Attack, 5];
        assert_eq!(
            lfo.calculate_envrate_with_lfo_param(LfoParam::Attack, 1.0),
            lfo.tables.envelope_rate_linear(5)
        );
        lfo.params[LfoParam::Attack] = pvali![LfoParam::Attack, 5, true];
        assert_eq!(
            lfo.calculate_envrate_with_lfo_param(LfoParam::Attack, 2.0),
            lfo.tables.envelope_rate_linear(5) * 2.0
        );
    }

    #[test]
    fn test_next_env_state() {
        let mut lfo = Lfo::new();
        lfo.env_state = LfoEnvState::Delay;
        lfo.next_env_state(0.5);
        assert_eq!(lfo.env_state, LfoEnvState::Attack);
        assert_eq!(lfo.env_phase, 0.0);
        lfo.next_env_state(0.5);
        assert_eq!(lfo.env_state, LfoEnvState::Hold);
        assert_eq!(lfo.env_phase, 0.0);
        lfo.next_env_state(0.5);
        assert_eq!(lfo.env_state, LfoEnvState::Decay);
        assert_eq!(lfo.env_phase, 0.0);
        lfo.next_env_state(0.5);
        assert_eq!(lfo.env_state, LfoEnvState::Stuck);
        assert_eq!(lfo.env_phase, 0.0);
        assert_eq!(lfo.env_val, 0.5);
        lfo.next_env_state(0.5);
        assert_eq!(lfo.env_state, LfoEnvState::Stuck);
        assert_eq!(lfo.env_phase, 0.0);
        assert_eq!(lfo.env_val, 0.0);
    }

    #[test]
    fn test_calculate_envrate() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Attack] = pvali![LfoParam::Attack, 5];
        lfo.env_state = LfoEnvState::Attack;
        assert_eq!(
            lfo.calculate_envrate(1.0),
            lfo.tables.envelope_rate_linear(5)
        );
    }

    #[test]
    fn test_calculate_envrate_with_lfo_param() {
        let mut lfo = Lfo::default();
        lfo.params[LfoParam::Delay] = 0.5;
        lfo.params[LfoParam::Attack] = 0.1;
        lfo.params[LfoParam::Hold] = 0.2;
        lfo.params[LfoParam::Decay] = 0.3;
        lfo.params[LfoParam::Release] = 0.4;
        lfo.params[LfoParam::Temposync] = true;
        let envrate = lfo.calculate_envrate_with_lfo_param(LfoParam::Attack, 2.0);
        assert_eq!(envrate, 0.4);
    }

    #[test]
    fn test_next_env_state() {
        let mut lfo = Lfo::default();
        assert_eq!(lfo.env_state, LfoEnvState::Delay);
        assert_eq!(lfo.env_phase, 0.0);
        lfo.next_env_state(0.5);
        assert_eq!(lfo.env_state, LfoEnvState::Attack);
        assert_eq!(lfo.env_phase, 0.0);
        lfo.next_env_state(0.5);
        assert_eq!(lfo.env_state, LfoEnvState::Hold);
        assert_eq!(lfo.env_phase, 0.0);
        lfo.next_env_state(0.5);
        assert_eq!(lfo.env_state, LfoEnvState::Decay);
        assert_eq!(lfo.env_phase, 0.0);
        assert_eq!(lfo.env_val, 0.5);
        lfo.next_env_state(0.5);
        assert_eq!(lfo.env_state, LfoEnvState::Stuck);
        assert_eq!(lfo.env_phase, 0.0);
        assert_eq!(lfo.env_val, 0.5);
        lfo.next_env_state(0.5);
        assert_eq!(lfo.env_state, LfoEnvState::Stuck);
        assert_eq!(lfo.env_phase, 0.0);
        assert_eq!(lfo.env_val, 0.0);
    }

    #[test]
    fn test_calculate_envrate() {
        let mut lfo = Lfo::default();
        assert_eq!(lfo.calculate_envrate(1.0), 0.0);
        lfo.env_state = LfoEnvState::Attack;
        lfo.params[LfoParam::Attack] = 0.5;
        assert_eq!(lfo.calculate_envrate(1.0), 0.5);
        lfo.env_state = LfoEnvState::Release;
        lfo.params[LfoParam::Release] = 0.1;
        assert_eq!(lfo.calculate_envrate(0.5), 0.05);
    }

    #[test]
    fn test_update_envelope_value_release_state() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Delay] = 0.0;
        lfo.params[LfoParam::Attack] = 0.0;
        lfo.params[LfoParam::Hold] = 0.0;
        lfo.params[LfoParam::Decay] = 0.0;
        lfo.params[LfoParam::Sustain] = 0.0;
        lfo.params[LfoParam::Release] = 0.5;

        lfo.env_state = LfoEnvState::Release;
        lfo.env_phase = 0.5;
        lfo.env_val = 0.0;

        lfo.update_envelope_value(0.0);

        assert_eq!(lfo.env_val, 0.75);
    }

    #[test]
    fn test_update_envelope_value_decay_state() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Delay] = 0.0;
        lfo.params[LfoParam::Attack] = 0.0;
        lfo.params[LfoParam::Hold] = 0.0;
        lfo.params[LfoParam::Decay] = 1.0;
        lfo.params[LfoParam::Sustain] = 0.5;
        lfo.params[LfoParam::Release] = 0.0;

        lfo.env_state = LfoEnvState::Decay;
        lfo.env_phase = 0.5;
        lfo.env_val = 0.0;

        lfo.update_envelope_value(0.5);

        assert_eq!(lfo.env_val, 0.75);
    }

    #[test]
    fn test_update_envelope_value_hold_state() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Delay] = 0.0;
        lfo.params[LfoParam::Attack] = 0.0;
        lfo.params[LfoParam::Hold] = 1.0;
        lfo.params[LfoParam::Decay] = 0.0;
        lfo.params[LfoParam::Sustain] = 0.5;
        lfo.params[LfoParam::Release] = 0.0;

        lfo.env_state = LfoEnvState::Hold;
        lfo.env_phase = 0.5;
        lfo.env_val = 0.0;

        lfo.update_envelope_value(0.5);

        assert_eq!(lfo.env_val, 1.0);
    }

    /// This test checks that when the envelope
    /// state is set to Attack and the envelope
    /// phase is set to 0.25, the
    /// `update_envelope_value` method correctly
    /// calculates the envelope value. 
    ///
    /// The expected value in this case is 0.25,
    /// since the envelope phase is equal to the
    /// attack time divided by 2 (0.5 / 2 = 0.25)
    /// and the attack phase of the envelope goes
    /// from
    /// 0 to 1 during this time.
    ///
    #[test]
    fn test_update_envelope_value_attack_state() {

        let mut lfo = Lfo::new();

        // set attack time to 0.5s
        lfo.params[LfoParam::Attack].value = 0.5;  

        // set sustain level to 0.5
        lfo.params[LfoParam::Sustain].value = 0.5; 

        // set envelope state to Attack
        lfo.env_state = LfoEnvState::Attack;       

        // set envelope phase to 0.25
        lfo.env_phase = 0.25;                      

        // update envelope value with sustain level of 0.5
        lfo.update_envelope_value(0.5);            

        // expected envelope value is 0.25
        assert_approx_eq!(lfo.env_val, 0.25);      
    }

    /// This tests the `update_envelope_value`
    /// method for different envelope states and
    /// values of `env_phase` and `sustainlevel`,
    /// ensuring that the computed envelope value
    /// `env_val` matches the expected value.
    ///
    #[test]
    fn test_update_envelope_value() {
        let mut lfo = Lfo::default();
        lfo.env_state = LfoEnvState::Attack;
        lfo.env_phase = 0.5;
        lfo.update_envelope_value(0.5);
        assert_eq!(lfo.env_val, 0.5);
        lfo.env_state = LfoEnvState::Hold;
        lfo.update_envelope_value(0.5);
        assert_eq!(lfo.env_val, 1.0);
        lfo.env_state = LfoEnvState::Decay;
        lfo.env_phase = 0.5;
        lfo.update_envelope_value(0.5);
        assert_eq!(lfo.env_val, 0.75);
        lfo.env_phase = 0.0;
        lfo.update_envelope_value(0.5);
        assert_eq!(lfo.env_val, 0.0);
        lfo.env_state = LfoEnvState::Release;
        lfo.env_phase = 0.5;
        lfo.env_releasestart = 0.5;
        lfo.update_envelope_value(0.5);
        assert_eq!(lfo.env_val, 0.75);
    }
}
*/
