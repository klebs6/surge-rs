crate::ix!();

#[cfg(target_arch = "x86_64")]
pub fn sign(x: i32) -> i32 
{
    match x < 0 { 
        true  => -1,
        false => 1,
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn sign(x: i32) -> i32 
{
    todo!("extract result");
    unsafe {
        asm![
            "mov eax, 1",
            "mov edx, -1",
            "cmp x, 0",
            "cmovle eax, edx",
        ]
    }
}

unsafe fn access_mut(ptr: *mut f32, offset: usize) -> *mut __m128 {
    (ptr as *mut __m128).add(offset) 
}

unsafe fn access(ptr: *const f32, offset: usize) -> *const __m128 {
    (ptr as *const __m128).add(offset) 
}

//______________________________________________________
pub fn mul_block<NQ>(
    src1: *mut f32, 
    src2: *mut f32, 
    dst:  *mut f32, 
    nquads: NQ) 
    where 
    <NQ as TryInto<u32>>::Error: fmt::Debug, 
    NQ: TryInto<u32>
{
    let do_mul = |offset: usize| {
        unsafe {
            let src1 = access(src1,offset);
            let src2 = access(src2,offset);
            let dst  = access_mut(dst, offset);
            *dst = _mm_mul_ps(*src1, *src2);
        }
    };

    let nquads: u32 = nquads.try_into().unwrap();

    for i in (0..nquads).step_by(4)
    {
        let i: usize = i.try_into().unwrap();

        do_mul(i);
        do_mul(i + 1);
        do_mul(i + 2);
        do_mul(i + 3);
    }
}

#[cfg(target_arch = "x86_64")] #[inline] 
pub fn rcp(mut x: f32) -> f32
{
    unsafe {_mm_store_ss(&mut x, _mm_rcp_ss(_mm_load_ss(&x))) };
    x
}

/**
  | dst += src
  |
  */
pub fn accumulate_block<NQ>(
    src:   *mut f32, 
    dst:   *mut f32, 
    nquads: NQ) 

    where <NQ as TryInto<u32>>::Error: fmt::Debug,
          NQ: TryInto<u32>
{
    let nquads: u32 = nquads.try_into().unwrap();

    let accumulate = |offset: usize| {
        unsafe {
            let src = access(src,offset);
            let dst = access_mut(dst,offset);

            *dst = _mm_add_ps( *dst, *src);
        }
    };

    for i in (0..nquads).step_by(4) {

        let i: usize = i.try_into().unwrap();

        accumulate(i);
        accumulate(i+1);
        accumulate(i+2);
        accumulate(i+3);
    }
}

pub fn add_block<NQ>(
    src1: *const f32, 
    src2: *const f32, 
    dst:  *mut f32, 
    nquads: NQ)

where <NQ as TryInto<u32>>::Error: fmt::Debug,
      NQ: TryInto<u32>

{
    let add = |offset: usize| {

        unsafe {
            let src1 = access(src1,offset);
            let src2 = access(src2,offset);
            let dst  = access_mut(dst,offset);

            *dst = _mm_add_ps(*src1,*src2);
        }
    };

    let nquads: u32 = nquads.try_into().unwrap();

    for i in (0..nquads).step_by(4)
    {
        let i: usize = i.try_into().unwrap();

        add(i);
        add(i + 1);
        add(i + 2);
        add(i + 3);
    }
}

//______________________________________________________
pub fn subtract_block<NQ>(
    src1: *const f32, 
    src2: *const f32, 
    dst:  *mut f32, 
    nquads: NQ) 

where 
    <NQ as TryInto<u32>>::Error: fmt::Debug,
    NQ: TryInto<u32>

{
    let sub = |offset: usize| {

        unsafe {

            let src1 = access(src1,offset);
            let src2 = access(src2,offset);
            let dst  = access_mut(dst,offset);

            *dst     = _mm_sub_ps(*src1,*src2);
        }
    };

    let nquads: u32 = nquads.try_into().unwrap();

    for i in (0..nquads).step_by(4)
    {
        let i = i as usize;

        sub(i);
        sub(i + 1);
        sub(i + 2);
        sub(i + 3);
    }
}

//______________________________________________________
#[inline] pub fn abs_ps(x: __m128 ) -> __m128
{
    unsafe{ _mm_and_ps(x, m128_mask_absval![]) }
}

#[inline] pub fn get_absmax<NQ>(
    d:     *mut f32, 
    nquads: NQ) -> f32

where <NQ as TryInto<u32>>::Error: fmt::Debug, 
      NQ: TryInto<u32>
{
    unsafe {

        let nquads: u32 = nquads.try_into().unwrap();

        let mut mx1: __m128 = _mm_setzero_ps();
        let mut mx2: __m128 = _mm_setzero_ps();

        for i in (0..nquads).step_by(2) {

            let i = i as usize;

            let d_0 = access(d,i);
            let d_1 = access(d,i+1);

            let mask = m128_mask_absval![];

            mx1 = _mm_max_ps( mx1, _mm_and_ps(*d_0, mask));
            mx2 = _mm_max_ps( mx2, _mm_and_ps(*d_1, mask));
        }

        mx1 = _mm_max_ps(mx1, mx2);
        mx1 = max_ps_to_ss(mx1);

        let mut f: f32 = 0.0;
        _mm_store_ss(&mut f, mx1);
        f
    }
}

#[inline] pub fn get_absmax_2<NQ>(
    d1:    *mut f32, 
    d2:    *mut f32, 
    nquads: NQ) -> f32

where 
    <NQ as TryInto<u32>>::Error: fmt::Debug, 
    NQ: TryInto<u32>

{
    unsafe {

        let nquads: u32 = nquads.try_into().unwrap();

        let mut mx1: __m128 =  _mm_setzero_ps();
        let mut mx2: __m128 =  _mm_setzero_ps();
        let mut mx3: __m128 =  _mm_setzero_ps();
        let mut mx4: __m128 =  _mm_setzero_ps();

        for i in (0..nquads).step_by(2) {

            let i = i as usize;

            let mask = m128_mask_absval![];

            let d1_0 = access(d1,i); 
            let d1_1 = access(d1,i+1); 
            let d2_0 = access(d2,i); 
            let d2_1 = access(d2,i+1); 

            mx1 = _mm_max_ps( mx1, _mm_and_ps( *d1_0, mask ));
            mx2 = _mm_max_ps( mx2, _mm_and_ps( *d1_1, mask ));
            mx3 = _mm_max_ps( mx3, _mm_and_ps( *d2_0, mask ));
            mx4 = _mm_max_ps( mx4, _mm_and_ps( *d2_1, mask ));
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
        _mm_add_ps( _mm_mul_ps(a, b), c) 
    }
}

#[inline] pub fn v_nmsub(a: __m128, b: __m128, c: __m128) -> __m128 {
    unsafe { 
        _mm_sub_ps( c, _mm_mul_ps(a, b)) 
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
