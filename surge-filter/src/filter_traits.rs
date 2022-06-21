crate::ix!();

/**
  |structs implementing this trait can be used as
  |waveshapers in our signal path
  |
  |it is best to keep the structs themselves
  |lightweight
  */
pub trait Waveshaper {
    fn shape(&self, input: __m128, drive: __m128) -> __m128;
}

pub unsafe fn clip_bipolar(y: __m128) -> __m128 {
    use core::arch::x86_64::*;
    let y_min: __m128 = _mm_set1_ps(-1.0);
    let y_max: __m128 = _mm_set1_ps(1.0);
    _mm_max_ps(_mm_min_ps(y, y_max), y_min)
}

/**
  |structs implementing this trait can perform
  |quad filter processing in our signal path
  |
  |it is best to keep the structs themselves
  |lightweight
  */
pub trait FilterProcessQuad {
    fn process_quad(&self, 
        qfu: &mut QuadFilterUnitState, 
        input: __m128) -> __m128;
}

/**
  | structs implementing this trait can
  | provide coefficients for CoefficientMaker
  |
  */
pub trait CoeffMake {
    fn coeff_make(&self,
        freq: f32,
        reso: f32) -> [f32; N_COEFFMAKER_COEFFS];
}

pub trait SurgeFilter = FilterProcessQuad + CoeffMake;
