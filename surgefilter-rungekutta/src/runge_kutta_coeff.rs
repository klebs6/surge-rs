ix!();

use crate::{
    RungeKuttaCoeff,
    RungeKuttaLadder,
};

impl CoeffMake for RungeKuttaLadder<'sr> {

    fn coeff_make(&self, freq: f32, reso: f32) -> [f32; N_COEFFMAKER_COEFFS] 
    {
        // Consideration: Do we want tuning aware or not?
        let mut coeffs = [0.0_f32; N_COEFFMAKER_COEFFS];

        let pitch: f32 = self.clamped_frequency( freq );

        coeffs[RungeKuttaCoeff::Cutoff as usize] = pitch * 2.0 * PI_32;

        // code says 0-10 is value but above 4 it is just out of tune self-oscillation
        coeffs[RungeKuttaCoeff::Reso as usize]  = limit_range( reso, 0.0, 1.0 ) * 4.5; 
        coeffs[RungeKuttaCoeff::GainCompensation as usize] = 0.0;

        if let Some(gc) = self.gain_compensation {
            coeffs[RungeKuttaCoeff::GainCompensation as usize] = gc;
        }

        coeffs
    }
}
