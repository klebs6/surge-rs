crate::ix!();

/// Multiplies two 32-bit unsigned integers, shifts the
/// 64-bit result right by 16 bits, and returns the result
/// as a 32-bit unsigned integer.
///
#[inline] pub fn big_mul_r16(a: u32, b: u32) -> u32 
{
    let c: u64 = (a as u64) * (b as u64);
    (c >> 16) as u32
}

///______________________________________________________
/// Performs an element-wise multiplication of two memory
/// blocks, storing the result in the destination memory
/// block.
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
pub fn mul_block<NQ>(
    src1: *mut f32, 
    src2: *mut f32, 
    dst:  *mut f32, 
    nquads: NQ) 
    where 
    <NQ as TryInto<u32>>::Error: fmt::Debug, 
    NQ: TryInto<u32>
{
    let do_mul = |offset: usize| {
        unsafe {
            let src1 = access(src1,offset);
            let src2 = access(src2,offset);
            let dst  = access_mut(dst, offset);
            *dst = _mm_mul_ps(*src1, *src2);
        }
    };

    let nquads: u32 = nquads.try_into().unwrap();

    for i in (0..nquads).step_by(4)
    {
        let i: usize = i.try_into().unwrap();

        do_mul(i);
        do_mul(i + 1);
        do_mul(i + 2);
        do_mul(i + 3);
    }
}
