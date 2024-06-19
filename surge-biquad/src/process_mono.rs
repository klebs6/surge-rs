crate::ix!();

/// Trait for processing a block of mono audio
/// samples.
///
pub trait ProcessBlockMono {

     /// # Safety
     ///
     /// `data` must point to `BLOCK_SIZE` valid
     /// contiguous data elements.
     ///
     /// If `out` is not `None`, `out.unwrap()`
     /// must also point to `BLOCK_SIZE` valid
     /// contiguous data elements.
     ///
    unsafe fn process_block_mono(
        &mut self, 
        data: *mut f32, 
        out: Option<*mut f32>);

    fn process_mono_block(
        &mut self,
        _input: &[f32],
        _output: Option<&mut [f32]>
    ) {
        unimplemented!();
    }
}

impl ProcessBlockMono for BiquadFilter {

    /// This is a function that processes a block
    /// of audio data. It takes a mutable
    /// reference to `self` (which is an instance
    /// of `BiquadFilter`), a pointer to the input
    /// audio data (`data`), and an optional
    /// pointer to the output audio data (`out`).
    ///
    unsafe fn process_block_mono(
        &mut self, 
        data: *mut f32, 
        out: Option<*mut f32>)
    {
        // This is a loop that processes each
        // sample in the audio block. `BLOCK_SIZE`
        // is assumed to be defined elsewhere.
        //
        for k in 0..BLOCK_SIZE {

            // These lines call the `process`
            // method on each of the filter
            // coefficients. 
            //
            // It's unclear what these methods do
            // without more context, but it's
            // likely that they update the
            // internal state of the filter.
            //
            self.a1.process();
            self.a2.process();
            self.b0.process();
            self.b1.process();
            self.b2.process();

            // These lines calculate the filter
            // output for the current sample. 
            //
            // The input sample is converted to
            // a `f64`, and then the output is
            // computed using the filter
            // coefficients (`b0`, `b1`, `b2`,
            // `a1`, `a2`) and the current and
            // previous input and output samples
            // (`reg0`, `reg1`). 
            //
            let input: f64 = *data.add(k) as f64;

            let op = input * self.b0.v[0] + self.reg0[0];

            self.reg0[0] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];
            self.reg1[0] = input * self.b2.v[0] - self.a2.v[0] * op;

            // This code writes the output sample
            // to the output buffer if `out` is
            // `Some`. Otherwise, it writes the
            // output sample back to the input
            // buffer. 
            //
            // The `as f32` conversion is
            // necessary because the output sample
            // is computed as a `f64`, but the
            // output buffer expects `f32`
            // samples.
            //
            match out {
                Some(out) => {
                    *out.add(k) = op as f32;
                },
                None => {
                    *data.add(k) = op as f32;
                }
            }
        }

        // Finally, the code calls
        // `flush_denormal` on the `reg0` and
        // `reg1` buffers to ensure that any
        // denormalized numbers are flushed to
        // zero. 
        //
        flush_denormal(&mut self.reg0[0]);
        flush_denormal(&mut self.reg1[0]);
    }
}
