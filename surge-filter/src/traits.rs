ix!();

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
  structs implementing this trait can provide
  coefficients for CoefficientMaker
  */
pub trait CoeffMake {
    fn coeff_make(&self,
        freq: f32,
        reso: f32) -> [f32; N_COEFFMAKER_COEFFS];
}

pub trait SurgeFilter = FilterProcessQuad + CoeffMake;

