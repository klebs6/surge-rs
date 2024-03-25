crate::ix!();

impl crate::AbstractBlitter {

    /// Clears all oscillator output values to 0.0.
    ///
    /// This method sets the values of the `osc_out_l`, `osc_out_2l`, `osc_out_r`, and `osc_out_2r`
    /// fields of the `AbstractBlitter` struct to 0.0. This method is typically called at the start
    /// of a new audio buffer processing cycle to clear the output from the previous cycle.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use crate::AbstractBlitter;
    ///
    /// let mut blitter = AbstractBlitter::new(&sample_rate);
    ///
    /// // Process audio buffer
    ///
    /// blitter.clear_all_oscout();
    ///
    /// // Process next audio buffer
    /// ```
    ///
    pub fn clear_all_oscout(&mut self) {
        unsafe {
            self.osc_out_l  = _mm_set1_ps(0.0);
            self.osc_out_2l = _mm_set1_ps(0.0);
            self.osc_out_r  = _mm_set1_ps(0.0);
            self.osc_out_2r = _mm_set1_ps(0.0);
        }
    }

    /// Clears all oscillator and DC offset buffers to 0.0.
    ///
    /// This method sets the values of the `oscbuffer_l`, `oscbuffer_r`, and `dcbuffer` fields of the
    /// `AbstractBlitter` struct to 0.0. This method is typically called at the start of a new audio
    /// buffer processing cycle to clear the buffers from the previous cycle.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use crate::AbstractBlitter;
    ///
    /// let mut blitter = AbstractBlitter::new(&sample_rate);
    ///
    /// // Process audio buffer
    ///
    /// blitter.clear_buffers();
    ///
    /// // Process next audio buffer
    /// ```
    ///
    pub fn clear_buffers(&mut self) {

        macro_rules! clear {
            ($buf:ident) => {
                for x in self.$buf.iter_mut() {
                    *x = 0.0;
                }
            }
        }

        clear!{oscbuffer_l}
        clear!{oscbuffer_r}
        clear!{dcbuffer}
    }
}
