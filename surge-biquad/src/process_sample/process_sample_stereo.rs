crate::ix!();

impl ProcessSampleStereo for BiquadFilter {

    /// This defines a method named `process_sample_stereo` which takes a mutable reference to
    /// `self` (the instance being operated on), two `f32` values `l` and `r` representing the left
    /// and right channel input samples, and two mutable references to `f32` values `l_out` and
    /// `r_out` representing the left and right channel output samples, respectively. 
    ///
    #[inline] fn process_sample_stereo(&mut self, 
        l: f32, r: f32, l_out: &mut f32, r_out: &mut f32)
    {
        // This method call is likely defined on the `BiquadFilter` type, but it's not shown in the
        // code snippet. It's probably used to calculate filter coefficients or update internal
        // state, but without the implementation of the `coeff_process` method it's hard to say
        // exactly what it does.
        //
        self.coeff_process();

        // This creates a mutable variable named `input` of type `f64` and initializes it with the
        // value of `l` converted to a `f64`. This conversion is necessary because the rest of the
        // filter calculations use `f64` values.
        //
        let mut input: f64 = l as f64;

        // This performs the filter calculation for the left channel input sample. It multiplies
        // the `input` value by the filter coefficient `b0` and adds the contents of the `reg0`
        // register. The result is stored in a mutable variable named `op`.
        //
        let mut op = input * self.b0.v[0] + self.reg0[0];

        // This updates the contents of the `reg0` register with the filtered value for the left
        // channel input sample. It multiplies the `input` value by the `b1` coefficient, subtracts
        // the product of the `a1` coefficient and `op`, and adds the contents of the `reg1`
        // register. The result is stored in the first element of the `reg0` array.
        //
        self.reg0[0] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];

        // This updates the contents of the `reg1` register for the left channel input sample. It
        // multiplies the `input` value by the `b2` coefficient and subtracts the product of the
        // `a2` coefficient and `op`. The result is stored in the first element of the `reg1`
        // array.
        //
        self.reg1[0] = input * self.b2.v[0] - self.a2.v[0] * op;

        // This assigns the filtered value for the left channel input sample, converted back to an
        // `f32`, to the `l_out` output reference.
        //
        *l_out = op as f32;

        // This updates the `input` variable with the value of the right channel input sample,
        // converted to a `f64`.
        //
        input = r as f64;

        // calculates the intermediate result of the filter operation for the right channel input
        // sample. The first biquad filter coefficient and the first element of the `reg0` array
        // are used to perform the calculation.
        //
        op = input * self.b0.v[0] + self.reg0[1];

        // calculates the new value of the second element of the `reg0` array using the previous
        // intermediate result `op`, the first and second biquad filter coefficients, and the
        // second element of the `reg1` array.
        //
        self.reg0[1] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[1];

        // calculates the new value of the second element of the `reg1` array using the previous
        // intermediate result `op` and the third biquad filter coefficient.
        //
        self.reg1[1] = input * self.b2.v[0] - self.a2.v[0] * op;

        // sets the value of the `r_out` output reference to the final result of the filter
        // operation for the right channel input sample, which is the converted `op` value.
        //
        *r_out = op as f32;
    }
}
