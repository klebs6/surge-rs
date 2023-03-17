crate::ix!();

/// This is Rust code defining two unsafe functions for fading blocks of audio
/// data. 
///
/// Both functions are part of the `LipolPs` struct implementation. Here is
/// a brief summary of what the two functions do:
/// 
/// Both functions have a `where` clause that restricts the type of `nquads` to
/// only types that can be converted into a `usize` using the `TryInto` trait. 
///
/// If the conversion fails, the function panics with a debug message.
///
/// In both methods, the mixing operation is performed using the `_mm_mul_ps`
/// and `_mm_add_ps` functions, which are intrinsic functions that allow for
/// low-level manipulation of 128-bit SSE registers. SSE stands for "Streaming
/// SIMD Extensions", and is a set of instructions that provide parallel
/// processing capabilities for SIMD (Single Instruction, Multiple Data)
/// operations on data stored in memory.
/// 
/// The `lipol_ps_sse_block!` macro is used to provide a loop over the data that
/// needs to be processed, taking into account the fact that SSE instructions
/// operate on 128-bit registers, and therefore the data needs to be processed
/// in 4-byte (32-bit) chunks.
/// 
/// Overall, these methods are low-level operations that are performing SIMD
/// operations on memory buffers using SSE instructions, and they require
/// careful use of pointers and unsafe code to achieve the desired performance
/// gains.
/// 
impl crate::LipolPs {

    /// The `fade_block_to` method takes three pointers to floating-point
    /// numbers (`src1`, `src2`, and `dst`) and an integer `nquads` as
    /// arguments. 
    ///
    /// It then performs a fade operation on the `src1` and `src2` buffers,
    /// mixing them together based on a set of fading parameters that are
    /// determined by the current state of the `LipolPs` object. 
    ///
    /// The resulting mix is written to the `dst` buffer.
    ///
    /// ----------------------------- 
    /// Fades a block of audio data from two sources into a destination buffer. 
    ///
    /// The function takes three pointers to `f32` buffers - `src1`, `src2`, and
    /// `dst` - and an integer `nquads`. 
    ///
    /// The `nquads` parameter specifies the number of quads (groups of four
    /// `f32` values) to process. 
    ///
    /// The function uses SSE intrinsics (SIMD operations) to perform the fade. 
    ///
    /// The operation of the function is unsafe, so the user needs to make sure
    /// that the input buffers have at least `nquads` elements and the function
    /// can safely access them.
    ///
    /// # Safety
    ///
    /// neet to make sure we can safely access nquads elements from src1, src2,
    /// and dst
    ///
    pub unsafe fn fade_block_to<NQ>(
        &mut self,
        src1:   *mut f32,
        src2:   *mut f32,
        dst:    *mut f32,
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug, NQ: TryInto<usize>
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 2; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {

                let dst  = dst as *mut __m128;
                let src1 = src1 as *mut __m128;
                let src2 = src2 as *mut __m128;
                let one  = m128_one![];


                let mut a: __m128 = _mm_mul_ps(
                    *src1.add(i), 
                    _mm_sub_ps(one, *y1)
                );

                let mut b: __m128 = _mm_mul_ps(
                    *src2.add(i), 
                    *y1
                );

                *dst.add(i) = _mm_add_ps(a, b);

                *y1 = _mm_add_ps(*y1, *dy);

                a = _mm_mul_ps(
                    *src1.add(i + 1), 
                    _mm_sub_ps(one, *y2)
                );

                b = _mm_mul_ps(
                    *src2.add(i + 1), 
                    *y2
                );

                *dst.add(i + 1) = _mm_add_ps(a, b);

                *y2 = _mm_add_ps(*y2, *dy);
            }
        );
    }

    /// The `fade_2_blocks_to` method is similar to `fade_block_to`, but it
    /// takes six pointers to floating-point numbers (`src11`, `src12`, `src21`,
    /// `src22`, `dst1`, and `dst2`) instead of three, and it performs two fades
    /// in parallel, writing the results to `dst1` and `dst2` respectively.
    ///
    /// ------------------------ 
    /// Fades two blocks of audio data from four sources into two destination
    /// buffers. 
    ///
    /// The function takes six pointers to `f32` buffers - `src11`, `src12`,
    /// `src21`, `src22`, `dst1`, and `dst2` - and an integer `nquads`. 
    ///
    /// The `nquads` parameter specifies the number of quads (groups of four
    /// `f32` values) to process. 
    ///
    /// The function uses SSE intrinsics (SIMD operations) to perform the
    /// fade. The operation of the function is also unsafe, so the user needs to
    /// make sure that the input buffers have at least `nquads` elements and the
    /// function can safely access them.
    ///
    /// # Safety
    ///
    /// neet to make sure we can safely access nquads elements from src11,
    /// src12, src21, src22, dst1, and dst2
    ///
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn fade_2_blocks_to<NQ>(
        &mut self,
        src11:  *mut f32,
        src12:  *mut f32,
        src21:  *mut f32,
        src22:  *mut f32,
        dst1:   *mut f32,
        dst2:   *mut f32,
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug, NQ: TryInto<usize>
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 2; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {

                let src11 = src11 as *mut __m128;
                let src12 = src12 as *mut __m128;

                let src21 = src21 as *mut __m128;
                let src22 = src22 as *mut __m128;

                let dst1 = dst1 as *mut __m128;
                let dst2 = dst2 as *mut __m128;

                let one = m128_one![];

                let mut a: __m128 = _mm_mul_ps(
                    *src11.add(i), 
                    _mm_sub_ps(one, *y1)
                );

                let mut b: __m128 = _mm_mul_ps(
                    *src12.add(i), 
                    *y1
                );

                *dst1.add(i) = _mm_add_ps(a, b);

                a = _mm_mul_ps(
                    *src21.add(i), 
                    _mm_sub_ps(one, *y1)
                );

                b = _mm_mul_ps(
                    *src22.add(i), 
                    *y1
                );

                *dst2.add(i) = _mm_add_ps(a, b);

                *y1 = _mm_add_ps(*y1, *dy);

                a = _mm_mul_ps(
                    *src11.add(i + 1), 
                    _mm_sub_ps(one, *y2)
                );

                b = _mm_mul_ps(
                    *src12.add(i + 1), 
                    *y2
                );

                *dst1.add(i + 1) = _mm_add_ps(a, b);

                a = _mm_mul_ps(
                    *src21.add(i + 1), 
                    _mm_sub_ps(one, *y2)
                );

                b = _mm_mul_ps(
                    *src22.add(i + 1), 
                    *y2
                );

                *dst2.add(i + 1) = _mm_add_ps(a, b);

                *y2 = _mm_add_ps(*y2, *dy);
            });
    }
}
