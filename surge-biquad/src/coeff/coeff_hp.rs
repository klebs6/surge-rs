ix!();

use crate::{
    BiquadSetCoeffs,
    BiquadCoeffHP,
    BiquadFilter
};

impl BiquadCoeffHP for BiquadFilter<'sr> {

    fn coeff_hp(&mut self, omega: f64, q: f64) {

        if omega > PI {
            self.set_coef(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        } else {
            let cosi:  f64 = omega.cos();
            let sinu:  f64 = omega.sin();
            let alpha: f64 = sinu / (2.0 * q);
            let b0:    f64 = (1.0 + cosi) * 0.5;
            let b1:    f64 = -(1.0 + cosi);
            let b2:    f64 = (1.0 + cosi) * 0.5;
            let a0:    f64 = 1.0 + alpha;
            let a1:    f64 = -2.0 * cosi;
            let a2:    f64 = 1.0 - alpha;

            self.set_coef(a0, a1, a2, b0, b1, b2);
        }
    }

    fn coeff_hp_with_bw(&mut self, omega: f64, bandwidth: f64)
    {
        self.coeff_hp(omega, 1.0 / bandwidth);
    }
}
