crate::ix!();

/// The third implementation is for processing
/// a single sample without any input
/// (`ProcessSampleNolagNoinput`). 
///
/// It takes in two mutable references to `f32`
/// values (`l_out` and `r_out`) representing the
/// left and right channel output samples,
/// respectively. 
///
/// This function updates the internal state of
/// the filter, but does not read any input
/// samples. 
///
/// It writes the resulting filtered samples to
/// the output references.
///
impl ProcessSampleNolagNoinput for BiquadFilter {

    /// This method takes two mutable references
    /// to `f32` values, which represent the left
    /// and right output channels of the filter,
    /// and updates them with the processed audio
    /// samples.
    ///
    /// Overall, these methods are used to apply
    /// a biquad filter to an audio stream. The
    /// input samples are processed through the
    /// filter's transfer function, which is
    /// defined by its coefficients
    /// (`self.b0.v[0]`, `self.b1.v[0]`,
    /// `self.b2.v[0]`, `self.a1.v[0]`, and
    /// `self.a2.v[0]`). 
    ///
    /// The processed samples are then output to
    /// the left and right channels. The filter's
    /// state is preserved between calls using two
    /// registers (`self.reg0`, `self.reg1` )
    ///
    #[inline] fn process_sample_nolag_noinput(&mut self, 
        l_out: &mut f32, r_out: &mut f32)
    {
        // The `op` variable is assigned the value
        // of the first register value
        // (`self.reg0[0]`), which corresponds to
        // the first input sample in the previous
        // block.

        let mut op = self.reg0[0];

        // This line calculates the first register
        // value (`self.reg0[0]`) of the current
        // block based on the previous output
        // (`op`) and the filter coefficients
        // (`self.a1.v[0]` and `self.reg1[0]`). 
        //
        // This value is then updated in
        // `self.reg0[0]`.
        //
        self.reg0[0] = -self.a1.v[0] * op + self.reg1[0];

        // This line calculates the second
        // register value (`self.reg1[0]`) of the
        // current block based on the previous
        // output (`op`) and the filter
        // coefficient (`self.a2.v[0]`). This
        // value is then updated in
        // `self.reg1[0]`.
        //
        self.reg1[0] = -self.a2.v[0] * op;

        // This line assigns the processed audio
        // sample (`op`) to the left output
        // channel (`*l_out`) after converting it
        // to an `f32` type.
        //
        *l_out = op as f32;

        // This line is similar to the first line,
        // but it assigns the second register
        // value (`self.reg0[1]`) to `op`.
        //
        op = self.reg0[1];

        // This line is similar to the second
        // line, but it updates the second
        // register value (`self.reg0[1]`) based
        // on the previous output (`op`) and the
        // filter coefficients (`self.a1.v[0]` and
        // `self.reg1[1]`). This value is then
        // updated in `self.reg0[1]`.
        //
        self.reg0[1] = -self.a1.v[0] * op + self.reg1[1];

        // This line is similar to the third line,
        // but it updates the second register
        // value (`self.reg1[1]`) based on the
        // previous output (`op`) and the filter
        // coefficient (`self.a2.v[0]`). This
        // value is then updated in
        // `self.reg1[1]`.
        //
        self.reg1[1] = -self.a2.v[0] * op;

        // This line is similar to the fifth line,
        // but it assigns the processed audio
        // sample (`op`) to the right output
        // channel (`*r_out`) after converting it
        // to an `f32` type.
        //
        *r_out = op as f32;
    }
}
