/// 14> how would you improve the algorithm?
/// 
/// Without knowing the specific use case and performance requirements, it's difficult to provide
/// a specific answer on how to improve the algorithm. However, here are some general ideas that
/// might help:
/// 
/// 1. Consider using a more efficient nonlinear function: The current implementation uses either
/// a hyperbolic tangent or softclip function for the nonlinear component. There are many other
/// functions that could be used instead, some of which might be more efficient to compute.
/// 
/// 2. Use SIMD instructions for the nonlinear function: SIMD instructions can be used to perform
/// the same nonlinear function on multiple data points simultaneously. This could potentially
/// speed up the nonlinear component of the filter.
/// 
/// 3. Consider a different filter structure: The current implementation uses a Direct Form
/// 1 filter structure. There are many other filter structures that could be used instead, some of
/// which might be more efficient for certain applications.
/// 
/// 4. Use more efficient memory access patterns: The current implementation accesses the filter
/// coefficients and state variables in a linear fashion. Depending on the memory layout and access
/// patterns, it might be possible to speed up the algorithm by accessing memory in a different
/// way.
/// 
/// 5. Consider parallelization: If the filter is being used in a real-time system, it might be
/// possible to parallelize the computation across multiple threads or cores. This could
/// potentially improve the performance of the algorithm.
/// 
/// 15> could you describe each of these points in more detail with some examples?
/// 
/// Sure, here are some possible ways to improve the algorithm, along with examples:
/// 
/// 1. Use a better nonlinearity function: The nonlinearity function used in the current
/// implementation is a choice between a hyperbolic tangent (tanh) and a soft clipper. These are
/// both common choices, but there are many other nonlinearity functions that could be used. For
/// example, one could try using a sigmoid function, a Gaussian function, or a piecewise linear
/// function. The choice of nonlinearity function can have a significant impact on the sound of the
/// filter, so it's worth experimenting with different options to find the best one for
/// a particular application.
/// 
/// 2. Adjust the filter coefficients more frequently: The current implementation adjusts the
/// filter coefficients once per processing block. Depending on the application, it may be
/// beneficial to adjust the coefficients more frequently, such as every sample or every few
/// samples. This can help the filter better adapt to changes in the input signal, resulting in
/// better sound quality. However, adjusting the coefficients more frequently can also increase the
/// computational cost of the filter.
/// 
/// 3. Use a different filter topology: The current implementation uses a biquad filter topology,
/// which is a popular choice for audio filters. However, there are many other filter topologies
/// that could be used, such as higher-order filters or allpass filters. The choice of filter
/// topology can affect the sound of the filter, so it's worth experimenting with different
/// options.
/// 
/// 4. Optimize the filter implementation: There are many possible ways to optimize the
/// implementation of the filter. For example, one could try using different SIMD instructions,
/// using more efficient memory access patterns, or using loop unrolling. These optimizations can
/// help improve the performance of the filter, which can be especially important when processing
/// large amounts of audio data in real time.
/// 
/// 5. Add more filter stages: The current implementation uses a fixed number of filter stages,
/// which is determined by the subtype of the filter. However, it may be beneficial to allow for
/// more filter stages, especially for more complex or demanding applications. Adding more filter
/// stages can help the filter achieve a more complex frequency response, but it can also increase
/// the computational cost of the filter.

crate::ix!();

impl FilterProcessQuad for crate::NonlinearStatesFilter {

    /// The `process_quad` method first extracts the number of stages and the saturation mode from
    /// the `qfu` struct. 
    ///
    /// It then iterates over each stage and performs the filtering operations:
    ///
    /// 1. Calculates the output of the filter as the sum of the previous state (`z1`) and the
    /// current input sample multiplied by the `b0` coefficient.
    ///
    /// 2. Updates the state of the filter as follows:
    ///
    ///   - Updates `z1` as the sum of the previous `z2`, the current input sample multiplied by
    ///   `b1`, and the current output sample multiplied by `a1`.
    ///
    ///   - Updates `z2` as the difference between the current input sample multiplied by `b2` and
    ///   the current output sample multiplied by `a2`.
    ///
    /// 3. Applies a nonlinear saturation function (either a hyperbolic tangent or a softclipper)
    /// to `z1` and `z2`.
    ///
    /// 4. Returns the filter output.
    ///
    /// After processing all stages, the `coeff` array
    /// is updated by adding the corresponding
    /// `dcoeff` array. The `dcoeff` array contains
    /// the change in coefficients computed by an
    /// external optimizer, which uses gradient
    /// descent to optimize the filter response to
    /// match a desired target.
    #[inline] fn process_quad(
        &self, 
        qfu: &mut QuadFilterUnitState, 
        mut input: __m128) -> __m128 
    {

        // This line calculates the number of stages for the filter by bitwise-and-ing the
        // `comb_write_position[0]` field of the `QuadFilterUnitState` with the binary number `11`
        // (i.e., `3` in decimal). 
        //
        // The result is stored in `stages`.
        //
        // lower 2 bits of subtype is the stage count
        //
        let stages: i32 = qfu.comb_write_position[0] & 3;

        // This line selects the type of saturator to use by bitwise-shifting the
        // `comb_write_position[0]` field of the `QuadFilterUnitState` to the right by `2` bits,
        // then bitwise-and-ing the result with the binary number `11` (i.e., `3` in decimal). 
        //
        // The resulting `usize` value is passed to the `try_from` method of the `NLSFSaturator`
        // enum to try to create a new `NLSFSaturator` value. The `unwrap()` method is called on
        // the resulting `Result` to extract the `NLSFSaturator` value.
        //
        // next two bits after that select the saturator
        //
        let sat = NLSFSaturator::try_from(((qfu.comb_write_position[0] >> 2) & 3) as usize).unwrap();

        // This line starts a loop that will iterate from `0` up to and including `stages`.
        //
        // n.b. stages is zero-indexed so use <=
        //
        for stage in 0..=stages {

            // This line calculates the offset into the `reg` field of the `QuadFilterUnitState`
            // based on the current `stage`.
            //
            let reg_offset = (stage * 2) as usize;

            // These lines extract the `z1` and `z2` values from the `reg` field of the
            // `QuadFilterUnitState` using the current `reg_offset`.
            //
            let z1 = qfu.z1(reg_offset);
            let z2 = qfu.z2(reg_offset);

            // These lines extract the filter coefficients from the `coeff` field of the
            // `QuadFilterUnitState` using the appropriate constants from the `NLSFCoeff` enum.
            //
            let a1 = &qfu.coeff[C::A1]; 
            let a2 = &qfu.coeff[C::A2]; 
            let b0 = &qfu.coeff[C::B0]; 
            let b1 = &qfu.coeff[C::B1];
            let b2 = &qfu.coeff[C::B2]; 

            // In this block of code, we are processing each stage of the filter. 
            //
            // The code calculates two temporary variables `out` and `z1` for each stage, which are
            // then used to calculate `z2`.
            //
            input = unsafe {

                // out = z1 + b0 * input
                //
                // `out`: The output signal of the current stage. 
                //
                // It is calculated as the sum of `z1` (the first state variable) and the product
                // of the input signal (`input`) and the first feedforward coefficient (`b0`).
                //
                let out: __m128 = _mm_add_ps(
                    z1, 
                    _mm_mul_ps(*b0, input)
                );

                // z1 = z2 + b1 * input - a1 * out
                //
                // `z1`: The first state variable of the current stage. 
                //
                // It is updated by calculating the sum of `z2` (the second state variable), the
                // product of the input signal (`input`) and the first feedback coefficient (`b1`),
                // and the negative product of `out` and the first feedforward coefficient (`a1`).
                //
                qfu.reg[(R::Z1 as usize) + reg_offset] = _mm_add_ps(
                    z2, 
                    _mm_sub_ps(
                        _mm_mul_ps(*b1, input), 
                        _mm_mul_ps(*a1, out)
                    )
                );

                // z2 = b2 * input - a2 * out
                //
                // `z2`: The second state variable of the current stage. 
                //
                // It is updated by calculating the difference between the product of the input
                // signal (`input`) and the second feedback coefficient (`a2`) and the product of
                // `out` and the second feedforward coefficient (`b2`).
                //
                qfu.reg[(R::Z2 as usize) + reg_offset] = _mm_sub_ps(
                    _mm_mul_ps(*b2, input), 
                    _mm_mul_ps(*a2, out)
                );

                // After updating `z1` and `z2`, we apply a nonlinearity to both of them based on
                // the type of saturator chosen (`Tanh` or `SoftClip`). The output `out` is then
                // returned.
                //
                match sat {
                    NLSFSaturator::Tanh => {
                        *qfu.z1_mut(reg_offset) = fasttanh_sse_clamped(z1);
                        *qfu.z2_mut(reg_offset) = fasttanh_sse_clamped(z2);
                    },
                    NLSFSaturator::SoftClip => {
                        // note, this is a bit different to Jatin's softclipper
                        *qfu.z1_mut(reg_offset) = softclip_ps(z1); 
                        *qfu.z2_mut(reg_offset) = softclip_ps(z2);
                    },
                }

                out
            }
        }

        // Finally, the code updates the coefficients of the filter by adding the corresponding
        // `dcoeff` (delta coefficients) to each `coeff` (coefficient) element, and returns the
        // original `input` signal.
        //
        for i in 0..C::count() {
            qfu.coeff[i] = unsafe { 
                _mm_add_ps(qfu.coeff[i], qfu.dcoeff[i]) 
            };
        }

        input
    }
}
