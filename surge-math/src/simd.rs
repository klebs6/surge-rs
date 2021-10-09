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

#[allow(dead_code)]
pub mod simd_m128 {

    use super::*;

    #[inline(always)] pub fn mask_signbit()          -> __m128 { unsafe { _mm_set1_ps(0x80000000 as u32 as f32) } }
    #[inline(always)] pub fn mask_absval()           -> __m128 { unsafe { _mm_set1_ps(0x7fffffff as f32) } }
    #[inline(always)] pub fn zero()                  -> __m128 { unsafe { _mm_set1_ps(0.0) } }
    #[inline(always)] pub fn half()                  -> __m128 { unsafe { _mm_set1_ps(0.5) } }
    #[inline(always)] pub fn four()                  -> __m128 { unsafe { _mm_set1_ps(4.0) } }
    #[inline(always)] pub fn _1234()                 -> __m128 { unsafe { _mm_set_ps(1.0, 2.0, 3.0, 4.0) } }
    #[inline(always)] pub fn _0123()                 -> __m128 { unsafe { _mm_set_ps(0.0, 1.0, 2.0, 3.0) } }
    #[inline(always)] pub fn nine_two_zero()         -> __m128 { unsafe { _mm_set1_ps(0.00920833) } }
    #[inline(always)] pub fn zero_zero_five()        -> __m128 { unsafe { _mm_set1_ps(0.05) } }
    #[inline(always)] pub fn eight_seven_six()       -> __m128 { unsafe { _mm_set1_ps(0.0876) } }
    #[inline(always)] pub fn one_zero_three()        -> __m128 { unsafe { _mm_set1_ps(0.0103592) } }
    #[inline(always)] pub fn one_eight_five()        -> __m128 { unsafe { _mm_set1_ps(0.185) } }
    #[inline(always)] pub fn zero_four_five()        -> __m128 { unsafe { _mm_set1_ps(0.45) } }
    #[inline(always)] pub fn zero_five()             -> __m128 { unsafe { _mm_set1_ps(0.5)   } }
    #[inline(always)] pub fn one()                   -> __m128 { unsafe { _mm_set1_ps(1.0)   } }
    #[inline(always)] pub fn one_three_five()        -> __m128 { unsafe { _mm_set1_ps(1.035) } }
    #[inline(always)] pub fn two()                   -> __m128 { unsafe { _mm_set1_ps(2.0)   } }
    #[inline(always)] pub fn three()                 -> __m128 { unsafe { _mm_set1_ps(3.0)   } }
    #[inline(always)] pub fn gain_adjustment_2pole() -> __m128 { unsafe { _mm_set1_ps(0.74)  } }
    #[inline(always)] pub fn gain_adjustment_4pole() -> __m128 { unsafe { _mm_set1_ps(0.6)   } } 
}
