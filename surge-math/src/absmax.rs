crate::ix!();

#[inline] pub fn abs_ps(x: __m128 ) -> __m128
{
    unsafe{ _mm_and_ps(x, m128_mask_absval![]) }
}

/// Returns the maximum absolute value found in a memory
/// block.
///
/// The d pointer is expected to be aligned to a multiple of
/// 16 bytes.
///
/// This function processes the input data in blocks of four
/// 32-bit floating-point numbers.
///
/// The number of blocks is determined by the nquads
/// parameter.
///
#[inline] pub unsafe fn get_absmax(d: *mut f32, nquads: usize) -> f32
{
    let mut mx1: __m128 = _mm_setzero_ps();
    let mut mx2: __m128 = _mm_setzero_ps();

    for i in (0..nquads).step_by(2) {

        let d_0 = access(d,i);
        let d_1 = access(d,i+1);

        let mask = m128_mask_absval![];

        mx1 = _mm_max_ps( mx1, _mm_and_ps(*d_0, mask));
        mx2 = _mm_max_ps( mx2, _mm_and_ps(*d_1, mask));
    }

    mx1 = _mm_max_ps(mx1, mx2);
    mx1 = max_ps_to_ss(mx1);

    let mut f: f32 = 0.0;

    _mm_store_ss(&mut f, mx1);

    f
}

/// Returns the maximum absolute value found in two memory blocks.
/// 
/// The `d1` and `d2` pointers are expected to be aligned to a multiple of 16 bytes.
/// This function processes the input data in blocks of four 32-bit floating-point numbers.
/// The number of blocks is determined by the `nblocks` parameter.
#[inline]
pub unsafe fn get_absmax_2(d1: *const f32, d2: *const f32, nblocks: usize) -> f32 {

    assert_eq!((d1 as usize) % 16, 0, "d1 pointer must be aligned");
    assert_eq!((d2 as usize) % 16, 0, "d2 pointer must be aligned");

    // Mask to compute absolute values, zeroing out the sign bit
    let mask = _mm_set1_ps(0x7FFFFFFFu32 as f32); 

    // Initialize max value accumulator
    let mut max_val = _mm_setzero_ps(); 

    for i in 0..nblocks {
        let offset = i * 4; // Calculate offset for each quad

        // Load blocks from d1 and d2, aligned loads assuming inputs are aligned
        let d1_block = _mm_load_ps(d1.add(offset)); 
        let d2_block = _mm_load_ps(d2.add(offset)); 

        // Apply mask for absolute values and find max per element between d1 and d2
        let abs_d1 = _mm_and_ps(d1_block, mask); 
        let abs_d2 = _mm_and_ps(d2_block, mask); 

        let max_d1_d2 = _mm_max_ps(abs_d1, abs_d2); // Get max per element between d1 and d2
        max_val = _mm_max_ps(max_val, max_d1_d2); // Update global max
    }

    // Horizontal max to reduce all values in `max_val` to a single maximum value
    let max_fold1 = _mm_max_ps(max_val, _mm_shuffle_ps(max_val, max_val, _MM_SHUFFLE(1, 0, 3, 2)));
    let max_fold2 = _mm_max_ps(max_fold1, _mm_shuffle_ps(max_fold1, max_fold1, _MM_SHUFFLE(2, 3, 0, 1)));
    let max_final = _mm_max_ps(max_fold2, _mm_shuffle_ps(max_fold2, max_fold2, _MM_SHUFFLE(0, 0, 3, 2)));

    // Extract the maximum value from the SIMD register
    _mm_cvtss_f32(max_final)
}
