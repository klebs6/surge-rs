ix!();

use crate::{
    BiquadSetCoeffs,
    BiquadCoeffBP,
    BiquadCoeffBP2A,
    BiquadFilter
};

impl BiquadCoeffBP for BiquadFilter {

    fn coeff_bp(&mut self, omega: f64, quality_factor: f64)
    {
        let cosi:  f64 = omega.cos();
        let sinu:  f64 = omega.sin();
        let alpha: f64 = sinu / (2.0 * quality_factor);
        let b0:    f64 = alpha;
        let b2:    f64 = -alpha;
        let a0:    f64 = 1.0 + alpha;
        let a1:    f64 = -2.0 * cosi;
        let a2:    f64 = 1.0 - alpha;

        self.set_coef(a0, a1, a2, b0, 0.0, b2);
    }
}

impl BiquadCoeffBP2A for BiquadFilter {

    fn coeff_bp2a(&mut self, omega: f64, bandwidth: f64)
    {
        let cosi:  f64 = omega.cos();
        let sinu:  f64 = omega.sin();
        let q:     f64 = 1.0 / (0.02 + 30.0 * bandwidth * bandwidth);
        let alpha: f64 = sinu / (2.0 * q);
        let b0:    f64 = alpha;
        let b2:    f64 = -alpha;
        let a0:    f64 = 1.0 + alpha;
        let a1:    f64 = -2.0 * cosi;
        let a2:    f64 = 1.0 - alpha;

        self.set_coef(a0, a1, a2, b0, 0.0, b2);
    }
}
