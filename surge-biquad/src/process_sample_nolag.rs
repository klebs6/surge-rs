crate::ix!();

/// `ProcessSampleNolag` trait is used to define
/// a process that operates directly on the audio
/// samples, without any delay or lag. 
///
pub trait ProcessSampleNolag {

    /// The `process_sample_nolag` method takes two
    /// mutable input `f32` samples, one for the left
    /// channel and the other for the right channel,
    /// and processes them in place.
    ///
    fn process_sample_nolag(
        &mut self, 
        l: &mut f32, 
        r: &mut f32);
}

/// The first implementation is for processing
/// a single sample without any latency
/// (`ProcessSampleNolag`). 
///
/// The function takes in two mutable references
/// to `f32` values (`l` and `r`) representing the
/// left and right channel input samples,
/// respectively. The function then performs the
/// filter operation on each sample separately,
/// updating the internal state of the filter
/// represented by the `reg0` and `reg1` arrays. 
///
/// Finally, the updated samples are written back
/// to the input references.
///
impl ProcessSampleNolag for BiquadFilter {

    /// This line defines a method named
    /// `process_sample_nolag` for the
    /// `BiquadFilter` struct that takes a mutable
    /// reference to `self` as well as two mutable
    /// references to `f32` values named `l` and
    /// `r`. 
    ///
    #[inline] fn process_sample_nolag(&mut self, 
        l: &mut f32, r: &mut f32)
    {
        // This line initializes a mutable
        // variable `op` to the result of
        // a mathematical expression. The value of
        // `l` is dereferenced with `*` and cast
        // to a `f64` type. 
        //
        // It is then multiplied by the `b0` value
        // in the `self` struct and added to the
        // first value in the `reg0` array.
        //
        let mut op = *l as f64 * self.b0.v[0] + self.reg0[0];

        // This line updates the first value in
        // the `reg0` array to the result of
        // another mathematical expression. 
        //
        // The value of `l` is dereferenced with
        // `*` and cast to a `f64` type. 
        //
        // It is then multiplied by the `b1` value
        // in the `self` struct and subtracted
        // from the product of `self.a1.v[0]` and
        // `op`. 
        //
        // This result is then added to the first
        // value in the `reg1` array.
        //
        self.reg0[0] = *l as f64 * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];

        // This line updates the first value in
        // the `reg1` array to the result of
        // another mathematical expression. 
        //
        // The value of `l` is dereferenced with
        // `*` and cast to a `f64` type. 
        //
        // It is then multiplied by the `b2` value
        // in the `self` struct and subtracted
        // from the product of `self.a2.v[0]` and
        // `op`.
        //
        self.reg1[0] = *l as f64 * self.b2.v[0] - self.a2.v[0] * op;

        // This line updates the value of `l` to
        // the value of `op` cast to an `f32`
        // type.
        //
        *l = op as f32;

        // This line initializes the value of `op`
        // to the result of another mathematical
        // expression. 
        //
        // The value of `r` is dereferenced with
        // `*` and cast to a `f64` type. 
        //
        // It is then multiplied by the `b0` value
        // in the `self` struct and added to the
        // second value in the `reg0` array.
        //
        op = *r as f64 * self.b0.v[0] + self.reg0[1];

        self.reg0[1] = *r as f64 * self.b1.v[0] - self.a1.v[0] * op + self.reg1[1];
        self.reg1[1] = *r as f64 * self.b2.v[0] - self.a2.v[0] * op;

        *r = op as f32;
    }
}
