crate::ix!();

impl BiquadCoeffNotch for BiquadFilter {

    fn coeff_notch(&mut self, omega: f64, qq: f64)
    {
        if omega > PI {

            self.set_coef(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        } else {

            let cosi:  f64 = omega.cos();
            let sinu:  f64 = omega.sin();
            let reso:  f64 = limit_range(qq as f32, 0.0_f32, 1.0_f32) as f64 ;
            let q:     f64 = 1.0 / (0.02 + 30.0 * reso * reso);
            let alpha: f64 = sinu / (2.0 * q);
            let b0:    f64 = 1.0;
            let b1:    f64 = -2.0 * cosi;
            let b2:    f64 = 1.0;
            let a0:    f64 = 1.0 + alpha;
            let a1:    f64 = -2.0 * cosi;
            let a2:    f64 = 1.0 - alpha;

            self.set_coef(a0, a1, a2, b0, b1, b2);
        }
    }
}
