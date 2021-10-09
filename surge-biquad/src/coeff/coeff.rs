ix!();

use crate::{
    BiquadSetCoeffs,
    BiquadFilter,
};

impl Redo for BiquadFilter<'sr> {

    ///coeff_same_as_last_time
    fn redo(&mut self)
    {
        // If you want to change interpolation then set dv = 0 here
    }
}

impl BiquadSetCoeffs for BiquadFilter<'sr> {

    fn set_coef(&mut self, 
        a0: f64, mut a1: f64, mut a2: f64, mut b0: f64, mut b1: f64, mut b2: f64)
    {
        let a0inv: f64 = 1.0 / a0;

        b0 *= a0inv;
        b1 *= a0inv;
        b2 *= a0inv;
        a1 *= a0inv;
        a2 *= a0inv;

        if self.first_run
        {
            self.a1.start_value(a1);
            self.a2.start_value(a2);
            self.b0.start_value(b0);
            self.b1.start_value(b1);
            self.b2.start_value(b2);
            self.first_run = false;
        }

        self.a1.new_value(a1);
        self.a2.new_value(a2);
        self.b0.new_value(b0);
        self.b1.new_value(b1);
        self.b2.new_value(b2);
    }
}
