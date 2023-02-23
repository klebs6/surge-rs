/*!
This code defines an implementation of an ADSR
envelope generator, which is used in digital audio
synthesis to control the amplitude of a sound over
time. The ADSR acronym stands for Attack, Decay,
Sustain, and Release, which are the four stages of
the envelope. The code implements the decay stage
of the envelope.

The `impl AdsrEnvelope` block defines a method
called `process_block_digital_decay` that updates
the internal state of the ADSR envelope generator
during the decay stage. The method computes the
lower and upper bounds of the decay stage, based
on the user-set `DecayShape` parameter, and then
limits the phase of the envelope to be within
those bounds.

The `decay_shape0_bounds`, `decay_shape1_bounds`,
and `decay_shape2_bounds` functions compute the
lower and upper bounds of the decay stage for
different shapes of the decay curve, based on the
current phase and the decay rate.

The `process_block_digital_decay` method computes
the decay rate from the user-set `Decay`
parameter, and scales it by a time-sync
ratio. Then, it computes the sustain level from
the user-set `Sustain` parameter, and the decay
shape from the `DecayShape` parameter. It calls
the appropriate decay shape function to compute
the decay bounds, and limits the current phase to
be within those bounds. Finally, it sets the
output of the envelope to be the current phase.

Overall, this code provides the functionality for
computing the bounds of the decay stage of an ADSR
envelope generator, based on different decay
shapes, and for updating the internal state of the
generator during the decay stage.
*/

crate::ix!();

impl AdsrEnvelope {

    /// This function computes the lower and upper
    /// bounds for the decay stage of the ADSR
    /// envelope when using a linear decay
    /// shape. The function takes the decay rate
    /// `rate` as an argument, and returns a tuple
    /// of two values, which are the lower and
    /// upper bounds of the decay stage.
    /// 
    /// The function computes the lower bound as
    /// `self.phase - rate`, and the upper bound
    /// as `self.phase + rate`. The phase value
    /// represents the current level of the
    /// envelope, and the rate represents the rate
    /// at which the envelope decays during the
    /// decay stage.
    ///
    fn decay_shape0_bounds(&mut self, rate: f32) -> (f32, f32) {
        (self.phase - rate, self.phase + rate)
    }

    /// This function computes the lower and upper
    /// bounds for the decay stage of the ADSR
    /// envelope when using a quadratic decay
    /// shape. The function takes the decay rate
    /// `rate` as an argument, and returns a tuple of
    /// two values, which are the lower and upper
    /// bounds of the decay stage.
    ///     
    /// The function computes the lower bound as
    /// `self.phase - 2.0 * sx * rate + rate * rate`,
    ///
    /// and the upper bound as 
    /// `self.phase + 2.0 * sx * rate + rate * rate`, 
    ///
    /// where `sx` is the square root of the phase
    /// value. 
    ///
    /// These formulas represent a quadratic curve
    /// that starts at the current phase value and
    /// reaches zero at the sustain level.
    /// 
    /// There is also a special case handled in this
    /// function, where if the sustain level is very
    /// low (less than `1e-3`) and the phase is close
    /// to zero (less than `1e-4`), the lower bound is
    /// set to zero, to avoid the envelope going above
    /// the sustain level during the decay stage.
    ///
    fn decay_shape1_bounds(&mut self, rate: f32) -> (f32, f32) {

        let sx: f32 = self.phase.sqrt();

        let mut l_lo = self.phase - 2.0 * sx * rate + rate * rate;
        let     l_hi = self.phase + 2.0 * sx * rate + rate * rate;

        // That + rate * rate in both means at low
        // sustain ( < 1e-3 or so) you end up with
        // lo and hi both pushing us up off
        // sustain. 
        //
        // Unfortunatley we ned to handle that
        // case specially by pushing lo down
        if pvalf![self.params[AdsrParam::Sustain]] < 1e-3 && self.phase < 1e-4 {
            l_lo = 0.0;
        } 

        (l_lo, l_hi)
    }

    /// This function computes the lower and upper
    /// bounds for the decay stage of the ADSR
    /// envelope when using a cubic decay shape. 
    ///
    /// The function takes the decay rate `rate`
    /// as an argument, and returns a tuple of two
    /// values, which are the lower and upper
    /// bounds of the decay stage.
    /// 
    /// The function computes the lower bound as
    ///
    /// `self.phase - 3.0 * sx_sx_rate + 3.0 * sx_rate_rate - rate_cubed`,
    ///
    /// and the upper bound as 
    ///
    /// `self.phase + 3.0 * sx_sx_rate + 3.0 * sx_rate_rate + rate_cubed`, 
    ///
    /// where `sx` is the cube root of the phase
    /// value. 
    ///
    /// These formulas represent a cubic curve
    /// that starts at the current phase value and
    /// reaches zero at the sustain level.
    /// 
    fn decay_shape2_bounds(&mut self, rate: f32) -> (f32, f32) {

        let sx: f32 = self.phase.powf(0.3333333);

        let three_sx_sx_rate   = 3.0 * sx * sx * rate;
        let three_sx_rate_rate = 3.0 * sx * rate * rate;
        let rate_cubed         = rate * rate * rate;

        let l_lo = self.phase - three_sx_sx_rate + three_sx_rate_rate - rate_cubed;
        let l_hi = self.phase + three_sx_sx_rate + three_sx_rate_rate + rate_cubed;

        (l_lo, l_hi)
    }

    /// This method updates the internal state of
    /// the ADSR envelope during the decay
    /// stage. The method takes no arguments and
    /// has no return value.
    /// 
    /// The method first computes the decay rate
    /// from the user-set `Decay` parameter, using
    /// a lookup table and a time-sync ratio. It
    /// then computes the sustain level from the
    /// user-set `Sustain` parameter, and the
    /// decay shape from the `DecayShape`
    /// parameter.
    /// 
    /// The method then calls the appropriate
    /// decay shape function to compute the lower
    /// and upper bounds of the decay stage, based
    /// on the current phase and the decay
    /// rate. It limits the current phase value to
    /// be within those bounds using the
    /// `limit_range` function (not shown in the
    /// code), which ensures that the phase value
    /// stays within the range of 0.0 to 1.0.
    /// 
    /// Finally, the method sets the output of the
    /// envelope to be the current phase value,
    /// which represents the level of the envelope
    /// at the end
    /// 
    pub fn process_block_digital_decay(&mut self) {

        let decay = pvalf![self.params[AdsrParam::Decay]];

        let rate: f32 = self.tables.envelope_rate_linear(decay) 
            * tsyncratio![self,Decay];

        let sustain     = pvalf![self.params[AdsrParam::Sustain]];
        let decay_shape = pvali![self.params[AdsrParam::DecayShape]];

        let (l_lo, l_hi): (f32, f32) = match decay_shape
        {
            1 => self.decay_shape1_bounds(rate),
            2 => self.decay_shape2_bounds(rate),
            _ => self.decay_shape0_bounds(rate)
        };

        self.phase = limit_range( sustain , l_lo, l_hi );
        self.output = self.phase;
    }
}
