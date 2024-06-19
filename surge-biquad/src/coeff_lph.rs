/*!
  | This code implements the `BiquadCoeffLPHMorph`
  | trait for the `BiquadFilter` type.
  |
  | The `BiquadCoeffLPHMorph` trait is
  | a collection of methods that provide
  | coefficients for a biquad filter, which is
  | a type of digital filter commonly used in
  | audio processing.
  */

crate::ix!();

/// A trait defining a function `coeff_lph_morph`
/// to set the coefficients of
/// a low-pass/high-pass filter morph. 
///
/// This morph is a type of filter that gradually
/// morphs from a low-pass filter to a high-pass
/// filter as a morphing parameter is
/// changed. This trait is used for creating and
/// modifying low-pass/high-pass filter morphs.
///
pub trait BiquadCoeffLPHMorph {

    fn coeff_lph_morph(
        &mut self, 
        omega:          f64, 
        quality_factor: f64, 
        morph:          f64);
}

/// Implements the `BiquadCoeffLPHMorph` trait for
/// the `BiquadFilter` struct, providing a method
/// for computing the coefficients for
/// a low-pass/high-pass morph filter.
///
/// The method `coeff_lph_morph` takes in the
/// angular frequency `omega`, the quality factor
/// `quality_factor`, and a morph parameter
/// `_morph` (which is currently unused). It
/// computes the coefficients for
/// a low-pass/high-pass morph filter using the
/// following formulae:
///
///     // b0 = alpha
///     // b1 = 0
///     // b2 = -alpha
///     // a0 = 1 + alpha
///     // a1 = -2*cos(omega)
///     // a2 = 1 - alpha
///
/// where `alpha` is defined as `sin(omega) / (2 * quality_factor)`.
///
impl BiquadCoeffLPHMorph for BiquadFilter {

    /// Computes the coefficients for
    /// a low-pass/high-pass morph filter and sets
    /// them on the `BiquadFilter` instance.
    ///
    /// # Arguments
    ///
    /// * `omega`: The angular frequency of the
    /// filter, in radians per second.
    ///
    /// * `quality_factor`: The quality factor of
    /// the filter, which controls the shape of
    /// the filter's frequency response.
    ///
    /// * `_morph`: A morph parameter that is
    /// currently unused.
    ///
    ///--------------------------------
    ///
    fn coeff_lph_morph(&mut self, 
        omega: f64, quality_factor: f64, _morph: f64)
    {
        // The first block of code is commented
        // out, so it is not executed. It appears
        // to be calculating the high-pass and
        // low-pass coefficients based on the
        // value of the morph parameter, but since
        // it is commented out, it is not actually
        // used in the implementation.

        /*
        let mut HP: f64 = limit_range(morph as f32, 0.0_f32, 1.0_f32) as f64; 
        let mut LP: f64 = 1.0 - HP; 
        let mut BP: f64 = LP * HP;

        HP *= HP;
        LP *= LP;
        */

        // The next block of code calculates the
        // coefficients for the biquad filter. The
        // `cosi` and `sinu` variables represent
        // the cosine and sine of `omega`,
        // respectively. The `alpha` variable
        // represents the filter's alpha value,
        // which is used in the calculation of the
        // filter coefficients.
        let cosi:  f64 = omega.cos();
        let sinu:  f64 = omega.sin();
        let alpha: f64 = sinu / (2.0 * quality_factor);

        // The `b0`, `b1`, and `b2` variables
        // represent the feedforward coefficients
        // of the filter, while the `a0`, `a1`,
        // and `a2` variables represent the
        // feedback coefficients of the filter.
        //
        let b0:    f64 = alpha;
        let b1:    f64 = 0.0;
        let b2:    f64 = -alpha;
        let a0:    f64 = 1.0 + alpha;
        let a1:    f64 = -2.0 * cosi;
        let a2:    f64 = 1.0 - alpha;

        // Finally, the `set_coef` method is called on
        // `self` (which is a `BiquadFilter` instance) to
        // set the filter coefficients to the values
        // calculated in the previous step.
        // 
        // In summary, this code calculates the
        // coefficients for a biquad filter based on the
        // input parameters, and sets those coefficients
        // on a `BiquadFilter` instance.
        self.set_coef(a0, a1, a2, b0, b1, b2);
    }
}
