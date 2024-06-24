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
-> Result<(), AlignmentError> 
{
    if input as usize % std::mem::align_of::<__m128>() != 0 {
        return Err(AlignmentError::SrcPtr { idx: 0, required_align: std::mem::align_of::<__m128>() });
    }

    let zero: __m128 = _mm_set1_ps(0.0);

    for i in (0..(NQUADS << 2)).step_by(4) {
        if input.add(i) as usize % std::mem::align_of::<__m128>() != 0 {
            return Err(AlignmentError::SrcPtr { idx: i, required_align: std::mem::align_of::<__m128>() });
        }
        _mm_store_ps(input.add(i), zero);
    }

    Ok(())
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
pub unsafe fn clear_block_antidenormalnoise<const NQUADS: usize>(input: *mut f32) 
-> Result<(), AlignmentError> 
{
    if input as usize % std::mem::align_of::<__m128>() != 0 {
        return Err(AlignmentError::SrcPtr { idx: 0, required_align: std::mem::align_of::<__m128>() });
    }

    let smallvalue1: __m128 = _mm_set_ps(
        -0.000000000000001, 
        -0.000000000000001, 
        0.000000000000001, 
        0.000000000000001
    );
    let smallvalue2: __m128 = _mm_set_ps(
        0.000000000000001, 
        0.000000000000001, 
        -0.000000000000001, 
        -0.000000000000001
    );

    for i in (0..(NQUADS << 2)).step_by(8) {
        if input.add(i) as usize % std::mem::align_of::<__m128>() != 0 {
            return Err(AlignmentError::SrcPtr { idx: i, required_align: std::mem::align_of::<__m128>() });
        }
        if input.add(i + 4) as usize % std::mem::align_of::<__m128>() != 0 {
            return Err(AlignmentError::SrcPtr { idx: i + 4, required_align: std::mem::align_of::<__m128>() });
        }
        _mm_store_ps(input.add(i), smallvalue1);
        _mm_store_ps(input.add(i + 4), smallvalue2);
    }

    Ok(())
}
