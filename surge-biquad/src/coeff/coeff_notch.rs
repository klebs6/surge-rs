/// A notch filter is a type of filter used to
/// attenuate a narrow band of frequencies around
/// a center frequency while allowing other
/// frequencies to pass through unaffected. It is
/// also called a band-stop filter or reject
/// filter. The attenuation at the center
/// frequency is typically very high, often around
/// 60 dB or more.
/// 
/// The transfer function of a second-order notch
/// filter can be written as:
/// 
/// ```
/// H(s) = (s^2 + w0^2) / (s^2 + alpha*s + w0^2)
/// ```
/// 
/// where `w0` is the center frequency of the
/// notch, `alpha` is a damping factor, and `s` is
/// the Laplace variable.
/// 
/// In practice, this transfer function is usually
/// implemented as a difference equation, or a set
/// of coefficients that define a difference
/// equation. A second-order notch filter can be
/// implemented using a biquad filter, which is
/// a type of IIR filter that uses two poles and
/// two zeros.
/// 
/// The difference equation for a second-order
/// biquad notch filter is:
/// 
/// ```
/// y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2] - a1*y[n-1] - a2*y[n-2]
/// ```
/// 
/// where `x[n]` and `y[n]` are the input and
/// output samples at time `n`, respectively, and
/// `b0`, `b1`, `b2`, `a1`, and `a2` are the
/// filter coefficients.
/// 
/// The coefficients for a biquad notch filter can
/// be calculated from the center frequency `w0`
/// and damping factor `alpha` using the following
/// formulas:
/// 
/// ```
/// b0 = 1
/// b1 = -2*cos(w0)
/// b2 = 1
/// a0 = 1 + alpha
/// a1 = -2*cos(w0)
/// a2 = 1 - alpha
/// ```
/// 
/// where `cos(w0)` is the cosine of the center
/// frequency `w0` and is given by:
/// 
/// ```
/// cos(w0) = (1 - tan^2(w0/2)) / (1 + tan^2(w0/2))
/// ```
/// 
/// where `tan(w0/2)` is the tangent of half the center frequency.
/// 
/// In summary, a notch filter is a filter that
/// attenuates a narrow band of frequencies around
/// a center frequency. It can be implemented as
/// a biquad filter using a set of difference
/// equations, or as a set of coefficients that
/// define the difference equation. The center
/// frequency and damping factor of the notch are
/// used to calculate the filter coefficients.
///
crate::ix!();

/// Implements the BiquadCoeffNotch trait for the
/// BiquadFilter struct.
///
impl BiquadCoeffNotch for BiquadFilter {

    /// Calculates the filter coefficients for
    /// a notch filter with the given `omega` and
    /// `qq`.
    ///
    /// # Arguments
    ///
    /// * `omega` - The frequency at which the
    /// notch occurs, in radians per second.
    ///
    /// * `qq` - The quality factor of the filter,
    /// which determines the width of the notch.
    ///
    fn set_notch_filter_coefficients(&mut self, omega: f64, qq: f64)
    {
        // If omega is greater than PI, set the
        // filter coefficients to a bypass state
        //
        // The value of pi represents the
        // frequency of half of the sampling rate,
        // which is known as the Nyquist
        // frequency. The Nyquist frequency is the
        // maximum frequency that can be
        // represented accurately in a digital
        // signal, and it is equal to half of the
        // sampling rate. Therefore, pi is equal
        // to the Nyquist frequency, but it is not
        // equal to the sampling rate itself.
        if omega > PI {

            // This line sets the coefficients of
            // the filter to bypass the filter. 
            //
            // calls the `set_coef` method of the
            // `BiquadFilter` struct and passes in
            // the six coefficient values as
            // arguments.
            //
            self.set_coef(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        } else {

            // If `omega` is less than or equal to
            // `PI`, the function continues with
            // the notch filter calculations.
            //
            // These lines calculate the
            // coefficients of the notch filter
            // using the input values `omega` and
            // `qq`. 

            // `cosi` and `sinu`
            // are the cosine and sine of `omega`,
            // respectively. 
            let cosi:  f64 = omega.cos();
            let sinu:  f64 = omega.sin();

            // `reso` is a range-limited version
            // of `qq` that is clamped between
            // `0.0` and `1.0`. 
            //
            // Note: on `limit_range`, it is not
            // clear what the range of the `reso`
            // parameter is, so it would be good
            // to add documentation to clarify
            // that.
            //
            let reso:  f64 = limit_range(qq as f32, 0.0_f32, 1.0_f32) as f64 ;

            // `q` is the quality factor of the
            // filter, calculated based on `reso`. 
            let q:     f64 = 1.0 / (0.02 + 30.0 * reso * reso);

            // `alpha` is a value used in the
            // calculation of the filter
            // coefficients. 
            let alpha: f64 = sinu / (2.0 * q);

            // `b0`, `b1`, `b2`, `a0`, `a1`, and
            // `a2` are the six coefficients of
            // the filter.
            let b0:    f64 = 1.0;
            let b1:    f64 = -2.0 * cosi;
            let b2:    f64 = 1.0;
            let a0:    f64 = 1.0 + alpha;
            let a1:    f64 = -2.0 * cosi;
            let a2:    f64 = 1.0 - alpha;

            // This line sets the coefficients of
            // the filter. It calls the `set_coef`
            // method of the `BiquadFilter` struct
            // and passes in the six coefficient
            // values as arguments.
            //
            self.set_coef(a0, a1, a2, b0, b1, b2);
        }
    }
}
