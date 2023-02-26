crate::ix!();

impl Lfo {

    /// Changes the LFO shape to a sine wave and adjusts the phase according to the
    /// value of the `unipolar` parameter. If `unipolar` is true, the phase is increased
    /// by 0.75 on each call to the function. If `unipolar` is false, the phase is increased
    /// by 0.5 on each call to the function.
    ///
    /// # Arguments
    ///
    /// * `self` - a mutable reference to the `Lfo` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use rustsynth::lfo::{Lfo, LfoShape};
    ///
    /// let mut lfo = Lfo::default();
    /// lfo.attack_shape(LfoShape::Sine, 0.0);
    ///
    /// assert_eq!(lfo.phase, 0.0);
    /// lfo.attack_shape(LfoShape::Sine, 0.0);
    /// assert_eq!(lfo.phase, 0.5);
    /// lfo.params[LfoParam::Unipolar] = 1.0.into();
    /// lfo.attack_shape(LfoShape::Sine, 0.0);
    /// assert_eq!(lfo.phase, 0.25);
    /// lfo.attack_shape(LfoShape::Sine, 0.0);
    /// assert_eq!(lfo.phase, 0.0);
    /// ```
    ///
    #[inline] pub fn attack_shape_sine(&mut self) {

        if pvalb![self.params[LfoParam::Unipolar]] { 
            self.phase += 0.75; 
        }

        if self.phase > 1.0 { 
            self.phase -= 1.0; 
        }
    }
}
