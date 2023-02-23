crate::ix!();

impl ProcessBlockSlowlag for BiquadFilter {

    /// takes two raw pointers to `f32` data,
    /// which are expected to be contiguous and
    /// have `BLOCK_SIZE` valid elements. 
    ///
    /// Note the `unsafe` keyword before the
    /// function signature, which indicates that
    /// this function makes certain assumptions
    /// about the correctness of its input that
    /// the compiler cannot verify.
    ///
    /// # Safety
    ///
    /// data_l and data_r must each point to
    /// BLOCK_SIZE valid contiguous data elements
    ///
    unsafe fn process_block_slowlag(&mut self, data_l: *mut f32, data_r: *mut f32)
    {
        self.a1.process();
        self.a2.process();
        self.b0.process();
        self.b1.process();
        self.b2.process();

        // This begins a loop that will execute
        // `BLOCK_SIZE` times, where `BLOCK_SIZE`
        // is likely a constant defined somewhere
        // in the code.
        //
        for k in 0..BLOCK_SIZE {

            // This line uses pointer arithmetic
            // to get the `k`th element of the
            // `data_l` slice (i.e., the `k`th
            // `f32` value that `data_l` points
            // to), casts it to an `f64`, and
            // stores it in the `input` variable.
            //
            let mut input: f64 = *data_l.add(k) as f64;

            // This line multiplies `input` by the
            // `v[0]` field of `self.b0`, adds the
            // resulting value to the `reg0[0]`
            // field of `self`, and stores the
            // result in the `op` variable.
            //
            let mut op = input * self.b0.v[0] + self.reg0[0];

            // This line updates the `reg0[0]`
            // field of `self` based on the
            // previous value of `op`, as well as
            // the `v[0]` fields of `self.b1` and
            // `self.a1`, and the current value of
            // `input` and `reg1[0]`.
            //
            self.reg0[0] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];

            // This line updates the `reg1[0]`
            // field of `self` based on the
            // previous value of `op`, as well as
            // the `v[0]` fields of `self.b2` and
            // `self.a2`
            //
            self.reg1[0] = input * self.b2.v[0] - self.a2.v[0] * op;

            // This line writes the value of `op`
            // (cast to an `f32`) back to the
            // `k`th element of the `data_l`
            // slice.
            //
            *data_l.add(k) = op as f32;

            // These lines are similar to the ones
            // we just saw, but they operate on
            // the `data_r` slice instead of the
            // `data_l` slice. 
            //
            // Specifically, they 
            //
            // read the `k`th element of `data_r`
            // into `input`,
            //
            // compute a value for `op` based on
            // `input` and the current state of
            // the `BiquadFilter`, 
            //
            // update the `reg0[1]` and `reg1[1]`
            // fields of `self` based on `input`,
            // `op`, and the `v[0]` fields of
            // various components of the
            // `BiquadFilter`,
            //
            // and write the value of `op` (cast
            // to an `f32`) back to the `k`th
            // element of `data_r`.
            //
            input = *data_r.add(k) as f64;

            op = input * self.b0.v[0] + self.reg0[1];

            self.reg0[1] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[1];
            self.reg1[1] = input * self.b2.v[0] - self.a2.v[0] * op;

            *data_r.add(k) = op as f32;
        }

        // These lines call a function called
        // `flush_denormal` on each of the four
        // fields of `self` that were just
        // updated. 
        //
        // Without more context, it's hard to say
        // exactly what `flush_denormal` does, but
        // it's likely some kind of utility
        // function that ensures that the values
        // in these fields are in a certain
        // numerical format.
        //
        flush_denormal(&mut self.reg0[0]);
        flush_denormal(&mut self.reg1[0]);
        flush_denormal(&mut self.reg0[1]);
        flush_denormal(&mut self.reg1[1]);
    }
}
