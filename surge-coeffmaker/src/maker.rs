crate::ix!();

pub const SMOOTH: f32 = 0.2;

#[derive(Debug)]
pub struct FilterCoefficientMaker 
{
    //K1,K2,Q1,Q2,V1,V2,V3,etc
    pub coeff:      A1d::<f32>,
    pub dcoeff:     A1d::<f32>,
    pub tcoeff:     A1d::<f32>,
    pub first_run:  bool,
    pub tuner:      TunerHandle,
    pub tables:     TablesHandle,
    pub srunit:     SampleRateHandle,
}

impl FilterCoefficientMaker {

    pub fn make_coeffs(
        &mut self, 
        freq: f32, 
        reso: f32, 
        generator: Box<dyn CoeffMake>)
    {
        let n = generator.coeff_make(freq, reso);
        self.from_direct(n);
    }

    fn from_direct(&mut self, coeffs: [f32; N_COEFFMAKER_COEFFS]) 
    {
        //TODO: why do we ignore the first run?
        if self.first_run
        {
            self.dcoeff.fill(0.0);
            self.coeff.fill(0.0);
            self.tcoeff.fill(0.0);
            self.first_run = false;

        } else {

            for (i, item) in coeffs.iter().enumerate().take(N_COEFFMAKER_COEFFS) {
                self.tcoeff[i] = (1.0 - SMOOTH) * self.tcoeff[i] + SMOOTH * item;
                self.dcoeff[i] = (self.tcoeff[i] - self.coeff[i]) * BLOCK_SIZE_OS_INV;
            }
        }
    }
}
