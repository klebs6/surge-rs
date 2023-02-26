/*!
  | possibly unrelated but awesome:
  |
  | A Butterworth filter is a type of low-pass filter designed to have a frequency response as flat
  | as possible in the passband and a sharp cutoff in the stopband. This makes it ideal for
  | applications where the filter is required to preserve the amplitude of the input signal in the
  | passband and attenuate frequencies above the cutoff frequency as much as possible.
  |
  | The frequency response of a Butterworth filter is characterized by its order, which determines
  | the steepness of the cutoff. The higher the order, the steeper the cutoff. The cutoff frequency
  | is the point at which the filter starts to attenuate the signal, and it is usually defined as
  | the -3dB frequency, where the amplitude of the signal is reduced by 3dB compared to the
  | passband.
  |
  | The Butterworth filter is special because it has a maximally flat frequency response in the
  | passband, which means that there is no ripple in the passband and the group delay is
  | minimized. This results in a filter that preserves the shape of the input signal as much as
  | possible in the passband.
  |
  | The relevant equations for designing a Butterworth filter are derived from the transfer function
  | of an analog filter using the bilinear transform or the impulse invariance method to convert it
  | to a digital filter. The transfer function of an analog Butterworth filter of order N is given
  | by:
  |
  | H(s) = 1 / (1 + (s / ω_c)^2N)^0.5
  |
  | where ω_c is the cutoff frequency in radians per second, s is the Laplace variable, and N is the
  | order of the filter. To convert this to a digital filter, the bilinear transform or impulse
  | invariance method is used to map the s-plane to the z-plane.
  |
  | The bilinear transform is given by:
  |
  | s = 2*(z - 1) / (z + 1)
  |
  | Substituting this into the transfer function of the analog Butterworth filter and simplifying
  | yields the transfer function of the digital Butterworth filter:
  |
  | H0(z)   = b0 + b1*z^-1 + b2*z^-2 + ... + bN*z^-N
  | H1(z) = a0 + a1*z^-1 + a2*z^-2 + ... + aN*z^-N
  |
  | H(z) = H0(z) / H1(z)
  |
  | where
  |
  | b0 = bN = (ω_c)^N / (2^N * H(s=jω_c))
  |
  | bi = (ω_c)^N-i / (2^N * H(s=jω_c)*prod(j=1 to N, j!=i)((s - s_j) / (s_j - jω_c))),
  | for i = 1, ..., N-1
  |
  | and
  |
  | a0 = 1
  |
  | ai = i^N / (2^N * H(s=jω_c)*prod(j=1 to N)(s_j - jω_c)),
  | for i = 1, ..., N
  |
  | where s_j = -jω_c * exp(jπ(2j + N - 1)/(2N)),
  | for j = 1, ..., N.
  |
  | The resulting digital Butterworth filter has a frequency response that approximates the analog
  | Butterworth filter, but with a slight deviation due to the mapping of the s-plane to the
  | z-plane. The deviation can be minimized by increasing the sampling rate or increasing the filter
  | order.
  |
  */
crate::ix!();

impl BiquadCoeffLP2B for BiquadFilter {

    /// This method takes in two arguments: `omega`, which is the angular frequency of the desired
    /// filter, and `quality_factor`, which is the quality factor of the filter. 
    ///
    /// It is implemented for the `BiquadFilter` struct and sets its coefficients for a 2nd-order
    /// low-pass filter with a Butterworth response.
    ///
    fn coeff_lp2b(&mut self, omega: f64, quality_factor: f64)
    {
        if omega > PI {

            // If the input frequency is greater than PI (half the sampling frequency), the filter
            // is simply a constant gain of
            // 1.0, so the coefficients are set accordingly.
            //
            self.set_coef(
                1.0, 
                0.0, 
                0.0, 
                1.0, 
                0.0, 
                0.0
            );

        } else {

            // Otherwise, we square the input frequency `omega` to use in subsequent calculations.
            //
            let w_sq:  f64 = omega * omega;

            // Next, we calculate a denominator term `den` which is used to calculate the gain `g1`
            // and the filter coefficients. `den` is the denominator of the transfer function,
            // which is derived using the Butterworth filter design formula.
            //
            let den:   f64 = 
                (w_sq * w_sq) 
                + (PI * PI * PI * PI) 
                + w_sq * (PI * PI) * (1.0 / quality_factor - 2.0);

            // Using `den`, we calculate the gain `g1` for the filter. 
            //
            // The `std::cmp::min` function is used to ensure that the gain is not greater than
            // 1.0, which could cause the filter to become unstable. 
            //
            // The gain formula is derived from the Butterworth filter design formula.
            //
            let g1:    f64 = std::cmp::min(
                FloatOrd(1.0), 
                FloatOrd(((w_sq * w_sq) / den).sqrt() * 0.5)
            ).0;

            // Using the gain `g1`, `omega`, and `quality_factor`, we calculate the filter
            // coefficients for a 2nd-order low-pass filter with a Butterworth response. 
            //
            // These coefficients are derived using the Butterworth filter design formula.
            //
            let cosi:  f64 = omega.cos();
            let sinu:  f64 = omega.sin();
            let alpha: f64 = sinu / (2.0 * quality_factor);
            let a:     f64 = 2.0 * g1.sqrt() * (2.0 - g1).sqrt();
            let b0:    f64 = (1.0 - cosi + g1 * (1.0 + cosi) + a * sinu) * 0.5;
            let b1:    f64 = 1.0 - cosi - g1 * (1.0 + cosi);
            let b2:    f64 = (1.0 - cosi + g1 * (1.0 + cosi) - a * sinu) * 0.5;
            let a0:    f64 = 1.0 + alpha; 
            let a1:    f64 = -2.0 * cosi;
            let a2:    f64 = 1.0 - alpha;

            self.set_coef(a0, a1, a2, b0, b1, b2);
        }
    }
}
