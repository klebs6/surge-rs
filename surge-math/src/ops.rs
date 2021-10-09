ix!();

use crate::max_ps_to_ss;

//______________________________________________________
pub fn mul_block<NQ>(
    src1: *mut f32, 
    src2: *mut f32, 
    dst: *mut f32, 
    nquads: NQ) 
    where <NQ as std::convert::TryInto<u32>>::Error: std::fmt::Debug, 
    NQ: TryInto<u32>
{
    let nquads: u32 = nquads.try_into().unwrap();
    for i in (0..nquads).step_by(4)
    {
        unsafe {
            *(dst as *mut __m128).offset((i) as isize) = _mm_mul_ps(*(src1 as *mut __m128).offset((i) as isize), *(src2 as *mut __m128).offset((i) as isize));
            *(dst as *mut __m128).offset((i + 1) as isize) = _mm_mul_ps(*(src1 as *mut __m128).offset((i + 1) as isize), *(src2 as *mut __m128).offset((i + 1) as isize));
            *(dst as *mut __m128).offset((i + 2) as isize) = _mm_mul_ps(*(src1 as *mut __m128).offset((i + 2) as isize), *(src2 as *mut __m128).offset((i + 2) as isize));
            *(dst as *mut __m128).offset((i + 3) as isize) = _mm_mul_ps(*(src1 as *mut __m128).offset((i + 3) as isize), *(src2 as *mut __m128).offset((i + 3) as isize));
        }
    }
}

//______________________________________________________
#[cfg(target_arch = "x86_64")] #[inline] 
pub fn rcp(mut x: f32) -> f32
{
    unsafe {_mm_store_ss(&mut x, _mm_rcp_ss(_mm_load_ss(&x))) };
    x
}

//______________________________________________________
pub fn accumulate_block<NQ>(src: *mut f32, dst: *mut f32, nquads: NQ) 
    where <NQ as std::convert::TryInto<u32>>::Error: std::fmt::Debug,
          NQ: TryInto<u32>
    // dst += src
{
    let nquads: u32 = nquads.try_into().unwrap();
    for i in (0..nquads).step_by(4) {

        unsafe {

            *(dst as *mut __m128).offset(i as isize ) = 
                _mm_add_ps(
                    *(dst as *mut __m128).offset(i as isize ), 
                    *(src as *mut __m128).offset(i as isize)
                );

            *(dst as *mut __m128).offset(i as isize + 1) = 
                _mm_add_ps(
                    *(dst as *mut __m128).offset(i as isize  + 1), 
                    *(src as *mut __m128).offset(i as isize + 1)
                );

            *(dst as *mut __m128).offset(i as isize + 2) = 
                _mm_add_ps(
                    *(dst as *mut __m128).offset(i as isize  + 2), 
                    *(src as *mut __m128).offset(i as isize + 2)
                );

            *(dst as *mut __m128).offset(i as isize + 3) = 
                _mm_add_ps(
                    *(dst as *mut __m128).offset(i as isize  + 3), 
                    *(src as *mut __m128).offset(i as isize + 3)
                );

        }
    }
}

pub fn add_block<NQ>(src1: *const f32, src2: *const f32, dst: *mut f32, nquads: NQ)
    where <NQ as std::convert::TryInto<u32>>::Error: std::fmt::Debug,
          NQ: TryInto<u32>
{
    let nquads: u32 = nquads.try_into().unwrap();
    for i in (0..nquads).step_by(4)
    {
        let i0 = i as isize;
        let i1 = (i + 1) as isize;
        let i2 = (i + 2) as isize;
        let i3 = (i + 3) as isize;
        unsafe {
            *(dst as *mut __m128).offset(i0) = _mm_add_ps(
                *(src1 as *const __m128).offset(i0),  
                *(src2 as *const __m128).offset(i0)
            );
            *(dst as *mut __m128).offset(i1) = _mm_add_ps(
                *(src1 as *const __m128).offset(i1), 
                *(src2 as *const __m128).offset(i1)
            );
            *(dst as *mut __m128).offset(i2) = _mm_add_ps(
                *(src1 as *const __m128).offset(i2), 
                *(src2 as *const __m128).offset(i2)
            );
            *(dst as *mut __m128).offset(i3) = _mm_add_ps(
                *(src1 as *const __m128).offset(i3), 
                *(src2 as *const __m128).offset(i3)
            );
        }
    }
}

//______________________________________________________
pub fn subtract_block<NQ>(src1: *const f32, src2: *const f32, dst: *mut f32, nquads: NQ) 
    where <NQ as std::convert::TryInto<u32>>::Error: std::fmt::Debug,
          NQ: TryInto<u32>
{

    let nquads: u32 = nquads.try_into().unwrap();

    for i in (0..nquads).step_by(4)
    {
        let i0 = i as isize;
        let i1 = (i + 1) as isize;
        let i2 = (i + 2) as isize;
        let i3 = (i + 3) as isize;

        unsafe {
            *(dst as *mut __m128).offset(i0) = _mm_sub_ps(*(src1 as *const __m128).offset(i0), *(src2 as *const __m128).offset(i0));
            *(dst as *mut __m128).offset(i1) = _mm_sub_ps(*(src1 as *const __m128).offset(i1), *(src2 as *const __m128).offset(i1));
            *(dst as *mut __m128).offset(i2) = _mm_sub_ps(*(src1 as *const __m128).offset(i2), *(src2 as *const __m128).offset(i2));
            *(dst as *mut __m128).offset(i3) = _mm_sub_ps(*(src1 as *const __m128).offset(i3), *(src2 as *const __m128).offset(i3));
        }
    }
}

//______________________________________________________
#[inline] pub fn abs_ps(x: __m128 ) -> __m128
{
    unsafe{ _mm_and_ps(x, m128_mask_absval![]) }
}


#[inline] pub fn get_absmax<NQ>(d: *mut f32, nquads: NQ) -> f32
where <NQ as std::convert::TryInto<u32>>::Error: std::fmt::Debug, 
      NQ: TryInto<u32>
{
    unsafe {

        let nquads: u32 = nquads.try_into().unwrap();

        let mut mx1: __m128 = _mm_setzero_ps();
        let mut mx2: __m128 = _mm_setzero_ps();

        for i in (0..nquads).step_by(2) {

            mx1 = _mm_max_ps(
                mx1, 
                _mm_and_ps(*(d as *mut __m128).offset(i as isize), m128_mask_absval![]));

            mx2 = _mm_max_ps(
                mx2, 
                _mm_and_ps(*(d as *mut __m128).offset(i as isize + 1), m128_mask_absval![]));

        }

        mx1 = _mm_max_ps(mx1, mx2);
        mx1 = max_ps_to_ss(mx1);

        let mut f: f32 = 0.0;
        _mm_store_ss(&mut f, mx1);
        f
    }
}

#[inline] pub fn get_absmax_2<NQ>(d1: *mut f32, d2: *mut f32, nquads: NQ) -> f32
where <NQ as std::convert::TryInto<u32>>::Error: std::fmt::Debug, 
      NQ: TryInto<u32>
{
    unsafe {

        let nquads: u32 = nquads.try_into().unwrap();

        let mut mx1: __m128 =  _mm_setzero_ps();
        let mut mx2: __m128 =  _mm_setzero_ps();
        let mut mx3: __m128 =  _mm_setzero_ps();
        let mut mx4: __m128 =  _mm_setzero_ps();

        for i in (0..nquads).step_by(2) {

            mx1 = _mm_max_ps( 
                mx1, 
                _mm_and_ps( 
                    *( d1 as *mut __m128 ).offset(i as isize), 
                    m128_mask_absval![] ));

            mx2 = _mm_max_ps( 
                mx2, 
                _mm_and_ps( 
                    *( d1 as *mut __m128 ).offset(i as isize + 1), 
                    m128_mask_absval![] ));

            mx3 = _mm_max_ps( 
                mx3, 
                _mm_and_ps( 
                    *( d2 as *mut __m128 ).offset(i as isize), 
                    m128_mask_absval![] ));

            mx4 = _mm_max_ps( 
                mx4, 
                _mm_and_ps( 
                    *( d2 as *mut __m128 ).offset(i as isize + 1), 
                    m128_mask_absval![] ));
        }

        mx1 = _mm_max_ps(mx1, mx2);
        mx3 = _mm_max_ps(mx3, mx4);
        mx1 = _mm_max_ps(mx1, mx3);
        mx1 = max_ps_to_ss(mx1);

        let mut f: f32 = 0.0;

        _mm_store_ss(&mut f, mx1); 

        f
    }
}

#[inline] pub fn v_madd(a: __m128, b: __m128, c: __m128) -> __m128 {
    unsafe { 
        _mm_add_ps(
            _mm_mul_ps(a, b), 
            c) 
    }
}

#[inline] pub fn v_nmsub(a: __m128, b: __m128, c: __m128) -> __m128 {
    unsafe { 
        _mm_sub_ps(
            c, 
            _mm_mul_ps(a, b)) 
    }
}

#[inline] pub fn v_sum(x: __m128) -> f32 
{
    unsafe {

        let mut f: f32 = 0.0;

        let mut a: __m128 = _mm_add_ps(
            x, 
            _mm_movehl_ps(x, x)
        );

        a = _mm_add_ss(
            a, 
            _mm_shuffle_ps(
                a, 
                a, 
                _MM_SHUFFLE(0, 0, 0, 1))
        );

        _mm_store_ss(&mut f, a);

        f
    }
}

/// 64-bit unsigned multiply with right shift by 16 bits
#[inline] pub fn big_mul_r16(a: u32, b: u32) -> u32 
{
    let c: u64 = (a as u64) * (b as u64);
    (c >> 16) as u32
}
