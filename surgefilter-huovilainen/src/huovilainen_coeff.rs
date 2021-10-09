ix!();

use crate::{
    HuovilainenLadder,
    C,
    HUOVILAINEN_EXTRA_OVERSAMPLE_INV,
};

impl CoeffMake for HuovilainenLadder<'sr> {

    fn coeff_make(&self, freq: f32, mut reso: f32) 
        -> [f32; N_COEFFMAKER_COEFFS]
    {
        let samplerate         = self.srunit.samplerate();
        let samplerate_os_inv  = self.srunit.samplerate_os_inv();

        let mut coeffs = [0.0_f32; N_COEFFMAKER_COEFFS];

        let cutoff = self.clamped_frequency( freq );

        coeffs[C::Cutoff] = cutoff;

        // Heueristically at higher cutoffs the resonance becomes less stable. 
        // This is purely ear tuned at 49khz with noise input
        let co: f32 = std::cmp::max( 
            FloatOrd(cutoff - samplerate * 0.33333), 
            FloatOrd(0.0) ).0 
            * 0.1 * samplerate_os_inv;

        let gctrim: f32 = match self.gain_compensation {
            Some(_) => 0.05,
            None => 0.0,
        };

        reso = limit_range( 
            limit_range( reso, 0.0, 0.9925 ), 
            0.0, 
            0.994 - co - gctrim 
        );

        coeffs[C::Res] = reso;

        let fc: f32 =  cutoff * samplerate_os_inv * HUOVILAINEN_EXTRA_OVERSAMPLE_INV;

        coeffs[C::Fc] = fc;

        coeffs[C::GainCompensation] = self.gain_compensation.unwrap_or(0.0);

        coeffs
    }
}
