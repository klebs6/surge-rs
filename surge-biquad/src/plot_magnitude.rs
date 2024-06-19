crate::ix!();

impl PlotMagnitude for BiquadFilter {

    /// The `plot_magnitude` function takes a frequency `freq` as input and returns the magnitude
    /// of the filter's frequency response at that frequency.
    ///
    fn plot_magnitude(&self, freq: f32) -> f32 {

        // The first few lines of the function define some variables and constants needed for the
        // calculation. `ca0`, `ca1`, `ca2`, `cb0`, `cb1`, and `cb2` are all complex numbers that
        // represent the coefficients of the filter transfer function. `i` is a complex number with
        // a value of `0 + 1i`, which is used in the calculation of `z`.
        //
        let ca0 = Complex64::new(1.0, 0.0);
        let ca1 = Complex64::new(self.a1.v[0], 0.0);
        let ca2 = Complex64::new(self.a2.v[0], 0.0);
        let cb0 = Complex64::new(self.b0.v[0], 0.0);
        let cb1 = Complex64::new(self.b1.v[0], 0.0);
        let cb2 = Complex64::new(self.b2.v[0], 0.0);
        let i   = Complex64::new(0.0, 1.0);

        // `z` is a complex number that represents the frequency response of the filter at the
        // given frequency `freq`. It is calculated by multiplying `-2 * PI * freq * i`, where
        // `freq` is the frequency in Hz, and then taking the exponential function of that
        // product. This calculation is done in radians per second, which is why the frequency is
        // multiplied by `2 * PI`.
        //
        let z: Complex64 = (-2.0 * PI * (freq as f64) * i).exp();

        // `h` is a complex number that represents the filter's frequency response at the given
        // frequency `freq`. It is calculated by evaluating the filter transfer function at
        // `z`. Specifically, `h` is the ratio of the output complex voltage to the input complex
        // voltage. The numerator of this ratio is the sum of three terms: `cb0`, which is the
        // constant term of the numerator polynomial, and `cb1` and `cb2`, which are the
        // coefficients of the first and second order terms of the numerator polynomial,
        // respectively. The denominator of the ratio is the sum of three terms as well: `ca0`,
        // which is the constant term of the denominator polynomial, and `ca1` and `ca2`, which are
        // the coefficients of the first and second order terms of the denominator polynomial,
        // respectively.
        //
        let h: Complex64 = (cb0 + cb1 * z + cb2 * z * z) / (ca0 + ca1 * z + ca2 * z * z);

        // Finally, the function computes the magnitude of `h` using the `to_polar()` method of the
        // `Complex64` struct. This method returns a tuple containing the magnitude and phase of
        // the complex number. In this case, we only care about the magnitude, so we take the first
        // element of the tuple (the magnitude), convert it to a `f32`, and return it as the
        // function's output.
        //
        let r: f64 = h.to_polar().0;

        r as f32
    }
}
