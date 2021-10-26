ix!();

coeffidx![
    C;
    G,                  // aka alpha
    LpfBeta,            // LPF beta
    HpfBeta,            // HPF beta
    KModded,            // k (m_k_modded)
    Alpha,              // aka m_alpha
    Saturation,         // amount of saturation to apply (scaling before tanh)
    SaturationBlend,    // above but clamped to 0..1, used to blend tanh version when <1
    SaturationBlendInv  // above but inverted, used to blend non-tanh version when <1
];

coeffidx![
    R;
    Lpf1z1,
    Hpf1z1,
    Xpf2z1
];

/**
 ** This contains various adaptations of the models found at
 ** https://github.com/TheWaveWarden/odin2/blob/master/Source/audio/Filters/Korg35Filter.cpp
 */
pub struct K35Filter {
    pub tuner:      TunerHandle,
    pub srunit:     SampleRateHandle,
    pub is_lowpass: bool,
    pub saturation: f32,
}

impl K35Filter {
    #[inline] pub fn clamped_frequency(&self, pitch: f32) -> f32 {
        let mut freq: f32 = self.tuner.n2p::<f32,true>( pitch + 69.0 ) * (MIDI_0_FREQ as f32);
        freq = limit_range( freq, 5.0, self.srunit.samplerate_os() * 0.3 );
        freq
    }

    /**
      | note that things that were NOPs in the
      | Odin code have been removed.
      |
      | m_gamma remains 1.0 so xn * m_gamma == xn;
      | that's a NOP
      |
      | m_feedback remains 0, that's a NOP
      |
      | m_epsilon remains 0, that's a NOP
      |
      | m_a_0 remains 1 so that's also a NOP
      |
      | so we only need to compute: (xn - z)
      | * alpha + za
      */
    #[inline] pub fn do_lpf(g: &__m128, input: &__m128, z: &mut __m128) -> __m128 
    {
        unsafe {
            let v:      __m128 = _mm_mul_ps(_mm_sub_ps(*input, *z), *g);
            let result: __m128 = _mm_add_ps(v, *z);
            *z = _mm_add_ps(v, result);
            result
        }
    }

    #[inline] pub fn do_hpf(g: &__m128, input: &__m128, z: &mut __m128) -> __m128 {
        unsafe {
            _mm_sub_ps(*input, Self::do_lpf(g, input, z))
        }
    }
}

