ix!();

use crate::clamp::*;

#[inline] pub fn float_2_int(i: f32) -> i32 
{
    unsafe{ 
        _mm_cvt_ss2si(_mm_load_ss(&i)) 
    }
}

/// # Safety
///
///both pointers, f and s must be to contiguous arrays of n elements long
#[inline] pub unsafe fn float2i15_block(f: *mut f32,  s: *mut i16, n:i32)
{
    for i in 0..n {
        *s.offset(i as isize) = 
            (
                limit_range_i(
                    (*f.offset(i as isize) * 16384.0) as i32, -16384, 16383
                ) as i32
            ) as i16;
    }
}

/// # Safety
///
///both pointers, f and s must be to contiguous arrays of n elements long
#[inline] pub unsafe fn i152float_block(s: *mut i16, f: *mut f32, n: i32)
{
    let scale: f32 = 1.0 / 16384.0;

    for i in 0..n {
        *f.offset(i as isize) = ((*s.offset(i as isize)) as f32) * scale;
   }
}

/// # Safety
///
///i mut be between 0 and 3 inclusive
#[inline] pub unsafe fn set1f(m: &mut __m128, i: i32, f: f32)
{
    assert!((0..4).contains(&i));

    let m_ptr: *mut __m128 = m;
    let m_ptr_f: *mut f32 = m_ptr as *mut f32;

    *(m_ptr_f.offset(i as isize)) = f;
}

/// # Safety
///
///e mut be between 0 and 3 inclusive
#[inline] pub unsafe fn set1i(m: &mut __m128i, e: i32, i: i32)
{
    assert!((0..4).contains(&e));

    let m_ptr: *mut __m128i = m;
    let m_ptr_i: *mut i32 = m_ptr as *mut i32;

    *(m_ptr_i.offset(e as isize)) = i;
}

/// # Safety
///
///i mut be between 0 and 3 inclusive
#[inline] pub unsafe fn get1f(m: &__m128, i: i32) -> f32
{
    assert!((0..4).contains(&i));

    let m_ptr: *const __m128 = &*m;
    let m_ptr_f: *const f32 = m_ptr as *const f32;

    *(m_ptr_f.offset(i as isize))
}

/// # Safety
///
///i mut be between 0 and 3 inclusive
#[inline] pub unsafe fn get1f_from_f32(m: &f32, i: i32) -> f32
{
    assert!((0..4).contains(&i));

    let m_ptr_f: *const f32 = &*m;

    *(m_ptr_f.offset(i as isize))
}

#[test] fn test_set1f() 
{
    let mut x: __m128 = unsafe { _mm_setzero_ps() };
    let i = 2;
    unsafe { set1f(&mut x, i, 0.59) };
    println!("{:?}",x);
}

#[test] fn test_get1f() {

    let mut x: __m128 = unsafe { _mm_setzero_ps() };
    let i = 2;
    unsafe { set1f(&mut x, i, 0.59) };
    println!("{:?}",x);
    let result = unsafe { get1f(&x,2) };
    println!("{:?}",result);
    assert_eq!(0.59,result);
}

#[test] fn test_set1i() {

    let val = 4;

    let lanes: [u128; 4] = [ 
        0b00000000000000000000000000000000_00000000000000000000000000000000_00000000000000000000000000000000_00000000000000000000000000000100,
        0b00000000000000000000000000000000_00000000000000000000000000000000_00000000000000000000000000000100_00000000000000000000000000000000,
        0b00000000000000000000000000000000_00000000000000000000000000000100_00000000000000000000000000000000_00000000000000000000000000000000,
        0b00000000000000000000000000000100_00000000000000000000000000000000_00000000000000000000000000000000_00000000000000000000000000000000
    ];
    for lane in 0..4 {

        let mut x: __m128i = unsafe { _mm_set_epi32(0, 0, 0, 0) };
        unsafe { set1i(&mut x, lane, val) };
        println!("{:?}",x);
        println!("{:?}",lanes[lane as usize]);
    }
}
