crate::ix!();

impl Lfo {

    /// selects the waveform shape to be processed
    /// and calls the corresponding method.
    ///
    #[inline] pub fn process_shape(&mut self,shape: LfoShape) {
        match shape {
            LfoShape::Sine          => self.process_shape_sine(),
            LfoShape::Tri           => self.process_shape_tri(),
            LfoShape::Square        => self.process_shape_square(),
            LfoShape::Ramp          => self.process_shape_ramp(),
            LfoShape::Noise         => self.process_shape_noise(),
            LfoShape::SampleAndHold => self.process_shape_snh(),
            LfoShape::Envelope      => self.process_shape_envelope(),
            LfoShape::StepSequencer => self.process_shape_stepseq(),
        }
    }

    /// generates a sine waveform with a lookup
    /// table.
    ///
    #[inline] pub fn process_shape_sine(&mut self) {
        self.iout = self.bend3(
            self.tables.lookup_waveshape_warp(
                3, 
                2.0 - 4.0 * self.phase
            )
        );
    }

    /// generates a triangle waveform using
    /// a conditional expression.
    ///
    #[inline] pub fn process_shape_tri(&mut self) {
        self.iout = self.bend3(
            -1.0 + 4.0 * 
            match self.phase > 0.5 {
                true  => 1.0 - self.phase,
                false => self.phase,
            }
        );
    }

    /// generates a square waveform using
    /// a conditional expression and a deformation
    /// parameter.
    ///
    #[inline] pub fn process_shape_square(&mut self) {

        let deform = pvalf![self.params[LfoParam::Deform]];

        self.iout = match self.phase > (0.5 + 0.5 * deform )
        { 
            true => -1.0, 
            false => 1.0 
        };
    }

    /// generates a ramp waveform with a lookup
    /// table.
    ///
    #[inline] pub fn process_shape_ramp(&mut self) {
        self.iout = self.bend3(1.0 - 2.0 * self.phase);
    }

    /// generates a noise waveform using cubic interpolation.
    ///
    #[inline] pub fn process_shape_noise(&mut self) {
        self.iout = cubic_interpolate(
            self.wf_history[3], 
            self.wf_history[2], 
            self.wf_history[1], 
            self.wf_history[0], 
            self.phase
        );
    }

    /// performs no operation.
    ///
    #[inline] pub fn process_shape_snh(&mut self) {
        /*no-op*/
    }

    /// generates an envelope waveform using
    /// a deformation parameter and an envelope
    /// value.
    ///
    #[inline] pub fn process_shape_envelope(&mut self) {

        let deform = pvalf![self.params[LfoParam::Deform]];

        self.iout = (1.0 - deform ) + deform * self.env_val;
    }
}
