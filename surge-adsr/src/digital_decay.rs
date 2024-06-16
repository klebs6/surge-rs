/*!
    This code defines an implementation
    of an ADSR envelope generator, which
    is used in digital audio synthesis to
    control the amplitude of a sound over
    time. The
    
    ADSR acronym stands for Attack, Decay,
    Sustain, and Release, which are the
    four stages of the envelope. The code
    implements the digital decay stage
    of the envelope.
  */

crate::ix!();

pub trait DigitalDecay {

    fn digital_decay(&mut self);
}

impl DigitalDecay for AdsrEnvelope {

    /// Computes the sustain level from the user-set `Sustain` parameter
    ///
    /// Computes the decay shape from the `DecayShape` parameter. 
    ///
    /// Calls the appropriate decay shape function to compute the decay bounds
    ///
    /// Limits the current phase to be within those bounds. 
    ///
    /// Sets the output of the envelope to be the current phase.
    ///
    fn digital_decay(&mut self) {
        let sustain = self.get_sustain_parameter();
        let rate    = self.decay_rate();
        let bounds  = self.get_decay_shape_bounds(rate);

        self.set_phase(limit_range(sustain, bounds.0, bounds.1));
        self.set_output(self.phase());
    }
}
