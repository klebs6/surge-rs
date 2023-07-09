crate::ix!();

/// This function is a helper function used in the
/// `encode_mid_side` and `decode_mid_side` functions to
/// calculate the memory address of a block of 4 `f32`
/// values based on a given index `idx`. 
///
/// This function converts the provided pointer `x` to
/// a pointer to `__m128`, which is a SIMD vector type that
/// can hold 4 `f32` values, and then adds the `idx` offset
/// to calculate the address of the block of 4 values.
///
#[inline] unsafe fn access(x: *mut f32, idx: usize) -> *mut __m128 {
    (x as *mut __m128).add(idx)
}

/// Encode mid-side from left and right channels.
///
/// The mid channel is created as the average of left and right,
/// and the side channel is created as the difference of left and right.
///
/// # Safety
///
/// This function is unsafe because it requires that the provided pointers
/// `l`, `r`, `m`, and `s` all point to valid, properly aligned memory that
/// can be read and written to for a total of `nquads * 4` elements.
///
/// # Arguments
///
/// * `l` - A pointer to the left channel samples.
/// * `r` - A pointer to the right channel samples.
/// * `m` - A pointer to the mid channel samples.
/// * `s` - A pointer to the side channel samples.
/// * `nquads` - The number of quads (blocks of 4) to encode.
///
/// # Panics
///
/// This function will panic if the `nquads` argument is too large and
/// the pointers would cause an out-of-bounds access.
///
pub unsafe fn encode_mid_side<NQ>(
    l: *mut f32, 
    r: *mut f32, 
    m: *mut f32, 
    s: *mut f32, 
    nquads: NQ)

where 
    <NQ as TryInto<usize>>::Error: fmt::Debug, 
    NQ: TryInto<usize>
{
    let nquads: usize = nquads.try_into().unwrap();

    let half = m128_half![];

    let encode = |offset: usize| {

        let m = access(m,offset);
        let s = access(s,offset);
        let r = access(r,offset);
        let l = access(l,offset);

        *m = _mm_mul_ps(_mm_add_ps(*l, *r), half);
        *s = _mm_mul_ps(_mm_sub_ps(*l, *r), half);
    };

    for i in (0..nquads).step_by(4)
    {
        encode(i);
        encode(i + 1);
        encode(i + 2);
        encode(i + 3);
    }
}

/// This function takes pointers to the mid and side channel
/// samples, as well as pointers to the left and right
/// channel samples, and decodes the left and right channels
/// based on the mid and side channels. The left channel is
/// created as the sum of the mid and side channels, and the
/// right channel is created as the difference of the mid
/// and side channels. The decoding is performed for a total
/// of `nquads` blocks of 4 samples each.
///
/// This function is marked as unsafe because it requires
/// that the provided pointers `m`, `s`, `l`, and `r` all
/// point to valid, properly aligned memory that can be read
/// and written to for a total of `nquads * 4` elements. If
/// the `nquads` argument is too large and the pointers
/// would cause an out-of-bounds access, this function will
/// panic.
///
/// # Safety
///
/// ensure we can access nquads * 4 values from both l and r
///
pub unsafe fn decode_mid_side<NQ>(
    m: *mut f32, 
    s: *mut f32, 
    l: *mut f32, 
    r: *mut f32, 
    nquads: NQ) 

where 
    <NQ as TryInto<usize>>::Error: fmt::Debug, 
    NQ: TryInto<usize>
{
    let nquads: usize = nquads.try_into().unwrap();

    let decode = |offset: usize| {

        let m = access(m,offset);
        let s = access(s,offset);
        let r = access(r,offset);
        let l = access(l,offset);

        *l = _mm_add_ps(*m, *s);
        *r = _mm_sub_ps(*m, *s);
    };

    for i in (0..nquads).step_by(4)
    {
        decode(i);
        decode(i + 1);
        decode(i + 2);
        decode(i + 3);
    }
}
