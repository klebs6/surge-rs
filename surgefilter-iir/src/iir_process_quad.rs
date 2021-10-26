ix!();

use crate::{
    IIRFilter,
};

//----------------------------------------------------
macro_rules! hook {
    ($f:ident, $cb:ident) => {
        #[inline] fn $f(qfu: &mut QuadFilterUnitState, input: __m128 ) -> __m128 { crate::$cb(qfu, input) }
    }
}

hook![lowpass_process_quad_rough2p,   iir_12_cfc_quad];
hook![lowpass_process_quad_smooth2p,  iir_12_b_quad];
hook![lowpass_process_quad_medium2p,  iir_12_cfl_quad];

hook![lowpass_process_quad_rough4p,   iir_24_cfc_quad];
hook![lowpass_process_quad_smooth4p,  iir_24_b_quad];
hook![lowpass_process_quad_medium4p,  iir_24_cfl_quad];

hook![highpass_process_quad_rough2p,  iir_12_cfc_quad];
hook![highpass_process_quad_smooth2p, iir_12_b_quad];
hook![highpass_process_quad_medium2p, iir_12_cfl_quad];

hook![highpass_process_quad_rough4p,  iir_24_cfc_quad];
hook![highpass_process_quad_smooth4p, iir_24_b_quad];
hook![highpass_process_quad_medium4p, iir_24_cfl_quad];

hook![bandpass_process_quad_rough2p,  iir_12_cfc_quad];
hook![bandpass_process_quad_smooth2p, iir_12_b_quad];
hook![bandpass_process_quad_medium2p, iir_12_cfl_quad];

hook![bandpass_process_quad_rough4p,  iir_24_cfc_quad];
hook![bandpass_process_quad_smooth4p, iir_24_b_quad];
hook![bandpass_process_quad_medium4p, iir_24_cfl_quad];
//----------------------------------------------------

impl FilterProcessQuad for IIRFilter {
    #[inline] fn process_quad(
        &self, 
        qfu: &mut QuadFilterUnitState, 
        input: __m128) -> __m128 
    {
        match (self.iirtype, self.subtype, self.pole) {
            (FilterTypeIIR::LowPass, FilterSubType::Rough,  PoleType::TwoPole)   => lowpass_process_quad_rough2p(qfu, input),
            (FilterTypeIIR::LowPass, FilterSubType::Rough,  PoleType::FourPole)  => lowpass_process_quad_rough4p(qfu, input),
            (FilterTypeIIR::LowPass, FilterSubType::Smooth, PoleType::TwoPole)   => lowpass_process_quad_smooth2p(qfu, input),
            (FilterTypeIIR::LowPass, FilterSubType::Smooth, PoleType::FourPole)  => lowpass_process_quad_smooth4p(qfu, input),
            (FilterTypeIIR::LowPass, FilterSubType::Medium, PoleType::TwoPole)   => lowpass_process_quad_medium2p(qfu, input),
            (FilterTypeIIR::LowPass, FilterSubType::Medium, PoleType::FourPole)  => lowpass_process_quad_medium4p(qfu, input),

            (FilterTypeIIR::HighPass, FilterSubType::Rough,  PoleType::TwoPole)  => highpass_process_quad_rough2p(qfu, input),
            (FilterTypeIIR::HighPass, FilterSubType::Rough,  PoleType::FourPole) => highpass_process_quad_rough4p(qfu, input),
            (FilterTypeIIR::HighPass, FilterSubType::Smooth, PoleType::TwoPole)  => highpass_process_quad_smooth2p(qfu, input),
            (FilterTypeIIR::HighPass, FilterSubType::Smooth, PoleType::FourPole) => highpass_process_quad_smooth4p(qfu, input),
            (FilterTypeIIR::HighPass, FilterSubType::Medium, PoleType::TwoPole)  => highpass_process_quad_medium2p(qfu, input),
            (FilterTypeIIR::HighPass, FilterSubType::Medium, PoleType::FourPole) => highpass_process_quad_medium4p(qfu, input),

            (FilterTypeIIR::BandPass, FilterSubType::Rough,  PoleType::TwoPole)  => bandpass_process_quad_rough2p(qfu, input),
            (FilterTypeIIR::BandPass, FilterSubType::Rough,  PoleType::FourPole) => bandpass_process_quad_rough4p(qfu, input),
            (FilterTypeIIR::BandPass, FilterSubType::Smooth, PoleType::TwoPole)  => bandpass_process_quad_smooth2p(qfu, input),
            (FilterTypeIIR::BandPass, FilterSubType::Smooth, PoleType::FourPole) => bandpass_process_quad_smooth4p(qfu, input),
            (FilterTypeIIR::BandPass, FilterSubType::Medium, PoleType::TwoPole)  => bandpass_process_quad_medium2p(qfu, input),
            (FilterTypeIIR::BandPass, FilterSubType::Medium, PoleType::FourPole) => bandpass_process_quad_medium4p(qfu, input),
            _ => unreachable!(),
        }
    }
}

