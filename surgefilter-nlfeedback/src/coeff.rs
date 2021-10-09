ix!();

use crate::{
    C,
    NonlinearFeedbackFilter, 
    NLFFType,
};

impl CoeffMake for NonlinearFeedbackFilter<'sr> {

    fn coeff_make(&self, freq: f32, mut reso: f32) -> [f32; N_COEFFMAKER_COEFFS] {

        let mut coeffs = [0.0_f32; N_COEFFMAKER_COEFFS];

        reso = limit_range(reso, 0.0, 1.0);

        let q:               f32 = (reso * reso * reso) * 18.0 + 0.1;
        let normalized_freq: f32 = 2.0 * self.clamped_frequency(freq) / self.srunit.samplerate_os();
        let wc:              f32 = PI_32 * normalized_freq;
        let wsin:            f32 = fastsin(wc);
        let wcos:            f32 = fastcos(wc);
        let alpha:           f32 = wsin / (2.0 * q);

        // note we actually calculate the reciprocal of a0 
        // because we only use a0 to normalize the
        // other coefficients, and multiplication by 
        // reciprocal is cheaper than dividing.
        let a0r: f32 = 1.0 / (1.0 + alpha);

        coeffs[C::A1]     = -2.0 * wcos * a0r;
        coeffs[C::A2]     = (1.0 - alpha) * a0r;
        coeffs[C::Makeup] = 1.0;

        //To see where this table comes from 
        //look in the HeadlessNonTestFunctions.
        let use_normalization = true;
        let mut norm_numerator: f32 = 1.0;

        // tweaked these by hand/ear after the RMS 
        // measuring program did its thing... 
        // this world still needs humans! :) - EvilDragon
        const LP_NORM_TABLE: [f32; 12] = [ 
            1.53273,  
            1.33407,  
            1.08197,  
            0.958219, 
            1.27374,  
            0.932342, 
            0.761765, 
            0.665462, 
            0.776856, 
            0.597575, 
            0.496207, 
            0.471714
        ];

        // extra resonance makeup term for OJD subtypes
        let exp_min: f32 = match self.ty == NLFFType::LowPass {
            true  => 0.1,
            false => 0.35,
        };

        //TODO: this match condition does not seem correct
        let res_makeup: f32 = match (self.subty as usize) < 8 {
            true => 1.0,
            false => {
                1.0 / std::cmp::max(FloatOrd(reso), FloatOrd(exp_min)).0.powf(0.5)
            },
        };

        match self.ty {
            NLFFType::LowPass => {
                coeffs[C::B1] = (1.0 - wcos) * a0r;
                coeffs[C::B0] = coeffs[C::B1] * 0.5;
                coeffs[C::B2] = coeffs[C::B0];

                if use_normalization
                {
                    norm_numerator = LP_NORM_TABLE[self.subty as usize];
                }

                coeffs[C::Makeup] = 
                    res_makeup * norm_numerator 
                    / std::cmp::max(
                        FloatOrd(normalized_freq), 
                        FloatOrd(0.001)
                    ).0.powf(0.1);
            },
            NLFFType::HighPass => {
                coeffs[C::B1] = -(1.0 + wcos) * a0r;
                coeffs[C::B0] = coeffs[C::B1] * -0.5;
                coeffs[C::B2] = coeffs[C::B0];

                if use_normalization 
                {
                    norm_numerator = LP_NORM_TABLE[self.subty as usize];
                }

                coeffs[C::Makeup] = 
                    res_makeup * norm_numerator 
                    / std::cmp::max(
                        FloatOrd(1.0 - normalized_freq), 
                        FloatOrd(0.001)
                    ).0.powf(0.1);
            },
            NLFFType::BandPass => {
                coeffs[C::B0] = wsin * 0.5 * a0r;
                coeffs[C::B1] = 0.0;
                coeffs[C::B2] = -coeffs[C::B0];
            },
            NLFFType::Notch => {
                coeffs[C::B0] = a0r;
                coeffs[C::B1] = -2.0 * wcos * a0r;
                coeffs[C::B2] = coeffs[C::B0];
            },
            NLFFType::Allpass => {
                coeffs[C::B0] = coeffs[C::A2];
                coeffs[C::B1] = coeffs[C::A1];
                coeffs[C::B2] = 1.0; // (1+a) / (1+a) = 1 (from normalising by a0)
            },
        }
        coeffs
    }
}
