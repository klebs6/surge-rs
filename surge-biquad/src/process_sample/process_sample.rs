crate::ix!();

impl ProcessSample for BiquadFilter {

    #[inline] fn process_sample(&mut self, input: f32) -> f32 
    {
        // This line is calling the
        // `coeff_process` method on `self`, which
        // is a method that updates the internal
        // filter coefficients based on the
        // current filter parameters. 
        //
        // This is necessary because the filter
        // coefficients can change whenever the
        // filter parameters (such as cutoff
        // frequency or resonance) are adjusted.
        //
        self.coeff_process();

        // This line computes the current output
        // value `op` based on the current input
        // value `input`, the first filter
        // coefficient `self.b0.v[0]`, and the
        // current contents of the `self.reg0`
        // buffer. 
        //
        // The input value is first cast to an
        // `f64` value to allow for more precise
        // calculations. 
        //
        // The multiplication of the input value
        // and the first coefficient is then added
        // to the current value in the `self.reg0`
        // buffer to compute the current output
        // value.
        //
        let op = (input as f64) * self.b0.v[0] + self.reg0[0] ;

        // This line updates the contents of the
        // `self.reg0` buffer based on the current
        // input value `input`, the second and
        // third filter coefficients
        // `self.b1.v[0]` and `self.a1.v[0]`, and
        // the current value of the `op` variable
        // computed in the previous line. 
        //
        // The new value of `self.reg0[0]` is
        // computed by multiplying the input value
        // by the second coefficient, subtracting
        // the product of the third coefficient
        // and the current output value `op`, and
        // adding the current value of the
        // `self.reg1` buffer.
        //
        self.reg0[0] = input as f64 * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];

        // This line updates the contents of the
        // `self.reg1` buffer based on the current
        // input value `input
        //
        self.reg1[0] = input as f64 * self.b2.v[0] - self.a2.v[0] * op;

        op as f32
    }
}
