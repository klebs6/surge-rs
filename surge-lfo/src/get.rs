crate::ix!();

impl Lfo {

    /// This function is a public method of the `Lfo` struct that returns the shape of the
    /// low-frequency oscillator (LFO) waveform. 
    ///
    /// The `LfoShape` is an enum type that represents different LFO waveforms, such as sine,
    /// square, triangle, sawtooth, and random. 
    ///
    /// The function retrieves the shape value from the `params` field of the `Lfo` struct,
    /// converts it to an unsigned integer, and returns the corresponding `LfoShape` enum value.
    ///
    #[inline] pub fn get_shape(&self) -> LfoShape {
        LfoShape::try_from(
            pvali![self.params[LfoParam::Shape]]
            as usize).unwrap()
    }

    /// This function is also a public method of the `Lfo` struct that returns the trigger mode of
    /// the LFO. 
    ///
    /// The `LfoMode` is an enum type that represents the trigger modes of the LFO, such as mono,
    /// poly, and sync. 
    ///
    /// The function retrieves the trigger mode value from the `params` field of the `Lfo` struct,
    /// converts it to an unsigned integer, and returns the corresponding `LfoMode` enum value.
    ///
    #[inline] pub fn get_mode(&self) -> LfoMode {
        LfoMode::try_from(
            pvali![self.params[LfoParam::Trigmode]]
            as usize).unwrap()
    }

    /// This function is a public method of the `Lfo` struct that returns the LFO rate, which
    /// determines the speed at which the LFO waveform oscillates. 
    ///
    /// The `temposyncratio` parameter is a ratio of the LFO rate to the tempo of the music. 
    ///
    /// If the `temposync` flag is enabled in the `params` field of the `Lfo` struct, the LFO rate
    /// is synchronized to the tempo of the music by multiplying the rate by the `temposyncratio`. 
    ///
    /// The function retrieves the rate value from the `params` field of the `Lfo` struct, passes
    /// it through a lookup table to convert it to a linear envelope rate, and returns the
    /// resulting rate value.
    ///
    #[inline] pub fn get_rate(&self, temposyncratio: f32) -> f32 {

        let mut rate = self.tables.envelope_rate_linear(
            pvalf![self.params[LfoParam::Rate]]
        );

        if self.params[LfoParam::Rate].temposync  
        {
            rate *=  temposyncratio;
        }

        rate
    }
}
