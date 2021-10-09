ix!();

///# Safety
///
///ensure we can access nquads * 4 values from both l and r
pub unsafe fn encode_mid_side<NQ>(l: *mut f32, r: *mut f32, m: *mut f32, s: *mut f32, nquads: NQ)
where 
    <NQ as TryInto<usize>>::Error: Debug, 
    NQ: TryInto<usize>
{
    let nquads: usize = nquads.try_into().unwrap();

    for i in (0..nquads).step_by(4)
    {
        *(m as *mut __m128).add(i)     = _mm_mul_ps(_mm_add_ps(*(l as *mut __m128).add(i),     *(r as *mut __m128).add(i)), m128_half![]);
        *(s as *mut __m128).add(i)     = _mm_mul_ps(_mm_sub_ps(*(l as *mut __m128).add(i),     *(r as *mut __m128).add(i)), m128_half![]);
        *(m as *mut __m128).add(i + 1) = _mm_mul_ps(_mm_add_ps(*(l as *mut __m128).add(i + 1), *(r as *mut __m128).add(i + 1)), m128_half![]);
        *(s as *mut __m128).add(i + 1) = _mm_mul_ps(_mm_sub_ps(*(l as *mut __m128).add(i + 1), *(r as *mut __m128).add(i + 1)), m128_half![]);
        *(m as *mut __m128).add(i + 2) = _mm_mul_ps(_mm_add_ps(*(l as *mut __m128).add(i + 2), *(r as *mut __m128).add(i + 2)), m128_half![]);
        *(s as *mut __m128).add(i + 2) = _mm_mul_ps(_mm_sub_ps(*(l as *mut __m128).add(i + 2), *(r as *mut __m128).add(i + 2)), m128_half![]);
        *(m as *mut __m128).add(i + 3) = _mm_mul_ps(_mm_add_ps(*(l as *mut __m128).add(i + 3), *(r as *mut __m128).add(i + 3)), m128_half![]);
        *(s as *mut __m128).add(i + 3) = _mm_mul_ps(_mm_sub_ps(*(l as *mut __m128).add(i + 3), *(r as *mut __m128).add(i + 3)), m128_half![]);
    }
}

///# Safety
///
///ensure we can access nquads * 4 values from both l and r
pub unsafe fn decode_mid_side<NQ>(m: *mut f32, s: *mut f32, l: *mut f32, r: *mut f32, nquads: NQ) 
where 
    <NQ as TryInto<usize>>::Error: Debug, 
    NQ: TryInto<usize>
{
    let nquads: usize = nquads.try_into().unwrap();

    for i in (0..nquads).step_by(4)
    {
        *(l as *mut __m128).add(i)     = _mm_add_ps(*(m as *mut __m128).add(i),     *(s as *mut __m128).add(i));
        *(r as *mut __m128).add(i)     = _mm_sub_ps(*(m as *mut __m128).add(i),     *(s as *mut __m128).add(i));
        *(l as *mut __m128).add(i + 1) = _mm_add_ps(*(m as *mut __m128).add(i + 1), *(s as *mut __m128).add(i + 1));
        *(r as *mut __m128).add(i + 1) = _mm_sub_ps(*(m as *mut __m128).add(i + 1), *(s as *mut __m128).add(i + 1));
        *(l as *mut __m128).add(i + 2) = _mm_add_ps(*(m as *mut __m128).add(i + 2), *(s as *mut __m128).add(i + 2));
        *(r as *mut __m128).add(i + 2) = _mm_sub_ps(*(m as *mut __m128).add(i + 2), *(s as *mut __m128).add(i + 2));
        *(l as *mut __m128).add(i + 3) = _mm_add_ps(*(m as *mut __m128).add(i + 3), *(s as *mut __m128).add(i + 3));
        *(r as *mut __m128).add(i + 3) = _mm_sub_ps(*(m as *mut __m128).add(i + 3), *(s as *mut __m128).add(i + 3));
    }
}
