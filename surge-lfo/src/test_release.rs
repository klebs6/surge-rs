crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_release() {

        // Create a new LFO with default parameters.
        let mut lfo = Lfo::new();

        // Assert that the initial state of the `env_state` field is `LfoEnvState::Attack`.
        assert_eq!(lfo.env_state, LfoEnvState::Attack);

        // Set the `Release` parameter to a low value.
        lfo.params[LfoParam::Release] = 0.1;

        // Trigger the release phase of the LFO.
        lfo.release();

        // Assert that the `env_state` field is now `LfoEnvState::Release`.
        assert_eq!(lfo.env_state, LfoEnvState::Release);
    }

    // Note the `reset` function is currently
    // unimplemented (`todo!();`), so it cannot be
    // tested or documented until it is
    // implemented.
    //
}
