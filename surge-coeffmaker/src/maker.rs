crate::ix!();

/// a constant value of 0.2, which represents the
/// smoothing factor used in the filter
/// coefficient calculations.
///
pub const SMOOTH: f32 = 0.2;

/// a struct that holds various fields for
/// creating filter coefficients.
///
#[derive(Debug)]
pub struct FilterCoefficientMaker 
{
    /// an array of length `N_COEFFMAKER_COEFFS`
    /// that holds the filter coefficients.
    ///
    /// K1,K2,Q1,Q2,V1,V2,V3,etc
    ///
    pub coeff:      A1d::<f32>,

    /// an array of length `N_COEFFMAKER_COEFFS`
    /// that holds the delta coefficients.
    ///
    pub dcoeff:     A1d::<f32>,

    /// an array of length `N_COEFFMAKER_COEFFS`
    /// that holds the target coefficients.
    ///
    pub tcoeff:     A1d::<f32>,

    /// a boolean that represents whether or not
    /// this is the first run of the filter
    /// coefficient maker.
    ///
    pub first_run:  bool,

    pub tuner:      TunerHandle,
    pub tables:     TablesHandle,
    pub srunit:     SampleRateHandle,
}

impl FilterCoefficientMaker {

    /// Inputs:
    ///
    /// - a frequency (`freq`), 
    ///
    /// - resonance (`reso`), and 
    ///
    /// - a generator (`Box<dyn CoeffMake>`) 
    ///
    /// uses them to create new filter
    /// coefficients.
    ///
    pub fn make_coeffs(
        &mut self, 
        freq: f32, 
        reso: f32, 
        generator: Box<dyn CoeffMake>)
    {
        let n = generator.coeff_make(freq, reso);
        self.from_direct(n);
    }

    /// updates the `dcoeff` and `tcoeff` arrays
    /// based on the values in `coeffs`.
    ///
    pub fn from_direct(&mut self, coeffs: [f32; N_COEFFMAKER_COEFFS]) 
    {
        // checks if this is the first run of the
        // filter coefficient maker. 
        //
        // If it is, it sets all of the arrays to
        // 0 and sets `first_run` to `false`. 
        //
        // If it's not the first run, it updates
        // the `dcoeff` and `tcoeff` arrays based
        // on the values in `coeffs`.
        //
        if self.first_run
        {
            self.dcoeff.fill(0.0);
            self.coeff.fill(0.0);
            self.tcoeff.fill(0.0);
            self.first_run = false;

        } else {

            // iterates over the `coeffs` array
            // and updates the `tcoeff` and
            // `dcoeff` arrays based on the values
            // in `coeffs`. 
            //
            // Specifically, it updates `tcoeff` using a smoothing function 
            // (`(1.0 - SMOOTH) * self.tcoeff[i] + SMOOTH * item`) 
            //
            // and `dcoeff` by taking the difference between `tcoeff` and `coeff` and 
            //
            // dividing by the inverse of the block size.
            //
            for (i, item) in coeffs.iter().enumerate().take(N_COEFFMAKER_COEFFS) {

                self.tcoeff[i] = (1.0 - SMOOTH) * self.tcoeff[i] + SMOOTH * item;

                self.dcoeff[i] = (self.tcoeff[i] - self.coeff[i]) * BLOCK_SIZE_OS_INV;
            }
        }
    }
}
