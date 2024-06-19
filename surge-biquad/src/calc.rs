crate::ix!();

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

impl BiquadCalcFreq for BiquadFilter {

    /// This is a method implementation for the `calc_omega` function, which takes a reference to
    /// the `self` instance of `BiquadFilter` and a `scfreq` parameter of type `f64`. The function
    /// returns an `f64` value. 
    ///
    /// This function calculates the angular frequency of a given frequency `scfreq` using the
    /// `tuner` and `srunit` fields of the `BiquadFilter` struct.
    ///
    fn calc_omega(&self, scfreq: f64) -> f64 {
        (2.0 * PI) 
            * CONCERT_A_HZ
            * self.tuner.n2p::<f64,true>((12.0 * scfreq) as f64) 
            * self.srunit.dsamplerate_inv() 
    }

    /// This is another method implementation for the `calc_omega_from_hertz` function, which takes
    /// a reference to the `self` instance of `BiquadFilter` and a `hertz` parameter of type
    /// `f64`. The function returns an `f64` value. This function calculates the angular frequency
    /// of a given frequency `hertz` using the `srunit` field of the `BiquadFilter` struct.
    ///
    fn calc_omega_from_hertz(&self, hertz: f64) -> f64 {
        (2.0 * PI) 
            * hertz * self.srunit.dsamplerate_inv() 
    }

    /// This is another method implementation for the `calc_v1_q` function, which takes a reference
    /// to the `self` instance of `BiquadFilter` and a `reso` parameter of type `f64`. The function
    /// returns an `f64` value. This function calculates the Q factor of a given resonance `reso`.
    ///
    fn calc_v1_q(&self, reso: f64) -> f64 {
        1.0 / (1.02 - limit_range(reso as f32, 0.0_f32, 1.0_f32) as f64)
    }
}
