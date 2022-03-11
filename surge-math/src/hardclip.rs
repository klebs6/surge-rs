ix!();

#[cfg(target_arch = "x86_64")] #[inline] 
pub fn hardclip_ss(x: __m128) -> __m128
{
    unsafe {
        let x_min: __m128 = _mm_set_ss(-1.0);
        let x_max: __m128 = _mm_set_ss(1.0);
        _mm_max_ss(_mm_min_ss(x, x_max), x_min)
    }
}

#[cfg(target_arch = "x86_64")] #[inline] 
pub fn hardclip_ps(x: __m128) -> __m128
{
    unsafe {
        let x_min: __m128 = _mm_set1_ps(-1.0);
        let x_max: __m128 = _mm_set1_ps(1.0);
        _mm_max_ps(_mm_min_ps(x, x_max), x_min)
    }
}

#[cfg(target_arch = "x86_64")] #[inline] 
pub fn hardclip8_sd(x: __m128d) -> __m128d
{
    unsafe {
        let x_min: __m128d = _mm_set_sd(-7.0);
        let x_max: __m128d =  _mm_set_sd(8.0);
        _mm_max_sd(_mm_min_sd(x, x_max), x_min)
    }
}


///# Safety
///
///ensure we can access nquads * 4 elements contiguousely 
///from a valid starting point x
pub unsafe fn hardclip_block<NQ>(x: *mut f32, nquads: NQ) 
    where <NQ as TryInto<usize>>::Error: Debug, NQ: TryInto<usize>
{
    let nquads: usize = nquads.try_into().unwrap();

    let x_min: __m128 = _mm_set1_ps(-1.0);
    let x_max: __m128 = _mm_set1_ps(1.0);

    for i in (0_usize..(nquads << 2)).step_by(8)
    {
        _mm_store_ps(
            x.add(i), 
            _mm_max_ps(
                _mm_min_ps(
                    _mm_load_ps(x.add(i)), 
                    x_max), 
                x_min));

        _mm_store_ps(
            x.add(i + 4), 
            _mm_max_ps(
                _mm_min_ps(
                    _mm_load_ps(x.add(i + 4)), 
                    x_max), 
                x_min));
    }
}

///# Safety
///
///ensure we can access nquads * 4 elements contiguousely 
///from a valid starting point x
pub unsafe fn hardclip_block8<NQ>(x: *mut f32, nquads: NQ) 
where 
    NQ: TryInto<usize>,
    <NQ as TryInto<usize>>::Error: Debug
{
    let nquads: usize = nquads.try_into().unwrap();

    let x_min: __m128 = _mm_set1_ps(-8.0);
    let x_max: __m128 = _mm_set1_ps(8.0);

    let do_clip = |x_in: *mut f32| {

        let elem = _mm_load_ps(x_in);

        let clipped = 
            _mm_max_ps(_mm_min_ps(elem,x_max), min);

        _mm_store_ps( x_in, clipped);
    };

    for i in (0_usize..(nquads << 2)).step_by(8) 
    {
        do_clip(x.add(i));
        do_clip(x.add(i + 4));
    }
}

