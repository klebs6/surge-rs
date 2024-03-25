crate::ix!();

#[cfg(target_arch = "x86_64")] #[inline] 
pub fn rcp(mut x: f32) -> f32
{
    unsafe {_mm_store_ss(&mut x, _mm_rcp_ss(_mm_load_ss(&x))) };
    x
}

/// Performs an element-wise multiply-add operation.
///
/// Multiplies a and b, then adds the result to c.
///
#[inline] pub fn v_madd(a: __m128, b: __m128, c: __m128) -> __m128 {
    unsafe { 
        _mm_add_ps( _mm_mul_ps(a, b), c) 
    }
}

/// Performs an element-wise negative multiply-subtract
/// operation.
///
/// Multiplies a and b, then subtracts the result from c.
///
#[inline] pub fn v_nmsub(a: __m128, b: __m128, c: __m128) -> __m128 {
    unsafe { 
        _mm_sub_ps( c, _mm_mul_ps(a, b)) 
    }
}
