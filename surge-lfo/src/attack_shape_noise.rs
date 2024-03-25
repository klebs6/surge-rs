crate::ix!();

impl Lfo {

    /// Sets the LFO waveform to a noise signal. The function generates correlated noise using
    /// the `correlated_noise_o2mk2` function, which takes the current LFO target value and the
    /// `deform` parameter as input. The noise signal is then multiplied by the current LFO phase
    /// and stored in the `wf_history` buffer.
    ///
    /// # Arguments
    ///
    /// * `self` - a mutable reference to the current LFO object
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rust_audio::Lfo;
    ///
    /// let mut lfo = Lfo::new();
    ///
    /// lfo.set_param(LfoParam::Unipolar, true);
    ///
    /// lfo.attack_shape(LfoShape::Noise, 0.0);
    /// ```
    ///
    /// # Notes
    ///
    /// The `correlated_noise_o2mk2` function generates noise with a correlation coefficient of
    /// `sqrt(deform)`. The `wf_history` buffer is used to store previous values of the waveform
    /// for interpolation, and is updated by the `step` function. The `phase` parameter determines
    /// the current phase of the LFO waveform.
    ///
    #[inline] pub fn attack_shape_noise(&mut self) 
    {
        self.noise   = 0.0;
        self.noised1 = 0.0;
        self.target  = 0.0;

        self.wf_history[3] = correlated_noise_o2mk2(
            self.target, 
            self.noised1, 
            pvalf![self.params[LfoParam::Deform]]
        ) * self.phase;

        self.wf_history[2] = correlated_noise_o2mk2(
            self.target, 
            self.noised1, 
            pvalf![self.params[LfoParam::Deform]]
        ) * self.phase;

        self.wf_history[1] = correlated_noise_o2mk2(
            self.target, 
            self.noised1, 
            pvalf![self.params[LfoParam::Deform]]
        ) * self.phase;

        self.wf_history[0] = correlated_noise_o2mk2(
            self.target, 
            self.noised1, 
            pvalf![self.params[LfoParam::Deform]]
        ) * self.phase;

        self.phase = 0.0;
    }
}
