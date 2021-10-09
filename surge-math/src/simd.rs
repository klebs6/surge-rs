ix!();

#[cfg(target_arch = "x86_64")]
#[inline] pub unsafe fn one_over_one_plus_x(x: __m128) -> __m128 {

    let one = _mm_set1_ps(1.0);

    _mm_div_ps(
        one, 
        _mm_add_ps(one, x)
    )
}

#[cfg(target_arch = "x86_64")]
#[inline] pub unsafe fn x_to_the_fourth(x: __m128) -> __m128 {

    //option #1
    //_mm_pow_ps(x, _mm_set1_ps(4.0))

    //option #2
    //_mm_mul_ps(_mm_mul_ps(_mm_mul_ps(x,x),x),x)

    let x2 = _mm_mul_ps(x,x);
    _mm_mul_ps(x2,x2)
}


