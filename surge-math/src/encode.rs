crate::ix!();

#[inline] unsafe fn access(x: *mut f32, idx: usize) -> *mut __m128 {
    (x as *mut __m128).add(idx)
}

///# Safety
///
///ensure we can access nquads * 4 values from
///both l and r
pub unsafe fn encode_mid_side<NQ>(
    l: *mut f32, 
    r: *mut f32, 
    m: *mut f32, 
    s: *mut f32, 
    nquads: NQ)

where 
    <NQ as TryInto<usize>>::Error: Debug, 
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

///# Safety
///
///ensure we can access nquads * 4 values from
///both l and r
pub unsafe fn decode_mid_side<NQ>(
    m: *mut f32, 
    s: *mut f32, 
    l: *mut f32, 
    r: *mut f32, 
    nquads: NQ) 

where 
    <NQ as TryInto<usize>>::Error: Debug, 
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
