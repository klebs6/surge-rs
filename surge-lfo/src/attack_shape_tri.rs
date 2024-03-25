crate::ix!();

/// This function implements a triangular waveform generator for an LFO (low frequency
/// oscillator). Here's how it works mathematically:
/// 
/// - The output waveform is a triangle wave.
///
/// - The frequency of the triangle wave is determined by the LFO's frequency parameter.
///
/// - The amplitude of the triangle wave is determined by the LFO's amplitude parameter.
///
/// - The waveform generator has a phase accumulator that is updated on each call to the
/// function. The phase accumulator is initialized to 0.0 when the LFO starts up.
///
/// - The phase accumulator is incremented by a step value on each call to the function. The step
/// value is determined by the LFO's frequency parameter.
///
/// - If the phase accumulator is greater than or equal to 1.0, it is reset to 0.0.
///
/// - If the LFO is set to bipolar mode (i.e., the unipolar parameter is false), then the waveform
/// generator "folds over" the triangle wave at the midpoint (i.e., 0.5 amplitude). This is
/// accomplished by adding 0.25 to the phase accumulator when it exceeds 0.5, effectively
/// reflecting the wave around the midpoint.
/// 
/// The actual code for this function is fairly short and simple. 
///
/// It checks whether the LFO is in unipolar mode, and if not, increments the phase accumulator by
/// 0.25 when it exceeds 0.5 (which is the midpoint of the waveform). If the phase accumulator
/// exceeds 1.0, it is reset to 0.0, so that the waveform repeats.
/// 
/// The behavior of this function can be described mathematically using the following
/// equations:
/// 
/// - If `self.params[LfoParam::Unipolar]` is true, there is no change in behavior and the phase
/// remains the same.
///
/// - If `self.params[LfoParam::Unipolar]` is false, then the phase is incremented by 0.25. If the
/// resulting phase exceeds 1.0, it is wrapped around to the range [0,1) by subtracting 1.0.
/// 
/// So mathematically, we can represent the behavior of this function as:
/// 
/// ```ignore
/// if !pvalb![self.params[LfoParam::Unipolar]] {
///     self.phase = (self.phase + 0.25) % 1.0;
/// }
/// ```
/// 
/// where `%` denotes the modulo operator that wraps the phase around to the range [0,1) if it
/// exceeds 1.0.
/// 
impl Lfo {

    /// Increments the phase of the LFO by 0.25 if the `Unipolar` parameter is set to false.
    /// If the resulting phase exceeds 1.0, it is wrapped around to the range [0,1).
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to the `Lfo` instance.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use your_crate::Lfo;
    /// let mut lfo = Lfo::new();
    /// lfo.set_param(LfoParam::Unipolar, true);
    /// lfo.attack_shape_tri();
    /// ```
    #[inline] pub fn attack_shape_tri(&mut self) {

        // If the `Unipolar` parameter is set to false, increment the phase by 0.25
        if !pvalb![self.params[LfoParam::Unipolar]] {

            self.phase += 0.25;

            // If the phase exceeds 1.0, wrap it around to the range [0,1)
            if self.phase > 1.0 {
                self.phase -= 1.0;
            }
        }
    }
}
