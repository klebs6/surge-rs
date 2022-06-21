crate::ix!();

impl FilterCoefficientMaker {

    pub fn new_coeffbank() -> A1d::<f32> {
        A1d::<f32>::zeros(N_COEFFMAKER_COEFFS)
    }

    pub fn new( 
        tuner:  TunerHandle,
        tables: TablesHandle,
        srunit: SampleRateHandle,
    ) -> Self {

        let mut x = Self {
            coeff:      Self::new_coeffbank(),
            dcoeff:     Self::new_coeffbank(),
            tcoeff:     Self::new_coeffbank(),
            first_run:  true,
            tuner:      tuner.clone(),
            tables:     tables.clone(),
            srunit:     srunit.clone()
        };

        x.reset();
        x
    }
}

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
