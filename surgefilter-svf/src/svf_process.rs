ix!();

//------------------------------------------------------
macro_rules! hook { 
    ($f:ident, $cb:ident) => { 
        #[inline] fn $f(qfu: &mut QuadFilterUnitState, input: __m128 ) -> __m128 { 
            crate::$cb(qfu, input) 
        } 
    } 
}
hook![lowpass_svf2p,  svf_lp12_a_quad];
hook![lowpass_svf4p,  svf_lp24_a_quad];
hook![highpass_svf2p, svf_hp12_a_quad];
hook![highpass_svf4p, svf_hp24_a_quad];
hook![bandpass_svf2p, svf_bp12_a_quad];
hook![bandpass_svf4p, svf_bp24_a_quad];
//------------------------------------------------------

impl FilterProcessQuad for crate::SvfFilter {

    #[inline] fn process_quad( &self, qfu: &mut QuadFilterUnitState<'_>, input: __m128) -> __m128 
    {
        match (self.iirtype, self.pole_type) {

            (FilterTypeIIR::LowPass,  PoleType::TwoPole)  => lowpass_svf2p(qfu, input),
            (FilterTypeIIR::BandPass, PoleType::TwoPole)  => bandpass_svf2p(qfu, input),
            (FilterTypeIIR::HighPass, PoleType::TwoPole)  => highpass_svf2p(qfu, input),

            (FilterTypeIIR::LowPass,  PoleType::FourPole) => lowpass_svf4p(qfu, input),
            (FilterTypeIIR::BandPass, PoleType::FourPole) => bandpass_svf4p(qfu, input),
            (FilterTypeIIR::HighPass, PoleType::FourPole) => highpass_svf4p(qfu, input),
        }
    }
}

