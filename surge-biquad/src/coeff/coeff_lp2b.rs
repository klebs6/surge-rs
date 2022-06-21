crate::ix!();

impl BiquadCoeffLP2B for BiquadFilter {

    fn coeff_lp2b(&mut self, omega: f64, quality_factor: f64)
    {
        if omega > PI {
            self.set_coef(
                1.0, 
                0.0, 
                0.0, 
                1.0, 
                0.0, 
                0.0
            );

        } else {

            let w_sq:  f64 = omega * omega;

            let den:   f64 = 
                (w_sq * w_sq) 
                + (PI * PI * PI * PI) 
                + w_sq * (PI * PI) * (1.0 / quality_factor - 2.0);

            let g1:    f64 = std::cmp::min(
                FloatOrd(1.0), 
                FloatOrd(((w_sq * w_sq) / den).sqrt() * 0.5)
            ).0;

            let cosi:  f64 = omega.cos();
            let sinu:  f64 = omega.sin();
            let alpha: f64 = sinu / (2.0 * quality_factor);
            let a:     f64 = 2.0 * g1.sqrt() * (2.0 - g1).sqrt();
            let b0:    f64 = (1.0 - cosi + g1 * (1.0 + cosi) + a * sinu) * 0.5;
            let b1:    f64 = 1.0 - cosi - g1 * (1.0 + cosi);
            let b2:    f64 = (1.0 - cosi + g1 * (1.0 + cosi) - a * sinu) * 0.5;
            let a0:    f64 = 1.0 + alpha; 
            let a1:    f64 = -2.0 * cosi;
            let a2:    f64 = 1.0 - alpha;

            self.set_coef(a0, a1, a2, b0, b1, b2);
        }
    }
}
