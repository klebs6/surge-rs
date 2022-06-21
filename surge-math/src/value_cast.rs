crate::ix!();

#[inline] pub fn float_2_int(i: f32) -> i32 
{
    unsafe{ 
        _mm_cvt_ss2si(_mm_load_ss(&i)) 
    }
}

#[inline] pub fn float2i15_block<const N: usize>(f: [f32; N],  s: &mut [i16; N])
{
    for i in 0..N {
        s[i] = ( limit_range((f[i] * 16384.0) as i32, -16384, 16383) as i32 ) as i16;
    }
}

#[inline] pub fn i152float_block<const N: usize>(s: &[i16; N], f: &mut [f32; N])
{
    const SCALE: f32 = 1.0 / 16384.0;

    for i in 0..N {
        f[i] = (s[i] as f32) * SCALE;
    }
}

/**
  | # Safety
  | 
  | i mut be between 0 and 3 inclusive
  |
  */
#[inline] pub unsafe fn set1f(m: &mut __m128, i: i32, f: f32)
{
    assert!((0..4).contains(&i));

    let m_ptr: *mut __m128 = m;
    let m_ptr_f: *mut f32 = m_ptr as *mut f32;

    *(m_ptr_f.offset(i as isize)) = f;
}

/**
  | # Safety
  | 
  | e mut be between 0 and 3 inclusive
  |
  */
#[inline] pub unsafe fn set1i(m: &mut __m128i, e: i32, i: i32)
{
    assert!((0..4).contains(&e));

    let m_ptr: *mut __m128i = m;
    let m_ptr_i: *mut i32 = m_ptr as *mut i32;

    *(m_ptr_i.offset(e as isize)) = i;
}

/**
  | # Safety
  | 
  | i mut be between 0 and 3 inclusive
  |
  */
#[inline] pub unsafe fn get1f(m: &__m128, i: i32) -> f32
{
    assert!((0..4).contains(&i));

    let m_ptr: *const __m128 = &*m;
    let m_ptr_f: *const f32 = m_ptr as *const f32;

    *(m_ptr_f.offset(i as isize))
}

/**
  | # Safety
  | 
  | i mut be between 0 and 3 inclusive
  |
  */
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

    let gold: [u128; 4] = [ 
        0b00000000000000000000000000000000_00000000000000000000000000000000_00000000000000000000000000000000_00000000000000000000000000000100,
        0b00000000000000000000000000000000_00000000000000000000000000000000_00000000000000000000000000000100_00000000000000000000000000000000,
        0b00000000000000000000000000000000_00000000000000000000000000000100_00000000000000000000000000000000_00000000000000000000000000000000,
        0b00000000000000000000000000000100_00000000000000000000000000000000_00000000000000000000000000000000_00000000000000000000000000000000
    ];

    unsafe fn u128_into_m128i(x: u128) -> __m128i {
        let x0: i32 = ((x >> 96) & 0xFFFFFFFF) as i32;
        let x1: i32 = ((x >> 64) & 0xFFFFFFFF) as i32;
        let x2: i32 = ((x >> 32) & 0xFFFFFFFF) as i32;
        let x3: i32 = (x & 0xFFFFFFFF) as i32;
        _mm_set_epi32(x0,x1,x2,x3)
    }

    unsafe {
        for lane in 0..4 {

            let mut x: __m128i = _mm_set_epi32(0, 0, 0, 0);

            set1i(&mut x, lane, val);

            let mask = _mm_cmpeq_epi32(x,u128_into_m128i(gold[lane as usize]));
            assert_eq!(0xFFFFFFFF, _mm_extract_epi32::<0>(mask) as u32);
            assert_eq!(0xFFFFFFFF, _mm_extract_epi32::<1>(mask) as u32);
            assert_eq!(0xFFFFFFFF, _mm_extract_epi32::<2>(mask) as u32);
            assert_eq!(0xFFFFFFFF, _mm_extract_epi32::<3>(mask) as u32);
        }
    }
}
