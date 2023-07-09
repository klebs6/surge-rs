crate::ix!();

/// Clears a block of memory containing `NQUADS * 4` f32
/// values by setting each value to 0.
///
/// # Safety
///
/// The `input` pointer must point to a valid, contiguous
/// memory range of at least `(NQUADS * 4)` f32 values.
///
#[cfg(target_feature = "sse")]
pub unsafe fn clear_block<const NQUADS: usize>(input: *mut f32) 
{
    let zero: __m128 = _mm_set1_ps(0.0);

    for i in (0..(NQUADS << 2)).step_by(4)
    {
        _mm_store_ps(input.add(i), zero);
    }
}

/// Clears a block of memory containing `NQUADS * 4` f32
/// values by setting each value to an anti-denormal noise
/// value.
///
/// # Safety
///
/// The `input` pointer must point to a valid, contiguous
/// memory range of at least `(NQUADS * 4)` f32 values.
///
#[cfg(target_feature = "sse")]
pub unsafe fn clear_block_antidenormalnoise<const NQUADS: usize>( input: *mut f32) 
{
    let smallvalue: __m128 = 
        _mm_set_ps(
            0.000000000000001, 
            0.000000000000001, 
            -0.000000000000001, 
            -0.000000000000001);

    for i in (0..(NQUADS << 2)).step_by(8)
    {
        _mm_store_ps(input.add(i),     smallvalue);
        _mm_store_ps(input.add(i + 4), smallvalue);
    }
}
