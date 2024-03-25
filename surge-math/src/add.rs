crate::ix!();

/// Performs an element-wise addition of two memory blocks,
/// storing the result in the destination memory block.
///
/// The src1, src2, and dst pointers are expected to be
/// aligned to a multiple of 16 bytes.
///
/// This function processes the input data in blocks of four
/// 32-bit floating-point numbers.
///
/// The number of blocks is determined by the nquads
/// parameter.
///
pub fn add_block<NQ>(
    src1: *const f32, 
    src2: *const f32, 
    dst:  *mut f32, 
    nquads: NQ)

where <NQ as TryInto<u32>>::Error: fmt::Debug,
      NQ: TryInto<u32>

{
    let add = |offset: usize| {

        unsafe {
            let src1 = access(src1,offset);
            let src2 = access(src2,offset);
            let dst  = access_mut(dst,offset);

            *dst = _mm_add_ps(*src1,*src2);
        }
    };

    let nquads: u32 = nquads.try_into().unwrap();

    for i in (0..nquads).step_by(4)
    {
        let i: usize = i.try_into().unwrap();

        add(i);
        add(i + 1);
        add(i + 2);
        add(i + 3);
    }
}

