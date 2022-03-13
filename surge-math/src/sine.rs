ix!();

// Snabba sin(x) substitut
// work in progress
#[inline] pub fn sine_float_nowrap(x: f32) -> f32 
{
   // http://www.devmaster.net/forums/showthread.php?t=5784
    let four_over_pi:         f32 = (4.0 / PI) as f32;
    let minus_four_over_pi_2: f32 = (-4.0 / (PI * PI)) as f32;

    let y: f32 = four_over_pi * x + minus_four_over_pi_2 * x * x.abs();

    // EXTRA_PRECISION
    //  const float Q = 0.775;
    let p: f32 = 0.225;

    // Q * y + P * y * abs(y)
    p * (y * y.abs() - y) + y 
}

#[inline] pub fn sine_ps_nowrap(x: __m128) -> __m128 
{
    unsafe {
        let four_over_pi:         __m128 = _mm_set1_ps((4.0 / PI) as f32);
        let minus_four_over_pi_2: __m128 = _mm_set1_ps((-4.0 / (PI * PI)) as f32);

        // todo wrap x [0..1] ?

        let mask = m128_mask_absval![];

        // y = B * x + C * x * abs(x);
        let y: __m128 = {

            let a1 = _mm_and_ps(mask, x);
            let p1 = _mm_mul_ps(minus_four_over_pi_2, x);
            let p2 = _mm_mul_ps(p1, a1);
            let p3 = _mm_mul_ps(four_over_pi, x);

            _mm_add_ps( p3, p2)
        };

        // EXTRA_PRECISION
        //  const float Q = 0.775;
        let p: __m128 = _mm_set1_ps(0.2250);

        let res = {
            let a1 = _mm_and_ps(mask, y);
            let p1 = _mm_mul_ps(a1, y);
            let s1 = _mm_sub_ps(p1, y);
            let p2 = _mm_mul_ps(s1, p);

            _mm_add_ps( p2, y)
        };

        res
    }
}

/// sin(x*pi)
#[inline] pub fn sine_xpi_ps_sse2(mut x: __m128) -> __m128 
{
    unsafe {
        let four:    __m128  = _mm_set1_ps(4.0);
        let mask:    __m128i = _mm_set1_epi32(0x01ffffff);
        let offset:  __m128i = _mm_set1_epi32(0x01000000);

        // wrap x
        x = {
            let a1 = _mm_add_epi32(offset, _mm_cvttps_epi32(x));
            let a2 = _mm_and_si128(a1, mask);
            let s1 = _mm_sub_epi32(a2, offset);

            _mm_cvtepi32_ps(s1)
        };

        let mask2 = m128_mask_absval![];

        let y: __m128 = {

            let a1 = _mm_and_ps(mask2, x);
            let p1 = _mm_mul_ps(x, a1);
            let s1 = _mm_sub_ps(x, p1);

            _mm_mul_ps( four, s1)
        };

        let p: __m128 = _mm_set1_ps(0.2250);

        let res = {
            let a1 = _mm_and_ps(mask2, y);
            let p1 = _mm_mul_ps(a1, y);
            let s1 = _mm_sub_ps(p1, y);
            let p2 = _mm_mul_ps(s1, p);
            _mm_add_ps( p2, y)
        };

        res
    }
}

#[cfg(target_argh = "x86")]
pub fn sine(x: __m64) -> __m64 
{
    unsafe {

        let xabs: __m64 = {

            let srai = _mm_srai_pi16(x, 15);

            _mm_xor_si64( x, srai)
        };

        let mut y: __m64 = {
            let srai  = _mm_srai_pi16(x, 1);
            let mulhi = _mm_mulhi_pi16(x, xabs);

            _mm_subs_pi16( srai, mulhi)
        };

        y = _mm_slli_pi16(y, 2);
        y = _mm_adds_pi16(y, y);

        let q: __m64 = _mm_set1_pi16(0x6333);
        let p: __m64 = _mm_set1_pi16(0x1CCD);

        let yabs: __m64 = {

            let srai = _mm_srai_pi16(y, 15);

            _mm_xor_si64( y, srai)
        };

        let y1: __m64 = _mm_mulhi_pi16(q, y);

        let y2: __m64 = {

            let mulhi1 = _mm_mulhi_pi16(y, yabs);
            let slli   = _mm_slli_pi16( mulhi1, 1);

            _mm_mulhi_pi16( p, slli)
        };

        y = _mm_add_pi16(y1, y2);

        _mm_adds_pi16(y, y)
    }
}

/// 16-bit sine
#[cfg(target_arch = "x86_64")]
pub fn sine(mut x: i32) -> i32 
{
    x = {
        let a = (x + 0x8000) & 0xffff;
        a - 0x8000
    };

    let y: i32 = {
        let a0 = x << 2;
        let b0 = abs(x >> 1) * (x >> 1);
        let a1 = b0 >> 11;
        a0 - a1
    };

    let q: i32 = (0.7750 * 65536.0) as i32;
    let p: i32 = (0.2250 * 32768.0) as i32;

    let res = {

        let a1 = (q * y) >> 16;

        let a2 = {
            let p0 = (y >> 2) * abs(y >> 2);
            let p1 = p0 >> 11;
            let p2 = p1 * p;
            p2 >> 15
        };

        a1 + a2
    };

    res
}
