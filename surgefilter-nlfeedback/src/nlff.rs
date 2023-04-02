and here is the struct definition itself!


crate::ix!();

enhanced_enum![ 
    NLFFSaturator {
        Tanh,
        Soft,
        Ojd,
        Sine,
    }
];

coeffidx![ 
    NLFFCoeff;
    A1,
    A2,
    B0,
    B1,
    B2,
    Makeup
];

enhanced_enum![ 
    NLFFType {
        LowPass,
        HighPass,
        BandPass,
        Notch,
        Allpass,
    }
];

coeffidx![ 
    NLFFState; 
    Z1, // 1st z-1 state for first  stage
    Z2, // 2nd z-1 state for first  stage
    Z3, // 1st z-1 state for second stage
    Z4, // 2nd z-1 state for second stage
    Z5, // 1st z-1 state for third  stage
    Z6, // 2nd z-1 state for third  stage
    Z7, // 1st z-1 state for fourth stage
    Z8 // 2nd z-1 state for fourth stage
];

pub type R = NLFFState;
pub type C = NLFFCoeff;

/**
** This contains an adaptation of the filter found at
** https://ccrma.stanford.edu/~jatin/ComplexNonlinearities/NLFeedback.html
** with coefficient calculation from
** https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html
*/
pub struct NonlinearFeedbackFilter {
    pub tuner:  TunerHandle,
    pub srunit: SampleRateHandle,
    pub ty:     NLFFType,
    pub subty:  FilterSubType,
}

impl NonlinearFeedbackFilter {

    #[inline] pub fn clamped_frequency(&self, pitch: f32) -> f32 {
        let freq = self.tuner.n2p::<f32,true>(pitch + 69.0) * (MIDI_0_FREQ as f32);
        limit_range( freq, 5.0, self.srunit.samplerate_os() * 0.3)
    }

    /// sine each element of a __m128 by breaking it into floats then reassembling
    #[inline] pub fn fastsin_ps(input: &__m128) -> __m128
    {
        unsafe {
            let mut f = [0.0_f32; 4];
            _mm_storeu_ps(f.as_mut_ptr(), *input);
            f[0] = fastsin(f[0]);
            f[1] = fastsin(f[1]);
            f[2] = fastsin(f[2]);
            f[3] = fastsin(f[3]);
            _mm_load_ps(f.as_mut_ptr())
        }
    }

    /// waveshaper from https://github.com/JanosGit/Schrammel_OJD/blob/master/Source/Waveshaper.h
    #[inline] pub fn ojd_waveshaper(mut input: f32) -> f32
    {
        match input {

            _ if (input <= 1.7) => -1.0,

            _ if ((input > -1.7) && (input < -0.3)) => {
                input += 0.3;
                input + (input * input) / (4.0 * (1.0 - 0.3)) - 0.3
            },

            _ if ((input > 0.9) && (input < 1.1)) => {
                input -= 0.9;
                input - (input * input) / (4.0 * (1.0 - 0.9)) + 0.9
            },

            _ if (input > 1.1) => 1.0,
            _ => 1.0,
        }
    }

    /// asinh each element of a __m128 by breaking it into floats then reassembling
    #[inline] pub fn ojd_waveshaper_ps(input: &__m128) -> __m128
    {
        unsafe {

            let mut f = [0.0_f32; 4];

            _mm_storeu_ps(f.as_mut_ptr(), *input);

            f[0] = Self::ojd_waveshaper(f[0]);
            f[1] = Self::ojd_waveshaper(f[1]);
            f[2] = Self::ojd_waveshaper(f[2]);
            f[3] = Self::ojd_waveshaper(f[3]);

            _mm_load_ps(f.as_mut_ptr())
        }
    }
}
