crate::ix!();

pub struct DiodeLadderFilter {
    pub tuner:  TunerHandle,
    pub srunit: SampleRateHandle,
}

impl DiodeLadderFilter {

    pub fn clamped_frequency( &self, pitch: f32) -> f32
    {
        let samplerate_os = self.srunit.samplerate_os();
        let freq = self.tuner.n2p::<f32, true>( pitch + 69.0 ) * (MIDI_0_FREQ as f32);
        limit_range( freq, 5.0, samplerate_os * 0.3 )
    }

    #[inline] pub fn get_feedback_output(
        beta:     &__m128,
        delta:    &__m128,
        feedback: &__m128,
        z:        &__m128) -> __m128 
    {
        // (feedback * delta + z) * beta
        unsafe {
            _mm_mul_ps(
                _mm_add_ps(
                    _mm_mul_ps(*feedback,*delta),
                    *z,
                ),
                *beta,
            )
        }
    }
}

// can't fit all the coefficients in 
// the 8-coefficient limit, so we have to 
// compute a lot of stuff per sample q_q
coeffidx![
    DiodeLadderCoeff;
    Alpha,
    Gamma,
    G,
    G4,
    G3,
    G2,
    G1,
    KModded
];

pub type C = DiodeLadderCoeff;
pub type R = DiodeLadderState;

coeffidx![
    DiodeLadderState;
    Z1, // z-1 state for LPF 1
    Z2, // LPF2
    Z3, // ...
    Z4,
    Feedback3, // feedback for LPF3 (feedback for LPF4 is 0)
    Feedback2,
    Feedback1
];
