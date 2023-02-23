crate::ix!();

pub trait BiquadSetCoeffs {

    /// used to set the coefficients of a biquad
    /// filter. The function takes six arguments,
    /// `a0`, `a1`, `a2`, `b0`, `b1`, and `b2`, which
    /// are the coefficients of the filter transfer
    /// function.
    ///
    fn set_coef(&mut self, 
        a0: f64, 
        a1: f64, 
        a2: f64, 
        b0: f64, 
        b1: f64, 
        b2: f64);
}

/// This trait defines three functions for
/// calculating the filter cutoff frequency and
/// resonance:
/// 
pub trait BiquadCalcFreq {

    /// This function takes a cutoff frequency in
    /// radians per sample (`scfreq`) and returns
    /// the corresponding angular frequency.
    ///
    fn calc_omega(&self, scfreq: f64) -> f64;

    /// This function takes a cutoff frequency in
    /// Hertz and returns the corresponding
    /// angular frequency.
    ///
    fn calc_omega_from_hertz(&self, hertz: f64) -> f64;

    /// `calc_v1_q`: This function takes
    /// a resonance parameter (`reso`) and returns
    /// the corresponding Q-factor.
    ///
    fn calc_v1_q(&self, reso: f64) -> f64;
}

/// This trait defines two functions for
/// calculating the coefficients of a low-pass
/// filter:
/// 
pub trait BiquadCoeffLP {

    /// Sets the filter coefficients to produce
    /// a low-pass filter with the specified
    /// cutoff frequency and resonance.
    ///
    fn coeff_lp(&mut self, omega: f64, quality_factor: f64);

    /// Sets the filter coefficients for a given
    /// cutoff frequency and bandwidth
    ///
    fn coeff_lp_with_bw(&mut self, omega: f64, bandwidth: f64);
}

/// This trait defines a function for calculating
/// the coefficients of a second-order low-pass
/// filter:
/// 
pub trait BiquadCoeffLP2B {

    /// This function takes an angular frequency
    /// (`omega`) and a quality factor
    /// (`quality_factor`) and sets the filter
    /// coefficients to produce a second-order
    /// low-pass filter with the specified cutoff
    /// frequency and resonance.
    ///
    fn coeff_lp2b(&mut self, omega: f64, quality_factor: f64);
}

/// This trait defines a function for calculating
/// the coefficients of a band-pass filter:
/// 
pub trait BiquadCoeffBP {

    /// This function takes an angular frequency
    /// (`omega`) and a quality factor
    /// (`quality_factor`) and sets the filter
    /// coefficients to produce a band-pass filter
    /// with the specified center frequency and
    /// resonance.
    ///
    fn coeff_bp(&mut self, omega: f64, quality_factor: f64);
}

/// This trait defines a function for calculating
/// the coefficients of a second-order band-pass
/// filter:
/// 
pub trait BiquadCoeffBP2A {

    /// This function takes an angular frequency
    /// (`omega`) and a quality factor
    /// (`quality_factor`) and sets the filter
    /// coefficients to produce a second-order
    /// band-pass filter with the specified center
    /// frequency and resonance.
    ///
    fn coeff_bp2a(&mut self, omega: f64, quality_factor: f64);
}

/// This trait defines a function for calculating
/// the coefficients of a peaking EQ filter:
/// 
pub trait BiquadCoeffPKA {

    /// This function takes an angular frequency
    /// (`omega`), a quality factor
    /// (`quality_factor`), and a gain (`gain`)
    /// and sets the filter coefficients to
    /// produce a peaking EQ filter with the
    /// specified center frequency, bandwidth, and
    /// gain.
    ///
    fn coeff_pka(&mut self, omega: f64, quality_factor: f64);
}

pub trait BiquadCoeffEQ {

    /// takes in a radian frequency `omega`, bandwidth
    /// `bandwidth`, and gain `gain` as `f64` and sets
    /// the coefficients of an equalizer filter in the
    /// implementing struct.
    ///
    fn coeff_peak_eq(&mut self, 
        omega: f64, 
        bandwidth: f64, 
        gain: f64);
}

/// A trait defining a function `coeff_apf` to set
/// the coefficients of an all-pass filter. The
/// all-pass filter is a type of IIR filter where
/// all frequencies are passed through, but with
/// a phase shift. This trait is used for creating
/// and modifying all-pass filters.
///
pub trait BiquadCoeffAPF {

    /// Calculates the coefficients of a biquad
    /// all-pass filter based on the given `omega`
    /// and `quality_factor` parameters.
    ///
    /// If `omega` is not within the range of 0 to
    /// pi, the filter coefficients are set to
    /// a simple identity filter.
    ///
    /// # Arguments
    ///
    /// * `omega` - Angular frequency of the filter (in radians)
    /// * `quality_factor` - Quality factor of the filter
    ///
    /// use my_crate::BiquadFilter;
    ///
    /// let mut filter = BiquadFilter::new();
    ///
    /// filter.coeff_apf(0.5, 0.7);
    /// ```
    ///
    fn coeff_apf(&mut self, omega: f64, quality_factor: f64);
}

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

/// A trait defining two functions, `apply` and
/// `reset`, for applying the filter to an input
/// value and resetting the filter's internal
/// state, respectively. This trait is used for
/// applying filters to input values.
///
pub trait BiquadFilterApply {
    fn apply(&mut self, x: f64) -> f64;
    fn reset(&mut self);
}

/// This trait is used for getting the
/// state of a filter.
///
pub trait BiquadGetState {

    /// returns a `BiquadFilterState` struct
    /// representing the internal state of
    /// a filter. 
    fn get_state(&self) -> BiquadFilterState;
}

/// This trait is used for setting the state of
/// a filter.
///
pub trait BiquadSetState {

    /// sets the internal state of a filter to
    /// a given `BiquadFilterState` struct. 
    ///
    fn set_state(&mut self, state: &BiquadFilterState);
}

pub trait BiquadCascade {

    /// takes a mutable reference to another
    /// filter implementing the
    /// `BiquadFilterApply` trait and returns
    /// a mutable reference to itself. 
    ///
    /// This function is used for cascading two
    /// filters together.
    ///
    fn cascade(&mut self, other: &mut dyn BiquadFilterApply) -> &mut Self;
}
