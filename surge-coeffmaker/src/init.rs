crate::ix!();

impl FilterCoefficientMaker {

    /// This function creates a new coefficient bank with `N_COEFFMAKER_COEFFS` coefficients and
    /// initializes them to zero. 
    ///
    /// It returns the coefficient bank as a 1-dimensional array of `f32` values.
    ///
    pub fn new_coeffbank() -> A1d::<f32> {
        A1d::<f32>::zeros(N_COEFFMAKER_COEFFS)
    }

    /// This function is the constructor for `FilterCoefficientMaker`. It takes three arguments:
    /// a `TunerHandle`, a `TablesHandle`, and a `SampleRateHandle`. 
    ///
    /// These handles are presumably used to access some resources needed to compute filter
    /// coefficients. The function returns a new instance of `FilterCoefficientMaker`.
    ///
    pub fn new( 
        tuner:  TunerHandle,
        tables: TablesHandle,
        srunit: SampleRateHandle,
    ) -> Self {

        // This creates a new instance of `FilterCoefficientMaker` and initializes its fields. 
        //
        // The `coeff`, `dcoeff`, and `tcoeff` fields are initialized by calling
        // `new_coeffbank()`. The `first_run` field is set to `true`. 
        //
        // The `tuner`, `tables`, and `srunit` fields are set to copies of the corresponding
        // argument handles.
        //
        let mut x = Self {
            coeff:      Self::new_coeffbank(),
            dcoeff:     Self::new_coeffbank(),
            tcoeff:     Self::new_coeffbank(),
            first_run:  true,
            tuner:      tuner.clone(),
            tables:     tables.clone(),
            srunit:     srunit.clone()
        };

        // This calls the `reset()` method on the newly created `FilterCoefficientMaker` instance
        // and returns it. 
        //
        // The `reset()` method sets the `first_run`, `coeff`, `dcoeff`, and `tcoeff` fields to
        // their initial values.
        //
        x.reset();
        x
    }
}

/// This code defines an implementation block for the `Reset` trait for the
/// `FilterCoefficientMaker` type. 
///
/// The `Reset` trait defines a single method, `reset()`, which resets the state of an object to
/// its initial values. 
///
/// The `reset()` method sets the `first_run`, `coeff`, `dcoeff`, and `tcoeff` fields to their
/// initial values.
///
impl Reset for FilterCoefficientMaker 
{
    fn reset(&mut self) 
    {
        self.first_run = true;
        self.coeff.fill(0.0);
        self.dcoeff.fill(0.0);
        self.tcoeff.fill(0.0);
    }
}
