/*!
  | Stereo processing is a technique used in audio
  | processing that involves processing two audio
  | signals simultaneously - one for the left
  | channel and one for the right channel. This is
  | in contrast to mono processing, where only
  | a single audio signal is processed.
  |
  | Stereo processing is often used in situations
  | where it's important to maintain the stereo
  | image of the original signal. For example,
  | when applying a filter to an audio signal, you
  | might want to apply the same filter to both
  | the left and right channels to maintain the
  | relative balance between them.
  |
  | To perform stereo processing, the audio
  | signals for the left and right channels are
  | processed separately, but typically using the
  | same processing algorithm. The processed
  | signals are then combined into a single stereo
  | signal before being output. This can be done
  | using a variety of techniques, such as panning
  | the processed left and right signals to their
  | respective channels, or summing the left and
  | right signals together.
  |
  | In the context of the code you provided, the
  | `process_block_stereo` function processes two
  | audio signals simultaneously, one for the left
  | channel and one for the right channel. The
  | processed signals are then optionally output
  | as a stereo signal, which is represented by
  | the `out` argument. If `out` is `Some`, the
  | processed signals are written to the memory
  | locations pointed to by the two pointers
  | contained in the `out` tuple. Otherwise, the
  | processed signals are written to the memory
  | locations pointed to by `data_l` and `data_r`,
  | representing the left and right channels
  | respectively.
  */

crate::ix!();

/// A trait implementation that provides a method
/// for processing stereo audio data in blocks.
///
impl ProcessBlockStereo for BiquadFilter {

    /// #### Arguments
    /// 
    /// - `&mut self`: The biquad filter instance
    /// to use for processing the audio data.
    ///
    /// - `data_l`: A mutable pointer to the left
    /// channel audio data.
    ///
    /// - `data_r`: A mutable pointer to the right
    /// channel audio data.
    ///
    /// - `out`: An optional tuple of mutable
    /// pointers to output buffers for the left
    /// and right channel audio data.
    /// 
    /// #### Output
    /// 
    /// This method mutates the input buffers
    /// pointed to by `data_l` and `data_r`. If an
    /// `out` tuple is provided, the output is
    /// also written to the corresponding output
    /// buffers.
    ///
    /// # Safety
    ///
    /// data_l and data_r must each point to
    /// BLOCK_SIZE valid contiguous data elements
    ///
    unsafe fn process_block_stereo(&mut self, 
        data_l: *mut f32, 
        data_r: *mut f32, 
        out: Option<(*mut f32, *mut f32)>
    )
    {

        // This is a loop over the range
        // `0..BLOCK_SIZE`. `BLOCK_SIZE` is not
        // defined in the code provided, but it is
        // likely a constant that defines the
        // number of samples to process.
        //
        for k in 0..BLOCK_SIZE {

            // This calls the `process` method on
            // five `FilterParameter` structs:
            // `a1`, `a2`, `b0`, `b1`, and `b2`. 
            //
            // These structs contain coefficients
            // used to perform filtering on the
            // input data.
            //
            self.a1.process();
            self.a2.process();
            self.b0.process();
            self.b1.process();
            self.b2.process();

            // This gets the `k`-th value from the
            // `data_l` array, converts it to
            // a `f64`, and assigns it to the
            // `input` variable.
            //
            let mut input: f64 = *data_l.add(k) as f64;

            // This computes the first stage of
            // the filter operation. `input` is
            // multiplied by the `b0` coefficient
            // and added to the current value in
            // `self.reg0[0]`.
            //
            let mut op = input * self.b0.v[0] + self.reg0[0];

            // This computes the second stage of
            // the filter operation and updates
            // the state variables in `self.reg0`
            // and `self.reg1`.
            //
            self.reg0[0] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];

            // This computes the third stage of
            // the filter operation and updates
            // the state variable in `self.reg1`.
            //
            self.reg1[0] = input * self.b2.v[0] - self.a2.v[0] * op;

            match out {
                Some(out) => *out.0.add(k)  = op as f32,
                None      => *data_l.add(k) = op as f32
            };

            input = *data_r.add(k) as f64;

            op = input * self.b0.v[0] + self.reg0[1];

            self.reg0[1] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[1];
            self.reg1[1] = input * self.b2.v[0] - self.a2.v[0] * op;

            match out {
                Some(out) => *out.1.add(k)  = op as f32,
                None      => *data_r.add(k) = op as f32
            };
        }

        flush_denormal(&mut self.reg0[0]);
        flush_denormal(&mut self.reg1[0]);
        flush_denormal(&mut self.reg0[1]);
        flush_denormal(&mut self.reg1[1]);
    }
}
