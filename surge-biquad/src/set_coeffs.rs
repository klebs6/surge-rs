crate::ix!();

pub trait BiquadSetCoeffs {

    /// used to set the coefficients of a biquad
    /// filter. The function takes six arguments,
    /// `a0`, `a1`, `a2`, `b0`, `b1`, and `b2`, which
    /// are the coefficients of the filter transfer
    /// function.
    ///
    fn set_coef(&mut self, 
        a0: f64, 
        a1: f64, 
        a2: f64, 
        b0: f64, 
        b1: f64, 
        b2: f64);
}

/// The `BiquadSetCoeffs` trait defines a method
/// `set_coef()` that sets the coefficients of
/// a biquad filter, which is used to process
/// audio signals. The implementation of this
/// trait sets the coefficients for the filter and
/// updates them with new values.
///
impl BiquadSetCoeffs for BiquadFilter {

    fn set_coef(&mut self, 
        a0: f64, 
        mut a1: f64, 
        mut a2: f64, 
        mut b0: f64, 
        mut b1: f64, 
        mut b2: f64)
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
