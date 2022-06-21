crate::ix!();

impl crate::FilterCoeffs {

    pub fn to_coupled_form(&self, gain: f64, clipscale: f64) -> [f32; N_COEFFMAKER_COEFFS]
    {
        let _a0    = self.a0;
        let a0inv  = self.a0inv();
        let mut a1 = self.a1;
        let mut a2 = self.a2;
        let mut b0 = self.b0 * gain;
        let mut b1 = self.b1 * gain;
        let mut b2 = self.b2 * gain;

        b0 *= a0inv;
        b1 *= a0inv;
        b2 *= a0inv;
        a1 *= a0inv;
        a2 *= a0inv;

        let ar: f64 = 0.5 * -a1;

        let sq: f64 = {
            let x: f64 = a1 * a1 - 4.0 * a2;
            std::cmp::min(FloatOrd(0.0), FloatOrd(x)).0
        };

        let ai: f64 = {
            let x = 0.5 * (-sq).sqrt();
            std::cmp::max(
                FloatOrd(x), 
                FloatOrd(8.0 * 1.192092896e-07)
            ).0
        };

        let bb1: f64 = b1 - a1 * b0;
        let bb2: f64 = b2 - a2 * b0;

        let d:  f64 = b0;
        let c1: f64 = bb1;
        let c2: f64 = (bb1 * ar + bb2) / ai;

        let _scalar: f64 = 1.0; // 0.01 + 0.99*sqrt(c1*c1);

        let mut coeffs = [0.0_f32; N_COEFFMAKER_COEFFS];

        coeffs[0] = ar  as f32;
        coeffs[1] = ai  as f32;
        coeffs[2] = 1.0_f32; // scalar;

        coeffs[3] = 0.0; //TODO: is this correct?

        coeffs[4] = c1  as f32; // /scalar;
        coeffs[5] = c2  as f32; // /scalar;
        coeffs[6] = d   as f32;

        coeffs[7] = clipscale as f32;

        coeffs
    }
}
