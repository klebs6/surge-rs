crate::ix!();

impl CoefficientLoadStore for crate::HalfRateFilterSSE {

    /// Load the coefficients of the filter into
    /// the HalfRateFilterSSE structure.
    ///
    /// The values for the filter's coefficients
    /// are set into the 'va' field of the
    /// structure.
    /// 
    /// When the 'steep' boolean flag is set to
    /// false, the filter will have softer slopes,
    /// more attenuation, and less stopband
    /// ripple. This flag affects the filter's
    /// behavior.
    ///
    fn load_coefficients(&mut self) {

        // Zero out all values in the 'va' field
        // of the structure
        //
        for i in 0..self.m {
            self.va[i] = unsafe{ _mm_setzero_ps() };
        }

        // Compute the order of the filter
        //
        let order: usize = self.m << 1;

        // Choose the appropriate coefficient
        // loading method based on the 'steep'
        // flag
        //
        match self.steep {
            true  => self.load_steep_coefficients(order),
            false => self.load_softer_coefficients(order),
        }
    }

    /// saves the filter's coefficients back to
    /// their original memory locations, which are
    /// provided as arguments to the function. 
    ///
    /// The values are read from the 'va' field of
    /// the HalfRateFilterSSE structure and are
    /// stored into the provided 'coefficient_a'
    /// and 'coefficient_b' memory locations.
    ///
    /// Note: Unsafe function that requires the
    /// caller to ensure that the memory locations
    /// pointed to by 'coefficient_a' and
    /// 'coefficient_b' are valid and have enough
    /// space to store all coefficients of the
    /// filter.
    ///
    unsafe fn store_coefficients(&mut self, coefficient_a: *mut f64, coefficient_b: *mut f64) {

        // Iterate through each coefficient value
        // and set it in the memory locations
        //
        for idx in 0..self.m {

            self.va[idx] = _mm_set_ps(
                *coefficient_b.add(idx) as f32, 
                *coefficient_a.add(idx) as f32, 
                *coefficient_b.add(idx) as f32, 
                *coefficient_a.add(idx) as f32);
        }
    }
}
