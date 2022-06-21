crate::ix!();

impl CoeffMake for DiodeLadderFilter {

    fn coeff_make(&self, freq: f32, reso: f32) -> [f32; N_COEFFMAKER_COEFFS]
    {
        let samplerate_os     = self.srunit.samplerate_os();
        let samplerate_os_inv = self.srunit.samplerate_os_inv();

        let mut coeffs   = [0.0_f32; N_COEFFMAKER_COEFFS];

        let wd: f32 = self.clamped_frequency( freq ) * 2.0 * PI_32;
        let wa: f32 = (2.0 * samplerate_os) * fasttan(wd * samplerate_os_inv * 0.5);
        let g:  f32 = wa * samplerate_os_inv * 0.5;

        let g4: f32 = 0.5 * g / (1.0 + g);
        let g3: f32 = 0.5 * g / (1.0 + g - 0.5 * g * g4);
        let g2: f32 = 0.5 * g / (1.0 + g - 0.5 * g * g3);
        let g1: f32 = g / (1.0 + g - g * g2);

        let m_gamma: f32 = g4 * g3 * g2 * g1;
        let g:       f32 = g / (1.0 + g);
        let k:       f32 = reso * 16.0;

        let km: f32 = limit_range(k, 0.0, 16.0);

        coeffs[C::Alpha]   = g;
        coeffs[C::Gamma]   = m_gamma;
        coeffs[C::G]       = g;
        coeffs[C::G4]      = g4;
        coeffs[C::G3]      = g3;
        coeffs[C::G2]      = g2;
        coeffs[C::G1]      = g1;
        coeffs[C::KModded] = km;
        coeffs
    }
}

