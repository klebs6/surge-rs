crate::ix!();

impl crate::AbstractBlitter {

    /// Updates the output attenuation values
    /// based on the number of unison voices.
    ///
    /// This should be called whenever the number
    /// of unison voices changes.
    ///
    /// - It sets `self.out_attenuation_inv` to
    /// the number of unison voices as a float.
    ///
    /// - It sets `self.out_attenuation` to the
    /// reciprocal of `self.out_attenuation_inv`.
    /// 
    #[inline] pub fn update_out_attenuation(&mut self) {

        // Compute the inverse and normal attenuation factors.
        self.out_attenuation_inv = self.n_unison as f32;
        self.out_attenuation     = 1.0 / self.out_attenuation_inv;
    }

    /// Prepares the state of the blitter for a single unison voice.
    #[inline] pub fn prepare_single_voice(&mut self) {

        // Reset the detune bias and offset to default values.
        self.detune_bias    = 1.0;
        self.detune_offset  = 0.0;

        // Set the panning to be centered.
        self.pan_l[0]       = 1.0;
        self.pan_r[0]       = 1.0;
    }

    /// Prepares the state of the blitter for
    /// a multi-voice unison configuration.
    ///
    /// This function iterates over the voices and
    /// calculates the panning position for each
    /// voice. 
    ///
    /// It does this by calculating the distance
    /// of each voice from the midpoint and
    /// scaling it by `self.detune_bias`. 
    ///
    /// It also checks if the voice is in the
    /// second half of the list and flips the sign
    /// of the panning position if the number of
    /// voices is odd. 
    ///
    /// Finally, it sets the left and right
    /// panning positions for the voice based on
    /// the calculated panning position.
    ///
    /// It sets the panning position for the first
    /// voice to `1.0`.
    ///
    /// # Arguments
    ///
    /// * `voices` - The number of voices in the
    /// unison configuration.
    ///
    /// # Panics
    ///
    /// This function panics if `voices` is less
    /// than 2 or greater than the maximum number
    /// of unison voices.
    ///
    #[inline] pub fn prepare_multi_voice(&mut self, voices: usize) {

        // Set the detune bias and offset based on
        // the number of voices.
        //
        self.detune_bias   = 2.0 / (self.n_unison as f32 - 1.0);
        self.detune_offset = -1.0;

        // Determine if the number of voices is
        // odd and compute the middle voice index.
        //
        let odd:  bool   = (voices & 1) != 0;
        let mid:  f32    = (voices as f32) * 0.5 - 0.5;

        let half: usize  = voices >> 1;

        // Compute the panning values for each voice.
        for i in 0..voices {

            // Compute the detune factor for this voice.
            let mut d: f32 = (i as f32 - mid).abs() / mid;

            if odd && (i >= half) {
                d = -d;
            }

            if (i & 1) != 0 {
                d = -d;
            }

            // Compute the left and right panning
            // values for this voice.
            //
            self.pan_l[i] = 1.0 - d;
            self.pan_r[i] = 1.0 + d;
        }
    }

    /// Prepares the state of the blitter for
    /// a given number of unison voices.
    ///
    /// This function automatically selects
    /// between `prepare_single_voice` and
    /// `prepare_multi_voice` based on the number
    /// of voices.
    ///
    /// This function updates the output
    /// attenuation based on the number of unison
    /// voices by calling
    /// `self.update_out_attenuation()`.
    ///
    /// This function chooses between single and
    /// multi-voice mode based on the number of
    /// voices by calling
    ///
    /// `self.prepare_single_voice()` or
    /// `self.prepare_multi_voice(voices)`.
    ///
    /// # Arguments
    ///
    /// * `voices` - The number of voices in the
    /// unison configuration.
    ///
    /// # Panics
    ///
    /// This function panics if `voices` is less
    /// than 1 or greater than the maximum number
    /// of unison voices.
    ///
    pub fn prepare_unison(&mut self, voices: usize) {

        // Update the output attenuation factor.
        self.update_out_attenuation();

        // Choose the appropriate preparation
        // function based on the number of voices.
        //
        match voices {
            1 => self.prepare_single_voice(),
            _ => self.prepare_multi_voice(voices)
        }
    }
}
