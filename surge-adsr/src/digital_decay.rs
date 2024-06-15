/**
  This code defines an implementation of an ADSR envelope generator, which is
  used in digital audio synthesis to control the amplitude of a sound over
  time. The
   
  ADSR acronym stands for Attack, Decay, Sustain, and Release, which are the
  four stages of the envelope. The code implements the digital decay stage of
  the envelope.
  */

crate::ix!();

impl DigitalDecay for AdsrEnvelope {

    /// The `process_block_digital_decay` method computes the decay rate from
    /// the user-set `Decay` parameter, and scales it by a time-sync
    /// ratio. Then, it computes the sustain level from the user-set `Sustain`
    /// parameter, and the decay shape from the `DecayShape` parameter. 
    ///
    /// It calls the appropriate decay shape function to compute the decay
    /// bounds, and limits the current phase to be within those bounds. Finally,
    /// it sets the output of the envelope to be the current phase.
    ///
    fn digital_decay(&mut self) {
        let decay     = self.get_decay_paramter();
        let sustain   = self.get_sustain_parameter();
        let rate: f32 = self.tables.envelope_rate_linear(decay) * tsyncratio![self,Decay];
        let bounds    = self.get_decay_shape_bounds(rate);
        self.phase    = limit_range(sustain, bounds);
        self.output   = self.phase;
    }
}
