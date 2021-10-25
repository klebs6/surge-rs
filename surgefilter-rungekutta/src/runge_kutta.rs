ix!();

coeffidx![ 
    RungeKuttaCoeff;
    Cutoff,
    Reso,
    GainCompensation
];

pub type C = RungeKuttaCoeff;

pub const RUNGE_KUTTA_EXTRA_OVERSAMPLE:          f32 = 4.0;
pub const RUNGE_KUTTA_EXTRA_OVERSAMPLE_INV:      f32 = 1.0 / 4.0;
pub const RUNGE_KUTTA_DEFAULT_SATURATION:        f32 = 3.0;
pub const RUNGE_KUTTA_DEFAULT_SATURATION_INV:    f32 = 1.0 / 3.0;
pub const RUNGE_KUTTA_DEFAULT_GAIN_COMPENSATION: f32 = 0.666;

/**
 ** This contains various adaptations of the models found at
 **
 ** https://github.com/ddiakopoulos/MoogLadders/blob/master/src/RKSimulationModel.h
 **
 ** Modifications include
 ** 1. Modifying to make surge compatible with state mamagenemt
 ** 2. SSe and so on
 ** 3. Model specici changes per model
 */
pub struct RungeKuttaLadder<'sr> {
    pub tuner:             TunerHandle<'sr>,
    pub srunit:            SampleRateHandle<'sr>,
    pub gain_compensation: Option<f32>,
}

impl RungeKuttaLadder<'sr> {

    pub fn clamped_frequency(&self, pitch: f32) -> f32
    {
        let freq = self.tuner.n2p::<f32,true>( pitch + 69.0 ) * (MIDI_0_FREQ as f32);
        limit_range( freq, 5.0, self.srunit.samplerate_os() * 0.3 )
    }

    #[inline] pub fn clip(
        value:             __m128, 
        saturation:        __m128, 
        saturationinverse: __m128) -> __m128 
    {
        unsafe {
            let minusone: __m128 = _mm_set_ps1(-1.0);
            let one:      __m128 = _mm_set_ps1(1.0); 
            let onethird: __m128 = _mm_set_ps1(1.0 / 3.0);

            let vtsi: __m128 = _mm_mul_ps( 
                value, 
                saturationinverse 
            );

            let v2: __m128 = _mm_min_ps( 
                one, 
                _mm_max_ps( 
                    minusone, 
                    vtsi 
                ) 
            );

            let v23: __m128 = _mm_mul_ps(
                v2, 
                _mm_mul_ps( 
                    v2, 
                    v2 ) 
            );

            let vkern: __m128 = _mm_sub_ps( 
                v2, 
                _mm_mul_ps( 
                    onethird, 
                    v23 ) 
            );

            _mm_mul_ps( 
                saturation, 
                vkern 
            )
        }
    }
}
