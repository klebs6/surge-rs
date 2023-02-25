crate::ix!();

/// This trait can be implemented by structs that
/// are used as waveshapers in the signal path.
/// 
/// Waveshaping is the process of changing the
/// shape of an input signal to create a desired
/// output.
/// 
/// This trait defines a method `shape` that should
/// be implemented by the struct to perform the
/// waveshaping. It is recommended to keep the
/// implementation of the struct lightweight.
///
pub trait Waveshaper {
    fn shape(&self, input: __m128, drive: __m128) -> __m128;
}

/// This function performs bipolar clipping on the
/// input signal.
/// 
/// Clipping is the process of limiting the
/// amplitude of a signal. This function clips the
/// input signal between -1.0 and 1.0. It is
/// recommended to use the AVX instruction set for
/// optimal performance.
///
pub unsafe fn clip_bipolar(y: __m128) -> __m128 {
    use core::arch::x86_64::*;
    let y_min: __m128 = _mm_set1_ps(-1.0);
    let y_max: __m128 = _mm_set1_ps(1.0);
    _mm_max_ps(_mm_min_ps(y, y_max), y_min)
}

/// This trait can be implemented by structs that
/// perform quad filter processing in the signal
/// path.
/// 
/// Quad filters are used to shape the frequency
/// response of a signal by attenuating or
/// amplifying certain frequencies. 
/// 
/// This trait defines a method `process_quad` that
/// should be implemented by the struct to perform
/// quad filter processing. 
/// 
/// It is recommended to keep the implementation of
/// the struct lightweight.
///
pub trait FilterProcessQuad {
    fn process_quad(&self, 
        qfu: &mut QuadFilterUnitState, 
        input: __m128) -> __m128;
}

/// This trait can be implemented by structs that
/// provide coefficients for the CoefficientMaker.
/// 
/// CoefficientMaker is used to generate filter
/// coefficients based on the frequency and
/// resonance parameters.
/// 
/// This trait defines a method `coeff_make` that
/// should be implemented by the struct to provide
/// the filter coefficients.
///
pub trait CoeffMake {
    fn coeff_make(&self,
        freq: f32,
        reso: f32) -> [f32; N_COEFFMAKER_COEFFS];
}

/// This trait can be implemented by structs that
/// perform surge filtering.
/// 
/// Surge filtering is a combination of quad filter
/// processing and coefficient making. 
/// 
/// This trait is a combination of the
/// FilterProcessQuad and CoeffMake traits, and
/// defines no additional methods.
///
pub trait SurgeFilter = FilterProcessQuad + CoeffMake;
