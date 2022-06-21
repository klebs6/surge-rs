crate::ix!();

/// y = x - (4/27)*x^3,  x € [-1.5 .. 1.5]
#[inline] 
#[cfg(target_arch = "x86_64")] 
pub fn softclip_ss(x_in: __m128) -> __m128
{
   let a: __m128     = unsafe{ _mm_set_ss(-4.0 / 27.0) };

   let x_min: __m128 = unsafe{ _mm_set_ss(-1.5) };
   let x_max: __m128 = unsafe{ _mm_set_ss(1.5) };

   let x: __m128     = unsafe{ _mm_max_ss(_mm_min_ss(x_in, x_max), x_min) };
   let xx: __m128    = unsafe{ _mm_mul_ss(x, x) };

   let mut t: __m128 = unsafe{ _mm_mul_ss(x, a) };
   t = unsafe{ _mm_mul_ss(t, xx) };
   t = unsafe{ _mm_add_ss(t, x) };
   t
}

/// y = x - (4/27)*x^3,  x € [-1.5 .. 1.5]
#[inline] 
#[cfg(target_arch = "x86_64")] 
pub fn softclip_ps(x_in: __m128) -> __m128
{
    unsafe {
        let a: __m128 = _mm_set1_ps(-4.0 / 27.0);

        let x_min: __m128 = _mm_set1_ps(-1.5);
        let x_max: __m128 = _mm_set1_ps(1.5);

        let x: __m128     = _mm_max_ps(_mm_min_ps(x_in, x_max), x_min);
        let xx: __m128    = _mm_mul_ps(x, x);

        let mut t: __m128 = _mm_mul_ps(x, a);
        t =  _mm_mul_ps(t, xx);
        t =  _mm_add_ps(t, x);
        t
    }
}

#[inline] pub fn softclip_f64(input: f64 ) -> f64
{
    let c0: f64 = 4.0 / 27.0;
    let mut x: f64 = match input > -1.5 { true => input, false => -1.5 };
    x = match x < 1.5  { true => x , false =>  1.5 };
    let ax: f64 = c0 * x;
    let xx: f64 = x * x;
    x - ax * xx
}

#[inline] pub fn softclip4_f64(input: f64 ) -> f64
{
   let mut x: f64 = match input > -6.0 { true => input, false => -6.0 };
   x = match x < 6.0 { true => x, false => 6.0 };
   let ax: f64 = 0.0023148148148 * x; // 1.0 / 27*16
   let xx: f64 = x * x;
   x - ax * xx
}

#[inline] pub fn softclip8_f64(input: f64 ) -> f64
{
   let mut x: f64 = match input > -12.0 { true => input, false => -12.0 };
   x = match x < 12.0 { true => x, false => 12.0 };
   let ax: f64 = 0.00028935185185 * x; // 1.0 / 27*128
   let xx: f64 = x * x;
   x - ax * xx
}

#[inline] pub fn softclip2_f64(input: f64 ) -> f64
{
    let mut x: f64 = match input > -3.0 { true => input, false => -3.0 };

    x = match x < 3.0 { true => x, false => 3.0 };

    let ax: f64 = 
        0.185_185_185_185_185_17
        * x; // 1.0 / 27*2

    let xx: f64 = x * x;
    x - ax * xx
}

#[inline] 
pub fn softclip8_ps(input: __m128 ) -> __m128
{
    unsafe {

        let a: __m128 = _mm_set1_ps(
            -0.000_289_351_85
        );

        let x_min: __m128 = _mm_set1_ps(-12.0);
        let x_max: __m128 = _mm_set1_ps(12.0);

        let x: __m128 = _mm_max_ps(_mm_min_ps(input, x_max), x_min);
        let xx: __m128 = _mm_mul_ps(x, x);
        let mut t: __m128 = _mm_mul_ps(x, a);
        t = _mm_mul_ps(t, xx);
        t = _mm_add_ps(t, x);
        t
    }
}

/**
  |# Safety
  |
  |must be able to access nquads * 4 contiguous
  |values from the valid starting point "input"
  */
pub unsafe fn softclip_block<NQ>(input: *mut f32, nquads: NQ) 
    where 
        <NQ as std::convert::TryInto<usize>>::Error: std::fmt::Debug, 
        NQ: TryInto<usize>
{
    let nquads: usize = nquads.try_into().unwrap();

    // y = x - (4/27)*x^3,  x [-1.5 .. 1.5]
    let a:     __m128 = _mm_set1_ps(-4.0 / 27.0);
    let x_min: __m128 = _mm_set1_ps(-1.50);
    let x_max: __m128 = _mm_set1_ps(1.50);

    for i in (0_usize..nquads).step_by(2) 
    {
        let mut x:  [__m128; 2] = [z128![]; 2];
        let mut xx: [__m128; 2] = [z128![]; 2];
        let mut t:  [__m128; 2] = [z128![]; 2];

        x[0] = _mm_min_ps(_mm_load_ps(input.add(i << 2)), x_max);
        x[1] = _mm_min_ps(_mm_load_ps(input.add((i + 1) << 2)), x_max);

        x[0] = _mm_max_ps(x[0], x_min);
        x[1] = _mm_max_ps(x[1], x_min);

        xx[0] = _mm_mul_ps(x[0], x[0]);
        xx[1] = _mm_mul_ps(x[1], x[1]);

        t[0] = _mm_mul_ps(x[0], a);
        t[1] = _mm_mul_ps(x[1], a);

        t[0] = _mm_mul_ps(t[0], xx[0]);
        t[1] = _mm_mul_ps(t[1], xx[1]);

        t[0] = _mm_add_ps(t[0], x[0]);
        t[1] = _mm_add_ps(t[1], x[1]);

        _mm_store_ps(input.add(i << 2), t[0]);
        _mm_store_ps(input.add((i + 1) << 2), t[1]);
    }
}
