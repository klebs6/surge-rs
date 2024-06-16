crate::ix!();

impl CheckBipolar for AdsrEnvelope {

    /// indicates whether the output of this
    /// modulation source is bipolar, returning
    /// `false` since the ADSR envelope is
    /// unipolar.
    ///
    fn is_bipolar(&self) -> bool { 
        false
    }
}

impl SetBipolar for AdsrEnvelope {

    /// sets whether the output of the ADSR
    /// envelope should be bipolar or not, which
    /// is a no-op since the ADSR envelope is
    /// always unipolar.
    ///
    #[inline] fn set_bipolar(&mut self, _b: bool) { /* no-op */ }
}
