crate::ix!();

impl Lfo {

    #[inline] pub fn attack_shape(&mut self, shape: LfoShape, start_phase: f32) {
        match shape {
            LfoShape::Sine          => self.attack_shape_sine(),
            LfoShape::Tri           => self.attack_shape_tri(),
            LfoShape::Square        => self.attack_shape_square(),
            LfoShape::Ramp          => self.attack_shape_ramp(),
            LfoShape::Noise         => self.attack_shape_noise(),
            LfoShape::SampleAndHold => self.attack_shape_snh(),
            LfoShape::Envelope      => self.attack_shape_envelope(),
            LfoShape::StepSequencer => self.attack_shape_stepseq(start_phase)
        }
    }

    /// For the `attack_shape_square` function, we set the `env_val` to 1.0 if `env_phase` is less
    /// than 0.5, and -1.0 otherwise. This creates a square wave with a 50% duty cycle.
    ///
    #[inline] pub fn attack_shape_square(&mut self)   { 

        if self.phase < 0.5 {
            self.output = 1.0;
        } else {
            self.output = -1.0;
        }
    }

    /// For the `attack_shape_ramp` function, we set the `env_val` to a linear ramp from -1.0 to
    /// 1.0 as `env_phase` goes from 0.0 to 1.0. This creates a ramp wave.
    ///
    #[inline] pub fn attack_shape_ramp(&mut self)     { 
        self.output = ((self.phase * 2.0) - 1.0) as f64;
    }

    /// For the `attack_shape_envelope` function, we retrieve the envelope from the `params_env`
    /// field and use it to get the value of the envelope at the current `env_phase`. This creates
    /// an arbitrary waveform defined by the envelope.
    ///
    #[inline] pub fn attack_shape_envelope(&mut self) { 
        self.output = self.env_val as f64;
    }
}
