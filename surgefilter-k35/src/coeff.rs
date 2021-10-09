ix!();

use crate::{
    K35Filter,
    C,
};

impl CoeffMake for K35Filter<'sr> {

    fn coeff_make(&self, 
        freq: f32, reso: f32) -> [f32; N_COEFFMAKER_COEFFS] 
    {
        let mut coeffs = [0.0_f32; N_COEFFMAKER_COEFFS];

        let samplerate_os     = self.srunit.samplerate_os();
        let samplerate_os_inv = self.srunit.samplerate_os_inv();

        let wd:  f32 = self.clamped_frequency( freq ) * 2.0 * PI_32;
        let wa:  f32 = (2.0 * samplerate_os) * fasttan(wd * samplerate_os_inv * 0.5);
        let g:   f32 = wa * samplerate_os_inv * 0.5;
        let gp1: f32 = 1.0 + g; // g plus 1
        let gscaled:   f32 = g / gp1;

        let k:   f32 = reso * 1.96;

        let mk:  f32 = limit_range(k, 0.01, 1.96);

        coeffs[C::G] = gscaled;

        if self.is_lowpass {
            coeffs[C::LpfBeta] = (mk - mk * gscaled) / gp1;
            coeffs[C::HpfBeta] = -1.0 / gp1;
        } else {
            coeffs[C::LpfBeta] =  1.0 / gp1;
            coeffs[C::HpfBeta] = -gscaled / gp1;
        }

        coeffs[C::KModded]    = mk;
        coeffs[C::Alpha]      = 1.0 / (1.0 - mk * gscaled + mk * gscaled * gscaled);
        coeffs[C::Saturation] = self.saturation;

        coeffs[C::SaturationBlend] = std::cmp::min(FloatOrd(self.saturation), FloatOrd(1.0)).0;

        coeffs[C::SaturationBlendInv] = 1.0 - coeffs[C::SaturationBlend];

        coeffs
    }
}
