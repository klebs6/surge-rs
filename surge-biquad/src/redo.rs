crate::ix!();

/// The `Redo` trait only has one method,
/// `redo()`, which in this implementation does
/// nothing but can be used to change
/// interpolation. 
///
/// It seems that the purpose of this trait is to
/// provide a way to trigger a recalculation of
/// the filter coefficients if necessary.
///
impl Redo for BiquadFilter {

    ///coeff_same_as_last_time
    fn redo(&mut self)
    {
        // If you want to change interpolation then set dv = 0 here
    }
}
