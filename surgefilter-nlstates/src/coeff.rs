crate::ix!();

impl CoeffMake for crate::NonlinearStatesFilter {

    fn coeff_make(&self, freq: f32, mut reso: f32) -> [f32; N_COEFFMAKER_COEFFS] {

        let mut c = [0.0_f32; N_COEFFMAKER_COEFFS];

        reso = limit_range(reso, 0.0, 1.0);

        let q: f32 = (reso * reso * reso) * 18.0 + 0.1;

        let wc: f32 = 2.0 * PI_32 * self.clamped_frequency(freq) / self.srunit.samplerate_os();

        let wsin:  f32 = fastsin(wc);
        let wcos:  f32 = fastcos(wc);
        let alpha: f32 = wsin / (2.0 * q);

        // note we actually calculate the reciprocal of a0 because we only use a0 to normalize the
        // other coefficients, and multiplication by reciprocal is cheaper than dividing.
        let a0r: f32 = 1.0 / (1.0 + alpha);

        c[C::A1] = -2.0 * wcos * a0r;
        c[C::A2] = (1.0 - alpha) * a0r;

        match self.ty {
            NLSFType::LowPass => {
                c[C::B1] = (1.0 - wcos) * a0r;
                c[C::B0] = c[C::B1] * 0.5;
                c[C::B2] = c[C::B0];
            },
            NLSFType::HighPass => {
                c[C::B1] = -(1.0 + wcos) * a0r;
                c[C::B0] = c[C::B1] * -0.5;
                c[C::B2] = c[C::B0];
            },
            NLSFType::BandPass => {
                c[C::B0] = wsin * 0.5 * a0r;
                c[C::B1] = 0.0;
                c[C::B2] = -c[C::B0];
            },
            NLSFType::Notch => {
                c[C::B0] = a0r;
                c[C::B1] = -2.0 * wcos * a0r;
                c[C::B2] = c[C::B0];
            },
            NLSFType::Allpass => {
                c[C::B0] = c[C::A2];
                c[C::B1] = c[C::A1];
                c[C::B2] = 1.0; // (1+a) / (1+a) = 1 (from normalising by a0)
            },
        }
        c
    }
}
