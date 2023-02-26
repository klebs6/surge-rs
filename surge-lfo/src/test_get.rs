#[cfg(test)]
mod tests {

    use super::*;

    /// 1. A basic test case where the `temposyncratio` parameter is 1.0, indicating that there is
    /// no tempo synchronization. In this case, the expected rate value is the same as the rate
    /// parameter in the `Lfo` struct.
    ///
    /// This function can be tested by creating an instance of the `Lfo` struct and setting the
    /// `params` field to different values for each test case. 
    ///
    /// Then, the expected `LfoShape` enum value can be compared with the actual value returned by the
    /// `get_shape()` method.
    ///
    #[test]
    fn test_get_shape() {
        let mut lfo = Lfo::new();

        // Test case 1: Sine waveform
        lfo.params[LfoParam::Shape] = 0;
        assert_eq!(lfo.get_shape(), LfoShape::Sine);

        // Test case 2: Square waveform
        lfo.params[LfoParam::Shape] = 1;
        assert_eq!(lfo.get_shape(), LfoShape::Square);

        // Test case 3: Triangle waveform
        lfo.params[LfoParam::Shape] = 2;
        assert_eq!(lfo.get_shape(), LfoShape::Triangle);

        // Test case 4: Sawtooth waveform
        lfo.params[LfoParam::Shape] = 3;
        assert_eq!(lfo.get_shape(), LfoShape::Sawtooth);

        // Test case 5: Random waveform
        lfo.params[LfoParam::Shape] = 4;
        assert_eq!(lfo.get_shape(), LfoShape::Random);
    }

    /// 2. A test case where the `temposyncratio` parameter is 0.5, indicating that the tempo is
    /// half of the default tempo. In this case, the expected rate value is half of the rate
    /// parameter in the `Lfo` struct, since the tempo synchronization feature is enabled.
    ///
    /// Similar to `get_shape()`, this function can be tested by creating an instance of the `Lfo`
    /// struct and setting the `params` field to different values for each test case. 
    ///
    /// Then, the expected `LfoMode` enum value can be compared with the actual value returned by the
    /// `get_mode()` method.
    ///
    #[test]
    fn test_get_mode() {
        let mut lfo = Lfo::new();

        // Test case 1: Mono mode
        lfo.params[LfoParam::Trigmode] = 0;
        assert_eq!(lfo.get_mode(), LfoMode::Mono);

        // Test case 2: Poly mode
        lfo.params[LfoParam::Trigmode] = 1;
        assert_eq!(lfo.get_mode(), LfoMode::Poly);

        // Test case 3: Sync mode
        lfo.params[LfoParam::Trigmode] = 2;
        assert_eq!(lfo.get_mode(), LfoMode::Sync);
    }

    /// 3. A test case where the `temposyncratio` parameter is 1/32 of the default tempo,
    /// indicating that the tempo is 32 times slower than the default tempo. In this case, the
    /// expected rate value is 32 times higher than the rate parameter in the `Lfo` struct, since
    /// the tempo synchronization feature is at maximum level.
    ///
    #[test]
    fn test_get_rate() {
        let mut lfo = Lfo::new();

        // Test case 1: LFO rate without tempo synchronization
        lfo.params[LfoParam::Rate] = 50;
        assert_eq!(lfo.get_rate(1.0), 50.0);

        // Test case 2: LFO rate with tempo synchronization
        lfo.params[LfoParam::Rate] = 50;
        lfo.params[LfoParam::Rate].temposync = true;
        assert_eq!(lfo.get_rate(0.5), 25.0);

        // Test case 3: LFO rate at maximum tempo synchronization
        lfo.params[LfoParam::Rate] = 50;
        lfo.params[LfoParam::Rate].temposync = true;
        assert_eq!(lfo.get_rate(1.0 / 32.0), 1600.0);
    }
}
