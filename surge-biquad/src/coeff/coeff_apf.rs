ix!();

use crate::{
    BiquadSetCoeffs,
    BiquadCoeffAPF,
    BiquadFilter
};

impl BiquadCoeffAPF for BiquadFilter {

    fn coeff_apf(&mut self, omega: f64, quality_factor: f64)
    {

        if !(0.0..=PI).contains(&omega) {

            self.set_coef(
                1.0, 
                0.0, 
                0.0, 
                1.0, 
                0.0, 
                0.0
            );

        } else {

            let cosi:  f64 = omega.cos();
            let sinu:  f64 = omega.sin();
            let alpha: f64 = sinu / (2.0 * quality_factor);
            let b0:    f64 = 1.0 - alpha;
            let b1:    f64 = -2.0 * cosi;
            let b2:    f64 = 1.0 + alpha;
            let a0:    f64 = 1.0 + alpha;
            let a1:    f64 = -2.0 * cosi;
            let a2:    f64 = 1.0 - alpha;

            self.set_coef(a0, a1, a2, b0, b1, b2);
        }
    }
}
