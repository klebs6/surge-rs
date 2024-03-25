crate::ix!();

impl Lfo {

    /// Sets the LFO's `noise`, `noised1`, and
    /// `target` fields to 0.0, and computes a new
    /// `iout` value based on the current `target`
    /// and `noised1` values and the `Deform`
    /// parameter.
    ///
    /// This method is used by the `Lfo` object to
    /// generate new LFO values with an attack
    /// phase that is shaped by the `Deform`
    /// parameter. It uses a second-order
    /// correlated noise algorithm to generate the
    /// `iout` value, which is the output value of
    /// the LFO.
    ///
    /// Sample and hold is a technique used in
    /// audio synthesis to generate stepped or
    /// quantized output values from an LFO
    /// waveform. In this technique, the LFO
    /// output value is sampled at a regular
    /// interval, and held until the next sample
    /// is taken. This generates a stepped
    /// waveform that can be used to create
    /// rhythmic or percussive effects in a sound.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use my_crate::Lfo;
    ///
    /// let mut lfo = Lfo::new();
    ///
    /// lfo.attack_shape_snh();
    ///
    /// assert_eq!(lfo.noise, 0.0);
    /// assert_eq!(lfo.noised1, 0.0);
    /// assert_eq!(lfo.target, 0.0);
    /// ```
    ///
    #[inline] pub fn attack_shape_snh(&mut self) 
    {
        self.noise   = 0.0;
        self.noised1 = 0.0;
        self.target  = 0.0;

        self.iout    = correlated_noise_o2mk2(
            self.target, 
            self.noised1, 
            pvalf![self.params[LfoParam::Deform]]
        );
    }
}

