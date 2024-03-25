crate::ix!();

/// Adds the contents of the source memory block to the
/// destination memory block.
///
/// The src and dst pointers are expected to be aligned to
/// a multiple of 16 bytes.
///
/// This function processes the input data in blocks of four
/// 32-bit floating-point numbers.
///
/// The number of blocks is determined by the nquads
/// parameter.
///
pub fn accumulate_block(src: *const f32, dst: *mut f32, nquads: usize) {
    assert_eq!((src as usize) % 16, 0, "src pointer must be aligned");
    assert_eq!((dst as usize) % 16, 0, "dst pointer must be aligned");

    for i in 0..nquads {
        unsafe {
            let src_quad = _mm_load_ps(src.add(i * 4));
            let dst_quad = _mm_load_ps(dst.add(i * 4));
            let result_quad = _mm_add_ps(dst_quad, src_quad);
            _mm_store_ps(dst.add(i * 4), result_quad);
        }
    }
}
