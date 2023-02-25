crate::ix!();

impl CoeffMake for crate::NonlinearStatesFilter {

    /// `freq` is a floating-point frequency value 
    ///
    /// `reso` is a mutable floating-point
    /// resonance value. 
    ///
    /// The function returns an array of
    /// floating-point values with length
    /// `N_COEFFMAKER_COEFFS`.
    ///
    fn coeff_make(&self, freq: f32, mut reso: f32) -> [f32; N_COEFFMAKER_COEFFS] {

        // This line creates a new array `c` of
        // type `f32` and initializes all its
        // elements to 0. `N_COEFFMAKER_COEFFS` is
        // a constant defined elsewhere in the
        // codebase.
        //
        let mut c = [0.0_f32; N_COEFFMAKER_COEFFS];

        // This line clamps the `reso` parameter
        // between 0 and 1 using the
        // `limit_range()` function. 
        //
        // If `reso` is less than 0, it is set to
        // 0. 
        //
        // If it is greater than 1, it is set to
        // 1.
        //
        reso = limit_range(reso, 0.0, 1.0);

        // This line computes the `q` value used
        // in the filter coefficient
        // calculation. `q` is computed as
        // a function of the `reso` parameter
        // using a simple polynomial formula.
        //
        let q: f32 = (reso * reso * reso) * 18.0 + 0.1;

        // This line computes the angular
        // frequency `wc` used in the filter
        // coefficient calculation. `wc` is
        // a function of the frequency `freq`
        // parameter and the sample rate.
        //
        let wc: f32 = 2.0 * PI_32 * self.clamped_frequency(freq) / self.srunit.samplerate_os();

        // These three lines compute the sine and
        // cosine of `wc` using the `fastsin()`
        // and `fastcos()` functions,
        // respectively. `alpha` is computed as
        // a function of `wsin` and `q`.
        //
        let wsin:  f32 = fastsin(wc);
        let wcos:  f32 = fastcos(wc);
        let alpha: f32 = wsin / (2.0 * q);

        // This line computes the reciprocal of
        // `a0`, which is used to normalize the
        // filter coefficients. 
        //
        // Since multiplication is cheaper than
        // division, it is more efficient to store
        // the reciprocal of `a0` instead of `a0`
        // itself.
        //
        let a0r: f32 = 1.0 / (1.0 + alpha);

        // These two lines compute the `A`
        // coefficients of the filter, which
        // determine the feedback part of the
        // filter. `A1` and `A2` coefficients are
        // stored in the array `c`.
        //
        c[C::A1] = -2.0 * wcos * a0r;
        c[C::A2] = (1.0 - alpha) * a0r;

        // The function returns the `c` array,
        // which contains the computed filter
        // coefficients. 
        //
        // The type of filter coefficients depends
        // on the value of `self.ty`, which
        // determines the type of nonlinear states
        // filter.
        // 
        match self.ty {

            // This is a low-pass filter, which
            // allows low-frequency signals to
            // pass through while attenuating
            // high-frequency signals. 
            //
            // The filter coefficients are
            // computed using the equations for
            // a second-order low-pass filter,
            // where `c[C::B0]` is set to half the
            // value of `c[C::B1]`, and `c[C::B2]`
            // is also set to `c[C::B0]`.
            //
            NLSFType::LowPass => {
                c[C::B1] = (1.0 - wcos) * a0r;
                c[C::B0] = c[C::B1] * 0.5;
                c[C::B2] = c[C::B0];
            },

            // This is a high-pass filter, which
            // allows high-frequency signals to
            // pass through while attenuating
            // low-frequency signals. 
            //
            // The filter coefficients are
            // computed using the equations for
            // a second-order high-pass filter,
            // where `c[C::B0]` is set to negative
            // half the value of `c[C::B1]`, and
            // `c[C::B2]` is also set to
            // `c[C::B0]`.
            //
            NLSFType::HighPass => {
                c[C::B1] = -(1.0 + wcos) * a0r;
                c[C::B0] = c[C::B1] * -0.5;
                c[C::B2] = c[C::B0];
            },

            // This is a band-pass filter, which
            // allows signals within a certain
            // frequency range to pass through
            // while attenuating frequencies
            // outside that range. 
            //
            // The filter coefficients are
            // computed using the equations for
            // a second-order band-pass filter,
            // where `c[C::B0]` is set to half the
            // value of `wsin * a0r`, `c[C::B1]`
            // is set to 0, and `c[C::B2]` is set
            // to negative `c[C::B0]`.
            //
            NLSFType::BandPass => {
                c[C::B0] = wsin * 0.5 * a0r;
                c[C::B1] = 0.0;
                c[C::B2] = -c[C::B0];
            },

            // This is a notch filter, which
            // attenuates signals at a certain
            // frequency. 
            //
            // The filter coefficients are
            // computed using the equations for
            // a second-order notch filter, where
            // `c[C::B0]` and `c[C::B2]` are set
            // to `a0r`, and `c[C::B1]` is set to
            // negative twice the value of `wcos
            // * a0r`.
            //
            NLSFType::Notch => {
                c[C::B0] = a0r;
                c[C::B1] = -2.0 * wcos * a0r;
                c[C::B2] = c[C::B0];
            },

            // This is an all-pass filter, which
            // allows all frequencies to pass
            // through but changes the phase of
            // the signal. 
            //
            // The filter coefficients are
            // computed using the equations for
            // a second-order all-pass filter,
            // where `c[C::B0]` is set to
            // `c[C::A2]`, `c[C::B1]` is set to
            // `c[C::A1]`, and `c[C::B2]` is set
            // to 1.0.
            //
            NLSFType::Allpass => {
                c[C::B0] = c[C::A2];
                c[C::B1] = c[C::A1];
                c[C::B2] = 1.0; // (1+a) / (1+a) = 1 (from normalising by a0)
            },
        }

        c

        // returns the `c` array containing the
        // computed filter coefficients.
    }
}
