crate::ix!();

/// This function takes in two pointers, `l` and `r`, which
/// point to arrays of `__m128` (128-bit floating-point
/// numbers) representing the left and right channels of an
/// input stereo signal. 
///
/// The function returns a new array of `__m128` by
/// interleaving the input stereo samples and storing them
/// in a 1-dimensional array of 128-bit values
///
pub fn create_halfrate_scratch_buffer(nsamples: usize, l: *mut __m128, r: *mut __m128) -> A1d<__m128> {

    // We start by creating a new array `o` of interleaved
    // stereo samples. 
    //
    let mut o = A1d::<__m128>::from_elem(HALFRATE_BLOCK_SIZE, z128());

    // We then fill the buffer with interleaved stereo
    // samples using an unsafe loop that steps by 4 for each
    // iteration.
    //
    for k in (0..nsamples).step_by(4) {

        // [o3,o2,o1,o0] = [L0,L0,R0,R0]
        //
        // For each iteration of the loop, the function
        // shuffles the values pointed to by `l` and `r`
        // pointers in order to create an interleaved stereo
        // sample. 
        //
        // The shuffle is performed using the Intel SSE
        // instruction `_mm_shuffle_ps`. The
        // `_mm_shuffle_ps` function takes four arguments. 
        //
        // The first two arguments are the two 128-bit
        // values to be shuffled. The third argument is used
        // to specify which values should be copied to the
        // output value in the format `[3, 2, 1, 0]`. 
        //
        // In this case, it is used to shuffle the left and
        // right channel values together to create the
        // interleaved stereo sample.
        //
        unsafe {
            o[k]     = _mm_shuffle_ps(*l.add(k >> 2), *r.add(k >> 2), _MM_SHUFFLE(0, 0, 0, 0));
            o[k + 1] = _mm_shuffle_ps(*l.add(k >> 2), *r.add(k >> 2), _MM_SHUFFLE(1, 1, 1, 1));
            o[k + 2] = _mm_shuffle_ps(*l.add(k >> 2), *r.add(k >> 2), _MM_SHUFFLE(2, 2, 2, 2));
            o[k + 3] = _mm_shuffle_ps(*l.add(k >> 2), *r.add(k >> 2), _MM_SHUFFLE(3, 3, 3, 3));
        }
    }

    o
}
