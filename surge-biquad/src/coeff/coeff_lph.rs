ix!();

use crate::{
    BiquadSetCoeffs,
    BiquadCoeffLPHMorph,
    BiquadFilter,
};

impl BiquadCoeffLPHMorph for BiquadFilter<'sr> {

    fn coeff_lph_morph(&mut self, 
        omega: f64, quality_factor: f64, _morph: f64)
    {
        /*TODO: why was this commented block uncommented?
        let mut HP: f64 = limit_range(morph as f32, 0.0_f32, 1.0_f32) as f64; 
        let mut LP: f64 = 1.0 - HP; 
        let mut BP: f64 = LP * HP;

        HP *= HP;
        LP *= LP;
        */

        let cosi:  f64 = omega.cos();
        let sinu:  f64 = omega.sin();
        let alpha: f64 = sinu / (2.0 * quality_factor);
        let b0:    f64 = alpha;
        let b1:    f64 = 0.0;
        let b2:    f64 = -alpha;
        let a0:    f64 = 1.0 + alpha;
        let a1:    f64 = -2.0 * cosi;
        let a2:    f64 = 1.0 - alpha;

        self.set_coef(a0, a1, a2, b0, b1, b2);
    }
}
