crate::ix!();

/// This code defines an implementation of the
/// `ModulationSourceControl` trait for the `Lfo`
/// struct. The `ModulationSourceControl` trait
/// provides an interface for objects that can be
/// used as modulation sources in a synthesizer.
/// 
/// The `Lfo` struct is an oscillator module that
/// produces a low-frequency waveform, such as
/// a sine wave, square wave, or triangle wave, at
/// a rate that is typically below the range of
/// audible frequencies. The
/// `ModulationSourceControl` trait defines
/// methods for setting and getting the output
/// value of the LFO, as well as controlling its
/// enable state and processing audio blocks.
///
impl GetModulationSourceType for Lfo {

    /// The `modulation_source_type` method returns the
    /// `ModSrcType::LFO` value, indicating that
    /// this modulation source is an LFO.
    ///
    fn modulation_source_type(&self) -> ModSrcType {
        ModSrcType::LFO
    }
}

impl SetModulationSourceOutput for Lfo {

    /// The `set_output` method sets the current
    /// output value of the LFO to the specified
    /// value `x`.
    ///
    fn set_output(&mut self, x: f64) {
        self.output = x;
    }
}

impl GetModulationSourceOutput for Lfo {

    /// The `get_output` method returns the
    /// current output value of the LFO.
    ///
    fn get_output(&self) -> f64 {
        //TODO
        0.0
    }

    /// The `get_output01` method returns the
    /// current output value of the LFO scaled to
    /// the range [0, 1].
    ///
    fn get_output01(&self) -> f64 {
        //TODO
        0.0
    }
}

impl CheckIsModulationSourcePerVoice for Lfo {

    /// The `per_voice` method returns a boolean
    /// value indicating whether the modulation
    /// source should be processed per voice or
    /// per block.
    ///
    fn per_voice(&self) -> bool { 
        //TODO
        false 
    }
}

impl CheckBipolar for Lfo {

    /// The `is_bipolar` method returns a boolean
    /// value indicating whether the output of the
    /// modulation source is bipolar, meaning it
    /// ranges from -1 to 1, or unipolar, meaning
    /// it ranges from 0 to 1.
    ///
    fn is_bipolar(&self) -> bool { 
        true
    }
}

impl SetBipolar for Lfo {

    /// The `set_bipolar` method sets the bipolar
    /// flag of the modulation source. Since the
    /// `Lfo` struct always produces a bipolar
    /// output, this method does nothing and
    /// asserts that the input value is true.
    ///
    fn set_bipolar(&mut self, b: bool) {
        assert!(b);
        /* no-op */
    }
}

impl ModulationSourceProcessBlock for Lfo {

    /// The `process_block` method processes
    /// a block of audio for the LFO.
    ///
    fn process_block(&mut self) {
        self.process();
    }
}

impl Enable for Lfo {

    /// The `enable` method sets the enabled state
    /// of the LFO.
    ///
    fn enable(&mut self, v: bool) {
        self.enabled = v;
    }
}

impl CheckEnabled for Lfo {

    /// The `enabled` method returns a boolean
    /// value indicating whether the LFO is
    /// currently enabled.
    ///
    fn enabled(&self) -> bool {
        self.enabled
    }
}
