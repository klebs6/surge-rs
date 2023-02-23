crate::ix!();

/// This is for processing a stereo pair of
/// samples without any latency
/// (`ProcessSampleStereoNolag`). 
///
/// It takes in two mutable references to `f32`
/// values (`l` and `r`) representing the left and
/// right channel input samples, respectively, and
/// two additional mutable references to `f32`
/// values (`l_out` and `r_out`) representing the
/// left and right channel output samples,
/// respectively. 
///
/// The function performs the same filter
/// operation as `ProcessSampleNolag`, but writes
/// the filtered samples to the output references
/// instead of overwriting the input references.
///
impl ProcessSampleStereoNolag for BiquadFilter {

    #[inline] fn process_sample_stereo_nolag(&mut self, 
        l: &mut f32, r: &mut f32, l_out: &mut f32, r_out: &mut f32)
    {
        // This line calculates the first
        // operation in the biquad filter, using
        // the current input sample `*l` and the
        // filter coefficients `self.b0.v[0]` and
        // `self.reg0[0]`. 
        //
        // The result is stored in a mutable
        // variable `op`, which is initially set
        // to 0.0.
        //
        let mut op = *l as f64 * self.b0.v[0] + self.reg0[0];

        // This line updates the first register
        // `self.reg0[0]` by calculating the
        // second operation in the biquad filter,
        // using the current input sample `*l`,
        // the filter coefficients `self.b1.v[0]`
        // and `self.a1.v[0]`, and the
        // intermediate result `op`. The result is
        // then added to the third register
        // `self.reg1[0]`.
        //
        self.reg0[0] = *l as f64 * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];

        // This line updates the second register
        // `self.reg1[0]` by calculating the third
        // operation in the biquad filter, using
        // the current input sample `*l`, the
        // filter coefficients `self.b2.v[0]` and
        // `self.a2.v[0]`, and the intermediate
        // result `op`.
        //
        self.reg1[0] = *l as f64 * self.b2.v[0] - self.a2.v[0] * op;

        // This line writes the result of the
        // first operation to the output sample
        // reference `*l_out`, casting the
        // intermediate result `op` from a `f64`
        // to a `f32`.
        //
        *l_out = op as f32;

        // This line initializes `op` with the
        // value of the right channel input sample
        // (`*r`) multiplied by the first
        // coefficient of the feedforward filter
        // (`self.b0.v[0]`), and adds to it the
        // second value of the register for the
        // feedback filter (`self.reg0[1]`).
        //
        op = *r as f64 * self.b0.v[0] + self.reg0[1];

        // This line calculates the first value of
        // the register for the feedback filter
        // (`self.reg0[1]`) for the right channel,
        // using the same calculations as for the
        // left channel.
        //
        self.reg0[1] = *r as f64 * self.b1.v[0] - self.a1.v[0] * op + self.reg1[1];

        // This line calculates the second value
        // of the register for the feedback filter
        // (`self.reg1[1]`) for the right channel,
        // using the same calculations as for the
        // left channel.
        //
        self.reg1[1] = *r as f64 * self.b2.v[0] - self.a2.v[0] * op;

        // This line assigns the value of `op` to
        // the output variable for the right
        // channel (`*r_out`), casting it back to
        // a `f32` type.
        *r_out = op as f32;
    }
}
