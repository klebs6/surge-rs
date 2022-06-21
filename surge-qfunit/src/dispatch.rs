crate::ix!();

pub type WaveshaperQFPtr 
= fn(&mut WaveshaperState, input: __m128, drive: __m128) -> __m128;

pub type FilterUnitQFPtr 
= fn(&mut QuadFilterUnitState, __m128) -> __m128;

pub struct FbqGlobal {
    pub fu1ptr:   Option<FilterUnitQFPtr>,
    pub fu2ptr:   Option<FilterUnitQFPtr>,
    pub wsptr:    Option<WaveshaperQFPtr>,
}

pub type FBQFPtr = dyn Fn(
    &mut WaveshaperState,
    &mut QuadFilterChainState, 
    &mut FbqGlobal, 
    *mut f32, *mut f32);


#[allow(unused_variables)]
pub fn get_quad_filter_waveshaper_ptr(ty: WaveshapeType) -> WaveshaperQFPtr {
    todo!();
    /*
    match ty {
        WaveshapeType::Off => {
            panic!("logic bug");
        },
        WaveshapeType::SoftTanh => {
            WaveshaperState::tanh
        },
        WaveshapeType::Hard => {
            WaveshaperState::clip
        },
        WaveshapeType::Asymmetric => {
            WaveshaperState::asym_sse2
        },
        WaveshapeType::Sine => {
            WaveshaperState::sine_sse2
        },
        WaveshapeType::Digital => {
            WaveshaperState::digi_sse2
        },
    }
    */
}

#[allow(unused_variables)]
pub fn get_quad_filter_ptr(ty: FilterType, subty: Option<FilterSubType>) -> FilterUnitQFPtr {
    todo!();
    /*
    match ty {

        FilterType::Off => {
            panic!("logic bug");//TODO: can we eliminate this branch?
        },

        FilterType::MoogLadder => { QuadFilterUnitState::lp_moog_quad },

        FilterType::Notch => {
            QuadFilterUnitState::iir_12_b_quad
        },

        /*
        FilterType::Comb => {
            QuadFilterUnitState::COMB_quad_SSE2
        },
        */

        FilterType::SampleAndHold => {
            QuadFilterUnitState::SNH_quad
        },
        FilterType::DiodeLadder => {
            QuadFilterUnitState::diode_ladder
        },
        FilterType::K35Highpass => {
            QuadFilterUnitState::k35_hp
        },
        FilterType::K35Lowpass => {
            QuadFilterUnitState::k35_lp
        },
        FilterType::Obxd2Pole => {
            QuadFilterUnitState::obxd_2pole
        },
        FilterType::Obxd4Pole => {
            QuadFilterUnitState::obxd_4pole
        },
        FilterType::VintageLadderRungeKutta => {
            QuadFilterUnitState::vintage_ladder_rk
        },
        FilterType::VintageLadderHuovilainen => {
            QuadFilterUnitState::vintage_ladder_hv
        },
        FilterType::NonlinearFeedback => {
            QuadFilterUnitState::nonlinear_feedback
        },
        FilterType::NonlinearStates => {
            QuadFilterUnitState::nonlinear_states
        },
    }
    */
}
