ix!();

use crate::CombFilter;

impl CoeffMake for CombFilter<'sr> {

    fn coeff_make(&self, freq: f32, mut reso: f32) -> [f32; N_COEFFMAKER_COEFFS]
    {
        let dtime: f32 = {

            let note_pitch_ignoring_tuning =
                self.tuner.n2p::<f64,true>(-freq as f64);

            let mut result: f32 = 
                (1.0 / CONCERT_A_HZ as f32) * 
                note_pitch_ignoring_tuning as f32;

            // 1 sample for feedback, 
            // 1 sample for the IIR-filter without resonance
            result = 
                result * self.srunit.samplerate_os() 
                - FIR_OFFSET as f32; 

            result = limit_range(
                result, 
                FIR_IPOL_N as f32, 
                MAX_FB_COMB as f32 - FIR_IPOL_N as f32
            );

            result
        };

        reso = self.reso_factor() * limit_range(reso, 0.0, 1.0);

        let mut coeffs = [0.0_f32; N_COEFFMAKER_COEFFS];
        coeffs[0] = dtime;
        coeffs[1] = reso;
        coeffs[2] = self.combmix();
        coeffs[3] = 1.0 - coeffs[2];
        coeffs
    }
}
