crate::ix!();

impl crate::FilterCoeffs {

    pub fn to_normalized_lattice(&self, gain: f64, clipscale: f64) -> [f32; N_COEFFMAKER_COEFFS]
    {
        let _a0     = self.a0;
        let a0inv   = self.a0inv();
        let mut a1  = self.a1;
        let mut a2  = self.a2;
        let mut b0  = self.b0 * gain;
        let mut b1  = self.b1 * gain;
        let mut b2  = self.b2 * gain;

        b0 *= a0inv;
        b1 *= a0inv;
        b2 *= a0inv;
        a1 *= a0inv;
        a2 *= a0inv;

        let k1: f64 = a1 / (1.0 + a2);
        let k2: f64 = a2;

        let q1: f64 = {
            let x = 1.0 - k1 * k1;
            x.abs().sqrt()
        };

        let q2: f64 = {
            let x = 1.0 - k2 * k2;
            x.abs().sqrt()
        };

        let v3: f64 = b2;
        let v2: f64 = (b1 - a1 * v3) / q2;
        let v1: f64 = (b0 - k1 * v2 * q2 - k2 * v3) / (q1 * q2);

        let mut coeffs = [0.0_f32; N_COEFFMAKER_COEFFS];

        coeffs[0] = k1 as f32;
        coeffs[1] = k2 as f32;
        coeffs[2] = q1 as f32;
        coeffs[3] = q2 as f32;
        coeffs[4] = v1 as f32;
        coeffs[5] = v2 as f32;
        coeffs[6] = v3 as f32;
        coeffs[7] = clipscale as f32;

        coeffs
    }
}
